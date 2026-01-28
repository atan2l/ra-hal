//! AdcTemperature — Reads the CPU die temperature sensor via ADC

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
    ofs0, ofs1,
    osm::sec_mpu::SecurityMpu,
    print_clock_config, sec_mpu,
};
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};

/// Option Function Select Register 0
/// Accepts either:
/// - a series of configuration values
/// - the literal `ArduinoCore` which emulates the Arduino defaults
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs0")]
pub static OFS0: u32 = ofs0!(ArduinoCore);

/// Option Function Select Register 1
/// Accepts either:
/// - a series of configuration values
/// - the literal `ArduinoCore` which emulates the Arduino defaults
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs1")]
pub static OFS1: u32 = ofs1!(ArduinoCore);

/// Configures the Security MPU.  See the reference manual for more details.
/// Setting all bits to 1 would also work.  Setting all bits to 0 is a good way to brick your board.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".sec_mpu")]
pub static SEC_MPU: SecurityMpu = sec_mpu!(Disabled);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra4_hal::init();

    print_clock_config();

    let adc = Adc::new(p.ADC14, AdcConfig::default());
    let adc_channel = adc.temperature_channel();

    for _ in 0..5 {
        let v_s = adc.blocking_read(&adc_channel);
        let temp = adc_channel.millivolt_to_celsius(v_s);

        info!("Temp: raw={} mV, act={} °C", v_s, temp);

        Timer::after_millis(500).await;
    }
}
