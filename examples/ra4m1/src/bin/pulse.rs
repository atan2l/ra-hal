//! `pulse` Pulses the onboard LED.
//!
//! Slightly more advanced than the blink example, the pulse example uses PWM to control the
//! brightness of the onboard LED. Combined with an easing function this gives the LED a somewhat
//! organic pulsating appearance.

#![no_std]
#![no_main]

use assign_resources::assign_resources;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use micromath::F32Ext as _;
use panic_probe as _;
use ra_hal::{
    Peri,
    clock::ClockConfig,
    peripherals,
    pwm::{self, Pwm, PwmChansetter as _},
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

const MAX_DUTY_CYCLE: f32 = 0.50;
const MIN_DUTY_CYCLE: f32 = 0.15;
const STEP: f32 = 0.00625;
const DELAY_MS: f32 = 4000.0 * STEP;

cfg_select! {
    feature = "uno-r4-minima" => {
        // Define the pins we want on the R4 Minima
        assign_resources! {
            pulse: PulseResources {
                led: P111,
                timer: GPT16_3,
            }
        }
    },
    feature = "uno-r4-wifi" => {
        // Define the pins we want on the R4 WiFi
        assign_resources! {
            pulse: PulseResources {
                led: P102,
                timer: GPT16_2,
            }
        }
    }
    _ => {
        compile_error!(
            "Ensure the pin and timer assignments are correct for your board before continuing."
        );
    }
}

fn ease(mut t: f32) -> f32 {
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

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());
    let r = split_resources!(p);

    let mut duty_cycle: f32 = 0.00;
    let mut direction_up = true;
    let delay_ms = DELAY_MS.round() as u64;
    let pwm_config = pwm::Config::default();

    let mut pwm = Pwm::new(r.pulse.timer, pwm_config).with_channel(r.pulse.led);
    pwm.set_frequency(5000, 0.0).unwrap();
    pwm.start();

    loop {
        Timer::after_millis(delay_ms).await;

        let cur_duty_cycle = duty_cycle;
        let duty_cycle_eased = ease(cur_duty_cycle);

        trace!("dc={}, ease={}", cur_duty_cycle, duty_cycle_eased);
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
