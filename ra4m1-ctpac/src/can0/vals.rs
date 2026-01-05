#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adef {
    #[doc = "No ACK delimiter error detected"]
    _0 = 0x0,
    #[doc = "ACK delimiter error detected"]
    _1 = 0x01,
}
impl Adef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adef {
    #[inline(always)]
    fn from(val: u8) -> Adef {
        Adef::from_bits(val)
    }
}
impl From<Adef> for u8 {
    #[inline(always)]
    fn from(val: Adef) -> u8 {
        Adef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aef {
    #[doc = "No ACK error detected"]
    _0 = 0x0,
    #[doc = "ACK error detected"]
    _1 = 0x01,
}
impl Aef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aef {
    #[inline(always)]
    fn from(val: u8) -> Aef {
        Aef::from_bits(val)
    }
}
impl From<Aef> for u8 {
    #[inline(always)]
    fn from(val: Aef) -> u8 {
        Aef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Be0f {
    #[doc = "No bit error (dominant) detected"]
    _0 = 0x0,
    #[doc = "Bit error (dominant) detected"]
    _1 = 0x01,
}
impl Be0f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Be0f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Be0f {
    #[inline(always)]
    fn from(val: u8) -> Be0f {
        Be0f::from_bits(val)
    }
}
impl From<Be0f> for u8 {
    #[inline(always)]
    fn from(val: Be0f) -> u8 {
        Be0f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Be1f {
    #[doc = "No bit error (recessive) detected"]
    _0 = 0x0,
    #[doc = "Bit error (recessive) detected"]
    _1 = 0x01,
}
impl Be1f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Be1f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Be1f {
    #[inline(always)]
    fn from(val: u8) -> Be1f {
        Be1f::from_bits(val)
    }
}
impl From<Be1f> for u8 {
    #[inline(always)]
    fn from(val: Be1f) -> u8 {
        Be1f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Beie {
    #[doc = "Bus error interrupt disabled"]
    _0 = 0x0,
    #[doc = "Bus error interrupt enabled"]
    _1 = 0x01,
}
impl Beie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Beie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Beie {
    #[inline(always)]
    fn from(val: u8) -> Beie {
        Beie::from_bits(val)
    }
}
impl From<Beie> for u8 {
    #[inline(always)]
    fn from(val: Beie) -> u8 {
        Beie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Beif {
    #[doc = "No bus error detected"]
    _0 = 0x0,
    #[doc = "Bus error detected"]
    _1 = 0x01,
}
impl Beif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Beif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Beif {
    #[inline(always)]
    fn from(val: u8) -> Beif {
        Beif::from_bits(val)
    }
}
impl From<Beif> for u8 {
    #[inline(always)]
    fn from(val: Beif) -> u8 {
        Beif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Blie {
    #[doc = "Bus lock interrupt disabled"]
    _0 = 0x0,
    #[doc = "Bus lock interrupt enabled"]
    _1 = 0x01,
}
impl Blie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Blie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Blie {
    #[inline(always)]
    fn from(val: u8) -> Blie {
        Blie::from_bits(val)
    }
}
impl From<Blie> for u8 {
    #[inline(always)]
    fn from(val: Blie) -> u8 {
        Blie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Blif {
    #[doc = "No bus lock detected"]
    _0 = 0x0,
    #[doc = "Bus lock detected"]
    _1 = 0x01,
}
impl Blif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Blif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Blif {
    #[inline(always)]
    fn from(val: u8) -> Blif {
        Blif::from_bits(val)
    }
}
impl From<Blif> for u8 {
    #[inline(always)]
    fn from(val: Blif) -> u8 {
        Blif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Boeie {
    #[doc = "Bus-off entry interrupt disabled"]
    _0 = 0x0,
    #[doc = "Bus-off entry interrupt enabled"]
    _1 = 0x01,
}
impl Boeie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Boeie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Boeie {
    #[inline(always)]
    fn from(val: u8) -> Boeie {
        Boeie::from_bits(val)
    }
}
impl From<Boeie> for u8 {
    #[inline(always)]
    fn from(val: Boeie) -> u8 {
        Boeie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Boeif {
    #[doc = "No bus-off entry detected"]
    _0 = 0x0,
    #[doc = "Bus-off entry detected"]
    _1 = 0x01,
}
impl Boeif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Boeif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Boeif {
    #[inline(always)]
    fn from(val: u8) -> Boeif {
        Boeif::from_bits(val)
    }
}
impl From<Boeif> for u8 {
    #[inline(always)]
    fn from(val: Boeif) -> u8 {
        Boeif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bom {
    #[doc = "Normal mode (ISO11898-1 compliant)"]
    _00 = 0x0,
    #[doc = "Entry to CAN halt mode automatically at bus-off entry"]
    _01 = 0x01,
    #[doc = "Entry to CAN halt mode automatically at bus-off end"]
    _10 = 0x02,
    #[doc = "Entry to CAN halt mode (during bus-off recovery period)"]
    _11 = 0x03,
}
impl Bom {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bom {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bom {
    #[inline(always)]
    fn from(val: u8) -> Bom {
        Bom::from_bits(val)
    }
}
impl From<Bom> for u8 {
    #[inline(always)]
    fn from(val: Bom) -> u8 {
        Bom::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Borie {
    #[doc = "Bus-off recovery interrupt disabled"]
    _0 = 0x0,
    #[doc = "Bus-off recovery interrupt enabled"]
    _1 = 0x01,
}
impl Borie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Borie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Borie {
    #[inline(always)]
    fn from(val: u8) -> Borie {
        Borie::from_bits(val)
    }
}
impl From<Borie> for u8 {
    #[inline(always)]
    fn from(val: Borie) -> u8 {
        Borie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Borif {
    #[doc = "No bus-off recovery detected"]
    _0 = 0x0,
    #[doc = "Bus-off recovery detected"]
    _1 = 0x01,
}
impl Borif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Borif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Borif {
    #[inline(always)]
    fn from(val: u8) -> Borif {
        Borif::from_bits(val)
    }
}
impl From<Borif> for u8 {
    #[inline(always)]
    fn from(val: Borif) -> u8 {
        Borif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bost {
    #[doc = "Not in bus-off state"]
    _0 = 0x0,
    #[doc = "In bus-off state"]
    _1 = 0x01,
}
impl Bost {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bost {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bost {
    #[inline(always)]
    fn from(val: u8) -> Bost {
        Bost::from_bits(val)
    }
}
impl From<Bost> for u8 {
    #[inline(always)]
    fn from(val: Bost) -> u8 {
        Bost::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Canm {
    #[doc = "CAN operation mode"]
    _00 = 0x0,
    #[doc = "CAN reset mode"]
    _01 = 0x01,
    #[doc = "CAN halt mode"]
    _10 = 0x02,
    #[doc = "CAN reset mode (forcible transition)"]
    _11 = 0x03,
}
impl Canm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Canm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Canm {
    #[inline(always)]
    fn from(val: u8) -> Canm {
        Canm::from_bits(val)
    }
}
impl From<Canm> for u8 {
    #[inline(always)]
    fn from(val: Canm) -> u8 {
        Canm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cclks {
    #[doc = "PCLK (generated by the PLL clock)"]
    _0 = 0x0,
    #[doc = "CANMCLK (generated by the main clock)"]
    _1 = 0x01,
}
impl Cclks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cclks {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cclks {
    #[inline(always)]
    fn from(val: u8) -> Cclks {
        Cclks::from_bits(val)
    }
}
impl From<Cclks> for u8 {
    #[inline(always)]
    fn from(val: Cclks) -> u8 {
        Cclks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cef {
    #[doc = "No CRC error detected"]
    _0 = 0x0,
    #[doc = "CRC error detected"]
    _1 = 0x01,
}
impl Cef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cef {
    #[inline(always)]
    fn from(val: u8) -> Cef {
        Cef::from_bits(val)
    }
}
impl From<Cef> for u8 {
    #[inline(always)]
    fn from(val: Cef) -> u8 {
        Cef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dlc {
    #[doc = "Data length = 0 byte"]
    _0000 = 0x0,
    #[doc = "Data length = 1 byte"]
    _0001 = 0x01,
    #[doc = "Data length = 2 bytes"]
    _0010 = 0x02,
    #[doc = "Data length = 3 bytes"]
    _0011 = 0x03,
    #[doc = "Data length = 4 bytes"]
    _0100 = 0x04,
    #[doc = "Data length = 5 bytes"]
    _0101 = 0x05,
    #[doc = "Data length = 6 bytes"]
    _0110 = 0x06,
    #[doc = "Data length = 7 bytes"]
    _0111 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Dlc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dlc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dlc {
    #[inline(always)]
    fn from(val: u8) -> Dlc {
        Dlc::from_bits(val)
    }
}
impl From<Dlc> for u8 {
    #[inline(always)]
    fn from(val: Dlc) -> u8 {
        Dlc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edpm {
    #[doc = "Output of first detected error code"]
    _0 = 0x0,
    #[doc = "Output of accumulated error code"]
    _1 = 0x01,
}
impl Edpm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edpm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edpm {
    #[inline(always)]
    fn from(val: u8) -> Edpm {
        Edpm::from_bits(val)
    }
}
impl From<Edpm> for u8 {
    #[inline(always)]
    fn from(val: Edpm) -> u8 {
        Edpm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Epie {
    #[doc = "Error-passive interrupt disabled"]
    _0 = 0x0,
    #[doc = "Error-passive interrupt enabled"]
    _1 = 0x01,
}
impl Epie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Epie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Epie {
    #[inline(always)]
    fn from(val: u8) -> Epie {
        Epie::from_bits(val)
    }
}
impl From<Epie> for u8 {
    #[inline(always)]
    fn from(val: Epie) -> u8 {
        Epie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Epif {
    #[doc = "No error-passive detected"]
    _0 = 0x0,
    #[doc = "Error-passive detected"]
    _1 = 0x01,
}
impl Epif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Epif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Epif {
    #[inline(always)]
    fn from(val: u8) -> Epif {
        Epif::from_bits(val)
    }
}
impl From<Epif> for u8 {
    #[inline(always)]
    fn from(val: Epif) -> u8 {
        Epif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Epst {
    #[doc = "Not in error-passive state"]
    _0 = 0x0,
    #[doc = "In error-passive state"]
    _1 = 0x01,
}
impl Epst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Epst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Epst {
    #[inline(always)]
    fn from(val: u8) -> Epst {
        Epst::from_bits(val)
    }
}
impl From<Epst> for u8 {
    #[inline(always)]
    fn from(val: Epst) -> u8 {
        Epst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Est {
    #[doc = "No error occurred"]
    _0 = 0x0,
    #[doc = "Error occurred"]
    _1 = 0x01,
}
impl Est {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Est {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Est {
    #[inline(always)]
    fn from(val: u8) -> Est {
        Est::from_bits(val)
    }
}
impl From<Est> for u8 {
    #[inline(always)]
    fn from(val: Est) -> u8 {
        Est::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ewie {
    #[doc = "Error-warning interrupt disabled"]
    _0 = 0x0,
    #[doc = "Error-warning interrupt enabled"]
    _1 = 0x01,
}
impl Ewie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ewie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ewie {
    #[inline(always)]
    fn from(val: u8) -> Ewie {
        Ewie::from_bits(val)
    }
}
impl From<Ewie> for u8 {
    #[inline(always)]
    fn from(val: Ewie) -> u8 {
        Ewie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ewif {
    #[doc = "No error-warning detected"]
    _0 = 0x0,
    #[doc = "Error-warning detected"]
    _1 = 0x01,
}
impl Ewif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ewif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ewif {
    #[inline(always)]
    fn from(val: u8) -> Ewif {
        Ewif::from_bits(val)
    }
}
impl From<Ewif> for u8 {
    #[inline(always)]
    fn from(val: Ewif) -> u8 {
        Ewif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fef {
    #[doc = "No form error detected"]
    _0 = 0x0,
    #[doc = "Form error detected"]
    _1 = 0x01,
}
impl Fef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fef {
    #[inline(always)]
    fn from(val: u8) -> Fef {
        Fef::from_bits(val)
    }
}
impl From<Fef> for u8 {
    #[inline(always)]
    fn from(val: Fef) -> u8 {
        Fef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FidcrIde {
    #[doc = "Standard ID"]
    _0 = 0x0,
    #[doc = "Extended ID"]
    _1 = 0x01,
}
impl FidcrIde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FidcrIde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FidcrIde {
    #[inline(always)]
    fn from(val: u8) -> FidcrIde {
        FidcrIde::from_bits(val)
    }
}
impl From<FidcrIde> for u8 {
    #[inline(always)]
    fn from(val: FidcrIde) -> u8 {
        FidcrIde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FidcrRtr {
    #[doc = "Data frame"]
    _0 = 0x0,
    #[doc = "Remote frame"]
    _1 = 0x01,
}
impl FidcrRtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FidcrRtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FidcrRtr {
    #[inline(always)]
    fn from(val: u8) -> FidcrRtr {
        FidcrRtr::from_bits(val)
    }
}
impl From<FidcrRtr> for u8 {
    #[inline(always)]
    fn from(val: FidcrRtr) -> u8 {
        FidcrRtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fmlst {
    #[doc = "RFMLF bit = 0"]
    _0 = 0x0,
    #[doc = "RFMLF bit = 1"]
    _1 = 0x01,
}
impl Fmlst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmlst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmlst {
    #[inline(always)]
    fn from(val: u8) -> Fmlst {
        Fmlst::from_bits(val)
    }
}
impl From<Fmlst> for u8 {
    #[inline(always)]
    fn from(val: Fmlst) -> u8 {
        Fmlst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hltst {
    #[doc = "Not in CAN halt mode"]
    _0 = 0x0,
    #[doc = "In CAN halt mode"]
    _1 = 0x01,
}
impl Hltst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hltst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hltst {
    #[inline(always)]
    fn from(val: u8) -> Hltst {
        Hltst::from_bits(val)
    }
}
impl From<Hltst> for u8 {
    #[inline(always)]
    fn from(val: Hltst) -> u8 {
        Hltst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idfm {
    #[doc = "Standard ID mode.All mailboxes (including FIFO mailboxes) handle only standard Ids."]
    _00 = 0x0,
    #[doc = "Extended ID mode.All mailboxes (including FIFO mailboxes) handle only extended IDs."]
    _01 = 0x01,
    #[doc = "Mixed ID mode.All mailboxes (including FIFO mailboxes) handle both standard IDs and extended IDs. Standard IDs or extended IDs are specified by using the IDE bit in the corresponding mailbox in normal mailbox mode. In FIFO mailbox mode, the IDE bit in the corresponding mailbox is used for mailboxes \\[0\\] to \\[23\\], the IDE bits in FIDCR0 and FIDCR1 are used for the receive FIFO, and the IDE bit in mailbox \\[24\\] is used for the transmit FIFO."]
    _10 = 0x02,
    #[doc = "Do not use this combination"]
    _11 = 0x03,
}
impl Idfm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idfm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idfm {
    #[inline(always)]
    fn from(val: u8) -> Idfm {
        Idfm::from_bits(val)
    }
}
impl From<Idfm> for u8 {
    #[inline(always)]
    fn from(val: Idfm) -> u8 {
        Idfm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Invaldata {
    #[doc = "Message valid"]
    _0 = 0x0,
    #[doc = "Message being updated"]
    _1 = 0x01,
}
impl Invaldata {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Invaldata {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Invaldata {
    #[inline(always)]
    fn from(val: u8) -> Invaldata {
        Invaldata::from_bits(val)
    }
}
impl From<Invaldata> for u8 {
    #[inline(always)]
    fn from(val: Invaldata) -> u8 {
        Invaldata::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MbIdIde {
    #[doc = "Standard ID"]
    _0 = 0x0,
    #[doc = "Extended ID"]
    _1 = 0x01,
}
impl MbIdIde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MbIdIde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MbIdIde {
    #[inline(always)]
    fn from(val: u8) -> MbIdIde {
        MbIdIde::from_bits(val)
    }
}
impl From<MbIdIde> for u8 {
    #[inline(always)]
    fn from(val: MbIdIde) -> u8 {
        MbIdIde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MbIdRtr {
    #[doc = "Data frame"]
    _0 = 0x0,
    #[doc = "Remote frame"]
    _1 = 0x01,
}
impl MbIdRtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MbIdRtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MbIdRtr {
    #[inline(always)]
    fn from(val: u8) -> MbIdRtr {
        MbIdRtr::from_bits(val)
    }
}
impl From<MbIdRtr> for u8 {
    #[inline(always)]
    fn from(val: MbIdRtr) -> u8 {
        MbIdRtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbm {
    #[doc = "Normal mailbox mode"]
    _0 = 0x0,
    #[doc = "FIFO mailbox mode"]
    _1 = 0x01,
}
impl Mbm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbm {
    #[inline(always)]
    fn from(val: u8) -> Mbm {
        Mbm::from_bits(val)
    }
}
impl From<Mbm> for u8 {
    #[inline(always)]
    fn from(val: Mbm) -> u8 {
        Mbm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbsm {
    #[doc = "Receive mailbox search mode"]
    _00 = 0x0,
    #[doc = "Transmit mailbox search mode"]
    _01 = 0x01,
    #[doc = "Message lost search mode"]
    _10 = 0x02,
    #[doc = "Channel search mode"]
    _11 = 0x03,
}
impl Mbsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbsm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbsm {
    #[inline(always)]
    fn from(val: u8) -> Mbsm {
        Mbsm::from_bits(val)
    }
}
impl From<Mbsm> for u8 {
    #[inline(always)]
    fn from(val: Mbsm) -> u8 {
        Mbsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MctlRxOneshot {
    #[doc = "One-shot reception or one-shot transmission disabled"]
    _0 = 0x0,
    #[doc = "One-shot reception or one-shot transmission enabled"]
    _1 = 0x01,
}
impl MctlRxOneshot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MctlRxOneshot {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MctlRxOneshot {
    #[inline(always)]
    fn from(val: u8) -> MctlRxOneshot {
        MctlRxOneshot::from_bits(val)
    }
}
impl From<MctlRxOneshot> for u8 {
    #[inline(always)]
    fn from(val: MctlRxOneshot) -> u8 {
        MctlRxOneshot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MctlRxRecreq {
    #[doc = "Not configured for reception"]
    _0 = 0x0,
    #[doc = "Configured for reception"]
    _1 = 0x01,
}
impl MctlRxRecreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MctlRxRecreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MctlRxRecreq {
    #[inline(always)]
    fn from(val: u8) -> MctlRxRecreq {
        MctlRxRecreq::from_bits(val)
    }
}
impl From<MctlRxRecreq> for u8 {
    #[inline(always)]
    fn from(val: MctlRxRecreq) -> u8 {
        MctlRxRecreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MctlRxTrmreq {
    #[doc = "Not configured for transmission"]
    _0 = 0x0,
    #[doc = "Configured for transmission"]
    _1 = 0x01,
}
impl MctlRxTrmreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MctlRxTrmreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MctlRxTrmreq {
    #[inline(always)]
    fn from(val: u8) -> MctlRxTrmreq {
        MctlRxTrmreq::from_bits(val)
    }
}
impl From<MctlRxTrmreq> for u8 {
    #[inline(always)]
    fn from(val: MctlRxTrmreq) -> u8 {
        MctlRxTrmreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MctlTxOneshot {
    #[doc = "One-shot reception or one-shot transmission disabled"]
    _0 = 0x0,
    #[doc = "One-shot reception or one-shot transmission enabled"]
    _1 = 0x01,
}
impl MctlTxOneshot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MctlTxOneshot {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MctlTxOneshot {
    #[inline(always)]
    fn from(val: u8) -> MctlTxOneshot {
        MctlTxOneshot::from_bits(val)
    }
}
impl From<MctlTxOneshot> for u8 {
    #[inline(always)]
    fn from(val: MctlTxOneshot) -> u8 {
        MctlTxOneshot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MctlTxRecreq {
    #[doc = "Not configured for reception"]
    _0 = 0x0,
    #[doc = "Configured for reception"]
    _1 = 0x01,
}
impl MctlTxRecreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MctlTxRecreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MctlTxRecreq {
    #[inline(always)]
    fn from(val: u8) -> MctlTxRecreq {
        MctlTxRecreq::from_bits(val)
    }
}
impl From<MctlTxRecreq> for u8 {
    #[inline(always)]
    fn from(val: MctlTxRecreq) -> u8 {
        MctlTxRecreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MctlTxTrmreq {
    #[doc = "Not configured for transmission"]
    _0 = 0x0,
    #[doc = "Configured for transmission"]
    _1 = 0x01,
}
impl MctlTxTrmreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MctlTxTrmreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MctlTxTrmreq {
    #[inline(always)]
    fn from(val: u8) -> MctlTxTrmreq {
        MctlTxTrmreq::from_bits(val)
    }
}
impl From<MctlTxTrmreq> for u8 {
    #[inline(always)]
    fn from(val: MctlTxTrmreq) -> u8 {
        MctlTxTrmreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb0 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb0 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb0 {
        MierFifoMb0::from_bits(val)
    }
}
impl From<MierFifoMb0> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb0) -> u8 {
        MierFifoMb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb1 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb1 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb1 {
        MierFifoMb1::from_bits(val)
    }
}
impl From<MierFifoMb1> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb1) -> u8 {
        MierFifoMb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb10 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb10 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb10 {
        MierFifoMb10::from_bits(val)
    }
}
impl From<MierFifoMb10> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb10) -> u8 {
        MierFifoMb10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb11 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb11 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb11 {
        MierFifoMb11::from_bits(val)
    }
}
impl From<MierFifoMb11> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb11) -> u8 {
        MierFifoMb11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb12 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb12 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb12 {
        MierFifoMb12::from_bits(val)
    }
}
impl From<MierFifoMb12> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb12) -> u8 {
        MierFifoMb12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb13 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb13 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb13 {
        MierFifoMb13::from_bits(val)
    }
}
impl From<MierFifoMb13> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb13) -> u8 {
        MierFifoMb13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb14 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb14 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb14 {
        MierFifoMb14::from_bits(val)
    }
}
impl From<MierFifoMb14> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb14) -> u8 {
        MierFifoMb14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb15 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb15 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb15 {
        MierFifoMb15::from_bits(val)
    }
}
impl From<MierFifoMb15> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb15) -> u8 {
        MierFifoMb15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb16 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb16 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb16 {
        MierFifoMb16::from_bits(val)
    }
}
impl From<MierFifoMb16> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb16) -> u8 {
        MierFifoMb16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb17 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb17 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb17 {
        MierFifoMb17::from_bits(val)
    }
}
impl From<MierFifoMb17> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb17) -> u8 {
        MierFifoMb17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb18 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb18 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb18 {
        MierFifoMb18::from_bits(val)
    }
}
impl From<MierFifoMb18> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb18) -> u8 {
        MierFifoMb18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb19 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb19 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb19 {
        MierFifoMb19::from_bits(val)
    }
}
impl From<MierFifoMb19> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb19) -> u8 {
        MierFifoMb19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb2 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb2 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb2 {
        MierFifoMb2::from_bits(val)
    }
}
impl From<MierFifoMb2> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb2) -> u8 {
        MierFifoMb2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb20 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb20 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb20 {
        MierFifoMb20::from_bits(val)
    }
}
impl From<MierFifoMb20> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb20) -> u8 {
        MierFifoMb20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb21 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb21 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb21 {
        MierFifoMb21::from_bits(val)
    }
}
impl From<MierFifoMb21> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb21) -> u8 {
        MierFifoMb21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb22 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb22 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb22 {
        MierFifoMb22::from_bits(val)
    }
}
impl From<MierFifoMb22> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb22) -> u8 {
        MierFifoMb22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb23 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb23 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb23 {
        MierFifoMb23::from_bits(val)
    }
}
impl From<MierFifoMb23> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb23) -> u8 {
        MierFifoMb23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb24 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb24 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb24 {
        MierFifoMb24::from_bits(val)
    }
}
impl From<MierFifoMb24> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb24) -> u8 {
        MierFifoMb24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb25 {
    #[doc = "Every time transmission is completed"]
    _0 = 0x0,
    #[doc = "When the transmit FIFO becomes empty due to completion of transmission"]
    _1 = 0x01,
}
impl MierFifoMb25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb25 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb25 {
        MierFifoMb25::from_bits(val)
    }
}
impl From<MierFifoMb25> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb25) -> u8 {
        MierFifoMb25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb28 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb28 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb28 {
        MierFifoMb28::from_bits(val)
    }
}
impl From<MierFifoMb28> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb28) -> u8 {
        MierFifoMb28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb29 {
    #[doc = "Every time reception is completed"]
    _0 = 0x0,
    #[doc = "When the receive FIFO becomes buffer warning by completion of reception"]
    _1 = 0x01,
}
impl MierFifoMb29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb29 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb29 {
        MierFifoMb29::from_bits(val)
    }
}
impl From<MierFifoMb29> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb29) -> u8 {
        MierFifoMb29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb3 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb3 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb3 {
        MierFifoMb3::from_bits(val)
    }
}
impl From<MierFifoMb3> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb3) -> u8 {
        MierFifoMb3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb4 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb4 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb4 {
        MierFifoMb4::from_bits(val)
    }
}
impl From<MierFifoMb4> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb4) -> u8 {
        MierFifoMb4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb5 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb5 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb5 {
        MierFifoMb5::from_bits(val)
    }
}
impl From<MierFifoMb5> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb5) -> u8 {
        MierFifoMb5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb6 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb6 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb6 {
        MierFifoMb6::from_bits(val)
    }
}
impl From<MierFifoMb6> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb6) -> u8 {
        MierFifoMb6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb7 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb7 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb7 {
        MierFifoMb7::from_bits(val)
    }
}
impl From<MierFifoMb7> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb7) -> u8 {
        MierFifoMb7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb8 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb8 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb8 {
        MierFifoMb8::from_bits(val)
    }
}
impl From<MierFifoMb8> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb8) -> u8 {
        MierFifoMb8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFifoMb9 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierFifoMb9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFifoMb9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFifoMb9 {
    #[inline(always)]
    fn from(val: u8) -> MierFifoMb9 {
        MierFifoMb9::from_bits(val)
    }
}
impl From<MierFifoMb9> for u8 {
    #[inline(always)]
    fn from(val: MierFifoMb9) -> u8 {
        MierFifoMb9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb0 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb0 {
    #[inline(always)]
    fn from(val: u8) -> MierMb0 {
        MierMb0::from_bits(val)
    }
}
impl From<MierMb0> for u8 {
    #[inline(always)]
    fn from(val: MierMb0) -> u8 {
        MierMb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb1 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb1 {
    #[inline(always)]
    fn from(val: u8) -> MierMb1 {
        MierMb1::from_bits(val)
    }
}
impl From<MierMb1> for u8 {
    #[inline(always)]
    fn from(val: MierMb1) -> u8 {
        MierMb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb10 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb10 {
    #[inline(always)]
    fn from(val: u8) -> MierMb10 {
        MierMb10::from_bits(val)
    }
}
impl From<MierMb10> for u8 {
    #[inline(always)]
    fn from(val: MierMb10) -> u8 {
        MierMb10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb11 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb11 {
    #[inline(always)]
    fn from(val: u8) -> MierMb11 {
        MierMb11::from_bits(val)
    }
}
impl From<MierMb11> for u8 {
    #[inline(always)]
    fn from(val: MierMb11) -> u8 {
        MierMb11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb12 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb12 {
    #[inline(always)]
    fn from(val: u8) -> MierMb12 {
        MierMb12::from_bits(val)
    }
}
impl From<MierMb12> for u8 {
    #[inline(always)]
    fn from(val: MierMb12) -> u8 {
        MierMb12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb13 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb13 {
    #[inline(always)]
    fn from(val: u8) -> MierMb13 {
        MierMb13::from_bits(val)
    }
}
impl From<MierMb13> for u8 {
    #[inline(always)]
    fn from(val: MierMb13) -> u8 {
        MierMb13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb14 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb14 {
    #[inline(always)]
    fn from(val: u8) -> MierMb14 {
        MierMb14::from_bits(val)
    }
}
impl From<MierMb14> for u8 {
    #[inline(always)]
    fn from(val: MierMb14) -> u8 {
        MierMb14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb15 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb15 {
    #[inline(always)]
    fn from(val: u8) -> MierMb15 {
        MierMb15::from_bits(val)
    }
}
impl From<MierMb15> for u8 {
    #[inline(always)]
    fn from(val: MierMb15) -> u8 {
        MierMb15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb16 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb16 {
    #[inline(always)]
    fn from(val: u8) -> MierMb16 {
        MierMb16::from_bits(val)
    }
}
impl From<MierMb16> for u8 {
    #[inline(always)]
    fn from(val: MierMb16) -> u8 {
        MierMb16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb17 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb17 {
    #[inline(always)]
    fn from(val: u8) -> MierMb17 {
        MierMb17::from_bits(val)
    }
}
impl From<MierMb17> for u8 {
    #[inline(always)]
    fn from(val: MierMb17) -> u8 {
        MierMb17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb18 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb18 {
    #[inline(always)]
    fn from(val: u8) -> MierMb18 {
        MierMb18::from_bits(val)
    }
}
impl From<MierMb18> for u8 {
    #[inline(always)]
    fn from(val: MierMb18) -> u8 {
        MierMb18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb19 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb19 {
    #[inline(always)]
    fn from(val: u8) -> MierMb19 {
        MierMb19::from_bits(val)
    }
}
impl From<MierMb19> for u8 {
    #[inline(always)]
    fn from(val: MierMb19) -> u8 {
        MierMb19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb2 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb2 {
    #[inline(always)]
    fn from(val: u8) -> MierMb2 {
        MierMb2::from_bits(val)
    }
}
impl From<MierMb2> for u8 {
    #[inline(always)]
    fn from(val: MierMb2) -> u8 {
        MierMb2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb20 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb20 {
    #[inline(always)]
    fn from(val: u8) -> MierMb20 {
        MierMb20::from_bits(val)
    }
}
impl From<MierMb20> for u8 {
    #[inline(always)]
    fn from(val: MierMb20) -> u8 {
        MierMb20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb21 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb21 {
    #[inline(always)]
    fn from(val: u8) -> MierMb21 {
        MierMb21::from_bits(val)
    }
}
impl From<MierMb21> for u8 {
    #[inline(always)]
    fn from(val: MierMb21) -> u8 {
        MierMb21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb22 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb22 {
    #[inline(always)]
    fn from(val: u8) -> MierMb22 {
        MierMb22::from_bits(val)
    }
}
impl From<MierMb22> for u8 {
    #[inline(always)]
    fn from(val: MierMb22) -> u8 {
        MierMb22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb23 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb23 {
    #[inline(always)]
    fn from(val: u8) -> MierMb23 {
        MierMb23::from_bits(val)
    }
}
impl From<MierMb23> for u8 {
    #[inline(always)]
    fn from(val: MierMb23) -> u8 {
        MierMb23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb24 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb24 {
    #[inline(always)]
    fn from(val: u8) -> MierMb24 {
        MierMb24::from_bits(val)
    }
}
impl From<MierMb24> for u8 {
    #[inline(always)]
    fn from(val: MierMb24) -> u8 {
        MierMb24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb25 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb25 {
    #[inline(always)]
    fn from(val: u8) -> MierMb25 {
        MierMb25::from_bits(val)
    }
}
impl From<MierMb25> for u8 {
    #[inline(always)]
    fn from(val: MierMb25) -> u8 {
        MierMb25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb26 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb26 {
    #[inline(always)]
    fn from(val: u8) -> MierMb26 {
        MierMb26::from_bits(val)
    }
}
impl From<MierMb26> for u8 {
    #[inline(always)]
    fn from(val: MierMb26) -> u8 {
        MierMb26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb27 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb27 {
    #[inline(always)]
    fn from(val: u8) -> MierMb27 {
        MierMb27::from_bits(val)
    }
}
impl From<MierMb27> for u8 {
    #[inline(always)]
    fn from(val: MierMb27) -> u8 {
        MierMb27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb28 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb28 {
    #[inline(always)]
    fn from(val: u8) -> MierMb28 {
        MierMb28::from_bits(val)
    }
}
impl From<MierMb28> for u8 {
    #[inline(always)]
    fn from(val: MierMb28) -> u8 {
        MierMb28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb29 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb29 {
    #[inline(always)]
    fn from(val: u8) -> MierMb29 {
        MierMb29::from_bits(val)
    }
}
impl From<MierMb29> for u8 {
    #[inline(always)]
    fn from(val: MierMb29) -> u8 {
        MierMb29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb3 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb3 {
    #[inline(always)]
    fn from(val: u8) -> MierMb3 {
        MierMb3::from_bits(val)
    }
}
impl From<MierMb3> for u8 {
    #[inline(always)]
    fn from(val: MierMb3) -> u8 {
        MierMb3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb30 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb30 {
    #[inline(always)]
    fn from(val: u8) -> MierMb30 {
        MierMb30::from_bits(val)
    }
}
impl From<MierMb30> for u8 {
    #[inline(always)]
    fn from(val: MierMb30) -> u8 {
        MierMb30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb31 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb31 {
    #[inline(always)]
    fn from(val: u8) -> MierMb31 {
        MierMb31::from_bits(val)
    }
}
impl From<MierMb31> for u8 {
    #[inline(always)]
    fn from(val: MierMb31) -> u8 {
        MierMb31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb4 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb4 {
    #[inline(always)]
    fn from(val: u8) -> MierMb4 {
        MierMb4::from_bits(val)
    }
}
impl From<MierMb4> for u8 {
    #[inline(always)]
    fn from(val: MierMb4) -> u8 {
        MierMb4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb5 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb5 {
    #[inline(always)]
    fn from(val: u8) -> MierMb5 {
        MierMb5::from_bits(val)
    }
}
impl From<MierMb5> for u8 {
    #[inline(always)]
    fn from(val: MierMb5) -> u8 {
        MierMb5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb6 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb6 {
    #[inline(always)]
    fn from(val: u8) -> MierMb6 {
        MierMb6::from_bits(val)
    }
}
impl From<MierMb6> for u8 {
    #[inline(always)]
    fn from(val: MierMb6) -> u8 {
        MierMb6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb7 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb7 {
    #[inline(always)]
    fn from(val: u8) -> MierMb7 {
        MierMb7::from_bits(val)
    }
}
impl From<MierMb7> for u8 {
    #[inline(always)]
    fn from(val: MierMb7) -> u8 {
        MierMb7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb8 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb8 {
    #[inline(always)]
    fn from(val: u8) -> MierMb8 {
        MierMb8::from_bits(val)
    }
}
impl From<MierMb8> for u8 {
    #[inline(always)]
    fn from(val: MierMb8) -> u8 {
        MierMb8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierMb9 {
    #[doc = "Interrupt disabled"]
    _0 = 0x0,
    #[doc = "Interrupt enabled"]
    _1 = 0x01,
}
impl MierMb9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierMb9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierMb9 {
    #[inline(always)]
    fn from(val: u8) -> MierMb9 {
        MierMb9::from_bits(val)
    }
}
impl From<MierMb9> for u8 {
    #[inline(always)]
    fn from(val: MierMb9) -> u8 {
        MierMb9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb0 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb0 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb0 {
        MkivlrMb0::from_bits(val)
    }
}
impl From<MkivlrMb0> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb0) -> u8 {
        MkivlrMb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb1 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb1 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb1 {
        MkivlrMb1::from_bits(val)
    }
}
impl From<MkivlrMb1> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb1) -> u8 {
        MkivlrMb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb10 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb10 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb10 {
        MkivlrMb10::from_bits(val)
    }
}
impl From<MkivlrMb10> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb10) -> u8 {
        MkivlrMb10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb11 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb11 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb11 {
        MkivlrMb11::from_bits(val)
    }
}
impl From<MkivlrMb11> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb11) -> u8 {
        MkivlrMb11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb12 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb12 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb12 {
        MkivlrMb12::from_bits(val)
    }
}
impl From<MkivlrMb12> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb12) -> u8 {
        MkivlrMb12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb13 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb13 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb13 {
        MkivlrMb13::from_bits(val)
    }
}
impl From<MkivlrMb13> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb13) -> u8 {
        MkivlrMb13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb14 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb14 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb14 {
        MkivlrMb14::from_bits(val)
    }
}
impl From<MkivlrMb14> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb14) -> u8 {
        MkivlrMb14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb15 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb15 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb15 {
        MkivlrMb15::from_bits(val)
    }
}
impl From<MkivlrMb15> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb15) -> u8 {
        MkivlrMb15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb16 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb16 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb16 {
        MkivlrMb16::from_bits(val)
    }
}
impl From<MkivlrMb16> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb16) -> u8 {
        MkivlrMb16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb17 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb17 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb17 {
        MkivlrMb17::from_bits(val)
    }
}
impl From<MkivlrMb17> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb17) -> u8 {
        MkivlrMb17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb18 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb18 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb18 {
        MkivlrMb18::from_bits(val)
    }
}
impl From<MkivlrMb18> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb18) -> u8 {
        MkivlrMb18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb19 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb19 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb19 {
        MkivlrMb19::from_bits(val)
    }
}
impl From<MkivlrMb19> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb19) -> u8 {
        MkivlrMb19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb2 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb2 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb2 {
        MkivlrMb2::from_bits(val)
    }
}
impl From<MkivlrMb2> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb2) -> u8 {
        MkivlrMb2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb20 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb20 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb20 {
        MkivlrMb20::from_bits(val)
    }
}
impl From<MkivlrMb20> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb20) -> u8 {
        MkivlrMb20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb21 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb21 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb21 {
        MkivlrMb21::from_bits(val)
    }
}
impl From<MkivlrMb21> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb21) -> u8 {
        MkivlrMb21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb22 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb22 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb22 {
        MkivlrMb22::from_bits(val)
    }
}
impl From<MkivlrMb22> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb22) -> u8 {
        MkivlrMb22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb23 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb23 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb23 {
        MkivlrMb23::from_bits(val)
    }
}
impl From<MkivlrMb23> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb23) -> u8 {
        MkivlrMb23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb24 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb24 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb24 {
        MkivlrMb24::from_bits(val)
    }
}
impl From<MkivlrMb24> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb24) -> u8 {
        MkivlrMb24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb25 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb25 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb25 {
        MkivlrMb25::from_bits(val)
    }
}
impl From<MkivlrMb25> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb25) -> u8 {
        MkivlrMb25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb26 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb26 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb26 {
        MkivlrMb26::from_bits(val)
    }
}
impl From<MkivlrMb26> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb26) -> u8 {
        MkivlrMb26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb27 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb27 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb27 {
        MkivlrMb27::from_bits(val)
    }
}
impl From<MkivlrMb27> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb27) -> u8 {
        MkivlrMb27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb28 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb28 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb28 {
        MkivlrMb28::from_bits(val)
    }
}
impl From<MkivlrMb28> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb28) -> u8 {
        MkivlrMb28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb29 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb29 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb29 {
        MkivlrMb29::from_bits(val)
    }
}
impl From<MkivlrMb29> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb29) -> u8 {
        MkivlrMb29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb3 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb3 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb3 {
        MkivlrMb3::from_bits(val)
    }
}
impl From<MkivlrMb3> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb3) -> u8 {
        MkivlrMb3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb30 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb30 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb30 {
        MkivlrMb30::from_bits(val)
    }
}
impl From<MkivlrMb30> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb30) -> u8 {
        MkivlrMb30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb31 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb31 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb31 {
        MkivlrMb31::from_bits(val)
    }
}
impl From<MkivlrMb31> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb31) -> u8 {
        MkivlrMb31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb4 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb4 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb4 {
        MkivlrMb4::from_bits(val)
    }
}
impl From<MkivlrMb4> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb4) -> u8 {
        MkivlrMb4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb5 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb5 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb5 {
        MkivlrMb5::from_bits(val)
    }
}
impl From<MkivlrMb5> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb5) -> u8 {
        MkivlrMb5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb6 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb6 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb6 {
        MkivlrMb6::from_bits(val)
    }
}
impl From<MkivlrMb6> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb6) -> u8 {
        MkivlrMb6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb7 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb7 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb7 {
        MkivlrMb7::from_bits(val)
    }
}
impl From<MkivlrMb7> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb7) -> u8 {
        MkivlrMb7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb8 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb8 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb8 {
        MkivlrMb8::from_bits(val)
    }
}
impl From<MkivlrMb8> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb8) -> u8 {
        MkivlrMb8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MkivlrMb9 {
    #[doc = "Mask valid"]
    _0 = 0x0,
    #[doc = "Mask invalid"]
    _1 = 0x01,
}
impl MkivlrMb9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MkivlrMb9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MkivlrMb9 {
    #[inline(always)]
    fn from(val: u8) -> MkivlrMb9 {
        MkivlrMb9::from_bits(val)
    }
}
impl From<MkivlrMb9> for u8 {
    #[inline(always)]
    fn from(val: MkivlrMb9) -> u8 {
        MkivlrMb9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mlm {
    #[doc = "Overwrite mode"]
    _0 = 0x0,
    #[doc = "Overrun mode"]
    _1 = 0x01,
}
impl Mlm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mlm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mlm {
    #[inline(always)]
    fn from(val: u8) -> Mlm {
        Mlm::from_bits(val)
    }
}
impl From<Mlm> for u8 {
    #[inline(always)]
    fn from(val: Mlm) -> u8 {
        Mlm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Msglost {
    #[doc = "Message is not overwritten or overrun"]
    _0 = 0x0,
    #[doc = "Message is overwritten or overrun"]
    _1 = 0x01,
}
impl Msglost {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msglost {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msglost {
    #[inline(always)]
    fn from(val: u8) -> Msglost {
        Msglost::from_bits(val)
    }
}
impl From<Msglost> for u8 {
    #[inline(always)]
    fn from(val: Msglost) -> u8 {
        Msglost::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndst {
    #[doc = "No mailbox with NEWDATA bit = 1"]
    _0 = 0x0,
    #[doc = "Mailbox(es) with NEWDATA bit = 1"]
    _1 = 0x01,
}
impl Ndst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndst {
    #[inline(always)]
    fn from(val: u8) -> Ndst {
        Ndst::from_bits(val)
    }
}
impl From<Ndst> for u8 {
    #[inline(always)]
    fn from(val: Ndst) -> u8 {
        Ndst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Newdata {
    #[doc = "No data has been received or 0 is written to the NEWDATA bit"]
    _0 = 0x0,
    #[doc = "A new message is being stored or has been stored to the mailbox"]
    _1 = 0x01,
}
impl Newdata {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Newdata {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Newdata {
    #[inline(always)]
    fn from(val: u8) -> Newdata {
        Newdata::from_bits(val)
    }
}
impl From<Newdata> for u8 {
    #[inline(always)]
    fn from(val: Newdata) -> u8 {
        Newdata::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nmlst {
    #[doc = "No mailbox with MSGLOST bit = 1"]
    _0 = 0x0,
    #[doc = "Mailbox(es) with MSGLOST bit = 1"]
    _1 = 0x01,
}
impl Nmlst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nmlst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nmlst {
    #[inline(always)]
    fn from(val: u8) -> Nmlst {
        Nmlst::from_bits(val)
    }
}
impl From<Nmlst> for u8 {
    #[inline(always)]
    fn from(val: Nmlst) -> u8 {
        Nmlst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Olie {
    #[doc = "Overload frame transmit interrupt disabled"]
    _0 = 0x0,
    #[doc = "Overload frame transmit interrupt enabled"]
    _1 = 0x01,
}
impl Olie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Olie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Olie {
    #[inline(always)]
    fn from(val: u8) -> Olie {
        Olie::from_bits(val)
    }
}
impl From<Olie> for u8 {
    #[inline(always)]
    fn from(val: Olie) -> u8 {
        Olie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Olif {
    #[doc = "No overload frame transmission detected"]
    _0 = 0x0,
    #[doc = "Overload frame transmission detected"]
    _1 = 0x01,
}
impl Olif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Olif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Olif {
    #[inline(always)]
    fn from(val: u8) -> Olif {
        Olif::from_bits(val)
    }
}
impl From<Olif> for u8 {
    #[inline(always)]
    fn from(val: Olif) -> u8 {
        Olif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Orie {
    #[doc = "Receive overrun interrupt disabled"]
    _0 = 0x0,
    #[doc = "Receive overrun interrupt enabled"]
    _1 = 0x01,
}
impl Orie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Orie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Orie {
    #[inline(always)]
    fn from(val: u8) -> Orie {
        Orie::from_bits(val)
    }
}
impl From<Orie> for u8 {
    #[inline(always)]
    fn from(val: Orie) -> u8 {
        Orie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Orif {
    #[doc = "No receive overrun detected"]
    _0 = 0x0,
    #[doc = "Receive overrun detected"]
    _1 = 0x01,
}
impl Orif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Orif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Orif {
    #[inline(always)]
    fn from(val: u8) -> Orif {
        Orif::from_bits(val)
    }
}
impl From<Orif> for u8 {
    #[inline(always)]
    fn from(val: Orif) -> u8 {
        Orif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rboc {
    #[doc = "Nothing occurred"]
    _0 = 0x0,
    #[doc = "Forcible return from bus-off"]
    _1 = 0x01,
}
impl Rboc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rboc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rboc {
    #[inline(always)]
    fn from(val: u8) -> Rboc {
        Rboc::from_bits(val)
    }
}
impl From<Rboc> for u8 {
    #[inline(always)]
    fn from(val: Rboc) -> u8 {
        Rboc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Recst {
    #[doc = "Bus idle or transmission in progress"]
    _0 = 0x0,
    #[doc = "Reception in progress"]
    _1 = 0x01,
}
impl Recst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Recst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Recst {
    #[inline(always)]
    fn from(val: u8) -> Recst {
        Recst::from_bits(val)
    }
}
impl From<Recst> for u8 {
    #[inline(always)]
    fn from(val: Recst) -> u8 {
        Recst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfe {
    #[doc = "Receive FIFO disabled"]
    _0 = 0x0,
    #[doc = "Receive FIFO enabled"]
    _1 = 0x01,
}
impl Rfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfe {
    #[inline(always)]
    fn from(val: u8) -> Rfe {
        Rfe::from_bits(val)
    }
}
impl From<Rfe> for u8 {
    #[inline(always)]
    fn from(val: Rfe) -> u8 {
        Rfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfest {
    #[doc = "Unread message in receive FIFO"]
    _0 = 0x0,
    #[doc = "No unread message in receive FIFO"]
    _1 = 0x01,
}
impl Rfest {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfest {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfest {
    #[inline(always)]
    fn from(val: u8) -> Rfest {
        Rfest::from_bits(val)
    }
}
impl From<Rfest> for u8 {
    #[inline(always)]
    fn from(val: Rfest) -> u8 {
        Rfest::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rffst {
    #[doc = "Receive FIFO is not full"]
    _0 = 0x0,
    #[doc = "Receive FIFO is full (4 unread messages)"]
    _1 = 0x01,
}
impl Rffst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rffst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rffst {
    #[inline(always)]
    fn from(val: u8) -> Rffst {
        Rffst::from_bits(val)
    }
}
impl From<Rffst> for u8 {
    #[inline(always)]
    fn from(val: Rffst) -> u8 {
        Rffst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfmlf {
    #[doc = "No receive FIFO message lost has occurred"]
    _0 = 0x0,
    #[doc = "Receive FIFO message lost has occurred"]
    _1 = 0x01,
}
impl Rfmlf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfmlf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfmlf {
    #[inline(always)]
    fn from(val: u8) -> Rfmlf {
        Rfmlf::from_bits(val)
    }
}
impl From<Rfmlf> for u8 {
    #[inline(always)]
    fn from(val: Rfmlf) -> u8 {
        Rfmlf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfst {
    #[doc = "No message in receive FIFO (empty)"]
    _0 = 0x0,
    #[doc = "Message in receive FIFO"]
    _1 = 0x01,
}
impl Rfst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfst {
    #[inline(always)]
    fn from(val: u8) -> Rfst {
        Rfst::from_bits(val)
    }
}
impl From<Rfst> for u8 {
    #[inline(always)]
    fn from(val: Rfst) -> u8 {
        Rfst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfust {
    #[doc = "No unread message"]
    _000 = 0x0,
    #[doc = "1 unread message"]
    _001 = 0x01,
    #[doc = "2 unread messages"]
    _010 = 0x02,
    #[doc = "3 unread messages"]
    _011 = 0x03,
    #[doc = "4 unread messages"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Rfust {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfust {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfust {
    #[inline(always)]
    fn from(val: u8) -> Rfust {
        Rfust::from_bits(val)
    }
}
impl From<Rfust> for u8 {
    #[inline(always)]
    fn from(val: Rfust) -> u8 {
        Rfust::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfwst {
    #[doc = "Receive FIFO is not buffer warning"]
    _0 = 0x0,
    #[doc = "Receive FIFO is buffer warning (3 unread messages)"]
    _1 = 0x01,
}
impl Rfwst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfwst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfwst {
    #[inline(always)]
    fn from(val: u8) -> Rfwst {
        Rfwst::from_bits(val)
    }
}
impl From<Rfwst> for u8 {
    #[inline(always)]
    fn from(val: Rfwst) -> u8 {
        Rfwst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstst {
    #[doc = "Not in CAN reset mode"]
    _0 = 0x0,
    #[doc = "In CAN reset mode"]
    _1 = 0x01,
}
impl Rstst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstst {
    #[inline(always)]
    fn from(val: u8) -> Rstst {
        Rstst::from_bits(val)
    }
}
impl From<Rstst> for u8 {
    #[inline(always)]
    fn from(val: Rstst) -> u8 {
        Rstst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdst {
    #[doc = "No mailbox with SENTDATA bit = 1"]
    _0 = 0x0,
    #[doc = "Mailbox(es) with SENTDATA bit = 1"]
    _1 = 0x01,
}
impl Sdst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdst {
    #[inline(always)]
    fn from(val: u8) -> Sdst {
        Sdst::from_bits(val)
    }
}
impl From<Sdst> for u8 {
    #[inline(always)]
    fn from(val: Sdst) -> u8 {
        Sdst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sef {
    #[doc = "No stuff error detected"]
    _0 = 0x0,
    #[doc = "Stuff error detected"]
    _1 = 0x01,
}
impl Sef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sef {
    #[inline(always)]
    fn from(val: u8) -> Sef {
        Sef::from_bits(val)
    }
}
impl From<Sef> for u8 {
    #[inline(always)]
    fn from(val: Sef) -> u8 {
        Sef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sentdata {
    #[doc = "Transmission is not completed"]
    _0 = 0x0,
    #[doc = "Transmission is completed"]
    _1 = 0x01,
}
impl Sentdata {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sentdata {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sentdata {
    #[inline(always)]
    fn from(val: u8) -> Sentdata {
        Sentdata::from_bits(val)
    }
}
impl From<Sentdata> for u8 {
    #[inline(always)]
    fn from(val: Sentdata) -> u8 {
        Sentdata::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sest {
    #[doc = "Search result found"]
    _0 = 0x0,
    #[doc = "No search result"]
    _1 = 0x01,
}
impl Sest {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sest {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sest {
    #[inline(always)]
    fn from(val: u8) -> Sest {
        Sest::from_bits(val)
    }
}
impl From<Sest> for u8 {
    #[inline(always)]
    fn from(val: Sest) -> u8 {
        Sest::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sjw {
    #[doc = "1 Tq"]
    _00 = 0x0,
    #[doc = "2 Tq"]
    _01 = 0x01,
    #[doc = "3 Tq"]
    _10 = 0x02,
    #[doc = "4 Tq"]
    _11 = 0x03,
}
impl Sjw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sjw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sjw {
    #[inline(always)]
    fn from(val: u8) -> Sjw {
        Sjw::from_bits(val)
    }
}
impl From<Sjw> for u8 {
    #[inline(always)]
    fn from(val: Sjw) -> u8 {
        Sjw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slpm {
    #[doc = "Other than CAN sleep mode"]
    _0 = 0x0,
    #[doc = "CAN sleep mode"]
    _1 = 0x01,
}
impl Slpm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slpm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slpm {
    #[inline(always)]
    fn from(val: u8) -> Slpm {
        Slpm::from_bits(val)
    }
}
impl From<Slpm> for u8 {
    #[inline(always)]
    fn from(val: Slpm) -> u8 {
        Slpm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slpst {
    #[doc = "Not in CAN sleep mode"]
    _0 = 0x0,
    #[doc = "In CAN sleep mode"]
    _1 = 0x01,
}
impl Slpst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slpst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slpst {
    #[inline(always)]
    fn from(val: u8) -> Slpst {
        Slpst::from_bits(val)
    }
}
impl From<Slpst> for u8 {
    #[inline(always)]
    fn from(val: Slpst) -> u8 {
        Slpst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tabst {
    #[doc = "No mailbox with TRMABT bit = 1"]
    _0 = 0x0,
    #[doc = "Mailbox(es) with TRMABT bit = 1"]
    _1 = 0x01,
}
impl Tabst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tabst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tabst {
    #[inline(always)]
    fn from(val: u8) -> Tabst {
        Tabst::from_bits(val)
    }
}
impl From<Tabst> for u8 {
    #[inline(always)]
    fn from(val: Tabst) -> u8 {
        Tabst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfe {
    #[doc = "Transmit FIFO disabled"]
    _0 = 0x0,
    #[doc = "Transmit FIFO enabled"]
    _1 = 0x01,
}
impl Tfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfe {
    #[inline(always)]
    fn from(val: u8) -> Tfe {
        Tfe::from_bits(val)
    }
}
impl From<Tfe> for u8 {
    #[inline(always)]
    fn from(val: Tfe) -> u8 {
        Tfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfest {
    #[doc = "Unsent message in transmit FIFO"]
    _0 = 0x0,
    #[doc = "No unsent message in transmit FIFO"]
    _1 = 0x01,
}
impl Tfest {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfest {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfest {
    #[inline(always)]
    fn from(val: u8) -> Tfest {
        Tfest::from_bits(val)
    }
}
impl From<Tfest> for u8 {
    #[inline(always)]
    fn from(val: Tfest) -> u8 {
        Tfest::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tffst {
    #[doc = "Transmit FIFO is not full"]
    _0 = 0x0,
    #[doc = "Transmit FIFO is full (4 unsent messages)"]
    _1 = 0x01,
}
impl Tffst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tffst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tffst {
    #[inline(always)]
    fn from(val: u8) -> Tffst {
        Tffst::from_bits(val)
    }
}
impl From<Tffst> for u8 {
    #[inline(always)]
    fn from(val: Tffst) -> u8 {
        Tffst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfst {
    #[doc = "Transmit FIFO is full"]
    _0 = 0x0,
    #[doc = "Transmit FIFO is not full"]
    _1 = 0x01,
}
impl Tfst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfst {
    #[inline(always)]
    fn from(val: u8) -> Tfst {
        Tfst::from_bits(val)
    }
}
impl From<Tfst> for u8 {
    #[inline(always)]
    fn from(val: Tfst) -> u8 {
        Tfst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfust {
    #[doc = "No unsent message"]
    _000 = 0x0,
    #[doc = "1 unsent message"]
    _001 = 0x01,
    #[doc = "2 unsent messages"]
    _010 = 0x02,
    #[doc = "3 unsent messages"]
    _011 = 0x03,
    #[doc = "4 unsent messages"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tfust {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfust {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfust {
    #[inline(always)]
    fn from(val: u8) -> Tfust {
        Tfust::from_bits(val)
    }
}
impl From<Tfust> for u8 {
    #[inline(always)]
    fn from(val: Tfust) -> u8 {
        Tfust::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpm {
    #[doc = "ID priority transmit mode"]
    _0 = 0x0,
    #[doc = "Mailbox number priority transmit mode"]
    _1 = 0x01,
}
impl Tpm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpm {
    #[inline(always)]
    fn from(val: u8) -> Tpm {
        Tpm::from_bits(val)
    }
}
impl From<Tpm> for u8 {
    #[inline(always)]
    fn from(val: Tpm) -> u8 {
        Tpm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trmabt {
    #[doc = "Transmission has started, transmission abort failed because transmission is completed, or transmission abort is not requested"]
    _0 = 0x0,
    #[doc = "Transmission abort is completed"]
    _1 = 0x01,
}
impl Trmabt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trmabt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trmabt {
    #[inline(always)]
    fn from(val: u8) -> Trmabt {
        Trmabt::from_bits(val)
    }
}
impl From<Trmabt> for u8 {
    #[inline(always)]
    fn from(val: Trmabt) -> u8 {
        Trmabt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trmactive {
    #[doc = "Transmission is pending or transmission is not requested"]
    _0 = 0x0,
    #[doc = "From acceptance of transmission request to completion of transmission, or error/arbitration-lost"]
    _1 = 0x01,
}
impl Trmactive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trmactive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trmactive {
    #[inline(always)]
    fn from(val: u8) -> Trmactive {
        Trmactive::from_bits(val)
    }
}
impl From<Trmactive> for u8 {
    #[inline(always)]
    fn from(val: Trmactive) -> u8 {
        Trmactive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trmst {
    #[doc = "Bus idle or reception in progress"]
    _0 = 0x0,
    #[doc = "Transmission in progress or in bus-off state"]
    _1 = 0x01,
}
impl Trmst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trmst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trmst {
    #[inline(always)]
    fn from(val: u8) -> Trmst {
        Trmst::from_bits(val)
    }
}
impl From<Trmst> for u8 {
    #[inline(always)]
    fn from(val: Trmst) -> u8 {
        Trmst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tseg1 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "4 Tq"]
    _0011 = 0x03,
    #[doc = "5 Tq"]
    _0100 = 0x04,
    #[doc = "6 Tq"]
    _0101 = 0x05,
    #[doc = "7 Tq"]
    _0110 = 0x06,
    #[doc = "8 Tq"]
    _0111 = 0x07,
    #[doc = "9 Tq"]
    _1000 = 0x08,
    #[doc = "10 Tq"]
    _1001 = 0x09,
    #[doc = "11 Tq"]
    _1010 = 0x0a,
    #[doc = "12 Tq"]
    _1011 = 0x0b,
    #[doc = "13 Tq"]
    _1100 = 0x0c,
    #[doc = "14 Tq"]
    _1101 = 0x0d,
    #[doc = "15 Tq"]
    _1110 = 0x0e,
    #[doc = "16 Tq"]
    _1111 = 0x0f,
}
impl Tseg1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tseg1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tseg1 {
    #[inline(always)]
    fn from(val: u8) -> Tseg1 {
        Tseg1::from_bits(val)
    }
}
impl From<Tseg1> for u8 {
    #[inline(always)]
    fn from(val: Tseg1) -> u8 {
        Tseg1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tseg2 {
    #[doc = "Setting prohibited"]
    _000 = 0x0,
    #[doc = "2 Tq"]
    _001 = 0x01,
    #[doc = "3 Tq"]
    _010 = 0x02,
    #[doc = "4 Tq"]
    _011 = 0x03,
    #[doc = "5 Tq"]
    _100 = 0x04,
    #[doc = "6 Tq"]
    _101 = 0x05,
    #[doc = "7 Tq"]
    _110 = 0x06,
    #[doc = "8 Tq"]
    _111 = 0x07,
}
impl Tseg2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tseg2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tseg2 {
    #[inline(always)]
    fn from(val: u8) -> Tseg2 {
        Tseg2::from_bits(val)
    }
}
impl From<Tseg2> for u8 {
    #[inline(always)]
    fn from(val: Tseg2) -> u8 {
        Tseg2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsps {
    #[doc = "Every bit time"]
    _00 = 0x0,
    #[doc = "Every 2-bit time"]
    _01 = 0x01,
    #[doc = "Every 4-bit time"]
    _10 = 0x02,
    #[doc = "Every 8-bit time"]
    _11 = 0x03,
}
impl Tsps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsps {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsps {
    #[inline(always)]
    fn from(val: u8) -> Tsps {
        Tsps::from_bits(val)
    }
}
impl From<Tsps> for u8 {
    #[inline(always)]
    fn from(val: Tsps) -> u8 {
        Tsps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsrc {
    #[doc = "Nothing occurred"]
    _0 = 0x0,
    #[doc = "Reset"]
    _1 = 0x01,
}
impl Tsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsrc {
    #[inline(always)]
    fn from(val: u8) -> Tsrc {
        Tsrc::from_bits(val)
    }
}
impl From<Tsrc> for u8 {
    #[inline(always)]
    fn from(val: Tsrc) -> u8 {
        Tsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tste {
    #[doc = "CAN test mode disabled"]
    _0 = 0x0,
    #[doc = "CAN test mode enabled"]
    _1 = 0x01,
}
impl Tste {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tste {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tste {
    #[inline(always)]
    fn from(val: u8) -> Tste {
        Tste::from_bits(val)
    }
}
impl From<Tste> for u8 {
    #[inline(always)]
    fn from(val: Tste) -> u8 {
        Tste::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstm {
    #[doc = "Other than CAN test mode"]
    _00 = 0x0,
    #[doc = "Listen-only mode"]
    _01 = 0x01,
    #[doc = "Self-test mode 0 (external loopback)"]
    _10 = 0x02,
    #[doc = "Self-test mode 1 (internal loopback)"]
    _11 = 0x03,
}
impl Tstm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstm {
    #[inline(always)]
    fn from(val: u8) -> Tstm {
        Tstm::from_bits(val)
    }
}
impl From<Tstm> for u8 {
    #[inline(always)]
    fn from(val: Tstm) -> u8 {
        Tstm::to_bits(val)
    }
}
