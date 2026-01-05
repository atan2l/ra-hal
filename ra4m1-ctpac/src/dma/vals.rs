#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmst {
    #[doc = "Disabled."]
    _0 = 0x0,
    #[doc = "Enabled."]
    _1 = 0x01,
}
impl Dmst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmst {
    #[inline(always)]
    fn from(val: u8) -> Dmst {
        Dmst::from_bits(val)
    }
}
impl From<Dmst> for u8 {
    #[inline(always)]
    fn from(val: Dmst) -> u8 {
        Dmst::to_bits(val)
    }
}
