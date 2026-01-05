#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cks {
    _RESERVED_0 = 0x0,
    #[doc = "PCLK/4"]
    _0001 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "PCLK/64"]
    _0100 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PCLK/512"]
    _0110 = 0x06,
    #[doc = "PCLK/2048"]
    _0111 = 0x07,
    #[doc = "PCLK/8192"]
    _1000 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "PCLK/128"]
    _1111 = 0x0f,
}
impl Cks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cks {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cks {
    #[inline(always)]
    fn from(val: u8) -> Cks {
        Cks::from_bits(val)
    }
}
impl From<Cks> for u8 {
    #[inline(always)]
    fn from(val: Cks) -> u8 {
        Cks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Refef {
    #[doc = "No refresh error occurred"]
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
pub enum Rpes {
    #[doc = "75 percent"]
    _00 = 0x0,
    #[doc = "50 percent"]
    _01 = 0x01,
    #[doc = "25 percent"]
    _10 = 0x02,
    #[doc = "0 percent (window end position is not specified)"]
    _11 = 0x03,
}
impl Rpes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rpes {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rpes {
    #[inline(always)]
    fn from(val: u8) -> Rpes {
        Rpes::from_bits(val)
    }
}
impl From<Rpes> for u8 {
    #[inline(always)]
    fn from(val: Rpes) -> u8 {
        Rpes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rpss {
    #[doc = "25 percent"]
    _00 = 0x0,
    #[doc = "50 percent"]
    _01 = 0x01,
    #[doc = "75 percent"]
    _10 = 0x02,
    #[doc = "100 percent (window start position is not specified)"]
    _11 = 0x03,
}
impl Rpss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rpss {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rpss {
    #[inline(always)]
    fn from(val: u8) -> Rpss {
        Rpss::from_bits(val)
    }
}
impl From<Rpss> for u8 {
    #[inline(always)]
    fn from(val: Rpss) -> u8 {
        Rpss::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstirqs {
    #[doc = "Non-maskable interrupt request or interrupt request output is enabled"]
    _0 = 0x0,
    #[doc = "Reset output is enabled."]
    _1 = 0x01,
}
impl Rstirqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstirqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstirqs {
    #[inline(always)]
    fn from(val: u8) -> Rstirqs {
        Rstirqs::from_bits(val)
    }
}
impl From<Rstirqs> for u8 {
    #[inline(always)]
    fn from(val: Rstirqs) -> u8 {
        Rstirqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slcstp {
    #[doc = "Count stop is disabled."]
    _0 = 0x0,
    #[doc = "Count is stopped at a transition to sleep mode."]
    _1 = 0x01,
}
impl Slcstp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slcstp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slcstp {
    #[inline(always)]
    fn from(val: u8) -> Slcstp {
        Slcstp::from_bits(val)
    }
}
impl From<Slcstp> for u8 {
    #[inline(always)]
    fn from(val: Slcstp) -> u8 {
        Slcstp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tops {
    #[doc = "1,024 cycles (03FFh)"]
    _00 = 0x0,
    #[doc = "4,096 cycles (0FFFh)"]
    _01 = 0x01,
    #[doc = "8,192 cycles (1FFFh)"]
    _10 = 0x02,
    #[doc = "16,384 cycles (3FFFh)"]
    _11 = 0x03,
}
impl Tops {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tops {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tops {
    #[inline(always)]
    fn from(val: u8) -> Tops {
        Tops::from_bits(val)
    }
}
impl From<Tops> for u8 {
    #[inline(always)]
    fn from(val: Tops) -> u8 {
        Tops::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Undff {
    #[doc = "No underflow occurred"]
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
