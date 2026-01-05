#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum B0wi {
    #[doc = "Writing to the PFSWE bit is enabled"]
    _0 = 0x0,
    #[doc = "Writing to the PFSWE bit is disabled"]
    _1 = 0x01,
}
impl B0wi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> B0wi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for B0wi {
    #[inline(always)]
    fn from(val: u8) -> B0wi {
        B0wi::from_bits(val)
    }
}
impl From<B0wi> for u8 {
    #[inline(always)]
    fn from(val: B0wi) -> u8 {
        B0wi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pfswe {
    #[doc = "Writing to the PFS register is disabled"]
    _0 = 0x0,
    #[doc = "Writing to the PFS register is enabled"]
    _1 = 0x01,
}
impl Pfswe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfswe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfswe {
    #[inline(always)]
    fn from(val: u8) -> Pfswe {
        Pfswe::from_bits(val)
    }
}
impl From<Pfswe> for u8 {
    #[inline(always)]
    fn from(val: Pfswe) -> u8 {
        Pfswe::to_bits(val)
    }
}
