#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eccmod {
    #[doc = "Disable ECC function"]
    _00 = 0x0,
    #[doc = "Setting prohibited"]
    _01 = 0x01,
    #[doc = "Enable ECC function without error checking"]
    _10 = 0x02,
    #[doc = "Enable ECC function with error checking"]
    _11 = 0x03,
}
impl Eccmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eccmod {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eccmod {
    #[inline(always)]
    fn from(val: u8) -> Eccmod {
        Eccmod::from_bits(val)
    }
}
impl From<Eccmod> for u8 {
    #[inline(always)]
    fn from(val: Eccmod) -> u8 {
        Eccmod::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EccprcrKw(u8);
impl EccprcrKw {
    #[doc = "Writing to the ECCRAMPRCR bit is valid, when the KEY bits are written 1111000b."]
    pub const _1111000: Self = Self(0x78);
}
impl EccprcrKw {
    pub const fn from_bits(val: u8) -> EccprcrKw {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for EccprcrKw {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x78 => f.write_str("_1111000"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EccprcrKw {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x78 => defmt::write!(f, "_1111000"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for EccprcrKw {
    #[inline(always)]
    fn from(val: u8) -> EccprcrKw {
        EccprcrKw::from_bits(val)
    }
}
impl From<EccprcrKw> for u8 {
    #[inline(always)]
    fn from(val: EccprcrKw) -> u8 {
        EccprcrKw::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Kw2(u8);
impl Kw2 {
    #[doc = "These bits enable or disable writes to the ECCPRCR2 bit.."]
    pub const _1111000: Self = Self(0x78);
}
impl Kw2 {
    pub const fn from_bits(val: u8) -> Kw2 {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Kw2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x78 => f.write_str("_1111000"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kw2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x78 => defmt::write!(f, "_1111000"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Kw2 {
    #[inline(always)]
    fn from(val: u8) -> Kw2 {
        Kw2::from_bits(val)
    }
}
impl From<Kw2> for u8 {
    #[inline(always)]
    fn from(val: Kw2) -> u8 {
        Kw2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SramprcrKw(u8);
impl SramprcrKw {
    #[doc = "Writing to the RAMPRCR bit is valid, when the KEY bits are written 1111000b."]
    pub const _1111000: Self = Self(0x78);
}
impl SramprcrKw {
    pub const fn from_bits(val: u8) -> SramprcrKw {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for SramprcrKw {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x78 => f.write_str("_1111000"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SramprcrKw {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x78 => defmt::write!(f, "_1111000"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for SramprcrKw {
    #[inline(always)]
    fn from(val: u8) -> SramprcrKw {
        SramprcrKw::from_bits(val)
    }
}
impl From<SramprcrKw> for u8 {
    #[inline(always)]
    fn from(val: SramprcrKw) -> u8 {
        SramprcrKw::to_bits(val)
    }
}
