//! Driver for `embassy-time`.
//!
//! Currently hardcoded to take up `GPT32_0`, `IEL0`, and `IEL1`.

use core::cell::Cell;
use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::{interrupt, interrupt::typelevel::Interrupt, pac};
use critical_section::{CriticalSection, Mutex};
use embassy_hal_internal::interrupt::InterruptExt;
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;
use ra4m1_ctpac::gpt32::{
    regs::{Gtdnsr, Gtupsr},
    vals::{Mode, Tpcs, Ud},
};

use crate::{
    event_link::{IcuEventer, InterruptEvent},
    peripherals::GPT32_0,
    write_protect::WriteProtect as _,
};

struct AlarmState {
    timestamp: Cell<u64>,
}

unsafe impl Send for AlarmState {}

impl AlarmState {
    const fn new() -> Self {
        Self {
            timestamp: Cell::new(u64::MAX),
        }
    }
}

trait Instance {
    type AlarmInterrupt: interrupt::typelevel::Interrupt;
    type OverflowInterrupt: interrupt::typelevel::Interrupt;
    const ALARM_EVENT: InterruptEvent;
    const OVERFLOW_EVENT: InterruptEvent;

    fn regs() -> pac::gpt32::Gpt32;
}

impl Instance for crate::peripherals::GPT32_0 {
    type AlarmInterrupt = crate::interrupt::typelevel::IEL1;
    type OverflowInterrupt = crate::interrupt::typelevel::IEL0;
    const ALARM_EVENT: InterruptEvent = InterruptEvent::Gpt0CmpC;
    const OVERFLOW_EVENT: InterruptEvent = InterruptEvent::Gpt0Ovf;

    #[inline(always)]
    fn regs() -> crate::pac::gpt32::Gpt32 {
        crate::pac::GPT32_0
    }
}

struct GptDriver {
    /// Number of 2^32 periods elapsed since boot.
    period: AtomicU32,
    queue: Mutex<RefCell<Queue>>,
    alarms: Mutex<AlarmState>,
}

impl GptDriver {
    // Fudge factor to ensure we don't fire early
    const FUDGE_FACTOR: u64 = 10;

    pub(crate) fn init(
        &'static self,
        // _timer: Peri<'static, crate::peripherals::GPT32_0>,
        _irq_prio: crate::interrupt::Priority,
    ) {
        debug!("GPT32_0: stop=false");

        let mstp = pac::MSTP;

        mstp.mstpcrd().write(|w| {
            w.set_mstpd5(false);
        });

        // Enable the interrupts at the NVIC level,
        // arm the overflow interrupt in the ICU.
        {
            type AlarmInt = <GPT32_0 as Instance>::AlarmInterrupt;
            type OverflowInt = <GPT32_0 as Instance>::OverflowInterrupt;

            unsafe {
                AlarmInt::IRQ.enable();
                OverflowInt::IRQ.enable();
            };

            OverflowInt::icu_enable(<GPT32_0 as Instance>::OVERFLOW_EVENT);
        }

        let timer = GPT32_0::regs();

        // Disable external things that might modify the counter
        timer.gtupsr().write_value(Gtupsr(0));
        timer.gtdnsr().write_value(Gtdnsr(0));

        timer.gtcr().write(|w| {
            w.set_md(Mode::SawWavePwm);
        });

        // Ensure count direction is UP
        timer.gtuddtyc().write(|w| {
            w.set_udf(true);
            w.set_ud(Ud::Up);
        });
        timer.gtuddtyc().write(|w| {
            w.set_udf(false);
            w.set_ud(Ud::Up);
        });

        // Since we're at 48 MHz just use the clock, undivided
        timer.gtcr().write(|w| {
            w.set_tpcs(Tpcs::_000);
        });
        trace!("GTCR: {}", timer.gtcr().read());

        // Overflow at u32::MAX
        timer.gtpr().write(|w| {
            w.set_gtpr(u32::MAX);
        });
        trace!("GTPR: {}", timer.gtpr().read());

        timer.gtcnt().write(|w| {
            w.set_gtcnt(0);
        });
        trace!("GTCNT: {}", timer.gtcnt().read());

        // This is faster??
        timer.gtssr().write(|w| {
            w.set_cstrt(true);
        });
        timer.gtstr().write(|w| {
            w.set_cstrt(0, true);
        });
    }

    fn interrupted_alarm(&'static self) {
        critical_section::with(|cs| {
            let mut next = self
                .queue
                .borrow(cs)
                .borrow_mut()
                .next_expiration(self.now());

            while !self.set_alarm(&cs, next) {
                next = self
                    .queue
                    .borrow(cs)
                    .borrow_mut()
                    .next_expiration(self.now());
            } //
        });
    }

    fn interrupted_overflow(&'static self) {
        critical_section::with(|_cs| {
            let _period = self
                .period
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |p| Some(p + 1))
                .unwrap_or_else(|p| {
                    error!("Unable to increment period. Time is now inaccurate");

                    p
                });
        });
    }

    #[must_use]
    fn set_alarm(&self, cs: &CriticalSection, timestamp: u64) -> bool {
        type AlarmInt = <GPT32_0 as Instance>::AlarmInterrupt;

        let timer = GPT32_0::regs();
        let icu = pac::ICU;

        let alarm = self.alarms.borrow(*cs);
        alarm.timestamp.set(timestamp);

        let t = self.now();
        if timestamp <= t {
            // Disarm the alarm and return `false` to indicate that.
            AlarmInt::icu_disable();
            alarm.timestamp.set(u64::MAX);

            return false;
        }

        let safe_timestamp = (timestamp.max(t + Self::FUDGE_FACTOR) & 0xFFFF_FFFF) as u32;

        let diff = timestamp - t;

        if diff < u64::from(u32::MAX) {
            timer.protected_write(|| {
                // Load the safe timestamp
                timer.gtccrc().write(|w| {
                    w.set_gtccrc(safe_timestamp);
                });
                // Enable the compare interrupt
                AlarmInt::icu_enable(<GPT32_0 as Instance>::ALARM_EVENT);
            });
        } else {
            // TODO: UHhhhh
            // If alarm must trigger some time after the current period, too far in the future,
            // don't setup the alarm enable, gpreg2, yet. It will be setup later by `next_period`.
        }

        true
    }
}

impl Driver for GptDriver {
    fn now(&self) -> u64 {
        let timer = GPT32_0::regs();

        let period = self.period.load(Ordering::Acquire);
        let count = timer.gtcnt().read().gtcnt();
        ((period as u64) << 32) + (count as u64)
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();
            if queue.schedule_wake(at, waker) {
                let mut next = queue.next_expiration(self.now());
                while !self.set_alarm(&cs, next) {
                    next = queue.next_expiration(self.now());
                }
            }
        });
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: GptDriver = GptDriver{
    period: AtomicU32::new(0),
    queue: Mutex::new(RefCell::new(Queue::new())),
    alarms: Mutex::new(AlarmState::new())
});

pub(crate) fn init(irq_prio: crate::interrupt::Priority) {
    DRIVER.init(irq_prio)
}

#[interrupt]
fn IEL0() {
    let icu = pac::ICU;

    icu.ielsr(0).modify(|w| {
        w.set_ir(false);
    });

    DRIVER.interrupted_overflow();
}

#[interrupt]
fn IEL1() {
    let icu = pac::ICU;

    icu.ielsr(1).modify(|w| {
        w.set_ir(false);
    });

    DRIVER.interrupted_alarm();
}
