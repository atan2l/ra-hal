#![allow(missing_docs)]

use core::marker::PhantomData;

use embassy_hal_internal::{Peri, PeripheralType};
use paste::paste;
use ra4m1_ctpac::gpt::vals::{Ccrb, Gtiob, Mode, Odty, Tpcs};

use crate::{
    CLOCK_FREQUENCY,
    gpio::{Pin, PortFunction},
    pac,
};

#[warn(dead_code)]
const WARNING: &str = "allow(missing_docs)";

pub enum Divider {
    Div1,
    Div4,
    Div16,
    Div64,
    Div256,
    Div1024,
}

pub struct Config {
    pub divider: Divider,
    pub compare_a: u16,
    pub compare_b: u16,
    pub top: u16,
}

pub struct Pwm<'d, I: Instance> {
    _instance: PhantomData<&'d I>,
}

impl<'d, I: Instance> Pwm<'d, I> {
    const DIVIDER: Tpcs = Tpcs::DIV_4;

    #[inline(always)]
    fn period() -> f32 {
        let divider = match Self::DIVIDER {
            Tpcs::DIV_1 => 1.0,
            Tpcs::DIV_4 => 4.0,
            Tpcs::DIV_16 => 16.0,
            Tpcs::DIV_64 => 64.0,
            Tpcs::DIV_256 => 256.0,
            Tpcs::DIV_1024 => 1024.0,
            _ => unreachable!(),
        };

        let period = (CLOCK_FREQUENCY as f32 / divider) / 240.0;

        // Currently we assume all timers are 16-bit, but really timers 0 and 1 are 32-bit
        assert!(period as u32 <= u16::MAX as u32);

        period
    }

    #[inline]
    pub fn set_duty_pct(&mut self, pct: f32) {
        let pwm = I::regs();
        let pct = pct.clamp(0.0, 1.0);
        let period = Self::period();
        let cmp = (period * (1.0 - pct)) as u32;

        if cmp == 0 {
            pwm.gtuddtyc().modify(|w| w.set_obdty(Odty::On));
        } else if cmp >= period as u32 {
            pwm.gtuddtyc().modify(|w| w.set_obdty(Odty::Off));
        } else {
            // This will center the peak
            pwm.gtuddtyc().modify(|w| w.set_obdty(Odty::CompareMatch));
            pwm.gtccrb().write_value(cmp);
            pwm.gtccre().write_value(cmp);
        }
    }

    #[inline]
    fn start(&mut self) {
        let pwm = I::regs();

        pwm.gtcr().modify(|w| w.set_cst(true));
    }

    pub fn new_b<B: PwmPin<I, ChanB>>(_peri: Peri<'d, I>, pin_b: Peri<'d, B>) -> Self {
        let pwm = I::regs();

        let period = Self::period();
        pwm.gtcr().modify(|w| {
            w.set_md(Mode::TrianglePwm1);
            w.set_tpcs(Self::DIVIDER);
        });

        pwm.gtpr().write_value(period as u32);
        pwm.gtccrb().write_value(period as u32);
        pwm.gtccre().write_value(period as u32);
        pwm.gtcnt().write_value(0);
        pwm.gtior().modify(|w| {
            w.set_gtiob(Gtiob::_00111);
            w.set_obe(true);
        });
        pwm.gtber().modify(|w| {
            w.set_ccrb(Ccrb::SingleBuffer);
        });

        pin_b.set_pfunc();

        let mut this = Self {
            _instance: PhantomData,
        };

        this.set_duty_pct(0.00);
        this.start();
        this
    }
}

#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

#[allow(private_bounds)]
pub trait PwmPin<I: Instance, C: PwmChannel>: SealedPwmPin<I, C> {}

pub(crate) trait SealedInstance {
    fn regs() -> pac::gpt::Gpt;
}

pub(crate) trait SealedPwmPin<I: SealedInstance, C: PwmChannel>:
    Pin + PeripheralType
{
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_pfunc(&self) {
        self.set_as_pf(Self::PERIPHERAL_FUNC);
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

macro_rules! gpt_instance {
    ($size:literal, $instance:literal) => {
        paste! {
            impl Instance for crate::peripherals::[< GPT $size _ $instance >] {}
            impl SealedInstance for crate::peripherals::[< GPT $size _ $instance >]{
                fn regs() -> crate::pac::gpt::Gpt {
                    crate::pac::[< GPT $size _ $instance >]
                }
            }
        }
    };
}

gpt_instance!(32, 0);
gpt_instance!(32, 1);

gpt_instance!(16, 2);
gpt_instance!(16, 3);
gpt_instance!(16, 4);
gpt_instance!(16, 5);
gpt_instance!(16, 6);
gpt_instance!(16, 7);
