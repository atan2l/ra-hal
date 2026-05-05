//! RA4L1 specific clock configuration.

use core::ops::RangeInclusive;

use fugit::{HertzU32, KilohertzU32, MegahertzU32, RateExtU32 as _};

use crate::{
    clock::{CLOCK_STATUS, ClockConfig, ClockStatus, HocoFrequency, PllConfig, SystemClockSource},
    pac::{
        self,
        system::vals::{
            Cksel, Fck, Hcfrq0, Hcstp, Ick, Opcm, Pcka, Pckb, Pckc, Pckd, Plidiv, Pllmul, Plsrcsel,
            Sodrv,
        },
    },
    write_protect::ProtectedPeripheral as _,
};

// Table 8.1
const PLL_INPUT_RANGE: RangeInclusive<HertzU32> = RangeInclusive::new(
    KilohertzU32::from_raw(4000).convert(),
    KilohertzU32::from_raw(125000).convert(),
);

// Table 8.1
const PLL_OUTPUT_RANGE: RangeInclusive<HertzU32> = RangeInclusive::new(
    MegahertzU32::from_raw(24).convert(),
    MegahertzU32::from_raw(80).convert(),
);

const OUTPUT_FACTOR: u32 = 10;

/// Input source for PLL or PLL2.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone)]
pub enum PllInput {
    /// PLL derives from HOCO.
    Hoco,

