//! Serial Peripheral Interface (`SPI`).

#![allow(missing_docs)]
#[warn(dead_code)]
const WARNING: &str = "allow(missing_docs)";

use core::marker::PhantomData;

use embassy_hal_internal::{Peri, PeripheralType};

use crate::gpio::{Pin, PortFunction};

#[allow(private_bounds)]
pub struct Spi<'d, I: Instance> {
    _instance: PhantomData<&'d I>,
}

#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

pub(crate) trait SealedInstance {
    fn module_stop();
    fn module_start();
}

#[allow(private_bounds)]
pub trait MisoPin<I: Instance>: MisoPinSealed<I> {}

pub(crate) trait MisoPinSealed<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;
}

#[allow(private_bounds)]
pub trait MosiPin<I: Instance>: MosiPinSealed<I> {}

pub(crate) trait MosiPinSealed<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;
}

#[allow(private_bounds)]
pub trait SckPin<I: Instance>: SckPinSealed<I> {}

pub(crate) trait SckPinSealed<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;
}

macro_rules! miso_pin_impl {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::spi::MisoPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::spi::MisoPinSealed<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use miso_pin_impl;

macro_rules! mosi_pin_impl {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::spi::MosiPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::spi::MosiPinSealed<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use mosi_pin_impl;

macro_rules! sck_pin_impl {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::spi::SckPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::spi::SckPinSealed<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use sck_pin_impl;

macro_rules! instance_impl {
    ($instance:ident, $mstp:ident, $rx_int:ident, $tx_int:ident, $te_int:ident) => {
        paste::paste! {
            impl Instance for crate::peripherals::$instance {}
            impl SealedInstance for crate::peripherals::$instance {
                #[inline(always)]
                fn module_stop() {
                    debug!("{}: stop=true", stringify!($instance));
                    let mstp = crate::pac::MSTP;
                    mstp.mstpcrb().modify(|w| w.[< set_ $mstp >](true));
                }

                #[inline(always)]
                fn module_start() {
                    debug!("{}: stop=false", stringify!($instance));
                    let mstp = crate::pac::MSTP;
                    mstp.mstpcrb().modify(|w| w.[< set_ $mstp >](false));
                }
            }
        }
    };
}

instance_impl!(SPI0, mstpb19, Spi0SpRi, Spi0SpTi, Spi0SpTi);
instance_impl!(SPI1, mstpb18, Spi1SpRi, Spi1SpTi, Spi1SpTi);

impl<'d, I: Instance> Spi<'d, I> {
    pub fn new(_spi: Peri<'d, I>) -> Self {
        I::module_start();
        todo!();
    }
}

impl<'d, I: Instance> Drop for Spi<'d, I> {
    fn drop(&mut self) {
        I::module_stop();
    }
}
