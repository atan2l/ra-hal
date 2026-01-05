#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dace0 {
    #[doc = "D/A conversion disabled for channel 0"]
    _0 = 0x0,
    #[doc = "D/A conversion enabled for channel 0."]
    _1 = 0x01,
}
impl Dace0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dace0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dace0 {
    #[inline(always)]
    fn from(val: u8) -> Dace0 {
        Dace0::from_bits(val)
    }
}
impl From<Dace0> for u8 {
    #[inline(always)]
    fn from(val: Dace0) -> u8 {
        Dace0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dace1 {
    #[doc = "D/A conversion disabled for channel 1"]
    _0 = 0x0,
    #[doc = "D/A conversion enabled for channel 1"]
    _1 = 0x01,
}
impl Dace1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dace1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dace1 {
    #[inline(always)]
    fn from(val: u8) -> Dace1 {
        Dace1::from_bits(val)
    }
}
impl From<Dace1> for u8 {
    #[inline(always)]
    fn from(val: Dace1) -> u8 {
        Dace1::to_bits(val)
    }
}
