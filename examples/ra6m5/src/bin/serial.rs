//! `serial` demonstrates SCI comms
//!
//! Writes a string to an SCI device in a loop.
//! On the EK-RA6M5 this uses SCI0 as available on the PMOD2 connector.

#![no_std]
#![no_main]

use assign_resources::assign_resources;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_hal_internal::Peri;
use embassy_time::Timer;
use embedded_io_async::{Read, Write};
use panic_probe as _;
use ra_hal::{
    bind_interrupts,
    clock::ClockConfig,
    peripherals::{self, SCI0},
    uart::{BufferedUart, Config, RxInterruptHandler, TeInterruptHandler, TxInterruptHandler},
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

#[cfg(feature = "ek-ra6m5")]
bind_interrupts!(struct Irqs {
    IEL2 => RxInterruptHandler<SCI0>;
    IEL3 => TxInterruptHandler<SCI0>;
    IEL4 => TeInterruptHandler<SCI0>;
});

#[cfg(not(feature = "ek-ra6m5"))]
compile_error!(
    "Ensure the pin and timer assignments are correct for your board before continuing."
);

// Define the pins we want on the RA6M5 Eval Kit
#[cfg(feature = "ek-ra6m5")]
assign_resources! {
    uart: UartResources {
        uart: SCI0,
        tx: P411,
        rx: P410,
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());
    let r = split_resources!(p);

    // Using small buffers to ensure we test the multiple iterations path
    let tx_buf = &mut [0u8; 8];
    let rx_buf = &mut [0u8; 8];

    let mut config = Config::default();
    config.baud_rate = 300;

    let mut uart = BufferedUart::new(
        r.uart.uart,
        r.uart.tx,
        tx_buf,
        r.uart.rx,
        rx_buf,
        Irqs,
        config,
    );

    loop {
        uart.blocking_write(b"All work and no play makes Jack a dull boy.\r\n");
        Timer::after_millis(333).await;
    }
}
