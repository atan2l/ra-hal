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

#[cfg(agtw)]
impl Instance<u32> for crate::peripherals::AGTW0 {
    const COMP_A_EVENT: crate::event_link::InterruptEvent =
        crate::event_link::InterruptEvent::Agt0CompareA;

    const COMP_B_EVENT: crate::event_link::InterruptEvent =
        crate::event_link::InterruptEvent::Agt0CompareB;

    const UNDERFLOW_EVENT: crate::event_link::InterruptEvent =
        crate::event_link::InterruptEvent::Agt0Int;
}

#[cfg(agtw)]
impl SealedInstance for crate::peripherals::AGTW0 {
    const INDEX: usize = 0;

    fn regs() -> crate::pac::agtw::Agtw {
        crate::pac::AGTW0
    }
}

#[cfg(agtw)]
impl Instance<u32> for crate::peripherals::AGTW1 {
    const COMP_A_EVENT: crate::event_link::InterruptEvent =
        crate::event_link::InterruptEvent::Agt1CompareA;

    const COMP_B_EVENT: crate::event_link::InterruptEvent =
        crate::event_link::InterruptEvent::Agt1CompareB;

    const UNDERFLOW_EVENT: crate::event_link::InterruptEvent =
        crate::event_link::InterruptEvent::Agt1Int;
}

#[cfg(agtw)]
impl SealedInstance for crate::peripherals::AGTW1 {
    const INDEX: usize = 1;

    fn regs() -> crate::pac::agtw::Agtw {
        crate::pac::AGTW1
    }
}
