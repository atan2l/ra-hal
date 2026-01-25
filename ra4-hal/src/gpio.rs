//! General Purpose Input/Output (`PORT`)

// TODO: Ensure PFS register access is correct and not clobbering other bits nor getting discarded by write-protection

use crate::{pac, write_protect::WriteProtect};

use embassy_hal_internal::{Peri, PeripheralType, impl_peripheral};
use ra4m1_ctpac::pfs::vals::{PortDrive, PortMode};

/// Digital input or output level.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Level {
    /// Low
    Low,
    /// High
    High,
}

/// Output drive capacity
///
/// For the `RA4M1` the maximum output of all pins is 60 mA, and each pin configured for max 4.0 mA or max 8.0 mA.
/// Cortex-M33 devices have different limits.
/// See the RA4 Quick Design Guide R01AN5988, §10.2.3 for more information.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DriveCapacity {
    /// Low drive, max output 4.0 mA.
    Low,
    /// Medium drive, max output 8.0 mA.
    Middle,
}

/// Type-erased GPIO pin
pub struct AnyPin {
    pin_port: u16,
}

/// GPIO flexible pin.
///
/// This pin can be configured for input, output, or attached to a peripheral.
pub struct Flex<'d> {
    pin: Peri<'d, AnyPin>,
}

/// GPIO input driver
pub struct Input<'d> {
    pin: Flex<'d>,
}

/// GPIO output driver
pub struct Output<'d> {
    pin: Flex<'d>,
}

