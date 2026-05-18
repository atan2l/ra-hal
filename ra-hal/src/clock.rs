//! Clock information and configuration.

#[cfg_attr(ra2a1, path = "clock/ra2a1.rs")]
#[cfg_attr(ra4m1, path = "clock/ra4m1.rs")]
#[cfg_attr(ra4l1, path = "clock/ra4l1.rs")]
#[cfg_attr(ra6m5, path = "clock/ra6m5.rs")]
#[cfg_attr(ra8m1, path = "clock/ra8m1.rs")]
mod _clock;

pub(crate) use _clock::init;
use embassy_sync::once_lock::OnceLock;
use fugit::{HertzU32, KilohertzU32, MegahertzU32};

use crate::pac;

static CLOCK_STATUS: OnceLock<ClockStatus> = OnceLock::new();
const _1MHZ: HertzU32 = MegahertzU32::from_raw(1).convert();

/// Current state of the clocks.
pub struct ClockStatus {
    /// Input frequency for `ICLK`.
    pub master: HertzU32,

    /// Main Oscillator frequency
    pub mosc: Option<HertzU32>,

    /// Is the 32,768 Hz sub-clock oscillator installed?
    pub sosc: bool,

    /// High-speed On-Chip Oscillator frequency (`HOCO`).
    pub hoco: HertzU32,

    /// PLL frequency (if enabled).
    #[cfg(all(pll, not(ra8m1)))]
    pub pll: Option<HertzU32>,
    /// PLL frequencies (if enabled).
    #[cfg(all(pll, ra8m1))]
    pub pll: Option<(HertzU32, HertzU32, HertzU32)>,

    /// PLL2 frequency (if enabled).
    #[cfg(all(pll2, not(ra8m1)))]
    pub pll2: Option<HertzU32>,
    /// PLL2 frequencies (if enabled).
    #[cfg(all(pll2, ra8m1))]
    pub pll2: Option<(HertzU32, HertzU32, HertzU32)>,

    #[cfg(bclk)]
    pub bus_clock: HertzU32,

    /// System clock frequency (`ICLK`).
    pub system: HertzU32,

    /// Flash interface clock (`FCLK`).
    pub flash: HertzU32,

    /// Peripheral Clock "A" (`PCLKA`).
    #[cfg(pclka)]
    pub peripheral_a: HertzU32,

    /// Peripheral Clock "B" (`PCLKB`).
    #[cfg(pclkb)]
    pub peripheral_b: HertzU32,

    /// Peripheral Clock "C" (`PCLKC`).
    #[cfg(pclkc)]
    pub peripheral_c: HertzU32,

    /// Peripheral Clock "D" (`PCLKD`).
    #[cfg(pclkd)]
    pub peripheral_d: HertzU32,

    /// Peripheral Clock "E" (`PCLKE`).
    #[cfg(pclke)]
    pub peripheral_e: HertzU32,

    /// USB Clock (`UCLK`)
    pub usb: Option<HertzU32>,
}

/// Indicates what clock source the system clock (`ICLK`) should derive from.
///
/// # TODO
/// Cortex-M85 chip support.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SystemClockSource {
    /// System clock source is Main Oscillator (`MOSC`).
    Mosc,

    /// System clock source is the 32.768 kHz Sub-clock Oscillator (`SOSC`).
    Sosc,

    /// System clock source is the High-speed On Chip Oscillator (`HOCO`).
    Hoco,

    /// System clock source is the 8 MHz Medium-speed On Chip Oscillator (`MOCO`).
    Moco,

    /// System clock source is the 32.768 kHz Low-speed On Chip Oscillator (`LOCO`).
    #[cfg(not(ra8m1))]
    Loco,

    #[cfg(all(not(ra8m1), pll))]
    /// System clock source is `PLL`.
    Pll,

    #[cfg(ra8m1)]
    Pll1P,
}

/// Intended clock configuration.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone)]
pub struct ClockConfig {
    system: SystemClockSource,

    /// High speed On Chip Oscillator frequency.
    hoco: HocoFrequency,

    /// Main Clock Oscillator frequency (external, 1–20 MHz)
    mosc: Option<HertzU32>,

    /// Enable Sub-clock Oscillator? (external, 32.768 kHz)
    sosc: bool,

    /// Enable Phase Locked Loop?
    #[cfg(pll)]
    pll: Option<PllConfig>,

    /// Enable Phase Locked Loop №2?
    #[cfg(pll2)]
    pll2: Option<PllConfig>,
}

