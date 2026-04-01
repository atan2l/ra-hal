//! RA4M1 specific clock configuration.

use fugit::MegahertzU32;

use crate::clock::{CLOCK_STATUS, ClockConfig, ClockStatus, HocoFrequency, SystemClockSource};
use crate::pac::{
    self,
    system::vals::{Cksel, Fck, Hcfrq1, Hcstp, Ick, Opcm, Pcka, Pckb, Pckc, Pckd, Sodrv},
};
use crate::write_protect::ProtectedPeripheral as _;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone)]
pub enum PllInput {
    Mosc,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone)]
pub enum PllOutDiv {
    Div2,
    Div4,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone)]
#[repr(u32)]
pub enum PllOutMul {
    Mul8,
    Mul9,
    Mul10,
    Mul11,
    Mul12,
    Mul13,
    Mul14,
    Mul15,
    Mul16,
    Mul17,
    Mul18,
    Mul19,
    Mul20,
    Mul21,
    Mul22,
    Mul23,
    Mul24,
    Mul25,
    Mul26,
    Mul27,
    Mul28,
    Mul29,
    Mul30,
    Mul31,
}

impl Default for ClockConfig {
    /// Config assumes EK-RA4M1.
    fn default() -> Self {
        Self {
            system: SystemClockSource::Hoco,
            hoco: HocoFrequency::_48mhz,
            mosc: None,
            sosc: false,
            pll: None,
        }
    }
}