    /// PLL derives from Main on-chip Oscillator (if equipped).
    Mosc,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum PllInDiv {
    Div1,
    Div4,
    Div6,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum PllOutMul {
    Mul4_0 = 40,
    Mul4_5 = 45,
    Mul5_0 = 50,
    Mul5_5 = 55,
    Mul6_0 = 60,
    Mul6_5 = 65,
    Mul7_0 = 70,
    Mul7_5 = 75,
    Mul8_0 = 80,
    Mul8_5 = 85,
    Mul9_0 = 90,
    Mul9_5 = 95,
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
}

// Max PLL output is 80 MHz

impl Default for ClockConfig {
    /// Config assumes EK-RA4L1.
    fn default() -> Self {
        Self {
            system: SystemClockSource::Hoco,
            hoco: HocoFrequency::_80mhz,
            mosc: Some(8_u32.MHz()),
            sosc: true,
            pll: Some(PllConfig {
                input: PllInput::Mosc,
                div: PllInDiv::Div1,
                mul: PllOutMul::Mul6_0,
            }),
        }
    }
}

fn calc_pll_frequency(
    pll_config: &PllConfig,
    config: &ClockConfig,
    input_range: RangeInclusive<HertzU32>,
    output_range: RangeInclusive<HertzU32>,
) -> HertzU32 {
    let output_mul: u32 = pll_config.mul as u32;

    let input_div: u32 = match pll_config.div {
        PllInDiv::Div1 => 1,
        PllInDiv::Div4 => 4,
        PllInDiv::Div6 => 6,
    };

    let pll_input_frequency = match pll_config.input {
        PllInput::Hoco => config.hoco.into(),
        PllInput::Mosc => config.mosc.unwrap(),
    } / input_div;

    assert!(
        input_range.contains(&pll_input_frequency),
        "Invalid PLL configuration, input ({pll_input_frequency}) not within range."
    );

    let pll_output = ((pll_input_frequency / input_div) * output_mul) / OUTPUT_FACTOR;

    assert!(
        output_range.contains(&pll_output),
        "Invalid PLL configuration, output ({pll_output}) not within range."
    );

    pll_output
}

// HOCO: No FLL
// PLL: Can drive root clock
// PLL: Source = MOSC or HOCO
pub(crate) fn init(config: ClockConfig) -> Result<(), ()> {
    // See tables: 8.4, 8.5, 8.6

    let system = pac::SYSTEM;
    let flcn = pac::FLCN;

    debug!("HOCO: status={}", system.hococr().read());

    // Sanity check PLL config
    let _pll_frequency = match config.pll {
        Some(ref pll_config) => Some(calc_pll_frequency(
            pll_config,
            &config,
            PLL_INPUT_RANGE,
            PLL_OUTPUT_RANGE,
        )),
        None => None,
    };

    system.protected_write(|| {
        // Writing to the HOCOCR2 is prohibited when the HOCOCR.HCSTP bit is 0 (the HOCO operates).
        let should_configure_hoco = match system.hococr().read().hcstp() {
            Hcstp::Start => {
                let frq = system.hococr2().read().hcfrq0();
                let should_configure = frq != config.hoco.into();
                if should_configure {
                    system.hococr().write(|w| w.set_hcstp(Hcstp::Stop));
                }
                should_configure
            }
            Hcstp::Stop => true,
        };

        if should_configure_hoco {
            system.hococr2().write(|r| r.set_hcfrq0(config.hoco.into()));
            system.hococr().write(|w| w.set_hcstp(Hcstp::Start));
        }

        if config.mosc.is_some() {
            system.mosccr().modify(|r| r.set_mostp(false));
            while !system.oscsf().read().moscsf() {}
        }

        if config.sosc {
            system.somcr().modify(|r| r.set_sodrv(Sodrv::_00));
            system.sosccr().modify(|r| r.set_sostp(false));
            // TODO: Set oscillator stabilization time.
            while system.sosccr().read().sostp() {}
        }

        let hoco_freq = system.hococr2().read().hcfrq0();

        if let Some(pll_config) = &config.pll {
            let pll_src = match pll_config.input {
                PllInput::Hoco => Plsrcsel::Hoco,
                PllInput::Mosc => Plsrcsel::Mosc,
            };

            system.pllcr().modify(|r| r.set_pllstp(true));
            while system.oscsf().read().pllsf() {}

            match pll_src {
                Plsrcsel::Mosc => {
                    assert!(
                        system.oscsf().read().moscsf(),
                        "PLL source = MOSC, but MOSC is not stabilized"
                    );
                }
                Plsrcsel::Hoco => {
                    assert!(
                        system.oscsf().read().hocosf(),
                        "PLL source = HOCO, but HOCO is not stabilized"
                    );
                }
            }

            // TODO: Don't hardcode DIV + MUL
            system.pllccr().modify(|r| {
                r.set_plsrcsel(pll_src);
                r.set_plidiv(Plidiv::Div1);
                r.set_pllmul(Pllmul::Mul6_0);
            });

            system.pllcr().modify(|r| r.set_pllstp(false));
            while !system.oscsf().read().pllsf() {}
        }

        // High speed mode needed for ƒICLK > 8 MHz.
        // Table 49.23
        trace!("Setting high speed mode on");
        system.opccr().write(|w| w.set_opcm(Opcm::HighSpeed));

        while system.opccr().read().opcmtsf() {}

        assert_eq!(
            config.system,
            SystemClockSource::Hoco,
            "TODO: SystemClockSource != HOCO"
        );

        match hoco_freq {
            Hcfrq0::_80mhz => {
                system.memwait().write(|r| r.set_memwait(2));
                flcn.fldwaitr().write(|r| r.set_fldwait1(true));
                system.sckdivcr().modify(|r| {
                    r.set_ick(Ick::Div1);
                    r.set_pcka(Pcka::Div1);
                    r.set_pckb(Pckb::Div2);
                    r.set_pckc(Pckc::Div2);
                    r.set_pckd(Pckd::Div1);
                    r.set_fck(Fck::Div2);
                });
            }
            Hcfrq0::_64mhz => {
                system.memwait().write(|r| r.set_memwait(2));
                flcn.fldwaitr().write(|r| r.set_fldwait1(true));
            }
            Hcfrq0::_48mhz => {
                system.memwait().write(|r| r.set_memwait(1));
                flcn.fldwaitr().write(|r| r.set_fldwait1(true));
                system.sckdivcr().modify(|r| {
                    r.set_ick(Ick::Div1);
                    r.set_pcka(Pcka::Div1);
                    r.set_pckb(Pckb::Div2);
                    r.set_pckc(Pckc::Div1);
                    r.set_pckd(Pckd::Div1);
                    r.set_fck(Fck::Div1);
                });
            }
            Hcfrq0::_40mhz => {
                system.memwait().write(|r| r.set_memwait(1));
                flcn.fldwaitr().write(|r| r.set_fldwait1(true));
            }
            Hcfrq0::_32mhz => {
                system.memwait().write(|r| r.set_memwait(0));
                flcn.fldwaitr().write(|r| r.set_fldwait1(false));
            }
            Hcfrq0::_24mhz => {
                system.memwait().write(|r| r.set_memwait(0));
                flcn.fldwaitr().write(|r| r.set_fldwait1(false));
            }
            _ => unreachable!(),
        }
        system.sckscr().write(|w| w.set_cksel(Cksel::Hoco));
    });

    {
        let system = pac::SYSTEM;

        let hoco = system.hococr2().read().hcfrq0();
        let hoco: HertzU32 = match hoco {
            Hcfrq0::_24mhz => 24_u32.MHz(),
            Hcfrq0::_32mhz => 32_u32.MHz(),
            Hcfrq0::_40mhz => 40_u32.MHz(),
            Hcfrq0::_48mhz => 48_u32.MHz(),
            Hcfrq0::_64mhz => 64_u32.MHz(),
            Hcfrq0::_80mhz => 80_u32.MHz(),
            _ => unimplemented!(),
        };

        let cksel = system.sckscr().read().cksel();
        let master = match cksel {
            Cksel::Hoco => hoco,
            Cksel::Moco => 8_u32.MHz(),
            Cksel::Loco => 32_768_u32.Hz(),
            Cksel::Sosc => 32_768_u32.Hz(),
            _ => 0.Hz(),
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

        let pll_running = !system.pllcr().read().pllstp();
        let pll = match pll_running {
            true => {
                let pllccr = system.pllccr().read();
                let input_frequency = match pllccr.plsrcsel() {
                    Plsrcsel::Mosc => config
                        .mosc
                        .expect("MOSC selected as PLL source but not configured"),
                    Plsrcsel::Hoco => hoco,
                };
                let output_mul: u32 = match pllccr.pllmul() {
                    Pllmul::Mul4_0 => 40,
                    Pllmul::Mul4_5 => 45,
                    Pllmul::Mul5_0 => 50,
                    Pllmul::Mul5_5 => 55,
                    Pllmul::Mul6_0 => 60,
                    Pllmul::Mul6_5 => 65,
                    Pllmul::Mul7_0 => 70,
                    Pllmul::Mul7_5 => 75,
                    Pllmul::Mul8_0 => 80,
                    Pllmul::Mul8_5 => 85,
                    Pllmul::Mul9_0 => 90,
                    Pllmul::Mul9_5 => 95,
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
                    _ => unimplemented!(),
                };

                let input_div: u32 = match pllccr.plidiv() {
                    Plidiv::Div1 => 1,
                    Plidiv::Div4 => 4,
                    Plidiv::Div6 => 6,
                    _ => unimplemented!(),
                };
                let output_factor = 10;

                Some(((input_frequency / input_div) * output_mul) / output_factor)
            }
            false => None,
        };

        CLOCK_STATUS
            .init(ClockStatus {
                master,
                hoco,
                system: iclk,
                flash,
                peripheral_a,
                peripheral_b,
                peripheral_c,
                peripheral_d,
                pll,
                sosc: config.sosc,
                mosc: config.mosc,
            })
            .or(Err(()))
    }
}
