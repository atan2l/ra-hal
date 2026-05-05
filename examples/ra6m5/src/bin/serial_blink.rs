//! `serial` demonstrates SCI comms
//!
//! Writes a string to an SCI device in a loop while blinking an LED in a separate task.
//! On the EK-RA6M5 this uses SCI0 as available on the PMOD2 connector.

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_time::Timer;
use embedded_io_async::{Read, Write};
use panic_probe as _;
use ra_hal::{
    bind_interrupts,
    clock::ClockConfig,
    gpio::{AnyPin, DriveCapacity, Level, Output},
    peripherals::SCI0,
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

#[cfg(feature = "ek-ra6m5")]
macro_rules! peripherals {
    ($p:ident) => {
        ($p.P006, $p.SCI0, $p.P411, $p.P410)
    };
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
    let (led, sci, tx_pin, rx_pin) = peripherals!(p);

    spawner.spawn(blink(led.into()).unwrap());

    let tx_buf = &mut [0u8; 48];
    let rx_buf = &mut [0u8; 48];

    let mut uart_config = Config::default();
    uart_config.baud_rate = 300;

    let mut uart = BufferedUart::new(sci, tx_pin, tx_buf, rx_pin, rx_buf, Irqs, uart_config);

    loop {
        uart.write(b"All work and no play makes Jack a dull boy.\r\n")
            .await
            .unwrap();
        Timer::after_millis(333).await;
    }
}
