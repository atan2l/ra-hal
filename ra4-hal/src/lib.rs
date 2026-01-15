#![no_std]

pub mod adc;
pub mod crc;
pub mod ofs0;
pub mod ofs1;
pub mod sce5;
#[cfg(feature = "time-driver")]
pub mod time_driver;
pub mod uart;
pub mod write_protect;

use cfg_if::cfg_if;
use cortex_m::asm;
#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
#[cfg(feature = "unstable-pac")]
pub use ra4m1_ctpac as pac;
#[cfg(not(feature = "unstable-pac"))]
pub(crate) use ra4m1_ctpac as pac;
use ra4m1_ctpac::system::vals::{Cksel, Fck, Hcfrq1, Hcstp, Ick, Opcm, Pcka, Pckb, Pckc, Pckd};

use crate::write_protect::WriteProtect as _;

pub fn init() -> Peripherals {
    // #define BSP_CLOCK_CFG_MAIN_OSC_WAIT (9)
    // #define BSP_LOCO_HZ                 (32768)
    // #define BSP_MOCO_HZ                 (8000000)
    // #define BSP_SUB_CLOCK_HZ            (32768)
    // #define BSP_MCU_VBATT_SUPPORT       (1)

    critical_section::with(|cs| {
        debug!("Starting board init");

        let fmifrt_base = pac::FMIFRT_BASE;
        // sanity check
        defmt::assert_eq!(0x0100_3C00, fmifrt_base.base().read().base());

        print_mcu_info();

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

            if system.hococr().read().hcstp() != Hcstp::Start {
                warn!("HOCO not running, attempt to start.");
                system.hococr().write(|w| {
                    w.set_hcstp(Hcstp::Start);
                });
            }

            info!("HOCO Frequency: {}", system.hococr2().read().hcfrqw());
            // let hococr2_ptr: *mut u8 = 0x4001E037 as _;
            // let val: u8 = unsafe { (hococr2_ptr as *mut u8).read_volatile() };
            // if val != (0b100 << 3) {
            //     warn!("Unexpected HOCO frequency: {:08b}", val);
            //     unsafe { (hococr2_ptr).write_volatile(0b100 << 3) };
            //     let val: u8 = unsafe { (hococr2_ptr as *mut u8).read_volatile() };
            //     defmt::warn!("HOCO Frequency: {:08b}", val);
            // }

            debug!("SYSTEM: HOCOStatus: {}", system.hococr().read().hcstp());

            // High speed mode needed for iclk > 32 MHz
            trace!("Setting high speed mode on");
            system.opccr().write(|w| {
                w.set_opcm(Opcm::HighSpeed);
            });

            while system.opccr().read().opcmtsf() {
                asm::nop();
            }

            // Wait states needed for > 32 MHz
            trace!("Setting memwait to 1");
            system.memwait().write(|w| w.set_memwait(true));

            system.sckscr().write(|w| {
                // Use HOCO which we set to 48 MHz
                w.set_cksel(Cksel::Hoco);
            });
            debug!("SYSTEM: ClkSource: {}", system.sckscr().read().cksel());

            system.sckdivcr().modify(|w| {
                w.set_ick(Ick::DIV_1);
                w.set_fck(Fck::DIV_2);
                w.set_pckd(Pckd::DIV_1);
                w.set_pckc(Pckc::DIV_1);
                w.set_pckb(Pckb::DIV_2);
                w.set_pcka(Pcka::DIV_1);
            });
        });

        debug!("Finished board init");

        let p = Peripherals::take_with_cs(cs);

        #[cfg(feature = "time-driver")]
        time_driver::init(crate::interrupt::Priority::P2);

        p
    })
}

