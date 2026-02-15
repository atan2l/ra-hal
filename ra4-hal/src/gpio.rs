//! General Purpose Input/Output (`PORT`).

use core::{future::poll_fn, marker::PhantomData, task::Poll};

use crate::interrupt;
use crate::{
    event_link::{IcuInterrupt as _, InterruptEvent},
    interrupt::typelevel::{Handler as InterruptHandler, Interrupt as InterruptType},
    pac,
    write_protect::ProtectedModify as _,
};

use embassy_hal_internal::{Peri, PeripheralType, impl_peripheral, interrupt::InterruptExt as _};
use embassy_sync::waitqueue::AtomicWaker;
use ra4m1_ctpac::icu::vals::Fclksel;
use ra4m1_ctpac::{
    icu::vals::Irqmd,
    pfs::vals::{PortDirection, PortDrive, PortMode},
};

/// Uniquely identifies a pin.
///
/// Upper bits are the port index, lower bits are the pin index.
/// The `RA4M1` has 10 GPIO ports each with up to 16 pins.
#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PinId(u8);

impl PinId {
    /// Constructs a `PinId` from a 16-bit number e.g. `408`.
    #[inline(always)]
    pub const fn from_pin_number(combined_number: u16) -> Self {
        let port = (combined_number / 100) as u8;
        let pin = (combined_number % 100) as u8;
        Self::from_port_pin(port, pin)
    }

    /// Constructs a `PinId` from separate port and pin numbers e.g. `4` and `8`.
    #[inline(always)]
    pub const fn from_port_pin(port: u8, pin: u8) -> Self {
        assert!(port <= 15);
        assert!(pin <= 15);

        let pin_number = ((port & 0x0F) << 4) | (pin & 0x0F);
        Self(pin_number)
    }

    /// Returns the index of the port associated with the pin.
    #[inline(always)]
    pub const fn port(&self) -> u8 {
        (self.0 >> 4) & 0x0F
    }

    /// Returns the index of the pin within its associated port.
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        self.0 & 0x0F
    }
}

/// Output drive capacity.
///
/// For the `RA4M1` (Cortex-M4F) the maximum combined output of all pins is 60 mA, and each pin can be configured for max 4.0 mA or max 8.0 mA.
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

/// On a change in input level, which edge to trigger an interrupt on.
#[derive(Clone, Copy)]
pub enum GpioTrigger {
    #[allow(missing_docs)]
    Falling,
    #[allow(missing_docs)]
    Rising,
    #[allow(missing_docs)]
    Both,
}

/// Digital (debounce) filter configuration.
///
/// §13.2.1, §13.4.3
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Debounce {
    /// Turn the digital filter off
    Off,

    /// Event must last 1 peripheral clock cycle to trigger an interrupt.
    Min1,

    /// Event must last 8 peripheral clock cycles to trigger an interrupt.
    Min8,

    /// Event must last 32 peripheral clock cycles to trigger an interrupt.
    Min32,

    /// Event must last 64 peripheral clock cycles to trigger an interrupt.
    Min64,
}

/// Digital input or output level.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Level {
    #[allow(missing_docs)]
    Low,
    #[allow(missing_docs)]
    High,
}

/// Links a GPIO pin to an interrupt. See also [`InterruptFlex`].
#[allow(private_bounds)]
pub trait GpioIrq<I: InterruptiblePin>: SealedGpioIrq {}

pub(crate) trait SealedGpioIrq: PeripheralType {}

macro_rules! declare_port_irq {
    ($num:literal) => {
        paste::paste! {
            impl SealedGpioIrq for crate::peripherals::[< GPIO_IRQ $num >] {}
        }
    };
}

declare_port_irq!(0);
declare_port_irq!(1);
declare_port_irq!(2);
declare_port_irq!(3);
declare_port_irq!(4);
declare_port_irq!(5);
declare_port_irq!(6);
declare_port_irq!(7);
declare_port_irq!(8);
declare_port_irq!(9);
declare_port_irq!(10);
declare_port_irq!(11);
declare_port_irq!(12);
declare_port_irq!(14);
declare_port_irq!(15);

