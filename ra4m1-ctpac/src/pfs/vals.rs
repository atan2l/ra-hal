#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsAsel {
    #[doc = "Used other than as analog pin"]
    _0 = 0x0,
    #[doc = "Used as analog pin"]
    _1 = 0x01,
}
impl P000pfsAsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsAsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsAsel {
    #[inline(always)]
    fn from(val: u8) -> P000pfsAsel {
        P000pfsAsel::from_bits(val)
    }
}
impl From<P000pfsAsel> for u8 {
    #[inline(always)]
    fn from(val: P000pfsAsel) -> u8 {
        P000pfsAsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsByNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P000pfsByNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsByNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsByNcodr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsByNcodr {
        P000pfsByNcodr::from_bits(val)
    }
}
impl From<P000pfsByNcodr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsByNcodr) -> u8 {
        P000pfsByNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsByPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P000pfsByPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsByPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsByPcr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsByPcr {
        P000pfsByPcr::from_bits(val)
    }
}
impl From<P000pfsByPcr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsByPcr) -> u8 {
        P000pfsByPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsByPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P000pfsByPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsByPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsByPdr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsByPdr {
        P000pfsByPdr::from_bits(val)
    }
}
impl From<P000pfsByPdr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsByPdr) -> u8 {
        P000pfsByPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsByPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P000pfsByPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsByPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsByPidr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsByPidr {
        P000pfsByPidr::from_bits(val)
    }
}
impl From<P000pfsByPidr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsByPidr) -> u8 {
        P000pfsByPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsByPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P000pfsByPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsByPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsByPodr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsByPodr {
        P000pfsByPodr::from_bits(val)
    }
}
impl From<P000pfsByPodr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsByPodr) -> u8 {
        P000pfsByPodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsDscr {
    #[doc = "Low drive"]
    _0 = 0x0,
    #[doc = "Middle drive."]
    _1 = 0x01,
}
impl P000pfsDscr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsDscr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsDscr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsDscr {
        P000pfsDscr::from_bits(val)
    }
}
impl From<P000pfsDscr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsDscr) -> u8 {
        P000pfsDscr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsHaAsel {
    #[doc = "Used other than as analog pin"]
    _0 = 0x0,
    #[doc = "Used as analog pin"]
    _1 = 0x01,
}
impl P000pfsHaAsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsHaAsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsHaAsel {
    #[inline(always)]
    fn from(val: u8) -> P000pfsHaAsel {
        P000pfsHaAsel::from_bits(val)
    }
}
impl From<P000pfsHaAsel> for u8 {
    #[inline(always)]
    fn from(val: P000pfsHaAsel) -> u8 {
        P000pfsHaAsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsHaDscr {
    #[doc = "Low drive"]
    _0 = 0x0,
    #[doc = "Middle drive."]
    _1 = 0x01,
}
impl P000pfsHaDscr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsHaDscr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsHaDscr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsHaDscr {
        P000pfsHaDscr::from_bits(val)
    }
}
impl From<P000pfsHaDscr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsHaDscr) -> u8 {
        P000pfsHaDscr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsHaIsel {
    #[doc = "Not used as IRQn input pin"]
    _0 = 0x0,
    #[doc = "Used as IRQn input pin"]
    _1 = 0x01,
}
impl P000pfsHaIsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsHaIsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsHaIsel {
    #[inline(always)]
    fn from(val: u8) -> P000pfsHaIsel {
        P000pfsHaIsel::from_bits(val)
    }
}
impl From<P000pfsHaIsel> for u8 {
    #[inline(always)]
    fn from(val: P000pfsHaIsel) -> u8 {
        P000pfsHaIsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsHaNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P000pfsHaNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsHaNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsHaNcodr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsHaNcodr {
        P000pfsHaNcodr::from_bits(val)
    }
}
impl From<P000pfsHaNcodr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsHaNcodr) -> u8 {
        P000pfsHaNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsHaPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P000pfsHaPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsHaPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsHaPcr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsHaPcr {
        P000pfsHaPcr::from_bits(val)
    }
}
impl From<P000pfsHaPcr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsHaPcr) -> u8 {
        P000pfsHaPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsHaPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P000pfsHaPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsHaPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsHaPdr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsHaPdr {
        P000pfsHaPdr::from_bits(val)
    }
}
impl From<P000pfsHaPdr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsHaPdr) -> u8 {
        P000pfsHaPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsHaPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P000pfsHaPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsHaPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsHaPidr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsHaPidr {
        P000pfsHaPidr::from_bits(val)
    }
}
impl From<P000pfsHaPidr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsHaPidr) -> u8 {
        P000pfsHaPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsHaPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P000pfsHaPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsHaPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsHaPodr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsHaPodr {
        P000pfsHaPodr::from_bits(val)
    }
}
impl From<P000pfsHaPodr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsHaPodr) -> u8 {
        P000pfsHaPodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsIsel {
    #[doc = "Not used as IRQn input pin"]
    _0 = 0x0,
    #[doc = "Used as IRQn input pin"]
    _1 = 0x01,
}
impl P000pfsIsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsIsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsIsel {
    #[inline(always)]
    fn from(val: u8) -> P000pfsIsel {
        P000pfsIsel::from_bits(val)
    }
}
impl From<P000pfsIsel> for u8 {
    #[inline(always)]
    fn from(val: P000pfsIsel) -> u8 {
        P000pfsIsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P000pfsNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsNcodr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsNcodr {
        P000pfsNcodr::from_bits(val)
    }
}
impl From<P000pfsNcodr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsNcodr) -> u8 {
        P000pfsNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P000pfsPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsPcr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsPcr {
        P000pfsPcr::from_bits(val)
    }
}
impl From<P000pfsPcr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsPcr) -> u8 {
        P000pfsPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P000pfsPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsPdr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsPdr {
        P000pfsPdr::from_bits(val)
    }
}
impl From<P000pfsPdr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsPdr) -> u8 {
        P000pfsPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P000pfsPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsPidr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsPidr {
        P000pfsPidr::from_bits(val)
    }
}
impl From<P000pfsPidr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsPidr) -> u8 {
        P000pfsPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsPmr {
    #[doc = "Uses the pin as a general I/O pin."]
    _0 = 0x0,
    #[doc = "Uses the pin as an I/O port for peripheral functions."]
    _1 = 0x01,
}
impl P000pfsPmr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsPmr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsPmr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsPmr {
        P000pfsPmr::from_bits(val)
    }
}
impl From<P000pfsPmr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsPmr) -> u8 {
        P000pfsPmr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P000pfsPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P000pfsPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P000pfsPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P000pfsPodr {
    #[inline(always)]
    fn from(val: u8) -> P000pfsPodr {
        P000pfsPodr::from_bits(val)
    }
}
impl From<P000pfsPodr> for u8 {
    #[inline(always)]
    fn from(val: P000pfsPodr) -> u8 {
        P000pfsPodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsAsel {
    #[doc = "Used other than as analog pin"]
    _0 = 0x0,
    #[doc = "Used as analog pin"]
    _1 = 0x01,
}
impl P108pfsAsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsAsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsAsel {
    #[inline(always)]
    fn from(val: u8) -> P108pfsAsel {
        P108pfsAsel::from_bits(val)
    }
}
impl From<P108pfsAsel> for u8 {
    #[inline(always)]
    fn from(val: P108pfsAsel) -> u8 {
        P108pfsAsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsByNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P108pfsByNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsByNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsByNcodr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsByNcodr {
        P108pfsByNcodr::from_bits(val)
    }
}
impl From<P108pfsByNcodr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsByNcodr) -> u8 {
        P108pfsByNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsByPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P108pfsByPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsByPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsByPcr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsByPcr {
        P108pfsByPcr::from_bits(val)
    }
}
impl From<P108pfsByPcr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsByPcr) -> u8 {
        P108pfsByPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsByPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P108pfsByPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsByPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsByPdr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsByPdr {
        P108pfsByPdr::from_bits(val)
    }
}
impl From<P108pfsByPdr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsByPdr) -> u8 {
        P108pfsByPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsByPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P108pfsByPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsByPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsByPidr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsByPidr {
        P108pfsByPidr::from_bits(val)
    }
}
impl From<P108pfsByPidr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsByPidr) -> u8 {
        P108pfsByPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsByPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P108pfsByPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsByPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsByPodr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsByPodr {
        P108pfsByPodr::from_bits(val)
    }
}
impl From<P108pfsByPodr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsByPodr) -> u8 {
        P108pfsByPodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsDscr {
    #[doc = "Low drive"]
    _0 = 0x0,
    #[doc = "Middle drive."]
    _1 = 0x01,
}
impl P108pfsDscr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsDscr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsDscr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsDscr {
        P108pfsDscr::from_bits(val)
    }
}
impl From<P108pfsDscr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsDscr) -> u8 {
        P108pfsDscr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsEof {
    #[doc = "No effected"]
    _0 = 0x0,
    #[doc = "Detect failing edge"]
    _1 = 0x01,
}
impl P108pfsEof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsEof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsEof {
    #[inline(always)]
    fn from(val: u8) -> P108pfsEof {
        P108pfsEof::from_bits(val)
    }
}
impl From<P108pfsEof> for u8 {
    #[inline(always)]
    fn from(val: P108pfsEof) -> u8 {
        P108pfsEof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsEor {
    #[doc = "No effected"]
    _0 = 0x0,
    #[doc = "Detect rising edge"]
    _1 = 0x01,
}
impl P108pfsEor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsEor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsEor {
    #[inline(always)]
    fn from(val: u8) -> P108pfsEor {
        P108pfsEor::from_bits(val)
    }
}
impl From<P108pfsEor> for u8 {
    #[inline(always)]
    fn from(val: P108pfsEor) -> u8 {
        P108pfsEor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsHaAsel {
    #[doc = "Used other than as analog pin"]
    _0 = 0x0,
    #[doc = "Used as analog pin"]
    _1 = 0x01,
}
impl P108pfsHaAsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsHaAsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsHaAsel {
    #[inline(always)]
    fn from(val: u8) -> P108pfsHaAsel {
        P108pfsHaAsel::from_bits(val)
    }
}
impl From<P108pfsHaAsel> for u8 {
    #[inline(always)]
    fn from(val: P108pfsHaAsel) -> u8 {
        P108pfsHaAsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsHaDscr {
    #[doc = "Low drive"]
    _0 = 0x0,
    #[doc = "Middle drive."]
    _1 = 0x01,
}
impl P108pfsHaDscr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsHaDscr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsHaDscr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsHaDscr {
        P108pfsHaDscr::from_bits(val)
    }
}
impl From<P108pfsHaDscr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsHaDscr) -> u8 {
        P108pfsHaDscr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsHaEof {
    #[doc = "No effected"]
    _0 = 0x0,
    #[doc = "Detect failing edge"]
    _1 = 0x01,
}
impl P108pfsHaEof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsHaEof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsHaEof {
    #[inline(always)]
    fn from(val: u8) -> P108pfsHaEof {
        P108pfsHaEof::from_bits(val)
    }
}
impl From<P108pfsHaEof> for u8 {
    #[inline(always)]
    fn from(val: P108pfsHaEof) -> u8 {
        P108pfsHaEof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsHaEor {
    #[doc = "No effected"]
    _0 = 0x0,
    #[doc = "Detect rising edge"]
    _1 = 0x01,
}
impl P108pfsHaEor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsHaEor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsHaEor {
    #[inline(always)]
    fn from(val: u8) -> P108pfsHaEor {
        P108pfsHaEor::from_bits(val)
    }
}
impl From<P108pfsHaEor> for u8 {
    #[inline(always)]
    fn from(val: P108pfsHaEor) -> u8 {
        P108pfsHaEor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsHaIsel {
    #[doc = "Not used as IRQn input pin"]
    _0 = 0x0,
    #[doc = "Used as IRQn input pin"]
    _1 = 0x01,
}
impl P108pfsHaIsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsHaIsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsHaIsel {
    #[inline(always)]
    fn from(val: u8) -> P108pfsHaIsel {
        P108pfsHaIsel::from_bits(val)
    }
}
impl From<P108pfsHaIsel> for u8 {
    #[inline(always)]
    fn from(val: P108pfsHaIsel) -> u8 {
        P108pfsHaIsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsHaNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P108pfsHaNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsHaNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsHaNcodr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsHaNcodr {
        P108pfsHaNcodr::from_bits(val)
    }
}
impl From<P108pfsHaNcodr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsHaNcodr) -> u8 {
        P108pfsHaNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsHaPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P108pfsHaPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsHaPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsHaPcr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsHaPcr {
        P108pfsHaPcr::from_bits(val)
    }
}
impl From<P108pfsHaPcr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsHaPcr) -> u8 {
        P108pfsHaPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsHaPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P108pfsHaPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsHaPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsHaPdr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsHaPdr {
        P108pfsHaPdr::from_bits(val)
    }
}
impl From<P108pfsHaPdr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsHaPdr) -> u8 {
        P108pfsHaPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsHaPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P108pfsHaPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsHaPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsHaPidr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsHaPidr {
        P108pfsHaPidr::from_bits(val)
    }
}
impl From<P108pfsHaPidr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsHaPidr) -> u8 {
        P108pfsHaPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsHaPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P108pfsHaPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsHaPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsHaPodr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsHaPodr {
        P108pfsHaPodr::from_bits(val)
    }
}
impl From<P108pfsHaPodr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsHaPodr) -> u8 {
        P108pfsHaPodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsIsel {
    #[doc = "Not used as IRQn input pin"]
    _0 = 0x0,
    #[doc = "Used as IRQn input pin"]
    _1 = 0x01,
}
impl P108pfsIsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsIsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsIsel {
    #[inline(always)]
    fn from(val: u8) -> P108pfsIsel {
        P108pfsIsel::from_bits(val)
    }
}
impl From<P108pfsIsel> for u8 {
    #[inline(always)]
    fn from(val: P108pfsIsel) -> u8 {
        P108pfsIsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P108pfsNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsNcodr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsNcodr {
        P108pfsNcodr::from_bits(val)
    }
}
impl From<P108pfsNcodr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsNcodr) -> u8 {
        P108pfsNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P108pfsPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsPcr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsPcr {
        P108pfsPcr::from_bits(val)
    }
}
impl From<P108pfsPcr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsPcr) -> u8 {
        P108pfsPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P108pfsPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsPdr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsPdr {
        P108pfsPdr::from_bits(val)
    }
}
impl From<P108pfsPdr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsPdr) -> u8 {
        P108pfsPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P108pfsPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsPidr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsPidr {
        P108pfsPidr::from_bits(val)
    }
}
impl From<P108pfsPidr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsPidr) -> u8 {
        P108pfsPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsPmr {
    #[doc = "Uses the pin as a general I/O pin."]
    _0 = 0x0,
    #[doc = "Uses the pin as an I/O port for peripheral functions."]
    _1 = 0x01,
}
impl P108pfsPmr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsPmr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsPmr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsPmr {
        P108pfsPmr::from_bits(val)
    }
}
impl From<P108pfsPmr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsPmr) -> u8 {
        P108pfsPmr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P108pfsPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P108pfsPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P108pfsPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P108pfsPodr {
    #[inline(always)]
    fn from(val: u8) -> P108pfsPodr {
        P108pfsPodr::from_bits(val)
    }
}
impl From<P108pfsPodr> for u8 {
    #[inline(always)]
    fn from(val: P108pfsPodr) -> u8 {
        P108pfsPodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsAsel {
    #[doc = "Used other than as analog pin"]
    _0 = 0x0,
    #[doc = "Used as analog pin"]
    _1 = 0x01,
}
impl P109pfsAsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsAsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsAsel {
    #[inline(always)]
    fn from(val: u8) -> P109pfsAsel {
        P109pfsAsel::from_bits(val)
    }
}
impl From<P109pfsAsel> for u8 {
    #[inline(always)]
    fn from(val: P109pfsAsel) -> u8 {
        P109pfsAsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsByNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P109pfsByNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsByNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsByNcodr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsByNcodr {
        P109pfsByNcodr::from_bits(val)
    }
}
impl From<P109pfsByNcodr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsByNcodr) -> u8 {
        P109pfsByNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsByPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P109pfsByPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsByPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsByPcr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsByPcr {
        P109pfsByPcr::from_bits(val)
    }
}
impl From<P109pfsByPcr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsByPcr) -> u8 {
        P109pfsByPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsByPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P109pfsByPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsByPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsByPdr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsByPdr {
        P109pfsByPdr::from_bits(val)
    }
}
impl From<P109pfsByPdr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsByPdr) -> u8 {
        P109pfsByPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsByPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P109pfsByPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsByPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsByPidr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsByPidr {
        P109pfsByPidr::from_bits(val)
    }
}
impl From<P109pfsByPidr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsByPidr) -> u8 {
        P109pfsByPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsByPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P109pfsByPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsByPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsByPodr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsByPodr {
        P109pfsByPodr::from_bits(val)
    }
}
impl From<P109pfsByPodr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsByPodr) -> u8 {
        P109pfsByPodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsDscr {
    #[doc = "Low drive"]
    _0 = 0x0,
    #[doc = "Middle drive."]
    _1 = 0x01,
}
impl P109pfsDscr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsDscr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsDscr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsDscr {
        P109pfsDscr::from_bits(val)
    }
}
impl From<P109pfsDscr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsDscr) -> u8 {
        P109pfsDscr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsEof {
    #[doc = "No effected"]
    _0 = 0x0,
    #[doc = "Detect failing edge"]
    _1 = 0x01,
}
impl P109pfsEof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsEof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsEof {
    #[inline(always)]
    fn from(val: u8) -> P109pfsEof {
        P109pfsEof::from_bits(val)
    }
}
impl From<P109pfsEof> for u8 {
    #[inline(always)]
    fn from(val: P109pfsEof) -> u8 {
        P109pfsEof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsEor {
    #[doc = "No effected"]
    _0 = 0x0,
    #[doc = "Detect rising edge"]
    _1 = 0x01,
}
impl P109pfsEor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsEor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsEor {
    #[inline(always)]
    fn from(val: u8) -> P109pfsEor {
        P109pfsEor::from_bits(val)
    }
}
impl From<P109pfsEor> for u8 {
    #[inline(always)]
    fn from(val: P109pfsEor) -> u8 {
        P109pfsEor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsHaAsel {
    #[doc = "Used other than as analog pin"]
    _0 = 0x0,
    #[doc = "Used as analog pin"]
    _1 = 0x01,
}
impl P109pfsHaAsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsHaAsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsHaAsel {
    #[inline(always)]
    fn from(val: u8) -> P109pfsHaAsel {
        P109pfsHaAsel::from_bits(val)
    }
}
impl From<P109pfsHaAsel> for u8 {
    #[inline(always)]
    fn from(val: P109pfsHaAsel) -> u8 {
        P109pfsHaAsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsHaDscr {
    #[doc = "Low drive"]
    _0 = 0x0,
    #[doc = "Middle drive."]
    _1 = 0x01,
}
impl P109pfsHaDscr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsHaDscr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsHaDscr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsHaDscr {
        P109pfsHaDscr::from_bits(val)
    }
}
impl From<P109pfsHaDscr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsHaDscr) -> u8 {
        P109pfsHaDscr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsHaEof {
    #[doc = "No effected"]
    _0 = 0x0,
    #[doc = "Detect failing edge"]
    _1 = 0x01,
}
impl P109pfsHaEof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsHaEof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsHaEof {
    #[inline(always)]
    fn from(val: u8) -> P109pfsHaEof {
        P109pfsHaEof::from_bits(val)
    }
}
impl From<P109pfsHaEof> for u8 {
    #[inline(always)]
    fn from(val: P109pfsHaEof) -> u8 {
        P109pfsHaEof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsHaEor {
    #[doc = "No effected"]
    _0 = 0x0,
    #[doc = "Detect rising edge"]
    _1 = 0x01,
}
impl P109pfsHaEor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsHaEor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsHaEor {
    #[inline(always)]
    fn from(val: u8) -> P109pfsHaEor {
        P109pfsHaEor::from_bits(val)
    }
}
impl From<P109pfsHaEor> for u8 {
    #[inline(always)]
    fn from(val: P109pfsHaEor) -> u8 {
        P109pfsHaEor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsHaIsel {
    #[doc = "Not used as IRQn input pin"]
    _0 = 0x0,
    #[doc = "Used as IRQn input pin"]
    _1 = 0x01,
}
impl P109pfsHaIsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsHaIsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsHaIsel {
    #[inline(always)]
    fn from(val: u8) -> P109pfsHaIsel {
        P109pfsHaIsel::from_bits(val)
    }
}
impl From<P109pfsHaIsel> for u8 {
    #[inline(always)]
    fn from(val: P109pfsHaIsel) -> u8 {
        P109pfsHaIsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsHaNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P109pfsHaNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsHaNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsHaNcodr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsHaNcodr {
        P109pfsHaNcodr::from_bits(val)
    }
}
impl From<P109pfsHaNcodr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsHaNcodr) -> u8 {
        P109pfsHaNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsHaPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P109pfsHaPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsHaPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsHaPcr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsHaPcr {
        P109pfsHaPcr::from_bits(val)
    }
}
impl From<P109pfsHaPcr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsHaPcr) -> u8 {
        P109pfsHaPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsHaPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P109pfsHaPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsHaPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsHaPdr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsHaPdr {
        P109pfsHaPdr::from_bits(val)
    }
}
impl From<P109pfsHaPdr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsHaPdr) -> u8 {
        P109pfsHaPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsHaPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P109pfsHaPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsHaPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsHaPidr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsHaPidr {
        P109pfsHaPidr::from_bits(val)
    }
}
impl From<P109pfsHaPidr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsHaPidr) -> u8 {
        P109pfsHaPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsHaPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P109pfsHaPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsHaPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsHaPodr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsHaPodr {
        P109pfsHaPodr::from_bits(val)
    }
}
impl From<P109pfsHaPodr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsHaPodr) -> u8 {
        P109pfsHaPodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsIsel {
    #[doc = "Not used as IRQn input pin"]
    _0 = 0x0,
    #[doc = "Used as IRQn input pin"]
    _1 = 0x01,
}
impl P109pfsIsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsIsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsIsel {
    #[inline(always)]
    fn from(val: u8) -> P109pfsIsel {
        P109pfsIsel::from_bits(val)
    }
}
impl From<P109pfsIsel> for u8 {
    #[inline(always)]
    fn from(val: P109pfsIsel) -> u8 {
        P109pfsIsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P109pfsNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsNcodr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsNcodr {
        P109pfsNcodr::from_bits(val)
    }
}
impl From<P109pfsNcodr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsNcodr) -> u8 {
        P109pfsNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P109pfsPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsPcr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsPcr {
        P109pfsPcr::from_bits(val)
    }
}
impl From<P109pfsPcr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsPcr) -> u8 {
        P109pfsPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P109pfsPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsPdr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsPdr {
        P109pfsPdr::from_bits(val)
    }
}
impl From<P109pfsPdr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsPdr) -> u8 {
        P109pfsPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P109pfsPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsPidr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsPidr {
        P109pfsPidr::from_bits(val)
    }
}
impl From<P109pfsPidr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsPidr) -> u8 {
        P109pfsPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsPmr {
    #[doc = "Uses the pin as a general I/O pin."]
    _0 = 0x0,
    #[doc = "Uses the pin as an I/O port for peripheral functions."]
    _1 = 0x01,
}
impl P109pfsPmr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsPmr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsPmr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsPmr {
        P109pfsPmr::from_bits(val)
    }
}
impl From<P109pfsPmr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsPmr) -> u8 {
        P109pfsPmr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P109pfsPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P109pfsPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P109pfsPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P109pfsPodr {
    #[inline(always)]
    fn from(val: u8) -> P109pfsPodr {
        P109pfsPodr::from_bits(val)
    }
}
impl From<P109pfsPodr> for u8 {
    #[inline(always)]
    fn from(val: P109pfsPodr) -> u8 {
        P109pfsPodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsAsel {
    #[doc = "Used other than as analog pin"]
    _0 = 0x0,
    #[doc = "Used as analog pin"]
    _1 = 0x01,
}
impl P201pfsAsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsAsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsAsel {
    #[inline(always)]
    fn from(val: u8) -> P201pfsAsel {
        P201pfsAsel::from_bits(val)
    }
}
impl From<P201pfsAsel> for u8 {
    #[inline(always)]
    fn from(val: P201pfsAsel) -> u8 {
        P201pfsAsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsByNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P201pfsByNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsByNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsByNcodr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsByNcodr {
        P201pfsByNcodr::from_bits(val)
    }
}
impl From<P201pfsByNcodr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsByNcodr) -> u8 {
        P201pfsByNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsByPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P201pfsByPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsByPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsByPcr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsByPcr {
        P201pfsByPcr::from_bits(val)
    }
}
impl From<P201pfsByPcr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsByPcr) -> u8 {
        P201pfsByPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsByPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P201pfsByPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsByPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsByPdr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsByPdr {
        P201pfsByPdr::from_bits(val)
    }
}
impl From<P201pfsByPdr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsByPdr) -> u8 {
        P201pfsByPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsByPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P201pfsByPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsByPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsByPidr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsByPidr {
        P201pfsByPidr::from_bits(val)
    }
}
impl From<P201pfsByPidr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsByPidr) -> u8 {
        P201pfsByPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsByPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P201pfsByPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsByPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsByPodr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsByPodr {
        P201pfsByPodr::from_bits(val)
    }
}
impl From<P201pfsByPodr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsByPodr) -> u8 {
        P201pfsByPodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsDscr {
    #[doc = "Low drive"]
    _0 = 0x0,
    #[doc = "High drive"]
    _1 = 0x01,
}
impl P201pfsDscr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsDscr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsDscr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsDscr {
        P201pfsDscr::from_bits(val)
    }
}
impl From<P201pfsDscr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsDscr) -> u8 {
        P201pfsDscr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsEof {
    #[doc = "Do not care"]
    _0 = 0x0,
    #[doc = "Detect falling edge"]
    _1 = 0x01,
}
impl P201pfsEof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsEof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsEof {
    #[inline(always)]
    fn from(val: u8) -> P201pfsEof {
        P201pfsEof::from_bits(val)
    }
}
impl From<P201pfsEof> for u8 {
    #[inline(always)]
    fn from(val: P201pfsEof) -> u8 {
        P201pfsEof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsEor {
    #[doc = "Do not care"]
    _0 = 0x0,
    #[doc = "Detect rising edge"]
    _1 = 0x01,
}
impl P201pfsEor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsEor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsEor {
    #[inline(always)]
    fn from(val: u8) -> P201pfsEor {
        P201pfsEor::from_bits(val)
    }
}
impl From<P201pfsEor> for u8 {
    #[inline(always)]
    fn from(val: P201pfsEor) -> u8 {
        P201pfsEor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsHaAsel {
    #[doc = "Used other than as analog pin"]
    _0 = 0x0,
    #[doc = "Used as analog pin"]
    _1 = 0x01,
}
impl P201pfsHaAsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsHaAsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsHaAsel {
    #[inline(always)]
    fn from(val: u8) -> P201pfsHaAsel {
        P201pfsHaAsel::from_bits(val)
    }
}
impl From<P201pfsHaAsel> for u8 {
    #[inline(always)]
    fn from(val: P201pfsHaAsel) -> u8 {
        P201pfsHaAsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsHaDscr {
    #[doc = "Low drive"]
    _0 = 0x0,
    #[doc = "High drive"]
    _1 = 0x01,
}
impl P201pfsHaDscr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsHaDscr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsHaDscr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsHaDscr {
        P201pfsHaDscr::from_bits(val)
    }
}
impl From<P201pfsHaDscr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsHaDscr) -> u8 {
        P201pfsHaDscr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsHaEof {
    #[doc = "Do not care"]
    _0 = 0x0,
    #[doc = "Detect falling edge"]
    _1 = 0x01,
}
impl P201pfsHaEof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsHaEof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsHaEof {
    #[inline(always)]
    fn from(val: u8) -> P201pfsHaEof {
        P201pfsHaEof::from_bits(val)
    }
}
impl From<P201pfsHaEof> for u8 {
    #[inline(always)]
    fn from(val: P201pfsHaEof) -> u8 {
        P201pfsHaEof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsHaEor {
    #[doc = "Do not care"]
    _0 = 0x0,
    #[doc = "Detect rising edge"]
    _1 = 0x01,
}
impl P201pfsHaEor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsHaEor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsHaEor {
    #[inline(always)]
    fn from(val: u8) -> P201pfsHaEor {
        P201pfsHaEor::from_bits(val)
    }
}
impl From<P201pfsHaEor> for u8 {
    #[inline(always)]
    fn from(val: P201pfsHaEor) -> u8 {
        P201pfsHaEor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsHaIsel {
    #[doc = "Not used as IRQn input pin"]
    _0 = 0x0,
    #[doc = "Used as IRQn input pin"]
    _1 = 0x01,
}
impl P201pfsHaIsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsHaIsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsHaIsel {
    #[inline(always)]
    fn from(val: u8) -> P201pfsHaIsel {
        P201pfsHaIsel::from_bits(val)
    }
}
impl From<P201pfsHaIsel> for u8 {
    #[inline(always)]
    fn from(val: P201pfsHaIsel) -> u8 {
        P201pfsHaIsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsHaNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P201pfsHaNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsHaNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsHaNcodr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsHaNcodr {
        P201pfsHaNcodr::from_bits(val)
    }
}
impl From<P201pfsHaNcodr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsHaNcodr) -> u8 {
        P201pfsHaNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsHaPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P201pfsHaPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsHaPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsHaPcr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsHaPcr {
        P201pfsHaPcr::from_bits(val)
    }
}
impl From<P201pfsHaPcr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsHaPcr) -> u8 {
        P201pfsHaPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsHaPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P201pfsHaPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsHaPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsHaPdr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsHaPdr {
        P201pfsHaPdr::from_bits(val)
    }
}
impl From<P201pfsHaPdr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsHaPdr) -> u8 {
        P201pfsHaPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsHaPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P201pfsHaPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsHaPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsHaPidr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsHaPidr {
        P201pfsHaPidr::from_bits(val)
    }
}
impl From<P201pfsHaPidr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsHaPidr) -> u8 {
        P201pfsHaPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsHaPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P201pfsHaPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsHaPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsHaPodr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsHaPodr {
        P201pfsHaPodr::from_bits(val)
    }
}
impl From<P201pfsHaPodr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsHaPodr) -> u8 {
        P201pfsHaPodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsIsel {
    #[doc = "Not used as IRQn input pin"]
    _0 = 0x0,
    #[doc = "Used as IRQn input pin"]
    _1 = 0x01,
}
impl P201pfsIsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsIsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsIsel {
    #[inline(always)]
    fn from(val: u8) -> P201pfsIsel {
        P201pfsIsel::from_bits(val)
    }
}
impl From<P201pfsIsel> for u8 {
    #[inline(always)]
    fn from(val: P201pfsIsel) -> u8 {
        P201pfsIsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P201pfsNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsNcodr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsNcodr {
        P201pfsNcodr::from_bits(val)
    }
}
impl From<P201pfsNcodr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsNcodr) -> u8 {
        P201pfsNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P201pfsPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsPcr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsPcr {
        P201pfsPcr::from_bits(val)
    }
}
impl From<P201pfsPcr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsPcr) -> u8 {
        P201pfsPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P201pfsPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsPdr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsPdr {
        P201pfsPdr::from_bits(val)
    }
}
impl From<P201pfsPdr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsPdr) -> u8 {
        P201pfsPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P201pfsPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsPidr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsPidr {
        P201pfsPidr::from_bits(val)
    }
}
impl From<P201pfsPidr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsPidr) -> u8 {
        P201pfsPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsPmr {
    #[doc = "Uses the pin as a general I/O pin."]
    _0 = 0x0,
    #[doc = "Uses the pin as an I/O port for peripheral functions."]
    _1 = 0x01,
}
impl P201pfsPmr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsPmr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsPmr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsPmr {
        P201pfsPmr::from_bits(val)
    }
}
impl From<P201pfsPmr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsPmr) -> u8 {
        P201pfsPmr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P201pfsPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P201pfsPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P201pfsPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P201pfsPodr {
    #[inline(always)]
    fn from(val: u8) -> P201pfsPodr {
        P201pfsPodr::from_bits(val)
    }
}
impl From<P201pfsPodr> for u8 {
    #[inline(always)]
    fn from(val: P201pfsPodr) -> u8 {
        P201pfsPodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsAsel {
    #[doc = "Used other than as analog pin"]
    _0 = 0x0,
    #[doc = "Used as analog pin"]
    _1 = 0x01,
}
impl P408pfsAsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsAsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsAsel {
    #[inline(always)]
    fn from(val: u8) -> P408pfsAsel {
        P408pfsAsel::from_bits(val)
    }
}
impl From<P408pfsAsel> for u8 {
    #[inline(always)]
    fn from(val: P408pfsAsel) -> u8 {
        P408pfsAsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsByNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P408pfsByNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsByNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsByNcodr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsByNcodr {
        P408pfsByNcodr::from_bits(val)
    }
}
impl From<P408pfsByNcodr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsByNcodr) -> u8 {
        P408pfsByNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsByPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P408pfsByPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsByPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsByPcr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsByPcr {
        P408pfsByPcr::from_bits(val)
    }
}
impl From<P408pfsByPcr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsByPcr) -> u8 {
        P408pfsByPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsByPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P408pfsByPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsByPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsByPdr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsByPdr {
        P408pfsByPdr::from_bits(val)
    }
}
impl From<P408pfsByPdr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsByPdr) -> u8 {
        P408pfsByPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsByPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P408pfsByPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsByPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsByPidr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsByPidr {
        P408pfsByPidr::from_bits(val)
    }
}
impl From<P408pfsByPidr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsByPidr) -> u8 {
        P408pfsByPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsByPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P408pfsByPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsByPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsByPodr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsByPodr {
        P408pfsByPodr::from_bits(val)
    }
}
impl From<P408pfsByPodr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsByPodr) -> u8 {
        P408pfsByPodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsDscr {
    #[doc = "Low drive(DSCR1 = 0)/Middle drive for llC Fast-mode(DSCR1 = 1)"]
    _0 = 0x0,
    #[doc = "Middle drive(DSCR1 = 0)/Setting prohibited(DSCR1 = 1)"]
    _1 = 0x01,
}
impl P408pfsDscr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsDscr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsDscr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsDscr {
        P408pfsDscr::from_bits(val)
    }
}
impl From<P408pfsDscr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsDscr) -> u8 {
        P408pfsDscr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsDscr1 {
    #[doc = "Low drive(DSCR = 0)/Middle drive(DSCR = 1)"]
    _0 = 0x0,
    #[doc = "Middle drive for IIC Fast-mode(DSCR = 0)/Setting prohibited(DSCR = 1)"]
    _1 = 0x01,
}
impl P408pfsDscr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsDscr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsDscr1 {
    #[inline(always)]
    fn from(val: u8) -> P408pfsDscr1 {
        P408pfsDscr1::from_bits(val)
    }
}
impl From<P408pfsDscr1> for u8 {
    #[inline(always)]
    fn from(val: P408pfsDscr1) -> u8 {
        P408pfsDscr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsEof {
    #[doc = "Do not care"]
    _0 = 0x0,
    #[doc = "Detect falling edge"]
    _1 = 0x01,
}
impl P408pfsEof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsEof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsEof {
    #[inline(always)]
    fn from(val: u8) -> P408pfsEof {
        P408pfsEof::from_bits(val)
    }
}
impl From<P408pfsEof> for u8 {
    #[inline(always)]
    fn from(val: P408pfsEof) -> u8 {
        P408pfsEof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsEor {
    #[doc = "Do not care"]
    _0 = 0x0,
    #[doc = "Detect rising edge"]
    _1 = 0x01,
}
impl P408pfsEor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsEor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsEor {
    #[inline(always)]
    fn from(val: u8) -> P408pfsEor {
        P408pfsEor::from_bits(val)
    }
}
impl From<P408pfsEor> for u8 {
    #[inline(always)]
    fn from(val: P408pfsEor) -> u8 {
        P408pfsEor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsHaAsel {
    #[doc = "Used other than as analog pin"]
    _0 = 0x0,
    #[doc = "Used as analog pin"]
    _1 = 0x01,
}
impl P408pfsHaAsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsHaAsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsHaAsel {
    #[inline(always)]
    fn from(val: u8) -> P408pfsHaAsel {
        P408pfsHaAsel::from_bits(val)
    }
}
impl From<P408pfsHaAsel> for u8 {
    #[inline(always)]
    fn from(val: P408pfsHaAsel) -> u8 {
        P408pfsHaAsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsHaDscr {
    #[doc = "Low drive(DSCR1 = 0)/Middle drive for llC Fast-mode(DSCR1 = 1)"]
    _0 = 0x0,
    #[doc = "Middle drive(DSCR1 = 0)/Setting prohibited(DSCR1 = 1)"]
    _1 = 0x01,
}
impl P408pfsHaDscr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsHaDscr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsHaDscr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsHaDscr {
        P408pfsHaDscr::from_bits(val)
    }
}
impl From<P408pfsHaDscr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsHaDscr) -> u8 {
        P408pfsHaDscr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsHaDscr1 {
    #[doc = "Low drive(DSCR = 0)/Middle drive(DSCR = 1)"]
    _0 = 0x0,
    #[doc = "Middle drive for IIC Fast-mode(DSCR = 0)/Setting prohibited(DSCR = 1)"]
    _1 = 0x01,
}
impl P408pfsHaDscr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsHaDscr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsHaDscr1 {
    #[inline(always)]
    fn from(val: u8) -> P408pfsHaDscr1 {
        P408pfsHaDscr1::from_bits(val)
    }
}
impl From<P408pfsHaDscr1> for u8 {
    #[inline(always)]
    fn from(val: P408pfsHaDscr1) -> u8 {
        P408pfsHaDscr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsHaEof {
    #[doc = "Do not care"]
    _0 = 0x0,
    #[doc = "Detect falling edge"]
    _1 = 0x01,
}
impl P408pfsHaEof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsHaEof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsHaEof {
    #[inline(always)]
    fn from(val: u8) -> P408pfsHaEof {
        P408pfsHaEof::from_bits(val)
    }
}
impl From<P408pfsHaEof> for u8 {
    #[inline(always)]
    fn from(val: P408pfsHaEof) -> u8 {
        P408pfsHaEof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsHaEor {
    #[doc = "Do not care"]
    _0 = 0x0,
    #[doc = "Detect rising edge"]
    _1 = 0x01,
}
impl P408pfsHaEor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsHaEor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsHaEor {
    #[inline(always)]
    fn from(val: u8) -> P408pfsHaEor {
        P408pfsHaEor::from_bits(val)
    }
}
impl From<P408pfsHaEor> for u8 {
    #[inline(always)]
    fn from(val: P408pfsHaEor) -> u8 {
        P408pfsHaEor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsHaIsel {
    #[doc = "Not used as IRQn input pin"]
    _0 = 0x0,
    #[doc = "Used as IRQn input pin"]
    _1 = 0x01,
}
impl P408pfsHaIsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsHaIsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsHaIsel {
    #[inline(always)]
    fn from(val: u8) -> P408pfsHaIsel {
        P408pfsHaIsel::from_bits(val)
    }
}
impl From<P408pfsHaIsel> for u8 {
    #[inline(always)]
    fn from(val: P408pfsHaIsel) -> u8 {
        P408pfsHaIsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsHaNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P408pfsHaNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsHaNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsHaNcodr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsHaNcodr {
        P408pfsHaNcodr::from_bits(val)
    }
}
impl From<P408pfsHaNcodr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsHaNcodr) -> u8 {
        P408pfsHaNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsHaPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P408pfsHaPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsHaPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsHaPcr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsHaPcr {
        P408pfsHaPcr::from_bits(val)
    }
}
impl From<P408pfsHaPcr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsHaPcr) -> u8 {
        P408pfsHaPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsHaPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P408pfsHaPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsHaPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsHaPdr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsHaPdr {
        P408pfsHaPdr::from_bits(val)
    }
}
impl From<P408pfsHaPdr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsHaPdr) -> u8 {
        P408pfsHaPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsHaPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P408pfsHaPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsHaPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsHaPidr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsHaPidr {
        P408pfsHaPidr::from_bits(val)
    }
}
impl From<P408pfsHaPidr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsHaPidr) -> u8 {
        P408pfsHaPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsHaPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P408pfsHaPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsHaPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsHaPodr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsHaPodr {
        P408pfsHaPodr::from_bits(val)
    }
}
impl From<P408pfsHaPodr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsHaPodr) -> u8 {
        P408pfsHaPodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsIsel {
    #[doc = "Not used as IRQn input pin"]
    _0 = 0x0,
    #[doc = "Used as IRQn input pin"]
    _1 = 0x01,
}
impl P408pfsIsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsIsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsIsel {
    #[inline(always)]
    fn from(val: u8) -> P408pfsIsel {
        P408pfsIsel::from_bits(val)
    }
}
impl From<P408pfsIsel> for u8 {
    #[inline(always)]
    fn from(val: P408pfsIsel) -> u8 {
        P408pfsIsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsNcodr {
    #[doc = "CMOS output"]
    _0 = 0x0,
    #[doc = "NMOS open-drain output"]
    _1 = 0x01,
}
impl P408pfsNcodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsNcodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsNcodr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsNcodr {
        P408pfsNcodr::from_bits(val)
    }
}
impl From<P408pfsNcodr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsNcodr) -> u8 {
        P408pfsNcodr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsPcr {
    #[doc = "Disables an input pull-up."]
    _0 = 0x0,
    #[doc = "Enables an input pull-up."]
    _1 = 0x01,
}
impl P408pfsPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsPcr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsPcr {
        P408pfsPcr::from_bits(val)
    }
}
impl From<P408pfsPcr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsPcr) -> u8 {
        P408pfsPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsPdr {
    #[doc = "Input (Functions as an input pin.)"]
    _0 = 0x0,
    #[doc = "Output (Functions as an output pin.)"]
    _1 = 0x01,
}
impl P408pfsPdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsPdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsPdr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsPdr {
        P408pfsPdr::from_bits(val)
    }
}
impl From<P408pfsPdr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsPdr) -> u8 {
        P408pfsPdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsPidr {
    #[doc = "Low input"]
    _0 = 0x0,
    #[doc = "High input"]
    _1 = 0x01,
}
impl P408pfsPidr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsPidr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsPidr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsPidr {
        P408pfsPidr::from_bits(val)
    }
}
impl From<P408pfsPidr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsPidr) -> u8 {
        P408pfsPidr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsPmr {
    #[doc = "Uses the pin as a general I/O pin."]
    _0 = 0x0,
    #[doc = "Uses the pin as an I/O port for peripheral functions."]
    _1 = 0x01,
}
impl P408pfsPmr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsPmr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsPmr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsPmr {
        P408pfsPmr::from_bits(val)
    }
}
impl From<P408pfsPmr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsPmr) -> u8 {
        P408pfsPmr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P408pfsPodr {
    #[doc = "Low output"]
    _0 = 0x0,
    #[doc = "High output"]
    _1 = 0x01,
}
impl P408pfsPodr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P408pfsPodr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P408pfsPodr {
    #[inline(always)]
    fn from(val: u8) -> P408pfsPodr {
        P408pfsPodr::from_bits(val)
    }
}
impl From<P408pfsPodr> for u8 {
    #[inline(always)]
    fn from(val: P408pfsPodr) -> u8 {
        P408pfsPodr::to_bits(val)
    }
}
