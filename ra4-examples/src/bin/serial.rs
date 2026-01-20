//! Serial demonstrates SCI comms

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use ra4_hal::{bind_interrupts, peripherals, print_clock_config};
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

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra4_hal::init();

    print_clock_config();

    let rx_buf = &mut [0u8; 128];

    let mut sci = ra4_hal::uart::Uart::new(p.SCI1, p.P501, p.P502, Irqs);
    ra4_hal::uart::Uart::<ra4_hal::peripherals::SCI0>::init_buffers(Some(rx_buf));

    sci.set_speed(115200);

    let cmd = b"SOFTRESETWIFI";
    sci.blocking_write(b"AT+");
    sci.blocking_write(cmd);
    sci.blocking_write(b"\r\n");

    let mut buf = [0_u8; 64];
    sci.blocking_read(&mut buf[0..cmd.len() + 5]);
    let readable = str::from_utf8(&buf).unwrap();
    error!("{:?}", readable);

    loop {
        Timer::after_millis(250 * 2).await;
    }
}
