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
    #[cfg(pll)]
    pub pll: Option<HertzU32>,

    /// PLL2 frequency (if enabled).
    #[cfg(pll2)]
    pub pll2: Option<HertzU32>,

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
    Loco,

    #[cfg(pll)]
    /// System clock source is `PLL`.
    Pll,
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

/// Returns the active state of the clock tree.
pub fn clock_status() -> &'static ClockStatus {
    CLOCK_STATUS.try_get().unwrap()
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
            hoco,
            #[cfg(pll)]
            pll,
            #[cfg(pll2)]
            pll2,
            #[cfg(bclk)]
            bus_clock,
            sosc,
            mosc,
        } = clock_status();

        let _1mhz: HertzU32 = 1_u32.MHz();

        defmt::write!(fmt, "SYSTEM: ");

        if hoco < _1mhz {
            let hoco: KilohertzU32 = hoco.convert();
            defmt::write!(fmt, "HOCO: {}", hoco);
        } else {
            let hoco: MegahertzU32 = hoco.convert();
            defmt::write!(fmt, "HOCO: {}", hoco);
        }
        defmt::write!(fmt, ", SOSC: {}", sosc);

        match mosc {
            Some(mosc) => {
                if mosc < _1mhz {
                    let mosc: KilohertzU32 = mosc.convert();
                    defmt::write!(fmt, ", MOSC: {}", mosc);
                } else {
                    let mosc: MegahertzU32 = mosc.convert();
                    defmt::write!(fmt, ", MOSC: {}", mosc);
                }
            }
            None => {
                defmt::write!(fmt, ", MOSC: None");
            }
        }

        #[cfg(pll)]
        match pll {
            Some(pll) => {
                if pll < _1mhz {
                    let pll: KilohertzU32 = pll.convert();
                    defmt::write!(fmt, ", PLL: {}", pll);
                } else {
                    let pll: MegahertzU32 = pll.convert();
                    defmt::write!(fmt, ", PLL: {}", pll);
                }
            }
            None => {
                defmt::write!(fmt, ", PLL: OFF");
            }
        }

        #[cfg(pll2)]
        match pll2 {
            Some(pll) => {
                if pll < _1mhz {
                    let pll: KilohertzU32 = pll.convert();
                    defmt::write!(fmt, ", PLL2: {}", pll);
                } else {
                    let pll: MegahertzU32 = pll.convert();
                    defmt::write!(fmt, ", PLL2: {}", pll);
                }
            }
            None => {
                defmt::write!(fmt, ", PLL2: OFF");
            }
        }

        defmt::write!(fmt, ", ROOT: {}", cksel);

        if system < _1mhz {
            let iclk: KilohertzU32 = system.convert();
            defmt::write!(fmt, ", ICLK: {}", iclk);
        } else {
            let iclk: MegahertzU32 = system.convert();
            defmt::write!(fmt, ", ICLK: {}", iclk);
        }

        if flash < _1mhz {
            let fclk: KilohertzU32 = flash.convert();
            defmt::write!(fmt, ", FCLK: {}", fclk);
        } else {
            let fclk: MegahertzU32 = flash.convert();
            defmt::write!(fmt, ", FCLK: {}", fclk);
        }

        #[cfg(bclk)]
        if bus_clock < _1mhz {
            let bclk: KilohertzU32 = bus_clock.convert();
            defmt::write!(fmt, ", BCLK: {}", bclk);
        } else {
            let bclk: MegahertzU32 = bus_clock.convert();
            defmt::write!(fmt, ", BCLK: {}", bclk);
        }

        #[cfg(pclka)]
        if peripheral_a < _1mhz {
            let pclk_a: KilohertzU32 = peripheral_a.convert();
            defmt::write!(fmt, ", PCLKA: {}", pclk_a);
        } else {
            let pclk_a: MegahertzU32 = peripheral_a.convert();
            defmt::write!(fmt, ", PCLKA: {}", pclk_a);
        }

        #[cfg(pclkb)]
        if peripheral_b < _1mhz {
            let pclk_b: KilohertzU32 = peripheral_b.convert();
            defmt::write!(fmt, ", PCLKB: {}", pclk_b);
        } else {
            let pclk_b: MegahertzU32 = peripheral_b.convert();
            defmt::write!(fmt, ", PCLKB: {}", pclk_b);
        }

        #[cfg(pclkc)]
        if peripheral_c < _1mhz {
            let pclk_c: KilohertzU32 = peripheral_c.convert();
            defmt::write!(fmt, ", PCLKC: {}", pclk_c);
        } else {
            let pclk_c: MegahertzU32 = peripheral_c.convert();
            defmt::write!(fmt, ", PCLKC: {}", pclk_c);
        }

        #[cfg(pclkd)]
        if peripheral_d < _1mhz {
            let pclk_d: KilohertzU32 = peripheral_d.convert();
            defmt::write!(fmt, ", PCLKD: {}", pclk_d);
        } else {
            let pclk_d: MegahertzU32 = peripheral_d.convert();
            defmt::write!(fmt, ", PCLKD: {}", pclk_d);
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/hoco.rs"));
