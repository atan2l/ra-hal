#![no_std]

pub mod crc;
pub mod ofs0;
pub mod ofs1;
#[cfg(feature = "time-driver")]
pub mod time_driver;
pub mod write_protect;

#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
#[cfg(feature = "unstable-pac")]
pub use ra4m1_ctpac as pac;
#[cfg(not(feature = "unstable-pac"))]
pub(crate) use ra4m1_ctpac as pac;
use ra4m1_ctpac::system::{
    regs::Sckdivcr,
    vals::{Fck, Ick, Pcka, Pckb, Pckc, Pckd},
};

#[rustfmt::skip]
embassy_hal_internal::peripherals_definition!(
    CRC,
    ICU,
    GPT320,
    GPT321,
);

#[rustfmt::skip]
embassy_hal_internal::peripherals_struct!(
    CRC,
    ICU,
    GPT320,
    GPT321,
);

#[rustfmt::skip]
embassy_hal_internal::interrupt_mod!(
    IEL0,
    IEL1,
    IEL2,
);

pub fn init() -> Peripherals {
    critical_section::with(|cs| {
        let p = Peripherals::take_with_cs(cs);

        #[cfg(feature = "time-driver")]
        time_driver::init(crate::interrupt::Priority::P2);

        p
    })
}

pub fn print_clock_config(config: Sckdivcr) {
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
