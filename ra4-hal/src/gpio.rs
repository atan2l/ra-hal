use crate::{pac, write_protect::WriteProtect};

use embassy_hal_internal::{Peri, PeripheralType, impl_peripheral};
use ra4m1_ctpac::pfs::vals::{PortDirection, PortFunction, PortMode};

/// Digital input or output level.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Level {
    /// Low
    Low,
    /// High
    High,
}

/// Type-erased GPIO pin
pub struct AnyPin {
    pin_port: u16,
}

/// GPIO flexible pin.
///
/// This pin can either be a input, output, or attached to a peripheral.
pub struct Flex<'d> {
    pub(crate) pin: Peri<'d, AnyPin>,
}

pub(crate) trait SealedPin {
    fn pin_port(&self) -> u16;

    #[inline]
    fn _pin(&self) -> u8 {
        (self.pin_port() % 100) as _
    }

    #[inline]
    fn _port(&self) -> u8 {
        (self.pin_port() / 100) as _
    }

    /// Set the output as high.
    #[inline]
    fn set_high(&self) {
        let port = self.block();

        port.pcntr3().write(|w| {
            w.set_posr(self._pin() as _, true);
        });
    }

    /// Set the output as low.
    #[inline]
    fn set_low(&self) {
        let port = self.block();

        port.pcntr3().write(|w| {
            w.set_porr(self._pin() as _, true);
        });
    }

    /// Set the output level.
    #[inline]
    fn set_level(&self, level: Level) {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }

    /// Is the output pin set as high?
    #[inline]
    fn is_set_high(&self) -> bool {
        let port = self.block();

        port.pcntr1().read().podr(self._pin() as _)
    }

    /// Is the output pin set as low?
    #[inline]
    fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }

    /// What level output is set to
    #[inline]
    fn get_output_level(&self) -> Level {
        match self.is_set_high() {
            true => Level::High,
            false => Level::Low,
        }
    }

    /// Toggle pin output
    #[inline]
    fn toggle(&self) {
        match self.is_set_high() {
            true => self.set_low(),
            false => self.set_high(),
        }
    }

    #[inline]
    fn set_as_output(&self) {
        let port = self.block();
        let pin = self._pin() as _;
        port.pcntr1().modify(|w| {
            w.set_pdr(pin, true);
        });

        // let port_num = self._port() as _;
        // let pin_num = self._pin() as _;

        // let pfs = pac::PFS;

        // pfs.protected_write(|| {
        //     // info!("Port{}, Pin{}, Output", port_num, pin_num);
        //     let pfs_reg = pfs.pin(port_num, pin_num);
        //     pfs_reg.modify(|w| {
        //         w.set_pdr(PortDirection::Output);
        //     });
        // });
    }

    #[inline]
    fn set_as_input(&self) {
        let port = self.block();
        let pin = self._pin() as _;
        port.pcntr1().modify(|w| {
            w.set_pdr(pin, false);
        });

        // let port_num = self._port() as _;
        // let pin_num = self._pin() as _;

        // let pfs = pac::PFS;

        // pfs.protected_write(|| {
        //     // info!("Port{}, Pin{}, Input", port_num, pin_num);
        //     let pfs_reg = pfs.pin(port_num, pin_num);
        //     pfs_reg.modify(|w| {
        //         w.set_pdr(PortDirection::Input);
        //     });
        // });
    }

    fn set_peripheral_func(&self, pf_index: u8) {
        let port_num = self._port() as _;
        let pin_num = self._pin() as _;

        let pfs = crate::pac::PFS;

        pfs.protected_write(|| {
            // info!("Port{}, Pin{}, pf={}", port_num, pin_num, pf_index);
            let pfs_reg = pfs.pin(port_num, pin_num);
            pfs_reg.modify(|w| {
                w.set_pmr(PortMode::Peripheral);
            });
            pfs_reg.modify(|w| {
                // w.set_pmr(PortMode::Peripheral);
                w.set_psel(PortFunction::from_bits(pf_index));
            });

            pfs_reg.modify(|w| {
                w.set_asel(false);
                // w.set_dscr(val);
            });
            info!("PFS={}", pfs_reg.read());
        });
    }

    /// Get the GPIO register block for this pin.
    #[inline]
    fn block(&self) -> crate::pac::port0::Port0 {
        match self._port() {
            // Safe because PORT1 only adds registers, no changes or removals.  Also we know that this is a valid pointer.
            1 => unsafe { crate::pac::port0::Port0::from_ptr(crate::pac::PORT1.as_ptr()) },
            // Safe because PORT1 only adds registers, no changes or removals.  Also we know that this is a valid pointer.
            2 => unsafe { crate::pac::port0::Port0::from_ptr(crate::pac::PORT2.as_ptr()) },
            // Safe because PORT1 only adds registers, no changes or removals.  Also we know that this is a valid pointer.
            3 => unsafe { crate::pac::port0::Port0::from_ptr(crate::pac::PORT3.as_ptr()) },
            // Safe because PORT1 only adds registers, no changes or removals.  Also we know that this is a valid pointer.
            4 => unsafe { crate::pac::port0::Port0::from_ptr(crate::pac::PORT4.as_ptr()) },
            0 => crate::pac::PORT0,
            5 => crate::pac::PORT5,
            6 => crate::pac::PORT6,
            7 => crate::pac::PORT7,
            8 => crate::pac::PORT8,
            9 => crate::pac::PORT9,
            _ => unreachable!(),
        }
    }
}