/// Type-erased GPIO pin
pub struct AnyPin {
    pub(crate) pin_port: PinId,
}

/// GPIO flexible pin.
///
/// This pin can be configured for input, output, or attached to a peripheral.
pub struct Flex<'d> {
    pin: Peri<'d, AnyPin>,
}

/// A flexible GPIO pin that can asynchronously wait for data.
pub struct InterruptFlex<'d, I: InterruptiblePin> {
    pin: Flex<'d>,
    phantom: PhantomData<&'d I>,
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
    #[doc = "Hi-Z / JTAG / SWD"]
    HiZ = 0x0,
    #[doc = "Low-Power Asynchronous General Purpose Timer"]
    Agt = 0x01,
    #[doc = "General PWM Timer"]
    Gpt1 = 0x02,
    #[doc = "General PWM Timer"]
    Gpt2 = 0x03,
    #[doc = "Serial Communications Interface"]
    Sci1 = 0x04,
    #[doc = "Serial Communications Interface"]
    Sci2 = 0x05,
    #[doc = "Serial Peripheral Interface"]
    Spi = 0x06,
    #[doc = "Inter-Integrated Circuit Bus Interface"]
    I2c = 0x07,
    #[doc = "Key Interrupt Function"]
    Kint = 0x08,
    #[doc = "CLKOUT / Analog Comparator / Real-Time Clock"]
    ClkCmpRtc = 0x09,
    #[doc = "Clock Frequency Accuracy / ADC14"]
    CacAdc = 0x0a,
    #[doc = "Capacitive Touch"]
    Ctsu = 0x0c,
    #[doc = "Segment LCD"]
    Slcdc = 0x0d,
    #[doc = "Controller Area Network Bus"]
    Can = 0x10,
    #[doc = "Serial Sound Interface Enhanced"]
    Ssie = 0x12,
    #[doc = "USB Full-Speed"]
    UsbFs = 0x13,
}

/// Interrupt handler for an [`InterruptFlex`] pin.
pub struct InputInterruptHandler<I: Pin> {
    _phantom: PhantomData<I>,
}

pub(crate) trait SealedPin {
    fn pin_port(&self) -> PinId;

    #[inline]
    fn _pin(&self) -> u8 {
        self.pin_port().pin()
    }

    #[inline]
    fn _port(&self) -> u8 {
        self.pin_port().port()
    }

    #[inline]
    fn set_drive_capacity(&self, drive_capacity: DriveCapacity) {
        let pfs = pac::PFS;
        let port_num = self._port() as _;
        let pin_num = self._pin() as _;
        let pfs_reg = pfs.pin(port_num, pin_num);

        pfs_reg.protected_modify(|w| match drive_capacity {
            DriveCapacity::Low => w.set_dscr(PortDrive::Low),
            DriveCapacity::Middle => w.set_dscr(PortDrive::Middle),
        });
    }

    fn set_pull_up(&self, pull_up: bool) {
        let pfs = pac::PFS;
        let port_num = self._port() as _;
        let pin_num = self._pin() as _;

        trace!("P{}{:02}: set_pull_up({})", port_num, pin_num, pull_up);

        let pfs_reg = pfs.pin(port_num, pin_num);

        pfs_reg.protected_modify(|w| w.set_pcr(pull_up));

        #[cfg(feature = "strict-assert")]
        assert_eq!(pfs_reg.read().pcr(), pull_up, "set_pull_up failed");

        trace!("P{}{:02}: {}", port_num, pin_num, pfs_reg.read());
    }

    /// Set the output as high.
    #[inline]
    fn set_high(&self) {
        let port = self.regs();

        port.pcntr3().write(|w| w.set_posr(self._pin() as _, true));
    }

    /// Set the output as low.
    #[inline]
    fn set_low(&self) {
        let port = self.regs();

        port.pcntr3().write(|w| w.set_porr(self._pin() as _, true));
    }

