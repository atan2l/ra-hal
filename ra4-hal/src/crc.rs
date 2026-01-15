#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
use embassy_hal_internal::Peri;
use ra4m1_ctpac::crc::{
    regs::{CrcdirBy, Crcdor, CrcdorBy, CrcdorHa},
    vals::Gps,
};

use crate::{pac, peripherals::CRC};

#[derive(Default)]
pub enum Polynomial {
    /// This will panic
    None,

    Crc8,

    #[default]
    Crc16,

    CrcCcit,

    Crc32,

    Crc32C,
}

#[derive(Default)]
pub enum Endian {
    /// MSB first
    Big,

    /// LSB first
    #[default]
    Little,
}

#[derive(Default)]
pub struct Config {
    pub polynomial: Polynomial,
    pub endian: Endian,
    pub seed: u32,
}

pub struct Crc<'d> {
    _peri: Peri<'d, CRC>,
    config: Config,
}

impl<'d> Crc<'d> {
    pub fn new(peri: Peri<'d, CRC>, config: Config) -> Self {
        debug!("CRC: stop=false");

        let mstp = pac::MSTP;

        mstp.mstpcrc().write(|w| {
            w.set_mstpc1(false);
        });

        let mut instance = Self {
            _peri: peri,
            config,
        };

        instance.reset();
        instance
    }

    /// Note: this will reset the state
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
        self.reset();
    }

    pub fn reset(&mut self) {
        let crc = crate::pac::CRC;

        let gps = match self.config.polynomial {
            Polynomial::None => Gps::None,
            Polynomial::Crc8 => Gps::Crc8,
            Polynomial::Crc16 => Gps::Crc16,
            Polynomial::CrcCcit => Gps::CrcCcit,
            Polynomial::Crc32 => Gps::Crc32,
            Polynomial::Crc32C => Gps::Crc32C,
        };

        let lms = match self.config.endian {
            Endian::Big => true,
            Endian::Little => false,
        };

        info!("New endian: {}", lms);

        crc.crccr0().write(|w| {
            w.set_gps(gps);
            w.set_lms(lms);
            w.set_dorclr(true);
        });

        if self.config.seed != 0 {
            info!("Setting seed to: {:08X}", self.config.seed);
            match self.config.polynomial {
                Polynomial::None => unimplemented!(),
                Polynomial::Crc8 => {
                    crc.crcdor_by()
                        .write_value(CrcdorBy((self.config.seed & 0xFF) as u8));
                }
                Polynomial::Crc16 | Polynomial::CrcCcit => {
                    crc.crcdor_ha()
                        .write_value(CrcdorHa((self.config.seed & 0xFFFF) as u16));
                }
                Polynomial::Crc32 | Polynomial::Crc32C => {
                    crc.crcdor().write_value(Crcdor(self.config.seed));
                }
            }
        }
    }

    pub fn feed_bytes(&mut self, bytes: &[u8]) -> u32 {
        let crc = crate::pac::CRC;

        let algo = crc.crccr0().read().gps();

        match algo {
            Gps::None | Gps::_RESERVED_6 | Gps::_RESERVED_7 => unimplemented!(),
            Gps::Crc8 | Gps::Crc16 | Gps::CrcCcit => {
                for byte in bytes.iter() {
                    crc.crcdir_by().write_value(CrcdirBy(*byte));
                }

                if algo == Gps::Crc8 {
                    crc.crcdor_by().read().0 as _
                } else {
                    crc.crcdor_ha().read().0 as _
                }
            }
            Gps::Crc32 | Gps::Crc32C => {
                //
                todo!()
            }
        }
    }
}

impl<'d> Drop for Crc<'d> {
    fn drop(&mut self) {
        debug!("CRC: stop=true");

        let mstp = pac::MSTP;

        mstp.mstpcrc().write(|w| {
            w.set_mstpc1(true);
        });
    }
}
