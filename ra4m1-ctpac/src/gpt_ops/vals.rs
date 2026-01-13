#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Grp {
    #[doc = "Select Group A output disable source"]
    _00 = 0x0,
    #[doc = "Select Group B output disable source"]
    _01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Grp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Grp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Grp {
    #[inline(always)]
    fn from(val: u8) -> Grp {
        Grp::from_bits(val)
    }
}
impl From<Grp> for u8 {
    #[inline(always)]
    fn from(val: Grp) -> u8 {
        Grp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nfcs {
    #[doc = "PCLK/1"]
    _00 = 0x0,
    #[doc = "PCLK/4"]
    _01 = 0x01,
    #[doc = "PCLK/16"]
    _10 = 0x02,
    #[doc = "PCLK/64"]
    _11 = 0x03,
}
impl Nfcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfcs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfcs {
    #[inline(always)]
    fn from(val: u8) -> Nfcs {
        Nfcs::from_bits(val)
    }
}
impl From<Nfcs> for u8 {
    #[inline(always)]
    fn from(val: Nfcs) -> u8 {
        Nfcs::to_bits(val)
    }
}
