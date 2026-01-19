use core::marker::PhantomData;

use crate::{gpio::AnyPin, pac};

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
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {
    // type Interrupt: interrupt::typelevel::Interrupt;
    // const INSTANCE: &str;
}

trait SealedInstance {
    // const PERIPHERAL_ID: Interrupt;

    fn regs() -> pac::iic::Iic;
}
