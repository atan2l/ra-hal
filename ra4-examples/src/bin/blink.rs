//! Blink, hello world example

#![no_std]
#![no_main]
#![warn(missing_docs)]

use cortex_m::asm;
#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
// use embassy_time::Timer;
use panic_probe as _;
use ra4_hal::{
    interrupt,
    interrupt::InterruptExt as _,
    pac::{
        self as pac,
        gpt32::{
            regs::{Gtdnsr, Gtupsr},
            vals::{Mode, Prkey, Tpcs, Ud},
        },
        icu::vals::Iels,
        system::{
            regs::Sckdivcr,
            vals::{Cksel, Fck, Hcfrq1, Hcstp, Ick, Opcm, Pcka, Pckb, Pckc, Pckd, Prc0},
        },
    },
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

fn print_clock_config(config: Sckdivcr) {
    let hoco_freq = 48;
    let ick_freq = match config.ick() {
        Ick::DIV_1 => hoco_freq,
        Ick::DIV_2 => hoco_freq / 2,
        Ick::DIV_4 => hoco_freq / 4,
        Ick::DIV_8 => hoco_freq / 8,
        Ick::DIV_16 => hoco_freq / 16,
        Ick::DIV_32 => hoco_freq / 32,
        Ick::DIV_64 => hoco_freq / 64,
        Ick::_RESERVED_7 => unimplemented!("Invalid sckdivcr.ick"),
    };
    let fck_freq = match config.fck() {
        Fck::DIV_1 => hoco_freq,
        Fck::DIV_2 => hoco_freq / 2,
        Fck::DIV_4 => hoco_freq / 4,
        Fck::DIV_8 => hoco_freq / 8,
        Fck::DIV_16 => hoco_freq / 16,
        Fck::DIV_32 => hoco_freq / 32,
        Fck::DIV_64 => hoco_freq / 64,
        Fck::_RESERVED_7 => unimplemented!("Invalid sckdivcr.fck"),
    };
    let pck_a = match config.pcka() {
        Pcka::DIV_1 => hoco_freq,
        Pcka::DIV_2 => hoco_freq / 2,
        Pcka::DIV_4 => hoco_freq / 4,
        Pcka::DIV_8 => hoco_freq / 8,
        Pcka::DIV_16 => hoco_freq / 16,
        Pcka::DIV_32 => hoco_freq / 32,
        Pcka::DIV_64 => hoco_freq / 64,
        Pcka::_RESERVED_7 => unimplemented!("Invalid sckdivcr.pcka"),
    };
    let pck_b = match config.pckb() {
        Pckb::DIV_1 => hoco_freq,
        Pckb::DIV_2 => hoco_freq / 2,
        Pckb::DIV_4 => hoco_freq / 4,
        Pckb::DIV_8 => hoco_freq / 8,
        Pckb::DIV_16 => hoco_freq / 16,
        Pckb::DIV_32 => hoco_freq / 32,
        Pckb::DIV_64 => hoco_freq / 64,
        Pckb::_RESERVED_7 => unimplemented!("Invalid sckdivcr.pckb"),
    };
    let pck_c = match config.pckc() {
        Pckc::DIV_1 => hoco_freq,
        Pckc::DIV_2 => hoco_freq / 2,
        Pckc::DIV_4 => hoco_freq / 4,
        Pckc::DIV_8 => hoco_freq / 8,
        Pckc::DIV_16 => hoco_freq / 16,
        Pckc::DIV_32 => hoco_freq / 32,
        Pckc::DIV_64 => hoco_freq / 64,
        Pckc::_RESERVED_7 => unimplemented!("Invalid sckdivcr.pckc"),
    };
    let pck_d = match config.pckd() {
        Pckd::DIV_1 => hoco_freq,
        Pckd::DIV_2 => hoco_freq / 2,
        Pckd::DIV_4 => hoco_freq / 4,
        Pckd::DIV_8 => hoco_freq / 8,
        Pckd::DIV_16 => hoco_freq / 16,
        Pckd::DIV_32 => hoco_freq / 32,
        Pckd::DIV_64 => hoco_freq / 64,
        Pckd::_RESERVED_7 => unimplemented!("Invalid sckdivcr.pckd"),
    };
    debug!(
        "ICK: {} MHz, FCK: {} MHz, PCKA: {} MHz, PCKB: {} MHz, PCKC: {} MHz, PCKD: {} MHz",
        ick_freq, fck_freq, pck_a, pck_b, pck_c, pck_d
    );
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Starting board init");

    // // #define BSP_CLOCK_CFG_MAIN_OSC_WAIT (9)
    // // #define BSP_LOCO_HZ                 (32768)
    // // #define BSP_MOCO_HZ                 (8000000)
    // // #define BSP_SUB_CLOCK_HZ            (32768)
    // // #define BSP_MCU_VBATT_SUPPORT       (1)

    let system = pac::SYSTEM;

    trace!("HOCO WaitState: {}", system.hocowtcr().read());
    trace!("HOCO Status: {}", system.hococr().read());

    system.protected_write(|| {
        let hoco_freq = system.hococr2().read().hcfrqw();
        if hoco_freq != Hcfrq1::_48mhz {
            warn!("Unexpected HOCO frequency: {}", hoco_freq);
            system.hococr2().write(|w| {
                w.set_hcfrqw(Hcfrq1::_48mhz);
            });
        };

        info!("HOCO Frequency: {}", system.hococr2().read().hcfrqw());
        // let hococr2_ptr: *mut u8 = 0x4001E037 as _;
        // let val: u8 = unsafe { (hococr2_ptr as *mut u8).read_volatile() };
        // if val != (0b100 << 3) {
        //     warn!("Unexpected HOCO frequency: {:08b}", val);
        //     unsafe { (hococr2_ptr).write_volatile(0b100 << 3) };
        //     let val: u8 = unsafe { (hococr2_ptr as *mut u8).read_volatile() };
        //     defmt::warn!("HOCO Frequency: {:08b}", val);
        // }

        if system.hococr().read().hcstp() != Hcstp::Start {
            warn!("HOCO not running, attempt to start.");
            system.hococr().write(|w| {
                w.set_hcstp(Hcstp::Start);
            });
        }

        debug!("HOCO Status: {}", system.hococr().read().hcstp());

        // High speed mode needed for iclk > 32 MHz
        trace!("Setting high speed mode on");
        system.opccr().write(|w| {
            w.set_opcm(Opcm::HighSpeed);
        });

        while system.opccr().read().opcmtsf() {
            asm::nop();
        }

        trace!("Setting memwait to 1");
        system.memwait().write(|w| w.set_memwait(true));

        system.sckscr().write(|w| {
            // Use HOCO which we set to 48 MHz
            w.set_cksel(Cksel::Hoco);
        });
        debug!("SYSTEM ClkSource: {}", system.sckscr().read().cksel());

        system.sckdivcr().modify(|w| {
            // ICLK = HOCO/1 = 48 MHz
            w.set_ick(Ick::DIV_1);

            // FCLK max 32 MHz, ICLK/2 = 24 MHz
            w.set_fck(Fck::DIV_2);

            // PCLKD max 64 MHz, ICLK/1 = 48 MHz
            w.set_pckd(Pckd::DIV_1);

            // PCLKC max 64 MHz, ICLK/1 = 48 MHz
            w.set_pckc(Pckc::DIV_1);

            // PCLKB max 32 MHz, ICLK/2 = 24 MHz
            w.set_pckb(Pckb::DIV_2);

            // PCKLA max 48 MHz, ICLK/1 = 48 MHz
            w.set_pcka(Pcka::DIV_1);
        });
    });

    print_clock_config(system.sckdivcr().read());

    info!("Finished board init");

    let _p = ra4_hal::init();

    loop {
        error!("Here");
        Timer::after_millis(500).await;
    }
}
