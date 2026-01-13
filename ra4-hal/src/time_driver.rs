use core::cell::Cell;
use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};

use cfg_if::cfg_if;
use critical_section::{CriticalSection, Mutex};
use defmt::error;
use embassy_hal_internal::Peri;
use embassy_hal_internal::interrupt::InterruptExt;
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;
use ra4m1_ctpac::gpt32::vals::{Prkey, Tpcs};

use crate::pac;

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
    fn regs() -> pac::gpt32::Gpt32;
    fn int() -> crate::interrupt::Interrupt;
}

impl Instance for crate::peripherals::GPT320 {
    #[inline(always)]
    fn regs() -> crate::pac::gpt32::Gpt32 {
        crate::pac::GPT320
    }

    #[inline(always)]
    fn int() -> crate::interrupt::Interrupt {
        crate::interrupt::IEL0
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
        _timer: Peri<'static, crate::peripherals::GPT320>,
        _irq_prio: crate::interrupt::Priority,
    ) {
        use crate::peripherals::GPT320;
        unsafe {
            GPT320::int().enable();
        };

        let timer = GPT320::regs();

        // Disable write prot
        timer.gtwp().write(|w| {
            w.set_wp(false);
            w.set_prkey(Prkey::_0X_A5);
        });

        timer.gtcr().write(|w| {
            w.set_tpcs(Tpcs::_000);
        });

        timer.gtstr().write(|w| {
            w.set_cstrt0(true);
        });
    }

    fn interrupted(&'static self) {
        critical_section::with(|cs| {});
    }

    // #[must_use]
    // fn set_alarm(&self, cs: &CriticalSection, timestamp: u64) -> bool {
    //     let timer = T::regs();

    //     let alarm = self.alarms.borrow(*cs);
    //     alarm.timestamp.set(timestamp);

    //     let t = self.now();
    //     if timestamp <= t {
    //         let timer = T::regs();

    //         // Disarm the alarm and return `false` to indicate that.
    //         timer.idr(TIMER_CHANNEL).write(|w| {
    //             w.set_cpcs(false);
    //         });

    //         alarm.timestamp.set(u64::MAX);

    //         return false;
    //     }

    //     let safe_timestamp = (timestamp.max(t + Self::FUDGE_FACTOR) & 0xFFFF_FFFF) as u32;

    //     let diff = timestamp - t;

    //     if diff < u64::from(u32::MAX) {
    //         // Enable the compare interrupt
    //         timer.ier(TIMER_CHANNEL).write(|w| {
    //             w.set_cpcs(true);
    //         });
    //         timer.protected_write(|| {
    //             // Load the safe timestamp
    //             timer.rc(TIMER_CHANNEL).write(|w| {
    //                 w.set_rc(safe_timestamp);
    //             });
    //             timer.ier(TIMER_CHANNEL).write(|w| {
    //                 w.set_cpcs(true);
    //             });
    //         });
    //     } else {
    //         // TODO: UHhhhh
    //         // If alarm must trigger some time after the current period, too far in the future,
    //         // don't setup the alarm enable, gpreg2, yet. It will be setup later by `next_period`.
    //     }

    //     true
    // }
}

// impl Driver for TcDriver {
//     fn now(&self) -> u64 {
//         let timer = T::regs();

//         // Ignoring overflows for now
//         let period = self.period.load(Ordering::Acquire);
//         ((period as u64) << 32) + (timer.cv(TIMER_CHANNEL).read().0 as u64)
//     }

//     fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
//         critical_section::with(|cs| {
//             let mut queue = self.queue.borrow(cs).borrow_mut();
//             if queue.schedule_wake(at, waker) {
//                 let mut next = queue.next_expiration(self.now());
//                 while !self.set_alarm(&cs, next) {
//                     next = queue.next_expiration(self.now());
//                 }
//             }
//         });
//     }
// }

// embassy_time_driver::time_driver_impl!(static DRIVER: TcDriver = TcDriver{
//     period: AtomicU32::new(0),
//     queue: Mutex::new(RefCell::new(Queue::new())),
//     alarms: Mutex::new(AlarmState::new())
// });

// pub(crate) fn init(irq_prio: crate::interrupt::Priority) {
//     DRIVER.init(irq_prio)
// }
