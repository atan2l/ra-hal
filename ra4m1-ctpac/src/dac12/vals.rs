#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ref {
    #[doc = "Not selected"]
    _000 = 0x0,
    #[doc = "AVCC0/AVSS0"]
    _001 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Internal reference voltage/AVSS0"]
    _011 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "VREFH/VREFL"]
    _110 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ref {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ref {
    #[inline(always)]
    fn from(val: u8) -> Ref {
        Ref::from_bits(val)
    }
}
impl From<Ref> for u8 {
    #[inline(always)]
    fn from(val: Ref) -> u8 {
        Ref::to_bits(val)
    }
}
