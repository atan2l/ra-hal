//! Cyclic Redundancy Check Calculator (`CRC`).
//!
//! # Notes
//! * The driver will turn the module off (`MSTPC1=1`) when it is dropped.

use embassy_hal_internal::Peri;

use crate::{module_stop::ModuleStop, pac::crc::vals::Gps, peripherals::CRC};

/// Polynomial to use for CRC calculation.
#[derive(Default, Copy, Clone, PartialEq)]
pub enum Polynomial {
    /// This will panic
    None,

    /// CRC-8, polynomial = `0x07`
    Crc8,

    /// CRC-16, polynomial = `0x8005`
    #[default]
    Crc16,

    /// CRC-16/CCIT, polynomial = `0x1021`
    CrcCcit,

    /// CRC-32, polynomial = `0x04C11DB7`
    Crc32,

    /// CRC-32C, polynomial = `0x1EDC6F41`
    Crc32C,
}

/// CRC Configuration
#[derive(Default)]
pub struct Config {
    #[allow(missing_docs)]
    pub polynomial: Polynomial,
    /// Calculate CRC by shifting right if true.
    /// CRC Calculation Switching in Renesas-speak, §32.2.1.
    pub reverse: bool,
    /// Initial output value.
    pub seed: u32,
    /// Apply a bitwise NOT operation to the result if true.
    pub reflect_output: bool,
}

/// `CRC` driver.
pub struct Crc<'d> {
    _peri: Peri<'d, CRC>,
    config: Config,
}

impl<'d> Crc<'d> {
    /// Consumes the `CRC` peripheral and returns a driver initialized with the provided values.
    ///
    /// # Arguments
    /// * `peri` The [`CRC`] peripheral.
    /// * `config` Algorithm configuration.
    ///
    /// # Returns
    ///
    /// A `CRC` driver.
    pub fn new(peri: Peri<'d, CRC>, config: Config) -> Self {
        CRC::start_module();

        let mut instance = Self {
            _peri: peri,
            config,
        };

        instance.reset();
        instance
    }

    /// Applies a new algorithm configuration and resets the state.
    ///
    /// # Arguments
    /// * `config` Algorithm configuration to apply.
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
        self.reset();
    }

    /// Applies the current configuration and initializes the output buffer with the selected seed.
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

        let lms = !self.config.reverse;

        crc.crccr0().write(|w| {
            w.set_gps(gps);
            w.set_lms(lms);
            w.set_dorclr(true);
        });

        if self.config.seed != 0 {
            match self.config.polynomial {
                Polynomial::None => unimplemented!(),
                Polynomial::Crc8 => {
                    crc.crcdor_by().write_value((self.config.seed & 0xFF) as u8);
                }
                Polynomial::Crc16 | Polynomial::CrcCcit => {
                    crc.crcdor_ha()
                        .write_value((self.config.seed & 0xFFFF) as u16);
                }
                Polynomial::Crc32 | Polynomial::Crc32C => {
                    crc.crcdor().write_value(self.config.seed);
                }
            }
        }
    }

    /// Computes a CRC value for given input.
    ///
    /// # Arguments
    /// * `bytes` Input data
    ///
    /// # Notes
    /// * CRC-32 / CRC-32C require 32-bit input values.
    ///   If a 32-bit polynomial is selected and the provided input is not a multiple of `4` bytes the function will panic.
    /// * TODO: implement a function that takes a `&[u32]` slice and writes to the 32-bit data register.
    pub fn feed_bytes(&mut self, bytes: &[u8]) {
        let crc = crate::pac::CRC;

        match self.config.polynomial {
            Polynomial::Crc8 | Polynomial::Crc16 | Polynomial::CrcCcit => {
                for byte in bytes.iter() {
                    crc.crcdir_by().write_value(*byte);
                }
            }
            Polynomial::Crc32 | Polynomial::Crc32C => {
                if !bytes.len().is_multiple_of(4) {
                    unimplemented!("CRC-32 input len must be a multiple of 4");
                }

                match self.config.reverse {
                    false => {
                        for chunk in bytes.chunks_exact(4) {
                            let word = u32::from_be_bytes(chunk.try_into().unwrap());
                            crc.crcdir().write_value(word);
                        }
                    }
                    true => {
                        for chunk in bytes.chunks_exact(4) {
                            let word = u32::from_ne_bytes(chunk.try_into().unwrap());
                            crc.crcdir().write_value(word);
                        }
                    }
                }
            }
            Polynomial::None => unimplemented!(),
        };
    }

    /// Returns the CRC calculated from the data inputted.
    pub fn read(&mut self) -> u32 {
        let crc = crate::pac::CRC;

        let output = match self.config.polynomial {
            Polynomial::None => unimplemented!(),
            Polynomial::Crc8 | Polynomial::Crc16 | Polynomial::CrcCcit => {
                if self.config.polynomial == Polynomial::Crc8 {
                    crc.crcdor_by().read() as _
                } else {
                    crc.crcdor_ha().read() as _
                }
            }
            Polynomial::Crc32 | Polynomial::Crc32C => crc.crcdor().read(),
        };

        if self.config.reflect_output {
            let mask = match self.config.polynomial {
                Polynomial::Crc8 => u8::MAX as _,
                Polynomial::Crc16 | Polynomial::CrcCcit => u16::MAX as _,
                Polynomial::Crc32 | Polynomial::Crc32C => u32::MAX,
                _ => unimplemented!(),
            };
            (!output) & mask
        } else {
            output
        }
    }
}

impl<'d> Drop for Crc<'d> {
    /// Turns off `CRC` and sets `MSTPC1=true`.
    fn drop(&mut self) {
        CRC::stop_module();
    }
}
