//! `ADC14` 14-bit Analog-to-Digital Converter

use core::marker::PhantomData;

use cortex_m::asm;
use embassy_hal_internal::{Peri, PeripheralType};
use ra4m1_ctpac::adc14::vals::{Adcs, Adprc};

use crate::{
    adc::channel::{AdcChannel, Temperature, Vref},
    pac, peripherals,
};

pub mod channel;

#[allow(private_bounds)]
pub struct Adc<'d, I: Instance> {
    _phantom: PhantomData<&'d I>,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default)]
pub enum Resolution {
    /// 12-bit resolution
    #[default]
    Low,
    /// 14-bit resolution
    High,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default)]
pub enum Alignment {
    FlushLeft,
    #[default]
    FlushRight,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default)]
pub struct AdcConfig {
    pub resolution: Resolution,
    pub clear_on_read: bool,
    pub alignment: Alignment,
}

/// `ADC14` peripheral instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

trait SealedInstance: PeripheralType {
    fn regs() -> pac::adc14::Adc14;
}

impl SealedInstance for peripherals::ADC14 {
    fn regs() -> ra4m1_ctpac::adc14::Adc14 {
        pac::ADC14
    }
}

impl Instance for peripherals::ADC14 {}

impl<'d, I: Instance> Adc<'d, I> {
    pub fn new(
        _adc: Peri<'d, I>,
        config: AdcConfig,
        // _irq: impl interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>> + 'd,
    ) -> Self {
        debug!("ADC14: stop=false");

        let mstp = pac::MSTP;

        mstp.mstpcrd().write(|w| {
            w.set_mstpd16(false);
        });

        let resolution = match config.resolution {
            Resolution::Low => Adprc::_00,
            Resolution::High => Adprc::_11,
        };

        let clear_on_read = config.clear_on_read;

        let alignment = match config.alignment {
            Alignment::FlushLeft => true,
            Alignment::FlushRight => false,
        };

        let adc = I::regs();

        adc.adcer().write(|w| {
            w.set_adprc(resolution);
            w.set_ace(clear_on_read);
            w.set_adrfmt(alignment);
        });

        Self {
            _phantom: PhantomData,
        }
    }

    /// Returns the currently configured resolution (12 or 14-bits).
    pub fn resolution(&self) -> Resolution {
        let adc = I::regs();

        match adc.adcer().read().adprc() {
            Adprc::_00 => Resolution::Low,
            Adprc::_RESERVED_1 => unimplemented!(),
            Adprc::_RESERVED_2 => unimplemented!(),
            Adprc::_11 => Resolution::High,
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

    /// Return the pseudo-channel struct for temperature measurement.  Nothing is configured here.
    pub fn temperature_channel(&self) -> Temperature {
        Temperature {}
    }

    pub fn vref_channel(&self) -> Vref {
        Vref {}
    }

    pub fn blocking_read(&self, channel: &impl AdcChannel) -> u16 {
        let adc = I::regs();

        channel.enable::<I>();

        // TODO: read ADST first to ensure we're stopped?

        adc.adcsr().modify(|w| {
            w.set_adcs(Adcs::Single);
        });

        adc.adcsr().modify(|w| {
            w.set_adst(true);
        });

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
        debug!("ADC14: stop=true");

        let mstp = pac::MSTP;

        mstp.mstpcrd().write(|w| {
            w.set_mstpd16(true);
        });
    }
}