#[allow(private_bounds)]
pub trait Pin: PeripheralType + Into<AnyPin> + SealedPin + Sized + 'static {
    /// Number of the pin within the port (0..31)
    #[inline]
    fn pin(&self) -> u8 {
        self._pin()
    }

    /// Port of the pin
    #[inline]
    fn port(&self) -> u8 {
        self._port()
    }
}

impl_peripheral!(AnyPin);

impl Pin for AnyPin {}

impl SealedPin for AnyPin {
    #[inline]
    fn pin_port(&self) -> u16 {
        self.pin_port
    }
}

impl AnyPin {
    /// Unsafely create an `AnyPin` from a pin+port number.
    ///
    /// `pin_port` is `port_num * 100 + pin_num`, where `port_num` is `0..=9`
    #[inline]
    pub unsafe fn steal(pin_port: u16) -> Peri<'static, Self> {
        unsafe { Peri::new_unchecked(Self { pin_port }) }
    }

    fn _port(&self) -> u16 {
        self.pin_port / 100
    }

    /// Get the GPIO register block for this pin.
    #[cfg(feature = "unstable-pac")]
    #[inline]
    pub fn block(&self) -> crate::pac::port0::Port0 {
        SealedPin::block(self)
    }
}

impl<'d> Flex<'d> {
    /// Create `Flex` from pin.
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>) -> Self {
        let s = Self { pin: pin.into() };
        trace!("Flex: port={}, pin={}", s.pin._port(), s.pin._pin());
        s
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    /// Set the output level.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level)
    }

    /// Is the output pin set as high?
    #[inline]
    pub fn is_set_high(&mut self) -> bool {
        self.pin.is_set_high()
    }

    /// Is the output pin set as low?
    #[inline]
    pub fn is_set_low(&mut self) -> bool {
        self.pin.is_set_low()
    }

    /// What level output is set to
    #[inline]
    pub fn get_output_level(&mut self) -> Level {
        self.pin.get_output_level()
    }

    /// Toggle pin output
    #[inline]
    pub fn toggle(&mut self) {
        self.pin.toggle();
    }

    #[inline(never)]
    pub fn set_as_output(&mut self) {
        self.pin.set_as_output();
    }

    #[inline(never)]
    pub fn set_as_input(&mut self) {
        self.pin.set_as_input();
    }

    /// Sets the pin into peripheral mode and enables peripheral func `index`
    pub fn set_peripheral_func(&mut self, index: u8) {
        self.pin.set_peripheral_func(index);
    }
}

