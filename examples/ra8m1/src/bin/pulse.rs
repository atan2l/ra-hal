//! `pulse` Pulses the LEDs onboard the EK-RA8M1.
//!
//! Slightly more advanced than the blink example, the pulse example uses PWM to control the
//! brightness of the onboard LEDs. Combined with an easing function this gives the LED a
//! somewhat organic pulsating appearance.

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
const DELAY_BLUE_MS: f32 = 4000.0 * STEP;
const DELAY_GREEN_MS: f32 = 3000.0 * STEP;
const DELAY_RED_MS: f32 = 2000.0 * STEP;

#[cfg(not(feature = "ek-ra8m1"))]
compile_error!(
    "Ensure the pin and timer assignments are correct for your board before continuing."
);

#[cfg(feature = "ek-ra8m1")]
assign_resources! {
    red: RedResources {
        p: P107 = RedPin,
        t: GPT16_8 = RedTimer,
    }
    green: GreenResources {
        p: P414 = GreenPin,
        t: GPT32_0 = GreenTimer,
    }
    blue: BlueResources {
        p: P600 = BluePin,
        t: GPT32_6 = BlueTimer,
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

async fn pulse<I: pwm::Instance, C: pwm::PwmChannel, P: pwm::PwmPin<I, C>>(
    timer: Peri<'static, I>,
    pin: Peri<'static, P>,
    delay: f32,
) where
    Pwm<'static, I>: pwm::PwmChansetter<'static, C, I>,
{
    let mut duty_cycle: f32 = 0.00;
    let mut direction_up = true;
    let delay_ms = delay.round() as u64;

    let pwm_config = pwm::Config::default();
    let mut pwm = Pwm::new(timer, pwm_config).with_channel(pin);
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

#[embassy_executor::task]
async fn pulse_red(r: RedResources) {
    pulse(r.t, r.p, DELAY_RED_MS).await;
}

#[embassy_executor::task]
async fn pulse_green(r: GreenResources) {
    pulse(r.t, r.p, DELAY_GREEN_MS).await;
}

#[embassy_executor::task]
async fn pulse_blue(r: BlueResources) {
    pulse(r.t, r.p, DELAY_BLUE_MS).await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());
    let r = split_resources!(p);

    spawner.spawn(pulse_red(r.red).unwrap());
    spawner.spawn(pulse_green(r.green).unwrap());
    spawner.spawn(pulse_blue(r.blue).unwrap());
}
