//! Serial demonstrates SCI comms

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use ra4_hal::{bind_interrupts, peripherals, print_clock_config, uart::Uart};
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
    IEL2 => ra4_hal::uart::RxInterruptHandler<peripherals::SCI1>;
});

fn query<I: ra4_hal::uart::Instance>(uart: &mut Uart<I>, cmd: &[u8]) {
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

    let rx_buf = &mut [0u8; 128];

    let mut uart = Uart::new(p.SCI1, p.P501, p.P502, Irqs);
    uart.init_buffers(Some(rx_buf));

    uart.set_speed(115200);

    query(&mut uart, b"SOFTRESETWIFI");
    Timer::after_millis(125).await;
    query(&mut uart, b"FWVERSION?");
    query(&mut uart, b"GETSTATUS?");
    query(&mut uart, b"WIFISCAN");
    query(&mut uart, b"GETSTATUS?");

    loop {
        Timer::after_millis(250 * 2).await;
    }
}
