#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Elcon {
    #[doc = "ELC function is disabled."]
    _0 = 0x0,
    #[doc = "ELC function is enabled."]
    _1 = 0x01,
}
impl Elcon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Elcon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Elcon {
    #[inline(always)]
    fn from(val: u8) -> Elcon {
        Elcon::from_bits(val)
    }
}
impl From<Elcon> for u8 {
    #[inline(always)]
    fn from(val: Elcon) -> u8 {
        Elcon::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Elsr12Els(u8);
impl Elsr12Els {
    #[doc = "Event output to the corresponding peripheral module is disabled."]
    pub const _0X00: Self = Self(0x0);
}
impl Elsr12Els {
    pub const fn from_bits(val: u8) -> Elsr12Els {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Elsr12Els {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0X00"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsr12Els {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0X00"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Elsr12Els {
    #[inline(always)]
    fn from(val: u8) -> Elsr12Els {
        Elsr12Els::from_bits(val)
    }
}
impl From<Elsr12Els> for u8 {
    #[inline(always)]
    fn from(val: Elsr12Els) -> u8 {
        Elsr12Els::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Elsr2Els(u8);
impl Elsr2Els {
    #[doc = "Event output to the corresponding peripheral module is disabled."]
    pub const _0X00: Self = Self(0x0);
}
impl Elsr2Els {
    pub const fn from_bits(val: u8) -> Elsr2Els {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Elsr2Els {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0X00"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsr2Els {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0X00"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Elsr2Els {
    #[inline(always)]
    fn from(val: u8) -> Elsr2Els {
        Elsr2Els::from_bits(val)
    }
}
impl From<Elsr2Els> for u8 {
    #[inline(always)]
    fn from(val: Elsr2Els) -> u8 {
        Elsr2Els::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ElsrEls(u8);
impl ElsrEls {
    #[doc = "Event output to the corresponding peripheral module is disabled."]
    pub const _0X00: Self = Self(0x0);
}
impl ElsrEls {
    pub const fn from_bits(val: u8) -> ElsrEls {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ElsrEls {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0X00"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsrEls {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0X00"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ElsrEls {
    #[inline(always)]
    fn from(val: u8) -> ElsrEls {
        ElsrEls::from_bits(val)
    }
}
impl From<ElsrEls> for u8 {
    #[inline(always)]
    fn from(val: ElsrEls) -> u8 {
        ElsrEls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Seg {
    #[doc = "Normal operation"]
    _0 = 0x0,
    #[doc = "Software event is generated."]
    _1 = 0x01,
}
impl Seg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Seg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Seg {
    #[inline(always)]
    fn from(val: u8) -> Seg {
        Seg::from_bits(val)
    }
}
impl From<Seg> for u8 {
    #[inline(always)]
    fn from(val: Seg) -> u8 {
        Seg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum We {
    #[doc = "Write to SEG bit is disabled."]
    _0 = 0x0,
    #[doc = "Write to SEG bit is enabled."]
    _1 = 0x01,
}
impl We {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> We {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for We {
    #[inline(always)]
    fn from(val: u8) -> We {
        We::from_bits(val)
    }
}
impl From<We> for u8 {
    #[inline(always)]
    fn from(val: We) -> u8 {
        We::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wi {
    #[doc = "Write to ELSEGR register is enabled."]
    _0 = 0x0,
    #[doc = "Write to ELSEGR register is disabled."]
    _1 = 0x01,
}
impl Wi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wi {
    #[inline(always)]
    fn from(val: u8) -> Wi {
        Wi::from_bits(val)
    }
}
impl From<Wi> for u8 {
    #[inline(always)]
    fn from(val: Wi) -> u8 {
        Wi::to_bits(val)
    }
}