    /// Is the input high?
    #[inline]
    fn is_high(&self) -> bool {
        let port = self.regs();

        port.pcntr2().read().pidr(self._pin() as _)
    }

    /// Is the input low?
    #[inline]
    fn is_low(&self) -> bool {
        !self.is_high()
    }

    /// What level is the input?
    #[inline]
    fn level(&self) -> Level {
        let port = self.regs();

        match port.pcntr2().read().pidr(self._pin() as _) {
            false => Level::Low,
            true => Level::High,
        }
    }

    /// Set the output level.
    #[inline]
    fn set_level(&self, level: Level) {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }

    /// Is the pin output set as logic low?
    #[inline]
    fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }

    /// Is the pin output set as logic high?
    #[inline]
    fn is_set_high(&self) -> bool {
        let port = self.regs();
        port.pcntr1().read().podr(self._pin() as _)
    }

    /// What is the output level set to?
    #[inline]
    fn output_level(&self) -> Level {
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
        // It's possible to configure direction via PCNTR registers too.
        // It's simpler.
        // But we want to disable the pull-up resistor so PFS it is.
        // let port = self.block();
        // let pin = self._pin() as _;
        // port.pcntr1().modify(|w| {
        //     w.set_pdr(pin, true);
        // });

        let pfs = pac::PFS;
        let port_num = self._port() as _;
        let pin_num = self._pin() as _;

        trace!("P{}{:02}: → Output", port_num, pin_num);
        let pfs_reg = pfs.pin(port_num, pin_num);

        pfs_reg.protected_modify(|w| {
            w.set_pmr(PortMode::Gpio);
            w.set_pdr(PortDirection::Output);
            w.set_pcr(false);
        });

        #[cfg(feature = "strict-assert")]
        {
            let status = pfs_reg.read();
            assert_eq!(status.pmr(), PortMode::Gpio);
            assert_eq!(status.pdr(), PortDirection::Output);
            assert_eq!(status.pcr(), false);
        }
    }

    #[inline]
    fn set_as_input(&self, pull_up: bool) {
        let pfs = pac::PFS;
        let port_num = self._port() as _;
        let pin_num = self._pin() as _;

        let pfs_reg = pfs.pin(port_num, pin_num);

        pfs_reg.protected_modify(|w| {
            w.set_pmr(PortMode::Gpio);
            w.set_pdr(PortDirection::Input);
            w.set_pcr(pull_up);
        });

        #[cfg(feature = "strict-assert")]
        {
            let status = pfs_reg.read();
            assert_eq!(status.pmr(), PortMode::Gpio);
            assert_eq!(status.pdr(), PortDirection::Input);
            assert_eq!(status.pcr(), pull_up);
        }
    }

    /// Sets a pin to analog mode for use with the ADC.
    fn set_as_analog(&self) {
        let pfs = pac::PFS;

        let port_num = self._port() as _;
        let pin_num = self._pin() as _;
        let pfs_reg = pfs.pin(port_num, pin_num);

        pfs_reg.protected_modify(|w| {
            w.set_pdr(PortDirection::Input);
            w.set_asel(true);
            w.set_pcr(false);
        });

        #[cfg(feature = "strict-assert")]
        {
            let status = pfs_reg.read();
            assert_eq!(status.pdr(), PortDirection::Input);
            assert_eq!(status.asel(), true);
            assert_eq!(status.pcr(), false);
        }
    }

    fn set_as_pf(&self, port_func: PortFunction) {
        let port_num = self._port() as _;
        let pin_num = self._pin() as _;

        let pfs = crate::pac::PFS;

        let pfs_reg = pfs.pin(port_num, pin_num);

        // The quick design guide suggests that first setting the pin to GPIO ensure the peripheral doesn't get any spurious input
        pfs_reg.protected_modify(|w| w.set_pmr(PortMode::Gpio));

        pfs_reg.protected_modify(|w| {
            w.set_pmr(PortMode::Peripheral);
            w.set_psel(port_func.into());
        });

        debug!("P{}{:02}: {}", port_num, pin_num, pfs_reg.read());

        #[cfg(feature = "strict-assert")]
        {
            let status = pfs_reg.read();
            assert_eq!(status.pmr(), PortMode::Peripheral, "PSEL was ignored");
            assert_eq!(status.psel(), port_func.into(), "PSEL was ignored");
        }
    }

    /// Get the GPIO register block for this pin.
    fn regs(&self) -> crate::pac::port::Port;
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

    /// Encoded port+pin number of the pin
    #[inline]
    fn pin_port(&self) -> PinId {
        SealedPin::pin_port(self)
    }
}

