//! Driver for [`embassy-time`](https://docs.embassy.dev/embassy-time).
//!
//! Currently hardcoded to take up `GPT32_0`, `IEL0`, and `IEL1`.
//!
//! # TODO
//! * Allow use of a different `GPT` instance
//! * Add an `AGT` implementation (only useful with external oscillators)

use core::{
    cell::{Cell, RefCell},
    marker::PhantomData,
    sync::atomic::{AtomicU32, Ordering},
};

use critical_section::{CriticalSection, Mutex};
use embassy_hal_internal::interrupt::InterruptExt as _;
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;

use crate::{
    event_link::IcuInterrupt,
    interrupt,
    interrupt::typelevel::Interrupt,
    pac::{
        self,
        gpt::{
            regs::{Gtdnsr, Gtupsr},
            vals::{Mode, Tpcs, Ud},
        },
    },
    peripherals::GPT32_0,
    write_protect::ProtectedPeripheral as _,
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

trait Instance: crate::timer::Instance<u32> + Send + Sync + 'static {
    type AlarmInterrupt: interrupt::typelevel::Interrupt;
    type OverflowInterrupt: interrupt::typelevel::Interrupt;
}

impl Instance for crate::peripherals::GPT32_0 {
    type AlarmInterrupt = crate::interrupt::typelevel::IEL1;
    type OverflowInterrupt = crate::interrupt::typelevel::IEL0;
}

impl Instance for crate::peripherals::GPT32_1 {
    type AlarmInterrupt = crate::interrupt::typelevel::IEL1;
    type OverflowInterrupt = crate::interrupt::typelevel::IEL0;
}

struct GptDriver<I: Instance> {
    /// Number of 2^32 periods elapsed since boot.
    period: AtomicU32,
    queue: Mutex<RefCell<Queue>>,
    alarms: Mutex<AlarmState>,
    phantom: PhantomData<I>,
}

impl<I: Instance> GptDriver<I> {
    // Fudge factor to ensure we don't fire early
    const FUDGE_FACTOR: u64 = 10;

    pub(crate) fn init(&'static self) {
        I::start_module();

        // Safety: These are safe because we've hardcoded interrupt handlers.
        unsafe {
            // Enable both interrupts at the NVIC level
            I::AlarmInterrupt::IRQ.enable();
            I::OverflowInterrupt::IRQ.enable();

            // Arm the overflow interrupt in the ICU
            I::OverflowInterrupt::IRQ.icu_enable(I::OVERFLOW_EVENT);
        };

        let timer = I::regs();

        // Disable external things that might modify the counter
        timer.gtupsr().write_value(Gtupsr(0));
        timer.gtdnsr().write_value(Gtdnsr(0));

        timer.gtcr().write(|r| r.set_md(Mode::SawWavePwm));

        // Ensure count direction is UP
        timer.gtuddtyc().write(|r| {
            r.set_udf(true);
            r.set_ud(Ud::Up);
        });
        timer.gtuddtyc().write(|r| {
            r.set_udf(false);
            r.set_ud(Ud::Up);
        });

        // Since PCLKD is configured at a rate that is directly supported by embassy-time
        // as a tick rate, let's just use the peripheral clock undivided.
        timer.gtcr().write(|r| r.set_tpcs(Tpcs::DIV_1));
        trace!("GTCR: {}", timer.gtcr().read());

        // Overflow at u32::MAX
        timer.gtpr().write_value(u32::MAX);
        trace!("GTPR: {}", timer.gtpr().read());

        timer.gtcnt().write_value(0);
        trace!("GTCNT: {}", timer.gtcnt().read());

        // This is faster??
        timer.gtssr().write(|r| r.set_cstrt(true));
        timer.gtstr().write(|r| r.set_cstrt(0, true));
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
            }
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
        let timer = I::regs();

        let alarm = self.alarms.borrow(*cs);
        alarm.timestamp.set(timestamp);

        let t = self.now();
        if timestamp <= t {
            // Disarm the alarm and return `false` to indicate that.
            I::AlarmInterrupt::IRQ.icu_disable();
            alarm.timestamp.set(u64::MAX);

            return false;
        }

        let safe_timestamp = (timestamp.max(t + Self::FUDGE_FACTOR) & 0xFFFF_FFFF) as u32;

        let diff = timestamp - t;

        if diff < u64::from(u32::MAX) {
            timer.protected_write(|| {
                // Load the safe timestamp
                timer.gtccrc().write_value(safe_timestamp);

                // Safety: interrupt handlers are hardcoded and thus it's safe to enable the compare interrupt
                unsafe { I::AlarmInterrupt::IRQ.icu_enable(I::COMP_C_EVENT) }
            });
        } else {
            // TODO: Uhhhhh
            // If alarm must trigger some time after the current period, too far in the future,
            // don't setup the alarm enable, gpreg2, yet. It will be setup later by `next_period`.
        }

        true
    }
}

impl<I: Instance> Driver for GptDriver<I> {
    fn now(&self) -> u64 {
        let timer = I::regs();

        let period = self.period.load(Ordering::Acquire);
        let count = timer.gtcnt().read();
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

embassy_time_driver::time_driver_impl!(static DRIVER: GptDriver<GPT32_0> = GptDriver {
    period:AtomicU32::new(0),
    queue:Mutex::new(RefCell::new(Queue::new())),
    alarms:Mutex::new(AlarmState::new()),
    phantom: PhantomData
});

pub(crate) fn init() {
    DRIVER.init()
}

#[interrupt]
fn IEL0() {
    let icu = pac::ICU;

    icu.ielsr(0).modify(|r| r.set_ir(false));

    DRIVER.interrupted_overflow();
}

#[interrupt]
fn IEL1() {
    let icu = pac::ICU;

    icu.ielsr(1).modify(|r| r.set_ir(false));

    DRIVER.interrupted_alarm();
}
