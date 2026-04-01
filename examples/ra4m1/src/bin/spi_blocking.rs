//! `spi` SPI example that writes an array out in a loop using the blocking API.

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, block_for};
use panic_probe as _;
use ra_hal::{
    bind_interrupts,
    clock::ClockConfig,
    dmac::DmacInterruptHandler,
    peripherals::{DMAC0, DMAC1, SPI0},
    spi::{self, Spi, TeInterruptHandler},
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

bind_interrupts!(struct Irqs {
    // Tx DMAC channel
    IEL3 => DmacInterruptHandler<DMAC0>;
    // Rx DMAC channel
    IEL4 => DmacInterruptHandler<DMAC1>;
    // SPI transfer finished
    IEL5 => TeInterruptHandler<SPI0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());

    let clocks = ra_hal::clock::clock_status();

    let mut config = spi::Config::default();
    config.bit_rate = 24_000_000;

    let (spi, sck, mosi, miso, ss) = pins!(p);

    let mut spi = Spi::new_blocking(spi, sck, mosi, miso, ss, config);

    // const SIZE: usize = 228;
    // let output = [0xA5_u8; SIZE];
    // let mut input = [0x5A_u8; SIZE];

    const SIZE: usize = 228 / 2;
    let output = [0xA5B8_u16; SIZE];
    let mut input = [0xE35A_u16; SIZE];

    // const SIZE: usize = 228 / 4;
    // let output = [0xA5A55A5A_u32; SIZE];
    // let mut input = [0x5A5A5A5A_u32; SIZE];

    spi.blocking_setup();

    loop {
        let now = Instant::now();
        spi.blocking_transfer(&mut input, &output).unwrap();
        let elapsed = now.elapsed().as_ticks() as f64;
        info!(
            "Elapsed: {} µs",
            (elapsed / clocks.system.to_Hz() as f64) * 1_000_000.0
        );
        info!("Read: {:08X}", input[SIZE.saturating_sub(5)..]);
        block_for(Duration::from_millis(300));
    }
}
