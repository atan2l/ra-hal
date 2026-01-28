//! Serial demonstrates SCI comms
//!
//! This writes AT commands to the onboard ESP32-S3 using different
//! APIs and checks for a result.

#![no_std]
#![no_main]
#![warn(missing_docs)]

use cortex_m::asm;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_io_async::{Read, Write};
use panic_probe as _;
use ra4_hal::{
    bind_interrupts,
    peripherals::SCI1,
    print_clock_config,
    uart::{BufferedUart, RxInterruptHandler, TeInterruptHandler, TxInterruptHandler},
};
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};
use ra4_hal::{ofs0, ofs1};

/// Option Function Select Register 0
/// Accepts either:
/// - a series of configuration values
/// - the literal `ArduinoCore` which emulates the Arduino defaults
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs0")]
pub static OFS0: u32 = ofs0!(ArduinoCore);

/// Option Function Select Register 1
/// Accepts either:
/// - a series of configuration values
/// - the literal `ArduinoCore` which emulates the Arduino defaults
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs1")]
pub static OFS1: u32 = ofs1!(ArduinoCore);

/// Configures the Security MPU.  See the reference manual for more details.
/// Setting all bits to 1 would also work.  Setting all bits to 0 is a good way to brick your board.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".sec_mpu")]
pub static SEC_MPU: [u32; 13] = [
    0x00fffffc, 0x00ffffff, 0x00fffffc, 0x00ffffff, 0x00fffffc, 0x00ffffff, 0x200ffffc, 0x200fffff,
    0x407ffffc, 0x407fffff, 0x400dfffc, 0x400dffff, 0xffffffff,
];

bind_interrupts!(struct Irqs {
    IEL2 => RxInterruptHandler<SCI1>;
    IEL3 => TxInterruptHandler<SCI1>;
    IEL4 => TeInterruptHandler<SCI1>;
});

fn query<I: ra4_hal::uart::Instance>(uart: &mut BufferedUart<I>, cmd: &[u8]) {
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
    let p = ra4_hal::init();

    print_clock_config();

    // Using small buffers to ensure we test the multiple iterations  path
    let tx_buf = &mut [0u8; 8];
    let rx_buf = &mut [0u8; 8];

    let mut uart = BufferedUart::new(p.SCI1, p.P501, tx_buf, p.P502, rx_buf, Irqs);

    // The ESP32 communicates at 115,200 baud with the default Arduino firmware
    uart.set_baudrate(115200);

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
