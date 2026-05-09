use std::{
    borrow::Cow, collections::HashSet, env, fmt, fs, path::PathBuf, str::FromStr as _,
    sync::LazyLock,
};

use convert_case::{Boundary, Case, Casing};
use indexmap::{IndexMap, IndexSet};
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, format_ident, quote};
use ra_metapac::metadata::{Metadata, Variant};
use regex::Regex;

/// Matches an enabled pin count feature
static RE_PACKAGE_FEATURE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^(?<count>\d+)(?<package>BGA|LGA|LQFP|QFN|QFP)$"#).unwrap());

static RE_PERIPHERAL_INSTANCE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^(?<kind>[A-Z_]+(\d{2})?)(_?(?<instance>\d+))?(?<insecure>_NS)?$"#).unwrap()
});

static RE_SCI_RX_TX_SIGNAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^SCI(?<instance>\d)_(?<signal>RX|TX)D$"#).unwrap());

static PORT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^P(?<port>[0-9A-F])(?<pin>[0-9]{2})$"#).unwrap());

static RE_ADC_SIGNAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^ADC1[246]_(?<instance>\d)_AN(?<channel>\d+)"#).unwrap());

static RE_CHIP_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^RA[02468][ADELMTP][1-9]$"#).unwrap());

type Features = Vec<String>;

#[derive(Debug, Copy, Clone)]
enum MstpPeripheral {
    A,
    B,
    C,
    D,
    E,
}

#[derive(Debug)]
enum GetOneError {
    None(&'static str),
    Multiple(&'static str),
}

trait IteratorExt: Iterator {
    fn get_one(self, context: &'static str) -> Result<Self::Item, GetOneError>;
}

impl fmt::Display for GetOneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetOneError::None(context) => {
                write!(f, "{context} One required, found none.")
            }
            GetOneError::Multiple(context) => {
                write!(f, "{context} One required, found multiple.")
            }
        }
    }
}

impl std::error::Error for GetOneError {}
impl<T: Iterator> IteratorExt for T {
    fn get_one(mut self, context: &'static str) -> Result<Self::Item, GetOneError> {
        match self.next() {
            None => Err(GetOneError::None(context)),
            Some(res) => match self.next() {
                Some(_) => Err(GetOneError::Multiple(context)),
                None => Ok(res),
            },
        }
    }
}

/// Beautify output from `quote!`
fn pretty_print(ts: &proc_macro2::TokenStream) -> String {
    let file = syn::parse_file(&ts.to_string()).unwrap();
    prettyplease::unparse(&file)
}

// fn do_dac(_peripheral: &str, signals: &PinEntry) -> Vec<TokenStream> {
//     signals
//         .iter()
//         .filter(|(signal, _)| signal.starts_with("DA"))
//         .fold(vec![], |mut acc, (_signal, pins)| {
//             for (pin, config) in pins.iter() {
//                 let pin = format_ident!("{}", pin);
//                 let conditions = config.pin_conditional();

//                 acc.push(quote! {
//                     #conditions
//                     crate::dac::dac_pin!(#pin);
//                 });
//             }

//             acc
//         })
// }

fn generate_interrupt_mod(metadata: &Metadata) -> anyhow::Result<()> {
    let interrupts = metadata.interrupts.iter().map(|irq| format_ident!("{irq}"));
    let num_interrupts = Literal::usize_unsuffixed(metadata.num_interrupts);

    let mut tokens = quote! {
        #[allow(clippy::missing_safety_doc)]
        mod _interrupt {
            embassy_hal_internal::interrupt_mod!(
                #(#interrupts),*
            );
        }
        pub use _interrupt::interrupt;

        /// Number of interrupts exposed to the `NVIC`.
        pub const NUM_INTERRUPTS : usize = #num_interrupts;
    };

    let dtc_links = (0..metadata.num_interrupts).map(|int| {
        let int = Literal::usize_unsuffixed(int);
        quote! {
            crate::dtc::dtc_link!(#int);
        }
    });
    tokens.extend(dtc_links);

    let tokens = pretty_print(&tokens);

    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("interrupts.rs"), tokens)?;

    Ok(())
}

impl ToTokens for MstpPeripheral {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // We're using features because the cfg bits are defined in this build script and not yet available.

