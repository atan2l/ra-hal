//! `watchdog` blinks an LED until the watchdog underflows
//!
//! This example enables the watchdog timer (`WDT`) at runtime but does not feed it. Execution
//! will halt once the `WDT` counter underflows.
//!
//! # Further Reading
//!
//! `WDT` can also be configured and enabled at reset time via the Option Setting Memory (`OSM`)
//! which is configured at compile time.  This is also the *only* way the independent watchdog timer
//! (`IWDT`) can be configured and enabled.

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use ra_hal::{
    clock::ClockConfig,
    gpio::{DriveCapacity, Level, Output},
    watchdog::{
        Action,
        wdt::{ClockDivider, Config, TimeoutPeriod, Watchdog},
    },
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

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());

    let mut led = Output::new_basic(pins!(p), Level::Low, DriveCapacity::Low);

    let mut wdt_config = Config::default();

    // Trigger a processor reset on watchdog underflow. §5.3.5.
    wdt_config.action = Action::Reset;

    // Decrement the watchdog counter every 8192 cycles of PCLKB (typically 24 MHz).
    wdt_config.divider = ClockDivider::Div8192;

    // Set the watchdog counter to 4096 on refresh.
    wdt_config.period = TimeoutPeriod::_4096;

    // wdt needs to be mutable if we want to poke at it later
    #[allow(unused_mut)]
    let mut wdt = Watchdog::new(p.WDT, wdt_config);

    loop {
        info!("Watchdog counter = {}", wdt.value());
        led.toggle();

        // Uncomment the next line to let the watchdog know we're alive
        // wdt.refresh();

        Timer::after_millis(333).await;
    }
}
