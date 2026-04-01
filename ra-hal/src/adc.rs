//!  Analog-to-Digital Converter (`ADC12`, `ADC14`, `ADC16`).
//!
//! # Supported Hardware
//! * `ADC12`: supports 12-bit precision (`RA4L1`) or 8-, 10-, 12-bit precision (`RA6M5`, soon).
//! * `ADC14`: supports 12- and 14-bit precision (`RA4M1`).
//! * `ADC16`: supports 16-bit precision (`RA2A1`).
//!
//! # Notes
//! * The driver will turn the module off when it is dropped.

use core::marker::PhantomData;

use cortex_m::asm;
use embassy_hal_internal::{Peri, PeripheralType};

use crate::{adc::channel::AdcChannel, module_stop::ModuleStop, pac};

#[cfg(adc12)]
use pac::adc12::{self as adc_pac, Adc12 as AdcPac};
#[cfg(adc14)]
use pac::adc14::{self as adc_pac, Adc14 as AdcPac};
#[cfg(adc16)]
use pac::adc16::{self as adc_pac, Adc16 as AdcPac};

use adc_pac::{
    regs::Adans,
    vals::{AdcCountSelect, Adcs},
};

#[cfg(adc12)]
use crate::peripherals::ADC12_0;

#[cfg(adc14)]
use crate::{pac::adc14::vals::Adprc, peripherals::ADC14_0};

#[cfg(adc16)]
use crate::peripherals::ADC16_0;

pub use channel::{AdcInputPin, AdcPin, AdcSequence, Temperature, Vref};

pub(crate) mod channel;

/// `ADC` driver.
#[allow(private_bounds)]
pub struct Adc<'d, I: Instance> {
    _phantom: PhantomData<&'d I>,
    average_mode: AverageMode,
    sample_time: u8,
}

/// Addition/Average method.
///
/// For a single reading:
/// * `ADC12` and `ADC14` can report the single sample or the sum or average of multiple samples.
/// * `ADC16` can report the single sample or the average of multiple samples.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Default, PartialEq)]
pub enum AverageMode {
    /// Sample the channel once.
    #[default]
    Off,

    /// Sample the channel 2x and place the average in the output register.
    Average2,

    /// Sample the channel 4x and place the average in the output register.
    Average4,

    /// Place the sum of 2 samples in the output register.
    #[cfg(any(adc12, adc14))]
    Add2,

    /// Place the sum of 3 samples in the output register.
    #[cfg(any(adc12, adc14))]
    Add3,

    /// Place the sum of 4 samples in the output register.
    #[cfg(any(adc12, adc14))]
    Add4,

    /// Place the sum of 16 samples in the output register.
    #[cfg(any(adc12, adc14))]
    Add16,
}

/// ADC resolution.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Default)]
pub enum Resolution {
    /// 12-bit resolution
    #[cfg(any(adc12, adc14))]
    #[default]
    _12bit,

    /// 14-bit resolution
    #[cfg(adc14)]
    _14bit,

    /// 16-bit resolution
    #[cfg(adc16)]
    #[default]
    _16bit,
}

/// Alignment of readings within a 16-bit variable.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Default)]
pub enum Alignment {
    /// Pad the value with trailing zeros.
    FlushLeft,

    /// Pad the value with leading zeros.
    #[default]
    FlushRight,
}

/// `ADC` configuration.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone)]
pub struct AdcConfig {
    #[allow(missing_docs)]
    pub resolution: Resolution,

    /// If true the data registers are zeroed after being read (whether by CPU or DMA).
    pub clear_on_read: bool,

    #[allow(missing_docs)]
    #[cfg(not(adc16))]
    pub alignment: Alignment,

    #[allow(missing_docs)]
    pub average_mode: AverageMode,

    /// Sample time in number of `ADCLK` cycles.
    ///
    /// # Notes
    /// Sample time can *never* be lower than 5 states.
    /// If the ratio of `PCLKB` to `PCLKC` is 1:2 or 1:4 the minimum is 6 states.
    pub sample_time: u8,
}

#[derive(Copy, Clone)]
struct AdcChannelConfig {
    pub average_mode: AverageMode,
    pub sample_time: u8,
}

/// `ADC` peripheral instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + ModuleStop + PeripheralType + 'static + Send {}

impl Default for AdcConfig {
    fn default() -> Self {
        Self {
            resolution: Default::default(),
            clear_on_read: Default::default(),
            #[cfg(not(adc16))]
            alignment: Default::default(),
            average_mode: Default::default(),
            sample_time: 13,
        }
    }
}

trait SealedInstance: PeripheralType {
    fn regs() -> AdcPac;
}

#[cfg(adc12)]
impl SealedInstance for ADC12_0 {
    fn regs() -> AdcPac {
        pac::ADC12_0
    }
}

#[cfg(adc14)]
impl SealedInstance for ADC14_0 {
    fn regs() -> AdcPac {
        pac::ADC14_0
    }
}

#[cfg(adc16)]
impl SealedInstance for ADC16_0 {
    fn regs() -> AdcPac {
        pac::ADC16_0
    }
}

