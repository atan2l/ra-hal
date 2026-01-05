#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inv {
    #[doc = "GTETRG Input"]
    _0 = 0x0,
    #[doc = "GTETRG Input Reversed."]
    _1 = 0x01,
}
impl Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inv {
    #[inline(always)]
    fn from(val: u8) -> Inv {
        Inv::from_bits(val)
    }
}
impl From<Inv> for u8 {
    #[inline(always)]
    fn from(val: Inv) -> u8 {
        Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ioce {
    #[doc = "Output-disable request from the GPT disable request disabled"]
    _0 = 0x0,
    #[doc = "Output-disable request from the GPT disable request enabled."]
    _1 = 0x01,
}
impl Ioce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ioce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ioce {
    #[inline(always)]
    fn from(val: u8) -> Ioce {
        Ioce::from_bits(val)
    }
}
impl From<Ioce> for u8 {
    #[inline(always)]
    fn from(val: Ioce) -> u8 {
        Ioce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iocf {
    #[doc = "No output-disable request from the GPT disable request has occurred"]
    _0 = 0x0,
    #[doc = "Output-disable request from the GPT disable request occurred."]
    _1 = 0x01,
}
impl Iocf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iocf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iocf {
    #[inline(always)]
    fn from(val: u8) -> Iocf {
        Iocf::from_bits(val)
    }
}
impl From<Iocf> for u8 {
    #[inline(always)]
    fn from(val: Iocf) -> u8 {
        Iocf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nfcs {
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB."]
    _00 = 0x0,
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /8."]
    _01 = 0x01,
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /32."]
    _10 = 0x02,
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /128."]
    _11 = 0x03,
}
impl Nfcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfcs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfcs {
    #[inline(always)]
    fn from(val: u8) -> Nfcs {
        Nfcs::from_bits(val)
    }
}
impl From<Nfcs> for u8 {
    #[inline(always)]
    fn from(val: Nfcs) -> u8 {
        Nfcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nfen {
    #[doc = "Filtering noise disabled"]
    _0 = 0x0,
    #[doc = "Filtering noise enabled"]
    _1 = 0x01,
}
impl Nfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfen {
    #[inline(always)]
    fn from(val: u8) -> Nfen {
        Nfen::from_bits(val)
    }
}
impl From<Nfen> for u8 {
    #[inline(always)]
    fn from(val: Nfen) -> u8 {
        Nfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ostpe {
    #[doc = "A output-disable request from the oscillation stop detection disabled."]
    _0 = 0x0,
    #[doc = "A output-disable request from the oscillation stop detection enabled."]
    _1 = 0x01,
}
impl Ostpe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ostpe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ostpe {
    #[inline(always)]
    fn from(val: u8) -> Ostpe {
        Ostpe::from_bits(val)
    }
}
impl From<Ostpe> for u8 {
    #[inline(always)]
    fn from(val: Ostpe) -> u8 {
        Ostpe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ostpf {
    #[doc = "No output-disable request from oscillation stop detection has occurred"]
    _0 = 0x0,
    #[doc = "Output-disable request from oscillation stop detection occurred."]
    _1 = 0x01,
}
impl Ostpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ostpf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ostpf {
    #[inline(always)]
    fn from(val: u8) -> Ostpf {
        Ostpf::from_bits(val)
    }
}
impl From<Ostpf> for u8 {
    #[inline(always)]
    fn from(val: Ostpf) -> u8 {
        Ostpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pide {
    #[doc = "Output-disable request from the GTETRG pins disabled"]
    _0 = 0x0,
    #[doc = "Output-disable request from the GTETRG pins enabled."]
    _1 = 0x01,
}
impl Pide {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pide {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pide {
    #[inline(always)]
    fn from(val: u8) -> Pide {
        Pide::from_bits(val)
    }
}
impl From<Pide> for u8 {
    #[inline(always)]
    fn from(val: Pide) -> u8 {
        Pide::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pidf {
    #[doc = "No output-disable request from the GTETRGn pin has occurred"]
    _0 = 0x0,
    #[doc = "Output-disable request from the GTETRGn pin occurred."]
    _1 = 0x01,
}
impl Pidf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pidf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pidf {
    #[inline(always)]
    fn from(val: u8) -> Pidf {
        Pidf::from_bits(val)
    }
}
impl From<Pidf> for u8 {
    #[inline(always)]
    fn from(val: Pidf) -> u8 {
        Pidf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssf {
    #[doc = "No output-disable request from software has occurred"]
    _0 = 0x0,
    #[doc = "Output-disable request from software occurred."]
    _1 = 0x01,
}
impl Ssf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssf {
    #[inline(always)]
    fn from(val: u8) -> Ssf {
        Ssf::from_bits(val)
    }
}
impl From<Ssf> for u8 {
    #[inline(always)]
    fn from(val: Ssf) -> u8 {
        Ssf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum St {
    #[doc = "GTETRG input after filtering is 0."]
    _0 = 0x0,
    #[doc = "GTETRG input after filtering is 1."]
    _1 = 0x01,
}
impl St {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> St {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for St {
    #[inline(always)]
    fn from(val: u8) -> St {
        St::from_bits(val)
    }
}
impl From<St> for u8 {
    #[inline(always)]
    fn from(val: St) -> u8 {
        St::to_bits(val)
    }
}
