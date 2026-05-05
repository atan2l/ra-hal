//! `blink` hello world example
//!
//! This example will blink an onboard LED, toggling the state every 333 milliseconds.
//!
//! # Further Reading
//! There are multiple ways to accomplish this.  The loop could contain explicitly set the output
//! level high and then low.  Or, the loop could read the current output state and set it
//! accordingly.

#![no_std]
#![no_main]

use assign_resources::assign_resources;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_hal_internal::Peri;
use embassy_time::Timer;
use panic_probe as _;
use ra_hal::{
    clock::ClockConfig,
    gpio::{DriveCapacity, Level, Output},
    peripherals,
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

cfg_select! {
    feature = "uno-r4-minima" => {
        // Define the pins we want on the R4 Minima
        assign_resources! {
            blink: BlinkResources {
                led: P111
            }
        }
    },
    feature = "uno-r4-wifi" => {
        // Define the pins we want on the R4 WiFi
        assign_resources! {
            blink: BlinkResources {
                led: P102
            }
        }
    }
    feature = "ek-ra4m1" => {
        // Define the pins we want on the EK-RA4M1
        assign_resources! {
            blink: BlinkResources {
                led: P106
            }
        }
    }
    _ => {
        compile_error!(
            "Ensure the pin and timer assignments are correct for your board before continuing."
        );
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());
    let r = split_resources!(p);

    let mut led = Output::new_basic(r.blink.led, Level::Low, DriveCapacity::Low);

    loop {
        led.toggle();
        Timer::after_millis(333).await;
    }
}
