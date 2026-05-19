//! `blink` Blinks an onboard LED and pulses a different one at the same time.
//!
//! This example uses RTICv2 with two equal priority tasks, one per action.
//! Because tasks cannot be generic, `assign-resources` is used to create
//! concrete types while still allowing for use with different boards with
//! relative ease.

#![no_main]
#![no_std]

use assign_resources::assign_resources;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use panic_probe as _;
use ra_hal::{Peri, peripherals};
use rtic_monotonics::systick::prelude::*;

pub mod pac {
    pub use ra_hal::pac::Interrupt as interrupt;
    pub use ra_hal::pac::*;
}

const MAX_DUTY_CYCLE: f32 = 0.50;
const MIN_DUTY_CYCLE: f32 = 0.15;
const STEP: f32 = 0.00625;
const DELAY_MS: f32 = 4000.0 * STEP;

systick_monotonic!(Mono, 1_000);

// Define the pins we want on the RA4L1 Eval Kit
#[cfg(feature = "ek-ra4l1")]
assign_resources! {
    blink: BlinkResources {
        led: P609,
    }
    pulse: PulseResources {
        pin: P610,
        timer: GPT16_5,
    }
}

fn ease(mut t: f32) -> f32 {
    use micromath::F32Ext as _;

    t *= 2.0;
    if t <= 1.0 {
        t = t.powi(3);
    } else {
        t -= 2.0;
        t = t.powi(3);
        t += 2.0;
    }
    t / 2.0
}

// For now IEL0 and IEL1 are occupied by the embassy time driver.
#[rtic::app(device = pac, peripherals = false, dispatchers = [IEL2, IEL3])]
mod app {
    use super::*;
    use micromath::F32Ext as _;
    use ra_hal::{
        clock::ClockConfig,
        gpio::{Level, Output},
        pwm::{self, Pwm, PwmChansetter as _},
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let p = ra_hal::init(ClockConfig::default());
        let r = split_resources!(p);

        let clock_config = ra_hal::clock::clock_status();
        Mono::start(cx.core.SYST, clock_config.system.to_Hz());

        blink::spawn(r.blink).ok();

        pulse::spawn(r.pulse).ok();

        (Shared {}, Local {})
    }

    #[task(priority = 1)]
    async fn blink(_cx: blink::Context, r: BlinkResources) {
        let mut led = Output::new_basic(r.led, Level::Low);

        loop {
            led.toggle();
            Mono::delay(450.millis()).await;
        }
    }

    #[task(priority = 1)]
    async fn pulse(_cx: pulse::Context, r: PulseResources) {
        let mut duty_cycle: f32 = 0.00;
        let mut direction_up = true;
        let delay_ms = (DELAY_MS.round() as u32 * 4) / 3;
        let pwm_config = pwm::Config::default();

        let mut pwm = Pwm::new(r.timer, pwm_config).with_channel(r.pin);
        pwm.set_frequency(5000, 0.0).unwrap();
        pwm.start();

        loop {
            Mono::delay(delay_ms.millis()).await;

            let cur_duty_cycle = duty_cycle;
            let duty_cycle_eased = ease(cur_duty_cycle);

            defmt::trace!("dc={}, ease={}", cur_duty_cycle, duty_cycle_eased);
            pwm.set_duty_pct(duty_cycle_eased);

            if cur_duty_cycle >= MAX_DUTY_CYCLE {
                direction_up = false;
            } else if cur_duty_cycle <= MIN_DUTY_CYCLE {
                direction_up = true;
            }

            match direction_up {
                true => duty_cycle = cur_duty_cycle + STEP,
                false => duty_cycle = cur_duty_cycle - STEP,
            }
            duty_cycle = duty_cycle.clamp(MIN_DUTY_CYCLE, MAX_DUTY_CYCLE);
        }
    }
}
