//! Factory MCU Information

use core::mem::transmute;

use crate::pac;

/// Contains information about the MCU that was programmed at the factory including UID and MCU configuration.
/// As the registers are all read-only we don't care about ownership of the peripheral.
pub struct McuInfo {
    uid: u128,
    part_number: [u8; 16],
    rev: u8,
}

impl McuInfo {
    const PN_LEN: usize = 13;

    /// # Returns
    ///
    /// The part number as a string.
    pub fn part_number(&self) -> &str {
        core::str::from_utf8(&self.part_number).unwrap().trim()
    }

    /// # Returns
    ///
    /// The unique 128-bit identifier associated with the MCU.
    #[inline]
    pub fn uid(&self) -> u128 {
        self.uid
    }

    /// # Returns
    ///
    /// The revision of the MCU.  Higher values are newer.
    #[inline]
    pub fn revision(&self) -> u8 {
        self.rev
    }

    /// # Returns
    ///
    /// - `true` if the part number was log enough to be parsed.
    /// - `false` otherwise.  In this case neither `flash_size` nor `pin_count` will work.
    pub fn ok(&self) -> bool {
        self.part_number().len() >= Self::PN_LEN
    }

    /// # Returns
    ///
    /// - The amount of on-die flash memory in kilobytes.
    /// - `None` if this cannot be determined.
    pub fn flash_size(&self) -> Option<u16> {
        match self.part_number[8] {
            b'9' => Some(128),
            b'B' => Some(256),
            b'C' => Some(384),
            b'D' => Some(512),
            b'E' => Some(768),
            b'F' => Some(1024),
            b'G' => Some(1536),
            b'H' => Some(2048),
            _ => {
                let flash_suffix = self.part_number[8] as char;
                if flash_suffix.is_ascii() {
                    warn!("Unknown flash suffix '{}'", flash_suffix);
                } else {
                    warn!("Unknown flash suffix 0x{}", self.part_number[8]);
                }
                None
            }
        }
    }

    /// # Returns
    ///
    /// - The MCU form factor as a tuple of pin count and package style.
    /// - `None` if this cannot be determined.
    pub fn package(&self) -> Option<(u8, &'static str)> {
        match &self.part_number[11..=12] {
            b"BD" => Some((224, "BGA")),
            b"BG" => Some((176, "BGA")),
            b"FC" => Some((176, "LQFP")),
            b"LK" => Some((145, "LGA")),
            b"FB" => Some((144, "LQFP")),
            b"BM" => Some((144, "BGA")),
            b"FP" => Some((100, "LQFP")),
            b"LJ" => Some((100, "LGA")),
            b"NB" => Some((64, "QFN")),
            b"BQ" => Some((64, "BGA")),
            b"BB" => Some((64, "BGA")),
            b"FM" => Some((64, "LQFP")),
            b"NG" => Some((56, "QFN")),
            b"NE" => Some((48, "QFN")),
            b"FL" => Some((48, "LQFP")),
            b"NF" => Some((40, "QFN")),
            b"BC" => Some((36, "BGA")),
            b"NH" => Some((32, "QFN")),
            b"FJ" => Some((32, "LQFP")),
            suffix => {
                match str::from_utf8(suffix) {
                    Ok(suffix) => warn!("Unknown suffix: '{}'", suffix),
                    Err(_) => warn!("Unknown suffix: {:X}", suffix),
                }
                None
            }
        }
    }

    /// Writes MCU information to the logger in human readable form.
    #[cfg(feature = "defmt")]
    pub fn print_info(&self) {
        if !self.ok() {
            info!(
                "MCU: {} rev {:02X}, uid={:08x}-{:08x}-{:08x}-{:08x}",
                self.part_number(),
                self.revision(),
                (self.uid() >> 96) & 0xFFFFFFFF,
                (self.uid() >> 64) & 0xFFFFFFFF,
                (self.uid() >> 32) & 0xFFFFFFFF,
                self.uid() & 0xFFFFFFFF,
            );
            warn!("PN too short to identify");
        } else {
            let package = self.package().unwrap_or((0, "Unknown"));
            info!(
                "MCU: {} rev {:02X}, flash={} KB, {}-pin {}, uid={:08x}-{:08x}-{:08x}-{:08x}",
                self.part_number(),
                self.revision(),
                self.flash_size().unwrap_or(0),
                package.0,
                package.1,
                (self.uid() >> 96) & 0xFFFFFFFF,
                (self.uid() >> 64) & 0xFFFFFFFF,
                (self.uid() >> 32) & 0xFFFFFFFF,
                self.uid() & 0xFFFFFFFF,
            );
        }
    }

    /// Compares the package as calculated from the part number to the package configured via Cargo features.
    /// Logs a warning if there is a mismatch.
    ///
    /// # Returns
    ///
    /// - `true` packages match.
    /// - `false` packages do not match or actual package could not be determined.
    pub fn validate_package(&self) -> bool {
        match self.package() {
            Some(actual) => {
                let configured = cfg_select! {
                    feature = "32lqfp" => (32, "LQFP"),
                    feature = "36bga" => (36, "BGA"),
                    feature = "40qfn" => (40, "QFN"),
                    feature = "48lqfp" => (48, "LQFP"),
                    feature = "48qfn" => (48, "QFN"),
                    feature = "64bga" => (64, "BGA"),
                    feature = "64lqfp" => (64, "LQFP"),
                    feature = "64qfn" => (64, "QFN"),
                    feature = "100bga" => (100, "BGA"),
                    feature = "100lga" => (100, "LGA"),
                    feature = "100lqfp" => (100, "LQFP"),
                    feature = "144bga" => (144, "BGA"),
                    feature = "144lqfp" => (144, "LQFP"),
                    feature = "176bga" => (176, "BGA"),
                    feature = "176lqfp" => (176, "LQFP"),
                    feature = "224bga" => (224, "BGA"),
                    _ => compile_error!("No package feature selected"),
                };

                if configured != actual {
                    warn!(
                        "May not behave as expected. HAL configured for {}-pin {} package, MCU is {}-pin {}.",
                        configured.0, configured.1, actual.0, actual.1
                    );
                }
                configured == actual
            }
            None => {
                warn!("Couldn't determine package");
                false
            }
        }
    }

    /// Reads MCU information
    ///
    /// # Returns
    ///
    /// A struct populated with the available configuration information.
    pub fn info() -> Self {
        let fmifrt = pac::INFO;

        let uid: [u32; 4] = [
            fmifrt.uidr(3).read(),
            fmifrt.uidr(2).read(),
            fmifrt.uidr(1).read(),
            fmifrt.uidr(0).read(),
        ];

        // Safety: alignment is less strict here, right?
        let uid: [u8; 16] = unsafe { transmute(uid) };
        let uid = u128::from_ne_bytes(uid);

        let mut part_number: [u8; 16] = [0; 16];
        part_number[0..4].copy_from_slice(&fmifrt.pnr(0).read().to_ne_bytes());
        part_number[4..8].copy_from_slice(&fmifrt.pnr(1).read().to_ne_bytes());
        part_number[8..12].copy_from_slice(&fmifrt.pnr(2).read().to_ne_bytes());
        part_number[12..16].copy_from_slice(&fmifrt.pnr(3).read().to_ne_bytes());

        // TODO: Why is this register 32 bits wide?
        let rev = fmifrt.mcuver().read();

        McuInfo {
            uid,
            part_number,
            rev,
        }
    }
}
