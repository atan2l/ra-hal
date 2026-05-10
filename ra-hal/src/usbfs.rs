//! Universal Serial Bus Full-Speed device support (`USBFS`).
//!
//! This module is scaffolded around the proven RA4M1 USBFS implementation used
//! on EK-RA4M1. The long-term design keeps the hardware-facing USBFS driver in
//! the HAL while the `usb-device` integration layer remains above it.
#![allow(missing_docs)]

mod bus;
mod driver;
mod regs;
mod types;

use embassy_hal_internal::PeripheralType;

use crate::gpio::Pin;

pub use self::{
    bus::Bus,
    driver::{Config, Driver, Instance, InterruptHandler, UsbClockSource},
    types::{BusState, Error, PipeBinding, UsbEventSnapshot, UsbIrqEvent, UsbIrqLocalState},
};

pub trait VbusPin<I: Instance>: SealedVbusPin<I> {}

pub(crate) trait SealedVbusPin<I: driver::SealedInstance>: Pin + PeripheralType {}

macro_rules! usbfs_vbus_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::usbfs::VbusPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::usbfs::SealedVbusPin<crate::peripherals::$instance>
            for crate::peripherals::$pin
        {
            // const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use usbfs_vbus_pin;

pub trait DpPin<I: Instance>: SealedDpPin<I> {}

pub(crate) trait SealedDpPin<I: driver::SealedInstance>: Pin + PeripheralType {}

macro_rules! usbfs_dp_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::usbfs::DpPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::usbfs::SealedDpPin<crate::peripherals::$instance> for crate::peripherals::$pin {
            // const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use usbfs_dp_pin;

pub trait DmPin<I: Instance>: SealedDmPin<I> {}

pub(crate) trait SealedDmPin<I: driver::SealedInstance>: Pin + PeripheralType {}

macro_rules! usbfs_dm_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::usbfs::DmPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::usbfs::SealedDmPin<crate::peripherals::$instance> for crate::peripherals::$pin {
            // const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use usbfs_dm_pin;
