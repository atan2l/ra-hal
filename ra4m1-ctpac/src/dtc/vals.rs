#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Act {
    #[doc = "DTC transfer operation is not in progress."]
    _0 = 0x0,
    #[doc = "DTC transfer operation is in progress."]
    _1 = 0x01,
}
impl Act {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Act {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Act {
    #[inline(always)]
    fn from(val: u8) -> Act {
        Act::from_bits(val)
    }
}
impl From<Act> for u8 {
    #[inline(always)]
    fn from(val: Act) -> u8 {
        Act::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtcst {
    #[doc = "DTC module stop"]
    _0 = 0x0,
    #[doc = "DTC module start"]
    _1 = 0x01,
}
impl Dtcst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtcst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtcst {
    #[inline(always)]
    fn from(val: u8) -> Dtcst {
        Dtcst::from_bits(val)
    }
}
impl From<Dtcst> for u8 {
    #[inline(always)]
    fn from(val: Dtcst) -> u8 {
        Dtcst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rrs {
    #[doc = "Do not skip transfer information read"]
    _0 = 0x0,
    #[doc = "Skip transfer information read when vector numbers match"]
    _1 = 0x01,
}
impl Rrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rrs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rrs {
    #[inline(always)]
    fn from(val: u8) -> Rrs {
        Rrs::from_bits(val)
    }
}
impl From<Rrs> for u8 {
    #[inline(always)]
    fn from(val: Rrs) -> u8 {
        Rrs::to_bits(val)
    }
}
