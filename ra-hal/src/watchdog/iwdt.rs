//! Independent Watchdog Timer (`IWDT`) §26.
//!
//! This watchdog operates off of its own 15 kHz clock independently of the peripheral clock that drives `WDT`.

use core::marker::PhantomData;

use cortex_m::asm;
use embassy_hal_internal::{Peri, PeripheralType, interrupt::InterruptExt as _};

use crate::{
    event_link::{IcuInterrupt as _, InterruptEvent},
    interrupt,
    interrupt::typelevel::{Handler as InterruptHandler, Interrupt as InterruptType},
    pac::{
        self,
        dbg::vals::DbgstopIwdt,
        osm::vals::{IwdtCks, IwdtTops, StartMode},
    },
    peripherals,
    watchdog::Action,
};

/// Interrupt handler for the watchdog timer, used with [`Watchdog::new_handler`].
pub struct IwdtInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

/// Watchdog driver.
pub struct Watchdog<'d, I: Instance> {
    phantom: PhantomData<&'d I>,
}

/// Watchdog instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {}

pub(crate) trait SealedInstance {
    fn regs() -> pac::iwdt::Iwdt;
}

impl<'d, I: Instance> Watchdog<'d, I> {
    /// Creates a new `IWDT` driver.
    ///
    /// # Notes
    /// `IWDT` can only be configured via [`OFS0`](module@crate::osm::ofs0) and cannot be configured or enabled at runtime.
    pub fn new(peri: Peri<'d, I>, action: Action) -> Self {
        let _ = peri;

        let osm = pac::OSM;
        let ofs0 = osm.ofs0().read();

        if ofs0.iwdtstrt() != StartMode::AutoStart {
            warn!("IWDT: Not enabled");
        } else if ofs0.iwdtstpctl() {
            warn!("IWDT: Suspended in sleep mode, may not behave as expected with e.g. Embassy")
        }

        match action {
            Action::Interrupt => {
                debug!("IWDT: Enabling NMI");

                cfg_select! {
                    not(ra8m1) => {
                        let icu = pac::ICU;

                        icu.nmicr().write(|r| r.set_nflten(false));
                        icu.nmicr().write(|r| r.set_nmimd(true));
                        icu.nmiclr().write(|r| r.set_nmiclr(true));
                    },
                    ra8m1 => {
                        let icu = pac::ICU;
                        let icu_common = pac::ICU_COMMON;

                        icu_common.nmicr().write(|r| r.set_nflten(false));
                        icu_common.nmicr().write(|r| r.set_nmimd(true));
                        icu.nmiclr().write(|r| r.set_nmiclr(true));
                    },
                }

                icu.nmier().write(|r| r.set_iwdten(true));
            }
            Action::Reset => {}
            Action::Handler => {}
        }

        // The timer will never run in debug mode while execution is stopped.
        // This just enables the timer when running in debug mode.
        let dbg = pac::DBG;
        dbg.dbgstopcr()
            .write(|r| r.set_dbgstop_iwdt(DbgstopIwdt::Enable));

        {
            let osm = pac::OSM;
            let ofs0 = osm.ofs0().read();
            let divider: u16 = match ofs0.iwdtcks() {
                IwdtCks::_1 => 1,
                IwdtCks::_16 => 16,
                IwdtCks::_32 => 32,
                IwdtCks::_64 => 64,
                IwdtCks::_128 => 128,
                IwdtCks::_256 => 256,
                _ => unreachable!(),
            };
            let period: u16 = match ofs0.iwdttops() {
                IwdtTops::_128 => 128,
                IwdtTops::_512 => 512,
                IwdtTops::_1024 => 1024,
                IwdtTops::_2048 => 2048,
            };

            let timeout = f32::from(period) / (15_000.0 / f32::from(divider));
            debug!("IWDT: timeout: {} sec", timeout);
        }

        Self {
            phantom: PhantomData,
        }
    }

    /// Configures `IWDT` to generate an interrupt, attaches a handler, and returns a new driver.
    #[inline]
    pub fn new_handler<IwdtInt: InterruptType>(
        peri: Peri<'d, I>,
        irq: impl interrupt::typelevel::Binding<IwdtInt, IwdtInterruptHandler<I>>,
    ) -> Self {
        let _ = peri;
        let _ = irq;

        // Safety: The interrupt handler is defined by the irq argument and thus the interrupt is safe to enable.
        unsafe {
            IwdtInt::IRQ.enable();
            IwdtInt::IRQ.icu_enable(InterruptEvent::IwdtUnderflow);
        }

        Self::new(peri, Action::Handler)
    }

    /// Refreshes the `IWDT` counter to prevent a timeout.
    pub fn refresh(&mut self) {
        let iwdt = I::regs();

        asm::dsb();
        iwdt.iwdtrr().write_value(0x00);
        asm::dsb();
        iwdt.iwdtrr().write_value(0xFF);
    }

    /// Returns the current value of the `IWDT` counter.
    #[inline(always)]
    pub fn value(&self) -> u16 {
        let iwdt = I::regs();

        iwdt.iwdtsr().read().cntval()
    }
}

impl Instance for peripherals::IWDT {}

impl SealedInstance for peripherals::IWDT {
    #[inline(always)]
    fn regs() -> pac::iwdt::Iwdt {
        pac::IWDT
    }
}

impl<I: Instance, IwdtInt: InterruptType> InterruptHandler<IwdtInt> for IwdtInterruptHandler<I> {
    unsafe fn on_interrupt() {
        IwdtInt::IRQ.icu_unpend();

        panic!("Independent watchdog underflow");
    }
}
