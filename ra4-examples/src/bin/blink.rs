//! Blink, hello world example

#![no_std]
#![no_main]
#![warn(missing_docs)]

use cortex_m::asm;
#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
use defmt_rtt as _;
use embassy_executor::Spawner;
// use embassy_time::Timer;
use panic_probe as _;
use ra4_hal::{ofs0, ofs1};

/// Option Function Select Register 0
/// Accepts either:
/// - a series of configuration values
/// - the literal `ArduinoCore` which emulates the Arduino defaults
// #[unsafe(no_mangle)]
// #[unsafe(link_section = ".ofs0")]
// pub static OFS0: u32 = ofs0!(ArduinoCore);

/// Option Function Select Register 1
/// Accepts either:
/// - a series of configuration values
/// - the literal `ArduinoCore` which emulates the Arduino defaults
// #[unsafe(no_mangle)]
// #[unsafe(link_section = ".ofs1")]
// pub static OFS1: u32 = ofs1!(ArduinoCore);

// #[unsafe(no_mangle)]
// #[unsafe(link_section = ".sec_mpu")]
// pub static SEC_MPU: [u32; 13] = [
//     0x00fffffc, 0x00ffffff, 0x00fffffc, 0x00ffffff, 0x00fffffc, 0x00ffffff, 0x200ffffc, 0x200fffff,
//     0x407ffffc, 0x407fffff, 0x400dfffc, 0x400dffff, 0xffffffff,
// ];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::error!("Starting main");

    // // #define BSP_CLOCK_CFG_MAIN_OSC_WAIT (9)
    // // #define BSP_LOCO_HZ                 (32768)
    // // #define BSP_MOCO_HZ                 (8000000)
    // // #define BSP_SUB_CLOCK_HZ            (32768)
    // // #define BSP_MCU_VBATT_SUPPORT       (1)

    let system = pac::SYSTEM;
    system.sckscr().write(|w| {
        // Use HOCO which we set to 48 MHz
        w.set_cksel(Cksel::_000);
    });

    system.sckdivcr().write(|w| {
        // ICLK = HOCO/1 = 48 MHz
        w.set_ick(Ick::_000);

        // FCLK max 32 MHz, ICLK/2 = 24 MHz
        w.set_fck(Fck::_001);

        // PCLKD max 64 MHz, ICLK/1 = 48 MHz
        w.set_pckd(Pckd::_000);

        // PCLKC max 64 MHz, ICLK/1 = 48 MHz
        w.set_pckc(Pckc::_000);

        // PCLKB max 32 MHz, ICLK/2 = 24 MHz
        w.set_pckb(Pckb::_001);

        // PCKLA max 48 MHz, ICLK/1 = 48 MHz
        w.set_pcka(Pcka::_000);
    });

    defmt::error!("DONE WITH INIT!");

    loop {
        asm::nop();
    }
}
