#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dctg {
    #[doc = "Software"]
    _00 = 0x0,
    #[doc = "Interrupts*1 from peripheral modules or external interrupt input pins"]
    _01 = 0x01,
    #[doc = "Setting prohibited"]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Dctg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dctg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dctg {
    #[inline(always)]
    fn from(val: u8) -> Dctg {
        Dctg::from_bits(val)
    }
}
impl From<Dctg> for u8 {
    #[inline(always)]
    fn from(val: Dctg) -> u8 {
        Dctg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dm {
    #[doc = "Fixed address"]
    _00 = 0x0,
    #[doc = "Offset addition"]
    _01 = 0x01,
    #[doc = "Incremented address"]
    _10 = 0x02,
    #[doc = "Decremented address."]
    _11 = 0x03,
}
impl Dm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dm {
    #[inline(always)]
    fn from(val: u8) -> Dm {
        Dm::from_bits(val)
    }
}
impl From<Dm> for u8 {
    #[inline(always)]
    fn from(val: Dm) -> u8 {
        Dm::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dmcrb(u16);
impl Dmcrb {
    #[doc = "65,536 blocks"]
    pub const _0000: Self = Self(0x0);
}
impl Dmcrb {
    pub const fn from_bits(val: u16) -> Dmcrb {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Dmcrb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0000"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmcrb {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0000"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Dmcrb {
    #[inline(always)]
    fn from(val: u16) -> Dmcrb {
        Dmcrb::from_bits(val)
    }
}
impl From<Dmcrb> for u16 {
    #[inline(always)]
    fn from(val: Dmcrb) -> u16 {
        Dmcrb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dts {
    #[doc = "The destination is specified as the repeat area or block area."]
    _00 = 0x0,
    #[doc = "The source is specified as the repeat area or block area."]
    _01 = 0x01,
    #[doc = "The repeat area or block area is not specified."]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Dts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dts {
    #[inline(always)]
    fn from(val: u8) -> Dts {
        Dts::from_bits(val)
    }
}
impl From<Dts> for u8 {
    #[inline(always)]
    fn from(val: Dts) -> u8 {
        Dts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Md {
    #[doc = "Normal transfer"]
    _00 = 0x0,
    #[doc = "Repeat transfer"]
    _01 = 0x01,
    #[doc = "Block transfer"]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Md {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Md {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Md {
    #[inline(always)]
    fn from(val: u8) -> Md {
        Md::from_bits(val)
    }
}
impl From<Md> for u8 {
    #[inline(always)]
    fn from(val: Md) -> u8 {
        Md::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm {
    #[doc = "Fixed address"]
    _00 = 0x0,
    #[doc = "Offset addition"]
    _01 = 0x01,
    #[doc = "Incremented address"]
    _10 = 0x02,
    #[doc = "Decremented address."]
    _11 = 0x03,
}
impl Sm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm {
    #[inline(always)]
    fn from(val: u8) -> Sm {
        Sm::from_bits(val)
    }
}
impl From<Sm> for u8 {
    #[inline(always)]
    fn from(val: Sm) -> u8 {
        Sm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sz {
    #[doc = "8 bits"]
    _00 = 0x0,
    #[doc = "16 bits"]
    _01 = 0x01,
    #[doc = "32 bits"]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Sz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sz {
    #[inline(always)]
    fn from(val: u8) -> Sz {
        Sz::from_bits(val)
    }
}
impl From<Sz> for u8 {
    #[inline(always)]
    fn from(val: Sz) -> u8 {
        Sz::to_bits(val)
    }
}
