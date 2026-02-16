//! Pulse Width Modulation driver (`GPT`).
//!
//! PWM driver utilizing the General PWM Timer (`GPT`).
//!
//! # Notes
//! * The `RA4M1` has both 16-bit and 32-bit timer instances.
//!   This driver treats all instances as 16-bit for the sake of brevity.

use core::marker::PhantomData;

use embassy_hal_internal::{Peri, PeripheralType};
use paste::paste;
use ra4m1_ctpac::gpt::vals::{Ccr, Gtio, Mode, Odty, Tpcs};

use crate::{
    gpio::{Flex, Pin, PortFunction, WithOpenDrain},
    pac,
};

/// PWM configuration
pub struct Config {
    /// Timer prescaler. §22.2.12.
    pub divider: Divider,

    /// Where to toggle the output (channel A). §22.3.3.1.
    pub compare_a: u16,

    /// Where to toggle the output (channel B). §22.3.3.1.
    pub compare_b: u16,

    /// Maximum value of `GTCNT`.  The counter will reverse direction at this point.
    pub top: u16,
}

/// PWM clock divider
pub enum Divider {
    /// `PCLKD/1`
    Div1,

    /// `PCLKD/4`
    Div4,

    /// `PCLKD/16`
    Div16,

    /// `PCLKD/64`
    Div64,

    /// `PCLKD/256`
    Div256,

    /// `PCLKD/1024`
    Div1024,
}

/// PWM driver
pub struct Pwm<'d, I: Instance> {
    _instance: PhantomData<&'d I>,
    // These are set to WithOpenDrain because on the RA4M1 all PWM pins have both capabilities
    channel_a: Option<Flex<'d, WithOpenDrain>>,
    channel_b: Option<Flex<'d, WithOpenDrain>>,
}

/// PWM instance
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

pub(crate) trait SealedInstance {
    fn regs() -> pac::gpt::Gpt;
}

pub(crate) trait PwmChannel {}

/// PWM output pin trait.
#[allow(private_bounds)]
pub trait PwmPin<I: Instance, C: PwmChannel>: SealedPwmPin<I, C> {}

pub(crate) trait SealedPwmPin<I: SealedInstance, C: PwmChannel>:
    Pin + PeripheralType
{
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_pfunc(&self) {
        self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

trait SealedPwmStruct {}
impl<'d, I: Instance> SealedPwmStruct for Pwm<'d, I> {}

/// Trait that allows setting specific channels with the same function name.
#[allow(private_bounds)]
pub trait PwmChansetter<'d, C: PwmChannel, I: Instance>: SealedPwmStruct {
    /// Takes ownership of a pin and assigns it to output channel `C`.
    fn with_channel<A: PwmPin<I, C>>(self, pin_a: Peri<'d, A>) -> Self;
}

impl<'d, I: Instance> PwmChansetter<'d, ChanA, I> for Pwm<'d, I> {
    /// This will panic if Channel A has already been set.
    fn with_channel<A: PwmPin<I, ChanA>>(self, pin_a: Peri<'d, A>) -> Self {
        assert!(self.channel_b.is_none());

        let pwm = I::regs();

        pwm.gtior().modify(|w| {
            // Start=Low, End=Low, Match=Toggle
            w.set_gtioa(Gtio::_00111);
            w.set_oae(true);
        });
        pwm.gtber().modify(|w| w.set_ccra(Ccr::SingleBuffer));

        pin_a.set_pfunc();
        let pin_a = Flex::new(pin_a);

        Self {
            channel_a: Some(pin_a),
            ..self
        }
    }
}

impl<'d, I: Instance> PwmChansetter<'d, ChanB, I> for Pwm<'d, I> {
    /// This will panic if Channel B has already been set.
    fn with_channel<B: PwmPin<I, ChanB>>(self, pin_b: Peri<'d, B>) -> Self {
        assert!(self.channel_b.is_none());

        let pwm = I::regs();

        pwm.gtior().modify(|w| {
            // Start=Low, End=Low, Match=Toggle
            w.set_gtiob(Gtio::_00111);
            w.set_obe(true);
        });
        pwm.gtber().modify(|w| w.set_ccrb(Ccr::SingleBuffer));

        pin_b.set_pfunc();
        let pin_b = Flex::new(pin_b);

        Self {
            channel_b: Some(pin_b),
            ..self
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            divider: Divider::Div4,
            compare_a: 0,
            compare_b: 0,
            top: 0xFFFF,
        }
    }
}

