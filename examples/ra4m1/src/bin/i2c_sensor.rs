//! `i2c-sensor` An example of how to query a Bosch BMI-160.

#![no_std]
#![no_main]

use assign_resources::assign_resources;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use mini_sensors::bmi160::{self, BoschBmi160, vals::GyroPowerMode};
use panic_probe as _;
use ra_hal::{
    Peri,
    clock::ClockConfig,
    i2c::{I2c, I2cSpeed},
    peripherals,
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

cfg_select! {
    any(feature = "uno-r4-minima", feature = "uno-r4-wifi") => {
        assign_resources! {
            i2c: I2cResources {
                peri: IIC1,
                scl: P100,
                sda: P101,
            }
        }
    },
    _ => {
        compile_error!(
            "Ensure the pin and timer assignments are correct for your board before continuing."
        );
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());
    let r = split_resources!(p);

    let i2c = I2c::new_blocking(r.i2c.peri, r.i2c.scl, r.i2c.sda, I2cSpeed::Normal);
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
