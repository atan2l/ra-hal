use std::{collections::HashMap, env, fs, path::PathBuf};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PinMap {
    #[serde(flatten)]
    the_map: HashMap<String, HashMap<String, HashMap<String, Vec<u8>>>>,
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
fn do_gpt(
    peripheral: &str,
    signals: &HashMap<String, HashMap<String, Vec<u8>>>,
) -> Vec<TokenStream> {
    let peripheral = match peripheral {
        "gpt0" | "gpt1" => {
            format_ident!("{}", peripheral.replace("gpt", "GPT32_"))
        }
        _ => {
            format_ident!("{}", peripheral.replace("gpt", "GPT16_"))
        }
    };

    let signals = signals
        .iter()
        .filter(|(signal, _)| signal.as_str() == "GTIOCA" || signal.as_str() == "GTIOCB")
        .map(|(signal, pins)| {
            let channel = match signal.as_str() {
                "GTIOCA" => format_ident!("ChanA"),
                "GTIOCB" => format_ident!("ChanB"),
                _ => unreachable!(),
            };
            pins.iter()
                .map(|(pin, config)| {
                    let pin = format_ident!("{}", pin);
                    let conditions = pin_conditional(config);
                    quote! {
                        #conditions
                        crate::pwm::pwm_pin!(#peripheral, #channel, #pin, Gpt2);
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    signals
}

fn inner_main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=support/pinmap.yaml");

    let yaml = std::fs::read_to_string("../support/pinmap.yaml")?;
    let pin_map = serde_yaml::from_str::<PinMap>(&yaml)?.the_map;

    let pins = pin_map
        .iter()
        .filter_map(|(peripheral, signal)| {
            if peripheral.starts_with("gpt") {
                return Some(do_gpt(peripheral, signal));
            }

            None
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
