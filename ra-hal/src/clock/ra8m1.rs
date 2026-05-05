use fugit::{HertzU32, MegahertzU32};

use crate::clock::ClockConfig;

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
        todo!()
    }
}