pub(crate) fn init(config: ClockConfig) -> Result<(), ()> {
    let system = pac::SYSTEM;

    #[cfg(feature = "diag")]
    {
        use crate::ResetCause;

        let reset_status_0 = system.rstsr0().read();
        let reset_status_1 = system.rstsr1().read();

        let reset_cause: ResetCause;
        if reset_status_0.porf() {
            reset_cause = ResetCause::PowerOn;
        } else if reset_status_0.lvd0rf() || reset_status_0.lvd1rf() || reset_status_0.lvd2rf() {
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

    // #[cfg(feature = "strict-assert")]
    // let fmifrt_base = pac::FMIFRT_BASE;
    // Sanity check.  The manual states that this should be fixed.
    // #[cfg(feature = "strict-assert")]
    // assert_eq!(
    //     pac::fmifrt_base::vals::ExpectedBase::RA4M1.to_bits(),
    //     fmifrt_base.base().read().base()
    // );

    trace!(
        "HOCO: wait={}, status={}",
        system.hocowtcr().read(),
        system.hococr().read()
    );

    system.protected_write(|| {
        // Writing to the HOCOCR2 is prohibited when the HOCOCR.HCSTP bit is 0 (the HOCO operates).
        let should_configure = match system.hococr().read().hcstp() {
            Hcstp::Start => {
                let frq = system.hococr2().read().hcfrqw();
                let should_configure = frq != config.hoco.into();
                if should_configure {
                    system.hococr().write(|w| w.set_hcstp(Hcstp::Stop));
                }
                should_configure
            }
            Hcstp::Stop => true,
        };

        if should_configure {
            system.hococr2().write(|r| r.set_hcfrqw(config.hoco.into()));
            system.hococr().write(|w| w.set_hcstp(Hcstp::Start));
        }

        if config.sosc {
            system.somcr().write(|r| r.set_sodrv(Sodrv::_00));
            system.sosccr().write(|r| r.set_sostp(false));
            // TODO: Set oscillator stabilization time.
            while system.sosccr().read().sostp() {}
        }

        let hoco_freq = system.hococr2().read().hcfrqw();

        // High speed mode needed for ICLK > 12 MHz.  Currently there are no features to select
        // ICLK <= 12 MHz so just enable high speed mode unconditionally.
        // Table 48.18
        trace!("Setting high speed mode on");
        system.opccr().write(|w| w.set_opcm(Opcm::HighSpeed));

        while system.opccr().read().opcmtsf() {}

        // Wait states needed for ICLK > 32 MHz
        if hoco_freq == Hcfrq1::_48mhz || hoco_freq == Hcfrq1::_64mhz {
            trace!("Setting SYSTEM_MEMWAIT to 1");
            system.memwait().write(|w| w.set_memwait(true));
        }

        // Use HOCO as clock source
        system.sckscr().write(|w| w.set_cksel(Cksel::Hoco));

        #[cfg(feature = "cache")]
        {
            let fcache = pac::FCACHE;
            fcache.fcacheiv().write(|r| r.set_fcacheiv(true));

            while fcache.fcacheiv().read().fcacheiv() {
                asm::nop();
            }

            fcache.fcachee().write(|r| r.set_fcacheen(true));

            info!("SYSTEM: fcache enabled");
        }
        #[cfg(not(feature = "cache"))]
        {
            let fcache = pac::FCACHE;
            fcache.fcacheiv().write(|r| r.set_fcacheiv(true));
            fcache.fcachee().write(|r| r.set_fcacheen(false));
            trace!("SYSTEM: fcache disabled");
        }

        // Max frequencies Table 8.2, p130
        // ICLK = 48 MHz
        // FCLK = 32 MHz
        // PCKLA = 48 MHz
        // PCLKB = 32 MHz
        // PCLKC = 64 MHz
        // PCLKD = 64 MHz
        match hoco_freq {
            Hcfrq1::_32mhz => system.sckdivcr().modify(|w| {
                // 32 MHz
                w.set_ick(Ick::Div1);
                // 32 MHz
                w.set_fck(Fck::Div1);
                // 32 MHz
                w.set_pcka(Pcka::Div1);
                // 32 MHz
                w.set_pckb(Pckb::Div1);
                // 32 MHz
                w.set_pckc(Pckc::Div1);
                // 32 MHz
                w.set_pckd(Pckd::Div1);
            }),
            Hcfrq1::_48mhz => system.sckdivcr().modify(|w| {
                // 48 MHz
                w.set_ick(Ick::Div1);
                // 24 MHz
                w.set_fck(Fck::Div2);
                // 48 MHz
                w.set_pcka(Pcka::Div1);
                // 24 MHz
                w.set_pckb(Pckb::Div2);
                // 48 MHz
                w.set_pckc(Pckc::Div1);
                // 48 MHz
                w.set_pckd(Pckd::Div1);
            }),
            // Faster peripheral clocks, slower CPU clock
            Hcfrq1::_64mhz => system.sckdivcr().modify(|w| {
                // 32 MHz
                w.set_ick(Ick::Div2);
                // 32 MHz
                w.set_fck(Fck::Div2);
                // 32 MHz
                w.set_pcka(Pcka::Div2);
                // 32 MHz
                w.set_pckb(Pckb::Div2);
                // 64 MHz
                w.set_pckc(Pckc::Div1);
                // 64 MHz
                w.set_pckd(Pckd::Div1);
            }),
            _ => unimplemented!(),
        }
    });

    {
        let system = pac::SYSTEM;

        let hoco = system.hococr2().read().hcfrqw();
        let hoco = match hoco {
            Hcfrq1::_24mhz => MegahertzU32::from_raw(24),
            Hcfrq1::_32mhz => MegahertzU32::from_raw(32),
            Hcfrq1::_48mhz => MegahertzU32::from_raw(48),
            Hcfrq1::_64mhz => MegahertzU32::from_raw(64),
            _ => unimplemented!(),
        }
        .convert();

        let prescaler = system.sckdivcr().read();

        let system = match prescaler.ick() {
            Ick::Div1 => hoco,
            Ick::Div2 => hoco / 2,
            Ick::Div4 => hoco / 4,
            Ick::Div8 => hoco / 8,
            Ick::Div16 => hoco / 16,
            Ick::Div32 => hoco / 32,
            Ick::Div64 => hoco / 64,
            Ick::_RESERVED_7 => unimplemented!("Invalid sckdivcr.ick"),
        };

        let flash = match prescaler.fck() {
            Fck::Div1 => hoco,
            Fck::Div2 => hoco / 2,
            Fck::Div4 => hoco / 4,
            Fck::Div8 => hoco / 8,
            Fck::Div16 => hoco / 16,
            Fck::Div32 => hoco / 32,
            Fck::Div64 => hoco / 64,
            Fck::_RESERVED_7 => unimplemented!("Invalid sckdivcr.fck"),
        };

        let peripheral_a = match prescaler.pcka() {
            Pcka::Div1 => hoco,
            Pcka::Div2 => hoco / 2,
            Pcka::Div4 => hoco / 4,
            Pcka::Div8 => hoco / 8,
            Pcka::Div16 => hoco / 16,
            Pcka::Div32 => hoco / 32,
            Pcka::Div64 => hoco / 64,
            Pcka::_RESERVED_7 => unimplemented!("Invalid sckdivcr.pcka"),
        };

        let peripheral_b = match prescaler.pckb() {
            Pckb::Div1 => hoco,
            Pckb::Div2 => hoco / 2,
            Pckb::Div4 => hoco / 4,
            Pckb::Div8 => hoco / 8,
            Pckb::Div16 => hoco / 16,
            Pckb::Div32 => hoco / 32,
            Pckb::Div64 => hoco / 64,
            Pckb::_RESERVED_7 => unimplemented!("Invalid sckdivcr.pckb"),
        };

        let peripheral_c = match prescaler.pckc() {
            Pckc::Div1 => hoco,
            Pckc::Div2 => hoco / 2,
            Pckc::Div4 => hoco / 4,
            Pckc::Div8 => hoco / 8,
            Pckc::Div16 => hoco / 16,
            Pckc::Div32 => hoco / 32,
            Pckc::Div64 => hoco / 64,
            Pckc::_RESERVED_7 => unimplemented!("Invalid sckdivcr.pckc"),
        };

        let peripheral_d = match prescaler.pckd() {
            Pckd::Div1 => hoco,
            Pckd::Div2 => hoco / 2,
            Pckd::Div4 => hoco / 4,
            Pckd::Div8 => hoco / 8,
            Pckd::Div16 => hoco / 16,
            Pckd::Div32 => hoco / 32,
            Pckd::Div64 => hoco / 64,
            Pckd::_RESERVED_7 => unimplemented!("Invalid sckdivcr.pckd"),
        };

        CLOCK_STATUS
            .init(ClockStatus {
                hoco,
                system,
                flash,
                peripheral_a,
                peripheral_b,
                peripheral_c,
                peripheral_d,
                master: hoco,
                pll: None,
                mosc: config.mosc,
                sosc: config.sosc,
            })
            .or(Err(()))
    }
}
