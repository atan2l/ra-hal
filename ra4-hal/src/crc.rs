#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
use embassy_hal_internal::Peri;
use ra4m1_ctpac::crc::{regs::CrcdirBy, vals::Gps};

use crate::{pac, peripherals::CRC};

pub struct Config {}

pub struct Crc<'d> {
    _peri: Peri<'d, CRC>,
    _config: Config,
}

impl<'d> Crc<'d> {
    pub fn new(peri: Peri<'d, CRC>) -> Self {
        let mstp = pac::MSTP;
        mstp.mstpcrc().write(|w| {
            w.set_mstpc1(false);
        });

        let mut instance = Self {
            _peri: peri,
            _config: Config {},
        };

        instance.reset();
        instance
    }

    pub fn reset(&mut self) {
        let crc = crate::pac::CRC;

        crc.crccr0().write(|w| {
            w.set_gps(Gps::Crc16);
            w.set_dorclr(true);
        });
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
