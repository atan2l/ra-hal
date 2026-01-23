#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brdv {
    #[doc = "These bits select the base bit rate"]
    _00 = 0x0,
    #[doc = "These bits select the base bit rate divided by 2"]
    _01 = 0x01,
    #[doc = "These bits select the base bit rate divided by 4"]
    _10 = 0x02,
    #[doc = "These bits select the base bit rate divided by 8"]
    _11 = 0x03,
}
impl Brdv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brdv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brdv {
    #[inline(always)]
    fn from(val: u8) -> Brdv {
        Brdv::from_bits(val)
    }
}
impl From<Brdv> for u8 {
    #[inline(always)]
    fn from(val: Brdv) -> u8 {
        Brdv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bysw {
    LittleEndian = 0x0,
    BigEndian = 0x01,
}
impl Bysw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bysw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bysw {
    #[inline(always)]
    fn from(val: u8) -> Bysw {
        Bysw::from_bits(val)
    }
}
impl From<Bysw> for u8 {
    #[inline(always)]
    fn from(val: Bysw) -> u8 {
        Bysw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckdl {
    #[doc = "1 RSPCK"]
    _000 = 0x0,
    #[doc = "2 RSPCK"]
    _001 = 0x01,
    #[doc = "3 RSPCK"]
    _010 = 0x02,
    #[doc = "4 RSPCK"]
    _011 = 0x03,
    #[doc = "5 RSPCK"]
    _100 = 0x04,
    #[doc = "6 RSPCK"]
    _101 = 0x05,
    #[doc = "7 RSPCK"]
    _110 = 0x06,
    #[doc = "8 RSPCK"]
    _111 = 0x07,
}
impl Sckdl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckdl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckdl {
    #[inline(always)]
    fn from(val: u8) -> Sckdl {
        Sckdl::from_bits(val)
    }
}
impl From<Sckdl> for u8 {
    #[inline(always)]
    fn from(val: Sckdl) -> u8 {
        Sckdl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slndl {
    #[doc = "1 RSPCK"]
    _000 = 0x0,
    #[doc = "2 RSPCK"]
    _001 = 0x01,
    #[doc = "3 RSPCK"]
    _010 = 0x02,
    #[doc = "4 RSPCK"]
    _011 = 0x03,
    #[doc = "5 RSPCK"]
    _100 = 0x04,
    #[doc = "6 RSPCK"]
    _101 = 0x05,
    #[doc = "7 RSPCK"]
    _110 = 0x06,
    #[doc = "8 RSPCK"]
    _111 = 0x07,
}
impl Slndl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slndl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slndl {
    #[inline(always)]
    fn from(val: u8) -> Slndl {
        Slndl::from_bits(val)
    }
}
impl From<Slndl> for u8 {
    #[inline(always)]
    fn from(val: Slndl) -> u8 {
        Slndl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spb {
    #[doc = "20 bits"]
    _0000 = 0x0,
    #[doc = "24 bits"]
    _0001 = 0x01,
    #[doc = "32 bits"]
    _0010 = 0x02,
    #[doc = "32 bits"]
    _0011 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "9 bits"]
    _1000 = 0x08,
    #[doc = "10 bits"]
    _1001 = 0x09,
    #[doc = "11 bits"]
    _1010 = 0x0a,
    #[doc = "12 bits"]
    _1011 = 0x0b,
    #[doc = "13 bits"]
    _1100 = 0x0c,
    #[doc = "14 bits"]
    _1101 = 0x0d,
    #[doc = "15 bits"]
    _1110 = 0x0e,
    #[doc = "16 bits"]
    _1111 = 0x0f,
}
impl Spb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spb {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spb {
    #[inline(always)]
    fn from(val: u8) -> Spb {
        Spb::from_bits(val)
    }
}
impl From<Spb> for u8 {
    #[inline(always)]
    fn from(val: Spb) -> u8 {
        Spb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spndl {
    #[doc = "1 RSPCK + 2 PCLK"]
    _000 = 0x0,
    #[doc = "2 RSPCK + 2 PCLK"]
    _001 = 0x01,
    #[doc = "3 RSPCK + 2 PCLK"]
    _010 = 0x02,
    #[doc = "4 RSPCK + 2 PCLK"]
    _011 = 0x03,
    #[doc = "5 RSPCK + 2 PCLK"]
    _100 = 0x04,
    #[doc = "6 RSPCK + 2 PCLK"]
    _101 = 0x05,
    #[doc = "7 RSPCK + 2 PCLK"]
    _110 = 0x06,
    #[doc = "8 RSPCK + 2 PCLK"]
    _111 = 0x07,
}
impl Spndl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spndl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spndl {
    #[inline(always)]
    fn from(val: u8) -> Spndl {
        Spndl::from_bits(val)
    }
}
impl From<Spndl> for u8 {
    #[inline(always)]
    fn from(val: Spndl) -> u8 {
        Spndl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssla {
    #[doc = "SSL0"]
    _000 = 0x0,
    #[doc = "SSL1"]
    _001 = 0x01,
    #[doc = "SSL2"]
    _010 = 0x02,
    #[doc = "SSL3"]
    _011 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ssla {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssla {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssla {
    #[inline(always)]
    fn from(val: u8) -> Ssla {
        Ssla::from_bits(val)
    }
}
impl From<Ssla> for u8 {
    #[inline(always)]
    fn from(val: Ssla) -> u8 {
        Ssla::to_bits(val)
    }
}
