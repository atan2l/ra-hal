//! `FMIFRT` Factory MCU Information Flash Root Table
//!
//! This is all read only so we don't care all that much about ownership of the peripheral.

use core::mem::transmute;

use cfg_if::cfg_if;
#[allow(unused)]
use defmt::{debug, error, info, trace, warn};

use crate::pac;

/// Contains information about the MCU that was programmed at the factory.
pub struct McuInfo {
    uid: u128,
    part_number: [u8; 16],
    rev: u8,
}

impl McuInfo {
    const PN_LEN: usize = 13;

    /// Returns the part number as a string.
    pub fn part_number(&self) -> &str {
        core::str::from_utf8(&self.part_number).unwrap().trim()
    }

    /// Returns the unique 128-bit identifier associated with the MCU.
    #[inline]
    pub fn uid(&self) -> u128 {
        self.uid
    }

    /// Returns the revision of the MCU.  Higher values are newer.
    #[inline]
    pub fn revision(&self) -> u8 {
        self.rev
    }

    /// Returns `true` if the part number was log enough to be parsed.
    /// If `false` is returned neither `flash_size` nor `pin_count` will work.
    pub fn ok(&self) -> bool {
        self.part_number().len() >= Self::PN_LEN
    }

    /// Returns the amount of on-die flash memory in kilobytes or `None` if this cannot be determined.
    pub fn flash_size(&self) -> Option<u16> {
        match self.part_number[8] {
            b'9' => Some(128),
            b'B' => Some(256),
            b'C' => Some(384),
            b'D' => Some(512),
            b'E' => Some(768),
            b'F' => Some(1024),
            _ => None,
        }
    }

    /// Returns the number of pins attached to the MCU, `None` if this cannot be determined.
    pub fn pin_count(&self) -> Option<u8> {
        match &self.part_number[11..=12] {
            b"FB" | b"BM" => Some(144),
            b"FP" | b"LJ" => Some(100),
            b"NB" | b"BQ" | b"BB" | b"FM" => Some(64),
            b"NG" => Some(56),
            b"NE" | b"FL" => Some(48),
            b"NF" => Some(40),
            b"BC" => Some(36),
            b"NH" | b"FJ" => Some(32),
            suffix => {
                warn!("Unknown suffix: {}", suffix);
                None
            }
        }
    }

    /// Writes MCU information to the logger
    #[cfg(feature = "defmt")]
    pub fn print_info(&self) {
        if !self.ok() {
            info!(
                "MCU: {} rev {:02X}, uid={:08x}-{:08x}-{:08x}-{:08x}",
                self.part_number(),
                self.revision(),
                (self.uid() >> 96 & 0xFFFFFFFF),
                (self.uid() >> 64 & 0xFFFFFFFF),
                (self.uid() >> 32 & 0xFFFFFFFF),
                (self.uid() >> 0 & 0xFFFFFFFF),
            );
            warn!("PN too short to identify");
        } else {
            info!(
                "MCU: {} rev {:02X}, flash={} KB, uid={:08x}-{:08x}-{:08x}-{:08x}",
                self.part_number(),
                self.revision(),
                self.flash_size().unwrap_or(0),
                (self.uid() >> 96 & 0xFFFFFFFF),
                (self.uid() >> 64 & 0xFFFFFFFF),
                (self.uid() >> 32 & 0xFFFFFFFF),
                (self.uid() >> 0 & 0xFFFFFFFF),
            );
        }
    }

    pub fn validate_pin_count(&self) {
        match self.pin_count() {
            Some(actual) => {
                cfg_if! {
                    if #[cfg(feature = "_100pin")] {
                        let configured = 100;
                    } else if #[cfg(feature = "_64pin")] {
                        let configured = 64;
                    } else if #[cfg(feature = "_48pin")] {
                        let configured = 48;
                    } else {
                        let configured = 40;
                    }
                }

                if configured != actual {
                    warn!(
                        "May not behave as expected. HAL configured with {} pins, MCU has {} pins",
                        configured, actual
                    );
                }
            }
            None => warn!("Couldn't determine appropriate pin count"),
        }
    }

    /// Loads MCU information from `FMIFRT`.
    pub fn info() -> Self {
        let fmifrt = pac::FMIFRT;

        let uid: [u32; 4] = [
            fmifrt.uidr(3).read().uid(),
            fmifrt.uidr(2).read().uid(),
            fmifrt.uidr(1).read().uid(),
            fmifrt.uidr(0).read().uid(),
        ];

        // Safety: alignment is less strict here, right?
        let uid: [u8; 16] = unsafe { transmute(uid) };
        let uid = u128::from_ne_bytes(uid);

        let mut part_number: [u8; 16] = [0; 16];
        part_number[0..4].copy_from_slice(&fmifrt.pnr(0).read().0.to_ne_bytes());
        part_number[4..8].copy_from_slice(&fmifrt.pnr(1).read().0.to_ne_bytes());
        part_number[8..12].copy_from_slice(&fmifrt.pnr(2).read().0.to_ne_bytes());
        part_number[12..16].copy_from_slice(&fmifrt.pnr(3).read().0.to_ne_bytes());

        // TODO: Why is this register 32 bits wide?
        let rev = fmifrt.mcuver().read().mcuver();

        McuInfo {
            uid,
            part_number,
            rev,
        }
    }
}
