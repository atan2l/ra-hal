//! `spi_sensor` Reads the CHIPID and current temperature from a Bosch BMI-160 sensor via SPI.

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use mini_sensors::bmi160::{BoschBmi160, vals::GyroPowerMode};
use panic_probe as _;
use ra_hal::{
    clock::ClockConfig,
    spi::{self, Spi},
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

// Define the pins we want on the R4 Minima
// Note: SPI1 will not work without the "swd-as-gpio" feature.
// Note: SPI0 will work with alternative pins that are not at the locations Arduino labels as "SPI".
#[cfg(feature = "uno-r4-minima")]
macro_rules! pins {
    ($p:ident) => {
        ($p.SPI0, $p.P102, $p.P101, $p.P100, $p.P103)
    };
}

// Define the pins we want on the R4 WiFi
#[cfg(feature = "uno-r4-wifi")]
macro_rules! pins {
    ($p:ident) => {
        ($p.SPI0, $p.P102, $p.P411, $p.P410, $p.P103)
    };
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());

    let mut config = spi::Config::default();
    // Configure a 250 kHz bitrate, slow enough to play nice with the cheap Saleae clones.
    config.bit_rate = 250_000;

    let (spi, sck, mosi, miso, cs) = pins!(p);

    // Configure an SPI driver with:
    // - 16-bit words because that's what the BMI-160 uses
    // - hardware CS control because why not
    let spi = Spi::<_, u16, _>::new_blocking(spi, sck, mosi, miso, cs, config);
    let mut sensor = BoschBmi160::new_spi(spi);

    info!("Chip ID: 0x{:02X}", sensor.chipid());

    sensor.set_gyro_mode(GyroPowerMode::Normal);

    loop {
        let temp = sensor.temperature();
        info!("Temp: {}", temp);
        Timer::after_millis(500).await;
    }
}
