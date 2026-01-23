//! `RTC` Realtime Clock

use core::marker::PhantomData;

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use cortex_m::asm;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_time::Timer;
use ra4m1_ctpac::rtc::vals::{Rcksel, RwkcntDayw};

use crate::{pac, peripherals};

pub struct Rtc<'d, I: Instance> {
    _phantom: PhantomData<&'d I>,
}

#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {
    // unsafe fn steal() -> Peri<'static, Self>;
}

trait SealedInstance {
    fn regs() -> pac::rtc::Rtc;
}

impl Instance for peripherals::RTC {}

impl SealedInstance for peripherals::RTC {
    fn regs() -> ra4m1_ctpac::rtc::Rtc {
        pac::RTC
    }
}

impl<'d, I: Instance> Rtc<'d, I> {
    pub async fn new(_rtc: Peri<'d, I>) -> Self {
        let system = pac::SYSTEM;
        if system.lococr().read().lcstp() {
            warn!("RTC: LOCO not running. Attempting start.");

            let vbatt_enabled = system.vbtcr1().read().bpwswstp();

            if vbatt_enabled {
                system.vbtcr1().write(|w| {
                    w.set_bpwswstp(true);
                });
            }

            system.lococr().modify(|w| {
                w.set_lcstp(false);
            });

            // § 48.3.2
            Timer::after_micros(100).await;

            if vbatt_enabled {
                system.vbtcr1().write(|w| {
                    w.set_bpwswstp(false);
                });

                while !system.vbtsr().read().vbtrvld() {
                    asm::nop();
                }
            }
        }

        let rtc = I::regs();

        let mut instance = Self {
            _phantom: PhantomData,
        };

        // §24.2.20
        rtc.rfrl().write(|w| {
            w.set_rfc(0xFF);
        });

        rtc.rcr4().modify(|w| {
            w.set_rcksel(Rcksel::Loco);
        });

        for _ in 0..6 {
            rtc.rcr4().read();
        }

        instance.stop();

        rtc.rcr2().modify(|w| {
            w.set_cntmd(false);
            w.set_aadje(true);
            w.set_aadjp(true);
        });
        while rtc.rcr2().read().cntmd() != false {
            asm::nop();
        }

        rtc.rcr2().modify(|w| {
            w.set_reset(true);
        });
        while rtc.rcr2().read().reset() {
            asm::nop();
        }

        rtc.rcr2().modify(|w| {
            w.set_start(false);
        });
        while rtc.rcr2().read().start() {
            asm::nop();
        }

        rtc.rseccnt().write(|w| {
            w.set_sec1(0);
            w.set_sec10(0);
        });

        rtc.rmincnt().write(|w| {
            w.set_min1(0);
            w.set_min10(0);
        });

        rtc.rhrcnt().write(|w| {
            w.set_hr1(0);
            w.set_hr10(0);
            w.set_pm(false);
        });

        rtc.rwkcnt().write(|w| {
            w.set_dayw(RwkcntDayw::Friday);
        });

        rtc.rmoncnt().write(|w| {
            w.set_mon10(false);
            w.set_mon1(1);
        });

        rtc.ryrcnt().write(|w| {
            w.set_yr10(7);
            w.set_yr1(0);
        });

        instance.start();

        instance
    }

    pub fn start(&mut self) {
        let rtc = I::regs();

        rtc.rcr2().modify(|w| {
            w.set_start(true);
        });

        while !rtc.rcr2().read().start() {
            asm::nop();
        }
    }

    pub fn stop(&mut self) {
        let rtc = I::regs();

        rtc.rcr2().modify(|w| {
            w.set_start(false);
        });

        while rtc.rcr2().read().start() {
            asm::nop();
        }
    }

    pub fn set_time(&mut self, date_time: NaiveDateTime) {
        let year = date_time.year_ce().1;
        let month = date_time.month();
        let day = date_time.day();
        let hour = date_time.hour();
        let minute = date_time.minute();
        let second = date_time.second();

        #[cfg(feature = "invariants")]
        {
            assert!(year < u16::MAX as u32);
            assert!(month < u8::MAX as u32);
            assert!(day < u8::MAX as u32);
            assert!(hour < u8::MAX as u32);
            assert!(minute < u8::MAX as u32);
            assert!(second < u8::MAX as u32);
        }

        self.set_time_ymd_hms(
            year as u16,
            month as u8,
            day as u8,
            hour as u8,
            minute as u8,
            second as u8,
        );
    }

    pub fn set_time_ymd_hms(
        &mut self,
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) {
        let rtc = I::regs();

        self.stop();

        rtc.ryrcnt().write(|w| {
            w.set_yr10(((year / 10) % 10) as _);
            w.set_yr1((year % 10) as _);
        });

        rtc.rmoncnt().write(|w| {
            w.set_mon10(((month / 10) % 10) == 1);
            w.set_mon1((month % 10) as _);
        });

        rtc.rdaycnt().write(|w| {
            w.set_date10(((day / 10) % 10) as _);
            w.set_date1((day % 10) as _);
        });

        rtc.rhrcnt().write(|w| {
            w.set_hr10(((hour / 10) % 10) as _);
            w.set_hr1((hour % 10) as _);
        });

        rtc.rmincnt().write(|w| {
            w.set_min10(((minute / 10) % 10) as _);
            w.set_min1((minute % 10) as _);
        });
        rtc.rseccnt().write(|w| {
            w.set_sec10(((second / 10) % 10) as _);
            w.set_sec1((second % 10) as _);
        });

        self.start();
    }

    pub fn now(&self) -> NaiveDateTime {
        let rtc = I::regs();

        let year = rtc.ryrcnt().read();
        let mut year = i32::from(year.yr1() + (10 * year.yr10()));
        if year < 69 {
            year += 2000;
        } else {
            year += 1900;
        }

        let month = rtc.rmoncnt().read();
        let month = u32::from(month.mon1() + (10 * month.mon10() as u8));

        let day = rtc.rdaycnt().read();
        let day = u32::from(day.date1() + (10 * day.date10()));

        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();

        let hour = rtc.rhrcnt().read();
        let minute = rtc.rmincnt().read();
        let seconds = rtc.rseccnt().read();

        let hour = u32::from(hour.hr1() + (10 * hour.hr10()));
        let minute = u32::from(minute.min1() + (10 * minute.min10()));
        let seconds = u32::from(seconds.sec1() + (10 * seconds.sec10()));

        let time = NaiveTime::from_hms_opt(hour, minute, seconds).unwrap();

        NaiveDateTime::new(date, time)
    }
}
