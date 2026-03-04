//! Timer that generates interrupts utilizing the General PWM Timer (`GPT`).
//!
//! # Notes
//!
//! * Only 16-bit `GPT` instances are currently supported
//! * `GPT` module stop will be disabled on instantiation, but not reenabled on drop.

use core::marker::PhantomData;

use embassy_hal_internal::{Peri, PeripheralType};

use crate::{
    event_link::InterruptEvent,
    pac::gpt::{
        regs::{Gtdnsr, Gtupsr},
        vals::{Ccr, Mode, Tpcs, Ud},
    },
    peripherals,
};

/// An [`InterruptTimer`] instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance {}

trait SealedInstance: PeripheralType {
    const INDEX: usize;

    fn regs() -> crate::pac::gpt::Gpt;
    // fn module_stop();
    fn module_start();

    fn overflow_interrupt() -> crate::event_link::InterruptEvent;
    fn underflow_interrupt() -> crate::event_link::InterruptEvent;
}

/// A timer that fires an [`InterruptEvent`] at a fixed interval.
pub struct InterruptTimer<'d, I: Instance> {
    phantom: PhantomData<&'d I>,
    triangle: bool,
}

impl<'d, I: Instance> InterruptTimer<'d, I> {
    /// Creates a new timer.
    pub fn new(peri: Peri<'d, I>) -> Self {
        let _ = peri;

        I::module_start();

        let gpt = I::regs();

        // Disable external things that might modify the counter
        gpt.gtupsr().write_value(Gtupsr(0));
        gpt.gtdnsr().write_value(Gtdnsr(0));
        gpt.gtcnt().write_value(0);
        gpt.gtssr().modify(|r| r.set_cstrt(true));

        Self {
            phantom: PhantomData,
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

        let mut period = (clocks.peripheral_d as u32 / 16) / frequency;

        if period > u32::from(u16::MAX) {
            period /= 2;
            self.triangle = true;
            gpt.gtcr().modify(|r| r.set_md(Mode::TrianglePwm1));
            gpt.gtber().modify(|w| w.set_ccra(Ccr::NoBuffer));
        } else {
            self.triangle = false;
            gpt.gtcr().modify(|r| r.set_md(Mode::SawWaveOneShot));
        }

        // Because we're using a 16-bit timer.
        assert!(period <= u32::from(u16::MAX));

        self.set_period(period as u16);
    }

    #[inline(always)]
    fn set_period(&mut self, period: u16) {
        let gpt = I::regs();
        gpt.gtpr().write_value(period as u32);
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
            I::underflow_interrupt()
        } else {
            I::overflow_interrupt()
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

impl<'d, I: Instance> Drop for InterruptTimer<'d, I> {
    fn drop(&mut self) {
        error!(
            "GPT{}: Drop not yet implemented, module will not be stopped",
            I::INDEX
        );
    }
}

macro_rules! instance_impl {
    ($size:ident, $instance:literal, $mstp:ident, $cmpa_int:ident, $cmpb_int:ident, $overflow_int:ident, $underflow_int:ident) => {
        paste::paste! {
            impl Instance for peripherals::[< $size _ $instance >] {}
            impl SealedInstance for peripherals::[< $size _ $instance >] {
                const INDEX : usize = $instance;

                #[inline(always)]
                fn regs() -> crate::pac::gpt::Gpt {
                    crate::pac::[< $size _ $instance >]
                }

                #[inline(always)]
                fn module_start() {
                    debug!("{}: stop=false", stringify!([< $size _ $instance >]));
                    let mstp = crate::pac::MSTP;
                    mstp.mstpcrd().modify(|r| r.[< set_ $mstp >](false));
                }

                #[inline(always)]
                fn overflow_interrupt() -> crate::event_link::InterruptEvent {
                    crate::event_link::InterruptEvent::$overflow_int
                }

                #[inline(always)]
                fn underflow_interrupt() -> crate::event_link::InterruptEvent {
                    crate::event_link::InterruptEvent::$underflow_int
                }
            }
        }
    };
}

instance_impl!(GPT16, 2, mstpd6, Gpt2CcmpA, Gpt2CcmpB, Gpt2Ovf, Gpt2Udf);
instance_impl!(GPT16, 3, mstpd6, Gpt3CcmpA, Gpt3CcmpB, Gpt3Ovf, Gpt3Udf);
instance_impl!(GPT16, 4, mstpd6, Gpt4CcmpA, Gpt4CcmpB, Gpt4Ovf, Gpt4Udf);
instance_impl!(GPT16, 5, mstpd6, Gpt5CcmpA, Gpt5CcmpB, Gpt5Ovf, Gpt5Udf);
instance_impl!(GPT16, 6, mstpd6, Gpt6CcmpA, Gpt6CcmpB, Gpt6Ovf, Gpt6Udf);
instance_impl!(GPT16, 7, mstpd6, Gpt7CcmpA, Gpt7CcmpB, Gpt7Ovf, Gpt7Udf);