/// A GPIO pin that can generate interrupts based on input events.
#[allow(private_bounds)]
pub trait InterruptiblePin: SealedIntPin + Pin {}

pub(crate) trait SealedIntPin {
    const INTERRUPT_EVENT: InterruptEvent;

    fn waker() -> &'static AtomicWaker;
}

impl From<PortFunction> for pac::pfs::vals::PortFunction {
    fn from(value: PortFunction) -> Self {
        Self::from_bits(value as u8)
    }
}

impl From<bool> for Level {
    fn from(value: bool) -> Self {
        match value {
            false => Self::Low,
            true => Self::High,
        }
    }
}

impl Pin for AnyPin {}

impl SealedPin for AnyPin {
    #[inline]
    fn pin_port(&self) -> PinId {
        self.pin_port
    }

    fn regs(&self) -> crate::pac::port::Port {
        match self._port() {
            0 => crate::pac::PORT0,
            1 => crate::pac::PORT1,
            2 => crate::pac::PORT2,
            3 => crate::pac::PORT3,
            4 => crate::pac::PORT4,
            5 => crate::pac::PORT5,
            6 => crate::pac::PORT6,
            7 => crate::pac::PORT7,
            8 => crate::pac::PORT8,
            9 => crate::pac::PORT9,
            _ => unreachable!(),
        }
    }
}

impl AnyPin {
    /// Unsafely create an `AnyPin` from a `PinId` number.
    ///
    /// # Safety
    ///
    /// This only works for valid port + pin combinations.
    /// Consult the reference manual to see which ports and pins your MCU has.
    #[inline]
    pub unsafe fn steal(pin_port: PinId) -> Peri<'static, Self> {
        // #[cfg(feature = "strict-assert")]
        // assert!(pin_port < 915);

        unsafe { Peri::new_unchecked(Self { pin_port }) }
    }

    /// Get the GPIO register block for this pin.
    #[cfg(feature = "unstable-pac")]
    #[inline]
    pub fn block(&self) -> crate::pac::port::Port {
        SealedPin::regs(self)
    }
}

impl<'d> Input<'d> {
    /// Create GPIO input driver for a [`Pin`].
    ///
    /// # Arguments
    /// * `pin` suitable input pin
    /// * `pull_up` enable the internal pull-up resistor
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>, pull_up: bool) -> Self {
        let mut pin = Flex::new(pin);

        pin.set_as_input(pull_up);

        Self { pin }
    }

    /// Enable the internal pull-up resistor?
    ///
    /// # Notes
    ///
    /// This will (maybe?) be ignored by the MCU unless the pin is configured for input.
    pub fn set_pull_up(&mut self, pull_up: bool) {
        self.pin.set_pull_up(pull_up);
    }

    /// Is the input level low?
    #[inline]
    pub fn is_low(&mut self) -> bool {
        self.pin.is_low()
    }

    /// Is the input level high?
    #[inline]
    pub fn is_high(&mut self) -> bool {
        self.pin.is_high()
    }

    /// Input level
    #[inline]
    pub fn level(&mut self) -> Level {
        self.pin.level()
    }
}

