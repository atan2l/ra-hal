//! Option Function Select Register 0 (`OFS0`)
//!
//! `OFS0` configures the after-reset behavior of the Independent Watchdog Timer (`IWDT`) and Watchdog Timer (`WDT`).
//! See §6.2.1, §25.3.7 in the reference manual for more details.
//!
//! Along with [`Ofs1`](struct@crate::osm::ofs1::Ofs1) and [`SecurityMpu`](struct@crate::osm::sec_mpu::SecurityMpu) the [`Ofs0`] struct must be instantiated exactly once and saved in a specific linker section.
//! e.g.:
//! ```rust,ignore
//! #[unsafe(no_mangle)]
//! #[unsafe(link_section = ".ofs0")]
//! static OFS0: Ofs0 = Ofs0::ArduinoCore();
//! ```

const OFS0_H: u32 = 0xA0010000;
const OFS0_L: u32 = 0x0000A001;

/// Option Function Select Register 0
pub struct Ofs0(#[allow(unused)] u32);

trait IwdtAutoStartMode {
    const OFS0: u32;
}

/// `IWDTSTRT` Automatically activate `IWDT` after reset (auto-start mode).
pub enum IwdtAutoStartOn {}
impl IwdtAutoStartMode for IwdtAutoStartOn {
    const OFS0: u32 = 0 << 1;
}

/// `IWDTSTRT` Do not automatically activate `IWDT` after reset.
pub enum IwdtAutoStartOff {}
impl IwdtAutoStartMode for IwdtAutoStartOff {
    const OFS0: u32 = 1 << 1;
}

trait IwdtTimeoutPeriod {
    const OFS0: u32;
}

/// `IWDTTOPS` Timeout period = 128 cycles (`0x007F`)
pub enum IwdtTimeout128 {}
impl IwdtTimeoutPeriod for IwdtTimeout128 {
    const OFS0: u32 = 0b00 << 2;
}

/// `IWDTTOPS` Timeout period = 512 cycles (`0x01FF`)
pub enum IwdtTimeout512 {}
impl IwdtTimeoutPeriod for IwdtTimeout512 {
    const OFS0: u32 = 0b01 << 2;
}

/// `IWDTTOPS` Timeout period = 1024 cycles (`0x03FF`)
pub enum IwdtTimeout1024 {}
impl IwdtTimeoutPeriod for IwdtTimeout1024 {
    const OFS0: u32 = 0b10 << 2;
}

/// `IWDTTOPS` Timeout period = 2048 cycles (`0x07FF`)
pub enum IwdtTimeout2048 {}
impl IwdtTimeoutPeriod for IwdtTimeout2048 {
    const OFS0: u32 = 0b11 << 2;
}

trait IwdtClockRatio {
    const OFS0: u32;
}

/// `IWDTCKS` Clock ratio = 1
pub enum IwdtClockRatio1 {}
impl IwdtClockRatio for IwdtClockRatio1 {
    const OFS0: u32 = 0b0000 << 4;
}

/// `IWDTCKS` Clock ratio = 1/16
pub enum IwdtClockRatio16 {}
impl IwdtClockRatio for IwdtClockRatio16 {
    const OFS0: u32 = 0b0010 << 4;
}

/// `IWDTCKS` Clock ratio = 1/32
pub enum IwdtClockRatio32 {}
impl IwdtClockRatio for IwdtClockRatio32 {
    const OFS0: u32 = 0b0011 << 4;
}

/// `IWDTCKS` Clock ratio = 1/64
pub enum IwdtClockRatio64 {}
impl IwdtClockRatio for IwdtClockRatio64 {
    const OFS0: u32 = 0b0100 << 4;
}

/// `IWDTCKS` Clock ratio = 1/128
pub enum IwdtClockRatio128 {}
impl IwdtClockRatio for IwdtClockRatio128 {
    const OFS0: u32 = 0b1111 << 4;
}

/// `IWDTCKS` Clock ratio = 1/256
pub enum IwdtClockRatio256 {}
impl IwdtClockRatio for IwdtClockRatio256 {
    const OFS0: u32 = 0b0101 << 4;
}

trait IwdtWindowEndPosition {
    const OFS0: u32;
}

/// `IWDTRPES` Window end position = 75%
pub enum IwdtWindowEnd75 {}
impl IwdtWindowEndPosition for IwdtWindowEnd75 {
    const OFS0: u32 = 0b00 << 8;
}

/// `IWDTRPES` Window end position = 50%
pub enum IwdtWindowEnd50 {}
impl IwdtWindowEndPosition for IwdtWindowEnd50 {
    const OFS0: u32 = 0b01 << 8;
}

/// `IWDTRPES` Window end position = 25%
pub enum IwdtWindowEnd25 {}
impl IwdtWindowEndPosition for IwdtWindowEnd25 {
    const OFS0: u32 = 0b10 << 8;
}

/// `IWDTRPES` Window end position = 0% (no window end position)
pub enum IwdtWindowEndNone {}
impl IwdtWindowEndPosition for IwdtWindowEndNone {
    const OFS0: u32 = 0b11 << 8;
}

trait IwdtWindowStartPosition {
    const OFS0: u32;
}

/// `IWDTRPSS` Window start position = 25%
pub enum IwdtWindowStart25 {}
impl IwdtWindowStartPosition for IwdtWindowStart25 {
    const OFS0: u32 = 0b00 << 10;
}

/// `IWDTRPSS` Window start position = 50%
pub enum IwdtWindowStart50 {}
impl IwdtWindowStartPosition for IwdtWindowStart50 {
    const OFS0: u32 = 0b01 << 10;
}

/// `IWDTRPSS` Window start position = 75%
pub enum IwdtWindowStart75 {}
impl IwdtWindowStartPosition for IwdtWindowStart75 {
    const OFS0: u32 = 0b10 << 10;
}

/// `IWDTRPSS` Window start position = 100% (no window start position)
pub enum IwdtWindowStartNone {}
impl IwdtWindowStartPosition for IwdtWindowStartNone {
    const OFS0: u32 = 0b11 << 10;
}

trait IwdtResetOrInterrupt {
    const OFS0: u32;
}

/// `IWDTRSTIRQS` Interrupt
pub enum IwdtInterrupt {}
impl IwdtResetOrInterrupt for IwdtInterrupt {
    const OFS0: u32 = 0 << 12;
}

/// `IWDRSTIRQST` Reset
pub enum IwdtReset {}
impl IwdtResetOrInterrupt for IwdtReset {
    const OFS0: u32 = 1 << 12;
}

trait IwdtStopControl {
    const OFS0: u32;
}

/// `IWDTSTPCTL` Continue counting when in sleep, snooze, or software standby mode.
pub enum IwdtSleepContinue {}
impl IwdtStopControl for IwdtSleepContinue {
    const OFS0: u32 = 0 << 14;
}

/// `IWDTSTPCTL` Stop counting when in sleep, snooze, or software standby mode.
pub enum IwdtSleepStop {}
impl IwdtStopControl for IwdtSleepStop {
    const OFS0: u32 = 1 << 14;
}

trait WdtAutoStartMode {
    const OFS0: u32;
}

/// `WDTSTRT` Automatically activate `WDT` after reset (auto-start mode).
pub enum WdtAutoStartOn {}
impl WdtAutoStartMode for WdtAutoStartOn {
    const OFS0: u32 = 0 << 17;
}

/// `WDTSTRT` Do not automatically activate `WDT` after reset.
pub enum WdtAutoStartOff {}
impl WdtAutoStartMode for WdtAutoStartOff {
    const OFS0: u32 = 1 << 17;
}

trait WdtTimeoutPeriod {
    const OFS0: u32;
}

/// `WDTTOPS` Timeout period = 1024 cycles (`0x03FF`)
pub enum WdtTimeout1024 {}
impl WdtTimeoutPeriod for WdtTimeout1024 {
    const OFS0: u32 = 0b00 << 18;
}

/// `WDTTOPS` Timeout period = 4096 cycles (`0x0FFF`)
pub enum WdtTimeout4096 {}
impl WdtTimeoutPeriod for WdtTimeout4096 {
    const OFS0: u32 = 0b01 << 18;
}

/// `WDTTOPS` Timeout period = 8192 cycles (`0x1FFF`)
pub enum WdtTimeout8192 {}
impl WdtTimeoutPeriod for WdtTimeout8192 {
    const OFS0: u32 = 0b10 << 18;
}

/// `WDTTOPS` Timeout period = 16,384 cycles (`0x3FFF`)
pub enum WdtTimeout16384 {}
impl WdtTimeoutPeriod for WdtTimeout16384 {
    const OFS0: u32 = 0b10 << 18;
}

trait WdtClockRatio {
    const OFS0: u32;
}

/// `WDTCKS` Clock ratio = PCLKB/4
pub enum WdtClockRatio4 {}
impl WdtClockRatio for WdtClockRatio4 {
    const OFS0: u32 = 0b0001 << 20;
}

/// `WDTCKS` Clock ratio = PCLKB/64
pub enum WdtClockRatio64 {}
impl WdtClockRatio for WdtClockRatio64 {
    const OFS0: u32 = 0b0100 << 20;
}

/// `WDTCKS` Clock ratio = PCLKB/128
pub enum WdtClockRatio128 {}
impl WdtClockRatio for WdtClockRatio128 {
    const OFS0: u32 = 0b1111 << 20;
}

/// `WDTCKS` Clock ratio = PCLKB/512
pub enum WdtClockRatio512 {}
impl WdtClockRatio for WdtClockRatio512 {
    const OFS0: u32 = 0b1110 << 20;
}

/// `WDTCKS` Clock ratio = PCLKB/2048
pub enum WdtClockRatio2048 {}
impl WdtClockRatio for WdtClockRatio2048 {
    const OFS0: u32 = 0b0111 << 20;
}

/// `WDTCKS` Clock ratio = PCLKB/8192
pub enum WdtClockRatio8192 {}
impl WdtClockRatio for WdtClockRatio8192 {
    const OFS0: u32 = 0b1000 << 20;
}

trait WdtWindowEndPosition {
    const OFS0: u32;
}

/// `WDTRPES` Window end position = 75%
pub enum WdtWindowEnd75 {}
impl WdtWindowEndPosition for WdtWindowEnd75 {
    const OFS0: u32 = 0b00 << 24;
}

/// `WDTRPES` Window end position = 50%
pub enum WdtWindowEnd50 {}
impl WdtWindowEndPosition for WdtWindowEnd50 {
    const OFS0: u32 = 0b01 << 24;
}

/// `WDTRPES` Window end position = 25%
pub enum WdtWindowEnd25 {}
impl WdtWindowEndPosition for WdtWindowEnd25 {
    const OFS0: u32 = 0b10 << 24;
}

/// `WDTRPES` Window end position = 0% (no window end position)
pub enum WdtWindowEndNone {}
impl WdtWindowEndPosition for WdtWindowEndNone {
    const OFS0: u32 = 0b11 << 24;
}

trait WdtWindowStartPosition {
    const OFS0: u32;
}

/// `WDTRPSS` Window start position = 25%
pub enum WdtWindowStart25 {}
impl WdtWindowStartPosition for WdtWindowStart25 {
    const OFS0: u32 = 0b00 << 26;
}

/// `WDTRPSS` Window start position = 50%
pub enum WdtWindowStart50 {}
impl WdtWindowStartPosition for WdtWindowStart50 {
    const OFS0: u32 = 0b01 << 26;
}

/// `WDTRPSS` Window start position = 75%
pub enum WdtWindowStart75 {}
impl WdtWindowStartPosition for WdtWindowStart75 {
    const OFS0: u32 = 0b10 << 26;
}

/// `WDTRPSS` Window start position = 100% (no window start position)
pub enum WdtWindowStartNone {}
impl WdtWindowStartPosition for WdtWindowStartNone {
    const OFS0: u32 = 0b11 << 26;
}

trait WdtResetOrInterrupt {
    const OFS0: u32;
}

/// `WDTRSTIRQS` Interrupt
pub enum WdtInterrupt {}
impl WdtResetOrInterrupt for WdtInterrupt {
    const OFS0: u32 = 0 << 28;
}

/// `WDRSTIRQST` Reset
pub enum WdtReset {}
impl WdtResetOrInterrupt for WdtReset {
    const OFS0: u32 = 1 << 28;
}

trait WdtStopControl {
    const OFS0: u32;
}

/// `WDTSTPCTL` Continue counting when in sleep, snooze, or software standby mode.
pub enum WdtSleepContinue {}
impl WdtStopControl for WdtSleepContinue {
    const OFS0: u32 = 0 << 30;
}

/// `WDTSTPCTL` Stop counting when in sleep, snooze, or software standby mode.
pub enum WdtSleepStop {}
impl WdtStopControl for WdtSleepStop {
    const OFS0: u32 = 1 << 30;
}

impl Ofs0 {
    /// Generates a value to configure `OFS0` with explicit configuration settings.
    ///
    /// ```rust,ignore
    /// #[unsafe(no_mangle)]
    /// #[unsafe(link_section = ".ofs0")]
    /// static OFS0: Ofs0 = Ofs0::new::<
    ///   IwdtAutoStartOff,
    ///   IwdtTimeout2048,
    ///   IwdtClockRatio128,
    ///   IwdtWindowEndNone,
    ///   IwdtWindowStartNone,
    ///   IwdtReset,
    ///   IwdtSleepStop,
    ///   WdtAutoStartOff,
    ///   WdtTimeout16384,
    ///   WdtClockRatio128,
    ///   WdtWindowEndNone,
    ///   WdtWindowStartNone,
    ///   WdtReset,
    ///   WdtSleepStop
    /// >();
    /// ```
    ///
    /// See the [module](module@crate::osm::ofs0) docs for the available values.
    #[allow(private_bounds)]
    pub const fn new<
        IWDTSTRT: IwdtAutoStartMode,
        IWDTTOPS: IwdtTimeoutPeriod,
        IWDTCKS: IwdtClockRatio,
        IWDTRPES: IwdtWindowEndPosition,
        IWDTRPSS: IwdtWindowStartPosition,
        IWDTRSTIRQS: IwdtResetOrInterrupt,
        IWDTSTPCTL: IwdtStopControl,
        WDTSTRT: WdtAutoStartMode,
        WDTTOPS: WdtTimeoutPeriod,
        WDTCKS: WdtClockRatio,
        WDTRPES: WdtWindowEndPosition,
        WDTRPSS: WdtWindowStartPosition,
        WDTRSTIRQS: WdtResetOrInterrupt,
        WDTSTPCTL: WdtStopControl,
    >() -> Ofs0 {
        Ofs0(
            OFS0_H
                | OFS0_L
                | IWDTSTRT::OFS0
                | IWDTTOPS::OFS0
                | IWDTCKS::OFS0
                | IWDTRPES::OFS0
                | IWDTRPSS::OFS0
                | IWDTRSTIRQS::OFS0
                | IWDTSTPCTL::OFS0
                | WDTSTRT::OFS0
                | WDTTOPS::OFS0
                | WDTCKS::OFS0
                | WDTRPES::OFS0
                | WDTRPSS::OFS0
                | WDTRSTIRQS::OFS0
                | WDTSTPCTL::OFS0,
        )
    }

    /// Generates a value to configure `OFS0` with the defaults ArduinoCore uses.
    ///
    /// Typical usage:
    /// ```rust,ignore
    /// #[unsafe(no_mangle)]
    /// #[unsafe(link_section = ".ofs0")]
    /// static OFS0: Ofs0 = Ofs0::ArduinoCore();
    /// ```
    pub const fn arduino_core() -> Self {
        Self::new::<
            IwdtAutoStartOff,
            IwdtTimeout2048,
            IwdtClockRatio128,
            IwdtWindowEndNone,
            IwdtWindowStartNone,
            IwdtReset,
            IwdtSleepStop,
            WdtAutoStartOff,
            WdtTimeout16384,
            WdtClockRatio128,
            WdtWindowEndNone,
            WdtWindowStartNone,
            WdtReset,
            WdtSleepStop,
        >()
    }
}
