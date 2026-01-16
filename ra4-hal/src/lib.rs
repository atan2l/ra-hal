#![no_std]

pub mod fmt;

pub mod adc;
pub mod crc;
pub mod gpio;
pub mod mcu_info;
pub mod ofs0;
pub mod ofs1;
pub mod rtc;
pub mod sce5;
#[cfg(feature = "time-driver")]
pub mod time_driver;
pub mod uart;
pub mod write_protect;

// Re-export
pub use chrono;
#[cfg(feature = "unstable-pac")]
pub use ra4m1_ctpac as pac;

use cortex_m::asm;
#[cfg(not(feature = "unstable-pac"))]
pub(crate) use ra4m1_ctpac as pac;
use ra4m1_ctpac::{
    fmifrt_base::vals::ExpectedBase,
    system::vals::{Cksel, Fck, Hcfrq1, Hcstp, Ick, Opcm, Pcka, Pckb, Pckc, Pckd},
};

use crate::{mcu_info::McuInfo, write_protect::WriteProtect as _};

/// Coarse indication of why the processor reset.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(unused)]
pub enum ResetCause {
    /// Power was turned don.
    PowerOn,

    /// Low voltage monitor 0, 1, or 2 tripped.
    LowVoltage,

    /// Watchdog or independent watchdog
    Watchdog,

    /// Bus error, parity error, or ECC error.
    HadwareError,

    /// Stack pointer error.
    StackPointer,

    /// Software reset requested.
    SoftwareReset,

    /// Should never be here
    Unknown,
}