impl<'d> Output<'d> {
    /// Create GPIO output driver for a [`Pin`].
    ///
    /// # Arguments
    /// * `pin` the [`Pin`]
    /// * `initial` set the initial output level to logic low or high
    /// * `drive_capacity` sets the output current, see [`DriveCapacity`] for details.
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>, initial: Level, drive_capacity: DriveCapacity) -> Self {
        let mut pin = Flex::new(pin);

        pin.set_as_output();
        pin.set_level(initial);
        pin.set_drive_capacity(drive_capacity);

        Self { pin }
    }

    /// Sets the output drive capacity of a pin.
    ///
    /// # Notes
    ///
    /// Be aware of the maximum permissible combined current output for all pins, see [`DriveCapacity`] for more information.
    #[inline]
    pub fn set_drive_capacity(&mut self, drive_capacity: DriveCapacity) {
        self.pin.set_drive_capacity(drive_capacity)
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// Set the output level.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level)
    }

    /// Toggle pin output.
    #[inline]
    pub fn toggle(&mut self) {
        self.pin.toggle();
    }

    /// Is the pin output set as logic low?
    #[inline]
    pub fn is_set_low(&mut self) -> bool {
        self.pin.is_set_low()
    }

    /// Is the pin output set as logic high?
    #[inline]
    pub fn is_set_high(&mut self) -> bool {
        self.pin.is_set_high()
    }

    /// What is the output level set to?
    #[inline]
    pub fn output_level(&mut self) -> Level {
        self.pin.output_level()
    }
    /// Log current PFS state at `debug` level.
    #[inline]
    pub fn print_pfs_state(&self) {
        self.pin.print_pfs_state();
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

    /// Enable the internal pull-up resistor?
    ///
    /// # Notes
    ///
    /// This will (maybe?) be ignored by the MCU unless the pin is configured for input.
    pub fn set_pull_up(&mut self, pull_up: bool) {
        self.pin.set_pull_up(pull_up);
    }

    /// Places pin into input mode.
    ///
    /// # Arguments
    /// * `pull_up` enable the built-in pull-up resistor if true.
    #[inline(never)]
    pub fn set_as_input(&mut self, pull_up: bool) {
        self.pin.set_as_input(pull_up);
    }

    /// Is the input level low?
    #[inline]
    pub fn is_low(&mut self) -> bool {
        self.pin.is_low()
    }

    /// Is the input level high?
    #[inline]
    pub fn is_high(&mut self) -> bool {
        self.pin.is_high()
    }

    /// Input level
    #[inline]
    pub fn level(&mut self) -> Level {
        self.pin.level()
    }

    /// Places pin in output mode.
    #[inline(never)]
    pub fn set_as_output(&mut self) {
        self.pin.set_as_output();
    }

    /// Sets the output drive capacity of a pin.
    ///
    /// Note: be aware of the maximum permissible combined current output for all pins, see [`DriveCapacity`] for more information.
    #[inline]
    pub fn set_drive_capacity(&mut self, drive_capacity: DriveCapacity) {
        self.pin.set_drive_capacity(drive_capacity)
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// Set the output level.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level)
    }

    /// Toggle pin output.
    #[inline]
    pub fn toggle(&mut self) {
        self.pin.toggle();
    }

    /// Is the pin output set as logic low?
    #[inline]
    pub fn is_set_low(&mut self) -> bool {
        self.pin.is_set_low()
    }

    /// Is the pin output set as logic high?
    #[inline]
    pub fn is_set_high(&mut self) -> bool {
        self.pin.is_set_high()
    }

    /// What is the output level set to?
    #[inline]
    pub fn output_level(&mut self) -> Level {
        self.pin.output_level()
    }

    /// Sets a pin to analog mode for use with the ADC.
    pub fn set_as_analog(&mut self) {
        self.pin.set_as_analog();
    }

    /// Sets the pin into peripheral mode and selects port function `func`.
    pub fn set_as_pf(&mut self, func: PortFunction) {
        self.pin.set_as_pf(func);
    }

    /// Log current PFS state at `debug` level.
    pub fn print_pfs_state(&self) {
        #[cfg(feature = "defmt")]
        {
            let pfs = pac::PFS;
            let port_num = self.pin._port() as _;
            let pin_num = self.pin._pin() as _;

            // info!("Port{}, Pin{}, Output", port_num, pin_num);
            let pfs_reg = pfs.pin(port_num, pin_num);
            debug!("P{}{:02}: {}", port_num, pin_num, pfs_reg.read());
        }
    }
}