impl From<Divider> for Tpcs {
    fn from(value: Divider) -> Self {
        match value {
            Divider::Div1 => Self::DIV_1,
            Divider::Div4 => Self::DIV_4,
            Divider::Div16 => Self::DIV_16,
            Divider::Div64 => Self::DIV_64,
            Divider::Div256 => Self::DIV_256,
            Divider::Div1024 => Self::DIV_1024,
        }
    }
}

impl<'d, I: Instance> Pwm<'d, I> {
    /// Consumes a `GPT` peripheral instance and returns a driver.
    ///
    /// # Arguments
    /// * `_peri` The `GPT` peripheral e.g. `GPT16_5` or `GPT32_1`.
    /// * `config` configuration
    ///
    /// # Returns
    ///
    /// A `PWM` driver with no output pins assigned and whose counter is initialized to `0` but has not been started.
    pub fn new(_peri: Peri<'d, I>, config: Config) -> Self {
        let pwm = I::regs();

        pwm.gtcr().modify(|w| w.set_md(Mode::TrianglePwm1));

        pwm.gtcnt().write_value(0);

        let mut this = Self {
            _instance: PhantomData,
            channel_a: None,
            channel_b: None,
        };

        this.set_config(config);

        this
    }

    /// Takes ownership of a pin and assigns it to output channel A.
    pub fn with_channel_a<A: PwmPin<I, ChanA>>(self, pin_a: Peri<'d, A>) -> Self {
        PwmChansetter::<ChanA, I>::with_channel::<A>(self, pin_a)
    }

    /// Takes ownership of a pin and assigns it to output channel B.
    pub fn with_channel_b<B: PwmPin<I, ChanB>>(self, pin_b: Peri<'d, B>) -> Self {
        PwmChansetter::<ChanB, I>::with_channel::<B>(self, pin_b)
    }

    /// Applies a `Config` struct.
    /// Does not reset the counter.
    pub fn set_config(&mut self, config: Config) {
        let pwm = I::regs();
        pwm.gtcr().modify(|w| w.set_tpcs(config.divider.into()));
        pwm.gtpr().write_value(config.top as u32);

        pwm.gtuddtyc().modify(|w| w.set_oadty(Odty::CompareMatch));
        pwm.gtccra().write_value(config.compare_a as u32);
        pwm.gtccrc().write_value(config.compare_a as u32);

        pwm.gtuddtyc().modify(|w| w.set_obdty(Odty::CompareMatch));
        pwm.gtccrb().write_value(config.compare_b as u32);
        pwm.gtccre().write_value(config.compare_b as u32);
    }

    /// Starts the PWM counter.
    #[inline]
    pub fn start(&mut self) {
        let pwm = I::regs();

        pwm.gtcr().modify(|w| w.set_cst(true));
    }

    /// Stops the PWM counter.
    #[inline]
    pub fn stop(&mut self) {
        let pwm = I::regs();

        pwm.gtcr().modify(|w| w.set_cst(false));
    }

    /// Sets the duty cycle for both channels to the same value.
    #[inline]
    pub fn set_duty_pct(&mut self, pct: f32) {
        self.set_duty_pct_a(pct);
        self.set_duty_pct_b(pct);
    }

    /// Sets the duty cycle for channel A if it's been assigned to a pin.
    pub fn set_duty_pct_a(&mut self, pct: f32) {
        if self.channel_a.is_none() {
            return;
        }

        let pwm = I::regs();
        let pct = pct.clamp(0.0, 1.0);
        let period = pwm.gtpr().read() as f32;
        let cmp = (period * (1.0 - pct)) as u32;

        if cmp == 0 {
            pwm.gtuddtyc().modify(|w| w.set_oadty(Odty::On));
        } else if cmp >= period as u32 {
            pwm.gtuddtyc().modify(|w| w.set_oadty(Odty::Off));
        } else {
            // This will center the peak

            pwm.gtuddtyc().modify(|w| w.set_oadty(Odty::CompareMatch));
            pwm.gtccra().write_value(cmp);
            pwm.gtccrc().write_value(cmp);
        }
    }

    /// Sets the duty cycle for channel B if it's been assigned to a pin.
    pub fn set_duty_pct_b(&mut self, pct: f32) {
        if self.channel_b.is_none() {
            return;
        }

        let pwm = I::regs();
        let pct = pct.clamp(0.0, 1.0);
        let period = pwm.gtpr().read() as f32;
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
}

macro_rules! declare_pwm_channel {
    ($chan:ident) => {
        paste! {
            // TODO: macro expansion in rustdoc?
            #[allow(missing_docs)]
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
