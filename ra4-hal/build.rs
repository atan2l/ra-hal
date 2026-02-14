use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    env, fs,
    path::PathBuf,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
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

#[derive(Debug, Deserialize)]
struct Peripherals {
    peripherals: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Interrupts {
    interrupts: Vec<String>,
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
        let conditions = valid_for_pins
            .iter()
            .map(|config| {
                let config = format!("_{config}pin");
                quote!(feature = #config)
            })
            .collect::<Vec<_>>();

        if conditions.len() == 1 {
            let condition = conditions.first().unwrap();
            quote!(#[cfg(#condition)])
        } else {
            quote!(#[cfg(any(#(#conditions),*))])
        }
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
            let signal = format_ident!("{}_pin_impl", signal.to_lowercase());

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

fn do_spi(peripheral: &str, signals: &PinEntry) -> Vec<TokenStream> {
    let peripheral = format_ident!("{}", peripheral.to_uppercase());

    signals
        .iter()
        .filter(|(signal, _)| {
            signal.as_str() == "MISO" || signal.as_str() == "MOSI" || signal.as_str() == "RSPCK"
        })
        .fold(vec![], |mut acc, (signal, pins)| {
            for (pin, config) in pins.iter() {
                let signal = match signal.as_str() {
                    "MISO" => format_ident!("miso_pin_impl"),
                    "MOSI" => format_ident!("mosi_pin_impl"),
                    "RSPCK" => format_ident!("sck_pin_impl"),
                    _ => unreachable!(),
                };

                let pin = format_ident!("{}", pin);

                let conditions = pin_conditional(&config.pin_count);

                let pfunc = match config.pfunc.as_ref().unwrap().as_str() {
                    "IOPORT_PERIPHERAL_SPI" => format_ident!("Spi"),
                    _ => unreachable!(),
                };

                acc.push(quote! {
                    #conditions
                    crate::spi::#signal!(#peripheral, #pin, #pfunc);
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
                    crate::adc::channel::adc_pin!(#channel, #pin);
                });
            }

            acc
        })
}

fn do_port(_peripheral: &str, signals: &PinEntry) -> Vec<TokenStream> {
    signals.iter().fold(vec![], |mut acc, (port, pins)| {
        for (pin_name, config) in pins.iter() {
            let port_ident = format_ident!("{}", port.replace("p", "PORT"));

            let pin_ident = format_ident!("P{}", &pin_name[1..]);

            let pin_number = pin_name[1..]
                .parse::<u16>()
                .expect("Expect pin name in form of pNNN");

            let conditions = pin_conditional(&config.pin_count);

            acc.push(quote! {
                #conditions
                crate::gpio::pin_impl!(#pin_ident, #pin_number, #port_ident);
            });

            if let Some(pfunc) = config.pfunc.as_ref() {
                if let Some(irq_number) = pfunc.strip_prefix("IRQ") {
                    let irq_ident = format_ident!("PortIrq{}", irq_number);
                    let irq_peri = format_ident!("GPIO_IRQ{}", irq_number);

                    acc.push(quote! {
                        #conditions
                        impl crate::gpio::InterruptiblePin for crate::peripherals::#pin_ident {}

                        #conditions
                        impl crate::gpio::SealedIntPin for crate::peripherals::#pin_ident {
                            const INTERRUPT_EVENT: crate::event_link::InterruptEvent = crate::event_link::InterruptEvent::#irq_ident;

                            fn waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
                                static WAKER: embassy_sync::waitqueue::AtomicWaker = embassy_sync::waitqueue::AtomicWaker::new();
                                &WAKER
                            }
                        }

                        #conditions
                        impl crate::gpio::GpioIrq<crate::peripherals::#pin_ident> for crate::peripherals::#irq_peri {}
                    });
                }
            } else {
                continue;
            };
        }

        acc
    })
}

fn generate_pinmap(pin_map: &PinMap) -> Result<(), Box<dyn std::error::Error>> {
    let pins = pin_map
        .the_map
        .iter()
        .filter_map(|(peripheral, signal)| {
            let peripheral_kind = match peripheral.strip_suffix(|c: char| c.is_ascii_digit()) {
                Some(kind) => kind,
                None => peripheral,
            };

            match peripheral_kind {
                "adc" => Some(do_adc(peripheral, signal)),
                "gpt" => Some(do_gpt(peripheral, signal)),
                "iic" => Some(do_i2c(peripheral, signal)),
                "spi" => Some(do_spi(peripheral, signal)),
                "sci" => Some(do_sci(peripheral, signal)),
                "port" => Some(do_port(peripheral, signal)),
                _ => None,
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    let pins = quote! {
        mod pin_trait_impl {
                #(#pins)*
        }
    };
    let pins = pretty_print(&pins);

    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("pin_traits.rs"), pins)?;

    Ok(())
}

fn generate_interrupt_mod(irq_map: &Interrupts) -> Result<(), Box<dyn std::error::Error>> {
    let interrupts = irq_map.interrupts.iter().map(|irq| format_ident!("{irq}"));

    let interrupts = quote! {
        #[allow(clippy::missing_safety_doc)]
        mod _interrupt {
            embassy_hal_internal::interrupt_mod!(
                #(#interrupts),*
            );
        }
        pub use _interrupt::interrupt;
    };
    let interrupts = pretty_print(&interrupts);

    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("interrupts.rs"), interrupts)?;

    Ok(())
}

fn generate_peripherals(
    pin_map: &PinMap,
    peripherals: &Peripherals,
) -> Result<(), Box<dyn std::error::Error>> {
    // Regex implicitly matches the whole line
    // If the result is an empty string the peripheral is removed
    let transform = [
        ("ADC140", "ADC14"),
        ("GPT(16|32)(\\d)", "GPT${1}_${2}"),
        ("PORT[0-9]", ""),
        ("TSN", ""),
        ("DMA", ""),
    ]
    .map(|(from, to)| (Regex::new(format!("^{from}$").as_str()).unwrap(), to));

    let mut peripheral_list = peripherals
        .peripherals
        .iter()
        .filter_map(|p| {
            for (from, to) in transform.iter() {
                if from.is_match(p) {
                    let replaced = from.replace_all(p, *to);
                    if replaced.is_empty() {
                        return None;
                    } else {
                        return Some(replaced);
                    }
                }
            }
            Some(Cow::Owned(p.to_owned()))
        })
        .map(|p| {
            let ident = format_ident!("{p}");
            (format!("{p}"), quote! { #ident })
        })
        .collect::<Vec<_>>();

    let gpio = &pin_map.the_map["port"];

    let gpio_peris = gpio.iter().fold(vec![], |mut acc, (_, pins)| {
        for (pin, config) in pins.iter() {
            let conditions = pin_conditional(&config.pin_count);
            let pin_ident = format_ident!("{}", pin.to_uppercase());

            acc.push((
                pin.to_uppercase(),
                quote! {
                    #conditions
                    #pin_ident
                },
            ));
        }
        acc
    });

    let gpio_irqs = gpio.iter().fold(HashSet::new(), |mut acc, (_, pins)| {
        for (_, config) in pins.iter() {
            let Some(pfunc) = &config.pfunc else { continue };

            if pfunc.starts_with("IRQ") {
                let gpio = format!("GPIO_{pfunc}");
                let gpio_ident = format_ident!("{gpio}");
                acc.insert((gpio, gpio_ident));
            }
        }
        acc
    });

    peripheral_list.extend(gpio_peris);
    peripheral_list.extend(
        gpio_irqs
            .into_iter()
            .map(|(name, irq)| (name, quote!(#irq))),
    );

    peripheral_list.sort_by(|a, b| a.0.cmp(&b.0));
    let peripheral_list = peripheral_list.into_iter().map(|p| p.1).collect::<Vec<_>>();

    let peripherals = quote! {
        embassy_hal_internal::peripherals_definition!(
                #(#peripheral_list),*
        );

        embassy_hal_internal::peripherals_struct!(
                #(#peripheral_list),*
        );
    };
    let peripherals = pretty_print(&peripherals);

    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("peripherals.rs"), peripherals)?;

    Ok(())
}

fn inner_main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=meta/pinmap.yaml");
    println!("cargo::rerun-if-changed=meta/peripherals.yaml");
    println!("cargo::rerun-if-changed=meta/interrupts.yaml");

    let pin_yaml = std::fs::read_to_string("meta/pinmap.yaml")?;
    let pin_map = serde_yaml::from_str::<PinMap>(&pin_yaml)?;

    let peri_yaml = std::fs::read_to_string("meta/peripherals.yaml")?;
    let peri_map = serde_yaml::from_str::<Peripherals>(&peri_yaml)?;

    let irq_yaml = std::fs::read_to_string("meta/interrupts.yaml")?;
    let irq_map = serde_yaml::from_str::<Interrupts>(&irq_yaml)?;

    generate_pinmap(&pin_map)?;
    generate_interrupt_mod(&irq_map)?;
    generate_peripherals(&pin_map, &peri_map)?;

    Ok(())
}
