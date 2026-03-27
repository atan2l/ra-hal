//! General Purpose Input/Output (`PORT`).

use core::{convert::Infallible, future::poll_fn, marker::PhantomData, task::Poll};

use embassy_hal_internal::{Peri, PeripheralType, impl_peripheral, interrupt::InterruptExt as _};
use embassy_sync::waitqueue::AtomicWaker;

use crate::{
    event_link::{IcuInterrupt as _, InterruptEvent},
    interrupt,
    interrupt::typelevel::{Handler as InterruptHandler, Interrupt as InterruptType},
    pac::{
        self,
        icu::vals::{Fclksel, Irqmd},
        pfs::vals::{OutputType, PortDirection, PortDrive, PortMode},
    },
    write_protect::ProtectedModify as _,
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
        // This isn't gated by "strict-assertions" because
        // SealedPin::regs() depends on port being valid.
        assert!(port <= 9);

        // A port may have less than 16 pins, but we're not
        // worried.
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

/// Output mode for a GPIO pin.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutputMode {
    /// Push-pull output
    PushPull,

    /// Open drain output
    OpenDrain,
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

    /// Event must last 1 `PCLKB` cycle to trigger an interrupt.
    Min1,

    /// Event must last 8 `PCLKB` cycles to trigger an interrupt.
    Min8,

    /// Event must last 32 `PCLKB` cycles to trigger an interrupt.
    Min32,

    /// Event must last 64 `PCLKB` cycles to trigger an interrupt.
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

/// Indicates what GPIO capabilities a pin has.
#[allow(private_bounds)]
pub trait ControlKind: SealedControlKind {}
trait SealedControlKind {}

/// GPIO pin with basic functionality
///
/// See [`Pin`] for a list of available pins.
pub enum Basic {}
impl ControlKind for Basic {}
impl SealedControlKind for Basic {}

/// GPIO pin that has a internal pull-up resistor but does not support open-drain output.
///
/// See [`PullUpPin`] for a list of compatible pins.
pub enum WithPullUp {}
impl ControlKind for WithPullUp {}
impl SealedControlKind for WithPullUp {}

/// GPIO pin that has a internal pull-up resistor and supports open-drain output.
///
/// See [`OpenDrainPin`] for a list of compatible pins.
pub enum WithOpenDrain {}
impl ControlKind for WithOpenDrain {}
impl SealedControlKind for WithOpenDrain {}

/// GPIO flexible pin.
///
/// This pin can be configured for input, output, or attached to a peripheral.
pub struct Flex<'d, C: ControlKind> {
    pin: Peri<'d, AnyPin>,
    phantom: PhantomData<C>,
}

/// A flexible GPIO pin that can asynchronously wait for data.
#[allow(private_bounds)]
pub struct InterruptFlex<'d, I: InterruptiblePin, C: ControlKind> {
    pin: Flex<'d, C>,
    phantom: PhantomData<&'d I>,
    phantom_k: PhantomData<C>,
}

/// GPIO input driver
pub struct Input<'d, C: ControlKind> {
    pin: Flex<'d, C>,
}

