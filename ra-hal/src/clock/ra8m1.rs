use fugit::{HertzU32, MegahertzU32, RateExtU32 as _};

use crate::{
    clock::{ClockConfig, HocoFrequency, PllConfig, SystemClockSource},
    pac::{
        self,
        system::vals::{
            Hcstp, Plidiv, Pllmul, Pllmulnf, Plodivp, Plodivq, Plodivr, Plsrcsel, Sodrv,
        },
    },
    write_protect::ProtectedPeripheral as _,
};

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

// Should we just make this an integer and range check at runtime?
#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub enum PllOutMul {
    Mul26_0 = 260,
    Mul27_0 = 270,
    Mul28_0 = 280,
    Mul40_0 = 400,
    Mul80_0 = 800,
    Mul89_0 = 890,
    Mul90_0 = 900,
    Mul91_0 = 910,
    Mul179_0 = 1790,
    Mul180_0 = 1800,
}

impl From<PllInput> for Plsrcsel {
    fn from(value: PllInput) -> Self {
        match value {
            PllInput::Hoco => Self::Hoco,
            PllInput::Mosc => Self::Mosc,
        }
    }
}

impl From<PllInDiv> for Plidiv {
    fn from(value: PllInDiv) -> Self {
        match value {
            PllInDiv::Div1 => Self::Div1,
            PllInDiv::Div2 => Self::Div2,
            PllInDiv::Div3 => Self::Div3,
            PllInDiv::Div4 => Self::Div4,
        }
    }
}

impl From<PllOutMul> for Pllmul {
    fn from(value: PllOutMul) -> Self {
        match value {
            PllOutMul::Mul26_0 => Self::Mul26,
            PllOutMul::Mul27_0 => Self::Mul27,
            PllOutMul::Mul28_0 => Self::Mul28,
            PllOutMul::Mul40_0 => Self::Mul40,
            PllOutMul::Mul80_0 => Self::Mul80,
            PllOutMul::Mul89_0 => Self::Mul89,
            PllOutMul::Mul90_0 => Self::Mul90,
            PllOutMul::Mul91_0 => Self::Mul91,
            PllOutMul::Mul179_0 => Self::Mul179,
            PllOutMul::Mul180_0 => Self::Mul180,
        }
    }
}

pub(crate) fn init(config: ClockConfig) -> Result<(), ()> {
    let system = pac::SYSTEM;
    debug!("HOCO: status={}", system.hococr().read());

    if !config.pll.is_none() && config.system == SystemClockSource::Pll1P {
        panic!("PLL selected as root clock, but not enabled/configured.");
    }

    if !config.sosc && config.system == SystemClockSource::Pll1P {
        panic!("SOSC required to use PLL as root clock, but SOSC not enabled.");
    }

    system.protected_write(|| {
        if config.sosc {
            system.somcr().modify(|r| r.set_sodrv(Sodrv::Normal));
            system.sosccr().modify(|r| r.set_sostp(false));
            // TODO: Set oscillator stabilization time.
            while system.sosccr().read().sostp() {}
        }

        let fll_magic = match config.hoco {
            HocoFrequency::_16mhz => crate::constants::FLL_16MHZ,
            HocoFrequency::_18mhz => crate::constants::FLL_18MHZ,
            HocoFrequency::_20mhz => crate::constants::FLL_20MHZ,
            HocoFrequency::_32mhz => crate::constants::FLL_32MHZ,
            HocoFrequency::_48mhz => crate::constants::FLL_48MHZ,
        };
        system.fllcr2().modify(|r| r.set_fllcntl(fll_magic));
        system.fllcr1().modify(|r| r.set_fllen(true));

        system.hococr2().write(|r| r.set_hcfrq0(config.hoco.into()));
        system.hococr().write(|r| r.set_hcstp(Hcstp::Start));
        while !system.oscsf().read().hocosf() {}

        system.pllcr().write(|r| r.set_pllstp(true));
        while system.oscsf().read().pllsf() {}

        if let Some(pll_config) = config.pll.as_ref() {
            system.pllccr().modify(|r| {
                r.set_plsrcsel(pll_config.input.into());
                r.set_plidiv(pll_config.div.into());
                r.set_pllmul(pll_config.mul.into());
                r.set_pllmulnf(Pllmulnf::_00);
            });
            system.pllccr2().modify(|r| {
                r.set_plodivp(Plodivp::Div2);
                r.set_plodivq(Plodivq::Div2);
                r.set_plodivr(Plodivr::Div2);
            });
            system.pllcr().write(|r| r.set_pllstp(false));
            while !system.oscsf().read().pllsf() {}
        }
    });

    error!("NOT YET");
    Ok(())
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
                div: PllInDiv::Div4,
                mul: PllOutMul::Mul80_0,
                div_p: PllPDiv::Div2,
                div_q: PllQDiv::Div2,
                div_r: PllRDiv::Div2,
            }),
            pll2: None,
        }
    }
}
