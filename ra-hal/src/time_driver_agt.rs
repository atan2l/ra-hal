//! Driver for [`embassy-time`](https://docs.embassy.dev/embassy-time).
//!
//! Currently hardcoded to take up `AGT1` or `AGTW1`.
//!
//! # Notes
//! This driver is hardcoded to use a 32.768 kHz clock source.  If the sub-clock oscillator
//! is installed it will use that.  Otherwise the low-speed on chip oscillator (`LOCO`) will.
//! be used. Note that `LOCO` is accurate to within ±15% and it may be more appropriate to use
//! the `GPT` based time driver.

use core::{
    cell::{Cell, RefCell},
    marker::PhantomData,
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
};

use critical_section::{CriticalSection, Mutex};
use embassy_hal_internal::interrupt::InterruptExt as _;
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;
use fugit::{HertzU32, KilohertzU32, MegahertzU32, RateExtU32 as _};

use crate::{
    event_link::IcuInterrupt,
    interrupt,
    interrupt::typelevel::Interrupt,
    pac::{
        self,
        agtw::{
            regs::Agtcr,
            vals::{Cks, Lpm, Tck, Tmod},
        },
    },
};

/// If timestamps are enabled in defmt `now()` can get called before the timer is actually
/// initialized. After reset the count value is garbage until the timer has been initialized…
static READY: AtomicBool = AtomicBool::new(false);

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

trait TimerPause {
    fn while_paused<F>(&mut self, func: F)
    where
        F: Fn(&Self);
}

impl TimerPause for TimerPeripheral {
    #[inline]
    fn while_paused<F>(&mut self, func: F)
    where
        F: FnOnce(&Self),
    {
        self.agtcr().modify(|r| r.set_tstop(true));
        while self.agtcr().read().tcstf() {}

        // RA4L1 § 21.4.3:
        // When the registers associated with AGT operating mode are changed, the values of TEDGF,
        // TUNDF, TCMAF, and TCMBF flags are undefined. Before starting the count, write 0 to
        // the following flags:
        // * TEDGF (no active edge received)
        // * TUNDF (no underflow)
        // * TCMAF (no match)
        // * TCMBF (no match).

        self.agtcr().modify(|r| {
            r.set_tedgf(false);
            r.set_tundf(false);
            r.set_tcmaf(false);
            r.set_tcmbf(false);
        });

        func(self);

        self.agtcr().modify(|r| r.set_tstart(true));
        while !self.agtcr().read().tcstf() {}
    }
}

trait Instance: crate::timer_agt::Instance<u32> + Send + Sync + 'static {
    type AlarmInterrupt: interrupt::typelevel::Interrupt;
    type UnderflowInterrupt: interrupt::typelevel::Interrupt;
}

macro_rules! driver_instance {
    ($peri:ident) => {
        impl Instance for crate::peripherals::$peri {
            type AlarmInterrupt = crate::interrupt::typelevel::IEL1;
            type UnderflowInterrupt = crate::interrupt::typelevel::IEL0;
        }
    };
}

// TODO: Make this more flexible, but also aware of potential interactions.
cfg_select! {
    agt => {
        use crate::peripherals::AGT1;
        driver_instance!(AGT1);
        type TimerPeripheral = crate::pac::agt::Agt;
    },
    agtw => {
        use crate::peripherals::AGTW1;
        driver_instance!(AGTW1);
        type TimerPeripheral = crate::pac::agtw::Agtw;
    }
}

struct AgtDriver<I: Instance> {
    /// Number of 2^16 or 2^32 periods elapsed since boot.
    period: AtomicU32,
    queue: Mutex<RefCell<Queue>>,
    alarms: Mutex<AlarmState>,
    phantom: PhantomData<I>,
}

impl<I: Instance> AgtDriver<I> {
    // Fudge factor to ensure we don't fire early
    const FUDGE_FACTOR: u64 = 10;

