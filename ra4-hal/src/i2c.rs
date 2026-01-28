//! I2C Bus Interface (`IIC`)
#![allow(missing_docs)]
#[warn(dead_code)]
const WARNING: &str = "allow(missing_docs)";

use core::marker::PhantomData;

use crate::{
    gpio::{AnyPin, Pin, PortFunction},
    pac,
};

use embassy_hal_internal::{Peri, PeripheralType};
use embedded_hal_1::i2c::{SevenBitAddress, TenBitAddress};

#[allow(private_bounds)]
pub struct I2c<'d, I: Instance> {
    _phantom: PhantomData<&'d I>,
}

#[allow(private_bounds)]
pub struct ClockPin<'d, I: SealedInstance> {
    pin: Peri<'d, AnyPin>,
    _phantom_i: PhantomData<I>,
}

#[allow(private_bounds)]
pub struct DataPin<'d, I: SealedInstance> {
    pin: Peri<'d, AnyPin>,
    _phantom_i: PhantomData<I>,
}

// TODO: Oh come on.  Don't use a 32-bit value here.  Map it in a function.
#[repr(u32)]
#[derive(Debug, Default)]
pub enum I2CSpeed {
    /// 100 kHz
    #[default]
    Normal = 100_000,

    /// 400 kHz
    Fast = 400_000,

    /// 1 MHz
    FastPlus = 1_000_000,

    /// 3.4 MHz
    High = 3_400_000,
}

#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

pub(crate) trait SealedInstance {
    fn regs() -> pac::iic::Iic;
}

pub(crate) trait ClockPinSealed<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn pfunc(&self) -> PortFunction {
        Self::PERIPHERAL_FUNC
    }
}

pub(crate) trait DataPinSealed<I: SealedInstance>: Pin {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn pfunc(&self) -> PortFunction {
        Self::PERIPHERAL_FUNC
    }
}

impl SealedInstance for crate::peripherals::IIC0 {
    #[inline(always)]
    fn regs() -> pac::iic::Iic {
        crate::pac::IIC0
    }
}

impl SealedInstance for crate::peripherals::IIC1 {
    #[inline(always)]
    fn regs() -> pac::iic::Iic {
        crate::pac::IIC1
    }
}

#[allow(private_bounds)]
impl<'d, I: Instance> I2c<'d, I> {
    pub fn new<C: ClockPinSealed<I>, D: DataPinSealed<I>>(
        _iic: Peri<'d, I>,
        clock_pin: Peri<'d, C>,
        data_pin: Peri<'d, D>,
        _speed: I2CSpeed,
    ) -> Self {
        let iic = I::regs();

        iic.iccr1().write(|w| {
            w.set_ice(false);
        });

        iic.iccr1().write(|w| {
            w.set_iicrst(true);
        });

        iic.iccr1().write(|w| {
            w.set_ice(true);
        });

        iic.iccr1().write(|w| {
            w.set_iicrst(false);
        });

        // p826
        // Do not assign the SCLn or SDAn pin to the IIC when setting up the pin function control. Slave address comparison is performed if the pins are assigned to the IIC.
        let _clock_pin = ClockPin::new(clock_pin);
        let _data_pin = DataPin::new(data_pin);

        Self {
            _phantom: PhantomData,
        }
    }
}

#[allow(private_bounds)]
impl<'d, I: SealedInstance> ClockPin<'d, I> {
    /// Takes a pin and configures it to be used as I2C clock line.
    pub fn new(pin: Peri<'d, impl ClockPinSealed<I>>) -> Self {
        pin.set_as_pf(pin.pfunc());

        Self {
            pin: pin.into(),
            _phantom_i: PhantomData,
        }
    }
}

impl<'d, I: SealedInstance> From<ClockPin<'d, I>> for AnyPin {
    fn from(value: ClockPin<I>) -> Self {
        *value.pin
    }
}

macro_rules! clock_pin_impl {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::i2c::ClockPinSealed<crate::peripherals::$instance>
            for crate::peripherals::$pin
        {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use clock_pin_impl;

#[allow(private_bounds)]
impl<'d, I: SealedInstance> DataPin<'d, I> {
    /// Takes a pin and configures it to be used as I2C data line.
    pub fn new(pin: Peri<'d, impl DataPinSealed<I>>) -> Self {
        pin.set_as_pf(pin.pfunc());

        Self {
            pin: pin.into(),
            _phantom_i: PhantomData,
        }
    }
}

impl<'d, I: SealedInstance> From<DataPin<'d, I>> for AnyPin {
    fn from(value: DataPin<I>) -> Self {
        *value.pin
    }
}

macro_rules! data_pin_impl {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::i2c::DataPinSealed<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use data_pin_impl;
