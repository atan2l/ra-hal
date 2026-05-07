use cortex_m::asm;
use fugit::{HertzU32, MegahertzU32, RateExtU32 as _};
use ra_metapac::{
    fcache::vals::Flwt,
    system::{
        regs::{Pllccr, Pllccr2},
        vals::{Cpuck, Hcfrq0, Opcm},
    },
};

use crate::{
    clock::{CLOCK_STATUS, ClockConfig, ClockStatus, HocoFrequency, PllConfig, SystemClockSource},
    pac::{
        self,
        system::vals::{
            Bck, Fck, Hcstp, Ick, Pcka, Pckb, Pckc, Pckd, Pcke, Plidiv, Pllmul, Pllmulnf, Plodivp,
            Plodivq, Plodivr, Plsrcsel, SckscrCksel, Sodrv,
        },
    },
    write_protect::ProtectedPeripheral as _,
};

// Max PLL output is 480 MHz

/// Input clock range
const PLL_RAW_INPUT_MIN: HertzU32 = MegahertzU32::from_raw(8).convert();
const PLL_RAW_INPUT_MAX: HertzU32 = MegahertzU32::from_raw(48).convert();

/// Input clock range after division
const PLL_INPUT_MIN: HertzU32 = MegahertzU32::from_raw(6).convert();
const PLL_INPUT_MAX: HertzU32 = MegahertzU32::from_raw(12).convert();

const PLL_OUTPUT_MIN: HertzU32 = MegahertzU32::from_raw(40).convert();
const PLL_OUTPUT_MAX: HertzU32 = MegahertzU32::from_raw(480).convert();

const OUTPUT_FACTOR: u32 = 10;

#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub enum PllInDiv {
    Div1,
    Div2,
    Div3,
    Div4,
}

#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub enum PllInput {
    Hoco,
    Mosc,
}

#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum PllPDiv {
    Div2 = 2,
    Div4 = 4,
    Div6 = 6,
    Div8 = 8,
    Div16 = 16,
}

#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum PllQDiv {
    Div2 = 2,
    Div3 = 3,
    Div4 = 4,
    Div5 = 5,
    Div6 = 6,
    Div8 = 8,
    Div9 = 9,
}

#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum PllRDiv {
    Div2 = 2,
    Div3 = 3,
    Div4 = 4,
    Div5 = 5,
    Div6 = 6,
    Div8 = 8,
    Div9 = 9,
}

// Should we just make this an integer and range check at runtime?
#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub enum PllOutMul {
    Mul26_0 = 260,
    Mul27_0 = 270,
    Mul28_0 = 280,
    Mul40_0 = 400,
    Mul80_0 = 800,
    Mul89_0 = 890,
    Mul90_0 = 900,
    Mul91_0 = 910,
    Mul179_0 = 1790,
    Mul180_0 = 1800,
}

impl From<PllInput> for Plsrcsel {
    fn from(value: PllInput) -> Self {
        match value {
            PllInput::Hoco => Self::Hoco,
            PllInput::Mosc => Self::Mosc,
        }
    }
}

impl From<PllInDiv> for Plidiv {
    fn from(value: PllInDiv) -> Self {
        match value {
            PllInDiv::Div1 => Self::Div1,
            PllInDiv::Div2 => Self::Div2,
            PllInDiv::Div3 => Self::Div3,
            PllInDiv::Div4 => Self::Div4,
        }
    }
}

impl From<PllOutMul> for Pllmul {
    fn from(value: PllOutMul) -> Self {
        match value {
            PllOutMul::Mul26_0 => Self::Mul26,
            PllOutMul::Mul27_0 => Self::Mul27,
            PllOutMul::Mul28_0 => Self::Mul28,
            PllOutMul::Mul40_0 => Self::Mul40,
            PllOutMul::Mul80_0 => Self::Mul80,
            PllOutMul::Mul89_0 => Self::Mul89,
            PllOutMul::Mul90_0 => Self::Mul90,
            PllOutMul::Mul91_0 => Self::Mul91,
            PllOutMul::Mul179_0 => Self::Mul179,
            PllOutMul::Mul180_0 => Self::Mul180,
        }
    }
}