#[cfg(adc12)]
impl Instance for ADC12_0 {}

#[cfg(adc14)]
impl Instance for ADC14_0 {}

#[cfg(adc16)]
impl Instance for ADC16_0 {}

impl<'d, I: Instance> Adc<'d, I> {
    /// Creates a new `ADC` driver.
    pub fn new(_adc: Peri<'d, I>, config: AdcConfig) -> Self {
        #[cfg(feature = "strict-assert")]
        assert!(config.sample_time >= 5);

        I::start_module();

        #[cfg(adc14)]
        let resolution = match config.resolution {
            Resolution::_12bit => Adprc::_12bit,
            Resolution::_14bit => Adprc::_14bit,
        };

        let clear_on_read = config.clear_on_read;

        #[cfg(not(adc16))]
        let alignment = match config.alignment {
            Alignment::FlushLeft => true,
            Alignment::FlushRight => false,
        };

        let adc = I::regs();

        // Turn everything off.
        adc.adansa(0).write_value(Adans(0));
        adc.adansa(1).write_value(Adans(0));

        adc.adexicr().modify(|w| {
            w.set_ocsa(false);
            w.set_tssa(false);
        });

        adc.adcer().write(|w| {
            #[cfg(adc14)]
            {
                w.set_adprc(resolution);
            }

            #[cfg(any(adc12, adc14))]
            {
                w.set_adrfmt(alignment);
            }

            w.set_ace(clear_on_read);
        });

        adc.adadc().write(|w| match config.average_mode {
            AverageMode::Off => {}
            AverageMode::Average2 => {
                #[cfg(any(adc12, adc14))]
                w.set_avee(true);
                w.set_adc(AdcCountSelect::Convert2);
            }
            AverageMode::Average4 => {
                #[cfg(any(adc12, adc14))]
                w.set_avee(true);
                w.set_adc(AdcCountSelect::Convert4);
            }
            #[cfg(any(adc12, adc14))]
            AverageMode::Add2 => {
                w.set_avee(false);
                w.set_adc(AdcCountSelect::Convert2);
            }
            #[cfg(any(adc12, adc14))]
            AverageMode::Add3 => {
                w.set_avee(false);
                w.set_adc(AdcCountSelect::Convert3);
            }
            #[cfg(any(adc12, adc14))]
            AverageMode::Add4 => {
                w.set_avee(false);
                w.set_adc(AdcCountSelect::Convert4);
            }
            #[cfg(any(adc12, adc14))]
            AverageMode::Add16 => {
                w.set_avee(false);
                w.set_adc(AdcCountSelect::Convert16);
            }
        });

        Self {
            _phantom: PhantomData,
            average_mode: config.average_mode,
            sample_time: config.sample_time,
        }
    }

    /// Returns the currently configured resolution (12 or 14-bits).
    pub fn resolution(&self) -> Resolution {
        #[cfg(adc12)]
        {
            Resolution::_12bit
        }

        #[cfg(adc14)]
        {
            let adc = I::regs();

            match adc.adcer().read().adprc() {
                Adprc::_12bit => Resolution::_12bit,
                Adprc::_RESERVED_1 => unimplemented!(),
                Adprc::_RESERVED_2 => unimplemented!(),
                Adprc::_14bit => Resolution::_14bit,
            }
        }

        #[cfg(adc16)]
        {
            Resolution::_16bit
        }
    }

    /// Returns the currently configured alignment. §35.2.1, §35.2.11
    #[cfg(adc14)]
    pub fn alignment(&self) -> Alignment {
        let adc = I::regs();

        match adc.adcer().read().adrfmt() {
            true => Alignment::FlushLeft,
            false => Alignment::FlushRight,
        }
    }

    /// Returns the on-die [temperature measurement pseudo-channel](Temperature) without performing configuration.
    pub fn temperature_channel(&self) -> Temperature {
        Temperature {}
    }

    /// Returns the [`Vref`] pseudo channel.
    pub fn vref_channel(&self) -> Vref {
        Vref {}
    }

    /// Returns a single reading from an ADC channel.
    /// Generic over `R` so we can return individual values or sequences as needed.
    pub fn blocking_read<R, C: AdcChannel<R>>(&self, channel: &C) -> R {
        let adc = I::regs();

        channel.enable::<I>(AdcChannelConfig {
            average_mode: self.average_mode,
            sample_time: self.sample_time,
        });

        // TODO: read ADST first to ensure we're stopped?

        adc.adcsr().modify(|w| w.set_adcs(Adcs::Single));

        adc.adcsr().modify(|w| w.set_adst(true));

        // When the conversion is finished an interrupt is fired (without modifying the registers) and the ADST bit is cleared
        while adc.adcsr().read().adst() {
            asm::nop()
        }

        channel.disable::<I>();

        channel.read_one::<I>()
    }
}

impl<'d, I: Instance> Drop for Adc<'d, I> {
    fn drop(&mut self) {
        I::stop_module();
    }
}
