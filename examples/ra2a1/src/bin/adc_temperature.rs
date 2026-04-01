//! `adc_temperature` Reads the CPU die temperature sensor via ADC and prints the result

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use ra_hal::{
    adc::{Adc, AdcConfig},
    clock::ClockConfig,
    exit
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());

    let adc_config = AdcConfig::default();
    let adc = Adc::new(p.ADC16_0, adc_config);
    let adc_channel = adc.temperature_channel();

    for _ in 0..5 {
        let v_s = adc.blocking_read(&adc_channel);

        // Use `raw_to_celsius_float` to get a floating-point result using softfloat.
        let temp = adc_channel.raw_to_celsius(v_s, 2500, adc_config.resolution);

        info!("Temp: raw={}, act={} °C", v_s, temp);

        Timer::after_millis(500).await;
    }

    exit!();
}
