#![no_std]
#![no_main]

use assign_resources::assign_resources;
#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use ra_hal::{
    Peri,
    clock::ClockConfig,
    gpio::{DriveCapacity, Level, Output},
    peripherals,
};

#[cfg(not(feature = "ek-ra6m5"))]
compile_error!(
    "Ensure the pin and timer assignments are correct for your board before continuing."
);

// Define the pins we want on the RA6M5 Eval Kit
#[cfg(feature = "ek-ra6m5")]
assign_resources! {
    blink: BlinkResources {
        led: P006
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
