#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bc {
    #[doc = "9 bits"]
    _000 = 0x0,
    #[doc = "2 bits"]
    _001 = 0x01,
    #[doc = "3 bits"]
    _010 = 0x02,
    #[doc = "4 bits"]
    _011 = 0x03,
    #[doc = "5 bits"]
    _100 = 0x04,
    #[doc = "6 bits"]
    _101 = 0x05,
    #[doc = "7 bits"]
    _110 = 0x06,
    #[doc = "8 bits"]
    _111 = 0x07,
}
impl Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bc {
    #[inline(always)]
    fn from(val: u8) -> Bc {
        Bc::from_bits(val)
    }
}
impl From<Bc> for u8 {
    #[inline(always)]
    fn from(val: Bc) -> u8 {
        Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cks {
    #[doc = "PCLKB/1 clock"]
    _000 = 0x0,
    #[doc = "PCLKB/2 clock"]
    _001 = 0x01,
    #[doc = "PCLKB/4 clock"]
    _010 = 0x02,
    #[doc = "PCLKB/8 clock"]
    _011 = 0x03,
    #[doc = "PCLKB/16 clock"]
    _100 = 0x04,
    #[doc = "PCLKB/32 clock"]
    _101 = 0x05,
    #[doc = "PCLKB/64 clock"]
    _110 = 0x06,
    #[doc = "PCLKB/128 clock"]
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
pub enum Nf {
    #[doc = "Noise of up to one fIIC cycle is filtered out (single-stage filter)."]
    _00 = 0x0,
    #[doc = "Noise of up to two fIIC cycles is filtered out (2-stage filter)."]
    _01 = 0x01,
    #[doc = "Noise of up to three fIIC cycles is filtered out (3-stage filter)."]
    _10 = 0x02,
    #[doc = "Noise of up to four fIIC cycles is filtered out (4-stage filter)"]
    _11 = 0x03,
}
impl Nf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nf {
    #[inline(always)]
    fn from(val: u8) -> Nf {
        Nf::from_bits(val)
    }
}
impl From<Nf> for u8 {
    #[inline(always)]
    fn from(val: Nf) -> u8 {
        Nf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sddl {
    #[doc = "No output delay"]
    _000 = 0x0,
    #[doc = "1 fIIC cycle (ICMR2.DLCS=0) / 1 or 2 fIIC cycles (ICMR2.DLCS=1)"]
    _001 = 0x01,
    #[doc = "2 fIIC cycles (ICMR2.DLCS=0) / 3 or 4 fIIC cycles (ICMR2.DLCS=1)"]
    _010 = 0x02,
    #[doc = "3 fIIC cycles (ICMR2.DLCS=0) / 5 or 6 fIIC cycles (ICMR2.DLCS=1)"]
    _011 = 0x03,
    #[doc = "4 fIIC cycles (ICMR2.DLCS=0) / 7 or 8 fIIC cycles (ICMR2.DLCS=1)"]
    _100 = 0x04,
    #[doc = "5 fIIC cycles (ICMR2.DLCS=0) / 9 or 10 fIIC cycles (ICMR2.DLCS=1)"]
    _101 = 0x05,
    #[doc = "6 fIIC cycles (ICMR2.DLCS=0) / 11 or 12 fIIC cycles (ICMR2.DLCS=1)"]
    _110 = 0x06,
    #[doc = "7 fIIC cycles (ICMR2.DLCS=0) / 13 or 14 fIIC cycles (ICMR2.DLCS=1)"]
    _111 = 0x07,
}
impl Sddl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sddl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sddl {
    #[inline(always)]
    fn from(val: u8) -> Sddl {
        Sddl::from_bits(val)
    }
}
impl From<Sddl> for u8 {
    #[inline(always)]
    fn from(val: Sddl) -> u8 {
        Sddl::to_bits(val)
    }
}
