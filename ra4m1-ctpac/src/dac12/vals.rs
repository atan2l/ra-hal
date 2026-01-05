#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Daadst {
    #[doc = "D/A converter operation does not synchronize with A/D converter operation (unit 1) (countermeasure against interference between D/A and A/D conversions is disabled)."]
    _0 = 0x0,
    #[doc = "D/A converter operation synchronizes with A/D converter operation (unit 1) (countermeasure against interference between D/A and A/D conversions is enabled)."]
    _1 = 0x01,
}
impl Daadst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Daadst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Daadst {
    #[inline(always)]
    fn from(val: u8) -> Daadst {
        Daadst::from_bits(val)
    }
}
impl From<Daadst> for u8 {
    #[inline(always)]
    fn from(val: Daadst) -> u8 {
        Daadst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Daoe0 {
    #[doc = "Analog output of channel 0 (DA0) is disabled."]
    _0 = 0x0,
    #[doc = "D/A conversion of channel 0 is enabled. Analog output of channel 0 (DA0) is enabled."]
    _1 = 0x01,
}
impl Daoe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Daoe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Daoe0 {
    #[inline(always)]
    fn from(val: u8) -> Daoe0 {
        Daoe0::from_bits(val)
    }
}
impl From<Daoe0> for u8 {
    #[inline(always)]
    fn from(val: Daoe0) -> u8 {
        Daoe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dpsel {
    #[doc = "Right justified format."]
    _0 = 0x0,
    #[doc = "Left justified format."]
    _1 = 0x01,
}
impl Dpsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dpsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dpsel {
    #[inline(always)]
    fn from(val: u8) -> Dpsel {
        Dpsel::from_bits(val)
    }
}
impl From<Dpsel> for u8 {
    #[inline(always)]
    fn from(val: Dpsel) -> u8 {
        Dpsel::to_bits(val)
    }
}
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
