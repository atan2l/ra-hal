#![no_std]
#![no_main]

#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use ra_hal::{
    clock::ClockConfig,
    gpio::{DriveCapacity, Level, Output},
};

// Define the pins we want on the RA6M5 Eval Kit
#[cfg(feature = "ek-ra8m1")]
macro_rules! pins {
    ($p:ident) => {
        $p.P006
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
