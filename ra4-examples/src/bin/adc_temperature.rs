//! `adc_temperature` Reads the CPU die temperature sensor via ADC and prints the result

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use ra4_hal::{
    adc::{Adc, AdcConfig},
    osm::{ofs0::Ofs0, ofs1::Ofs1, sec_mpu::SecurityMpu},
};
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};

// Option Function Select Register 0 (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs0")]
static OFS0: Ofs0 = Ofs0::default();

// Option Function Select Register 1 (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs1")]
static OFS1: Ofs1 = Ofs1::default();

// Security MPU (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".sec_mpu")]
static SEC_MPU: SecurityMpu = SecurityMpu::disabled();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra4_hal::init();

    let adc = Adc::new(p.ADC14, AdcConfig::default());
    let adc_channel = adc.temperature_channel();

    for _ in 0..5 {
        let v_s = adc.blocking_read(&adc_channel);
        let temp = adc_channel.millivolt_to_celsius(v_s);

        info!("Temp: raw={} mV, act={} °C", v_s, temp);

        Timer::after_millis(500).await;
    }
}
