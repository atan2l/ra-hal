#![allow(missing_docs)]

use embassy_hal_internal::PeripheralType;
use paste::paste;

use crate::gpio::{Pin, PortFunction};

#[warn(dead_code)]
const WARNING: &str = "allow(missing_docs)";

#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

#[allow(private_bounds)]
pub trait PwmPin<I: Instance, C: PwmChannel>: SealedPwmPin<I, C> {}

pub(crate) trait SealedInstance {}

pub(crate) trait SealedPwmPin<I: SealedInstance, C: PwmChannel>:
    Pin + PeripheralType
{
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn pfunc(&self) -> PortFunction {
        Self::PERIPHERAL_FUNC
    }
}

pub(crate) trait PwmChannel {}
macro_rules! declare_pwm_channel {
    ($chan:ident) => {
        paste! {
            pub enum [< Chan $chan >]  {}
            impl PwmChannel for [< Chan $chan >] {}
        }
    };
}

/// Declares a PWM pin
///
/// # Arguments
/// * `$instance` GPT instance e.g. `GPT32_0`, `GPT16_2`
/// * `$chan` channel either `ChanA` or `ChanB`
/// * `$pin` peripheral name for the pin
/// * `$pf` Peripheral Function
macro_rules! pwm_pin {
    ($instance:ident, $chan:ident, $pin:ident, $pf:ident) => {
        impl crate::pwm::PwmPin<crate::peripherals::$instance, crate::pwm::$chan>
            for crate::peripherals::$pin
        {
        }
        impl crate::pwm::SealedPwmPin<crate::peripherals::$instance, crate::pwm::$chan>
            for crate::peripherals::$pin
        {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pf;
        }
    };
}
pub(crate) use pwm_pin;

declare_pwm_channel!(A);
declare_pwm_channel!(B);

impl Instance for crate::peripherals::GPT32_0 {}
impl SealedInstance for crate::peripherals::GPT32_0 {}

impl Instance for crate::peripherals::GPT32_1 {}
impl SealedInstance for crate::peripherals::GPT32_1 {}

impl Instance for crate::peripherals::GPT16_2 {}
impl SealedInstance for crate::peripherals::GPT16_2 {}
impl Instance for crate::peripherals::GPT16_3 {}
impl SealedInstance for crate::peripherals::GPT16_3 {}
impl Instance for crate::peripherals::GPT16_4 {}
impl SealedInstance for crate::peripherals::GPT16_4 {}
impl Instance for crate::peripherals::GPT16_5 {}
impl SealedInstance for crate::peripherals::GPT16_5 {}
impl Instance for crate::peripherals::GPT16_6 {}
impl SealedInstance for crate::peripherals::GPT16_6 {}
impl Instance for crate::peripherals::GPT16_7 {}
impl SealedInstance for crate::peripherals::GPT16_7 {}
