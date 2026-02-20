//! `adc_sequence` — Reads multiple ADC channels sequentially.

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use ra4_hal::adc::{Adc, AdcConfig, AdcPin, AdcSequence, AverageMode};
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra4_hal::init();

    // All of these pins are exposed on the R4 Minima and R4 WiFi, but at different locations.
    let input = [
        AdcPin::new(p.P100),
        AdcPin::new(p.P101),
        AdcPin::new(p.P102),
        AdcPin::new(p.P103),
    ];

    let sequence = AdcSequence::new(input);

    let adc = Adc::new(
        p.ADC14,
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
