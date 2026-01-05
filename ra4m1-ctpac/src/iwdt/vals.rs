#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Refef {
    #[doc = "Refresh error not occurred"]
    _0 = 0x0,
    #[doc = "Refresh error occurred"]
    _1 = 0x01,
}
impl Refef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Refef {
    #[inline(always)]
    fn from(val: u8) -> Refef {
        Refef::from_bits(val)
    }
}
impl From<Refef> for u8 {
    #[inline(always)]
    fn from(val: Refef) -> u8 {
        Refef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Undff {
    #[doc = "Underflow not occurred"]
    _0 = 0x0,
    #[doc = "Underflow occurred"]
    _1 = 0x01,
}
impl Undff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Undff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Undff {
    #[inline(always)]
    fn from(val: u8) -> Undff {
        Undff::from_bits(val)
    }
}
impl From<Undff> for u8 {
    #[inline(always)]
    fn from(val: Undff) -> u8 {
        Undff::to_bits(val)
    }
}
