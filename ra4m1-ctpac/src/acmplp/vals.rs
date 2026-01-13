#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C0fck {
    #[doc = "No Sampling (bypass)"]
    _00 = 0x0,
    #[doc = "Sampling at PCLK"]
    _01 = 0x01,
    #[doc = "Sampling at PCLK/8"]
    _10 = 0x02,
    #[doc = "Sampling at PCLK/32"]
    _11 = 0x03,
}
impl C0fck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C0fck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C0fck {
    #[inline(always)]
    fn from(val: u8) -> C0fck {
        C0fck::from_bits(val)
    }
}
impl From<C0fck> for u8 {
    #[inline(always)]
    fn from(val: C0fck) -> u8 {
        C0fck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C1fck {
    #[doc = "No Sampling (bypass)"]
    _00 = 0x0,
    #[doc = "Sampling at PCLK"]
    _01 = 0x01,
    #[doc = "Sampling at PCLK/8"]
    _10 = 0x02,
    #[doc = "Sampling at PCLK/32"]
    _11 = 0x03,
}
impl C1fck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C1fck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C1fck {
    #[inline(always)]
    fn from(val: u8) -> C1fck {
        C1fck::from_bits(val)
    }
}
impl From<C1fck> for u8 {
    #[inline(always)]
    fn from(val: C1fck) -> u8 {
        C1fck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpsel20 {
    #[doc = "No input"]
    _000 = 0x0,
    #[doc = "CMPIN0 (P100)"]
    _001 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "CMPIN0 (P503)"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cmpsel20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpsel20 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpsel20 {
    #[inline(always)]
    fn from(val: u8) -> Cmpsel20 {
        Cmpsel20::from_bits(val)
    }
}
impl From<Cmpsel20> for u8 {
    #[inline(always)]
    fn from(val: Cmpsel20) -> u8 {
        Cmpsel20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpsel64 {
    #[doc = "No input"]
    _000 = 0x0,
    #[doc = "CMPIN1 (P102)"]
    _001 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "CMPIN1 (P501)"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cmpsel64 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpsel64 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpsel64 {
    #[inline(always)]
    fn from(val: u8) -> Cmpsel64 {
        Cmpsel64::from_bits(val)
    }
}
impl From<Cmpsel64> for u8 {
    #[inline(always)]
    fn from(val: Cmpsel64) -> u8 {
        Cmpsel64::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crvs20 {
    #[doc = "No input"]
    _000 = 0x0,
    #[doc = "CMPREF0 (P101)"]
    _001 = 0x01,
    #[doc = "DAC8 (ch0) output"]
    _010 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "CMPREF0 (P502)"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Crvs20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crvs20 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crvs20 {
    #[inline(always)]
    fn from(val: u8) -> Crvs20 {
        Crvs20::from_bits(val)
    }
}
impl From<Crvs20> for u8 {
    #[inline(always)]
    fn from(val: Crvs20) -> u8 {
        Crvs20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crvs64 {
    #[doc = "No input"]
    _000 = 0x0,
    #[doc = "CMPREF1 (P103)"]
    _001 = 0x01,
    #[doc = "DAC8 (ch1) output"]
    _010 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "CMPREF1 (P500)"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Crvs64 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crvs64 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crvs64 {
    #[inline(always)]
    fn from(val: u8) -> Crvs64 {
        Crvs64::from_bits(val)
    }
}
impl From<Crvs64> for u8 {
    #[inline(always)]
    fn from(val: Crvs64) -> u8 {
        Crvs64::to_bits(val)
    }
}