    pub(crate) fn init(&'static self) {
        I::start_module();

        let system = pac::SYSTEM;
        let timer = I::regs();
        let clock_config = crate::clock::clock_status();
        let agt_source = match clock_config.sosc {
            true => {
                if system.sosccr().read().sostp() {
                    panic!("Sub-clock Oscillator required but disabled.");
                }

                Tck::Agtsclk
            }
            false => {
                warn!(
                    "Sub-clock Oscillator not installed/configured. Using Low-speed On Chip Oscillator (±15%)."
                );
                Tck::Agtlclk
            }
        };

        // Safety: These are safe because we've hardcoded interrupt handlers.
        unsafe {
            // Enable both interrupts at the NVIC level
            I::AlarmInterrupt::IRQ.enable();
            I::UnderflowInterrupt::IRQ.enable();

            // Arm the underflow interrupt in the ICU
            I::AlarmInterrupt::IRQ.icu_enable(I::COMP_A_EVENT);
            I::UnderflowInterrupt::IRQ.icu_enable(I::UNDERFLOW_EVENT);
        };

        timer.agtcr().write_value(Agtcr(0));
        while timer.agtcr().read().tcstf() {}

        // Do not switch the TCK[2:0] bits in the AGTMR1 register when CKS[2:0] bits are not 000b.
        // Switch the TCK[2:0] bits in the AGTMR1 register after CKS[2:0] bits are set to 000b,
        // and wait for 1 cycle of the count source.
        timer.agtmr2().modify(|r| r.set_cks(Cks::Div1));
        for _ in 0..2 {
            timer.agt().read();
        }

        timer.agtmr1().modify(|r| {
            r.set_tck(agt_source);
            r.set_tmod(Tmod::Timer);
        });
        timer.agtmr2().modify(|r| {
            r.set_cks(Cks::Div1);
            r.set_lpm(Lpm::Normal);
        });
        timer.agt().write_value(u32::MAX);
        timer.agtcr().modify(|r| {
            r.set_tedgf(false);
            r.set_tundf(false);
            r.set_tcmaf(false);
            r.set_tcmbf(false);
        });
        timer.agtcr().modify(|r| {
            r.set_tstart(true);
        });

        READY.store(true, Ordering::SeqCst);

        let mhz: HertzU32 = 1_u32.MHz();
        let tick: HertzU32 = (embassy_time_driver::TICK_HZ as u32).Hz();
        let peri = cfg_select! {
            agtw => "AGTW1",
            agt => "AGT1",
            _ => unreachable!()
        };
        if tick >= mhz {
            let tick: MegahertzU32 = tick.convert();
            info!("{}: Time driver attached, tick={}", peri, tick);
        } else {
            let tick: KilohertzU32 = tick.convert();
            info!("{}: Time driver attached, tick={}", peri, tick);
        }
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

    fn interrupted_underflow(&'static self) {
        critical_section::with(|_cs| {
            let mut timer = I::regs();

            timer.while_paused(|timer| {
                timer.agt().write_value(u32::MAX);

                for _ in 0..4 {
                    timer.agtcr().read();
                }
            });

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
        let mut timer = I::regs();

        let alarm = self.alarms.borrow(*cs);
        alarm.timestamp.set(timestamp);

        let t = self.now();
        if timestamp <= t {
            // Disarm the alarm and return `false` to indicate that.
            // Okay so we're not really disarming anything because setup/tear down is so slow.
            // Should we?
            alarm.timestamp.set(u64::MAX);

            return false;
        }

        let safe_timestamp = (timestamp.max(t + Self::FUDGE_FACTOR) & 0xFFFF_FFFF) as u32;

        let diff = timestamp - t;

        let cur = timer.agt().read();
        if diff < u64::from(u32::MAX) {
            let desired = u32::MAX
                .checked_sub(safe_timestamp)
                .expect("safe_timestamp > u32::MAX");

            // If we update the counter while the compare register is valid we have to wait for underflow for the new value to take effect.
            timer.while_paused(|timer| {
                // Guess there's some sort of waiting period after updating AGTCR too.
                // TODO: Looks like 1 or 2 AGT[SL]CLK cycles is appropriate.
                for _ in 0..500 {
                    timer.agtcr().read();
                }

                timer.agt().write_value(cur);

                timer.agtcma().write_value(desired);
            });

            timer.agtcmsr().modify(|r| r.set_tcmea(true));
        } else {
            // TODO: Uhhhhh
            // If alarm must trigger some time after the current period, too far in the future,
            // don't setup the alarm enable yet. It will be setup later by `next_period`.
        }

        true
    }
}

impl<I: Instance> Driver for AgtDriver<I> {
    fn now(&self) -> u64 {
        let timer = I::regs();

        if !READY.load(Ordering::SeqCst) {
            return 0;
        }

        let period = self.period.load(Ordering::Acquire);
        let count = timer.agt().read();
        let count = u32::MAX.checked_sub(count).unwrap();
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

#[cfg(agtw)]
embassy_time_driver::time_driver_impl!(static DRIVER: AgtDriver<AGTW1> = AgtDriver {
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
    let icu = crate::pac::ICU;

    icu.ielsr(0).modify(|r| r.set_ir(false));
    DRIVER.interrupted_underflow();
}

#[interrupt]
fn IEL1() {
    let icu = crate::pac::ICU;

    icu.ielsr(1).modify(|r| r.set_ir(false));
    DRIVER.interrupted_alarm();
}
