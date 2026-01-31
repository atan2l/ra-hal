//! Rtc, demonstrate the (in)accuracy of the RTC.

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use ra4_hal::{
    chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike},
    osm::{ofs0::Ofs0, ofs1::Ofs1, sec_mpu::SecurityMpu},
    print_clock_config,
    rtc::Rtc,
};
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};

// Option Function Select Register 0 (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs0")]
static OFS0: Ofs0 = Ofs0::arduino_core();

// Option Function Select Register 1 (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs1")]
static OFS1: Ofs1 = Ofs1::arduino_core();

// Security MPU (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".sec_mpu")]
static SEC_MPU: SecurityMpu = SecurityMpu::disabled();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra4_hal::init();

    print_clock_config();

    let mut rtc = Rtc::new(p.RTC).await;

    rtc.set_time(NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
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
