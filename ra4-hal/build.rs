use std::{collections::HashMap, env, fs, path::PathBuf};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PinDef {
    pin_count: Vec<u8>,
    pfunc: Option<String>,
}

type PinEntry = HashMap<String, HashMap<String, PinDef>>;

#[derive(Debug, Deserialize)]
struct PinMap {
    #[serde(flatten)]
    the_map: HashMap<String, PinEntry>,
}

fn main() {
    if let Err(e) = inner_main() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

/// The `RA4M1` is available with the following pin configurations
const ALL_PINS: &[u8] = &[40, 48, 64, 100];

/// Beautify output from `quote!`
fn pretty_print(ts: &proc_macro2::TokenStream) -> String {
    let file = syn::parse_file(&ts.to_string()).unwrap();
    prettyplease::unparse(&file)
}

fn pin_conditional(valid_for_pins: &[u8]) -> TokenStream {
    if valid_for_pins == ALL_PINS {
        quote!()
    } else {
        // #[cfg(any(feature = "_64pin", feature = "_100pin"))]
        let conditions = valid_for_pins
            .iter()
            .map(|config| {
                let config = format!("_{config}pin");
                quote!(feature = #config)
            })
            .collect::<Vec<_>>();
        quote!(#[cfg(any(#(#conditions),*))])
    }
}

/// Add the appropriate impls for a `GPT` timer
fn do_gpt(peripheral: &str, signals: &PinEntry) -> Vec<TokenStream> {
    let peripheral = match peripheral {
        "gpt0" | "gpt1" => {
            format_ident!("{}", peripheral.replace("gpt", "GPT32_"))
        }
        _ => {
            format_ident!("{}", peripheral.replace("gpt", "GPT16_"))
        }
    };

    signals
        .iter()
        .filter(|(signal, _)| signal.as_str() == "GTIOCA" || signal.as_str() == "GTIOCB")
        .fold(vec![], |mut acc, (signal, pins)| {
            let channel = match signal.as_str() {
                "GTIOCA" => format_ident!("ChanA"),
                "GTIOCB" => format_ident!("ChanB"),
                _ => unreachable!(),
            };

            for (pin, config) in pins.iter() {
                let pin = format_ident!("{}", pin);

                let conditions = pin_conditional(&config.pin_count);

                acc.push(quote! {
                    #conditions
                    crate::pwm::pwm_pin!(#peripheral, #channel, #pin, Gpt2);
                });
            }

            acc
        })
}

fn do_sci(peripheral: &str, signals: &PinEntry) -> Vec<TokenStream> {
    let peripheral = format_ident!("{}", peripheral.to_uppercase());

    signals
        .iter()
        .filter(|(signal, _)| signal.as_str() == "TXD_MOSI" || signal.as_str() == "RXD_MISO")
        .fold(vec![], |mut acc, (signal, pins)| {
            let signal = match signal.as_str() {
                "TXD_MOSI" => format_ident!("tx_pin_impl"),
                "RXD_MISO" => format_ident!("rx_pin_impl"),
                _ => unreachable!(),
            };

            for (pin, config) in pins.iter() {
                let pin = format_ident!("{}", pin);

                let conditions = pin_conditional(&config.pin_count);

                let pfunc = match config.pfunc.as_ref().unwrap().as_str() {
                    "IOPORT_PERIPHERAL_SCI0_2_4_6_8" => format_ident!("Sci1"),
                    "IOPORT_PERIPHERAL_SCI1_3_5_7_9" => format_ident!("Sci2"),
                    _ => unreachable!(),
                };

                acc.push(quote! {
                    #conditions
                    crate::uart::#signal!(#peripheral, #pin, #pfunc);
                });
            }

            acc
        })
}

fn do_i2c(peripheral: &str, signals: &PinEntry) -> Vec<TokenStream> {
    let peripheral = format_ident!("{}", peripheral.to_uppercase());

    signals
        .iter()
        .filter(|(signal, _)| signal.as_str() == "SDA" || signal.as_str() == "SCL")
        .fold(vec![], |mut acc, (signal, pins)| {
            let signal = match signal.as_str() {
                "SDA" => format_ident!("data_pin_impl"),
                "SCL" => format_ident!("clock_pin_impl"),
                _ => unreachable!(),
            };

            for (pin, config) in pins.iter() {
                let pin = format_ident!("{}", pin);

                let conditions = pin_conditional(&config.pin_count);

                let pfunc = match config.pfunc.as_ref().unwrap().as_str() {
                    "IOPORT_PERIPHERAL_IIC" => format_ident!("I2c"),
                    _ => unreachable!(),
                };

                acc.push(quote! {
                    #conditions
                    crate::i2c::#signal!(#peripheral, #pin, #pfunc);
                });
            }

            acc
        })
}

fn do_adc(peripheral: &str, signals: &PinEntry) -> Vec<TokenStream> {
    // Not really needed because the 'adc' instance doesn't define any signals we care about but why not
    if peripheral == "adc" {
        return vec![];
    }

    signals
        .iter()
        .filter(|(signal, _)| signal.starts_with("AN"))
        .fold(vec![], |mut acc, (signal, pins)| {
            let channel = signal.strip_prefix("AN").unwrap().parse::<u8>().unwrap();

            for (pin, config) in pins.iter() {
                let pin = format_ident!("{}", pin);
                let conditions = pin_conditional(&config.pin_count);

                acc.push(quote! {
                    #conditions
                    crate::adc::channel::input_pin_impl!(#pin);
                });

                acc.push(quote! {
                    #conditions
                    crate::adc::channel::chan_impl!(#channel, #pin);
                });
            }

            acc
        })
}

fn inner_main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=support/pinmap.yaml");

    let yaml = std::fs::read_to_string("../support/pinmap.yaml")?;
    let pin_map = serde_yaml::from_str::<PinMap>(&yaml)?.the_map;

    let pins = pin_map
        .iter()
        .filter_map(|(peripheral, signal)| {
            let peripheral_kind = &peripheral[0..3.min(peripheral.len())];
            match peripheral_kind {
                "adc" => Some(do_adc(peripheral, signal)),
                "gpt" => Some(do_gpt(peripheral, signal)),
                "iic" => Some(do_i2c(peripheral, signal)),
                "sci" => Some(do_sci(peripheral, signal)),
                _ => None,
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    let pins = quote! {
        #(#pins)*
    };
    let pins = pretty_print(&pins);

    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("pin_traits.rs"), pins)?;

    Ok(())
}
