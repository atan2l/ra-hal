//! `serial` demonstrates SCI comms
//!
//! This writes AT commands to the onboard ESP32-S3 using different
//! APIs and checks for a result.

#![no_std]
#![no_main]

use cortex_m::asm;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_io_async::{Read, Write};
use panic_probe as _;
use ra_hal::{
    bind_interrupts,
    clock::ClockConfig,
    peripherals::SCI1,
    uart::{BufferedUart, Config, RxInterruptHandler, TeInterruptHandler, TxInterruptHandler},
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

#[cfg(not(feature = "uno-r4-wifi"))]
compile_error!("This example is only designed to work with the Arduino Uno R4 WiFi.");

bind_interrupts!(struct Irqs {
    IEL2 => RxInterruptHandler<SCI1>;
    IEL3 => TxInterruptHandler<SCI1>;
    IEL4 => TeInterruptHandler<SCI1>;
});

fn query<I: ra_hal::uart::Instance>(uart: &mut BufferedUart<I>, cmd: &[u8]) {
    uart.blocking_write(b"AT+");
    uart.blocking_write(cmd);
    uart.blocking_write(b"\r\n");

    let mut buf = [0_u8; 64];

    loop {
        let line_len = uart.read_line(&mut buf);
        let line = &buf[..line_len];
        let line = str::from_utf8(line).unwrap();
        info!("{}", line);
        if line == "OK" || line == "ERROR" {
            return;
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());

    // Using small buffers to ensure we test the multiple iterations path
    let tx_buf = &mut [0u8; 8];
    let rx_buf = &mut [0u8; 8];

    let mut config = Config::default();
    // The ESP32 communicates at 115,200 baud with the default Arduino firmware
    config.baud_rate = 115200;

    let mut uart = BufferedUart::new(p.SCI1, p.P501, tx_buf, p.P502, rx_buf, Irqs, config);

    // Try a command with multiple small blocking writes
    query(&mut uart, b"SOFTRESETWIFI");
    Timer::after_millis(125).await;

    // Try a command with one large blocking write
    info!("Sending SOFTRESETWIFI");
    uart.blocking_write(b"AT+SOFTRESETWIFI\r\n");
    let mut in_buf = [0_u8; 22];
    uart.read_exact(&mut in_buf).await.unwrap();
    let s = str::from_utf8(&in_buf).unwrap();
    warn!("{:?}", s);

    // Clear the RX buffer just in case.
    uart.drain();

    // Write this with the async API
    info!("Sending ASYNC SOFTRESETWIFI");
    uart.write(b"AT+SOFTRESETWIFI\r\n").await.unwrap();
    info!("Reading?");
    let mut in_buf = [0_u8; 22];
    uart.read_exact(&mut in_buf).await.unwrap();
    let s = str::from_utf8(&in_buf).unwrap();
    warn!("{:?}", s);

    uart.drain();

    // Lots o data to read over a long time.
    info!("WiFi scan");
    query(&mut uart, b"WIFISCAN");

    // And make sure we still work
    info!("Sending FWVERSION");
    uart.write(b"AT+FWVERSION?\r\n").await.unwrap();
    let mut in_buf = [0_u8; 23];
    uart.read_exact(&mut in_buf).await.unwrap();
    let s = str::from_utf8(&in_buf).unwrap();
    warn!("{:?}", s);

    loop {
        asm::nop();
    }
}
