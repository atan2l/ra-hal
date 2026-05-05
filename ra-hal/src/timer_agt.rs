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

    #[cfg(agt)]
    fn regs() -> crate::pac::agt::Agt;
    #[cfg(agtw)]
    fn regs() -> crate::pac::agtw::Agtw;
}

macro_rules! timer_instance {
    ($peri:ident, $peri_mod:ident::$peri_type:ident, $instance:literal, $width:ident) => {
        paste::paste! {
            impl Instance<$width> for crate::peripherals::[< $peri $instance >] {
                const COMP_A_EVENT: crate::event_link::InterruptEvent =
                    crate::event_link::InterruptEvent::[< Agt $instance CompareA >];

                const COMP_B_EVENT: crate::event_link::InterruptEvent =
                    crate::event_link::InterruptEvent::[< Agt $instance CompareB >];

                const UNDERFLOW_EVENT: crate::event_link::InterruptEvent =
                    crate::event_link::InterruptEvent::[< Agt $instance Int >];
            }

            impl SealedInstance for crate::peripherals::[< $peri $instance >] {
                const INDEX: usize = [< $instance >];

                fn regs() -> crate::pac::$peri_mod::$peri_type {
                    crate::pac::[< $peri $instance >]
                }
            }
        }
    };
}

cfg_select! {
    agt => {
        timer_instance!(AGT, agt::Agt, 0, u16);
        timer_instance!(AGT, agt::Agt, 1, u16);
    },
    agtw => {
        timer_instance!(AGTW, agtw::Agtw, 0, u32);
        timer_instance!(AGTW, agtw::Agtw, 1, u32);
    },
    _ => {
        compile_error!("AGT timer enabled, but no AGT/AGTW hardware found");
    }
}
