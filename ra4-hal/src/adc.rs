//! 14-bit Analog-to-Digital Converter (`ADC14`).
//!
//! # Notes
//! * The driver will turn the module off (`MSTPD16=1`) when it is dropped.

use core::marker::PhantomData;

use cortex_m::asm;
use embassy_hal_internal::{Peri, PeripheralType};

use crate::{
    adc::channel::AdcChannel,
    module_stop::ModuleStop,
    pac::{
        self,
        adc14::{
            regs::Adans,
            vals::{AdcCountSelect, Adcs, Adprc},
        },
    },
    peripherals::ADC14,
};

pub use channel::{AdcInputPin, AdcPin, AdcSequence, Temperature, Vref};

pub(crate) mod channel;

/// `ADC14` driver.
#[allow(private_bounds)]
pub struct Adc<'d, I: Instance> {
    _phantom: PhantomData<&'d I>,
    average_mode: AverageMode,
    sample_time: u8,
}

/// Addition/Average method.
///
/// For a single reading `ADC14` can report the single sample or the sum or average of multiple samples.
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
    Add2,

    /// Place the sum of 3 samples in the output register.
    Add3,

    /// Place the sum of 4 samples in the output register.
    Add4,

    /// Place the sum of 16 samples in the output register.
    Add16,
}

/// ADC resolution.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default)]
pub enum Resolution {
    /// 12-bit resolution
    #[default]
    Low,

    /// 14-bit resolution
    High,
}

/// Alignment of readings within a 16-bit variable.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default)]
pub enum Alignment {
    /// Pad the value with trailing zeros.
    FlushLeft,

    /// Pad the value with leading zeros.
    #[default]
    FlushRight,
}

/// `ADC14` configuration.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcConfig {
    #[allow(missing_docs)]
    pub resolution: Resolution,

    /// If true the data registers are zeroed after being read (whether by CPU or DMA).
    pub clear_on_read: bool,

    #[allow(missing_docs)]
    pub alignment: Alignment,

    #[allow(missing_docs)]
    pub average_mode: AverageMode,

    /// Sample time in number of `PCLKC` (a.k.a. `ADCLK`) cycles.
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

/// `ADC14` peripheral instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + ModuleStop + PeripheralType + 'static + Send {}

impl Default for AdcConfig {
    fn default() -> Self {
        Self {
            resolution: Default::default(),
            clear_on_read: Default::default(),
            alignment: Default::default(),
            average_mode: Default::default(),
            sample_time: 13,
        }
    }
}

trait SealedInstance: PeripheralType {
    fn regs() -> pac::adc14::Adc14;
}

impl SealedInstance for ADC14 {
    fn regs() -> pac::adc14::Adc14 {
        pac::ADC14
    }
}

impl Instance for ADC14 {}

impl<'d, I: Instance> Adc<'d, I> {
    /// Creates a new `ADC14` driver.
    pub fn new(_adc: Peri<'d, I>, config: AdcConfig) -> Self {
        #[cfg(feature = "strict-assert")]
        assert!(config.sample_time >= 5);

        I::start_module();

        let resolution = match config.resolution {
            Resolution::Low => Adprc::_12bit,
            Resolution::High => Adprc::_14bit,
        };

        let clear_on_read = config.clear_on_read;

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
            w.set_adprc(resolution);
            w.set_ace(clear_on_read);
            w.set_adrfmt(alignment);
        });

        adc.adadc().write(|w| match config.average_mode {
            AverageMode::Off => {}
            AverageMode::Average2 => {
                w.set_avee(true);
                w.set_adc(AdcCountSelect::Convert2);
            }
            AverageMode::Average4 => {
                w.set_avee(true);
                w.set_adc(AdcCountSelect::Convert4);
            }
            AverageMode::Add2 => {
                w.set_avee(false);
                w.set_adc(AdcCountSelect::Convert2);
            }
            AverageMode::Add3 => {
                w.set_avee(false);
                w.set_adc(AdcCountSelect::Convert3);
            }
            AverageMode::Add4 => {
                w.set_avee(false);
                w.set_adc(AdcCountSelect::Convert4);
            }
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
        let adc = I::regs();

        match adc.adcer().read().adprc() {
            Adprc::_12bit => Resolution::Low,
            Adprc::_RESERVED_1 => unimplemented!(),
            Adprc::_RESERVED_2 => unimplemented!(),
            Adprc::_14bit => Resolution::High,
        }
    }

    /// Returns the currently configured alignment. §35.2.1, §35.2.11
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