        let ident = match self {
            MstpPeripheral::A => {
                cfg_select! {
                    feature = "ra2a1" => format_ident!("SYSTEM"),
                    feature = "ra4m1" => format_ident!("SYSTEM"),
                    _ => format_ident!("MSTP"),
                }
            }
            MstpPeripheral::B => format_ident!("MSTP"),
            MstpPeripheral::C => format_ident!("MSTP"),
            MstpPeripheral::D => format_ident!("MSTP"),
            MstpPeripheral::E => format_ident!("MSTP"),
        };
        tokens.extend(quote! {#ident})
    }
}

fn generate_module_stops(metadata: &Metadata) -> anyhow::Result<()> {
    let mstp_impls = metadata
        .peripherals
        .iter()
        .filter(|peri| peri.stop.is_some())
        .fold(vec![], |mut acc, peri| {
            let peri_ident = format_ident!("{}", peri.name);
            let mstp = peri.stop.as_ref().unwrap();
            let mstp_peri = match mstp.mstp {
                "A" => MstpPeripheral::A,
                "B" => MstpPeripheral::B,
                "C" => MstpPeripheral::C,
                "D" => MstpPeripheral::D,
                "E" => MstpPeripheral::E,
                mstp => unreachable!("Unknown MSTP: {mstp:?}"),
            };

            let field_prefix = match mstp_peri {
                MstpPeripheral::A => "a",
                MstpPeripheral::B => "b",
                MstpPeripheral::C => "c",
                MstpPeripheral::D => "d",
                MstpPeripheral::E => "e",
            };

            let mstp_write = format_ident!("set_mstp{field_prefix}{}", mstp.bit);
            let mstp_read = format_ident!("mstp{field_prefix}{}", mstp.bit);

            let mstp_reg = format_ident!("mstpcr{field_prefix}");

            acc.push(quote! {
                impl crate::module_stop::ModuleStop for crate::peripherals::#peri_ident {
                    #[inline(always)]
                    fn start_module() {
                        debug!("{}: stop=false", stringify!(#peri_ident));
                        let mstp = crate::pac::#mstp_peri;
                        mstp.#mstp_reg().write(|r| r.#mstp_write(false));
                    }

                    #[inline(always)]
                    fn stop_module() {
                        debug!("{}: stop=true", stringify!(#peri_ident));
                        let mstp = crate::pac::#mstp_peri;
                        mstp.#mstp_reg().write(|r| r.#mstp_write(true));
                    }

                    #[inline(always)]
                    fn active() -> bool {
                        let mstp = crate::pac::#mstp_peri;
                        mstp.#mstp_reg().read().#mstp_read()
                    }
                }

                impl crate::module_stop::SealedModuleStop for crate::peripherals::#peri_ident {}
            });

            acc
        });

    let mstp_impls = quote! {
        #(#mstp_impls)*
    };

    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("module_stops.rs"), mstp_impls.to_string())?;

    Ok(())
}

fn generate_event_link_enum(metadata: &Metadata) -> anyhow::Result<()> {
    let mut events = metadata
        .events
        .iter()
        .filter(|(_id, event)| event.interrupt)
        .map(|(_id, event)| event)
        .collect::<Vec<_>>();
    events.sort_by(|a, b| a.name.cmp(b.name));

    let event_tokens = events.iter().map(|event| {
        let name = format_ident!("{}", event.name);
        let value = Literal::from_str(&format!("0x{:02X}", event.value)).unwrap();
        let doc = event.description;
        quote! {
            #[doc = #doc]
            #name = #value
        }
    });

    let tokens = quote! {
        #[allow(unused, missing_docs)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        #[repr(u16)]
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub enum InterruptEvent {
            None = 0x00,
            #(#event_tokens),*
        }
    };

    let contents = pretty_print(&tokens);
    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("event_link.rs"), contents)?;

    Ok(())
}

fn generate_peripherals(metadata: &Metadata, package: String) -> anyhow::Result<()> {
    let mut contents = quote! {};

    // List of concrete peripherals
    let mut peripheral_list = metadata
        .peripherals
        .iter()
        .filter_map(|peripheral| peripheral.block.map(|_block| peripheral.name))
        .map(Cow::Borrowed)
        .collect::<Vec<_>>();

    // List of all valid pins for this package
    let mut all_pins = metadata
        .pins
        .iter()
        .filter(|(_name, pin)| pin.packages.contains(&package.as_str()))
        .collect::<IndexMap<_, _>>();

    all_pins.sort_keys();

    peripheral_list.sort();

    let unique_gpio_irqs = all_pins
        .values()
        .filter_map(|pin| pin.irq)
        .filter_map(|irq| irq.strip_prefix("GPIO_IRQ"))
        .filter(|irq| !irq.ends_with("-DS"))
        .collect::<HashSet<&str>>();

    for gpio_irq in unique_gpio_irqs.iter() {
        let irq_num = gpio_irq
            .parse::<usize>()
            .unwrap_or_else(|_| panic!("Couldn't parse GPIO_IRQ: {gpio_irq:?}"));
        let irq_lit = Literal::usize_unsuffixed(irq_num);
        contents.extend(quote! {
            crate::gpio::declare_port_irq!(#irq_lit);
        });
    }

    for (name, pin) in all_pins.iter() {
        // TODO: Validate P[0-9A-F]\d{2}
        if !name.starts_with("P") {
            continue;
        }
        eprintln!("{name:?} {pin:?}");
        let pin_ident = format_ident!("{name}");

        assert_eq!(name.len(), 4, "{name:?}");
        let Some(port_captures) = PORT_RE.captures(name) else {
            panic!("Couldn't parse port: {name:?}");
        };
        let port_name = port_captures.name("port").unwrap().as_str();
        let port_number = u32::from_str_radix(port_name, 16).unwrap();
        let port_number_lit = Literal::usize_unsuffixed(port_number as _);
        let pin_number = port_captures
            .name("pin")
            .unwrap()
            .as_str()
            .parse::<usize>()?;
        let port_ident = format_ident!("PORT{port_name}");
        let pin_number_lit = Literal::usize_unsuffixed(pin_number);

        eprintln!("{}, port={}, pin={}", pin_ident, port_ident, pin_number_lit);
        contents.extend(quote! {
            crate::gpio::gpio_pin!(#pin_ident, #port_number_lit, #pin_number_lit, #port_ident);
        });

        if pin.pull_up {
            contents.extend(quote! {
                impl crate::gpio::PullUpPin for crate::peripherals::#pin_ident {}
                impl crate::gpio::SealedPullUpPin for crate::peripherals::#pin_ident {}
            });
        }

        if pin.open_drain {
            contents.extend(quote! {
                impl crate::gpio::OpenDrainPin for crate::peripherals::#pin_ident {}
                impl crate::gpio::SealedOpenDrainPin for crate::peripherals::#pin_ident {}
            });
        }

        if let Some(gpio_irq) = pin.irq {
            if !gpio_irq.starts_with("GPIO") || gpio_irq.ends_with("-DS") {
                continue;
            }

            let event_ident = format_ident!("{}", gpio_irq.to_case(Case::Pascal));
            let irq_ident = format_ident!("{gpio_irq}");
            contents.extend(quote! {
                    impl crate::gpio::InterruptiblePin for crate::peripherals::#pin_ident {}

                    impl crate::gpio::SealedIntPin for crate::peripherals::#pin_ident {
                        const INTERRUPT_EVENT: crate::event_link::InterruptEvent = crate::event_link::InterruptEvent::#event_ident;

                        fn waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
                            static WAKER: embassy_sync::waitqueue::AtomicWaker = embassy_sync::waitqueue::AtomicWaker::new();
                            &WAKER
                        }
                    }

                    impl crate::gpio::GpioIrq<crate::peripherals::#pin_ident> for crate::peripherals::#irq_ident {}
                });
        }
    }

    for peripheral in peripheral_list.iter() {
        let peripheral = metadata
            .peripherals
            .iter()
            .find(|p| p.name == peripheral)
            .unwrap();
        if peripheral.name.ends_with("_NS") {
            continue;
        }
        let captures = RE_PERIPHERAL_INSTANCE
            .captures(peripheral.name)
            .unwrap_or_else(|| panic!("No captures for {}", peripheral.name));
        let peripheral_kind = captures.name("kind").unwrap().as_str();
        let instance = captures.name("instance").map(|c| c.as_str());
        let peripheral_ident = format_ident!("{}", peripheral.name);

        match (
            peripheral_kind,
            peripheral.block.expect("Shouldn't be here"),
        ) {
            ("ADC12", _driver) | ("ADC14", _driver) | ("ADC16", _driver) => {
                for signal in peripheral.signals.iter() {
                    if let Some(captures) = RE_ADC_SIGNAL.captures(signal) {
                        let channel = captures
                            .name("channel")
                            .unwrap()
                            .as_str()
                            .parse::<u8>()
                            .unwrap();

                        let channel = Literal::u8_unsuffixed(channel);

                        let pins = &metadata.signals[signal];
                        let pins = pins.iter().filter(|pin| {
                            metadata.pins[pin.pin].packages.contains(&package.as_str())
                        });

                        for pin_config in pins {
                            let pin_ident = format_ident!("{}", pin_config.pin);
                            contents.extend(quote! {
                                crate::adc::channel::adc_pin!(#channel, #pin_ident);
                            });
                        }
                    }
                }
            }
            ("DAC12", _driver) => {
                for signal in peripheral.signals.iter() {
                    let pins = &metadata
                        .signals
                        .get(signal)
                        .unwrap_or_else(|| panic!("Couldn't find signal: {signal}"));
                    let pins = pins
                        .iter()
                        .filter(|pin| metadata.pins[pin.pin].packages.contains(&package.as_str()));

                    for pin_config in pins {
                        let pin_ident = format_ident!("{}", pin_config.pin);
                        contents.extend(quote! {
                            crate::dac::dac_pin!(#pin_ident);
                        });
                    }
                }
            }
            ("GPT16" | "GPT32", _driver) => {
                let Some(instance) = instance else {
                    panic!("GPT with no instance");
                };

                // ($peripheral:ident, $width:ident, $index:literal, $cmpa_int:ident, $cmpb_int:ident, $cmpc_int:ident, $overflow_int:ident, $underflow_int:ident) => {
                let width_ident = match peripheral_kind {
                    "GPT16" => format_ident!("u16"),
                    "GPT32" => format_ident!("u32"),
                    _ => unreachable!(),
                };

                let index_lit = Literal::usize_unsuffixed(instance.parse().unwrap());

                let ccmpa_ident = format_ident!(
                    "{}",
                    metadata.events[format!("event.gpt{instance}.capture.compare.a").as_str()].name
                );

                let ccmpb_ident = format_ident!(
                    "{}",
                    metadata.events[format!("event.gpt{instance}.capture.compare.b").as_str()].name
                );

                let cmpc_ident = format_ident!(
                    "{}",
                    metadata.events[format!("event.gpt{instance}.compare.c").as_str()].name
                );

                let ovf_ident = format_ident!(
                    "{}",
                    metadata.events[format!("event.gpt{instance}.counter.overflow").as_str()].name
                );

                let udf_ident = format_ident!(
                    "{}",
                    metadata.events[format!("event.gpt{instance}.counter.underflow").as_str()].name
                );

                contents.extend(quote! {
                    crate::timer_gpt::timer_instance!(#peripheral_ident, #width_ident, #index_lit, #ccmpa_ident, #ccmpb_ident, #cmpc_ident, #ovf_ident, #udf_ident);
                    crate::pwm::gpt_instance!(#peripheral_ident, #ccmpa_ident, #ccmpb_ident, #cmpc_ident, #ovf_ident, #udf_ident);
                });

                let pwm_signals = peripheral
                    .signals
                    .iter()
                    .filter(|signal| signal.ends_with("_GTIOCA") || signal.ends_with("_GTIOCB"));

                for signal in pwm_signals {
                    let pins = &metadata
                        .signals
                        .get(signal)
                        .unwrap_or_else(|| panic!("Couldn't find signal: {signal:?}"));
                    let pins = pins
                        .iter()
                        .filter(|pin| metadata.pins[pin.pin].packages.contains(&package.as_str()));

                    for pin_config in pins {
                        let pin_ident = format_ident!("{}", pin_config.pin);
                        let pfunc = format_ident!(
                            "{}",
                            pin_config.pfunc.unwrap().to_string().to_case(Case::Pascal)
                        );

                        let channel = signal.chars().next_back().unwrap();
                        let channel_ident = format_ident!("Chan{channel}");
                        contents.extend(quote! {
                            crate::pwm::pwm_pin!(#peripheral_ident, #channel_ident, #pin_ident, #pfunc);
                        });
                    }
                }
            }
            ("IIC", _driver) => {
                let Some(instance) = instance else {
                    panic!("IIC with no instance");
                };
                let instance = instance.parse::<usize>()?;

                let rx_int_ident = format_ident!("Iic{instance}Rxi");
                let te_int_ident = format_ident!("Iic{instance}Tei");
                let tx_int_ident = format_ident!("Iic{instance}Txi");

                contents.extend(quote! {
                   crate::i2c::instance_impl!(#peripheral_ident, #rx_int_ident, #te_int_ident, #tx_int_ident);
                });

                for signal in peripheral.signals.iter() {
                    let pins = &metadata
                        .signals
                        .get(signal)
                        .unwrap_or_else(|| panic!("Couldn't find {signal:?}"));
                    let pins = pins
                        .iter()
                        .filter(|pin| metadata.pins[pin.pin].packages.contains(&package.as_str()));

                    for pin_config in pins {
                        let pin_ident = format_ident!("{}", pin_config.pin);
                        let pfunc = format_ident!(
                            "{}",
                            pin_config
                                .pfunc
                                .unwrap()
                                .to_string()
                                .remove_boundaries(&[Boundary::DigitUpper])
                                .to_case(Case::Pascal)
                        );

                        let signal_ident = match signal.split_once("_").unwrap().1 {
                            "SCL" => format_ident!("scl_pin"),
                            "SDA" => format_ident!("sda_pin"),
                            _ => continue,
                        };

                        contents.extend(quote! {
                            crate::i2c::#signal_ident!(#peripheral_ident, #pin_ident, #pfunc);
                        });
                    }
                }
            }
            ("SCI", driver) => {
                // Skip non-fifo instances for now
                if !driver.contains("SciFifo") {
                    continue;
                }

                let Some(instance) = instance else {
                    panic!("IIC with no instance");
                };
                let instance = instance.parse::<usize>()?;

                let rx_int_ident = format_ident!("Sci{instance}Rxi");
                let te_int_ident = format_ident!("Sci{instance}Tei");
                let tx_int_ident = format_ident!("Sci{instance}Txi");

                contents.extend(quote! {
                   crate::uart::instance_impl!(#peripheral_ident, #rx_int_ident, #te_int_ident, #tx_int_ident);
                });

                for signal in peripheral.signals.iter() {
                    if let Some(captures) = RE_SCI_RX_TX_SIGNAL.captures(signal) {
                        let pins = &metadata
                            .signals
                            .get(signal)
                            .unwrap_or_else(|| panic!("Couldn't find {signal:?}"));

                        let peri_pins = pins
                            .iter()
                            .filter(|pin_config| all_pins.contains_key(&pin_config.pin));

                        for pin_config in peri_pins {
                            let pin_ident = format_ident!("{}", pin_config.pin);
                            let pfunc = format_ident!(
                                "{}",
                                pin_config.pfunc.unwrap().to_string().to_case(Case::Pascal)
                            );
                            let signal_ident = format_ident!(
                                "{}_pin",
                                captures.name("signal").unwrap().as_str().to_lowercase()
                            );
                            contents.extend(quote! {
                                crate::uart::#signal_ident!(#peripheral_ident, #pin_ident, #pfunc);
                            });
                        }
                    }
                }
            }
            ("SPI", _driver) => {
                for signal in peripheral.signals.iter() {
                    let pins = &metadata.signals[signal];
                    let pins = pins
                        .iter()
                        .filter(|pin| metadata.pins[pin.pin].packages.contains(&package.as_str()));

                    for pin_config in pins {
                        let pin_ident = format_ident!("{}", pin_config.pin);
                        let pfunc = format_ident!(
                            "{}",
                            pin_config.pfunc.unwrap().to_string().to_case(Case::Pascal)
                        );

                        let signal_ident = match signal.split_once("_").unwrap().1 {
                            "MISO" => format_ident!("miso_pin"),
                            "MOSI" => format_ident!("mosi_pin"),
                            "RSPCK" => format_ident!("sck_pin"),
                            "SSL0" => format_ident!("cs_pin"),
                            _ => continue,
                        };

                        contents.extend(quote! {
                            crate::spi::#signal_ident!(#peripheral_ident, #pin_ident, #pfunc);
                        });
                    }
                }
            }
            (_kind, _driver) => {
                // println!(
                //     "cargo::warning=Skipping peri={}, class={kind}, driver={driver}",
                //     peripheral.name
                // );
            }
        }
    }

    // Let's add the pseudo-peripherals
    {
        for irq in 0..metadata.interrupts.len() {
            peripheral_list.push(Cow::Owned(format!("DTC_CHAN{irq}")));
        }

        let gpio_irqs = all_pins
            .values()
            .filter_map(|pin| pin.irq)
            .map(Cow::Borrowed)
            .collect::<HashSet<_>>();
        peripheral_list.extend(gpio_irqs);

        peripheral_list.extend(all_pins.keys().map(|s| Cow::Borrowed(**s)));
    }
    peripheral_list.sort();

    let peripheral_list = peripheral_list
        .into_iter()
        .filter(|p| !p.ends_with("-DS"))
        .map(|p| format_ident!("{p}"))
        .collect::<Vec<_>>();

    contents.extend(quote! {
        embassy_hal_internal::peripherals_definition!(
                #(#peripheral_list),*
        );

        embassy_hal_internal::peripherals_struct!(
                #(#peripheral_list),*
        );
    });

    let contents = pretty_print(&contents);
    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("peripherals.rs"), contents)?;

    Ok(())
}

fn validate_arch(metadata: &Metadata) {
    let arch = match metadata.cpu {
        "CortexM4" => "thumbv7em-none-eabihf",
        "CortexM23" => "thumbv8m.base-none-eabi",
        "CortexM33" | "CortexM85" => "thumbv8m.main-none-eabihf",
        _ => unreachable!(),
    };

    assert_eq!(env::var("TARGET").unwrap(), arch);
}

fn get_package(_metadata: &Metadata, features: &Features) -> anyhow::Result<String> {
    let pin_count = features
        .iter()
        .filter(|feature| RE_PACKAGE_FEATURE.is_match(feature))
        .get_one("pin count")?;

    Ok(pin_count.to_owned())
}

fn set_cfgs(metadata: &Metadata, features: &Features) -> anyhow::Result<()> {
    let cfg_debug = env::var("RA_PRINT_CFG").map(|_| true).unwrap_or(false);

    let common = ra_metapac::common_metadata::COMMON_METADATA;

    for variant in common.cores.iter() {
        println!("cargo::rustc-check-cfg=cfg({variant})");
    }

    for driver in common.drivers.iter() {
        println!("cargo::rustc-check-cfg=cfg({driver})");
    }

    for extra in common.extras.iter() {
        println!("cargo::rustc-check-cfg=cfg({extra})");
    }

    for n in (&[2, 4, 6, 8]).iter() {
        println!("cargo::rustc-check-cfg=cfg(ra{n})");
    }

    let chip_feature = features
        .iter()
        .filter_map(|feature| {
            if RE_CHIP_MATCH.is_match(feature) {
                Some(feature.to_owned())
            } else {
                None
            }
        })
        .get_one("MCU model")?
        .to_lowercase();

    let family_feature = &chip_feature[0..3];

    println!("cargo::rustc-cfg={chip_feature}");
    println!("cargo::rustc-cfg={family_feature}");

    for extra in metadata.extras {
        println!("cargo::rustc-cfg={extra}");
    }

    let drivers = metadata
        .peripherals
        .iter()
        .filter_map(|peri| peri.block)
        .collect::<HashSet<_>>();

    let mut enabled_features = drivers
        .iter()
        .map(|driver| driver.split_once("::").unwrap().0)
        .flat_map(|driver| {
            if let Some(parts) = driver.split_once('_') {
                vec![parts.0, driver]
            } else {
                vec![driver]
            }
        })
        .collect::<IndexSet<_>>();
    enabled_features.sort();

    for feature in enabled_features.iter() {
        println!("cargo::rustc-cfg={feature}");
        if cfg_debug {
            println!("cargo::warning=DRIVER_FEATURE={feature}");
        }
    }

    println!("cargo::rustc-check-cfg=cfg(secure)");

    if features.iter().any(|feature| feature == "SECURE") {
        println!("cargo::rustc-cfg=secure");
    }

    println!("cargo::rustc-check-cfg=cfg(trust_zone)");

    if metadata.trust_zone {
        println!("cargo::rustc-cfg=trust_zone");
    }

    Ok(())
}

fn generate_hoco_rs(metadata: &Metadata) -> anyhow::Result<()> {
    // Clock config
    // Generate HOCO enum
    let hoco_frequencies = metadata
        .hoco_frequencies
        .iter()
        .map(|frequency| format_ident!("_{frequency}mhz"))
        .collect::<Vec<_>>();
    let hoco_docs = metadata
        .hoco_frequencies
        .iter()
        .map(|frequency| format!("ƒHOCO ={frequency} MHz"))
        .collect::<Vec<_>>();
    let hoco_megahertz = metadata
        .hoco_frequencies
        .iter()
        .map(|frequency| {
            let frequency = *frequency as u32;
            quote! { #frequency.MHz() }
        })
        .collect::<Vec<_>>();

    let osm_driver = metadata
        .peripherals
        .iter()
        .filter_map(|peri| peri.block)
        .filter_map(|block| block.strip_prefix("osm_"))
        .filter_map(|block| block.strip_suffix("::Osm"))
        .next()
        .unwrap();

    let hcfrq = match osm_driver {
        "hocov1" => format_ident!("Hcfrq1"),
        "hocov2slow" | "hocov2mid" | "hocov2fast" => format_ident!("Hcfrq0"),
        _ => unimplemented!("OSM driver = {osm_driver}"),
    };
    let hoco_frequencies = quote! {
        use fugit::RateExtU32 as _;

        /// Frequency of the High Speed On-Chip Oscillator (`ƒHOCO`).
        #[derive(Copy, Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum HocoFrequency {
            #(
                #[doc = #hoco_docs]
                #hoco_frequencies
            ),*
        }

        impl From<HocoFrequency> for crate::pac::system::vals::#hcfrq {
            fn from(val: HocoFrequency) -> Self {
                match val {
                    #(HocoFrequency::#hoco_frequencies => Self::#hoco_frequencies),*
                }
            }
        }

        impl From<HocoFrequency> for HertzU32 {
            fn from(val: HocoFrequency) -> Self {
                match val {
                    #(HocoFrequency::#hoco_frequencies => #hoco_megahertz),*
                }
            }
        }
    };
    let hoco_frequencies = pretty_print(&hoco_frequencies);

    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("hoco.rs"), hoco_frequencies)?;

    Ok(())
}

fn main() {
    if let Err(e) = inner_main() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn inner_main() -> anyhow::Result<()> {
    println!("cargo::rerun-if-changed=build.rs");

    let features = env::vars()
        .filter_map(|(var, _value)| var.strip_prefix("CARGO_FEATURE_").map(ToOwned::to_owned))
        .collect::<Vec<_>>();
    let metadata = ra_metapac::metadata::metadata();
    let common = ra_metapac::common_metadata::COMMON_METADATA;

    validate_arch(&metadata);
    set_cfgs(&metadata, &features)?;

    {
        let has_agt = common
            .drivers
            .iter()
            .any(|driver| *driver == "agt" || *driver == "agtw");
        let agt_time_driver = features.iter().any(|feature| *feature == "TIME_DRIVER_AGT");
        assert!(
            !agt_time_driver || has_agt,
            "agt_time_driver={agt_time_driver}, has_agt={has_agt}"
        );
    }

    {
        let has_ulpt = common.drivers.iter().any(|driver| *driver == "ulpt");
        let ulpt_time_driver = features
            .iter()
            .any(|feature| *feature == "TIME_DRIVER_ULPT");
        assert!(
            !ulpt_time_driver || has_ulpt,
            "ulpt_time_driver={ulpt_time_driver}, has_ulpt={has_ulpt}"
        );
    }

    {
        let mut tokens = quote! {};
        for (name, val) in metadata.constants.iter() {
            let name_ident = format_ident!("{name}");
            match val {
                Variant::I32(val) => {
                    let val_lit = Literal::i32_unsuffixed(*val);

                    tokens.extend(quote! { pub const #name_ident : i32 = #val_lit; })
                }
                Variant::U16(val) => {
                    let val_lit = Literal::from_str(&format!("0x{val:04X}")).unwrap();

                    tokens.extend(quote! { pub const #name_ident : u16 = #val_lit; })
                }
                Variant::String(val) => {
                    tokens.extend(quote! { pub const #name_ident : i32 = #val; })
                }
            }
        }

        let tokens = pretty_print(&tokens);

        let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        fs::write(out_dir.join("constants.rs"), tokens)?;
    }

    generate_hoco_rs(&metadata)?;
    generate_module_stops(&metadata)?;
    generate_interrupt_mod(&metadata)?;
    let package = get_package(&metadata, &features)?;
    generate_peripherals(&metadata, package)?;
    generate_event_link_enum(&metadata)?;

    Ok(())
}
