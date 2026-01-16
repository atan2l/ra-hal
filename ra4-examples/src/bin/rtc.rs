//! Rtc, demonsrate the (in)accuracy of the RTC.

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use ra4_hal::chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};
use ra4_hal::{ofs0, ofs1, print_clock_config, rtc::Rtc};

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

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra4_hal::init();

    print_clock_config();

    let mut rtc = Rtc::new(p.RTC).await;

    rtc.set_time(NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2026, 02, 01).unwrap(),
        NaiveTime::from_hms_opt(7, 19, 0).unwrap(),
    ));

    loop {
        let now = rtc.now().and_utc();
        let date = now.date_naive();
        let time = now.time();
        info!(
            "{:04}/{:02}/{:02} {:02}:{:02}:{:02}",
            date.year_ce().1,
            date.month(),
            date.day(),
            time.hour(),
            time.minute(),
            time.second()
        );
        Timer::after_secs(1).await;
    }
}