cfg_select! {
    not(ra8m1) => {
        #[cfg(all(pll, not(ra4m1)))]
        pub use _clock::PllInDiv;
        #[cfg(ra4m1)]
        pub use _clock::PllOutDiv;
        #[cfg(any(pll, pll2))]
        pub use _clock::{PllInput, PllOutMul};

        /// Configuration for a Phase Locked Loop.
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        #[cfg(any(pll, pll2))]
        #[derive(Debug, Clone)]
        pub struct PllConfig {
            input: PllInput,
            #[cfg(not(ra4m1))]
            div: PllInDiv,
            #[cfg(ra4m1)]
            div: PllOutDiv,
            mul: PllOutMul,
        }
    },
    ra8m1 => {
        pub use _clock::PllInDiv;
        pub use _clock::{PllInput, PllPDiv, PllQDiv, PllRDiv, PllOutMul};

        /// Configuration for a Phase Locked Loop.
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        #[derive(Debug, Clone)]
        pub struct PllConfig {
            /// Reference clock
            input: PllInput,
            /// Input clock divider
            div: PllInDiv,
            /// Multiplication ratio
            mul: PllOutMul,
            /// Output clock divider
            div_p: PllPDiv,
            /// Output clock divider
            div_q: PllQDiv,
            /// Output clock divider
            div_r: PllRDiv,
        }
    }
}

/// Returns the active state of the clock tree.
pub fn clock_status() -> &'static ClockStatus {
    CLOCK_STATUS
        .try_get()
        .expect("CLOCK_STATUS not initialized")
}

#[cfg(feature = "defmt")]
fn print_frequency(fmt: defmt::Formatter, label: &str, frequency: HertzU32) {
    if frequency < _1MHZ {
        let pll: KilohertzU32 = frequency.convert();
        defmt::write!(fmt, ", {}: {}", label, pll);
    } else {
        let pll: MegahertzU32 = frequency.convert();
        defmt::write!(fmt, ", {}: {}", label, pll);
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ClockStatus {
    fn format(&self, fmt: defmt::Formatter) {
        let system = pac::SYSTEM;
        let cksel = system.sckscr().read().cksel();

        let &ClockStatus {
            master: _,
            system,
            flash,
            #[cfg(pclka)]
            peripheral_a,
            #[cfg(pclkb)]
            peripheral_b,
            #[cfg(pclkc)]
            peripheral_c,
            #[cfg(pclkd)]
            peripheral_d,
            #[cfg(pclke)]
            peripheral_e,
            hoco,
            #[cfg(pll)]
            pll,
            #[cfg(pll2)]
            pll2,
            #[cfg(bclk)]
            bus_clock,
            sosc,
            mosc,
            usb,
        } = clock_status();

        defmt::write!(fmt, "SYSTEM: ");

        if hoco < _1MHZ {
            let hoco: KilohertzU32 = hoco.convert();
            defmt::write!(fmt, "HOCO: {}", hoco);
        } else {
            let hoco: MegahertzU32 = hoco.convert();
            defmt::write!(fmt, "HOCO: {}", hoco);
        }
        defmt::write!(fmt, ", SOSC: {}", sosc);

        match mosc {
            Some(mosc) => {
                print_frequency(fmt, "MOSC", mosc.convert());
            }
            None => {
                defmt::write!(fmt, ", MOSC: None");
            }
        }

        #[cfg(pll)]
        cfg_select! {
            not(ra8m1) => {
                match pll {
                    Some(pll) => print_frequency(fmt, "PLL", pll),
                    None => defmt::write!(fmt, ", PLL: OFF"),
                }
            }
            ra8m1 => {
                match pll {
                    Some((pll_p, pll_q, pll_r)) => {
                        print_frequency(fmt, "PLL.P", pll_p);
                        print_frequency(fmt, "PLL.Q", pll_q);
                        print_frequency(fmt, "PLL.R", pll_r);
                    }
                    None => defmt::write!(fmt, ", PLL: OFF"),
                }
            }
        }

        #[cfg(pll2)]
        cfg_select! {
            not(ra8m1) => {
                match pll2 {
                    Some(pll) => print_frequency(fmt, "PLL2", pll),
                    None => defmt::write!(fmt, ", PLL2: OFF"),
                }
            }
            ra8m1 => {
                match pll2 {
                    Some((pll_p, pll_q, pll_r)) => {
                        print_frequency(fmt, "PLL2.P", pll_p);
                        print_frequency(fmt, "PLL2.Q", pll_q);
                        print_frequency(fmt, "PLL2.R", pll_r);
                    }
                    None => defmt::write!(fmt, ", PLL2: OFF"),
                }
            }
        }

        defmt::write!(fmt, ", ROOT: {}", cksel);

        print_frequency(fmt, "ICLK", system.convert());
        print_frequency(fmt, "FCLK", flash.convert());

        #[cfg(bclk)]
        print_frequency(fmt, "BCLK", bus_clock.convert());

        #[cfg(pclka)]
        print_frequency(fmt, "PCLKA", peripheral_a.convert());

        #[cfg(pclkb)]
        print_frequency(fmt, "PCLKB", peripheral_b.convert());

        #[cfg(pclkc)]
        print_frequency(fmt, "PCLKC", peripheral_c.convert());

        #[cfg(pclkd)]
        print_frequency(fmt, "PCLKD", peripheral_d.convert());

        #[cfg(pclke)]
        print_frequency(fmt, "PCLKE", peripheral_e.convert());

        if let Some(usb) = usb {
            let usb: MegahertzU32 = usb.convert();
            defmt::write!(fmt, ", USB: {}", usb);
        } else {
            defmt::write!(fmt, ", USB: OFF");
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/hoco.rs"));
