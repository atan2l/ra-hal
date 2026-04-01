//! RA2A1 specific clock configuration.

use fugit::MegahertzU32;

use crate::{
    clock::{CLOCK_STATUS, ClockConfig, ClockStatus, HocoFrequency, SystemClockSource},
    pac::{
        self,
        system::vals::{Cksel, Fck, Hcfrq1, Hcstp, Ick, Opcm, Pckb, Pckd, Sodrv},
    },
    write_protect::ProtectedPeripheral as _,
};

impl Default for ClockConfig {
    /// Config assumes EK-RA2A1.
    fn default() -> Self {
        Self {
            system: SystemClockSource::Hoco,
            hoco: HocoFrequency::_48mhz,
            mosc: None,
            sosc: false,
        }
    }
}

pub(crate) fn init(config: ClockConfig) -> Result<(), ()> {
    let system = pac::SYSTEM;
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

        // #[cfg(feature = "cache")]
        // {
        //     let fcache = pac::FCACHE;
        //     fcache.fcacheiv().write(|r| r.set_fcacheiv(true));

        //     while fcache.fcacheiv().read().fcacheiv() {
        //         asm::nop();
        //     }

        //     fcache.fcachee().write(|r| r.set_fcacheen(true));

        //     info!("SYSTEM: fcache enabled");
        // }
        // #[cfg(not(feature = "cache"))]
        // {
        //     let fcache = pac::FCACHE;
        //     fcache.fcacheiv().write(|r| r.set_fcacheiv(true));
        //     fcache.fcachee().write(|r| r.set_fcacheen(false));
        //     trace!("SYSTEM: fcache disabled");
        // }

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
                w.set_pckb(Pckb::Div1);
                // 32 MHz
                w.set_pckd(Pckd::Div1);
            }),
            Hcfrq1::_48mhz => system.sckdivcr().modify(|w| {
                // 48 MHz
                w.set_ick(Ick::Div1);
                // 24 MHz
                w.set_fck(Fck::Div2);
                // 24 MHz
                w.set_pckb(Pckb::Div2);
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
                w.set_pckb(Pckb::Div2);
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
                peripheral_b,
                peripheral_d,
                master: hoco,
                mosc: config.mosc,
                sosc: config.sosc,
            })
            .or(Err(()))
    }
}
