use fugit::{HertzU32, MegahertzU32, RateExtU32 as _};

use crate::clock::{ClockConfig, HocoFrequency, PllConfig, SystemClockSource};

// Max PLL output is 480 MHz

/// Input clock range
const PLL_RAW_INPUT_MIN: HertzU32 = MegahertzU32::from_raw(8).convert();
const PLL_RAW_INPUT_MAX: HertzU32 = MegahertzU32::from_raw(48).convert();

/// Input clock range after division
const PLL_INPUT_MIN: HertzU32 = MegahertzU32::from_raw(6).convert();
const PLL_INPUT_MAX: HertzU32 = MegahertzU32::from_raw(12).convert();

const PLL_OUTPUT_MIN: HertzU32 = MegahertzU32::from_raw(40).convert();
const PLL_OUTPUT_MAX: HertzU32 = MegahertzU32::from_raw(480).convert();

const OUTPUT_FACTOR: u32 = 10;

#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub enum PllInDiv {
    Div1,
    Div2,
    Div3,
    Div4,
}

#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub enum PllInput {
    Hoco,
    Mosc,
}

#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub enum PllPDiv {
    Div2,
    Div4,
    Div6,
    Div8,
    Div16,
}

#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub enum PllQDiv {
    Div2,
    Div3,
    Div4,
    Div5,
    Div6,
    Div8,
    Div9,
}

#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub enum PllRDiv {
    Div2,
    Div3,
    Div4,
    Div5,
    Div6,
    Div8,
    Div9,
}

#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub enum PllOutMul {
    Mul26_0 = 260,
    Mul27_0 = 270,
    Mul28_0 = 280,
    Mul89_0 = 890,
    Mul90_0 = 900,
    Mul91_0 = 910,
    Mul179_0 = 1790,
    Mul180_0 = 1800,
}

pub(crate) fn init(config: ClockConfig) -> Result<(), ()> {
    todo!()
}

impl Default for ClockConfig {
    /// Config assumes EK-RA8M1.
    fn default() -> Self {
        Self {
            system: SystemClockSource::Pll1P,
            hoco: HocoFrequency::_48mhz,
            mosc: Some(20.MHz()),
            sosc: true,
            pll: Some(PllConfig {
                input: PllInput::Hoco,
                div: PllInDiv::Div1,
                mul: PllOutMul::Mul26_0,
                div_p: PllPDiv::Div2,
                div_q: PllQDiv::Div2,
                div_r: PllRDiv::Div2,
            }),
            pll2: None,
        }
    }
}