pub fn print_mcu_info() {
    let fmifrt = pac::FMIFRT;

    let uid: [u32; 4] = [
        fmifrt.uidr(0).read().uid(),
        fmifrt.uidr(1).read().uid(),
        fmifrt.uidr(2).read().uid(),
        fmifrt.uidr(3).read().uid(),
    ];

    let mut part_number: [u8; 16] = [0; 16];

    part_number[0..4].copy_from_slice(&fmifrt.pnr(0).read().0.to_ne_bytes());
    part_number[4..8].copy_from_slice(&fmifrt.pnr(1).read().0.to_ne_bytes());
    part_number[8..12].copy_from_slice(&fmifrt.pnr(2).read().0.to_ne_bytes());
    part_number[12..16].copy_from_slice(&fmifrt.pnr(3).read().0.to_ne_bytes());

    let pn = core::str::from_utf8(&part_number).unwrap().trim();
    let ver = fmifrt.mcuver().read().mcuver();

    const PN_LEN: usize = 13;

    if pn.len() < PN_LEN {
        info!(
            "MCU: {} rev {:02X}, UID: {:08x}-{:08x}-{:08x}-{:08x}",
            pn, ver, uid[0], uid[1], uid[2], uid[3]
        );
        warn!("PN too short to identify");
    } else {
        let flash_size = match part_number[8] {
            b'9' => Some(128),
            b'B' => Some(256),
            b'C' => Some(384),
            b'D' => Some(512),
            b'E' => Some(768),
            b'F' => Some(1024),
            _ => todo!(),
        };

        // Check if the crate was configured correctly
        let pin_count = match &part_number[11..=12] {
            b"FB" | b"BM" => Some(144),
            b"FP" | b"LJ" => Some(100),
            b"NB" | b"BQ" | b"BB" | b"FM" => Some(64),
            b"NG" => Some(56),
            b"NE" | b"FL" => Some(48),
            b"NF" => Some(40),
            b"BC" => Some(36),
            b"NH" | b"FJ" => Some(32),
            suffix => {
                warn!("Unknown suffix: {}", suffix);
                None
            }
        };

        info!(
            "MCU: {} rev {:02X}, flash={} KB, UID: {:08x}-{:08x}-{:08x}-{:08x}",
            pn,
            ver,
            flash_size.unwrap_or(0),
            uid[0],
            uid[1],
            uid[2],
            uid[3]
        );

        match pin_count {
            Some(actual) => {
                cfg_if! {
                    if #[cfg(feature = "100pin")] {
                        let configured = 100;
                    } else if #[cfg(feature = "64pin")] {
                        let configured = 64;
                    } else if #[cfg(feature = "48pin")] {
                        let configured = 48;
                    } else {
                        let configured = 40;
                    }
                }

                if configured != actual {
                    warn!(
                        "May not behave as expected. HAL configured with {} pins, MCU has {} pins",
                        configured, actual
                    );
                }
            }
            None => warn!("Couldn't determine appropriate pin count"),
        }
    }
}

pub fn print_clock_config() {
    let system = pac::SYSTEM;
    let config = system.sckdivcr().read();
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
        "SYSTEM: ICK: {} MHz, FCK: {} MHz, PCKA: {} MHz, PCKB: {} MHz, PCKC: {} MHz, PCKD: {} MHz",
        ick_freq, fck_freq, pck_a, pck_b, pck_c, pck_d
    );
}

#[rustfmt::skip]
embassy_hal_internal::peripherals_definition!(
    ADC14,
    CRC,
    ICU,
    SCE5,
    SCI0,
    SCI1,
    SCI2,
    SCI9,
    GPT32_0,
    GPT32_1,

    P000,
    P001,
    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P002,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P003,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P004,
    #[cfg(feature = "100pin")]
    P005,
    #[cfg(feature = "100pin")]
    P006,
    #[cfg(feature = "100pin")]
    P007,
    #[cfg(feature = "100pin")]
    P008,
    P010,
    P011,
    P012,
    P013,
    P014,
    P015,

    P100,
    P101,
    P102,
    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P103,
    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P104,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P105,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P106,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P107,
    P108,
    P109,
    P110,
    P111,
    P112,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P113,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P114,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P115,

    P200,
    P201,
    #[cfg(feature = "100pin")]
    P202,
    #[cfg(feature = "100pin")]
    P203,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P204,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P205,
    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P206,
    P212,
    P213,
    P214,
    P215,

    P300,
    P301,
    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P302,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P303,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P304,
    #[cfg(feature = "100pin")]
    P305,
    #[cfg(feature = "100pin")]
    P306,
    #[cfg(feature = "100pin")]
    P307,

    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P400,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P401,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P402,
    #[cfg(feature = "100pin")]
    P403,
    #[cfg(feature = "100pin")]
    P404,
    #[cfg(feature = "100pin")]
    P405,
    #[cfg(feature = "100pin")]
    P406,
    P407,
    P408,
    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P409,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P4010,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P4011,
    #[cfg(feature = "100pin")]
    P4012,
    #[cfg(feature = "100pin")]
    P4013,
    #[cfg(feature = "100pin")]
    P4014,
    #[cfg(feature = "100pin")]
    P4015,

    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P500,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P501,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P502,
    #[cfg(feature = "100pin")]
    P503,
    #[cfg(feature = "100pin")]
    P504,
    #[cfg(feature = "100pin")]
    P505,

    #[cfg(feature = "100pin")]
    P600,
    #[cfg(feature = "100pin")]
    P601,
    #[cfg(feature = "100pin")]
    P602,
    #[cfg(feature = "100pin")]
    P603,
    #[cfg(feature = "100pin")]
    P608,
    #[cfg(feature = "100pin")]
    P609,
    #[cfg(feature = "100pin")]
    P6010,

    #[cfg(feature = "100pin")]
    P708,

    #[cfg(feature = "100pin")]
    P808,
    #[cfg(feature = "100pin")]
    P809,

    P914,
    P915,
);

