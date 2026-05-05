//! `gpio_poll` GPIO input example (polling)
//!
//! Connect a button between D12 and ground on an Uno R4 and this will toggle the builtin LED
//! when the button is pressed.
//!
//! This version will poll the PORT peripheral for the current input status.
//! To see an interrupt driven version see `gpio_interrupt`.

#![no_std]
#![no_main]

use assign_resources::assign_resources;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_hal_internal::Peri;
use panic_probe as _;
use ra_hal::{
    clock::ClockConfig,
    gpio::{DriveCapacity, Input, Level, Output},
    peripherals,
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

cfg_select! {
    feature = "uno-r4-minima" => {
        // Define the pins we want on the R4 Minima
        assign_resources! {
            gpio: GpioResources {
                button: P110,
                led: P111,
            }
        }
    },
    feature = "uno-r4-wifi" => {
        // Define the pins we want on the R4 WiFi
        assign_resources! {
            gpio: GpioResources {
                button: P410,
                led: P102,
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

    let mut button = Input::new_with_pull_up(r.gpio.button, true);

    let mut led = Output::new_basic(r.gpio.led, Level::Low, DriveCapacity::Low);

    loop {
        // The button pulls the line to ground so is_high() == false when the button is pressed.
        // set_level() takes level which implements From<bool>
        // So press the button and the LED lights up.
        led.set_level(button.is_low().into());
    }
}
