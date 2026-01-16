use core::marker::PhantomData;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use cortex_m::asm;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_time::Timer;
use ra4m1_ctpac::{
    rtc::vals::{Rcksel, RwkcntDayw},
    system::vals::Sodrv,
};

use crate::{
    pac,
    peripherals::{self, RTC},
};

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
            info!("RTC: LOCO not running");

            debug!("RTC: Starting LOCO");

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
        } else {
            info!("RTC: LOCO running");
        }

        if system.lococr().read().lcstp() {
            info!("RTC: LOCO  stopped??");
        } else {
            info!("RTC: LOCO running");
        }

        let rtc = I::regs();

        rtc.rfrl().write(|w| {
            w.set_rfc(0xFF);
        });

        rtc.rcr4().modify(|w| {
            w.set_rcksel(Rcksel::Loco);
        });

        for _ in 0..6 {
            rtc.rcr4().read();
        }

        rtc.rcr2().modify(|w| {
            w.set_start(false);
            w.set_cntmd(false);
        });
        warn!("Waiting for RTC to stop");
        while rtc.rcr2().read().start() {
            asm::nop();
        }
        warn!("RTC Stopped");

        rtc.rcr2().modify(|w| {
            w.set_aadje(true);
            w.set_aadjp(true);
        });
        warn!("Waiting for count mode to change");
        while rtc.rcr2().read().cntmd() != false {
            asm::nop();
        }
        warn!("Count mode changed");

        rtc.rcr2().modify(|w| {
            w.set_reset(true);
        });
        warn!("Waiting for reset");
        while rtc.rcr2().read().reset() {
            asm::nop();
        }
        warn!("Reset done");

        rtc.rcr2().modify(|w| {
            w.set_start(false);
        });
        warn!("Waiting for RTC to stop");
        while rtc.rcr2().read().start() {
            asm::nop();
        }
        warn!("RTC stopped");

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
            w.set_yr10(2);
            w.set_yr1(6);
        });

        rtc.rcr2().modify(|w| {
            w.set_start(true);
        });
        warn!("Waiting for RTC to start");
        while !rtc.rcr2().read().start() {
            asm::nop();
        }
        warn!("RTC ready");

        Self {
            _phantom: PhantomData,
        }
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
