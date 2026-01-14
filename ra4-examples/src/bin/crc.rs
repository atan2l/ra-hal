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
    pac::{
        self as pac,
        system::vals::{Cksel, Fck, Hcfrq1, Hcstp, Ick, Opcm, Pcka, Pckb, Pckc, Pckd, Prc0},
    },
    print_clock_config,
    write_protect::WriteProtect,
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
    info!("Starting board init");

    let system = pac::SYSTEM;

    system.protected_write(|| {
        let hoco_freq = system.hococr2().read().hcfrqw();
        if hoco_freq != Hcfrq1::_48mhz {
            warn!("Unexpected HOCO frequency: {}", hoco_freq);
            system.hococr2().write(|w| {
                w.set_hcfrqw(Hcfrq1::_48mhz);
            });
        };

        if system.hococr().read().hcstp() != Hcstp::Start {
            warn!("HOCO not running, attempt to start.");
            system.hococr().write(|w| {
                w.set_hcstp(Hcstp::Start);
            });
        }

        // High speed mode needed for iclk > 32 MHz
        system.opccr().write(|w| {
            w.set_opcm(Opcm::HighSpeed);
        });

        while system.opccr().read().opcmtsf() {
            asm::nop();
        }

        system.memwait().write(|w| w.set_memwait(true));

        system.sckscr().write(|w| {
            // Use HOCO which we set to 48 MHz
            w.set_cksel(Cksel::Hoco);
        });

        system.sckdivcr().modify(|w| {
            w.set_ick(Ick::DIV_1);
            w.set_fck(Fck::DIV_2);
            w.set_pckd(Pckd::DIV_1);
            w.set_pckc(Pckc::DIV_1);
            w.set_pckb(Pckb::DIV_2);
            w.set_pcka(Pcka::DIV_1);
        });
    });

    print_clock_config(system.sckdivcr().read());

    info!("Finished board init");

    let p = ra4_hal::init();

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
