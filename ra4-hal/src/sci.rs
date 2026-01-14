use core::marker::PhantomData;

use embassy_hal_internal::{Peri, PeripheralType};

use crate::{pac, peripherals};

#[allow(private_bounds)]
pub struct Sci<'d, I: Instance> {
    _phantom: PhantomData<&'d I>,
}

#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

trait SealedInstance {
    fn regs() -> pac::sci0::Sci0;
}

impl Instance for peripherals::SCI0 {}
impl SealedInstance for peripherals::SCI0 {
    fn regs() -> ra4m1_ctpac::sci0::Sci0 {
        crate::pac::SCI0
    }
}

impl<'d, I: Instance> Sci<'d, I> {
    pub fn new(_peri: Peri<'d, I>) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
