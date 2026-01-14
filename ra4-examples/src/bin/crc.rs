//! Crc, demonstrates use of the CRC calculator

#![no_std]
#![no_main]
#![warn(missing_docs)]

use cortex_m::asm;
#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;
use ra4_hal::{
    crc::{Config as CrcConfig, Crc, Endian, Polynomial},
    print_clock_config,
};
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

// There are reasons for this.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".sec_mpu")]
pub static SEC_MPU: [u32; 13] = [
    0x00fffffc, 0x00ffffff, 0x00fffffc, 0x00ffffff, 0x00fffffc, 0x00ffffff, 0x200ffffc, 0x200fffff,
    0x407ffffc, 0x407fffff, 0x400dfffc, 0x400dffff, 0xffffffff,
];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra4_hal::init();

    print_clock_config();

    let data = b"123456789";

    let mut crc = Crc::new(
        p.CRC,
        CrcConfig {
            polynomial: Polynomial::Crc16,
            ..Default::default()
        },
    );

    let output = crc.feed_bytes(data);
    defmt::assert_eq!(output, 0xbb3d);
    info!("Crc16 Passed");

    crc.set_config(CrcConfig {
        polynomial: Polynomial::Crc16,
        endian: Endian::Big,
        seed: 0x800d,
        ..Default::default()
    });
    let output = crc.feed_bytes(data);
    defmt::assert_eq!(output, 0x9ecf);
    info!("Crc16 DDS-110 Passed");

    loop {
        asm::nop();
    }
}
