//! `spi_sensor` Reads the CHIPID and current temperature from a Bosch BMI-160 sensor via SPI.

#![no_std]
#![no_main]

use assign_resources::assign_resources;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_hal_internal::Peri;
use embassy_time::Timer;
use mini_sensors::bmi160::{BoschBmi160, vals::GyroPowerMode};
use panic_probe as _;
use ra_hal::{
    clock::ClockConfig,
    peripherals,
    spi::{self, Spi},
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

cfg_select! {
    feature = "uno-r4-minima" => {
        // Define the pins we want on the R4 Minima
        // Note: SPI1 will not work without the "swd-as-gpio" feature.
        // Note: SPI0 will work with alternative pins that are not at the locations Arduino labels as "SPI".
        assign_resources! {
            spi: SpiResources {
                peri: SPI0,
                sck: P102,
                mosi: P101,
                miso: P100,
                cs: P103,
            }
        }
    },
    feature = "uno-r4-wifi" => {
        // Define the pins we want on the R4 WiFi
        assign_resources! {
            spi: SpiResources {
                peri: SPI0,
                sck: P102,
                mosi: P411,
                miso: P410,
                cs: P103,
            }
        }
    }
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

    let mut config = spi::Config::default();
    // Configure a 250 kHz bitrate, slow enough to play nice with the cheap Saleae clones.
    config.bit_rate = 250_000;

    // Configure an SPI driver with:
    // - 16-bit words because that's what the BMI-160 uses
    // - hardware CS control because why not
    let spi = Spi::<_, u16, _>::new_blocking(
        r.spi.peri, r.spi.sck, r.spi.mosi, r.spi.miso, r.spi.cs, config,
    );
    let mut sensor = BoschBmi160::new_spi(spi);

    info!("Chip ID: 0x{:02X}", sensor.chipid());

    sensor.set_gyro_mode(GyroPowerMode::Normal);

    loop {
        let temp = sensor.temperature();
        info!("Temp: {}", temp);
        Timer::after_millis(500).await;
    }
}
