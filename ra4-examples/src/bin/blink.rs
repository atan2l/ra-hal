//! Blink, hello world example

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
use defmt_rtt as _;
use embassy_executor::Spawner;
// use embassy_time::Timer;
use panic_probe as _;
use ra4_hal::{ofs0, ofs1};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs0")]
/// Option Function Select Register 0
pub static OFS0: u32 = ofs0!(
    IwdtAutoStartOff,
    IwdtTimeout2048,
    IwdtClockRatio1_128,
    IwdtWindowEndNone,
    IwdtWindowStartNone,
    IwdtReset,
    IwdtSleepStop,
    WdtAutoStartOff,
    WdtTimeout16384,
    WdtClockRatio128,
    WdtWindowEndNone,
    WdtWindowStartNone,
    WdtReset,
    WdtSleepStop
);

#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs1")]
/// Option Function Select Register 1
pub static OFS1: u32 = ofs1!(LvdasOff, Vdsel384, HocoOn, Hoco48Mhz);
