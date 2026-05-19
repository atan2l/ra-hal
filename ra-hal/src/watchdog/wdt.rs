//! Watchdog Timer (`WDT`) §25.

use core::marker::PhantomData;

use cortex_m::asm;
use embassy_hal_internal::{Peri, PeripheralType, interrupt::InterruptExt as _};

use crate::{
    event_link::{IcuInterrupt as _, InterruptEvent},
    interrupt::{
        self,
        typelevel::{Handler as InterruptHandler, Interrupt as InterruptType},
    },
    pac::{
        self,
        dbg::vals::DbgstopWdt,
        osm::vals::StartMode,
        wdt::vals::{Cks, Rpes, Rpss, Tops},
    },
    watchdog::Action,
};

/// `PCLKB` divider used to decrement the watchdog timer.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone)]
pub enum ClockDivider {
    /// `PCLKB/4`
    Div4,

    /// `PCLKB/64`
    Div64,

    /// `PCLKB/128`
    Div128,

    /// `PCLKB/512`
    Div512,

    /// `PCLKB/2048`
    Div2048,

    /// `PCLKB/8192`
    Div8192,
}

/// Watchdog configuration.
///
/// # TODO
/// Handle the reset window settings.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    #[allow(missing_docs)]
    pub action: Action,
    #[allow(missing_docs)]
    pub divider: ClockDivider,
    #[allow(missing_docs)]
    pub period: TimeoutPeriod,
}

/// Reset / refresh value for watchdog timer.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone)]
pub enum TimeoutPeriod {
    /// `CNTVAL` = 1024
    _1024,

    /// `CNTVAL` = 4096
    _4096,

    /// `CNTVAL` = 8192
    _8192,

    /// `CNTVAL` = 16,384
    _16384,
}

/// Watchdog driver.
pub struct Watchdog<'d, I: Instance> {
    phantom: PhantomData<&'d I>,
}

/// Interrupt handler for the watchdog timer, used with [`Watchdog::new_handler`].
pub struct WdtInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

/// Watchdog instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {}

pub(crate) trait SealedInstance {
    fn regs() -> pac::wdt::Wdt;
}

impl From<ClockDivider> for u16 {
    #[inline]
    fn from(value: ClockDivider) -> Self {
        match value {
            ClockDivider::Div4 => 4,
            ClockDivider::Div64 => 64,
            ClockDivider::Div128 => 128,
            ClockDivider::Div512 => 512,
            ClockDivider::Div2048 => 2048,
            ClockDivider::Div8192 => 8192,
        }
    }
}

impl From<ClockDivider> for Cks {
    #[inline]
    fn from(value: ClockDivider) -> Self {
        match value {
            ClockDivider::Div4 => Cks::Pclkb4,
            ClockDivider::Div64 => Cks::Pclkb64,
            ClockDivider::Div128 => Cks::Pclkb128,
            ClockDivider::Div512 => Cks::Pclkb512,
            ClockDivider::Div2048 => Cks::Pclkb2048,
            ClockDivider::Div8192 => Cks::Pclkb8192,
        }
    }
}

impl From<TimeoutPeriod> for Tops {
    #[inline]
    fn from(value: TimeoutPeriod) -> Self {
        match value {
            TimeoutPeriod::_1024 => Self::_1024Cycles,
            TimeoutPeriod::_4096 => Self::_4096Cycles,
            TimeoutPeriod::_8192 => Self::_8192Cycles,
            TimeoutPeriod::_16384 => Self::_16384Cycles,
        }
    }
}

impl From<TimeoutPeriod> for u16 {
    #[inline]
    fn from(value: TimeoutPeriod) -> Self {
        match value {
            TimeoutPeriod::_1024 => 1024,
            TimeoutPeriod::_4096 => 4096,
            TimeoutPeriod::_8192 => 8192,
            TimeoutPeriod::_16384 => 16384,
        }
    }
}

impl Default for Config {
    /// ≈2.8 s, ƒPCLKB = 24 MHz
    fn default() -> Self {
        Self {
            divider: ClockDivider::Div8192,
            period: TimeoutPeriod::_8192,
            action: Action::Interrupt,
        }
    }
}