fn pll_status(
    status: bool,
    hoco: HertzU32,
    mosc: Option<HertzU32>,
    pllccr: Pllccr,
    pllccr2: Pllccr2,
) -> Option<(HertzU32, HertzU32, HertzU32)> {
    match status {
        true => {
            let input_frequency = match pllccr.plsrcsel() {
                Plsrcsel::Mosc => mosc.unwrap(),
                Plsrcsel::Hoco => hoco,
            };
            let output_mul: u32 = match pllccr.pllmul() {
                Pllmul::Mul26 => 26,
                Pllmul::Mul27 => 27,
                Pllmul::Mul28 => 28,
                Pllmul::Mul40 => 40,
                Pllmul::Mul80 => 80,
                Pllmul::Mul89 => 89,
                Pllmul::Mul90 => 90,
                Pllmul::Mul91 => 91,
                Pllmul::Mul180 => 180,
                _ => unimplemented!(),
            };

            let input_div: u32 = match pllccr.plidiv() {
                Plidiv::Div1 => 1,
                Plidiv::Div2 => 2,
                Plidiv::Div3 => 3,
                Plidiv::Div4 => 4,
            };

            let output_factor = 10;

            let vco = (((input_frequency * 10) / input_div) / output_factor) * output_mul;

            let pll_p = vco
                / match pllccr2.plodivp() {
                    Plodivp::_0000 => 1,
                    Plodivp::Div2 => 2,
                    Plodivp::Div4 => 4,
                    Plodivp::Div6 => 6,
                    Plodivp::Div8 => 8,
                    Plodivp::Div16 => 16,
                    Plodivp::_RESERVED_2
                    | Plodivp::_RESERVED_4
                    | Plodivp::_RESERVED_6
                    | Plodivp::_RESERVED_8
                    | Plodivp::_RESERVED_9
                    | Plodivp::_RESERVED_a
                    | Plodivp::_RESERVED_b
                    | Plodivp::_RESERVED_c
                    | Plodivp::_RESERVED_d
                    | Plodivp::_RESERVED_e => unimplemented!(),
                };

            let pll_q = vco
                / match pllccr2.plodivq() {
                    Plodivq::Div2 => 2,
                    Plodivq::Div3 => 3,
                    Plodivq::Div4 => 4,
                    Plodivq::Div5 => 5,
                    Plodivq::Div6 => 6,
                    Plodivq::Div8 => 8,
                    Plodivq::Div9 => 9,
                    Plodivq::_RESERVED_0
                    | Plodivq::_RESERVED_6
                    | Plodivq::_RESERVED_9
                    | Plodivq::_RESERVED_a
                    | Plodivq::_RESERVED_b
                    | Plodivq::_RESERVED_c
                    | Plodivq::_RESERVED_d
                    | Plodivq::_RESERVED_e
                    | Plodivq::_RESERVED_f => unimplemented!(),
                };

            let pll_r = vco
                / match pllccr2.plodivr() {
                    Plodivr::Div2 => 2,
                    Plodivr::Div3 => 3,
                    Plodivr::Div4 => 4,
                    Plodivr::Div5 => 5,
                    Plodivr::Div6 => 6,
                    Plodivr::Div8 => 8,
                    Plodivr::Div9 => 9,
                    Plodivr::_RESERVED_0
                    | Plodivr::_RESERVED_6
                    | Plodivr::_RESERVED_9
                    | Plodivr::_RESERVED_a
                    | Plodivr::_RESERVED_b
                    | Plodivr::_RESERVED_c
                    | Plodivr::_RESERVED_d
                    | Plodivr::_RESERVED_e
                    | Plodivr::_RESERVED_f => unimplemented!(),
                };

            Some((pll_p, pll_q, pll_r))
        }
        false => None,
    }
}

fn calc_pll_frequency(
    pll_config: &PllConfig,
    config: &ClockConfig,
    input_range: [HertzU32; 2],
    output_range: [HertzU32; 2],
) -> (HertzU32, HertzU32, HertzU32) {
    let output_mul: u32 = pll_config.mul as u32;

    let input_div: u32 = match pll_config.div {
        PllInDiv::Div1 => 1,
        PllInDiv::Div2 => 2,
        PllInDiv::Div3 => 3,
        PllInDiv::Div4 => 4,
    };

    let raw_input_frequency = match pll_config.input {
        PllInput::Hoco => config.hoco.into(),
        PllInput::Mosc => config.mosc.unwrap(),
    };

    let pll_input_frequency = raw_input_frequency / input_div;

    assert!(
        pll_input_frequency >= input_range[0],
        "Invalid PLL configuration, input too slow {pll_input_frequency} < {}",
        input_range[0]
    );
    assert!(
        pll_input_frequency <= input_range[1],
        "Invalid PLL configuration, input too fast {pll_input_frequency} > {}",
        input_range[1]
    );

    let pll_output = (pll_input_frequency * output_mul) / OUTPUT_FACTOR;

    assert!(
        pll_output >= output_range[0],
        "Invalid PLL configuration, output too slow {pll_output} < {}",
        output_range[0]
    );
    assert!(
        pll_output <= output_range[1],
        "Invalid PLL configuration, output too fast {pll_output} > {}",
        output_range[1]
    );

    (
        (pll_output / pll_config.div_p as u32),
        (pll_output / pll_config.div_q as u32),
        (pll_output / pll_config.div_r as u32),
    )
}