pub fn init() -> Peripherals {
    // #define BSP_CLOCK_CFG_MAIN_OSC_WAIT (9)
    // #define BSP_LOCO_HZ                 (32768)
    // #define BSP_MOCO_HZ                 (8000000)
    // #define BSP_SUB_CLOCK_HZ            (32768)
    // #define BSP_MCU_VBATT_SUPPORT       (1)

    critical_section::with(|cs| {
        let system = pac::SYSTEM;
        let fmifrt_base = pac::FMIFRT_BASE;

        debug!("Starting board init");

        #[cfg(feature = "diag")]
        {
            let reset_status_0 = system.rstsr0().read();
            let reset_status_1 = system.rstsr1().read();

            let reset_cause: ResetCause;
            if reset_status_0.porf() {
                reset_cause = ResetCause::PowerOn;
            } else if reset_status_0.lvd0rf() || reset_status_0.lvd1rf() || reset_status_0.lvd2rf()
            {
                reset_cause = ResetCause::LowVoltage;
            } else if reset_status_1.wdtrf() || reset_status_1.iwdtrf() {
                reset_cause = ResetCause::Watchdog;
            } else if reset_status_1.rperf()
                || reset_status_1.reerf()
                || reset_status_1.bussrf()
                || reset_status_1.busmrf()
            {
                reset_cause = ResetCause::HadwareError;
            } else if reset_status_1.sperf() {
                reset_cause = ResetCause::StackPointer;
            } else if reset_status_1.swrf() {
                reset_cause = ResetCause::SoftwareReset;
            } else {
                reset_cause = ResetCause::Unknown;
            }

            debug!("Reset reason: {}", reset_cause);
        }

        // Sanity check.  The manual states that this should be fixed.
        #[cfg(feature = "invariants")]
        assert_eq!(
            ExpectedBase::RA4M1.to_bits(),
            fmifrt_base.base().read().base()
        );

        let mcu_info = McuInfo::info();

        #[cfg(feature = "defmt")]
        mcu_info.print_info();

        // Check if the crate was configured correctly
        mcu_info.validate_pin_count();

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

/// Logs the current clock configuration at the `debug` level.
pub fn print_clock_config() {
    #[cfg(feature = "defmt")]
    {
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
}

#[rustfmt::skip]
embassy_hal_internal::peripherals_definition!(
    ADC14,
    CRC,
    GPT32_0,
    GPT32_1,
    ICU,
    RTC,
    SCE5,
    SCI0,
    SCI1,
    SCI2,
    SCI9,

    P000,
    P001,
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P002,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P003,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P004,
    #[cfg(feature = "_100pin")]
    P005,
    #[cfg(feature = "_100pin")]
    P006,
    #[cfg(feature = "_100pin")]
    P007,
    #[cfg(feature = "_100pin")]
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
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P103,
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P104,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P105,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P106,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P107,
    P108,
    P109,
    P110,
    P111,
    P112,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P113,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P114,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P115,

    P200,
    P201,
    #[cfg(feature = "_100pin")]
    P202,
    #[cfg(feature = "_100pin")]
    P203,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P204,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P205,
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P206,
    P212,
    P213,
    P214,
    P215,

    P300,
    P301,
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P302,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P303,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P304,
    #[cfg(feature = "_100pin")]
    P305,
    #[cfg(feature = "_100pin")]
    P306,
    #[cfg(feature = "_100pin")]
    P307,

    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P400,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P401,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P402,
    #[cfg(feature = "_100pin")]
    P403,
    #[cfg(feature = "_100pin")]
    P404,
    #[cfg(feature = "_100pin")]
    P405,
    #[cfg(feature = "_100pin")]
    P406,
    P407,
    P408,
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P409,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P4010,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P4011,
    #[cfg(feature = "_100pin")]
    P4012,
    #[cfg(feature = "_100pin")]
    P4013,
    #[cfg(feature = "_100pin")]
    P4014,
    #[cfg(feature = "_100pin")]
    P4015,

    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P500,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P501,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P502,
    #[cfg(feature = "_100pin")]
    P503,
    #[cfg(feature = "_100pin")]
    P504,
    #[cfg(feature = "_100pin")]
    P505,

    #[cfg(feature = "_100pin")]
    P600,
    #[cfg(feature = "_100pin")]
    P601,
    #[cfg(feature = "_100pin")]
    P602,
    #[cfg(feature = "_100pin")]
    P603,
    #[cfg(feature = "_100pin")]
    P608,
    #[cfg(feature = "_100pin")]
    P609,
    #[cfg(feature = "_100pin")]
    P610,

    #[cfg(feature = "_100pin")]
    P708,

    #[cfg(feature = "_100pin")]
    P808,
    #[cfg(feature = "_100pin")]
    P809,

    P914,
    P915,
);

#[rustfmt::skip]
embassy_hal_internal::peripherals_struct!(
    ADC14,
    CRC,
    GPT32_0,
    GPT32_1,
    ICU,
    RTC,
    SCE5,
    SCI0,
    SCI1,
    SCI2,
    SCI9,

    P000,
    P001,
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P002,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P003,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P004,
    #[cfg(feature = "_100pin")]
    P005,
    #[cfg(feature = "_100pin")]
    P006,
    #[cfg(feature = "_100pin")]
    P007,
    #[cfg(feature = "_100pin")]
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
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P103,
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P104,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P105,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P106,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P107,
    P108,
    P109,
    P110,
    P111,
    P112,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P113,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P114,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P115,

    P200,
    P201,
    #[cfg(feature = "_100pin")]
    P202,
    #[cfg(feature = "_100pin")]
    P203,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P204,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P205,
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P206,
    P212,
    P213,
    P214,
    P215,

    P300,
    P301,
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P302,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P303,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P304,
    #[cfg(feature = "_100pin")]
    P305,
    #[cfg(feature = "_100pin")]
    P306,
    #[cfg(feature = "_100pin")]
    P307,

    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P400,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P401,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P402,
    #[cfg(feature = "_100pin")]
    P403,
    #[cfg(feature = "_100pin")]
    P404,
    #[cfg(feature = "_100pin")]
    P405,
    #[cfg(feature = "_100pin")]
    P406,
    P407,
    P408,
    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P409,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P4010,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P4011,
    #[cfg(feature = "_100pin")]
    P4012,
    #[cfg(feature = "_100pin")]
    P4013,
    #[cfg(feature = "_100pin")]
    P4014,
    #[cfg(feature = "_100pin")]
    P4015,

    #[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
    P500,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P501,
    #[cfg(any(feature = "_64pin", feature = "_100pin"))]
    P502,
    #[cfg(feature = "_100pin")]
    P503,
    #[cfg(feature = "_100pin")]
    P504,
    #[cfg(feature = "_100pin")]
    P505,

    #[cfg(feature = "_100pin")]
    P600,
    #[cfg(feature = "_100pin")]
    P601,
    #[cfg(feature = "_100pin")]
    P602,
    #[cfg(feature = "_100pin")]
    P603,
    #[cfg(feature = "_100pin")]
    P608,
    #[cfg(feature = "_100pin")]
    P609,
    #[cfg(feature = "_100pin")]
    P610,

    #[cfg(feature = "_100pin")]
    P708,

    #[cfg(feature = "_100pin")]
    P808,
    #[cfg(feature = "_100pin")]
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
