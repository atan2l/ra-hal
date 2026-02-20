//! `i2c-sensor` An example of how to query a Bosch BMI-160.

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use mini_sensors::bmi160::{self, BoschBmi160, vals::GyroPowerMode};
use panic_probe as _;
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};
use ra4_hal::{
    i2c::{I2c, I2cSpeed},
    mode::Blocking,
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra4_hal::init();

    let scl = p.P100;
    let sda = p.P101;

    let i2c = I2c::<Blocking, _>::new(p.IIC1, scl, sda, I2cSpeed::Normal);
    let mut sensor = BoschBmi160::new_i2c(i2c, bmi160::I2cAddress::Alt);

    let chip_id = sensor.regs().chip_id().read().unwrap();
    info!("ChipId: {}", chip_id);

    let power_mode = sensor.regs().power_mode().read().unwrap();
    info!("PowerMode: {}", power_mode);

    let status = sensor.regs().status().read().unwrap();
    info!("Status: {}", status);

    let step_config = sensor.regs().step_config().read().unwrap();
    info!("StepConfig: {}", step_config);

    // Temperature readings aren't valid without the gyroscope running
    sensor.set_gyro_mode(GyroPowerMode::Normal);

    loop {
        let time = sensor.regs().sensor_time().read().unwrap();
        info!("Time: {}", time);

        let temp = sensor.temperature();
        info!("Temperature: {} °C", temp);

        Timer::after_millis(333).await;
    }
}
