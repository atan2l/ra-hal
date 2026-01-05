#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Crcsa(u16);
impl Crcsa {
    #[doc = "SCI0.TDR"]
    pub const _0X0003: Self = Self(0x03);
    #[doc = "SCI0.RDR"]
    pub const _0X0005: Self = Self(0x05);
    #[doc = "SCI1.TDR"]
    pub const _0X0023: Self = Self(0x23);
    #[doc = "SCI1.RDR"]
    pub const _0X0025: Self = Self(0x25);
    #[doc = "SCI2.TDR"]
    pub const _0X0043: Self = Self(0x43);
    #[doc = "SCI2.RDR"]
    pub const _0X0045: Self = Self(0x45);
    #[doc = "SCI3.TDR"]
    pub const _0X0063: Self = Self(0x63);
    #[doc = "SCI3.RDR"]
    pub const _0X0065: Self = Self(0x65);
    #[doc = "SCI4.TDR"]
    pub const _0X0083: Self = Self(0x83);
    #[doc = "SCI4.RDR"]
    pub const _0X0085: Self = Self(0x85);
    #[doc = "SCI9.TDR"]
    pub const _0X0123: Self = Self(0x0123);
    #[doc = "SCI9.RDR"]
    pub const _0X0125: Self = Self(0x0125);
}
impl Crcsa {
    pub const fn from_bits(val: u16) -> Crcsa {
        Self(val & 0x3fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Crcsa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x03 => f.write_str("_0X0003"),
            0x05 => f.write_str("_0X0005"),
            0x23 => f.write_str("_0X0023"),
            0x25 => f.write_str("_0X0025"),
            0x43 => f.write_str("_0X0043"),
            0x45 => f.write_str("_0X0045"),
            0x63 => f.write_str("_0X0063"),
            0x65 => f.write_str("_0X0065"),
            0x83 => f.write_str("_0X0083"),
            0x85 => f.write_str("_0X0085"),
            0x0123 => f.write_str("_0X0123"),
            0x0125 => f.write_str("_0X0125"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crcsa {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x03 => defmt::write!(f, "_0X0003"),
            0x05 => defmt::write!(f, "_0X0005"),
            0x23 => defmt::write!(f, "_0X0023"),
            0x25 => defmt::write!(f, "_0X0025"),
            0x43 => defmt::write!(f, "_0X0043"),
            0x45 => defmt::write!(f, "_0X0045"),
            0x63 => defmt::write!(f, "_0X0063"),
            0x65 => defmt::write!(f, "_0X0065"),
            0x83 => defmt::write!(f, "_0X0083"),
            0x85 => defmt::write!(f, "_0X0085"),
            0x0123 => defmt::write!(f, "_0X0123"),
            0x0125 => defmt::write!(f, "_0X0125"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Crcsa {
    #[inline(always)]
    fn from(val: u16) -> Crcsa {
        Crcsa::from_bits(val)
    }
}
impl From<Crcsa> for u16 {
    #[inline(always)]
    fn from(val: Crcsa) -> u16 {
        Crcsa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gps {
    #[doc = "No calculation is executed."]
    None = 0x0,
    #[doc = "8-bit CRC-8 (X8 + X2 + X + 1)"]
    Crc8 = 0x01,
    #[doc = "16-bit CRC-16 (X16 + X15 + X2 + 1)"]
    Crc16 = 0x02,
    #[doc = "16-bit CRC-CCITT (X16 + X12 + X5 + 1)"]
    CrcCcit = 0x03,
    #[doc = "32-bit CRC-32 (X32+X26+X23+X22+X16+X12+X11+X10+X8+X7+X5+X4+X2+X+1)"]
    Crc32 = 0x04,
    #[doc = "32-bit CRC-32C (X32+X28+X27+X26+ X25+X23+X22+X20+X19+X18+X14+X13+X11+X10+X9+X8+X6+1)"]
    Crc32C = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Gps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gps {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gps {
    #[inline(always)]
    fn from(val: u8) -> Gps {
        Gps::from_bits(val)
    }
}
impl From<Gps> for u8 {
    #[inline(always)]
    fn from(val: Gps) -> u8 {
        Gps::to_bits(val)
    }
}
