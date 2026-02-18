#![allow(missing_docs)]

use core::marker::PhantomData;

use embassy_hal_internal::{Peri, PeripheralType};
use ra4m1_ctpac::can::vals::{Tseg1, Tseg2};

use crate::{
    gpio::{Basic, Flex, Pin, PortFunction},
    pac,
};

#[allow(private_bounds)]
pub struct Can<'d, I: Instance> {
    _instance: PhantomData<&'d I>,
    _rx: Flex<'d, Basic>,
    _tx: Flex<'d, Basic>,
}

#[derive(Copy, Clone)]
pub enum Bitrate {
    _125,
    _250,
    _500,
    _1000,
}

struct Timing {
    bitrate: Bitrate,
    prescaler: u8,
    ts1: Tseg1,
    ts2: Tseg2,
}

const TIMING: [Timing; 1] = [Timing {
    bitrate: Bitrate::_500,
    prescaler: 3,
    ts1: Tseg1::Tq11,
    ts2: Tseg2::Tq4,
}];

#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

pub(crate) trait SealedInstance {
    fn regs() -> pac::can::Can;
    fn module_stop();
    fn module_start();
}

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

impl<'d, I: Instance> Can<'d, I> {
    pub fn new<R: RxPin<I>, T: TxPin<I>>(
        _can: Peri<'d, I>,
        rx: Peri<'d, R>,
        tx: Peri<'d, T>,
        _bitrate: Bitrate,
    ) -> Self {
        I::module_start();

        let can = I::regs();

        rx.set_as_crx();
        tx.set_as_ctx();

        let can_config = &TIMING[0];

        can.bcr().modify(|r| {
            r.set_tseg1(can_config.ts1);
            r.set_tseg2(can_config.ts2);
            r.set_brp(can_config.prescaler.into());
        });

        Self {
            _instance: PhantomData,
            _rx: Flex::new_basic(rx),
            _tx: Flex::new_basic(tx),
        }
    }
}

impl<'d, I: Instance> Drop for Can<'d, I> {
    fn drop(&mut self) {
        I::module_stop();
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
                #[inline(always)]
                fn regs() -> pac::can::Can {
                    crate::pac::$instance
                }

                #[inline(always)]
                fn module_stop() {
                    debug!("{}: stop=true", stringify!($instance));
                    let mstp = pac::MSTP;
                    mstp.mstpcrb().modify(|r| r.[< set_ $mstp >](true));
                }

                #[inline(always)]
                fn module_start() {
                    debug!("{}: stop=false", stringify!($instance));
                    let mstp = pac::MSTP;
                    mstp.mstpcrb().modify(|r| r.[< set_ $mstp >](false));
                }
            }
        }
    };
}

instance_impl!(CAN0, mstpb2, Can0Rxf, Can0Txf, Can0Txf);
