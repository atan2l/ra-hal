//! PWM example

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use micromath::F32Ext;
use panic_probe as _;
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};
use ra4_hal::{
    osm::{ofs0::Ofs0, ofs1::Ofs1, sec_mpu::SecurityMpu},
    print_clock_config,
    pwm::Pwm,
};

// Option Function Select Register 0 (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs0")]
static OFS0: Ofs0 = Ofs0::arduino_core();

// Option Function Select Register 1 (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs1")]
static OFS1: Ofs1 = Ofs1::arduino_core();

// Security MPU (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".sec_mpu")]
static SEC_MPU: SecurityMpu = SecurityMpu::disabled();

const MAX_DUTY_CYCLE: f32 = 1.00;
const MIN_DUTY_CYCLE: f32 = 0.00;
// Each step is a 1.25% change in the duty cycle
const STEP: f32 = 0.0125;

// Set up the delay so that a pulse takes 1 second.
const DELAY_MS: f32 = 2000.0 * STEP;

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
    let p = ra4_hal::init();

    print_clock_config();

    let mut pwm = Pwm::new_b(p.GPT16_2, p.P102);
    let mut duty_cycle: f32 = 0.00;
    let mut direction_up = true;
    let delay_ms = DELAY_MS.round() as u64;

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