/// GPIO output driver
pub struct Output<'d, C: ControlKind> {
    pin: Flex<'d, C>,
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

    #[inline(always)]
    fn _pin(&self) -> u8 {
        self.pin_port().pin()
    }

    #[inline(always)]
    fn _port(&self) -> u8 {
        self.pin_port().port()
    }

    #[inline]
    fn set_drive_capacity(&self, drive_capacity: DriveCapacity) {
        let pfs = pac::PFS;
        let port_num = self._port() as _;
        let pin_num = self._pin() as _;
        let pfs_reg = pfs.pin(port_num, pin_num);

        pfs_reg.protected_modify(|r| match drive_capacity {
            DriveCapacity::Low => r.set_dscr(PortDrive::Low),
            DriveCapacity::Middle => r.set_dscr(PortDrive::Middle),
        });
    }

    fn set_output_mode(&self, output_mode: OutputMode) {
        let pfs = pac::PFS;
        let port_num = self._port() as _;
        let pin_num = self._pin() as _;
        let pfs_reg = pfs.pin(port_num, pin_num);

        match output_mode {
            OutputMode::PushPull => pfs_reg.protected_modify(|r| r.set_ncodr(OutputType::Cmos)),
            OutputMode::OpenDrain => pfs_reg.protected_modify(|r| r.set_ncodr(OutputType::Nmos)),
        }
    }

    fn set_pull_up(&self, pull_up: bool) {
        let pfs = pac::PFS;
        let port_num = self._port() as _;
        let pin_num = self._pin() as _;
        let pfs_reg = pfs.pin(port_num, pin_num);

        trace!("P{}{:02}: set_pull_up({})", port_num, pin_num, pull_up);

        // TODO: §19.5.5
        // When the P402, P403, and P404 pins are configured as outputs or inputs with the internal
        // pull-up resistor, set the VBTCR1.BPWSWSTP bit to 1 before setting the I/O registers
        // regardless of whether or not the battery backup function is used. This setting is only
        // required one time after a power-on reset. Clear the VBTCR1.BPWSWSTP bit to 0 again after
        // setting registers associated with the battery backup function, when using the battery
        // backup function.

        pfs_reg.protected_modify(|r| r.set_pcr(pull_up));

        #[cfg(feature = "strict-assert")]
        assert_eq!(pfs_reg.read().pcr(), pull_up, "set_pull_up failed");

        trace!("P{}{:02}: {}", port_num, pin_num, pfs_reg.read());
    }

    /// Set the output as high.
    #[inline(always)]
    fn set_high(&self) {
        let port = self.regs();
        port.pcntr3().write(|r| r.set_posr(self._pin() as _, true));
    }

    /// Set the output as low.
    #[inline(always)]
    fn set_low(&self) {
        let port = self.regs();
        port.pcntr3().write(|r| r.set_porr(self._pin() as _, true));
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
    #[inline(always)]
    fn set_level(&self, level: Level) {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }

    /// Is the pin output set as logic low?
    #[inline(always)]
    fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }

    /// Is the pin output set as logic high?
    #[inline(always)]
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
    #[inline(always)]
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
        // port.pcntr1().modify(|r| {
        //     r.set_pdr(pin, true);
        // });

        let pfs = pac::PFS;
        let port_num = self._port() as _;
        let pin_num = self._pin() as _;
        let pfs_reg = pfs.pin(port_num, pin_num);

        trace!("P{}{:02}: → Output", port_num, pin_num);

        // TODO: §19.5.7
        // When P914 and P915 are used as GPIO pins, their operation is affected by the pull-up /
        // pull-down function of the USBFS registers. Therefore, before using the GPIO function,
        // disable the pull-up and pull-down control of the USBFS registers using the SYSCFG.DMRPU,
        // SYSCFG.DPRPU and SYSCFG.DRPD bits.

        pfs_reg.protected_modify(|r| {
            r.set_pmr(PortMode::Gpio);
            r.set_pdr(PortDirection::Output);
            r.set_pcr(false);
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
    fn set_as_input(&self) {
        let pfs = pac::PFS;
        let port_num = self._port() as _;
        let pin_num = self._pin() as _;
        let pfs_reg = pfs.pin(port_num, pin_num);

        trace!("P{}{:02}: → Input", port_num, pin_num);

        // TODO: §19.5.7
        // When P914 and P915 are used as GPIO pins, their operation is affected by the pull-up /
        // pull-down function of the USBFS registers. Therefore, before using the GPIO function,
        // disable the pull-up and pull-down control of the USBFS registers using the SYSCFG.DMRPU,
        // SYSCFG.DPRPU and SYSCFG.DRPD bits.

        pfs_reg.protected_modify(|r| {
            r.set_pmr(PortMode::Gpio);
            r.set_pdr(PortDirection::Input);
        });

        #[cfg(feature = "strict-assert")]
        {
            let status = pfs_reg.read();
            assert_eq!(status.pmr(), PortMode::Gpio);
            assert_eq!(status.pdr(), PortDirection::Input);
        }
    }

    /// Sets a pin to analog mode for use with the ADC.
    fn set_as_analog(&self) {
        let pfs = pac::PFS;

        let port_num = self._port() as _;
        let pin_num = self._pin() as _;
        let pfs_reg = pfs.pin(port_num, pin_num);

        pfs_reg.protected_modify(|r| {
            r.set_pdr(PortDirection::Input);
            r.set_asel(true);
            r.set_pcr(false);
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
        pfs_reg.protected_modify(|r| r.set_pmr(PortMode::Gpio));

        pfs_reg.protected_modify(|r| {
            r.set_pmr(PortMode::Peripheral);
            r.set_psel(port_func.into());
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

/// A GPIO pin with an internal pull-up resistor.
///
/// # Notes
/// Pull-ups do not work when a pin is assigned to a peripheral.
#[allow(private_bounds)]
pub trait PullUpPin: SealedPullUpPin {}

pub(crate) trait SealedPullUpPin: Pin {}

/// A GPIO pin with an internal pull-up resistor and the ability to be configured for open-drain output.
///
/// # Notes
/// Pull-ups do not work when a pin is assigned to a peripheral.
#[allow(private_bounds)]
pub trait OpenDrainPin: SealedOpenDrainPin {}

pub(crate) trait SealedOpenDrainPin: Pin {}

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
    #[inline(always)]
    fn pin_port(&self) -> PinId {
        self.pin_port
    }

    #[inline(always)]
    fn regs(&self) -> crate::pac::port::Port {
        // This is safe because we know the ports are laid out
        // contiguously at fixed intervals and that the PinId
        // constructor won't allow an invalid port number.
        let port_ptr = crate::pac::PORT0.as_ptr() as usize;
        let offset = 0x20 * self._port() as usize;
        unsafe { crate::pac::port::Port::from_ptr(port_ptr.wrapping_add(offset) as _) }
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
        unsafe { Peri::new_unchecked(Self { pin_port }) }
    }

    /// Get the GPIO register block for this pin.
    #[cfg(feature = "unstable-pac")]
    #[inline]
    pub fn block(&self) -> crate::pac::port::Port {
        SealedPin::regs(self)
    }
}

impl<'d> Input<'d, Basic> {
    /// Create GPIO input driver for a [`Pin`] without pull-up capabilities.
    ///
    /// # Arguments
    /// * `pin` the [`Pin`]
    pub fn new_basic(pin: Peri<'d, impl Pin>) -> Self {
        Self::new(pin)
    }
}

impl<'d> Input<'d, WithPullUp> {
    /// Create GPIO input driver for a [`Pin`] with the ability to toggle an internal pull-up resistor.
    ///
    /// # Arguments
    /// * `pin` suitable input pin
    /// * `pull_up` enable the internal pull-up resistor
    #[inline]
    pub fn new_with_pull_up(pin: Peri<'d, impl Pin>, pull_up: bool) -> Self {
        let mut this = Self::new(pin);

        this.set_pull_up(pull_up);

        this
    }

    /// Enable the internal pull-up resistor?
    ///
    /// # Notes
    ///
    /// This is ignored by the MCU unless the pin is configured for input.
    pub fn set_pull_up(&mut self, pull_up: bool) {
        self.pin.set_pull_up(pull_up);
    }
}

impl<'d, C: ControlKind> Input<'d, C> {
    /// Create GPIO input driver for a [`Pin`].
    ///
    /// # Arguments
    /// * `pin` suitable input pin
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>) -> Self {
        let mut pin = Flex::new(pin);

        pin.set_as_input();

        Self { pin }
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

impl<'d> Output<'d, Basic> {
    /// Create GPIO output driver for a [`Pin`] without pull-up or open-drain capabilities.
    ///
    /// # Arguments
    /// * `pin` the [`Pin`]
    /// * `initial` set the initial output level to logic low or high
    /// * `drive_capacity` sets the output current, see [`DriveCapacity`] for details.
    pub fn new_basic(
        pin: Peri<'d, impl Pin>,
        initial: Level,
        drive_capacity: DriveCapacity,
    ) -> Self {
        Self::new(pin, initial, drive_capacity)
    }
}

impl<'d> Output<'d, WithOpenDrain> {
    /// Create GPIO output driver for a [`Pin`] that supports open-drain output.
    ///
    /// # Arguments
    /// * `pin` the [`Pin`]
    /// * `initial` set the initial output level to logic low or high
    /// * `drive_capacity` sets the output current, see [`DriveCapacity`] for details.
    #[inline]
    pub fn new_with_open_drain(
        pin: Peri<'d, impl Pin>,
        initial: Level,
        drive_capacity: DriveCapacity,
        output_mode: OutputMode,
    ) -> Self {
        let mut this = Self::new(pin, initial, drive_capacity);

        this.set_output_mode(output_mode);

        this
    }

    /// Sets a pin's output mode to either open-drain or push-pull.
    #[inline]
    pub fn set_output_mode(&mut self, output_mode: OutputMode) {
        self.pin.set_output_mode(output_mode);
    }
}

impl<'d, C: ControlKind> Output<'d, C> {
    /// Create GPIO output driver for a [`Pin`].
    ///
    /// # Arguments
    /// * `pin` the [`Pin`]
    /// * `initial` set the initial output level to logic low or high
    /// * `drive_capacity` sets the output current, see [`DriveCapacity`] for details.
    #[inline]
    pub(crate) fn new(
        pin: Peri<'d, impl Pin>,
        initial: Level,
        drive_capacity: DriveCapacity,
    ) -> Self {
        let mut this = Flex::new(pin);

        this.set_as_output();
        this.set_level(initial);
        this.set_drive_capacity(drive_capacity);

        Self { pin: this }
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
    #[inline(always)]
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    /// Set the output as high.
    #[inline(always)]
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// Set the output level.
    #[inline(always)]
    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level)
    }

    /// Toggle pin output.
    #[inline(always)]
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

impl<'d> Flex<'d, WithPullUp> {
    /// Create `Flex` from pin with the ability to toggle an internal pull-up resistor.
    #[inline]
    pub fn new_with_pull_up(pin: Peri<'d, impl PullUpPin>, pull_up: bool) -> Self {
        let mut this = Self {
            pin: pin.into(),
            phantom: PhantomData,
        };

        this.set_pull_up(pull_up);

        trace!(
            "Flex<WithPullUp>: port={}, pin={}",
            this.pin._port(),
            this.pin._pin()
        );

        this
    }

    /// Enable the internal pull-up resistor?
    ///
    /// # Notes
    ///
    /// This will be ignored by the MCU unless the pin is configured for input.
    pub fn set_pull_up(&mut self, pull_up: bool) {
        self.pin.set_pull_up(pull_up);
    }
}

impl<'d> Flex<'d, WithOpenDrain> {
    /// Create `Flex` from pin with support for an internal pull-up resistor and open-drain capability.
    #[inline]
    pub fn new_with_open_drain(
        pin: Peri<'d, impl OpenDrainPin>,
        pull_up: bool,
        output_mode: OutputMode,
    ) -> Self {
        let mut this = Self {
            pin: pin.into(),
            phantom: PhantomData,
        };

        this.set_pull_up(pull_up);
        this.set_output_mode(output_mode);

        trace!(
            "Flex<OpenDrain>: port={}, pin={}",
            this.pin._port(),
            this.pin._pin()
        );

        this
    }

    /// Enable the internal pull-up resistor?
    ///
    /// # Notes
    ///
    /// This will be ignored by the MCU unless the pin is configured for input.
    #[inline]
    pub fn set_pull_up(&mut self, pull_up: bool) {
        self.pin.set_pull_up(pull_up);
    }

    /// Sets a pin's output mode to either open-drain or push-pull.
    #[inline]
    pub fn set_output_mode(&mut self, output_mode: OutputMode) {
        self.pin.set_output_mode(output_mode);
    }
}

impl<'d> Flex<'d, Basic> {
    /// Create `Flex` from pin with no internal pull-up resistor or open-drain capability.
    #[inline]
    pub fn new_basic(pin: Peri<'d, impl Pin>) -> Self {
        let this = Self {
            pin: pin.into(),
            phantom: PhantomData,
        };

        trace!(
            "Flex<Basic>: port={}, pin={}",
            this.pin._port(),
            this.pin._pin()
        );

        this
    }
}

impl<'d, C: ControlKind> Flex<'d, C> {
    /// Create `Flex` from pin.
    #[inline]
    pub(crate) fn new(pin: Peri<'d, impl Pin>) -> Self {
        let this = Self {
            pin: pin.into(),
            phantom: PhantomData,
        };

        trace!(
            "Flex<Basic>: port={}, pin={}",
            this.pin._port(),
            this.pin._pin()
        );

        this
    }

    /// Places pin into input mode.
    #[inline]
    pub fn set_as_input(&mut self) {
        self.pin.set_as_input();
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
    #[inline]
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
    #[inline(always)]
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    /// Set the output as high.
    #[inline(always)]
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// Set the output level.
    #[inline(always)]
    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level)
    }

    /// Toggle pin output.
    #[inline(always)]
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

impl<'d, C: ControlKind> Drop for Flex<'d, C> {
    fn drop(&mut self) {
        trace!("P{}{:02} Flex::drop", self.pin.port(), self.pin._pin());
        self.set_as_input();
        self.pin.set_pull_up(false);
    }
}

impl<'d, I: InterruptiblePin> InterruptFlex<'d, I, WithPullUp> {
    /// Enable the internal pull-up resistor?
    ///
    /// # Notes
    /// This is ignored by the MCU unless the pin is configured for input.
    pub fn set_pull_up(&mut self, pull_up: bool) {
        self.pin.set_pull_up(pull_up);
    }
}

impl<'d, I: InterruptiblePin, C: ControlKind> InterruptFlex<'d, I, C> {
    /// Creates an `InterruptFlex` configured for input that detects falling edge events and has the debounce filter disabled.
    #[inline]
    pub fn new<Int: InterruptType>(
        pin: Peri<'d, I>,
        gpio_irq: Peri<'d, impl GpioIrq<I>>,
        irq: impl interrupt::typelevel::Binding<Int, InputInterruptHandler<I>>,
    ) -> Self {
        let _ = gpio_irq;
        let _ = irq;

        // Safety: Interrupt handlers are defined by the irq argument and thus the interrupt is safe to enable.
        unsafe {
            Int::IRQ.enable();
            Int::IRQ.icu_enable(I::INTERRUPT_EVENT);
        }

        let pfs = pac::PFS;
        let pin = Flex::new(pin);

        let port_num = pin.pin._port() as _;
        let pin_num = pin.pin._pin() as _;
        let pfs_reg = pfs.pin(port_num, pin_num);

        pfs_reg.protected_modify(|r| r.set_isel(true));

        let mut this = Self {
            pin,
            phantom: PhantomData,
            phantom_k: PhantomData,
        };

        this.set_as_input();
        this.set_trigger(GpioTrigger::Falling);
        this.set_debounce(Debounce::Off);

        this
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
            GpioTrigger::Falling => reg.modify(|r| r.set_irqmd(Irqmd::FallingEdge)),
            GpioTrigger::Rising => reg.modify(|r| r.set_irqmd(Irqmd::RisingEdge)),
            GpioTrigger::Both => reg.modify(|r| r.set_irqmd(Irqmd::AnyEdge)),
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
            Debounce::Off => reg.modify(|r| r.set_flten(false)),
            Debounce::Min1 => {
                reg.modify(|r| {
                    r.set_flten(true);
                    r.set_fclksel(Fclksel::PCLKB_1);
                });
            }
            Debounce::Min8 => {
                reg.modify(|r| {
                    r.set_flten(true);
                    r.set_fclksel(Fclksel::PCLKB_8);
                });
            }
            Debounce::Min32 => {
                reg.modify(|r| {
                    r.set_flten(true);
                    r.set_fclksel(Fclksel::PCLKB_32);
                });
            }
            Debounce::Min64 => {
                reg.modify(|r| {
                    r.set_flten(true);
                    r.set_fclksel(Fclksel::PCLKB_64);
                });
            }
        }
    }

    /// Places pin into input mode.
    ///
    /// # Arguments
    /// * `pull_up` enable the built-in pull-up resistor if true.
    #[inline]
    pub fn set_as_input(&mut self) {
        self.pin.set_as_input();
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
    #[inline]
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

macro_rules! gpio_pin {
    ($pin_name:ident, $pin_number:literal, $port:ident) => {
        impl crate::gpio::Pin for crate::peripherals::$pin_name {}

        impl crate::gpio::SealedPin for crate::peripherals::$pin_name {
            #[inline(always)]
            fn pin_port(&self) -> crate::gpio::PinId {
                crate::gpio::PinId::from_pin_number($pin_number)
            }

            #[inline(always)]
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
pub(crate) use gpio_pin;

impl_peripheral!(AnyPin);

impl<Pin: InterruptiblePin, Int: InterruptType> InterruptHandler<Int>
    for InputInterruptHandler<Pin>
{
    unsafe fn on_interrupt() {
        Int::IRQ.icu_unpend();
        Pin::waker().wake()
    }
}

impl<'d, C: ControlKind> embedded_hal_1::digital::ErrorType for Input<'d, C> {
    type Error = Infallible;
}

impl<'d, C: ControlKind> embedded_hal_1::digital::InputPin for Input<'d, C> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

impl<'d, C: ControlKind> embedded_hal_1::digital::ErrorType for Output<'d, C> {
    type Error = Infallible;
}

impl<'d, C: ControlKind> embedded_hal_1::digital::OutputPin for Output<'d, C> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }
}

impl<'d, C: ControlKind> embedded_hal_1::digital::StatefulOutputPin for Output<'d, C> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_set_high())
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}
