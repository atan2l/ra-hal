#![allow(missing_docs)]

use core::marker::PhantomData;

use embassy_hal_internal::PeripheralType;

use crate::gpio::{Pin, PortFunction};

#[allow(private_bounds)]
pub struct Can<'d, I: Instance> {
    _instance: PhantomData<&'d I>,
}

#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

pub(crate) trait SealedInstance {}

#[allow(private_bounds)]
pub trait RxPin<I: Instance>: SealedRxPin<I> {}

pub(crate) trait SealedRxPin<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_as_crx(&self) {
        trace!("P{}{:02}: CanRxPin::new", self.port(), self.pin());
        self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

#[allow(private_bounds)]
pub trait TxPin<I: Instance>: SealedTxPin<I> {}

pub(crate) trait SealedTxPin<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_as_ctx(&self) {
        trace!("P{}{:02}: CanTxPin::new", self.port(), self.pin());
        self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

macro_rules! crx_pin_impl {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::can::RxPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::can::SealedRxPin<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use crx_pin_impl;

macro_rules! ctx_pin_impl {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::can::TxPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::can::SealedTxPin<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use ctx_pin_impl;

macro_rules! instance_impl {
    ($instance:ident, $mstp:ident, $rx_int:ident, $te_int:ident, $tx_int:ident) => {
        paste::paste! {
            impl Instance for crate::peripherals::$instance {}
            impl SealedInstance for crate::peripherals::$instance {
            }
        }
    };
}

instance_impl!(CAN0, mstpb2, Can0Rxf, Can0Txf, Can0Txf);