impl<'d> Drop for Flex<'d> {
    fn drop(&mut self) {
        trace!("P{}{:02} Flex::drop", self.pin.port(), self.pin._pin());
        self.set_as_input(false);
    }
}

impl<'d, I: InterruptiblePin> InterruptFlex<'d, I> {
    /// Create `InterruptFlex` from pin.
    #[inline]
    pub fn new<Int: InterruptType>(
        pin: Peri<'d, I>,
        _irq: Peri<'d, impl GpioIrq<I>>,
        _handler: impl interrupt::typelevel::Binding<Int, InputInterruptHandler<I>>,
    ) -> Self {
        unsafe { Int::IRQ.enable() };

        Int::IRQ.icu_enable(I::INTERRUPT_EVENT);
        let pfs = pac::PFS;
        let pin = Flex::new(pin);

        let port_num = pin.pin._port() as _;
        let pin_num = pin.pin._pin() as _;
        let pfs_reg = pfs.pin(port_num, pin_num);

        pfs_reg.protected_modify(|w| w.set_isel(true));

        Self {
            pin,
            phantom: PhantomData,
        }
    }

    // The RM says:
    // For a CPU interrupt or DTC trigger:
    // Change the IRQCRi register setting before setting the target IELSRn (n = 0 to 31).
    // You can change the register values only when the IELSRn.IELS[7:0] bits are 00h.
    //
    // But this doesn't seem to be the case?

    /// When to trigger the interrupt (edge detection only).
    ///
    /// §13.2.1
    pub fn set_trigger(&mut self, trigger: GpioTrigger) {
        let icu = pac::ICU;

        // GROSS
        let reg = icu.irqcr((I::INTERRUPT_EVENT as u8 - 1) as _);

        match trigger {
            GpioTrigger::Falling => reg.modify(|w| w.set_irqmd(Irqmd::FallingEdge)),
            GpioTrigger::Rising => reg.modify(|w| w.set_irqmd(Irqmd::RisingEdge)),
            GpioTrigger::Both => reg.modify(|w| w.set_irqmd(Irqmd::AnyEdge)),
        }
    }

    /// Configure the internal digital (debounce) filter.
    ///
    /// §13.2.1, §13.4.3
    pub fn set_debounce(&mut self, debounce: Debounce) {
        let icu = pac::ICU;

        // GROSS
        let reg = icu.irqcr((I::INTERRUPT_EVENT as u8 - 1) as _);

        match debounce {
            Debounce::Off => reg.modify(|w| w.set_flten(false)),
            Debounce::Min1 => {
                reg.modify(|w| {
                    w.set_flten(true);
                    w.set_fclksel(Fclksel::PCLKB_1);
                });
            }
            Debounce::Min8 => {
                reg.modify(|w| {
                    w.set_flten(true);
                    w.set_fclksel(Fclksel::PCLKB_8);
                });
            }
            Debounce::Min32 => {
                reg.modify(|w| {
                    w.set_flten(true);
                    w.set_fclksel(Fclksel::PCLKB_32);
                });
            }
            Debounce::Min64 => {
                reg.modify(|w| {
                    w.set_flten(true);
                    w.set_fclksel(Fclksel::PCLKB_64);
                });
            }
        }
    }

    /// Enable the internal pull-up resistor?
    /// # Notes
    ///
    /// This will (maybe?) be ignored by the MCU unless the pin is configured for input.
    pub fn set_pull_up(&mut self, pull_up: bool) {
        self.pin.set_pull_up(pull_up);
    }

    /// Places pin into input mode.
    ///
    /// # Arguments
    /// * `pull_up` enable the built-in pull-up resistor if true.
    #[inline(never)]
    pub fn set_as_input(&mut self, pull_up: bool) {
        self.pin.set_as_input(pull_up);
    }

