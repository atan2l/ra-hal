//! `adc_input` Reads an ADC channel and prints the result

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use ra4_hal::{
    adc::{Adc, AdcConfig, AdcPin},
    gpio::{DriveCapacity, Level, Output},
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

    let _output = Output::new_basic(p.P410, Level::High, DriveCapacity::Low);

    let input = AdcPin::new(p.P103);

    let adc = Adc::new(p.ADC14, AdcConfig::default());

    loop {
        let mv = adc.blocking_read(&input);
        info!("mV = {}", mv);

        Timer::after_millis(500).await;
    }
}
