//! `serial` demonstrates SCI comms
//!
//! Writes a string to an SCI device in a loop.
//! On the EK-RA4L1 this uses SCI3 as available on the PMOD2 connector.

#![no_std]
#![no_main]

use assign_resources::assign_resources;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_io_async::Write;
use panic_probe as _;
use ra_hal::{
    Peri, bind_interrupts,
    clock::ClockConfig,
    peripherals::{self, SCI3},
    uart::{BufferedUart, Config, RxInterruptHandler, TeInterruptHandler, TxInterruptHandler},
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

#[cfg(not(feature = "ek-ra4l1"))]
compile_error!(
    "Ensure the pin and timer assignments are correct for your board before continuing."
);

// Define the pins we want on the RA4L1 Eval Kit
#[cfg(feature = "ek-ra4l1")]
assign_resources! {
    uart: UartResources {
        peri: SCI3,
        tx: P409,
        rx: P408,
    }
}

#[cfg(feature = "ek-ra4l1")]
bind_interrupts!(struct Irqs {
    IEL2 => RxInterruptHandler<SCI3>;
    IEL3 => TxInterruptHandler<SCI3>;
    IEL4 => TeInterruptHandler<SCI3>;
});

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
        r.uart.peri,
        r.uart.tx,
        tx_buf,
        r.uart.rx,
        rx_buf,
        Irqs,
        config,
    );

    for _ in 0..100 {
        uart.blocking_write(b"All work and no play makes Jack a dull boy.\r\n");
        Timer::after_millis(333).await;
    }

    loop {
        uart.write(b"All work and no play makes Jack a dull boy.\r\n").await.unwrap();
        Timer::after_millis(333).await;
        
    }
}
