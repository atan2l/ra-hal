//! `gpio_poll` GPIO input example (polling)
//!
//! Connect a button between D12 and ground on an Uno R4 and this will toggle the builtin LED
//! when the button is pressed.
//!
//! This version will poll the PORT peripheral for the current input status.
//! To see an interrupt driven version see `gpio_interrupt`.

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;
use ra_hal::{
    clock::ClockConfig,
    gpio::{DriveCapacity, Input, Level, Output},
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

// Define the pins we want on the R4 Minima
#[cfg(feature = "uno-r4-minima")]
macro_rules! pins {
    ($p:ident) => {
        ($p.P110, $p.P111)
    };
}

// Define the pins we want on the R4 WiFi
#[cfg(feature = "uno-r4-wifi")]
macro_rules! pins {
    ($p:ident) => {
        ($p.P410, $p.P102)
    };
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());

    // Grab D12 and the LED pins
    let (button, led) = pins!(p);

    let mut button = Input::new_with_pull_up(button, true);

    let mut led = Output::new_basic(led, Level::Low, DriveCapacity::Low);

    loop {
        // The button pulls the line to ground so is_high() == false when the button is pressed.
        // set_level() takes level which implements From<bool>
        // So press the button and the LED lights up.
        led.set_level(button.is_low().into());
    }
}
