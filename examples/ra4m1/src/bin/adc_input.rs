//! `adc_input` Reads an ADC channel and prints the result

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use ra_hal::{
    adc::{Adc, AdcConfig, AdcPin},
    clock::ClockConfig,
    gpio::{DriveCapacity, Level, Output},
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());

    let _output = Output::new_basic(p.P410, Level::High, DriveCapacity::Low);

    let input = AdcPin::new(p.P103);

    let adc = Adc::new(p.ADC14_0, AdcConfig::default());

    loop {
        let mv = adc.blocking_read(&input);
        info!("mV = {}", mv);

        Timer::after_millis(500).await;
    }
}
