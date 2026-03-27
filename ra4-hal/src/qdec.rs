//! Quadrature decoder driver (`GPT`) utilizing the General PWM Timer (`GPT`).
//!
//! The `GPT` peripheral provides support for what Renesas calls "phase counting" a.k.a. quadrature decoding.
//! This driver configures a `GPT` instance for "phase counting mode 1" (Table 22.7).
//! In mode 1, the following events are recognized as up-counting events:
//! - Rising edge on pin A while pin B is low
//! - Falling edge on pin A while pin B is high
//! - Rising edge on pin B while pin A is high
//! - Falling edge on pin B while pin A is low
//!
//! In mode 1, the following events are recognized as down-counting events:
//! - Rising edge on pin A while pin B is high
//! - Falling edge on pin A while pin B is low
//! - Rising edge on pin B while pin A is low
//! - Falling edge on pin B while pin A is high
//!
//! # TODO
//! - Support other phase counting modes
//! - Handle over/underflow (or verify correct operation)

use core::{future::poll_fn, marker::PhantomData, task::Poll};

use embassy_hal_internal::{Peri, interrupt::InterruptExt as _};

use crate::{
    event_link::IcuInterrupt as _,
    gpio::{Flex, WithOpenDrain},
    interrupt,
    interrupt::typelevel::{Handler as InterruptHandler, Interrupt as InterruptType},
    pac::gpt::{
        regs::{Gtdnsr, Gtupsr},
        vals::{Ccr, Nfcs},
    },
    pwm::{ChanA, ChanB, Divider, Instance, PwmPin},
};

/// Quadrature decoder configuration
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// Clock divider for GPT instance (`PCLKD/n`).
    pub divider: Divider,

    /// Debounce filter configuration.
    pub debounce: Debounce,
}

/// Noise (debounce) filter for channels A and B.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq)]
pub enum Debounce {
    /// Noise filter is disabled.
    Off,

    /// Event must last for at least 1 `PCLKD` cycle.
    Min1,

    /// Event must last for at least 4 `PCLKD` cycles.
    Min4,

    /// Event must last for at least 16 `PCLKD` cycles.
    Min16,

    /// Event must last for at least 64 `PCLKD` cycles.
    Min64,
}

/// Direction of rotation
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    #[allow(missing_docs)]
    Up,

    #[allow(missing_docs)]
    Down,

    #[allow(missing_docs)]
    Unknown,
}

/// Quadrature decoder driver for the `GPT` timer.
#[allow(private_bounds)]
pub struct Qdec<'d, I: Instance, Int: InterruptType> {
    _instance: PhantomData<&'d I>,
    _interrupt: PhantomData<Int>,
    // These are set to WithOpenDrain because on the RA4M1 all PWM pins have both capabilities
    _pin_a: Flex<'d, WithOpenDrain>,
    _pin_b: Flex<'d, WithOpenDrain>,
}

/// Interrupt handler for rotation event.
#[allow(private_bounds)]
pub struct QdecInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

impl From<Debounce> for Nfcs {
    fn from(value: Debounce) -> Self {
        match value {
            // This is not actually off, to do that we need to poke a different register
            Debounce::Off => Self::Pclkd1,
            Debounce::Min1 => Self::Pclkd1,
            Debounce::Min4 => Self::Pclkd4,
            Debounce::Min16 => Self::Pclkd16,
            Debounce::Min64 => Self::Pclkd64,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            divider: Divider::Div1024,
            debounce: Debounce::Min64,
        }
    }
}

impl<I: Instance, Int: InterruptType> InterruptHandler<Int> for QdecInterruptHandler<I> {
    unsafe fn on_interrupt() {
        Int::IRQ.icu_unpend();
        I::cmpa_waker().wake()
    }
}

#[allow(private_bounds)]
impl<'d, I: Instance, Int: InterruptType> Qdec<'d, I, Int> {
    /// Consumes a `GPT` peripheral instance and returns a quadrature decoder driver.
    pub fn new<A: PwmPin<I, ChanA>, B: PwmPin<I, ChanB>>(
        peri: Peri<'d, I>,
        pin_a: Peri<'d, A>,
        pin_b: Peri<'d, B>,
        config: Config,
        irq: impl interrupt::typelevel::Binding<Int, QdecInterruptHandler<I>>,
    ) -> Self {
        let _ = peri;
        let _ = irq;

        let pwm = I::regs();

        pwm.gticasr().modify(|r| {
            r.set_ascarbl(true);
            r.set_ascarbh(false);
            r.set_ascafbl(true);
            r.set_ascafbh(false);
        });
        pwm.gtcr().modify(|w| w.set_tpcs(config.divider.into()));
        pwm.gtpr().write_value(0xFFFF as u32);

        // These were taken from the manual § 22.3.10.
        pwm.gtupsr().write_value(Gtupsr(0x6900));
        pwm.gtdnsr().write_value(Gtdnsr(0x9600));

        let debounce_enable = config.debounce != Debounce::Off;

        // Debouncing. Maybe.
        pwm.gtior().modify(|r| {
            r.set_nfaen(debounce_enable);
            r.set_nfcsa(config.debounce.into());
            r.set_nfben(debounce_enable);
            r.set_nfcsb(config.debounce.into());
        });

        pwm.gtcnt().write_value(0);
        pwm.gtccra().write_value(0);
        pwm.gtccrc().write_value(0);
        pwm.gtber().modify(|w| w.set_ccra(Ccr::SingleBuffer));

        pin_a.set_pfunc();
        let pin_a = Flex::new(pin_a);

        pin_b.set_pfunc();
        let pin_b = Flex::new(pin_b);

        unsafe {
            Int::IRQ.enable();
            Int::IRQ.icu_enable(I::CAPTURE_COMP_A_EVENT)
        };

        pwm.gtcr().modify(|w| w.set_cst(true));
        pwm.gtst().modify(|r| r.set_tcfa(false));

        Self {
            _instance: PhantomData,
            _interrupt: PhantomData,
            _pin_a: pin_a,
            _pin_b: pin_b,
        }
    }

    /// Returns the current count.
    pub fn count(&self) -> u32 {
        let pwm = I::regs();
        // Let's consider not all instances are 32-bits wide
        pwm.gtcnt().read()
    }

    /// Waits for a rotation event and returns the direction of rotation.
    pub async fn read(&self) -> Direction {
        let pwm = I::regs();

        poll_fn(|ctx| {
            I::cmpa_waker().register(ctx.waker());

            if pwm.gtst().read().tcfa() {
                // Clear the event
                pwm.gtst().modify(|r| r.set_tcfa(false));

                let count = pwm.gtcnt().read();
                let prev_count = pwm.gtccra().read();

                if count > prev_count {
                    Poll::Ready(Direction::Up)
                } else if count < prev_count {
                    Poll::Ready(Direction::Down)
                } else {
                    Poll::Ready(Direction::Unknown)
                }
            } else {
                Poll::Pending
            }
        })
        .await
    }

    /// Resets the counter and buffers to zero.
    pub fn reset(&mut self) {
        let pwm = I::regs();

        critical_section::with(|_| {
            pwm.gtcnt().write_value(0);
            pwm.gtccra().write_value(0);
            pwm.gtccrc().write_value(0);
        });
    }
}

impl<'d, I: Instance, Int: InterruptType> Drop for Qdec<'d, I, Int> {
    /// Note: this will unlink the interrupt and stop the timer but does not turn off the GPT clock.
    fn drop(&mut self) {
        Int::IRQ.disable();
        Int::IRQ.icu_disable();

        let pwm = I::regs();
        pwm.gtcr().modify(|w| w.set_cst(false));
    }
}