macro_rules! pin_impl {
    ($pin_number:literal) => {
        paste! {
            impl Pin for crate::peripherals::[< P $pin_number >] {}

            impl SealedPin for crate::peripherals::[< P $pin_number >] {
                fn pin_port(&self) -> u16 {
                    $pin_number
                }
            }

            impl From<crate::peripherals::[< P $pin_number >]> for AnyPin {
                fn from(val: crate::peripherals::[< P $pin_number >]) -> Self {
                    Self {
                        pin_port: val.pin_port(),
                    }
                }
            }
        }
    };
}

/// Move this into its own mod so we can quiet the Clippy lint only for the macro invocations.
mod pin_impls {
    #![allow(clippy::zero_prefixed_literal)]

    use super::{AnyPin, Pin, SealedPin};
    use paste::paste;

    pin_impl!(000);
    pin_impl!(001);
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    pin_impl!(002);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(003);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(004);
    #[cfg(feature = "_100pin")]
    pin_impl!(005);
    #[cfg(feature = "_100pin")]
    pin_impl!(006);
    #[cfg(feature = "_100pin")]
    pin_impl!(007);
    #[cfg(feature = "_100pin")]
    pin_impl!(008);
    pin_impl!(010);
    pin_impl!(011);
    pin_impl!(012);
    pin_impl!(013);
    pin_impl!(014);
    pin_impl!(015);

    pin_impl!(100);
    pin_impl!(101);
    pin_impl!(102);
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    pin_impl!(103);
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    pin_impl!(104);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(105);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(106);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(107);
    pin_impl!(108);
    pin_impl!(109);
    pin_impl!(110);
    pin_impl!(111);
    pin_impl!(112);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(113);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(114);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(115);

    pin_impl!(200);
    pin_impl!(201);
    #[cfg(feature = "_100pin")]
    pin_impl!(202);
    #[cfg(feature = "_100pin")]
    pin_impl!(203);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(204);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(205);
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    pin_impl!(206);
    pin_impl!(212);
    pin_impl!(213);
    pin_impl!(214);
    pin_impl!(215);

    pin_impl!(300);
    pin_impl!(301);
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    pin_impl!(302);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(303);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(304);
    #[cfg(feature = "_100pin")]
    pin_impl!(305);
    #[cfg(feature = "_100pin")]
    pin_impl!(306);
    #[cfg(feature = "_100pin")]
    pin_impl!(307);

    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    pin_impl!(400);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(401);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(402);
    #[cfg(feature = "_100pin")]
    pin_impl!(403);
    #[cfg(feature = "_100pin")]
    pin_impl!(404);
    #[cfg(feature = "_100pin")]
    pin_impl!(405);
    #[cfg(feature = "_100pin")]
    pin_impl!(406);
    pin_impl!(407);
    pin_impl!(408);
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    pin_impl!(409);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(4010);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(4011);
    #[cfg(feature = "_100pin")]
    pin_impl!(4012);
    #[cfg(feature = "_100pin")]
    pin_impl!(4013);
    #[cfg(feature = "_100pin")]
    pin_impl!(4014);
    #[cfg(feature = "_100pin")]
    pin_impl!(4015);

    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    pin_impl!(500);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(501);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(502);
    #[cfg(feature = "_100pin")]
    pin_impl!(503);
    #[cfg(feature = "_100pin")]
    pin_impl!(504);
    #[cfg(feature = "_100pin")]
    pin_impl!(505);

    #[cfg(feature = "_100pin")]
    pin_impl!(600);
    #[cfg(feature = "_100pin")]
    pin_impl!(601);
    #[cfg(feature = "_100pin")]
    pin_impl!(602);
    #[cfg(feature = "_100pin")]
    pin_impl!(603);
    #[cfg(feature = "_100pin")]
    pin_impl!(608);
    #[cfg(feature = "_100pin")]
    pin_impl!(609);
    #[cfg(feature = "_100pin")]
    pin_impl!(610);

    #[cfg(feature = "_100pin")]
    pin_impl!(708);

    #[cfg(feature = "_100pin")]
    pin_impl!(808);
    #[cfg(feature = "_100pin")]
    pin_impl!(809);

    pin_impl!(914);
    pin_impl!(915);
}
