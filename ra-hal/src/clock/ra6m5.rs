use cortex_m::asm;
use fugit::{HertzU32, MegahertzU32, RateExtU32 as _};

use crate::{
    clock::{CLOCK_STATUS, ClockConfig, ClockStatus, HocoFrequency, PllConfig, SystemClockSource},
    pac::{
        self,
        fcache::regs::Flwt,
        system::{
            regs::Pllccr,
            vals::{
                Bck, Bclkdiv, Cksel, Fck, Hcfrq0, Hcstp, Ick, Opcm, Pcka, Pckb, Pckc, Pckd, Plidiv,
                Pllmul, Plsrcsel, Sodrv,
            },
        },
    },
    write_protect::ProtectedPeripheral as _,
};

// Max PLL output is 200 MHz
const PLL_INPUT_MIN: HertzU32 = MegahertzU32::from_raw(8).convert();
const PLL_INPUT_MAX: HertzU32 = MegahertzU32::from_raw(24).convert();
const PLL_OUTPUT_MIN: HertzU32 = MegahertzU32::from_raw(120).convert();
const PLL_OUTPUT_MAX: HertzU32 = MegahertzU32::from_raw(200).convert();

// Max PLL2 output is 240 MHz
const PLL2_INPUT_MIN: HertzU32 = MegahertzU32::from_raw(8).convert();
const PLL2_INPUT_MAX: HertzU32 = MegahertzU32::from_raw(24).convert();
const PLL2_OUTPUT_MIN: HertzU32 = MegahertzU32::from_raw(120).convert();
const PLL2_OUTPUT_MAX: HertzU32 = MegahertzU32::from_raw(240).convert();

const OUTPUT_FACTOR: u32 = 10;

#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub enum PllInDiv {
    Div1,
    Div2,
    Div3,
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
pub enum PllOutMul {
    Mul10_0 = 100,
    Mul10_5 = 105,
    Mul11_0 = 110,
    Mul11_5 = 115,
    Mul12_0 = 120,
    Mul12_5 = 125,
    Mul13_0 = 130,
    Mul13_5 = 135,
    Mul14_0 = 140,
    Mul14_5 = 145,
    Mul15_0 = 150,
    Mul15_5 = 155,
    Mul16_0 = 160,
    Mul16_5 = 165,
    Mul17_0 = 170,
    Mul17_5 = 175,
    Mul18_0 = 180,
    Mul18_5 = 185,
    Mul19_0 = 190,
    Mul19_5 = 195,
    Mul20_0 = 200,
    Mul20_5 = 205,
    Mul21_0 = 210,
    Mul21_5 = 215,
    Mul22_0 = 220,
    Mul22_5 = 225,
    Mul23_0 = 230,
    Mul23_5 = 235,
    Mul24_0 = 240,
    Mul24_5 = 245,
    Mul25_0 = 250,
    Mul25_5 = 255,
    Mul26_0 = 260,
    Mul26_5 = 265,
    Mul27_0 = 270,
    Mul27_5 = 275,
    Mul28_0 = 280,
    Mul28_5 = 285,
    Mul29_0 = 290,
    Mul29_5 = 295,
    Mul30_0 = 300,
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
        }
    }
}

