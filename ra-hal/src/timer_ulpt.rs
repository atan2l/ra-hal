//! Timer that generates interrupts utilizing the Low Power Asynchronous General Purpose Timer (`AGT` / `AGTW`).

use embassy_hal_internal::PeripheralType;

use crate::{module_stop::ModuleStop, timer_gpt::TimerWidth};

/// An [`InterruptTimer`] instance.
#[allow(private_bounds)]
pub trait Instance<Width: TimerWidth>: SealedInstance + ModuleStop + PeripheralType {
    /// Event link event upon a match with capture/compare value A.
    const COMP_A_EVENT: crate::event_link::InterruptEvent;

    /// Event link event upon a match with capture/compare value B.
    const COMP_B_EVENT: crate::event_link::InterruptEvent;

    /// Event link event for an overflow event.
    const UNDERFLOW_EVENT: crate::event_link::InterruptEvent;
}

pub(crate) trait SealedInstance: PeripheralType {
    const INDEX: usize;

    fn regs() -> crate::pac::ulpt::Ulpt;
}

macro_rules! timer_instance {
    ($peri:ident, $instance:literal, $width:ident) => {
        paste::paste! {
            impl Instance<$width> for crate::peripherals::[< $peri $instance >] {
                const COMP_A_EVENT: crate::event_link::InterruptEvent =
                    crate::event_link::InterruptEvent::[< Ulpt $instance CompareA >];

                const COMP_B_EVENT: crate::event_link::InterruptEvent =
                    crate::event_link::InterruptEvent::[< Ulpt $instance CompareB >];

                const UNDERFLOW_EVENT: crate::event_link::InterruptEvent =
                    crate::event_link::InterruptEvent::[< Ulpt $instance Int >];
            }

            impl SealedInstance for crate::peripherals::[< $peri $instance >] {
                const INDEX: usize = [< $instance >];

                fn regs() -> crate::pac::ulpt::Ulpt {
                    crate::pac::[< $peri $instance >]
                }
            }
        }
    };
}

timer_instance!(ULPT, 0, u32);
timer_instance!(ULPT, 1, u32);