#[rustfmt::skip]
embassy_hal_internal::peripherals_struct!(
    ADC14,
    CRC,
    ICU,
    SCE5,
    SCI0,
    SCI1,
    SCI2,
    SCI9,
    GPT32_0,
    GPT32_1,

    P000,
    P001,
    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P002,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P003,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P004,
    #[cfg(feature = "100pin")]
    P005,
    #[cfg(feature = "100pin")]
    P006,
    #[cfg(feature = "100pin")]
    P007,
    #[cfg(feature = "100pin")]
    P008,
    P010,
    P011,
    P012,
    P013,
    P014,
    P015,

    P100,
    P101,
    P102,
    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P103,
    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P104,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P105,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P106,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P107,
    P108,
    P109,
    P110,
    P111,
    P112,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P113,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P114,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P115,

    P200,
    P201,
    #[cfg(feature = "100pin")]
    P202,
    #[cfg(feature = "100pin")]
    P203,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P204,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P205,
    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P206,
    P212,
    P213,
    P214,
    P215,

    P300,
    P301,
    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P302,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P303,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P304,
    #[cfg(feature = "100pin")]
    P305,
    #[cfg(feature = "100pin")]
    P306,
    #[cfg(feature = "100pin")]
    P307,

    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P400,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P401,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P402,
    #[cfg(feature = "100pin")]
    P403,
    #[cfg(feature = "100pin")]
    P404,
    #[cfg(feature = "100pin")]
    P405,
    #[cfg(feature = "100pin")]
    P406,
    P407,
    P408,
    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P409,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P4010,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P4011,
    #[cfg(feature = "100pin")]
    P4012,
    #[cfg(feature = "100pin")]
    P4013,
    #[cfg(feature = "100pin")]
    P4014,
    #[cfg(feature = "100pin")]
    P4015,

    #[cfg(any(feature = "48pin", feature = "64pin", feature = "100pin"))]
    P500,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P501,
    #[cfg(any(feature = "64pin", feature = "100pin"))]
    P502,
    #[cfg(feature = "100pin")]
    P503,
    #[cfg(feature = "100pin")]
    P504,
    #[cfg(feature = "100pin")]
    P505,

    #[cfg(feature = "100pin")]
    P600,
    #[cfg(feature = "100pin")]
    P601,
    #[cfg(feature = "100pin")]
    P602,
    #[cfg(feature = "100pin")]
    P603,
    #[cfg(feature = "100pin")]
    P608,
    #[cfg(feature = "100pin")]
    P609,
    #[cfg(feature = "100pin")]
    P6010,

    #[cfg(feature = "100pin")]
    P708,

    #[cfg(feature = "100pin")]
    P808,
    #[cfg(feature = "100pin")]
    P809,

    P914,
    P915,
);

#[rustfmt::skip]
embassy_hal_internal::interrupt_mod!(
    IEL0,
    IEL1,
    IEL2,
);