impl From<PllOutMul> for Pllmul {
    fn from(value: PllOutMul) -> Self {
        match value {
            PllOutMul::Mul10_0 => Self::Mul10_0,
            PllOutMul::Mul10_5 => Self::Mul10_5,
            PllOutMul::Mul11_0 => Self::Mul11_0,
            PllOutMul::Mul11_5 => Self::Mul11_5,
            PllOutMul::Mul12_0 => Self::Mul12_0,
            PllOutMul::Mul12_5 => Self::Mul12_5,
            PllOutMul::Mul13_0 => Self::Mul13_0,
            PllOutMul::Mul13_5 => Self::Mul13_5,
            PllOutMul::Mul14_0 => Self::Mul14_0,
            PllOutMul::Mul14_5 => Self::Mul14_5,
            PllOutMul::Mul15_0 => Self::Mul15_0,
            PllOutMul::Mul15_5 => Self::Mul15_5,
            PllOutMul::Mul16_0 => Self::Mul16_0,
            PllOutMul::Mul16_5 => Self::Mul16_5,
            PllOutMul::Mul17_0 => Self::Mul17_0,
            PllOutMul::Mul17_5 => Self::Mul17_5,
            PllOutMul::Mul18_0 => Self::Mul18_0,
            PllOutMul::Mul18_5 => Self::Mul18_5,
            PllOutMul::Mul19_0 => Self::Mul19_0,
            PllOutMul::Mul19_5 => Self::Mul19_5,
            PllOutMul::Mul20_0 => Self::Mul20_0,
            PllOutMul::Mul20_5 => Self::Mul20_5,
            PllOutMul::Mul21_0 => Self::Mul21_0,
            PllOutMul::Mul21_5 => Self::Mul21_5,
            PllOutMul::Mul22_0 => Self::Mul22_0,
            PllOutMul::Mul22_5 => Self::Mul22_5,
            PllOutMul::Mul23_0 => Self::Mul23_0,
            PllOutMul::Mul23_5 => Self::Mul23_5,
            PllOutMul::Mul24_0 => Self::Mul24_0,
            PllOutMul::Mul24_5 => Self::Mul24_5,
            PllOutMul::Mul25_0 => Self::Mul25_0,
            PllOutMul::Mul25_5 => Self::Mul25_5,
            PllOutMul::Mul26_0 => Self::Mul26_0,
            PllOutMul::Mul26_5 => Self::Mul26_5,
            PllOutMul::Mul27_0 => Self::Mul27_0,
            PllOutMul::Mul27_5 => Self::Mul27_5,
            PllOutMul::Mul28_0 => Self::Mul28_0,
            PllOutMul::Mul28_5 => Self::Mul28_5,
            PllOutMul::Mul29_0 => Self::Mul29_0,
            PllOutMul::Mul29_5 => Self::Mul29_5,
            PllOutMul::Mul30_0 => Self::Mul30_0,
        }
    }
}

fn pll_status(
    status: bool,
    hoco: HertzU32,
    mosc: Option<HertzU32>,
    pllccr: Pllccr,
) -> Option<HertzU32> {
    match status {
        true => {
            let input_frequency = match pllccr.plsrcsel() {
                Plsrcsel::Mosc => mosc.unwrap(),
                Plsrcsel::Hoco => hoco,
            };
            let output_mul: u32 = match pllccr.pllmul() {
                Pllmul::Mul10_0 => 100,
                Pllmul::Mul10_5 => 105,
                Pllmul::Mul11_0 => 110,
                Pllmul::Mul11_5 => 115,
                Pllmul::Mul12_0 => 120,
                Pllmul::Mul12_5 => 125,
                Pllmul::Mul13_0 => 130,
                Pllmul::Mul13_5 => 135,
                Pllmul::Mul14_0 => 140,
                Pllmul::Mul14_5 => 145,
                Pllmul::Mul15_0 => 150,
                Pllmul::Mul15_5 => 155,
                Pllmul::Mul16_0 => 160,
                Pllmul::Mul16_5 => 165,
                Pllmul::Mul17_0 => 170,
                Pllmul::Mul17_5 => 175,
                Pllmul::Mul18_0 => 180,
                Pllmul::Mul18_5 => 185,
                Pllmul::Mul19_0 => 190,
                Pllmul::Mul19_5 => 195,
                Pllmul::Mul20_0 => 200,
                Pllmul::Mul20_5 => 205,
                Pllmul::Mul21_0 => 210,
                Pllmul::Mul21_5 => 215,
                Pllmul::Mul22_0 => 220,
                Pllmul::Mul22_5 => 225,
                Pllmul::Mul23_0 => 230,
                Pllmul::Mul23_5 => 235,
                Pllmul::Mul24_0 => 240,
                Pllmul::Mul24_5 => 245,
                Pllmul::Mul25_0 => 250,
                Pllmul::Mul25_5 => 255,
                Pllmul::Mul26_0 => 260,
                Pllmul::Mul26_5 => 265,
                Pllmul::Mul27_0 => 270,
                Pllmul::Mul27_5 => 275,
                Pllmul::Mul28_0 => 280,
                Pllmul::Mul28_5 => 285,
                Pllmul::Mul29_0 => 290,
                Pllmul::Mul29_5 => 295,
                Pllmul::Mul30_0 => 300,
                _ => unimplemented!(),
            };

            let input_div: u32 = match pllccr.plidiv() {
                Plidiv::Div1 => 1,
                Plidiv::Div2 => 2,
                Plidiv::Div3 => 3,
                _ => unimplemented!(),
            };

            let output_factor = 10;

            Some(((input_frequency / input_div) * output_mul) / output_factor)
        }
        false => None,
    }
}

