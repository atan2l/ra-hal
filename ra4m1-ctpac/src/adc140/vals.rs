#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ace {
    #[doc = "Disables automatic clearing."]
    _0 = 0x0,
    #[doc = "Enables automatic clearing."]
    _1 = 0x01,
}
impl Ace {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ace {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ace {
    #[inline(always)]
    fn from(val: u8) -> Ace {
        Ace::from_bits(val)
    }
}
impl From<Ace> for u8 {
    #[inline(always)]
    fn from(val: Ace) -> u8 {
        Ace::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc {
    #[doc = "1-time conversion (no addition; same as normal conversion)"]
    _000 = 0x0,
    #[doc = "2-time conversion (addition once)"]
    _001 = 0x01,
    #[doc = "3-time conversion (addition twice)"]
    _010 = 0x02,
    #[doc = "4-time conversion (addition three times)"]
    _011 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "16-time conversion (addition 15 times), can be set when selecting 12-bit accuracy."]
    _101 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Adc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc {
    #[inline(always)]
    fn from(val: u8) -> Adc {
        Adc::from_bits(val)
    }
}
impl From<Adc> for u8 {
    #[inline(always)]
    fn from(val: Adc) -> u8 {
        Adc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adcs {
    #[doc = "Single scan mode"]
    _00 = 0x0,
    #[doc = "Group scan mode"]
    _01 = 0x01,
    #[doc = "Continuous scan mode"]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Adcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adcs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adcs {
    #[inline(always)]
    fn from(val: u8) -> Adcs {
        Adcs::from_bits(val)
    }
}
impl From<Adcs> for u8 {
    #[inline(always)]
    fn from(val: Adcs) -> u8 {
        Adcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adhsc {
    #[doc = "High speed A/D conversion mode"]
    _0 = 0x0,
    #[doc = "Low current A/D conversion mode"]
    _1 = 0x01,
}
impl Adhsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adhsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adhsc {
    #[inline(always)]
    fn from(val: u8) -> Adhsc {
        Adhsc::from_bits(val)
    }
}
impl From<Adhsc> for u8 {
    #[inline(always)]
    fn from(val: Adhsc) -> u8 {
        Adhsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adndis {
    #[doc = "Disconnection detection is disabled"]
    _0000 = 0x0,
    #[doc = "Setting prohibited"]
    _0001 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Adndis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adndis {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adndis {
    #[inline(always)]
    fn from(val: u8) -> Adndis {
        Adndis::from_bits(val)
    }
}
impl From<Adndis> for u8 {
    #[inline(always)]
    fn from(val: Adndis) -> u8 {
        Adndis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adprc {
    #[doc = "A/D conversion is performed with 12-bit accuracy."]
    _00 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "A/D conversion is performed with 14-bit accuracy."]
    _11 = 0x03,
}
impl Adprc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adprc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adprc {
    #[inline(always)]
    fn from(val: u8) -> Adprc {
        Adprc::from_bits(val)
    }
}
impl From<Adprc> for u8 {
    #[inline(always)]
    fn from(val: Adprc) -> u8 {
        Adprc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adrfmt {
    #[doc = "Flush-right is selected for the A/D data register format."]
    _0 = 0x0,
    #[doc = "Flush-left is selected for the A/D data register format."]
    _1 = 0x01,
}
impl Adrfmt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adrfmt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adrfmt {
    #[inline(always)]
    fn from(val: u8) -> Adrfmt {
        Adrfmt::from_bits(val)
    }
}
impl From<Adrfmt> for u8 {
    #[inline(always)]
    fn from(val: Adrfmt) -> u8 {
        Adrfmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads00 {
    #[doc = "AN000 is not selected."]
    _0 = 0x0,
    #[doc = "AN000 is selected."]
    _1 = 0x01,
}
impl Ads00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads00 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads00 {
    #[inline(always)]
    fn from(val: u8) -> Ads00 {
        Ads00::from_bits(val)
    }
}
impl From<Ads00> for u8 {
    #[inline(always)]
    fn from(val: Ads00) -> u8 {
        Ads00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads01 {
    #[doc = "AN001 is not selected."]
    _0 = 0x0,
    #[doc = "AN001 is selected."]
    _1 = 0x01,
}
impl Ads01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads01 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads01 {
    #[inline(always)]
    fn from(val: u8) -> Ads01 {
        Ads01::from_bits(val)
    }
}
impl From<Ads01> for u8 {
    #[inline(always)]
    fn from(val: Ads01) -> u8 {
        Ads01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads02 {
    #[doc = "AN002 is not selected."]
    _0 = 0x0,
    #[doc = "AN002 is selected."]
    _1 = 0x01,
}
impl Ads02 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads02 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads02 {
    #[inline(always)]
    fn from(val: u8) -> Ads02 {
        Ads02::from_bits(val)
    }
}
impl From<Ads02> for u8 {
    #[inline(always)]
    fn from(val: Ads02) -> u8 {
        Ads02::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads03 {
    #[doc = "AN003 is not selected."]
    _0 = 0x0,
    #[doc = "AN003 is selected."]
    _1 = 0x01,
}
impl Ads03 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads03 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads03 {
    #[inline(always)]
    fn from(val: u8) -> Ads03 {
        Ads03::from_bits(val)
    }
}
impl From<Ads03> for u8 {
    #[inline(always)]
    fn from(val: Ads03) -> u8 {
        Ads03::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads04 {
    #[doc = "AN004 is not selected."]
    _0 = 0x0,
    #[doc = "AN004 is selected."]
    _1 = 0x01,
}
impl Ads04 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads04 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads04 {
    #[inline(always)]
    fn from(val: u8) -> Ads04 {
        Ads04::from_bits(val)
    }
}
impl From<Ads04> for u8 {
    #[inline(always)]
    fn from(val: Ads04) -> u8 {
        Ads04::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads05 {
    #[doc = "AN005 is not selected."]
    _0 = 0x0,
    #[doc = "AN005 is selected."]
    _1 = 0x01,
}
impl Ads05 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads05 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads05 {
    #[inline(always)]
    fn from(val: u8) -> Ads05 {
        Ads05::from_bits(val)
    }
}
impl From<Ads05> for u8 {
    #[inline(always)]
    fn from(val: Ads05) -> u8 {
        Ads05::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads06 {
    #[doc = "AN006 is not selected."]
    _0 = 0x0,
    #[doc = "AN006 is selected."]
    _1 = 0x01,
}
impl Ads06 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads06 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads06 {
    #[inline(always)]
    fn from(val: u8) -> Ads06 {
        Ads06::from_bits(val)
    }
}
impl From<Ads06> for u8 {
    #[inline(always)]
    fn from(val: Ads06) -> u8 {
        Ads06::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads07 {
    #[doc = "AN007 is not selected."]
    _0 = 0x0,
    #[doc = "AN007 is selected."]
    _1 = 0x01,
}
impl Ads07 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads07 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads07 {
    #[inline(always)]
    fn from(val: u8) -> Ads07 {
        Ads07::from_bits(val)
    }
}
impl From<Ads07> for u8 {
    #[inline(always)]
    fn from(val: Ads07) -> u8 {
        Ads07::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads08 {
    #[doc = "AN008 is not selected."]
    _0 = 0x0,
    #[doc = "AN008 is selected."]
    _1 = 0x01,
}
impl Ads08 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads08 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads08 {
    #[inline(always)]
    fn from(val: u8) -> Ads08 {
        Ads08::from_bits(val)
    }
}
impl From<Ads08> for u8 {
    #[inline(always)]
    fn from(val: Ads08) -> u8 {
        Ads08::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads09 {
    #[doc = "AN009 is not selected."]
    _0 = 0x0,
    #[doc = "AN009 is selected."]
    _1 = 0x01,
}
impl Ads09 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads09 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads09 {
    #[inline(always)]
    fn from(val: u8) -> Ads09 {
        Ads09::from_bits(val)
    }
}
impl From<Ads09> for u8 {
    #[inline(always)]
    fn from(val: Ads09) -> u8 {
        Ads09::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads10 {
    #[doc = "AN010 is not selected."]
    _0 = 0x0,
    #[doc = "AN010 is selected."]
    _1 = 0x01,
}
impl Ads10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads10 {
    #[inline(always)]
    fn from(val: u8) -> Ads10 {
        Ads10::from_bits(val)
    }
}
impl From<Ads10> for u8 {
    #[inline(always)]
    fn from(val: Ads10) -> u8 {
        Ads10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads11 {
    #[doc = "AN011 is not selected."]
    _0 = 0x0,
    #[doc = "AN011 is selected."]
    _1 = 0x01,
}
impl Ads11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads11 {
    #[inline(always)]
    fn from(val: u8) -> Ads11 {
        Ads11::from_bits(val)
    }
}
impl From<Ads11> for u8 {
    #[inline(always)]
    fn from(val: Ads11) -> u8 {
        Ads11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads12 {
    #[doc = "AN012 is not selected."]
    _0 = 0x0,
    #[doc = "AN012 is selected."]
    _1 = 0x01,
}
impl Ads12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads12 {
    #[inline(always)]
    fn from(val: u8) -> Ads12 {
        Ads12::from_bits(val)
    }
}
impl From<Ads12> for u8 {
    #[inline(always)]
    fn from(val: Ads12) -> u8 {
        Ads12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads13 {
    #[doc = "AN013 is not selected."]
    _0 = 0x0,
    #[doc = "AN013 is selected."]
    _1 = 0x01,
}
impl Ads13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads13 {
    #[inline(always)]
    fn from(val: u8) -> Ads13 {
        Ads13::from_bits(val)
    }
}
impl From<Ads13> for u8 {
    #[inline(always)]
    fn from(val: Ads13) -> u8 {
        Ads13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads14 {
    #[doc = "AN014 is not selected."]
    _0 = 0x0,
    #[doc = "AN014 is selected."]
    _1 = 0x01,
}
impl Ads14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads14 {
    #[inline(always)]
    fn from(val: u8) -> Ads14 {
        Ads14::from_bits(val)
    }
}
impl From<Ads14> for u8 {
    #[inline(always)]
    fn from(val: Ads14) -> u8 {
        Ads14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads16 {
    #[doc = "AN016 is not selected."]
    _0 = 0x0,
    #[doc = "AN016 is selected."]
    _1 = 0x01,
}
impl Ads16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads16 {
    #[inline(always)]
    fn from(val: u8) -> Ads16 {
        Ads16::from_bits(val)
    }
}
impl From<Ads16> for u8 {
    #[inline(always)]
    fn from(val: Ads16) -> u8 {
        Ads16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads17 {
    #[doc = "AN017 is not selected."]
    _0 = 0x0,
    #[doc = "AN017 is selected."]
    _1 = 0x01,
}
impl Ads17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads17 {
    #[inline(always)]
    fn from(val: u8) -> Ads17 {
        Ads17::from_bits(val)
    }
}
impl From<Ads17> for u8 {
    #[inline(always)]
    fn from(val: Ads17) -> u8 {
        Ads17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads18 {
    #[doc = "AN018 is not selected."]
    _0 = 0x0,
    #[doc = "AN018 is selected."]
    _1 = 0x01,
}
impl Ads18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads18 {
    #[inline(always)]
    fn from(val: u8) -> Ads18 {
        Ads18::from_bits(val)
    }
}
impl From<Ads18> for u8 {
    #[inline(always)]
    fn from(val: Ads18) -> u8 {
        Ads18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads19 {
    #[doc = "AN019 is not selected."]
    _0 = 0x0,
    #[doc = "AN019 is selected."]
    _1 = 0x01,
}
impl Ads19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads19 {
    #[inline(always)]
    fn from(val: u8) -> Ads19 {
        Ads19::from_bits(val)
    }
}
impl From<Ads19> for u8 {
    #[inline(always)]
    fn from(val: Ads19) -> u8 {
        Ads19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads20 {
    #[doc = "AN020 is not selected."]
    _0 = 0x0,
    #[doc = "AN020 is selected."]
    _1 = 0x01,
}
impl Ads20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads20 {
    #[inline(always)]
    fn from(val: u8) -> Ads20 {
        Ads20::from_bits(val)
    }
}
impl From<Ads20> for u8 {
    #[inline(always)]
    fn from(val: Ads20) -> u8 {
        Ads20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads21 {
    #[doc = "AN021 is not selected."]
    _0 = 0x0,
    #[doc = "AN021 is selected."]
    _1 = 0x01,
}
impl Ads21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads21 {
    #[inline(always)]
    fn from(val: u8) -> Ads21 {
        Ads21::from_bits(val)
    }
}
impl From<Ads21> for u8 {
    #[inline(always)]
    fn from(val: Ads21) -> u8 {
        Ads21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads22 {
    #[doc = "AN022 is not selected."]
    _0 = 0x0,
    #[doc = "AN022 is selected."]
    _1 = 0x01,
}
impl Ads22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads22 {
    #[inline(always)]
    fn from(val: u8) -> Ads22 {
        Ads22::from_bits(val)
    }
}
impl From<Ads22> for u8 {
    #[inline(always)]
    fn from(val: Ads22) -> u8 {
        Ads22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads23 {
    #[doc = "AN023 is not selected."]
    _0 = 0x0,
    #[doc = "AN023 is selected."]
    _1 = 0x01,
}
impl Ads23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads23 {
    #[inline(always)]
    fn from(val: u8) -> Ads23 {
        Ads23::from_bits(val)
    }
}
impl From<Ads23> for u8 {
    #[inline(always)]
    fn from(val: Ads23) -> u8 {
        Ads23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads24 {
    #[doc = "AN024 is not selected."]
    _0 = 0x0,
    #[doc = "AN024 is selected."]
    _1 = 0x01,
}
impl Ads24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads24 {
    #[inline(always)]
    fn from(val: u8) -> Ads24 {
        Ads24::from_bits(val)
    }
}
impl From<Ads24> for u8 {
    #[inline(always)]
    fn from(val: Ads24) -> u8 {
        Ads24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ads25 {
    #[doc = "AN025 is not selected."]
    _0 = 0x0,
    #[doc = "AN025 is selected."]
    _1 = 0x01,
}
impl Ads25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ads25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ads25 {
    #[inline(always)]
    fn from(val: u8) -> Ads25 {
        Ads25::from_bits(val)
    }
}
impl From<Ads25> for u8 {
    #[inline(always)]
    fn from(val: Ads25) -> u8 {
        Ads25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adslp {
    #[doc = "Normal operation"]
    _0 = 0x0,
    #[doc = "Standby state."]
    _1 = 0x01,
}
impl Adslp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adslp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adslp {
    #[inline(always)]
    fn from(val: u8) -> Adslp {
        Adslp::from_bits(val)
    }
}
impl From<Adslp> for u8 {
    #[inline(always)]
    fn from(val: Adslp) -> u8 {
        Adslp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adst {
    #[doc = "Stops A/D conversion process."]
    _0 = 0x0,
    #[doc = "Starts A/D conversion process."]
    _1 = 0x01,
}
impl Adst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adst {
    #[inline(always)]
    fn from(val: u8) -> Adst {
        Adst::from_bits(val)
    }
}
impl From<Adst> for u8 {
    #[inline(always)]
    fn from(val: Adst) -> u8 {
        Adst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa00 {
    #[doc = "AN000 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN000 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa00 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa00 {
    #[inline(always)]
    fn from(val: u8) -> Ansa00 {
        Ansa00::from_bits(val)
    }
}
impl From<Ansa00> for u8 {
    #[inline(always)]
    fn from(val: Ansa00) -> u8 {
        Ansa00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa01 {
    #[doc = "AN001 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN001 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa01 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa01 {
    #[inline(always)]
    fn from(val: u8) -> Ansa01 {
        Ansa01::from_bits(val)
    }
}
impl From<Ansa01> for u8 {
    #[inline(always)]
    fn from(val: Ansa01) -> u8 {
        Ansa01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa010 {
    #[doc = "AN010 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN010 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa010 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa010 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa010 {
    #[inline(always)]
    fn from(val: u8) -> Ansa010 {
        Ansa010::from_bits(val)
    }
}
impl From<Ansa010> for u8 {
    #[inline(always)]
    fn from(val: Ansa010) -> u8 {
        Ansa010::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa011 {
    #[doc = "AN011 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN011 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa011 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa011 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa011 {
    #[inline(always)]
    fn from(val: u8) -> Ansa011 {
        Ansa011::from_bits(val)
    }
}
impl From<Ansa011> for u8 {
    #[inline(always)]
    fn from(val: Ansa011) -> u8 {
        Ansa011::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa012 {
    #[doc = "AN012 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN012 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa012 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa012 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa012 {
    #[inline(always)]
    fn from(val: u8) -> Ansa012 {
        Ansa012::from_bits(val)
    }
}
impl From<Ansa012> for u8 {
    #[inline(always)]
    fn from(val: Ansa012) -> u8 {
        Ansa012::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa013 {
    #[doc = "AN013 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN013 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa013 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa013 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa013 {
    #[inline(always)]
    fn from(val: u8) -> Ansa013 {
        Ansa013::from_bits(val)
    }
}
impl From<Ansa013> for u8 {
    #[inline(always)]
    fn from(val: Ansa013) -> u8 {
        Ansa013::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa014 {
    #[doc = "AN014 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN014 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa014 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa014 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa014 {
    #[inline(always)]
    fn from(val: u8) -> Ansa014 {
        Ansa014::from_bits(val)
    }
}
impl From<Ansa014> for u8 {
    #[inline(always)]
    fn from(val: Ansa014) -> u8 {
        Ansa014::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa02 {
    #[doc = "AN002 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN002 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa02 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa02 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa02 {
    #[inline(always)]
    fn from(val: u8) -> Ansa02 {
        Ansa02::from_bits(val)
    }
}
impl From<Ansa02> for u8 {
    #[inline(always)]
    fn from(val: Ansa02) -> u8 {
        Ansa02::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa03 {
    #[doc = "AN003 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN003 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa03 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa03 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa03 {
    #[inline(always)]
    fn from(val: u8) -> Ansa03 {
        Ansa03::from_bits(val)
    }
}
impl From<Ansa03> for u8 {
    #[inline(always)]
    fn from(val: Ansa03) -> u8 {
        Ansa03::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa04 {
    #[doc = "AN004 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN004 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa04 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa04 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa04 {
    #[inline(always)]
    fn from(val: u8) -> Ansa04 {
        Ansa04::from_bits(val)
    }
}
impl From<Ansa04> for u8 {
    #[inline(always)]
    fn from(val: Ansa04) -> u8 {
        Ansa04::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa05 {
    #[doc = "AN005 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN005 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa05 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa05 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa05 {
    #[inline(always)]
    fn from(val: u8) -> Ansa05 {
        Ansa05::from_bits(val)
    }
}
impl From<Ansa05> for u8 {
    #[inline(always)]
    fn from(val: Ansa05) -> u8 {
        Ansa05::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa06 {
    #[doc = "AN006 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN006 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa06 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa06 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa06 {
    #[inline(always)]
    fn from(val: u8) -> Ansa06 {
        Ansa06::from_bits(val)
    }
}
impl From<Ansa06> for u8 {
    #[inline(always)]
    fn from(val: Ansa06) -> u8 {
        Ansa06::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa07 {
    #[doc = "AN007 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN007 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa07 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa07 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa07 {
    #[inline(always)]
    fn from(val: u8) -> Ansa07 {
        Ansa07::from_bits(val)
    }
}
impl From<Ansa07> for u8 {
    #[inline(always)]
    fn from(val: Ansa07) -> u8 {
        Ansa07::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa08 {
    #[doc = "AN008 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN008 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa08 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa08 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa08 {
    #[inline(always)]
    fn from(val: u8) -> Ansa08 {
        Ansa08::from_bits(val)
    }
}
impl From<Ansa08> for u8 {
    #[inline(always)]
    fn from(val: Ansa08) -> u8 {
        Ansa08::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa09 {
    #[doc = "AN009 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN009 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa09 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa09 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa09 {
    #[inline(always)]
    fn from(val: u8) -> Ansa09 {
        Ansa09::from_bits(val)
    }
}
impl From<Ansa09> for u8 {
    #[inline(always)]
    fn from(val: Ansa09) -> u8 {
        Ansa09::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa16 {
    #[doc = "AN016 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN016 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa16 {
    #[inline(always)]
    fn from(val: u8) -> Ansa16 {
        Ansa16::from_bits(val)
    }
}
impl From<Ansa16> for u8 {
    #[inline(always)]
    fn from(val: Ansa16) -> u8 {
        Ansa16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa17 {
    #[doc = "AN017 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN017 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa17 {
    #[inline(always)]
    fn from(val: u8) -> Ansa17 {
        Ansa17::from_bits(val)
    }
}
impl From<Ansa17> for u8 {
    #[inline(always)]
    fn from(val: Ansa17) -> u8 {
        Ansa17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa18 {
    #[doc = "AN018 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN018 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa18 {
    #[inline(always)]
    fn from(val: u8) -> Ansa18 {
        Ansa18::from_bits(val)
    }
}
impl From<Ansa18> for u8 {
    #[inline(always)]
    fn from(val: Ansa18) -> u8 {
        Ansa18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa19 {
    #[doc = "AN019 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN019 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa19 {
    #[inline(always)]
    fn from(val: u8) -> Ansa19 {
        Ansa19::from_bits(val)
    }
}
impl From<Ansa19> for u8 {
    #[inline(always)]
    fn from(val: Ansa19) -> u8 {
        Ansa19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa20 {
    #[doc = "AN020 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN020 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa20 {
    #[inline(always)]
    fn from(val: u8) -> Ansa20 {
        Ansa20::from_bits(val)
    }
}
impl From<Ansa20> for u8 {
    #[inline(always)]
    fn from(val: Ansa20) -> u8 {
        Ansa20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa21 {
    #[doc = "AN021 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN021 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa21 {
    #[inline(always)]
    fn from(val: u8) -> Ansa21 {
        Ansa21::from_bits(val)
    }
}
impl From<Ansa21> for u8 {
    #[inline(always)]
    fn from(val: Ansa21) -> u8 {
        Ansa21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa22 {
    #[doc = "AN022 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN022 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa22 {
    #[inline(always)]
    fn from(val: u8) -> Ansa22 {
        Ansa22::from_bits(val)
    }
}
impl From<Ansa22> for u8 {
    #[inline(always)]
    fn from(val: Ansa22) -> u8 {
        Ansa22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa23 {
    #[doc = "AN023 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN023 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa23 {
    #[inline(always)]
    fn from(val: u8) -> Ansa23 {
        Ansa23::from_bits(val)
    }
}
impl From<Ansa23> for u8 {
    #[inline(always)]
    fn from(val: Ansa23) -> u8 {
        Ansa23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa24 {
    #[doc = "AN024 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN024 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa24 {
    #[inline(always)]
    fn from(val: u8) -> Ansa24 {
        Ansa24::from_bits(val)
    }
}
impl From<Ansa24> for u8 {
    #[inline(always)]
    fn from(val: Ansa24) -> u8 {
        Ansa24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansa25 {
    #[doc = "AN025 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN025 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansa25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansa25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansa25 {
    #[inline(always)]
    fn from(val: u8) -> Ansa25 {
        Ansa25::from_bits(val)
    }
}
impl From<Ansa25> for u8 {
    #[inline(always)]
    fn from(val: Ansa25) -> u8 {
        Ansa25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb00 {
    #[doc = "AN000 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN000 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb00 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb00 {
    #[inline(always)]
    fn from(val: u8) -> Ansb00 {
        Ansb00::from_bits(val)
    }
}
impl From<Ansb00> for u8 {
    #[inline(always)]
    fn from(val: Ansb00) -> u8 {
        Ansb00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb01 {
    #[doc = "AN001 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN001 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb01 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb01 {
    #[inline(always)]
    fn from(val: u8) -> Ansb01 {
        Ansb01::from_bits(val)
    }
}
impl From<Ansb01> for u8 {
    #[inline(always)]
    fn from(val: Ansb01) -> u8 {
        Ansb01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb02 {
    #[doc = "AN002 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN002 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb02 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb02 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb02 {
    #[inline(always)]
    fn from(val: u8) -> Ansb02 {
        Ansb02::from_bits(val)
    }
}
impl From<Ansb02> for u8 {
    #[inline(always)]
    fn from(val: Ansb02) -> u8 {
        Ansb02::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb03 {
    #[doc = "AN003 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN003 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb03 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb03 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb03 {
    #[inline(always)]
    fn from(val: u8) -> Ansb03 {
        Ansb03::from_bits(val)
    }
}
impl From<Ansb03> for u8 {
    #[inline(always)]
    fn from(val: Ansb03) -> u8 {
        Ansb03::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb04 {
    #[doc = "AN004 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN004 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb04 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb04 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb04 {
    #[inline(always)]
    fn from(val: u8) -> Ansb04 {
        Ansb04::from_bits(val)
    }
}
impl From<Ansb04> for u8 {
    #[inline(always)]
    fn from(val: Ansb04) -> u8 {
        Ansb04::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb05 {
    #[doc = "AN005 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN005 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb05 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb05 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb05 {
    #[inline(always)]
    fn from(val: u8) -> Ansb05 {
        Ansb05::from_bits(val)
    }
}
impl From<Ansb05> for u8 {
    #[inline(always)]
    fn from(val: Ansb05) -> u8 {
        Ansb05::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb06 {
    #[doc = "AN006 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN006 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb06 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb06 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb06 {
    #[inline(always)]
    fn from(val: u8) -> Ansb06 {
        Ansb06::from_bits(val)
    }
}
impl From<Ansb06> for u8 {
    #[inline(always)]
    fn from(val: Ansb06) -> u8 {
        Ansb06::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb07 {
    #[doc = "AN007 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN007 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb07 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb07 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb07 {
    #[inline(always)]
    fn from(val: u8) -> Ansb07 {
        Ansb07::from_bits(val)
    }
}
impl From<Ansb07> for u8 {
    #[inline(always)]
    fn from(val: Ansb07) -> u8 {
        Ansb07::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb08 {
    #[doc = "AN008 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN008 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb08 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb08 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb08 {
    #[inline(always)]
    fn from(val: u8) -> Ansb08 {
        Ansb08::from_bits(val)
    }
}
impl From<Ansb08> for u8 {
    #[inline(always)]
    fn from(val: Ansb08) -> u8 {
        Ansb08::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb09 {
    #[doc = "AN009 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN009 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb09 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb09 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb09 {
    #[inline(always)]
    fn from(val: u8) -> Ansb09 {
        Ansb09::from_bits(val)
    }
}
impl From<Ansb09> for u8 {
    #[inline(always)]
    fn from(val: Ansb09) -> u8 {
        Ansb09::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb10 {
    #[doc = "AN010 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN010 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb10 {
    #[inline(always)]
    fn from(val: u8) -> Ansb10 {
        Ansb10::from_bits(val)
    }
}
impl From<Ansb10> for u8 {
    #[inline(always)]
    fn from(val: Ansb10) -> u8 {
        Ansb10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb11 {
    #[doc = "AN011 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN011 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb11 {
    #[inline(always)]
    fn from(val: u8) -> Ansb11 {
        Ansb11::from_bits(val)
    }
}
impl From<Ansb11> for u8 {
    #[inline(always)]
    fn from(val: Ansb11) -> u8 {
        Ansb11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb12 {
    #[doc = "AN012 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN012 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb12 {
    #[inline(always)]
    fn from(val: u8) -> Ansb12 {
        Ansb12::from_bits(val)
    }
}
impl From<Ansb12> for u8 {
    #[inline(always)]
    fn from(val: Ansb12) -> u8 {
        Ansb12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb13 {
    #[doc = "AN013 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN013 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb13 {
    #[inline(always)]
    fn from(val: u8) -> Ansb13 {
        Ansb13::from_bits(val)
    }
}
impl From<Ansb13> for u8 {
    #[inline(always)]
    fn from(val: Ansb13) -> u8 {
        Ansb13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb14 {
    #[doc = "AN014 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN014 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb14 {
    #[inline(always)]
    fn from(val: u8) -> Ansb14 {
        Ansb14::from_bits(val)
    }
}
impl From<Ansb14> for u8 {
    #[inline(always)]
    fn from(val: Ansb14) -> u8 {
        Ansb14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb16 {
    #[doc = "AN016 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN016 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb16 {
    #[inline(always)]
    fn from(val: u8) -> Ansb16 {
        Ansb16::from_bits(val)
    }
}
impl From<Ansb16> for u8 {
    #[inline(always)]
    fn from(val: Ansb16) -> u8 {
        Ansb16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb17 {
    #[doc = "AN017 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN017 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb17 {
    #[inline(always)]
    fn from(val: u8) -> Ansb17 {
        Ansb17::from_bits(val)
    }
}
impl From<Ansb17> for u8 {
    #[inline(always)]
    fn from(val: Ansb17) -> u8 {
        Ansb17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb18 {
    #[doc = "AN018 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN018 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb18 {
    #[inline(always)]
    fn from(val: u8) -> Ansb18 {
        Ansb18::from_bits(val)
    }
}
impl From<Ansb18> for u8 {
    #[inline(always)]
    fn from(val: Ansb18) -> u8 {
        Ansb18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb19 {
    #[doc = "AN019 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN019 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb19 {
    #[inline(always)]
    fn from(val: u8) -> Ansb19 {
        Ansb19::from_bits(val)
    }
}
impl From<Ansb19> for u8 {
    #[inline(always)]
    fn from(val: Ansb19) -> u8 {
        Ansb19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb20 {
    #[doc = "AN020 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN020 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb20 {
    #[inline(always)]
    fn from(val: u8) -> Ansb20 {
        Ansb20::from_bits(val)
    }
}
impl From<Ansb20> for u8 {
    #[inline(always)]
    fn from(val: Ansb20) -> u8 {
        Ansb20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb21 {
    #[doc = "AN021 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN021 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb21 {
    #[inline(always)]
    fn from(val: u8) -> Ansb21 {
        Ansb21::from_bits(val)
    }
}
impl From<Ansb21> for u8 {
    #[inline(always)]
    fn from(val: Ansb21) -> u8 {
        Ansb21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb22 {
    #[doc = "AN022 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN022 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb22 {
    #[inline(always)]
    fn from(val: u8) -> Ansb22 {
        Ansb22::from_bits(val)
    }
}
impl From<Ansb22> for u8 {
    #[inline(always)]
    fn from(val: Ansb22) -> u8 {
        Ansb22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb23 {
    #[doc = "AN023 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN023 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb23 {
    #[inline(always)]
    fn from(val: u8) -> Ansb23 {
        Ansb23::from_bits(val)
    }
}
impl From<Ansb23> for u8 {
    #[inline(always)]
    fn from(val: Ansb23) -> u8 {
        Ansb23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb24 {
    #[doc = "AN024 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN024 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb24 {
    #[inline(always)]
    fn from(val: u8) -> Ansb24 {
        Ansb24::from_bits(val)
    }
}
impl From<Ansb24> for u8 {
    #[inline(always)]
    fn from(val: Ansb24) -> u8 {
        Ansb24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ansb25 {
    #[doc = "AN025 is not subjected to conversion."]
    _0 = 0x0,
    #[doc = "AN025 is subjected to conversion."]
    _1 = 0x01,
}
impl Ansb25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ansb25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ansb25 {
    #[inline(always)]
    fn from(val: u8) -> Ansb25 {
        Ansb25::from_bits(val)
    }
}
impl From<Ansb25> for u8 {
    #[inline(always)]
    fn from(val: Ansb25) -> u8 {
        Ansb25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Avee {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled"]
    _1 = 0x01,
}
impl Avee {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avee {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avee {
    #[inline(always)]
    fn from(val: u8) -> Avee {
        Avee::from_bits(val)
    }
}
impl From<Avee> for u8 {
    #[inline(always)]
    fn from(val: Avee) -> u8 {
        Avee::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpab {
    #[doc = "ADC140_WCMPM is output when window A comparison conditions are met OR window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    _00 = 0x0,
    #[doc = "S14ADWMELC0 is output when window A comparison conditions are met EXOR window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    _01 = 0x01,
    #[doc = "ADC140_WCMPM is output when window A comparison conditions are met and window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    _10 = 0x02,
    #[doc = "Setting prohibited."]
    _11 = 0x03,
}
impl Cmpab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpab {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpab {
    #[inline(always)]
    fn from(val: u8) -> Cmpab {
        Cmpab::from_bits(val)
    }
}
impl From<Cmpab> for u8 {
    #[inline(always)]
    fn from(val: Cmpab) -> u8 {
        Cmpab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpae {
    #[doc = "Compare window A operation is disabled. ADC140_WCMPM and ADC140_WCMPUM outputs are disabled."]
    _0 = 0x0,
    #[doc = "Compare window A operation is enabled."]
    _1 = 0x01,
}
impl Cmpae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpae {
    #[inline(always)]
    fn from(val: u8) -> Cmpae {
        Cmpae::from_bits(val)
    }
}
impl From<Cmpae> for u8 {
    #[inline(always)]
    fn from(val: Cmpae) -> u8 {
        Cmpae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpaie {
    #[doc = "ADC140_CMPAI interrupt is disabled when comparison conditions (window A) are met."]
    _0 = 0x0,
    #[doc = "ADC140_CMPAI interrupt is enabled when comparison conditions (window A) are met."]
    _1 = 0x01,
}
impl Cmpaie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpaie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpaie {
    #[inline(always)]
    fn from(val: u8) -> Cmpaie {
        Cmpaie::from_bits(val)
    }
}
impl From<Cmpaie> for u8 {
    #[inline(always)]
    fn from(val: Cmpaie) -> u8 {
        Cmpaie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpbe {
    #[doc = "Compare window B operation is disabled. ADC140_WCMPM and ADC140_WCMPUM outputs are disabled."]
    _0 = 0x0,
    #[doc = "Compare window B operation is enabled."]
    _1 = 0x01,
}
impl Cmpbe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpbe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpbe {
    #[inline(always)]
    fn from(val: u8) -> Cmpbe {
        Cmpbe::from_bits(val)
    }
}
impl From<Cmpbe> for u8 {
    #[inline(always)]
    fn from(val: Cmpbe) -> u8 {
        Cmpbe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpbie {
    #[doc = "ADC140_CMPAI interrupt is disabled when comparison conditions (window B) are met."]
    _0 = 0x0,
    #[doc = "ADC140_CMPAI interrupt is enabled when comparison conditions (window B) are met."]
    _1 = 0x01,
}
impl Cmpbie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpbie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpbie {
    #[inline(always)]
    fn from(val: u8) -> Cmpbie {
        Cmpbie::from_bits(val)
    }
}
impl From<Cmpbie> for u8 {
    #[inline(always)]
    fn from(val: Cmpbie) -> u8 {
        Cmpbie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha00 {
    #[doc = "Excludes AN000 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN000 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha00 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha00 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha00 {
        Cmpcha00::from_bits(val)
    }
}
impl From<Cmpcha00> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha00) -> u8 {
        Cmpcha00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha01 {
    #[doc = "Excludes AN001 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN001 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha01 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha01 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha01 {
        Cmpcha01::from_bits(val)
    }
}
impl From<Cmpcha01> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha01) -> u8 {
        Cmpcha01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha02 {
    #[doc = "Excludes AN002 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN002 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha02 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha02 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha02 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha02 {
        Cmpcha02::from_bits(val)
    }
}
impl From<Cmpcha02> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha02) -> u8 {
        Cmpcha02::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha03 {
    #[doc = "Excludes AN003 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN003 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha03 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha03 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha03 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha03 {
        Cmpcha03::from_bits(val)
    }
}
impl From<Cmpcha03> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha03) -> u8 {
        Cmpcha03::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha04 {
    #[doc = "Excludes AN004 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN004 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha04 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha04 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha04 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha04 {
        Cmpcha04::from_bits(val)
    }
}
impl From<Cmpcha04> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha04) -> u8 {
        Cmpcha04::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha05 {
    #[doc = "Excludes AN005 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN005 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha05 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha05 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha05 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha05 {
        Cmpcha05::from_bits(val)
    }
}
impl From<Cmpcha05> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha05) -> u8 {
        Cmpcha05::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha06 {
    #[doc = "Excludes AN006 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN006 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha06 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha06 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha06 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha06 {
        Cmpcha06::from_bits(val)
    }
}
impl From<Cmpcha06> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha06) -> u8 {
        Cmpcha06::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha07 {
    #[doc = "Excludes AN007 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN007 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha07 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha07 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha07 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha07 {
        Cmpcha07::from_bits(val)
    }
}
impl From<Cmpcha07> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha07) -> u8 {
        Cmpcha07::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha08 {
    #[doc = "Excludes AN008 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN008 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha08 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha08 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha08 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha08 {
        Cmpcha08::from_bits(val)
    }
}
impl From<Cmpcha08> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha08) -> u8 {
        Cmpcha08::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha09 {
    #[doc = "Excludes AN009 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN009 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha09 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha09 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha09 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha09 {
        Cmpcha09::from_bits(val)
    }
}
impl From<Cmpcha09> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha09) -> u8 {
        Cmpcha09::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha10 {
    #[doc = "Excludes AN010 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN010 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha10 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha10 {
        Cmpcha10::from_bits(val)
    }
}
impl From<Cmpcha10> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha10) -> u8 {
        Cmpcha10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha11 {
    #[doc = "Excludes AN011 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN011 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha11 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha11 {
        Cmpcha11::from_bits(val)
    }
}
impl From<Cmpcha11> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha11) -> u8 {
        Cmpcha11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha12 {
    #[doc = "Excludes AN012 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN012 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha12 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha12 {
        Cmpcha12::from_bits(val)
    }
}
impl From<Cmpcha12> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha12) -> u8 {
        Cmpcha12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha13 {
    #[doc = "Excludes AN013 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN013 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha13 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha13 {
        Cmpcha13::from_bits(val)
    }
}
impl From<Cmpcha13> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha13) -> u8 {
        Cmpcha13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha14 {
    #[doc = "Excludes AN014 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN014 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha14 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha14 {
        Cmpcha14::from_bits(val)
    }
}
impl From<Cmpcha14> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha14) -> u8 {
        Cmpcha14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha16 {
    #[doc = "Excludes AN016 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN016 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha16 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha16 {
        Cmpcha16::from_bits(val)
    }
}
impl From<Cmpcha16> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha16) -> u8 {
        Cmpcha16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha17 {
    #[doc = "Excludes AN017 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN017 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha17 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha17 {
        Cmpcha17::from_bits(val)
    }
}
impl From<Cmpcha17> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha17) -> u8 {
        Cmpcha17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha18 {
    #[doc = "Excludes AN018 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN018 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha18 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha18 {
        Cmpcha18::from_bits(val)
    }
}
impl From<Cmpcha18> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha18) -> u8 {
        Cmpcha18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha19 {
    #[doc = "Excludes AN019 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN019 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha19 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha19 {
        Cmpcha19::from_bits(val)
    }
}
impl From<Cmpcha19> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha19) -> u8 {
        Cmpcha19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha20 {
    #[doc = "Excludes AN020 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN020 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha20 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha20 {
        Cmpcha20::from_bits(val)
    }
}
impl From<Cmpcha20> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha20) -> u8 {
        Cmpcha20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha21 {
    #[doc = "Excludes AN021 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN021 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha21 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha21 {
        Cmpcha21::from_bits(val)
    }
}
impl From<Cmpcha21> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha21) -> u8 {
        Cmpcha21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha22 {
    #[doc = "Excludes AN022 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN022 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha22 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha22 {
        Cmpcha22::from_bits(val)
    }
}
impl From<Cmpcha22> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha22) -> u8 {
        Cmpcha22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha23 {
    #[doc = "Excludes AN023 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN023 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha23 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha23 {
        Cmpcha23::from_bits(val)
    }
}
impl From<Cmpcha23> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha23) -> u8 {
        Cmpcha23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha24 {
    #[doc = "Excludes AN024 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN024 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha24 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha24 {
        Cmpcha24::from_bits(val)
    }
}
impl From<Cmpcha24> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha24) -> u8 {
        Cmpcha24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpcha25 {
    #[doc = "Excludes AN025 from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes AN025 from the compare window A target range."]
    _1 = 0x01,
}
impl Cmpcha25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpcha25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpcha25 {
    #[inline(always)]
    fn from(val: u8) -> Cmpcha25 {
        Cmpcha25::from_bits(val)
    }
}
impl From<Cmpcha25> for u8 {
    #[inline(always)]
    fn from(val: Cmpcha25) -> u8 {
        Cmpcha25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpchb {
    #[doc = "AN000"]
    _0X00 = 0x0,
    #[doc = "AN001"]
    _0X01 = 0x01,
    #[doc = "AN002"]
    _0X02 = 0x02,
    #[doc = "AN003"]
    _0X03 = 0x03,
    #[doc = "AN004"]
    _0X04 = 0x04,
    #[doc = "AN005"]
    _0X05 = 0x05,
    #[doc = "AN006"]
    _0X06 = 0x06,
    #[doc = "AN007"]
    _0X07 = 0x07,
    #[doc = "AN008"]
    _0X08 = 0x08,
    #[doc = "AN009"]
    _0X09 = 0x09,
    #[doc = "AN010"]
    _0X0A = 0x0a,
    #[doc = "AN011"]
    _0X0B = 0x0b,
    #[doc = "AN012"]
    _0X0C = 0x0c,
    #[doc = "AN013"]
    _0X0D = 0x0d,
    #[doc = "AN014"]
    _0X0E = 0x0e,
    #[doc = "AN015"]
    _0X0F = 0x0f,
    #[doc = "AN016"]
    _0X10 = 0x10,
    #[doc = "AN017"]
    _0X11 = 0x11,
    #[doc = "AN018"]
    _0X12 = 0x12,
    #[doc = "AN019"]
    _0X13 = 0x13,
    #[doc = "AN020"]
    _0X14 = 0x14,
    #[doc = "AN021"]
    _0X15 = 0x15,
    #[doc = "AN022"]
    _0X16 = 0x16,
    #[doc = "AN023"]
    _0X17 = 0x17,
    #[doc = "AN024"]
    _0X18 = 0x18,
    #[doc = "AN025"]
    _0X19 = 0x19,
    #[doc = "AN026"]
    _0X1A = 0x1a,
    #[doc = "AN027"]
    _0X1B = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "Temperature sensor"]
    _0X20 = 0x20,
    #[doc = "Internal reference voltage"]
    _0X21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "No channel is selected"]
    _0X3F = 0x3f,
}
impl Cmpchb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpchb {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpchb {
    #[inline(always)]
    fn from(val: u8) -> Cmpchb {
        Cmpchb::from_bits(val)
    }
}
impl From<Cmpchb> for u8 {
    #[inline(always)]
    fn from(val: Cmpchb) -> u8 {
        Cmpchb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplb {
    #[doc = "CMPLLB value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < CMPLLB value or CMPULB value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "CMPLLB value < A/D converted value(ADCMPCR.WCMPE=0) / CMPLLB value < A/D converted value < CMPULB value (ADCMPCR.WCMPE=1)"]
    _1 = 0x01,
}
impl Cmplb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplb {
    #[inline(always)]
    fn from(val: u8) -> Cmplb {
        Cmplb::from_bits(val)
    }
}
impl From<Cmplb> for u8 {
    #[inline(always)]
    fn from(val: Cmplb) -> u8 {
        Cmplb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha00 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha00 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha00 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha00 {
        Cmplcha00::from_bits(val)
    }
}
impl From<Cmplcha00> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha00) -> u8 {
        Cmplcha00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha01 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha01 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha01 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha01 {
        Cmplcha01::from_bits(val)
    }
}
impl From<Cmplcha01> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha01) -> u8 {
        Cmplcha01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha02 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha02 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha02 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha02 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha02 {
        Cmplcha02::from_bits(val)
    }
}
impl From<Cmplcha02> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha02) -> u8 {
        Cmplcha02::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha03 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha03 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha03 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha03 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha03 {
        Cmplcha03::from_bits(val)
    }
}
impl From<Cmplcha03> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha03) -> u8 {
        Cmplcha03::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha04 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha04 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha04 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha04 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha04 {
        Cmplcha04::from_bits(val)
    }
}
impl From<Cmplcha04> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha04) -> u8 {
        Cmplcha04::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha05 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha05 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha05 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha05 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha05 {
        Cmplcha05::from_bits(val)
    }
}
impl From<Cmplcha05> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha05) -> u8 {
        Cmplcha05::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha06 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha06 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha06 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha06 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha06 {
        Cmplcha06::from_bits(val)
    }
}
impl From<Cmplcha06> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha06) -> u8 {
        Cmplcha06::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha07 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha07 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha07 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha07 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha07 {
        Cmplcha07::from_bits(val)
    }
}
impl From<Cmplcha07> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha07) -> u8 {
        Cmplcha07::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha08 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha08 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha08 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha08 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha08 {
        Cmplcha08::from_bits(val)
    }
}
impl From<Cmplcha08> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha08) -> u8 {
        Cmplcha08::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha09 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha09 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha09 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha09 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha09 {
        Cmplcha09::from_bits(val)
    }
}
impl From<Cmplcha09> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha09) -> u8 {
        Cmplcha09::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha10 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha10 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha10 {
        Cmplcha10::from_bits(val)
    }
}
impl From<Cmplcha10> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha10) -> u8 {
        Cmplcha10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha11 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha11 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha11 {
        Cmplcha11::from_bits(val)
    }
}
impl From<Cmplcha11> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha11) -> u8 {
        Cmplcha11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha12 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha12 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha12 {
        Cmplcha12::from_bits(val)
    }
}
impl From<Cmplcha12> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha12) -> u8 {
        Cmplcha12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha13 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha13 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha13 {
        Cmplcha13::from_bits(val)
    }
}
impl From<Cmplcha13> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha13) -> u8 {
        Cmplcha13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha14 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha14 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha14 {
        Cmplcha14::from_bits(val)
    }
}
impl From<Cmplcha14> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha14) -> u8 {
        Cmplcha14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha16 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha16 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha16 {
        Cmplcha16::from_bits(val)
    }
}
impl From<Cmplcha16> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha16) -> u8 {
        Cmplcha16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha17 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha17 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha17 {
        Cmplcha17::from_bits(val)
    }
}
impl From<Cmplcha17> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha17) -> u8 {
        Cmplcha17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha18 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha18 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha18 {
        Cmplcha18::from_bits(val)
    }
}
impl From<Cmplcha18> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha18) -> u8 {
        Cmplcha18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha19 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha19 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha19 {
        Cmplcha19::from_bits(val)
    }
}
impl From<Cmplcha19> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha19) -> u8 {
        Cmplcha19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha20 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha20 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha20 {
        Cmplcha20::from_bits(val)
    }
}
impl From<Cmplcha20> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha20) -> u8 {
        Cmplcha20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha21 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha21 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha21 {
        Cmplcha21::from_bits(val)
    }
}
impl From<Cmplcha21> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha21) -> u8 {
        Cmplcha21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha22 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha22 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha22 {
        Cmplcha22::from_bits(val)
    }
}
impl From<Cmplcha22> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha22) -> u8 {
        Cmplcha22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha23 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha23 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha23 {
        Cmplcha23::from_bits(val)
    }
}
impl From<Cmplcha23> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha23) -> u8 {
        Cmplcha23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha24 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha24 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha24 {
        Cmplcha24::from_bits(val)
    }
}
impl From<Cmplcha24> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha24) -> u8 {
        Cmplcha24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmplcha25 {
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmplcha25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmplcha25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmplcha25 {
    #[inline(always)]
    fn from(val: u8) -> Cmplcha25 {
        Cmplcha25::from_bits(val)
    }
}
impl From<Cmplcha25> for u8 {
    #[inline(always)]
    fn from(val: Cmplcha25) -> u8 {
        Cmplcha25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmploca {
    #[doc = "ADCMPDR0 value > A/D converted value(ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or A/D converted value > ADCMPDR1 value (ADCMPCR.WCMPE=1)"]
    _0 = 0x0,
    #[doc = "ADCMPDR0 value < A/D converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 value < A/D converted value < ADCMPDR1 value(ADCMPCR.WCMPE=1)"]
    _1 = 0x01,
}
impl Cmploca {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmploca {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmploca {
    #[inline(always)]
    fn from(val: u8) -> Cmploca {
        Cmploca::from_bits(val)
    }
}
impl From<Cmploca> for u8 {
    #[inline(always)]
    fn from(val: Cmploca) -> u8 {
        Cmploca::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpltsa {
    #[doc = "ADCMPDR0 register value > A/D-converted value(ADCMPCR.WCMPE=0) / AD-converted value < ADCMPDR0 register value or A/D-converted value > ADCMPDR1 register value(ADCMPCR.WCMPE=1)."]
    _0 = 0x0,
    #[doc = "ADCMPDR0 register value < A/D-converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 register value < A/D-converted value < ADCMPDR1 register value(ADCMPCR.WCMPE=1)."]
    _1 = 0x01,
}
impl Cmpltsa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpltsa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpltsa {
    #[inline(always)]
    fn from(val: u8) -> Cmpltsa {
        Cmpltsa::from_bits(val)
    }
}
impl From<Cmpltsa> for u8 {
    #[inline(always)]
    fn from(val: Cmpltsa) -> u8 {
        Cmpltsa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpoca {
    #[doc = "Excludes the internal reference voltage from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes the internal reference voltage in the compare window A target range."]
    _1 = 0x01,
}
impl Cmpoca {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpoca {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpoca {
    #[inline(always)]
    fn from(val: u8) -> Cmpoca {
        Cmpoca::from_bits(val)
    }
}
impl From<Cmpoca> for u8 {
    #[inline(always)]
    fn from(val: Cmpoca) -> u8 {
        Cmpoca::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstb {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstb {
    #[inline(always)]
    fn from(val: u8) -> Cmpstb {
        Cmpstb::from_bits(val)
    }
}
impl From<Cmpstb> for u8 {
    #[inline(always)]
    fn from(val: Cmpstb) -> u8 {
        Cmpstb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha00 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha00 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha00 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha00 {
        Cmpstcha00::from_bits(val)
    }
}
impl From<Cmpstcha00> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha00) -> u8 {
        Cmpstcha00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha01 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha01 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha01 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha01 {
        Cmpstcha01::from_bits(val)
    }
}
impl From<Cmpstcha01> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha01) -> u8 {
        Cmpstcha01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha02 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha02 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha02 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha02 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha02 {
        Cmpstcha02::from_bits(val)
    }
}
impl From<Cmpstcha02> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha02) -> u8 {
        Cmpstcha02::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha03 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha03 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha03 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha03 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha03 {
        Cmpstcha03::from_bits(val)
    }
}
impl From<Cmpstcha03> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha03) -> u8 {
        Cmpstcha03::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha04 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha04 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha04 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha04 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha04 {
        Cmpstcha04::from_bits(val)
    }
}
impl From<Cmpstcha04> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha04) -> u8 {
        Cmpstcha04::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha05 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha05 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha05 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha05 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha05 {
        Cmpstcha05::from_bits(val)
    }
}
impl From<Cmpstcha05> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha05) -> u8 {
        Cmpstcha05::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha06 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha06 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha06 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha06 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha06 {
        Cmpstcha06::from_bits(val)
    }
}
impl From<Cmpstcha06> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha06) -> u8 {
        Cmpstcha06::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha07 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha07 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha07 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha07 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha07 {
        Cmpstcha07::from_bits(val)
    }
}
impl From<Cmpstcha07> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha07) -> u8 {
        Cmpstcha07::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha08 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha08 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha08 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha08 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha08 {
        Cmpstcha08::from_bits(val)
    }
}
impl From<Cmpstcha08> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha08) -> u8 {
        Cmpstcha08::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha09 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha09 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha09 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha09 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha09 {
        Cmpstcha09::from_bits(val)
    }
}
impl From<Cmpstcha09> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha09) -> u8 {
        Cmpstcha09::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha10 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha10 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha10 {
        Cmpstcha10::from_bits(val)
    }
}
impl From<Cmpstcha10> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha10) -> u8 {
        Cmpstcha10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha11 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha11 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha11 {
        Cmpstcha11::from_bits(val)
    }
}
impl From<Cmpstcha11> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha11) -> u8 {
        Cmpstcha11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha12 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha12 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha12 {
        Cmpstcha12::from_bits(val)
    }
}
impl From<Cmpstcha12> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha12) -> u8 {
        Cmpstcha12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha13 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha13 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha13 {
        Cmpstcha13::from_bits(val)
    }
}
impl From<Cmpstcha13> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha13) -> u8 {
        Cmpstcha13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha14 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha14 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha14 {
        Cmpstcha14::from_bits(val)
    }
}
impl From<Cmpstcha14> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha14) -> u8 {
        Cmpstcha14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha16 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha16 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha16 {
        Cmpstcha16::from_bits(val)
    }
}
impl From<Cmpstcha16> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha16) -> u8 {
        Cmpstcha16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha17 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha17 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha17 {
        Cmpstcha17::from_bits(val)
    }
}
impl From<Cmpstcha17> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha17) -> u8 {
        Cmpstcha17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha18 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha18 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha18 {
        Cmpstcha18::from_bits(val)
    }
}
impl From<Cmpstcha18> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha18) -> u8 {
        Cmpstcha18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha19 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha19 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha19 {
        Cmpstcha19::from_bits(val)
    }
}
impl From<Cmpstcha19> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha19) -> u8 {
        Cmpstcha19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha20 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha20 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha20 {
        Cmpstcha20::from_bits(val)
    }
}
impl From<Cmpstcha20> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha20) -> u8 {
        Cmpstcha20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha21 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha21 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha21 {
        Cmpstcha21::from_bits(val)
    }
}
impl From<Cmpstcha21> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha21) -> u8 {
        Cmpstcha21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha22 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha22 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha22 {
        Cmpstcha22::from_bits(val)
    }
}
impl From<Cmpstcha22> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha22) -> u8 {
        Cmpstcha22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha23 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha23 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha23 {
        Cmpstcha23::from_bits(val)
    }
}
impl From<Cmpstcha23> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha23) -> u8 {
        Cmpstcha23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha24 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha24 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha24 {
        Cmpstcha24::from_bits(val)
    }
}
impl From<Cmpstcha24> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha24) -> u8 {
        Cmpstcha24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstcha25 {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstcha25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstcha25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstcha25 {
    #[inline(always)]
    fn from(val: u8) -> Cmpstcha25 {
        Cmpstcha25::from_bits(val)
    }
}
impl From<Cmpstcha25> for u8 {
    #[inline(always)]
    fn from(val: Cmpstcha25) -> u8 {
        Cmpstcha25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpstoca {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpstoca {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpstoca {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpstoca {
    #[inline(always)]
    fn from(val: u8) -> Cmpstoca {
        Cmpstoca::from_bits(val)
    }
}
impl From<Cmpstoca> for u8 {
    #[inline(always)]
    fn from(val: Cmpstoca) -> u8 {
        Cmpstoca::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpsttsa {
    #[doc = "Comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Comparison conditions are met."]
    _1 = 0x01,
}
impl Cmpsttsa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpsttsa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpsttsa {
    #[inline(always)]
    fn from(val: u8) -> Cmpsttsa {
        Cmpsttsa::from_bits(val)
    }
}
impl From<Cmpsttsa> for u8 {
    #[inline(always)]
    fn from(val: Cmpsttsa) -> u8 {
        Cmpsttsa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmptsa {
    #[doc = "Excludes the temperature sensor output from the compare window A target range."]
    _0 = 0x0,
    #[doc = "Includes the temperature sensor output in the compare window A target range."]
    _1 = 0x01,
}
impl Cmptsa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmptsa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmptsa {
    #[inline(always)]
    fn from(val: u8) -> Cmptsa {
        Cmptsa::from_bits(val)
    }
}
impl From<Cmptsa> for u8 {
    #[inline(always)]
    fn from(val: Cmptsa) -> u8 {
        Cmptsa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dble {
    #[doc = "Double trigger mode non-selection"]
    _0 = 0x0,
    #[doc = "Double trigger mode selection"]
    _1 = 0x01,
}
impl Dble {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dble {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dble {
    #[inline(always)]
    fn from(val: u8) -> Dble {
        Dble::from_bits(val)
    }
}
impl From<Dble> for u8 {
    #[inline(always)]
    fn from(val: Dble) -> u8 {
        Dble::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diagld {
    #[doc = "Rotation mode for self-diagnosis voltage"]
    _0 = 0x0,
    #[doc = "Fixed mode for self-diagnosis voltage"]
    _1 = 0x01,
}
impl Diagld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diagld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diagld {
    #[inline(always)]
    fn from(val: u8) -> Diagld {
        Diagld::from_bits(val)
    }
}
impl From<Diagld> for u8 {
    #[inline(always)]
    fn from(val: Diagld) -> u8 {
        Diagld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diagm {
    #[doc = "Disables self-diagnosis of A/D converter."]
    _0 = 0x0,
    #[doc = "Enables self-diagnosis of A/D converter."]
    _1 = 0x01,
}
impl Diagm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diagm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diagm {
    #[inline(always)]
    fn from(val: u8) -> Diagm {
        Diagm::from_bits(val)
    }
}
impl From<Diagm> for u8 {
    #[inline(always)]
    fn from(val: Diagm) -> u8 {
        Diagm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diagst {
    #[doc = "Self-diagnosis has never been executed since power-on."]
    _00 = 0x0,
    #[doc = "Self-diagnosis using the voltage of 0 V has been executed."]
    _01 = 0x01,
    #[doc = "Self-diagnosis using the voltage of reference power supply(VREFH) x 1/2 has been executed."]
    _10 = 0x02,
    #[doc = "Self-diagnosis using the voltage of reference power supply(VREFH) has been executed."]
    _11 = 0x03,
}
impl Diagst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diagst {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diagst {
    #[inline(always)]
    fn from(val: u8) -> Diagst {
        Diagst::from_bits(val)
    }
}
impl From<Diagst> for u8 {
    #[inline(always)]
    fn from(val: Diagst) -> u8 {
        Diagst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diagval {
    #[doc = "When the self-diagnosis fixation mode is selected, it set prohibits it."]
    _00 = 0x0,
    #[doc = "The self-diagnosis by using the voltage of 0V."]
    _01 = 0x01,
    #[doc = "The self-diagnosis by using the voltage of reference supply x 1/2."]
    _10 = 0x02,
    #[doc = "The self-diagnosis by using the voltage of the reference supply."]
    _11 = 0x03,
}
impl Diagval {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diagval {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diagval {
    #[inline(always)]
    fn from(val: u8) -> Diagval {
        Diagval::from_bits(val)
    }
}
impl From<Diagval> for u8 {
    #[inline(always)]
    fn from(val: Diagval) -> u8 {
        Diagval::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extrg {
    #[doc = "A/D conversion is started by the synchronous trigger (ELC)."]
    _0 = 0x0,
    #[doc = "A/D conversion is started by the asynchronous trigger (ADTRG0#)."]
    _1 = 0x01,
}
impl Extrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extrg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extrg {
    #[inline(always)]
    fn from(val: u8) -> Extrg {
        Extrg::from_bits(val)
    }
}
impl From<Extrg> for u8 {
    #[inline(always)]
    fn from(val: Extrg) -> u8 {
        Extrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gbadie {
    #[doc = "Disables S12GBADI0 interrupt generation upon group B scan completion."]
    _0 = 0x0,
    #[doc = "Enables S12GBADI0 interrupt generation upon group B scan completion."]
    _1 = 0x01,
}
impl Gbadie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gbadie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gbadie {
    #[inline(always)]
    fn from(val: u8) -> Gbadie {
        Gbadie::from_bits(val)
    }
}
impl From<Gbadie> for u8 {
    #[inline(always)]
    fn from(val: Gbadie) -> u8 {
        Gbadie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gbrp {
    #[doc = "Single scan for group B is not continuously activated."]
    _0 = 0x0,
    #[doc = "Single scan for group B is continuously activated."]
    _1 = 0x01,
}
impl Gbrp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gbrp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gbrp {
    #[inline(always)]
    fn from(val: u8) -> Gbrp {
        Gbrp::from_bits(val)
    }
}
impl From<Gbrp> for u8 {
    #[inline(always)]
    fn from(val: Gbrp) -> u8 {
        Gbrp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gbrscn {
    #[doc = "Scanning for group B is not restarted after having been discontinued due to group A priority control."]
    _0 = 0x0,
    #[doc = "Scanning for group B is restarted after having been discontinued due to group A priority control."]
    _1 = 0x01,
}
impl Gbrscn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gbrscn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gbrscn {
    #[inline(always)]
    fn from(val: u8) -> Gbrscn {
        Gbrscn::from_bits(val)
    }
}
impl From<Gbrscn> for u8 {
    #[inline(always)]
    fn from(val: Gbrscn) -> u8 {
        Gbrscn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hvsel {
    #[doc = "AVCC0 is selected as the high-potential reference voltage"]
    _00 = 0x0,
    #[doc = "VREFH0 is selected as the high-potential reference voltage"]
    _01 = 0x01,
    #[doc = "Internal reference voltage is selected as the high-potential reference voltage"]
    _10 = 0x02,
    #[doc = "Internal node discharge. No reference voltage pin is selected."]
    _11 = 0x03,
}
impl Hvsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hvsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hvsel {
    #[inline(always)]
    fn from(val: u8) -> Hvsel {
        Hvsel::from_bits(val)
    }
}
impl From<Hvsel> for u8 {
    #[inline(always)]
    fn from(val: Hvsel) -> u8 {
        Hvsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvsel {
    #[doc = "AVSS0 is selected as the low-potential reference voltage"]
    _0 = 0x0,
    #[doc = "VREFL0 is selected as the low-potential reference voltage."]
    _1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lvsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvsel {
    #[inline(always)]
    fn from(val: u8) -> Lvsel {
        Lvsel::from_bits(val)
    }
}
impl From<Lvsel> for u8 {
    #[inline(always)]
    fn from(val: Lvsel) -> u8 {
        Lvsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Moncmpa {
    #[doc = "Window A comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Window A comparison conditions are met."]
    _1 = 0x01,
}
impl Moncmpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Moncmpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Moncmpa {
    #[inline(always)]
    fn from(val: u8) -> Moncmpa {
        Moncmpa::from_bits(val)
    }
}
impl From<Moncmpa> for u8 {
    #[inline(always)]
    fn from(val: Moncmpa) -> u8 {
        Moncmpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Moncmpb {
    #[doc = "Window B comparison conditions are not met."]
    _0 = 0x0,
    #[doc = "Window B comparison conditions are met."]
    _1 = 0x01,
}
impl Moncmpb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Moncmpb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Moncmpb {
    #[inline(always)]
    fn from(val: u8) -> Moncmpb {
        Moncmpb::from_bits(val)
    }
}
impl From<Moncmpb> for u8 {
    #[inline(always)]
    fn from(val: Moncmpb) -> u8 {
        Moncmpb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Moncomb {
    #[doc = "Window A / window B composite conditions are not met."]
    _0 = 0x0,
    #[doc = "Window A / window B composite conditions are met."]
    _1 = 0x01,
}
impl Moncomb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Moncomb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Moncomb {
    #[inline(always)]
    fn from(val: u8) -> Moncomb {
        Moncomb::from_bits(val)
    }
}
impl From<Moncomb> for u8 {
    #[inline(always)]
    fn from(val: Moncomb) -> u8 {
        Moncomb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ocsa {
    #[doc = "The internal reference voltage is not selected."]
    _0 = 0x0,
    #[doc = "The internal reference voltage is selected for group A in single scan mode, continuous scan mode, or group scan mode."]
    _1 = 0x01,
}
impl Ocsa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ocsa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ocsa {
    #[inline(always)]
    fn from(val: u8) -> Ocsa {
        Ocsa::from_bits(val)
    }
}
impl From<Ocsa> for u8 {
    #[inline(always)]
    fn from(val: Ocsa) -> u8 {
        Ocsa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ocsad {
    #[doc = "Internal reference voltage A/D-converted value addition/average mode is not selected."]
    _0 = 0x0,
    #[doc = "Internal reference voltage A/D-converted value addition/average mode is selected."]
    _1 = 0x01,
}
impl Ocsad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ocsad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ocsad {
    #[inline(always)]
    fn from(val: u8) -> Ocsad {
        Ocsad::from_bits(val)
    }
}
impl From<Ocsad> for u8 {
    #[inline(always)]
    fn from(val: Ocsad) -> u8 {
        Ocsad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pchg {
    #[doc = "Discharge"]
    _0 = 0x0,
    #[doc = "Precharge"]
    _1 = 0x01,
}
impl Pchg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pchg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pchg {
    #[inline(always)]
    fn from(val: u8) -> Pchg {
        Pchg::from_bits(val)
    }
}
impl From<Pchg> for u8 {
    #[inline(always)]
    fn from(val: Pchg) -> u8 {
        Pchg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pgs {
    #[doc = "Operation is without group A priority control"]
    _0 = 0x0,
    #[doc = "Operation is with group A priority control"]
    _1 = 0x01,
}
impl Pgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pgs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pgs {
    #[inline(always)]
    fn from(val: u8) -> Pgs {
        Pgs::from_bits(val)
    }
}
impl From<Pgs> for u8 {
    #[inline(always)]
    fn from(val: Pgs) -> u8 {
        Pgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trge {
    #[doc = "Disables A/D conversion to be started by the synchronous or asynchronous trigger."]
    _0 = 0x0,
    #[doc = "Enables A/D conversion to be started by the synchronous or asynchronous trigger."]
    _1 = 0x01,
}
impl Trge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trge {
    #[inline(always)]
    fn from(val: u8) -> Trge {
        Trge::from_bits(val)
    }
}
impl From<Trge> for u8 {
    #[inline(always)]
    fn from(val: Trge) -> u8 {
        Trge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tssa {
    #[doc = "The temperature sensor output is not selected."]
    _0 = 0x0,
    #[doc = "The temperature sensor output is selected."]
    _1 = 0x01,
}
impl Tssa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tssa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tssa {
    #[inline(always)]
    fn from(val: u8) -> Tssa {
        Tssa::from_bits(val)
    }
}
impl From<Tssa> for u8 {
    #[inline(always)]
    fn from(val: Tssa) -> u8 {
        Tssa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tssad {
    #[doc = "Temperature sensor output A/D-converted value addition/average mode is not selected."]
    _0 = 0x0,
    #[doc = "Temperature sensor output A/D-converted value addition/average mode is selected."]
    _1 = 0x01,
}
impl Tssad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tssad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tssad {
    #[inline(always)]
    fn from(val: u8) -> Tssad {
        Tssad::from_bits(val)
    }
}
impl From<Tssad> for u8 {
    #[inline(always)]
    fn from(val: Tssad) -> u8 {
        Tssad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wcmpe {
    #[doc = "Window function is disabled. Window A and window B operate as a comparator to comparator the single value on the lower side with the A/D conversion result."]
    _0 = 0x0,
    #[doc = "Window function is enabled. Window A and window B operate as a comparator to comparator the two values on the upper and lower sides with the A/D conversion result."]
    _1 = 0x01,
}
impl Wcmpe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wcmpe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wcmpe {
    #[inline(always)]
    fn from(val: u8) -> Wcmpe {
        Wcmpe::from_bits(val)
    }
}
impl From<Wcmpe> for u8 {
    #[inline(always)]
    fn from(val: Wcmpe) -> u8 {
        Wcmpe::to_bits(val)
    }
}
