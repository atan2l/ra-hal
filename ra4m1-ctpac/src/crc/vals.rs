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
pub enum Crcsen {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled"]
    _1 = 0x01,
}
impl Crcsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crcsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crcsen {
    #[inline(always)]
    fn from(val: u8) -> Crcsen {
        Crcsen::from_bits(val)
    }
}
impl From<Crcsen> for u8 {
    #[inline(always)]
    fn from(val: Crcsen) -> u8 {
        Crcsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crcswr {
    #[doc = "Snoop-on-read"]
    _0 = 0x0,
    #[doc = "Snoop-on-write"]
    _1 = 0x01,
}
impl Crcswr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crcswr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crcswr {
    #[inline(always)]
    fn from(val: u8) -> Crcswr {
        Crcswr::from_bits(val)
    }
}
impl From<Crcswr> for u8 {
    #[inline(always)]
    fn from(val: Crcswr) -> u8 {
        Crcswr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dorclr {
    #[doc = "No effect."]
    _0 = 0x0,
    #[doc = "Clears the CRCDOR register."]
    _1 = 0x01,
}
impl Dorclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dorclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dorclr {
    #[inline(always)]
    fn from(val: u8) -> Dorclr {
        Dorclr::from_bits(val)
    }
}
impl From<Dorclr> for u8 {
    #[inline(always)]
    fn from(val: Dorclr) -> u8 {
        Dorclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gps {
    #[doc = "No calculation is executed."]
    _000 = 0x0,
    #[doc = "8-bit CRC-8 (X8 + X2 + X + 1)"]
    _001 = 0x01,
    #[doc = "16-bit CRC-16 (X16 + X15 + X2 + 1)"]
    _010 = 0x02,
    #[doc = "16-bit CRC-CCITT (X16 + X12 + X5 + 1)"]
    _011 = 0x03,
    #[doc = "32-bit CRC-32 (X32+X26+X23+X22+X16+X12+X11+X10+X8+X7+X5+X4+X2+X+1)"]
    _100 = 0x04,
    #[doc = "32-bit CRC-32C (X32+X28+X27+X26+ X25+X23+X22+X20+X19+X18+X14+X13+X11+X10+X9+X8+X6+1)"]
    _101 = 0x05,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lms {
    #[doc = "Generates CRC for LSB first communication."]
    _0 = 0x0,
    #[doc = "Generates CRC for MSB first communication."]
    _1 = 0x01,
}
impl Lms {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lms {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lms {
    #[inline(always)]
    fn from(val: u8) -> Lms {
        Lms::from_bits(val)
    }
}
impl From<Lms> for u8 {
    #[inline(always)]
    fn from(val: Lms) -> u8 {
        Lms::to_bits(val)
    }
}