fn calc_pll_frequency(
    pll_config: &PllConfig,
    config: &ClockConfig,
    input_range: [HertzU32; 2],
    output_range: [HertzU32; 2],
) -> HertzU32 {
    let output_mul: u32 = pll_config.mul as u32;

    let input_div: u32 = match pll_config.div {
        PllInDiv::Div1 => 1,
        PllInDiv::Div2 => 2,
        PllInDiv::Div3 => 3,
    };

    let pll_input_frequency = match pll_config.input {
        PllInput::Hoco => config.hoco.into(),
        PllInput::Mosc => config.mosc.unwrap(),
    } / input_div;

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

    pll_output
}

pub(crate) fn init(config: ClockConfig) -> Result<(), ()> {
    let system = pac::SYSTEM;

    debug!("HOCO: status={}", system.hococr().read());

    // Sanity check PLL config
    let pll_frequency = match config.pll {
        Some(ref pll_config) => Some(calc_pll_frequency(
            pll_config,
            &config,
            [PLL_INPUT_MIN, PLL_INPUT_MAX],
            [PLL_OUTPUT_MIN, PLL_OUTPUT_MAX],
        )),
        None => None,
    };
    let _pll2_frequency = match config.pll2 {
        Some(ref pll_config) => Some(calc_pll_frequency(
            pll_config,
            &config,
            [PLL2_INPUT_MIN, PLL2_INPUT_MAX],
            [PLL2_OUTPUT_MIN, PLL2_OUTPUT_MAX],
        )),
        None => None,
    };

    // TODO: Validate MOSC

    system.protected_write(|| {
        if config.sosc {
            system.somcr().modify(|r| r.set_sodrv(Sodrv::Normal));
            system.sosccr().modify(|r| r.set_sostp(false));
            // TODO: Set oscillator stabilization time.
            while system.sosccr().read().sostp() {}
        }

        // Writing to the HOCOCR2 is prohibited when the HOCOCR.HCSTP bit is 0 (the HOCO operates).
        system.hococr().write(|r| r.set_hcstp(Hcstp::Stop));

        let fll_magic = match config.hoco {
            HocoFrequency::_16mhz => crate::constants::FLL_16MHZ,
            HocoFrequency::_18mhz => crate::constants::FLL_18MHZ,
            HocoFrequency::_20mhz => crate::constants::FLL_20MHZ,
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
            });

            system.pllcr().write(|r| r.set_pllstp(false));
            while !system.oscsf().read().pllsf() {}
        }

        system.pll2cr().write(|r| r.set_pll2stp(true));
        while system.oscsf().read().pll2sf() {}

        if let Some(pll_config) = config.pll2.as_ref() {
            system.pll2ccr().modify(|r| {
                r.set_plsrcsel(pll_config.input.into());
                r.set_plidiv(pll_config.div.into());
                r.set_pllmul(pll_config.mul.into());
            });

            system.pll2cr().write(|r| r.set_pll2stp(false));
            while !system.oscsf().read().pll2sf() {}
        }

        // High speed mode needed for ƒICLK > 1 MHz. Table 53.14
        // In theory we're in HS mode after reset (Table 10.4)
        trace!("Setting high speed mode on");
        system.opccr().write(|r| r.set_opcm(Opcm::HighSpeed));

        while system.opccr().read().opcmtsf() {}

        let master_frequency;
        match config.system {
            SystemClockSource::Mosc => todo!(),
            SystemClockSource::Sosc => todo!(),
            SystemClockSource::Hoco => todo!(),
            SystemClockSource::Moco => todo!(),
            SystemClockSource::Loco => todo!(),
            SystemClockSource::Pll => {
                // When the PLL is selected as the clock source, MSTPmi bits must be changed only one bit
                // at a time. In this case, wait at least 250 ns after changing each MSTP bit (§ 10.4).
                let mstp = pac::MSTP;
                mstp.mstpcrb().modify(|r| r.set_mstpb12(true));
                for _ in 0..100 {
                    asm::nop()
                }
                mstp.mstpcrc().modify(|r| r.set_mstpc31(true));
                for _ in 0..100 {
                    asm::nop()
                }
                master_frequency =
                    pll_frequency.expect("Pll selected as clock source but not configured");
            }
        };

        // TODO: This needs some love to be more flexible.
        // For now we assume that we want to run at top speed.
        let (ick_div, pcka_div, pckb_div, pckc_div, pckd_div, fck_div, bck_div) = {
            let ick_div = Ick::Div1;
            let _50mhz = master_frequency.to_Hz().div_ceil(50_000_000);
            let _100mhz = master_frequency.to_Hz().div_ceil(100_000_000);

            let (pckb_div, pckc_div, fck_div, bck_div) = match _50mhz {
                1 => (Pckb::Div1, Pckc::Div1, Fck::Div1, Bck::Div1),
                2 => (Pckb::Div2, Pckc::Div2, Fck::Div2, Bck::Div2),
                3..=4 => (Pckb::Div4, Pckc::Div4, Fck::Div4, Bck::Div4),
                5..=8 => (Pckb::Div8, Pckc::Div8, Fck::Div8, Bck::Div8),
                9..=16 => (Pckb::Div16, Pckc::Div16, Fck::Div16, Bck::Div16),
                17..=32 => (Pckb::Div32, Pckc::Div32, Fck::Div32, Bck::Div32),
                33..=64 => (Pckb::Div64, Pckc::Div64, Fck::Div64, Bck::Div64),
                _ => unimplemented!(),
            };
            let (pcka_div, pckd_div) = match _100mhz {
                1 => (Pcka::Div1, Pckd::Div1),
                2 => (Pcka::Div2, Pckd::Div2),
                3..=4 => (Pcka::Div4, Pckd::Div4),
                5..=8 => (Pcka::Div8, Pckd::Div8),
                9..=16 => (Pcka::Div16, Pckd::Div16),
                17..=32 => (Pcka::Div32, Pckd::Div32),
                33..=64 => (Pcka::Div64, Pckd::Div64),
                _ => unimplemented!(),
            };

            (
                ick_div, pcka_div, pckb_div, pckc_div, pckd_div, fck_div, bck_div,
            )
        };

        let ick_frq = match ick_div {
            Ick::Div1 => master_frequency / 1,
            Ick::Div2 => master_frequency / 2,
            Ick::Div4 => master_frequency / 4,
            Ick::Div8 => master_frequency / 8,
            Ick::Div16 => master_frequency / 16,
            Ick::Div32 => master_frequency / 32,
            Ick::Div64 => master_frequency / 64,
            Ick::_RESERVED_7 => todo!(),
        };

        let bck_frq = match bck_div {
            Bck::Div1 => master_frequency / 1,
            Bck::Div2 => master_frequency / 2,
            Bck::Div4 => master_frequency / 4,
            Bck::Div8 => master_frequency / 8,
            Bck::Div16 => master_frequency / 16,
            Bck::Div32 => master_frequency / 32,
            Bck::Div64 => master_frequency / 64,
            Bck::_RESERVED_7 => todo!(),
        };

        let fcache = pac::FCACHE;
        if ick_frq < 50.MHz::<1, 1>() {
            fcache.flwt().write_value(Flwt(0));
        } else if ick_frq < 100.MHz::<1, 1>() {
            fcache.flwt().write_value(Flwt(1));
        } else if ick_frq < 150.MHz::<1, 1>() {
            fcache.flwt().write_value(Flwt(2));
        } else {
            fcache.flwt().write_value(Flwt(3));
        }

        system.sckdivcr().modify(|r| {
            r.set_ick(ick_div);
            r.set_pcka(pcka_div);
            r.set_pckb(pckb_div);
            r.set_pckc(pckc_div);
            r.set_pckd(pckd_div);
            r.set_fck(fck_div);
            r.set_bck(bck_div);
        });

        if bck_frq > 50.MHz::<1, 1>() {
            system.bckcr().write(|r| r.set_bclkdiv(Bclkdiv::Div2));
        } else {
            system.bckcr().write(|r| r.set_bclkdiv(Bclkdiv::Div1));
        }

        match config.system {
            SystemClockSource::Mosc => todo!(),
            SystemClockSource::Sosc => {
                assert_eq!(config.sosc, true);
                system.sckscr().modify(|r| r.set_cksel(Cksel::Sosc));
            }
            SystemClockSource::Hoco => {
                system.sckscr().modify(|r| r.set_cksel(Cksel::Hoco));
            }
            SystemClockSource::Moco => todo!(),
            SystemClockSource::Loco => todo!(),
            SystemClockSource::Pll => {
                // § 8.2.3 When changing the value from a non-PLL clock source to the PLL, wait at least
                // 250 ns after changing the value, before starting subsequent processing.
                system.sckscr().modify(|r| r.set_cksel(Cksel::Pll));
                for _ in 0..100 {
                    asm::nop();
                }
            }
        }
    });

    {
        let system = pac::SYSTEM;
        let hoco = system.hococr2().read().hcfrq0();
        let hoco: HertzU32 = match hoco {
            Hcfrq0::_16mhz => 16_u32.MHz(),
            Hcfrq0::_18mhz => 18_u32.MHz(),
            Hcfrq0::_20mhz => 20_u32.MHz(),
            _ => unimplemented!(),
        };

        let pll_running = !system.pllcr().read().pllstp();
        let pll = pll_status(pll_running, hoco, config.mosc, system.pllccr().read());
        let pll2_running = !system.pll2cr().read().pll2stp();
        let pll2 = pll_status(pll2_running, hoco, config.mosc, system.pll2ccr().read());

        let cksel = system.sckscr().read().cksel();
        let master = match cksel {
            Cksel::Hoco => hoco,
            Cksel::Moco => 8_u32.MHz(),
            Cksel::Loco => 32_768_u32.Hz(),
            Cksel::Sosc => 32_768_u32.Hz(),
            Cksel::Pll => pll.unwrap(),
            _ => unimplemented!(),
        };

        let prescaler = system.sckdivcr().read();

        let iclk = match prescaler.ick() {
            Ick::Div1 => master,
            Ick::Div2 => master / 2,
            Ick::Div4 => master / 4,
            Ick::Div8 => master / 8,
            Ick::Div16 => master / 16,
            Ick::Div32 => master / 32,
            Ick::Div64 => master / 64,
            Ick::_RESERVED_7 => unimplemented!("Invalid sckdivcr.ick"),
        };

        let flash = match prescaler.fck() {
            Fck::Div1 => master,
            Fck::Div2 => master / 2,
            Fck::Div4 => master / 4,
            Fck::Div8 => master / 8,
            Fck::Div16 => master / 16,
            Fck::Div32 => master / 32,
            Fck::Div64 => master / 64,
            Fck::_RESERVED_7 => unimplemented!("Invalid sckdivcr.fck"),
        };

        let peripheral_a = match prescaler.pcka() {
            Pcka::Div1 => master,
            Pcka::Div2 => master / 2,
            Pcka::Div4 => master / 4,
            Pcka::Div8 => master / 8,
            Pcka::Div16 => master / 16,
            Pcka::Div32 => master / 32,
            Pcka::Div64 => master / 64,
            Pcka::_RESERVED_7 => unimplemented!("Invalid sckdivcr.pcka"),
        };

        let peripheral_b = match prescaler.pckb() {
            Pckb::Div1 => master,
            Pckb::Div2 => master / 2,
            Pckb::Div4 => master / 4,
            Pckb::Div8 => master / 8,
            Pckb::Div16 => master / 16,
            Pckb::Div32 => master / 32,
            Pckb::Div64 => master / 64,
            Pckb::_RESERVED_7 => unimplemented!("Invalid sckdivcr.pckb"),
        };

        let peripheral_c = match prescaler.pckc() {
            Pckc::Div1 => master,
            Pckc::Div2 => master / 2,
            Pckc::Div4 => master / 4,
            Pckc::Div8 => master / 8,
            Pckc::Div16 => master / 16,
            Pckc::Div32 => master / 32,
            Pckc::Div64 => master / 64,
            Pckc::_RESERVED_7 => unimplemented!("Invalid sckdivcr.pckc"),
        };

        let peripheral_d = match prescaler.pckd() {
            Pckd::Div1 => master,
            Pckd::Div2 => master / 2,
            Pckd::Div4 => master / 4,
            Pckd::Div8 => master / 8,
            Pckd::Div16 => master / 16,
            Pckd::Div32 => master / 32,
            Pckd::Div64 => master / 64,
            Pckd::_RESERVED_7 => unimplemented!("Invalid sckdivcr.pckd"),
        };

        let bus_clock = match prescaler.bck() {
            Bck::Div1 => master,
            Bck::Div2 => master / 2,
            Bck::Div4 => master / 4,
            Bck::Div8 => master / 8,
            Bck::Div16 => master / 16,
            Bck::Div32 => master / 32,
            Bck::Div64 => master / 64,
            Bck::_RESERVED_7 => unimplemented!("Invalid sckdivcr.bck"),
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
                bus_clock,
            })
            .or(Err(()))
    }
}

impl Default for ClockConfig {
    /// Config assumes EK-RA6M5.
    fn default() -> Self {
        Self {
            system: SystemClockSource::Pll,
            hoco: HocoFrequency::_20mhz,
            mosc: Some(24.MHz()),
            sosc: true,
            // 200 MHz output
            pll: Some(PllConfig {
                input: PllInput::Hoco,
                div: PllInDiv::Div1,
                mul: PllOutMul::Mul10_0,
            }),
            pll2: None,
        }
    }
}
