//! 12-bit Digital-to-Analog Converter (`DAC12`).
//!
//! # Notes
//!
//! The driver will turn the module off (`MSTPD20=1`) when it is dropped.

use core::marker::PhantomData;

use embassy_hal_internal::{Peri, PeripheralType};
use embassy_time::{Duration, block_for};

use crate::{
    gpio::{Basic, Flex, Pin},
    module_stop::ModuleStop,
    pac::dac12::vals::Ref,
};

/// `DAC12` driver.
#[allow(private_bounds)]
pub struct Dac<'d, I: Instance> {
    _output_pin: Flex<'d, Basic>,
    _phantom: PhantomData<&'d I>,
}

/// Alignment of output values within a 16-bit variable.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Default, Debug)]
pub enum Align {
    /// The value is padded with trailing zeros.
    Left,

    /// The value is padded with leading zeros.
    #[default]
    Right,
}

/// Reference voltage for `DAC12`. §35.1.
pub enum Vref {
    /// `Vrefh0`, analog reference voltage
    AnalogReference,

    /// `Avcc0`
    AnalogSupply,

    /// Internal ref ≈ 1.43V
    Internal,

    /// No reference, turn off `DAC12`.
    None,
}

/// For the physical GPIO pins connected to `DAC12` channels.
#[allow(private_bounds)]
pub trait DacOutputPin: SealedDacOutputPin {}

/// For the physical GPIO pins connected to `DAC12` channels.
pub(crate) trait SealedDacOutputPin: Pin + PeripheralType {}

/// `DAC12` peripheral instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + ModuleStop + PeripheralType {}

pub(crate) trait SealedInstance: PeripheralType {
    fn regs() -> crate::pac::dac12::Dac12;
}

impl<'d, I: Instance> Dac<'d, I> {
    /// Creates a new `DAC12` driver and configures `Vref` to use the analog block power supply (`Avcc0`).
    pub fn new<P: DacOutputPin>(peri: Peri<'d, I>, output_pin: Peri<'d, P>) -> Self {
        let _ = peri;

        I::start_module();
        output_pin.set_as_analog();

        let mut this = Self {
            _output_pin: Flex::new(output_pin),
            _phantom: PhantomData,
        };

        this.set_vref(Vref::AnalogSupply);

        this
    }

    /// Returns a mutable pointer to the output register.
    ///
    /// This should probably go away and/or be made not-pub.
    #[inline(always)]
    pub fn buffer(&self) -> *mut u16 {
        let dac = I::regs();
        dac.dadr0().as_ptr()
    }

    /// Sets the data alignment.
    #[inline(always)]
    pub fn set_align(&mut self, align: Align) {
        let dac = I::regs();
        match align {
            Align::Left => dac.dadpr().modify(|r| r.set_dpsel(true)),
            Align::Right => dac.dadpr().modify(|r| r.set_dpsel(false)),
        }
    }

    /// Sets the reference voltage source and enables DAC output when voltage has stabilized.
    #[inline(always)]
    pub fn set_vref(&mut self, vref: Vref) {
        // AVCC0 ≈ 4.8V on Arduino
        // Internal ref ≈ 1.43V
        // VREFH = unstable, slowly grows from ≈ 1V?

        let dac = I::regs();

        // §36.2.5, §36.3.2
        dac.davrefcr().modify(|r| r.set_ref_(Ref::None));
        dac.dadr0().write_value(0);
        dac.dacr().modify(|r| r.set_daoe0(false));

        block_for(Duration::from_micros(10));

        let vref = match vref {
            Vref::AnalogReference => Ref::Vref,
            Vref::AnalogSupply => Ref::Avcc0,
            Vref::Internal => Ref::Internal,
            Vref::None => return,
        };

        dac.davrefcr().modify(|r| r.set_ref_(vref));
        dac.dacr().modify(|r| r.set_daoe0(true));
        block_for(Duration::from_micros(5));
    }

    /// Sets the output value.
    #[inline(always)]
    pub fn set(&mut self, value: u16) {
        trace!("Set {}", value);
        let dac = I::regs();

        dac.dadr0().write_value(value);
    }

    /// Starts DAC output.
    #[inline(always)]
    pub fn start(&mut self) {
        let dac = I::regs();
        dac.dacr().write(|w| w.set_daoe0(true));
    }

    /// Stops DAC output.
    #[inline(always)]
    pub fn stop(&mut self) {
        let dac = I::regs();
        dac.dacr().write(|w| w.set_daoe0(false));
    }
}

impl<'d, I: Instance> Drop for Dac<'d, I> {
    /// Turns off `DAC12` output and sets `MSTPD20=true`.
    fn drop(&mut self) {
        warn!("DAC12: Drop");
        let dac = I::regs();
        dac.dacr().modify(|r| r.set_daoe0(false));
        I::stop_module();
    }
}

impl Instance for crate::peripherals::DAC12 {}
impl SealedInstance for crate::peripherals::DAC12 {
    #[inline(always)]
    fn regs() -> crate::pac::dac12::Dac12 {
        crate::pac::DAC12
    }
}

macro_rules! dac_pin {
    ($pin:ident) => {
        impl crate::dac::DacOutputPin for crate::peripherals::$pin {}
        impl crate::dac::SealedDacOutputPin for crate::peripherals::$pin {}
    };
}
pub(crate) use dac_pin;
