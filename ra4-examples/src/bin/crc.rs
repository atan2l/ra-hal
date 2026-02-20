//! `crc` demonstrates use of the CRC calculator

#![no_std]
#![no_main]
#![warn(missing_docs)]

use cortex_m::asm;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;
use ra4_hal::crc::{Config as CrcConfig, Crc, Polynomial};
#[allow(unused)]
use ra4_hal::{assert_eq, debug, error, info, trace, warn};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra4_hal::init();

    let data = b"123456789";

    let mut crc = Crc::new(
        p.CRC,
        CrcConfig {
            polynomial: Polynomial::Crc16,
            reverse: true,
            ..Default::default()
        },
    );

    let output = crc.feed_bytes(data);
    assert_eq!(output, 0xbb3d);
    info!("Crc16 ARC Passed");

    crc.set_config(CrcConfig {
        polynomial: Polynomial::Crc16,
        seed: 0xffff,
        ..Default::default()
    });
    let output = crc.feed_bytes(data);
    assert_eq!(output, 0xaee7);
    info!("Crc16 CMS Passed");

    crc.set_config(CrcConfig {
        polynomial: Polynomial::Crc16,
        reverse: false,
        seed: 0x800d,
        ..Default::default()
    });
    let output = crc.feed_bytes(data);
    assert_eq!(output, 0x9ecf);
    info!("Crc16 DDS-110 Passed");

    crc.set_config(CrcConfig {
        polynomial: Polynomial::Crc16,
        reverse: true,
        reflect_output: true,
        seed: 0x0000,
    });
    let output = crc.feed_bytes(data);
    assert_eq!(output, 0x44c2);
    info!("Crc16 MAXIM-DOW Passed");

    crc.set_config(CrcConfig {
        polynomial: Polynomial::Crc16,
        reverse: true,
        seed: 0xffff,
        ..Default::default()
    });
    let output = crc.feed_bytes(data);
    assert_eq!(output, 0x4b37);
    info!("Crc16 MODBUS Passed");

    crc.set_config(CrcConfig {
        polynomial: Polynomial::CrcCcit,
        reverse: true,
        seed: 0x0000,
        ..Default::default()
    });
    let output = crc.feed_bytes(data);
    assert_eq!(output, 0x2189);
    info!("Crc16 KERMIT Passed");

    // The CRC accelerator only works on 32-bit values
    // <https://www.crccalc.com/>
    let data = b"123456789012";

    crc.set_config(CrcConfig {
        polynomial: Polynomial::Crc32,
        seed: 0xffff_ffff,
        reflect_output: true,
        ..Default::default()
    });
    let output = crc.feed_bytes(data);
    assert_eq!(output, 0x481f_7de5);
    info!("Crc32 BZIP2 Passed");

    crc.set_config(CrcConfig {
        polynomial: Polynomial::Crc32,
        seed: 0x0000_0000,
        reflect_output: true,
        ..Default::default()
    });
    let output = crc.feed_bytes(data);
    assert_eq!(output, 0x4183_29c4);
    info!("Crc32 CKSUM/POSIX Passed");

    crc.set_config(CrcConfig {
        polynomial: Polynomial::Crc32,
        seed: 0xffff_ffff,
        reverse: true,
        reflect_output: true,
    });
    let output = crc.feed_bytes(data);
    assert_eq!(output, 0x5d34_eb96);
    info!("Crc32 ISO-HDLC Passed");

    crc.set_config(CrcConfig {
        polynomial: Polynomial::Crc32,
        seed: 0xffff_ffff,
        reverse: true,
        ..Default::default()
    });
    let output = crc.feed_bytes(data);
    assert_eq!(output, 0xA2CB_1469);
    info!("Crc32 JAMCRC Passed");

    crc.set_config(CrcConfig {
        polynomial: Polynomial::Crc32,
        seed: 0xffff_ffff,
        ..Default::default()
    });
    let output = crc.feed_bytes(data);
    assert_eq!(output, 0xb7e0_821a);
    info!("Crc32 MPEG-2 Passed");

    crc.set_config(CrcConfig {
        polynomial: Polynomial::Crc32C,
        seed: 0xffff_ffff,
        reverse: true,
        reflect_output: true,
    });
    let output = crc.feed_bytes(data);
    assert_eq!(output, 0xd75d_fdfb);
    info!("Crc32 BASE-91C Passed");

    loop {
        asm::nop();
    }
}
