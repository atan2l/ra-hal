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
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use ra_hal::{
    clock::ClockConfig,
    gpio::{DriveCapacity, Level, Output},
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

// Define the pins we want on the R4 Minima
#[cfg(feature = "uno-r4-minima")]
macro_rules! pins {
    ($p:ident) => {
        $p.P111
    };
}

// Define the pins we want on the R4 WiFi
#[cfg(feature = "uno-r4-wifi")]
macro_rules! pins {
    ($p:ident) => {
        $p.P102
    };
}

// Define the pins we want on the EK-RA4M1
#[cfg(feature = "ek-ra4m1")]
macro_rules! pins {
    ($p:ident) => {
        $p.P106
    };
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());

    let mut led = Output::new_basic(pins!(p), Level::Low, DriveCapacity::Low);

    loop {
        led.toggle();
        Timer::after_millis(333).await;
    }
}
