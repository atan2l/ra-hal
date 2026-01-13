#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oms {
    #[doc = "Data comparison mode"]
    _00 = 0x0,
    #[doc = "Data addition mode"]
    _01 = 0x01,
    #[doc = "Data subtraction mode"]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Oms {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oms {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oms {
    #[inline(always)]
    fn from(val: u8) -> Oms {
        Oms::from_bits(val)
    }
}
impl From<Oms> for u8 {
    #[inline(always)]
    fn from(val: Oms) -> u8 {
        Oms::to_bits(val)
    }
}
