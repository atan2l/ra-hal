#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flwt {
    #[doc = "zero wait"]
    _000 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Flwt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flwt {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flwt {
    #[inline(always)]
    fn from(val: u8) -> Flwt {
        Flwt::from_bits(val)
    }
}
impl From<Flwt> for u8 {
    #[inline(always)]
    fn from(val: Flwt) -> u8 {
        Flwt::to_bits(val)
    }
}
