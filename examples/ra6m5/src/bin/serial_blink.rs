//! `serial` demonstrates SCI comms
//!
//! Writes a string to an SCI device in a loop while blinking an LED in a separate task.
//! On the EK-RA6M5 this uses SCI0 as available on the PMOD2 connector.

#![no_std]
#![no_main]

use assign_resources::assign_resources;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_io_async::{Read, Write};
use panic_probe as _;
use ra_hal::{
    Peri, bind_interrupts,
    clock::ClockConfig,
    gpio::{AnyPin, DriveCapacity, Level, Output},
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
        peri: SCI0,
        tx: P411,
        rx: P410,
    }
    blink: BlinkResources {
        led: P006,
    }
}

#[embassy_executor::task]
async fn blink(pin: Peri<'static, AnyPin>) {
    let mut led = Output::new_basic(pin, Level::Low, DriveCapacity::Low);

    loop {
        led.toggle();
        Timer::after_millis(333).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());
    let r = split_resources!(p);

    spawner.spawn(blink(r.blink.led.into()).unwrap());

    let tx_buf = &mut [0u8; 48];
    let rx_buf = &mut [0u8; 48];

    let mut uart_config = Config::default();
    uart_config.baud_rate = 300;

    let mut uart = BufferedUart::new(
        r.uart.peri,
        r.uart.tx,
        tx_buf,
        r.uart.rx,
        rx_buf,
        Irqs,
        uart_config,
    );

    loop {
        uart.write(b"All work and no play makes Jack a dull boy.\r\n")
            .await
            .unwrap();
        Timer::after_millis(333).await;
    }
}
