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
pub enum Eeps {
    #[doc = "An event is counted during the low-level period"]
    _0 = 0x0,
    #[doc = "An event is counted during the high-level period"]
    _1 = 0x01,
}
impl Eeps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eeps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eeps {
    #[inline(always)]
    fn from(val: u8) -> Eeps {
        Eeps::from_bits(val)
    }
}
impl From<Eeps> for u8 {
    #[inline(always)]
    fn from(val: Eeps) -> u8 {
        Eeps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpm {
    #[doc = "Normal mode"]
    _0 = 0x0,
    #[doc = "Low Power mode"]
    _1 = 0x01,
}
impl Lpm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpm {
    #[inline(always)]
    fn from(val: u8) -> Lpm {
        Lpm::from_bits(val)
    }
}
impl From<Lpm> for u8 {
    #[inline(always)]
    fn from(val: Lpm) -> u8 {
        Lpm::to_bits(val)
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
pub enum Tcmaf {
    #[doc = "No match"]
    _0 = 0x0,
    #[doc = "Match."]
    _1 = 0x01,
}
impl Tcmaf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcmaf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcmaf {
    #[inline(always)]
    fn from(val: u8) -> Tcmaf {
        Tcmaf::from_bits(val)
    }
}
impl From<Tcmaf> for u8 {
    #[inline(always)]
    fn from(val: Tcmaf) -> u8 {
        Tcmaf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcmbf {
    #[doc = "No match"]
    _0 = 0x0,
    #[doc = "Match."]
    _1 = 0x01,
}
impl Tcmbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcmbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcmbf {
    #[inline(always)]
    fn from(val: u8) -> Tcmbf {
        Tcmbf::from_bits(val)
    }
}
impl From<Tcmbf> for u8 {
    #[inline(always)]
    fn from(val: Tcmbf) -> u8 {
        Tcmbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcmea {
    #[doc = "Disable compare match A register"]
    _0 = 0x0,
    #[doc = "Enable compare match A register"]
    _1 = 0x01,
}
impl Tcmea {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcmea {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcmea {
    #[inline(always)]
    fn from(val: u8) -> Tcmea {
        Tcmea::from_bits(val)
    }
}
impl From<Tcmea> for u8 {
    #[inline(always)]
    fn from(val: Tcmea) -> u8 {
        Tcmea::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcmeb {
    #[doc = "Disable compare match B register"]
    _0 = 0x0,
    #[doc = "Enable compare match B register"]
    _1 = 0x01,
}
impl Tcmeb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcmeb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcmeb {
    #[inline(always)]
    fn from(val: u8) -> Tcmeb {
        Tcmeb::from_bits(val)
    }
}
impl From<Tcmeb> for u8 {
    #[inline(always)]
    fn from(val: Tcmeb) -> u8 {
        Tcmeb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcstf {
    #[doc = "Count stops"]
    _0 = 0x0,
    #[doc = "Count in progress."]
    _1 = 0x01,
}
impl Tcstf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcstf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcstf {
    #[inline(always)]
    fn from(val: u8) -> Tcstf {
        Tcstf::from_bits(val)
    }
}
impl From<Tcstf> for u8 {
    #[inline(always)]
    fn from(val: Tcstf) -> u8 {
        Tcstf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tedgf {
    #[doc = "No active edge received"]
    _0 = 0x0,
    #[doc = "Active edge received."]
    _1 = 0x01,
}
impl Tedgf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tedgf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tedgf {
    #[inline(always)]
    fn from(val: u8) -> Tedgf {
        Tedgf::from_bits(val)
    }
}
impl From<Tedgf> for u8 {
    #[inline(always)]
    fn from(val: Tedgf) -> u8 {
        Tedgf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tedgpl {
    #[doc = "Single-edge"]
    _0 = 0x0,
    #[doc = "Both-edge."]
    _1 = 0x01,
}
impl Tedgpl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tedgpl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tedgpl {
    #[inline(always)]
    fn from(val: u8) -> Tedgpl {
        Tedgpl::from_bits(val)
    }
}
impl From<Tedgpl> for u8 {
    #[inline(always)]
    fn from(val: Tedgpl) -> u8 {
        Tedgpl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ties {
    #[doc = "External event input is disabled during Software Standby mode"]
    _0 = 0x0,
    #[doc = "External event input is enabled during Software Standby mode."]
    _1 = 0x01,
}
impl Ties {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ties {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ties {
    #[inline(always)]
    fn from(val: u8) -> Ties {
        Ties::from_bits(val)
    }
}
impl From<Ties> for u8 {
    #[inline(always)]
    fn from(val: Ties) -> u8 {
        Ties::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Toe {
    #[doc = "AGTOn output disabled"]
    _0 = 0x0,
    #[doc = "AGTOn output enabled."]
    _1 = 0x01,
}
impl Toe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Toe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Toe {
    #[inline(always)]
    fn from(val: u8) -> Toe {
        Toe::from_bits(val)
    }
}
impl From<Toe> for u8 {
    #[inline(always)]
    fn from(val: Toe) -> u8 {
        Toe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Toea {
    #[doc = "AGTOA output disabled (port)"]
    _0 = 0x0,
    #[doc = "AGTOA output enabled"]
    _1 = 0x01,
}
impl Toea {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Toea {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Toea {
    #[inline(always)]
    fn from(val: u8) -> Toea {
        Toea::from_bits(val)
    }
}
impl From<Toea> for u8 {
    #[inline(always)]
    fn from(val: Toea) -> u8 {
        Toea::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Toeb {
    #[doc = "AGTOB output disabled (port)"]
    _0 = 0x0,
    #[doc = "AGTOB output enabled"]
    _1 = 0x01,
}
impl Toeb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Toeb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Toeb {
    #[inline(always)]
    fn from(val: u8) -> Toeb {
        Toeb::from_bits(val)
    }
}
impl From<Toeb> for u8 {
    #[inline(always)]
    fn from(val: Toeb) -> u8 {
        Toeb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Topola {
    #[doc = "AGTOA Output is started at low"]
    _0 = 0x0,
    #[doc = "AGTOA Output is started at high"]
    _1 = 0x01,
}
impl Topola {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Topola {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Topola {
    #[inline(always)]
    fn from(val: u8) -> Topola {
        Topola::from_bits(val)
    }
}
impl From<Topola> for u8 {
    #[inline(always)]
    fn from(val: Topola) -> u8 {
        Topola::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Topolb {
    #[doc = "AGTOB Output is started at low"]
    _0 = 0x0,
    #[doc = "AGTOB Output is started at high"]
    _1 = 0x01,
}
impl Topolb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Topolb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Topolb {
    #[inline(always)]
    fn from(val: u8) -> Topolb {
        Topolb::from_bits(val)
    }
}
impl From<Topolb> for u8 {
    #[inline(always)]
    fn from(val: Topolb) -> u8 {
        Topolb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstart {
    #[doc = "Count stops"]
    _0 = 0x0,
    #[doc = "Count starts."]
    _1 = 0x01,
}
impl Tstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstart {
    #[inline(always)]
    fn from(val: u8) -> Tstart {
        Tstart::from_bits(val)
    }
}
impl From<Tstart> for u8 {
    #[inline(always)]
    fn from(val: Tstart) -> u8 {
        Tstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstop {
    #[doc = "Writing is invalid"]
    _0 = 0x0,
    #[doc = "The count is forcibly stopped."]
    _1 = 0x01,
}
impl Tstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstop {
    #[inline(always)]
    fn from(val: u8) -> Tstop {
        Tstop::from_bits(val)
    }
}
impl From<Tstop> for u8 {
    #[inline(always)]
    fn from(val: Tstop) -> u8 {
        Tstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tundf {
    #[doc = "No match"]
    _0 = 0x0,
    #[doc = "Match."]
    _1 = 0x01,
}
impl Tundf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tundf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tundf {
    #[inline(always)]
    fn from(val: u8) -> Tundf {
        Tundf::from_bits(val)
    }
}
impl From<Tundf> for u8 {
    #[inline(always)]
    fn from(val: Tundf) -> u8 {
        Tundf::to_bits(val)
    }
}