    /// Waits for the pin to go low.
    ///
    /// # Notes
    ///
    /// If the pin is low when this function is called it will return immediately.
    /// See also [`set_trigger`](InterruptFlex::set_trigger).
    pub async fn wait_for_low(&mut self) {
        poll_fn(|cx| {
            if self.pin.is_low() {
                return Poll::Ready(());
            }
            I::waker().register(cx.waker());
            Poll::Pending
        })
        .await
    }

    /// Waits for the pin to go high.
    ///
    /// # Notes
    ///
    /// If the pin is high when this function is called it will return immediately.
    /// See also [`set_trigger`](InterruptFlex::set_trigger).
    pub async fn wait_for_high(&mut self) {
        poll_fn(|cx| {
            if self.pin.is_high() {
                return Poll::Ready(());
            }
            I::waker().register(cx.waker());
            Poll::Pending
        })
        .await
    }

    /// Waits for a change in input level and returns the level after the interrupt is fired.
    /// See also [`set_trigger`](InterruptFlex::set_trigger).
    pub async fn wait_for_event(&mut self) -> Level {
        let mut event = false;
        poll_fn(|cx| {
            if !event {
                event = true;
                I::waker().register(cx.waker());
                Poll::Pending
            } else {
                Poll::Ready(self.pin.level())
            }
        })
        .await
    }

    /// Is the input level low?
    #[inline]
    pub fn is_low(&mut self) -> bool {
        self.pin.is_low()
    }

    /// Is the input level high?
    #[inline]
    pub fn is_high(&mut self) -> bool {
        self.pin.is_high()
    }

    /// Input level
    #[inline]
    pub fn level(&mut self) -> Level {
        self.pin.level()
    }

    /// Places pin in output mode.
    #[inline(never)]
    pub fn set_as_output(&mut self) {
        self.pin.set_as_output();
    }

    /// Sets the output drive capacity of a pin.
    ///
    /// Note: be aware of the maximum permissible combined current output for all pins, see [`DriveCapacity`] for more information.
    #[inline]
    pub fn set_drive_capacity(&mut self, drive_capacity: DriveCapacity) {
        self.pin.set_drive_capacity(drive_capacity)
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// Set the output level.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level)
    }

    /// Toggle pin output.
    #[inline]
    pub fn toggle(&mut self) {
        self.pin.toggle();
    }

    /// Is the pin output set as logic low?
    #[inline]
    pub fn is_set_low(&mut self) -> bool {
        self.pin.is_set_low()
    }

    /// Is the pin output set as logic high?
    #[inline]
    pub fn is_set_high(&mut self) -> bool {
        self.pin.is_set_high()
    }

    /// What is the output level set to?
    #[inline]
    pub fn output_level(&mut self) -> Level {
        self.pin.output_level()
    }

    /// Sets the pin into peripheral mode and selects port function `func`.
    pub fn set_as_pf(&mut self, func: PortFunction) {
        self.pin.set_as_pf(func);
    }

    /// Log current PFS state at `debug` level.
    pub fn print_pfs_state(&self) {
        self.pin.print_pfs_state();
    }
}

macro_rules! pin_impl {
    ($pin_name:ident, $pin_number:literal, $port:ident) => {
        impl crate::gpio::Pin for crate::peripherals::$pin_name {}

        impl crate::gpio::SealedPin for crate::peripherals::$pin_name {
            fn pin_port(&self) -> crate::gpio::PinId {
                crate::gpio::PinId::from_pin_number($pin_number)
            }

            fn regs(&self) -> crate::pac::port::Port {
                crate::pac::$port
            }
        }

        impl From<crate::peripherals::$pin_name> for crate::gpio::AnyPin {
            fn from(val: crate::peripherals::$pin_name) -> Self {
                use crate::gpio::SealedPin as _;
                Self {
                    pin_port: val.pin_port(),
                }
            }
        }
    };
}
pub(crate) use pin_impl;

impl_peripheral!(AnyPin);

impl<Pin: InterruptiblePin, Int: InterruptType> InterruptHandler<Int>
    for InputInterruptHandler<Pin>
{
    unsafe fn on_interrupt() {
        Int::IRQ.icu_unpend();
        Pin::waker().wake()
    }
}