impl<'d, I: Instance> Watchdog<'d, I> {
    fn new_inner(peri: Peri<'d, I>, config: Config) -> Self {
        let _ = peri;

        {
            let osm = pac::OSM;
            let ofs0 = osm.ofs0().read();

            if ofs0.wdtstrt() == StartMode::AutoStart {
                debug!("WDT: Auto-start mode selected, timing changes will be ignored.");
            }
        }

        let mut reset = false;
        match config.action {
            Action::Interrupt => {
                debug!("WDT: Enabling NMI");

                cfg_select! {
                    not(ra8m1) => {
                        let icu = pac::ICU;

                        icu.nmicr().write(|r| r.set_nflten(false));
                        icu.nmicr().write(|r| r.set_nmimd(true));
                        icu.nmiclr().write(|r| r.set_nmiclr(true));
                        icu.nmier().write(|r| r.set_wdten(true));
                    },
                    ra8m1 => {
                        let icu = pac::ICU;
                        let icu_common = pac::ICU_COMMON;

                        icu_common.nmicr().write(|r| r.set_nflten(false));
                        icu_common.nmicr().write(|r| r.set_nmimd(true));
                        icu.nmiclr().write(|r| r.set_nmiclr(true));
                        icu.nmier().write(|r| r.set_wdten(true));
                    },
                }
            }
            Action::Reset => {
                reset = true;
            }
            Action::Handler => {}
        }

        // The timer will never run in debug mode while execution is stopped.
        // This just enables the timer when running in debug mode.
        let dbg = pac::DBG;
        dbg.dbgstopcr()
            .write(|r| r.set_dbgstop_wdt(DbgstopWdt::Enable));

        let wdt = I::regs();

        wdt.wdtcr().write(|r| {
            r.set_tops(config.period.into());
            r.set_cks(config.divider.into());
            r.set_rpes(Rpes::None);
            r.set_rpss(Rpss::None);
        });

        {
            let config = wdt.wdtcr().read();
            let clocks = crate::clock::clock_status();
            let divider = match config.cks() {
                Cks::Pclkb4 => 4.0,
                Cks::Pclkb64 => 64.0,
                Cks::Pclkb128 => 128.0,
                Cks::Pclkb512 => 512.0,
                Cks::Pclkb2048 => 2048.0,
                Cks::Pclkb8192 => 8192.0,
                _ => unreachable!(),
            };
            let period = match config.tops() {
                Tops::_1024Cycles => 1024.0,
                Tops::_4096Cycles => 4096.0,
                Tops::_8192Cycles => 8192.0,
                Tops::_16384Cycles => 16384.0,
            };

            let timeout = period / ((clocks.peripheral_b.to_Hz() as f32) / divider);
            debug!("WDT: timeout: {} sec", timeout);
        }

        wdt.wdtrcr().write(|r| r.set_rstirqs(reset));
        wdt.wdtcstpr().write(|r| r.set_slcstp(false));

        let mut this = Self {
            phantom: PhantomData,
        };

        this.refresh();

        this
    }

    /// Configures and enables the watchdog timer.
    ///
    /// # Returns
    /// - A new driver instance.
    /// # Notes
    /// - If configured to generate an NMI, by default `cortex-m-rt` provides a handler that silently halts execution.
    /// - If `OFS0` is configured to auto-start `WDT` runtime changes to configuration will be silently ignored.
    #[inline]
    pub fn new(peri: Peri<'d, I>, config: Config) -> Self {
        Self::new_inner(peri, config)
    }

    /// Configures the watchdog to trigger our handler. Utility of this TBD.
    ///
    /// - Use for performance counting metrics?
    /// - Allow user defined watchdog handlers?
    #[inline]
    pub fn new_handler<WdtInt: InterruptType>(
        peri: Peri<'d, I>,
        config: Config,
        irq: impl interrupt::typelevel::Binding<WdtInt, WdtInterruptHandler<I>>,
    ) -> Self {
        let _ = peri;
        let _ = irq;

        assert_eq!(config.action, Action::Handler);

        // Safety: The interrupt handler is defined by the irq argument and thus the interrupt is safe to enable.
        unsafe {
            WdtInt::IRQ.enable();
            WdtInt::IRQ.icu_enable(InterruptEvent::WdtUnderflow);
        }

        Self::new_inner(peri, config)
    }

    /// Refreshes the `WDT` counter to prevent a timeout.
    #[inline]
    pub fn refresh(&mut self) {
        let wdt = I::regs();

        asm::dsb();
        wdt.wdtrr().write_value(0x00);
        asm::dsb();
        wdt.wdtrr().write_value(0xFF);
    }

    /// Returns the current value of the `WDT` counter.
    #[inline]
    pub fn value(&self) -> u16 {
        let wdt = I::regs();
        let status = wdt.wdtsr().read();

        status.cntval()
    }
}

impl Instance for crate::peripherals::WDT {}

impl SealedInstance for crate::peripherals::WDT {
    #[inline(always)]
    fn regs() -> pac::wdt::Wdt {
        pac::WDT
    }
}

impl<I: Instance, WdtInt: InterruptType> InterruptHandler<WdtInt> for WdtInterruptHandler<I> {
    unsafe fn on_interrupt() {
        WdtInt::IRQ.icu_unpend();

        panic!("Watchdog underflow");
    }
}
