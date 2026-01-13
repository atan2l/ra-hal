#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ckdv {
    #[doc = "AUDIO_MCK"]
    _0000 = 0x0,
    #[doc = "AUDIO_MCK/2"]
    _0001 = 0x01,
    #[doc = "AUDIO_MCK/4"]
    _0010 = 0x02,
    #[doc = "AUDIO_MCK/8"]
    _0011 = 0x03,
    #[doc = "AUDIO_MCK/16"]
    _0100 = 0x04,
    #[doc = "AUDIO_MCK/32"]
    _0101 = 0x05,
    #[doc = "AUDIO_MCK/64"]
    _0110 = 0x06,
    #[doc = "AUDIO_MCK/128"]
    _0111 = 0x07,
    #[doc = "AUDIO_MCK/6"]
    _1000 = 0x08,
    #[doc = "AUDIO_MCK/12"]
    _1001 = 0x09,
    #[doc = "AUDIO_MCK/24"]
    _1010 = 0x0a,
    #[doc = "AUDIO_MCK/48"]
    _1011 = 0x0b,
    #[doc = "AUDIO_MCK/96"]
    _1100 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ckdv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ckdv {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ckdv {
    #[inline(always)]
    fn from(val: u8) -> Ckdv {
        Ckdv::from_bits(val)
    }
}
impl From<Ckdv> for u8 {
    #[inline(always)]
    fn from(val: Ckdv) -> u8 {
        Ckdv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dwl {
    #[doc = "8 bits"]
    _000 = 0x0,
    #[doc = "16 bits"]
    _001 = 0x01,
    #[doc = "18 bits"]
    _010 = 0x02,
    #[doc = "20 bits"]
    _011 = 0x03,
    #[doc = "22 bits"]
    _100 = 0x04,
    #[doc = "24 bits"]
    _101 = 0x05,
    #[doc = "32 bits"]
    _110 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Dwl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dwl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dwl {
    #[inline(always)]
    fn from(val: u8) -> Dwl {
        Dwl::from_bits(val)
    }
}
impl From<Dwl> for u8 {
    #[inline(always)]
    fn from(val: Dwl) -> u8 {
        Dwl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Omod {
    #[doc = "I2S format"]
    _00 = 0x0,
    #[doc = "Setting prohibited"]
    _01 = 0x01,
    #[doc = "Monaural format"]
    _10 = 0x02,
    #[doc = "Setting prohibited."]
    _11 = 0x03,
}
impl Omod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Omod {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Omod {
    #[inline(always)]
    fn from(val: u8) -> Omod {
        Omod::from_bits(val)
    }
}
impl From<Omod> for u8 {
    #[inline(always)]
    fn from(val: Omod) -> u8 {
        Omod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdfs {
    #[doc = "SSIFRDR has one stage or more data size"]
    _000 = 0x0,
    #[doc = "SSIFRDR has two stages or more data size (snip)"]
    _001 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "SSIFRDR has seven stages or more data size"]
    _110 = 0x06,
    #[doc = "SSIFRDR has eight stages or more data size."]
    _111 = 0x07,
}
impl Rdfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdfs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdfs {
    #[inline(always)]
    fn from(val: u8) -> Rdfs {
        Rdfs::from_bits(val)
    }
}
impl From<Rdfs> for u8 {
    #[inline(always)]
    fn from(val: Rdfs) -> u8 {
        Rdfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swl {
    #[doc = "8 bits"]
    _000 = 0x0,
    #[doc = "16 bits"]
    _001 = 0x01,
    #[doc = "24 bits"]
    _010 = 0x02,
    #[doc = "32 bits"]
    _011 = 0x03,
    #[doc = "48 bits"]
    _100 = 0x04,
    #[doc = "64 bits"]
    _101 = 0x05,
    #[doc = "128 bits"]
    _110 = 0x06,
    #[doc = "256 bits."]
    _111 = 0x07,
}
impl Swl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swl {
    #[inline(always)]
    fn from(val: u8) -> Swl {
        Swl::from_bits(val)
    }
}
impl From<Swl> for u8 {
    #[inline(always)]
    fn from(val: Swl) -> u8 {
        Swl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdes {
    #[doc = "SSIFTDR has one stage or more free space"]
    _000 = 0x0,
    #[doc = "SSIFTDR has two stages or more free space (snip)"]
    _001 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "SSIFTDR has seven stages or more free space"]
    _110 = 0x06,
    #[doc = "SSIFTDR has eight stages or more free space."]
    _111 = 0x07,
}
impl Tdes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdes {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdes {
    #[inline(always)]
    fn from(val: u8) -> Tdes {
        Tdes::from_bits(val)
    }
}
impl From<Tdes> for u8 {
    #[inline(always)]
    fn from(val: Tdes) -> u8 {
        Tdes::to_bits(val)
    }
}