// Should this just export the type from the PAC?
/// Additional, non-GPIO, functions that can be assigned to a pin.
///
/// Not every function applies to every pin.
/// See tables 19.5–19.17 in the reference manual for details.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum PortFunction {
    #[doc = "Hi-Z / JTAG / SWD (0b00000)"]
    HiZ = 0x0,
    #[doc = "Low-Power Asynchronous General Purpose Timer (0b00001)"]
    Agt = 0x01,
    #[doc = "General PWM Timer (0b00010)"]
    Gpt1 = 0x02,
    #[doc = "General PWM Timer (0b00011)"]
    Gpt2 = 0x03,
    #[doc = "Serial Communications Interface (0b00100)"]
    Sci1 = 0x04,
    #[doc = "Serial Communications Interface (0b00101)"]
    Sci2 = 0x05,
    #[doc = "Serial Peripheral Interface (0b00110)"]
    Spi = 0x06,
    #[doc = "Inter-Integrated Circuit Bus Interface (0b00111)"]
    I2c = 0x07,
    #[doc = "Key Interrupt Function (0b01000)"]
    Kint = 0x08,
    #[doc = "CLKOUT / Analog Comparator / Real-Time Clock (0b01001)"]
    ClkCmpRtc = 0x09,
    #[doc = "Clock Frequency Accuracy / ADC14 (0b01010)"]
    CacAdc = 0x0a,
    #[doc = "Capacitive Touch (0b01100)"]
    Ctsu = 0x0c,
    #[doc = "Segment LCD (0b01101)"]
    Slcdc = 0x0d,
    #[doc = "Controller Area Network Bus (0b10000)"]
    Can = 0x10,
    #[doc = "Serial Sound Interface Enhanced (0b10010)"]
    Ssie = 0x12,
    #[doc = "USB Full-Speed (0b10011)"]
    UsbFs = 0x13,
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

    #[inline]
    fn set_drive_capacity(&self, drive_capacity: DriveCapacity) {
        let pfs = pac::PFS;
        let port_num = self._port() as _;
        let pin_num = self._pin() as _;
        let pfs_reg = pfs.pin(port_num, pin_num);

        pfs.protected_write(|| {
            pfs_reg.modify(|w| match drive_capacity {
                DriveCapacity::Low => w.set_dscr(PortDrive::Low),
                DriveCapacity::Middle => w.set_dscr(PortDrive::Middle),
            })
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

        // It's possible to configure direction via PFS registers too.
        // Keeping this in the comments as an example, but it seems
        // overkill for toggling the direction of one pin.

        // let pfs = pac::PFS;
        // let port_num = self._port() as _;
        // let pin_num = self._pin() as _;
        //
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
    }

    fn set_as_pf(&self, port_func: PortFunction) {
        let port_num = self._port() as _;
        let pin_num = self._pin() as _;

        let pfs = crate::pac::PFS;

        let pfs_reg = pfs.pin(port_num, pin_num);
        debug!("Port{}, Pin{}, pf={}", port_num, pin_num, port_func);

        // The quick design guide suggests that first setting the pin to GPIO ensure the peripheral doesn't get any spurious input
        pfs.protected_write(|| {
            pfs_reg.modify(|w| {
                w.set_pmr(PortMode::Gpio);
            });
        });

        // Le sigh.  Write protection gets re-enabled after each write.
        pfs.protected_write(|| {
            pfs_reg.modify(|w| {
                w.set_pmr(PortMode::Peripheral);
                w.set_psel(port_func.into());
            });
        });

        #[cfg(feature = "strict-assert")]
        assert_eq!(
            pfs_reg.read().psel(),
            pac::pfs::vals::PortFunction::from_bits(port_func as u8),
            "PSEL was ignored"
        );

        info!("PFS={}", pfs_reg.read());
    }

    /// Get the GPIO register block for this pin.
    #[inline]
    fn block(&self) -> crate::pac::port0::Port0 {
        match self._port() {
            0 => crate::pac::PORT0,
            // Safe because PORT1 only adds registers, no changes or removals compared to PORT0.  Also we know that this is a valid pointer.
            1 => unsafe { crate::pac::port0::Port0::from_ptr(crate::pac::PORT1.as_ptr()) },
            // Safe because PORT1 only adds registers, no changes or removals compared to PORT0.  Also we know that this is a valid pointer.
            2 => unsafe { crate::pac::port0::Port0::from_ptr(crate::pac::PORT2.as_ptr()) },
            // Safe because PORT1 only adds registers, no changes or removals compared to PORT0.  Also we know that this is a valid pointer.
            3 => unsafe { crate::pac::port0::Port0::from_ptr(crate::pac::PORT3.as_ptr()) },
            // Safe because PORT1 only adds registers, no changes or removals.  compared to PORT0. Also we know that this is a valid pointer.
            4 => unsafe { crate::pac::port0::Port0::from_ptr(crate::pac::PORT4.as_ptr()) },
            // Safe because the pins have identical features to those of PORT0.
            5 => crate::pac::PORT5,
            // Safe because the pins have identical features to those of PORT0.
            6 => crate::pac::PORT6,
            // Safe because the pins have identical features to those of PORT0.
            7 => crate::pac::PORT7,
            // Safe because the pins have identical features to those of PORT0.
            8 => crate::pac::PORT8,
            // Safe because the pins have identical features to those of PORT0.
            9 => crate::pac::PORT9,
            _ => unreachable!(),
        }
    }
}

/// Peripheral that can be used as a GPIO pin.
#[allow(private_bounds)]
pub trait Pin: PeripheralType + Into<AnyPin> + SealedPin + Sized + 'static {
    /// Number of the pin within the port, typically 0..16.
    ///
    /// Consult the reference manual tables 19.5–19.17 for more details.
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

impl From<PortFunction> for pac::pfs::vals::PortFunction {
    fn from(value: PortFunction) -> Self {
        Self::from_bits(value as u8)
    }
}

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

impl<'d> Output<'d> {
    /// Create GPIO output driver for a [Pin] with the provided [Level] and [DriveCapacity] configuration.
    #[inline]
    pub fn new(
        pin: Peri<'d, impl Pin>,
        initial_output: Level,
        drive_capacity: DriveCapacity,
    ) -> Self {
        let mut pin = Flex::new(pin);
        pin.set_as_output();
        pin.set_level(initial_output);
        pin.set_drive_capacity(drive_capacity);
        Self { pin }
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

    /// Sets the output drive capacity of a pin.
    ///
    /// Note: be aware of the maximum permissible combined current output for all pins, see [`DriveCapacity`] for more information.
    #[inline]
    pub fn set_drive_capacity(&mut self, drive_capacity: DriveCapacity) {
        self.pin.set_drive_capacity(drive_capacity)
    }

    /// What level output is set to?
    #[inline]
    pub fn get_output_level(&mut self) -> Level {
        self.pin.get_output_level()
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

    /// Sets the output drive capacity of a pin.
    ///
    /// Note: be aware of the maximum permissible combined current output for all pins, see [`DriveCapacity`] for more information.
    #[inline]
    pub fn set_drive_capacity(&mut self, drive_capacity: DriveCapacity) {
        self.pin.set_drive_capacity(drive_capacity)
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

    /// What level output is set to?
    #[inline]
    pub fn get_output_level(&mut self) -> Level {
        self.pin.get_output_level()
    }

    /// Toggle pin output.
    #[inline]
    pub fn toggle(&mut self) {
        self.pin.toggle();
    }

    /// Places pin in output mode.
    #[inline(never)]
    pub fn set_as_output(&mut self) {
        self.pin.set_as_output();
    }

    /// Places pin into input mode.
    #[inline(never)]
    pub fn set_as_input(&mut self) {
        self.pin.set_as_input();
    }

    /// Sets the pin into peripheral mode and selects port function `func`.
    pub fn set_as_pf(&mut self, func: PortFunction) {
        self.pin.set_as_pf(func);
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

impl_peripheral!(AnyPin);

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
    pin_impl!(410);
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    pin_impl!(411);
    #[cfg(feature = "_100pin")]
    pin_impl!(412);
    #[cfg(feature = "_100pin")]
    pin_impl!(413);
    #[cfg(feature = "_100pin")]
    pin_impl!(414);
    #[cfg(feature = "_100pin")]
    pin_impl!(415);

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
