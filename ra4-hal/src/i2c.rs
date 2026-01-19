use core::marker::PhantomData;

use crate::{
    gpio::{AnyPin, PeripheralFunction, Pin},
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

#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

trait SealedInstance {
    fn regs() -> pac::iic::Iic;
}

trait ClockPinSealed<I: SealedInstance>: Pin + PeripheralType {}

trait DataPinSealed<I: SealedInstance>: Pin {}

impl SealedInstance for crate::peripherals::IIC0 {
    #[inline(always)]
    fn regs() -> pac::iic::Iic {
        crate::pac::IIC0
    }
}

#[allow(private_bounds)]
impl<'d, I: SealedInstance> ClockPin<'d, I> {
    /// Takes a pin and configures it to be used as I2C clock line.
    pub fn new(pin: Peri<'d, impl ClockPinSealed<I>>) -> Self {
        pin.set_peripheral_func(PeripheralFunction::I2c);

        Self {
            pin: pin.into(),
            _phantom_i: PhantomData,
        }
    }
}

macro_rules! clock_pin_impl {
    ($iic:ident, $pin:ident) => {
        impl crate::i2c::ClockPinSealed<crate::peripherals::$iic> for crate::peripherals::$pin {}
    };
}

#[allow(private_bounds)]
impl<'d, I: SealedInstance> DataPin<'d, I> {
    /// Takes a pin and configures it to be used as I2C data line.
    pub fn new(pin: Peri<'d, impl DataPinSealed<I>>) -> Self {
        pin.set_peripheral_func(PeripheralFunction::I2c);

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
    ($iic:ident, $pin:ident) => {
        impl crate::i2c::DataPinSealed<crate::peripherals::$iic> for crate::peripherals::$pin {}
    };
}

// data_pin_impl!(IIC1, P101);
// data_pin_impl!(IIC1, P206);

data_pin_impl!(IIC0, P401);
data_pin_impl!(IIC0, P407);
clock_pin_impl!(IIC0, P204);
clock_pin_impl!(IIC0, P400);
clock_pin_impl!(IIC0, P408);
