//! Driver for [`embassy-time`](https://docs.embassy.dev/embassy-time).
//!
//! Can hardcoded to consume `IEL0` and `IEL1` and can be configured to use `GPT32_0` or `GPT32_1`.
//!
//! # TODO
//! * Allow use of a different `GPT` instance

use core::{
    cell::{Cell, RefCell},
    marker::PhantomData,
    sync::atomic::{AtomicU32, Ordering},
};

use critical_section::{CriticalSection, Mutex};
use embassy_hal_internal::interrupt::InterruptExt as _;
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;
use fugit::{HertzU32, KilohertzU32, MegahertzU32, RateExtU32};

use crate::{
    event_link::IcuInterrupt,
    interrupt,
    interrupt::typelevel::Interrupt,
    pac::gpt::{
        regs::{Gtdnsr, Gtupsr},
        vals::{Mode, Tpcs, Ud},
    },
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

trait Instance: crate::timer_gpt::Instance<u32> + Send + Sync + 'static {
    const PERIPHERAL: &'static str;
    const GPT_INDEX: usize;
    type AlarmInterrupt: interrupt::typelevel::Interrupt;
    type OverflowInterrupt: interrupt::typelevel::Interrupt;
}

cfg_select! {
    feature = "time-driver-gpt0" => {
        type TimerPeripheral = crate::peripherals::GPT32_0;
        impl Instance for crate::peripherals::GPT32_0 {
            const PERIPHERAL: &'static str = "GPT32_0";
            const GPT_INDEX: usize = 0;
            type AlarmInterrupt = crate::interrupt::typelevel::IEL1;
            type OverflowInterrupt = crate::interrupt::typelevel::IEL0;
        }
    }
    feature = "time-driver-gpt1" => {
        type TimerPeripheral = crate::peripherals::GPT32_1;
        impl Instance for crate::peripherals::GPT32_1 {
            const PERIPHERAL: &'static str = "GPT32_1";
            const GPT_INDEX: usize = 1;
            type AlarmInterrupt = crate::interrupt::typelevel::IEL1;
            type OverflowInterrupt = crate::interrupt::typelevel::IEL0;
        }
    }
    _ => {
        compil_error!("TODO: impl Instance for all 32-bit GPT instances.");
    }
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

        let clock_config = crate::clock::clock_status();

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

        let pclkd: HertzU32 = clock_config.peripheral_d;
        let tick: HertzU32 = (embassy_time_driver::TICK_HZ as u32).Hz();
        assert!(
            pclkd >= tick,
            "GPT clock must be greater than or equal to the tick rate. tick={}, pclkd={}",
            tick,
            pclkd,
        );
        let divider = pclkd / tick;
        assert_eq!(
            tick * divider,
            pclkd,
            "Tick rate is not a factor of the GPT clock. tick={}, pclkd={}",
            tick,
            pclkd,
        );

        let tpcs = match divider {
            1 => Tpcs::Div1,
            4 => Tpcs::Div4,
            16 => Tpcs::Div16,
            64 => Tpcs::Div64,
            256 => Tpcs::Div256,
            1024 => Tpcs::Div1024,
            _ => unimplemented!("ƒPCLKD must be a multiple (1,4,16,64,256,1024) of tick rate"),
        };

        timer.gtcr().write(|r| r.set_tpcs(tpcs));
        trace!("GTCR: {}", timer.gtcr().read());

        // Overflow at u32::MAX
        timer.gtpr().write_value(u32::MAX);
        trace!("GTPR: {}", timer.gtpr().read());

        timer.gtcnt().write_value(0);
        trace!("GTCNT: {}", timer.gtcnt().read());

        // This is faster??
        timer.gtssr().write(|r| r.set_cstrt(true));
        timer.gtstr().write(|r| r.set_cstrt(I::GPT_INDEX, true));

        let mhz: HertzU32 = 1_u32.MHz();
        if tick >= mhz {
            let tick: MegahertzU32 = tick.convert();
            info!("{}: Time driver attached, tick={}", I::PERIPHERAL, tick);
        } else {
            let tick: KilohertzU32 = tick.convert();
            info!("{}: Time driver attached, tick={}", I::PERIPHERAL, tick);
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

embassy_time_driver::time_driver_impl!(static DRIVER: GptDriver<TimerPeripheral> = GptDriver {
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
    crate::interrupt::IEL0.icu_unpend();

    DRIVER.interrupted_overflow();
}

#[interrupt]
fn IEL1() {
    crate::interrupt::IEL1.icu_unpend();

    DRIVER.interrupted_alarm();
}
