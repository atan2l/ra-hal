//! Timer that generates interrupts utilizing the General PWM Timer (`GPT`).
//!
//! # Notes
//!
//! * 32-bit timers are supported but one instance is used for embassy-time, if support is enabled.
//! * `GPT` module stop will be disabled on instantiation, but not reenabled on drop.

use core::marker::PhantomData;

use embassy_hal_internal::{Peri, PeripheralType};

use crate::{
    event_link::InterruptEvent,
    module_stop::ModuleStop,
    pac::gpt::{
        regs::{Gtdnsr, Gtupsr},
        vals::{Ccr, Mode, Tpcs, Ud},
    },
};

/// An [`InterruptTimer`] instance.
#[allow(private_bounds)]
pub trait Instance<Width: TimerWidth>: SealedInstance + ModuleStop + PeripheralType {
    /// Event link event upon a match with capture/compare value A.
    const COMP_A_EVENT: crate::event_link::InterruptEvent;

    /// Event link event upon a match with capture/compare value B.
    const COMP_B_EVENT: crate::event_link::InterruptEvent;

    /// Event link event upon a match with compare value C.
    const COMP_C_EVENT: crate::event_link::InterruptEvent;

    /// Event link event for an overflow event.
    const OVERFLOW_EVENT: crate::event_link::InterruptEvent;

    /// Event link event for an underflow event.
    const UNDERFLOW_EVENT: crate::event_link::InterruptEvent;
}

pub(crate) trait SealedInstance: PeripheralType {
    const INDEX: usize;

    fn regs() -> crate::pac::gpt::Gpt;
}

trait TimerWidth: Into<u32> {
    fn max() -> u64;
}

impl TimerWidth for u32 {
    #[inline(always)]
    fn max() -> u64 {
        u32::MAX.into()
    }
}

impl TimerWidth for u16 {
    #[inline(always)]
    fn max() -> u64 {
        u16::MAX.into()
    }
}

/// A timer that fires an [`InterruptEvent`] at a fixed interval.
#[allow(private_bounds)]
pub struct InterruptTimer<'d, W: TimerWidth, I: Instance<W>> {
    phantom_i: PhantomData<&'d I>,
    phantom_w: PhantomData<W>,
    triangle: bool,
}

#[allow(private_bounds)]
impl<'d, W: TimerWidth, I: Instance<W>> InterruptTimer<'d, W, I> {
    /// Creates a new timer.
    pub fn new(peri: Peri<'d, I>) -> Self {
        let _ = peri;

        I::start_module();

        let gpt = I::regs();

        // Disable external things that might modify the counter
        gpt.gtupsr().write_value(Gtupsr(0));
        gpt.gtdnsr().write_value(Gtdnsr(0));
        gpt.gtcnt().write_value(0);
        gpt.gtssr().modify(|r| r.set_cstrt(true));

        Self {
            phantom_i: PhantomData,
            phantom_w: PhantomData,
            triangle: false,
        }
    }

    /// Sets the number of times per second a timer should fire.
    #[inline(always)]
    pub fn set_frequency(&mut self, frequency: u32) {
        let gpt = I::regs();

        // Set clock divider to 16
        gpt.gtcr().modify(|r| r.set_tpcs(Tpcs::DIV_16));

        let clocks = crate::clock_config();

        let mut period = (clocks.peripheral_d as u64 / 16) / (frequency as u64);

        if period > W::max() {
            period /= 2;
            self.triangle = true;
            gpt.gtcr().modify(|r| r.set_md(Mode::TrianglePwm1));
            gpt.gtber().modify(|w| w.set_ccra(Ccr::NoBuffer));
        } else {
            self.triangle = false;
            gpt.gtcr().modify(|r| r.set_md(Mode::SawWaveOneShot));
        }

        assert!(period <= W::max());

        self.set_period(period as u32);
    }

    #[inline(always)]
    fn set_period(&mut self, period: u32) {
        let gpt = I::regs();

        // Note: 16-bit instances still use 32-bit registers.
        gpt.gtpr().write_value(period);
    }

    /// Starts the timer and resets the counter and returns the associated [`InterruptEvent`].
    /// The event is either over- or underflow depending on the frequency of the timer.
    #[inline]
    pub fn start(&mut self) -> InterruptEvent {
        let gpt = I::regs();

        gpt.gtuddtyc().write(|r| {
            r.set_udf(true);
            r.set_ud(Ud::Up);
        });
        gpt.gtuddtyc().write(|r| {
            r.set_udf(false);
            r.set_ud(Ud::Up);
        });

        gpt.gtcnt().write_value(0);

        // Start the timer
        gpt.gtstr().modify(|r| r.set_cstrt(I::INDEX, true));

        if self.triangle {
            I::UNDERFLOW_EVENT
        } else {
            I::OVERFLOW_EVENT
        }
    }

    /// Stops the timer.
    #[inline(always)]
    pub fn stop(&mut self) {
        let gpt = I::regs();

        // Stop the timer
        gpt.gtstr().modify(|r| r.set_cstrt(I::INDEX, false));
    }
}

impl<'d, W: TimerWidth, I: Instance<W>> Drop for InterruptTimer<'d, W, I> {
    fn drop(&mut self) {
        error!(
            "GPT{}: Drop not yet implemented, module will not be stopped",
            I::INDEX
        );
    }
}

macro_rules! timer_instance {
    ($peripheral:ident, $width:ident, $index:literal, $cmpa_int:ident, $cmpb_int:ident, $cmpc_int:ident, $overflow_int:ident, $underflow_int:ident) => {
        impl crate::timer::Instance<$width> for crate::peripherals::$peripheral {
            const COMP_A_EVENT: crate::event_link::InterruptEvent =
                crate::event_link::InterruptEvent::$cmpa_int;
            const COMP_B_EVENT: crate::event_link::InterruptEvent =
                crate::event_link::InterruptEvent::$cmpb_int;
            const COMP_C_EVENT: crate::event_link::InterruptEvent =
                crate::event_link::InterruptEvent::$cmpc_int;
            const OVERFLOW_EVENT: crate::event_link::InterruptEvent =
                crate::event_link::InterruptEvent::$overflow_int;
            const UNDERFLOW_EVENT: crate::event_link::InterruptEvent =
                crate::event_link::InterruptEvent::$underflow_int;
        }

        impl crate::timer::SealedInstance for crate::peripherals::$peripheral {
            const INDEX: usize = $index;

            #[inline(always)]
            fn regs() -> crate::pac::gpt::Gpt {
                crate::pac::$peripheral
            }
        }
    };
}
pub(crate) use timer_instance;
