//! `spi` SPI example that reads the CHIPID from a Bosch BMI-160 sensor.

#![no_std]
#![no_main]
#![warn(missing_docs)]

use cortex_m::asm;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;
use ra4_hal::spi::{self, Spi};
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};

// Define the pins we want on the R4 Minima
// Note: this will not work without the "swd-as-gpio" feature.
#[cfg(feature = "uno-r4-minima")]
macro_rules! pins {
    ($p:ident) => {
        ($p.SPI1, $p.P111, $p.P110, $p.P109, $p.P112)
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
    let p = ra4_hal::init();

    let mut config = spi::Config::default();
    config.bit_rate = 12_000_000;

    let (spi, sck, mosi, miso, ss) = pins!(p);

    let mut spi = Spi::<_, u16>::new_with_ss(spi, sck, mosi, miso, ss, config);

    let reg = 0x00_u8;
    spi.write(&[(reg as u16) << 1 | 0x01]);

    let mut in_buf = [0_u16; 1];
    spi.read(&mut in_buf);
    info!("Got: 0x{:02X}, expected: 0xD1", in_buf[0] & 0xFF);

    loop {
        asm::nop();
    }
}