pub(crate) fn init(config: ClockConfig) -> Result<(), ()> {
    let system = pac::SYSTEM;
    debug!("HOCO: status={}", system.hococr().read());

    if config.pll.is_none() && config.system == SystemClockSource::Pll1P {
        panic!("PLL selected as root clock, but not enabled/configured.");
    }

    if !config.sosc && config.system == SystemClockSource::Pll1P {
        panic!("SOSC required to use PLL as root clock, but SOSC not enabled.");
    }

    system.protected_write(|| {
        trace!("Setting high speed mode");
        system.opccr().modify(|r| r.set_opcm(Opcm::_00));
        while system.opccr().read().opcmtsf() {}

        if config.sosc {
            trace!("Enabling SCOSC");
            system.somcr().modify(|r| r.set_sodrv(Sodrv::Normal));
            system.sosccr().modify(|r| r.set_sostp(false));
            // TODO: Set oscillator stabilization time.
            while system.sosccr().read().sostp() {}
        }

        let fll_magic = match config.hoco {
            HocoFrequency::_16mhz => crate::constants::FLL_16MHZ,
            HocoFrequency::_18mhz => crate::constants::FLL_18MHZ,
            HocoFrequency::_20mhz => crate::constants::FLL_20MHZ,
            HocoFrequency::_32mhz => crate::constants::FLL_32MHZ,
            HocoFrequency::_48mhz => crate::constants::FLL_48MHZ,
        };
        system.fllcr2().modify(|r| r.set_fllcntl(fll_magic));
        system.fllcr1().modify(|r| r.set_fllen(true));

        system.hococr2().write(|r| r.set_hcfrq0(config.hoco.into()));
        system.hococr().write(|r| r.set_hcstp(Hcstp::Start));
        while !system.oscsf().read().hocosf() {}

        system.pllcr().write(|r| r.set_pllstp(true));
        while system.oscsf().read().pllsf() {}

        if let Some(pll_config) = config.pll.as_ref() {
            system.pllccr().modify(|r| {
                r.set_plsrcsel(pll_config.input.into());
                r.set_plidiv(pll_config.div.into());
                r.set_pllmul(pll_config.mul.into());
                r.set_pllmulnf(Pllmulnf::_00);
            });
            system.pllccr2().modify(|r| {
                r.set_plodivp(Plodivp::Div2);
                r.set_plodivq(Plodivq::Div2);
                r.set_plodivr(Plodivr::Div2);
            });
            system.pllcr().write(|r| r.set_pllstp(false));
            while !system.oscsf().read().pllsf() {}
            trace!("PLL1 stabilized");
        }

        let root_frequency: HertzU32;
        match config.system {
            SystemClockSource::Mosc => todo!(),
            SystemClockSource::Sosc => todo!(),
            SystemClockSource::Hoco => todo!(),
            SystemClockSource::Moco => todo!(),
            SystemClockSource::Pll1P => root_frequency = 480_u32.MHz(),
        }

        let (pckb_div, pckc_div, fck_div, pcka_div, pckd_div, ick_div, pcke_div, bck_div) = {
            let _60mhz = root_frequency.to_Hz().div_ceil(60_000_000);
            let _100mhz = root_frequency.to_Hz().div_ceil(100_000_000);
            let _120mhz = root_frequency.to_Hz().div_ceil(120_000_000);
            let _240mhz = root_frequency.to_Hz().div_ceil(240_000_000);

            let (pckb_div, pckc_div, fck_div) = match _60mhz {
                1 => (Pckb::Div1, Pckc::Div1, Fck::Div1),
                2 => (Pckb::Div2, Pckc::Div2, Fck::Div2),
                3 => (Pckb::Div3, Pckc::Div3, Fck::Div3),
                4 => (Pckb::Div4, Pckc::Div4, Fck::Div4),
                5..=6 => (Pckb::Div6, Pckc::Div6, Fck::Div6),
                7..=8 => (Pckb::Div8, Pckc::Div8, Fck::Div8),
                9..=12 => (Pckb::Div12, Pckc::Div12, Fck::Div12),
                13..=16 => (Pckb::Div16, Pckc::Div16, Fck::Div16),
                17..=32 => (Pckb::Div32, Pckc::Div32, Fck::Div32),
                33..=64 => (Pckb::Div64, Pckc::Div64, Fck::Div64),
                _ => unimplemented!(),
            };

            let (pcka_div, pckd_div, bck_div) = match _120mhz {
                1 => (Pcka::Div1, Pckd::Div1, Bck::Div1),
                2 => (Pcka::Div2, Pckd::Div2, Bck::Div2),
                3 => (Pcka::Div3, Pckd::Div3, Bck::Div3),
                4 => (Pcka::Div4, Pckd::Div4, Bck::Div4),
                5..=6 => (Pcka::Div6, Pckd::Div6, Bck::Div6),
                7..=8 => (Pcka::Div8, Pckd::Div8, Bck::Div8),
                9..=12 => (Pcka::Div12, Pckd::Div12, Bck::Div12),
                13..=16 => (Pcka::Div16, Pckd::Div16, Bck::Div16),
                17..=32 => (Pcka::Div32, Pckd::Div32, Bck::Div32),
                33..=64 => (Pcka::Div64, Pckd::Div64, Bck::Div64),
                _ => unimplemented!(),
            };

            let (ick_div, pcke_div) = match _240mhz {
                1 => (Ick::Div1, Pcke::Div1),
                2 => (Ick::Div2, Pcke::Div2),
                3 => (Ick::Div3, Pcke::Div3),
                4 => (Ick::Div4, Pcke::Div4),
                5..=6 => (Ick::Div6, Pcke::Div6),
                7..=8 => (Ick::Div8, Pcke::Div8),
                9..=12 => (Ick::Div12, Pcke::Div12),
                13..=16 => (Ick::Div16, Pcke::Div16),
                17..=32 => (Ick::Div32, Pcke::Div32),
                33..=64 => (Ick::Div64, Pcke::Div64),
                _ => unimplemented!(),
            };

            (
                pckb_div, pckc_div, fck_div, pcka_div, pckd_div, ick_div, pcke_div, bck_div,
            )
        };

        let ick_frq = match ick_div {
            Ick::Div1 => root_frequency / 1,
            Ick::Div2 => root_frequency / 2,
            Ick::Div3 => root_frequency / 3,
            Ick::Div4 => root_frequency / 4,
            Ick::Div6 => root_frequency / 6,
            Ick::Div8 => root_frequency / 8,
            Ick::Div12 => root_frequency / 12,
            Ick::Div16 => root_frequency / 16,
            Ick::Div32 => root_frequency / 32,
            Ick::Div64 => root_frequency / 64,
            Ick::_RESERVED_7 => todo!(),
            Ick::_RESERVED_b => todo!(),
            Ick::_RESERVED_c => todo!(),
            Ick::_RESERVED_d => todo!(),
            Ick::_RESERVED_e => todo!(),
            Ick::_RESERVED_f => todo!(),
        };

        // Needed for ƒICLK >= 120 MHz
        let sram = pac::SRAM;
        sram.sramwtsc().write(|r| r.set_wten(true));

        let fcache = pac::FCACHE;
        fcache.flwt().modify(|r| r.set_flwt(Flwt::_100));

        system.sckdivcr().write(|r| {
            r.set_ick(ick_div);
            r.set_fck(fck_div);
            r.set_bck(bck_div);
            r.set_pcka(pcka_div);
            r.set_pckb(pckb_div);
            r.set_pckc(pckc_div);
            r.set_pckd(pckd_div);
            r.set_pcke(pcke_div);
        });
        while system.sckdivcr().read().pcke() != pcke_div {
            error!("If stuck here, clock didn't set correctly.");
        }
        system.sckdivcr2().modify(|r| r.set_cpuck(Cpuck::_0000));

        match config.system {
            SystemClockSource::Mosc => todo!(),
            SystemClockSource::Sosc => {
                assert_eq!(config.sosc, true);
                system.sckscr().modify(|r| r.set_cksel(SckscrCksel::Sosc));
            }
            SystemClockSource::Hoco => {
                system.sckscr().modify(|r| r.set_cksel(SckscrCksel::Hoco));
            }
            SystemClockSource::Moco => todo!(),
            SystemClockSource::Pll1P => {
                // RA6M5: § 8.2.3 When changing the value from a non-PLL clock source to the PLL, wait at least
                // 250 ns after changing the value, before starting subsequent processing.
                system.sckscr().modify(|r| r.set_cksel(SckscrCksel::Pll1P));
                for _ in 0..100 {
                    asm::nop();
                }
            }
        }

        {
            let system = pac::SYSTEM;
            let hoco = system.hococr2().read().hcfrq0();
            let hoco: HertzU32 = match hoco {
                Hcfrq0::_16mhz => 16.MHz(),
                Hcfrq0::_18mhz => 18.MHz(),
                Hcfrq0::_20mhz => 20.MHz(),
                Hcfrq0::_32mhz => 32.MHz(),
                Hcfrq0::_48mhz => 48.MHz(),
                Hcfrq0::_RESERVED_3 | Hcfrq0::_RESERVED_5 | Hcfrq0::_RESERVED_6 => unimplemented!(),
            };

            let pll_running = !system.pllcr().read().pllstp();
            let pll = pll_status(
                pll_running,
                hoco,
                config.mosc,
                system.pllccr().read(),
                system.pllccr2().read(),
            );

            let pll2_running = !system.pll2cr().read().pllstp();
            let pll2 = pll_status(
                pll2_running,
                hoco,
                config.mosc,
                system.pll2ccr().read(),
                system.pll2ccr2().read(),
            );

            let cksel = system.sckscr().read().cksel();
            let master = match cksel {
                SckscrCksel::Hoco => hoco,
                SckscrCksel::Moco => 8_u32.MHz(),
                SckscrCksel::Sosc => 32_768_u32.Hz(),
                SckscrCksel::Pll1P => pll.unwrap().0,
                _ => unimplemented!(),
            };

            let prescaler = system.sckdivcr().read();

            let iclk = match prescaler.ick() {
                Ick::Div1 => master,
                Ick::Div2 => master / 2,
                Ick::Div3 => master / 3,
                Ick::Div4 => master / 4,
                Ick::Div6 => master / 6,
                Ick::Div8 => master / 8,
                Ick::Div12 => master / 12,
                Ick::Div16 => master / 16,
                Ick::Div32 => master / 32,
                Ick::Div64 => master / 64,
                Ick::_RESERVED_7
                | Ick::_RESERVED_b
                | Ick::_RESERVED_c
                | Ick::_RESERVED_d
                | Ick::_RESERVED_e
                | Ick::_RESERVED_f => unimplemented!("Invalid sckdivcr.ick"),
            };

            let flash = match prescaler.fck() {
                Fck::Div1 => master,
                Fck::Div2 => master / 2,
                Fck::Div3 => master / 3,
                Fck::Div4 => master / 4,
                Fck::Div6 => master / 6,
                Fck::Div8 => master / 8,
                Fck::Div12 => master / 12,
                Fck::Div16 => master / 16,
                Fck::Div32 => master / 32,
                Fck::Div64 => master / 64,
                Fck::_RESERVED_7
                | Fck::_RESERVED_b
                | Fck::_RESERVED_c
                | Fck::_RESERVED_d
                | Fck::_RESERVED_e
                | Fck::_RESERVED_f => unimplemented!("Invalid sckdivcr.fck"),
            };

            let peripheral_a = match prescaler.pcka() {
                Pcka::Div1 => master,
                Pcka::Div2 => master / 2,
                Pcka::Div3 => master / 3,
                Pcka::Div4 => master / 4,
                Pcka::Div6 => master / 6,
                Pcka::Div8 => master / 8,
                Pcka::Div12 => master / 12,
                Pcka::Div16 => master / 16,
                Pcka::Div32 => master / 32,
                Pcka::Div64 => master / 64,
                Pcka::_RESERVED_7
                | Pcka::_RESERVED_b
                | Pcka::_RESERVED_c
                | Pcka::_RESERVED_d
                | Pcka::_RESERVED_e
                | Pcka::_RESERVED_f => unimplemented!("Invalid sckdivcr.pcka"),
            };

            let peripheral_b = match prescaler.pckb() {
                Pckb::Div1 => master,
                Pckb::Div2 => master / 2,
                Pckb::Div3 => master / 3,
                Pckb::Div4 => master / 4,
                Pckb::Div6 => master / 6,
                Pckb::Div8 => master / 8,
                Pckb::Div12 => master / 12,
                Pckb::Div16 => master / 16,
                Pckb::Div32 => master / 32,
                Pckb::Div64 => master / 64,
                Pckb::_RESERVED_7
                | Pckb::_RESERVED_b
                | Pckb::_RESERVED_c
                | Pckb::_RESERVED_d
                | Pckb::_RESERVED_e
                | Pckb::_RESERVED_f => unimplemented!("Invalid sckdivcr.pckb"),
            };

            let peripheral_c = match prescaler.pckc() {
                Pckc::Div1 => master,
                Pckc::Div2 => master / 2,
                Pckc::Div3 => master / 3,
                Pckc::Div4 => master / 4,
                Pckc::Div6 => master / 6,
                Pckc::Div8 => master / 8,
                Pckc::Div12 => master / 12,
                Pckc::Div16 => master / 16,
                Pckc::Div32 => master / 32,
                Pckc::Div64 => master / 64,
                Pckc::_RESERVED_7
                | Pckc::_RESERVED_b
                | Pckc::_RESERVED_c
                | Pckc::_RESERVED_d
                | Pckc::_RESERVED_e
                | Pckc::_RESERVED_f => unimplemented!("Invalid sckdivcr.pckc"),
            };

            let peripheral_d = match prescaler.pckd() {
                Pckd::Div1 => master,
                Pckd::Div2 => master / 2,
                Pckd::Div3 => master / 3,
                Pckd::Div4 => master / 4,
                Pckd::Div6 => master / 6,
                Pckd::Div8 => master / 8,
                Pckd::Div12 => master / 12,
                Pckd::Div16 => master / 16,
                Pckd::Div32 => master / 32,
                Pckd::Div64 => master / 64,
                Pckd::_RESERVED_7
                | Pckd::_RESERVED_b
                | Pckd::_RESERVED_c
                | Pckd::_RESERVED_d
                | Pckd::_RESERVED_e
                | Pckd::_RESERVED_f => unimplemented!("Invalid sckdivcr.pckd"),
            };

            let peripheral_e = match prescaler.pcke() {
                Pcke::Div1 => master,
                Pcke::Div2 => master / 2,
                Pcke::Div3 => master / 3,
                Pcke::Div4 => master / 4,
                Pcke::Div6 => master / 6,
                Pcke::Div8 => master / 8,
                Pcke::Div12 => master / 12,
                Pcke::Div16 => master / 16,
                Pcke::Div32 => master / 32,
                Pcke::Div64 => master / 64,
                Pcke::_RESERVED_7
                | Pcke::_RESERVED_b
                | Pcke::_RESERVED_c
                | Pcke::_RESERVED_d
                | Pcke::_RESERVED_e
                | Pcke::_RESERVED_f => unimplemented!("Invalid sckdivcr.pcke"),
            };

            let bus_clock = match prescaler.bck() {
                Bck::Div1 => master,
                Bck::Div2 => master / 2,
                Bck::Div3 => master / 3,
                Bck::Div4 => master / 4,
                Bck::Div6 => master / 6,
                Bck::Div8 => master / 8,
                Bck::Div12 => master / 12,
                Bck::Div16 => master / 16,
                Bck::Div32 => master / 32,
                Bck::Div64 => master / 64,
                Bck::_RESERVED_7
                | Bck::_RESERVED_b
                | Bck::_RESERVED_c
                | Bck::_RESERVED_d
                | Bck::_RESERVED_e
                | Bck::_RESERVED_f => unimplemented!("Invalid sckdivcr.bck"),
            };

            CLOCK_STATUS
                .init(ClockStatus {
                    master,
                    mosc: config.mosc,
                    sosc: config.sosc,
                    hoco,
                    pll,
                    pll2,
                    system: iclk,
                    flash,
                    peripheral_a,
                    peripheral_b,
                    peripheral_c,
                    peripheral_d,
                    peripheral_e,
                    bus_clock,
                })
                .or(Err(()));
        }
    });

    Ok(())
}

impl Default for ClockConfig {
    /// Config assumes EK-RA8M1.
    fn default() -> Self {
        Self {
            system: SystemClockSource::Pll1P,
            hoco: HocoFrequency::_48mhz,
            mosc: Some(20.MHz()),
            sosc: true,
            pll: Some(PllConfig {
                input: PllInput::Hoco,
                div: PllInDiv::Div4,
                mul: PllOutMul::Mul80_0,
                div_p: PllPDiv::Div2,
                div_q: PllQDiv::Div2,
                div_r: PllRDiv::Div2,
            }),
            pll2: None,
        }
    }
}
