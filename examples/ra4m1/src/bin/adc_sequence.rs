//! `adc_sequence` — Reads multiple ADC channels sequentially.
//!
//! Note: `ADC14` doesn't support arbitrary sequences, instead reads each selected channel in order.

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use ra_hal::{
    adc::{Adc, AdcConfig, AdcPin, AdcSequence, AverageMode},
    clock::ClockConfig,
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());

    // All of these pins are exposed on the R4 Minima and R4 WiFi, but at different locations.
    let input = [
        // AN022
        AdcPin::new(p.P100),
        // AN021
        AdcPin::new(p.P101),
        // AN020
        AdcPin::new(p.P102),
        // AN019
        AdcPin::new(p.P103),
    ];

    let sequence = AdcSequence::new(input);

    let adc = Adc::new(
        p.ADC14_0,
        // For each channel take 4 samples and return the average
        AdcConfig {
            average_mode: AverageMode::Average4,
            ..AdcConfig::default()
        },
    );

    loop {
        let mv = adc.blocking_read(&sequence);
        info!("ch = {}", sequence.channels());
        info!("mV = {}", mv);

        Timer::after_millis(500).await;
    }
}
