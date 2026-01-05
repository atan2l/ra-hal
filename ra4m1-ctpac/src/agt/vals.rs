#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cks {
    #[doc = "1/1"]
    _000 = 0x0,
    #[doc = "1/2"]
    _001 = 0x01,
    #[doc = "1/4"]
    _010 = 0x02,
    #[doc = "1/8"]
    _011 = 0x03,
    #[doc = "1/16"]
    _100 = 0x04,
    #[doc = "1/32"]
    _101 = 0x05,
    #[doc = "1/64"]
    _110 = 0x06,
    #[doc = "1/128."]
    _111 = 0x07,
}
impl Cks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cks {
        unsafe { core::mem::transmute(val & 0x07) }
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
pub enum Sel {
    #[doc = "Select the AGTIOn except for below pins"]
    _00 = 0x0,
    #[doc = "Setting prohibited"]
    _01 = 0x01,
    #[doc = "Select the P402/AGTIOn. P402/AGTIOn is input only. It is not possible to output"]
    _10 = 0x02,
    #[doc = "Select the P403/AGTIOn. P403/AGTIOn is input only. It is not possible to output"]
    _11 = 0x03,
}
impl Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sel {
    #[inline(always)]
    fn from(val: u8) -> Sel {
        Sel::from_bits(val)
    }
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(val: Sel) -> u8 {
        Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tck {
    #[doc = "PCLKB"]
    _000 = 0x0,
    #[doc = "PCLKB/8"]
    _001 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "PCLKB/2"]
    _011 = 0x03,
    #[doc = "Divided clock AGTLCLK specified by CKS\\[2:0\\] bits in the AGTMR2 register"]
    _100 = 0x04,
    #[doc = "Underflow event signal from AGT0*6"]
    _101 = 0x05,
    #[doc = "Divided clock AGTSCLK specified by CKS\\[2:0\\] bits in the AGTMR2 register."]
    _110 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tck {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tck {
    #[inline(always)]
    fn from(val: u8) -> Tck {
        Tck::from_bits(val)
    }
}
impl From<Tck> for u8 {
    #[inline(always)]
    fn from(val: Tck) -> u8 {
        Tck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tiogt {
    #[doc = "Event is always counted"]
    _00 = 0x0,
    #[doc = "Event is counted during polarity period specified for AGTEEn."]
    _01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Tiogt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tiogt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tiogt {
    #[inline(always)]
    fn from(val: u8) -> Tiogt {
        Tiogt::from_bits(val)
    }
}
impl From<Tiogt> for u8 {
    #[inline(always)]
    fn from(val: Tiogt) -> u8 {
        Tiogt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tipf {
    #[doc = "No filter"]
    _00 = 0x0,
    #[doc = "Filter sampled at PCLKB"]
    _01 = 0x01,
    #[doc = "Filter sampled at PCLKB/8"]
    _10 = 0x02,
    #[doc = "Filter sampled at PCLKB/32"]
    _11 = 0x03,
}
impl Tipf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tipf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tipf {
    #[inline(always)]
    fn from(val: u8) -> Tipf {
        Tipf::from_bits(val)
    }
}
impl From<Tipf> for u8 {
    #[inline(always)]
    fn from(val: Tipf) -> u8 {
        Tipf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmod {
    #[doc = "Timer mode"]
    _000 = 0x0,
    #[doc = "Pulse output mode"]
    _001 = 0x01,
    #[doc = "Event counter mode"]
    _010 = 0x02,
    #[doc = "Pulse width measurement mode"]
    _011 = 0x03,
    #[doc = "Pulse period measurement mode."]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmod {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmod {
    #[inline(always)]
    fn from(val: u8) -> Tmod {
        Tmod::from_bits(val)
    }
}
impl From<Tmod> for u8 {
    #[inline(always)]
    fn from(val: Tmod) -> u8 {
        Tmod::to_bits(val)
    }
}
