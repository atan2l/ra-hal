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
    Little = 0x0,
    Big = 0x01,
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
pub enum Cpha {
    #[doc = "Data sampling on odd edge, data variation on even edge"]
    SampleShift = 0x0,
    #[doc = "Data variation on odd edge, data sampling on even edge"]
    ShiftSample = 0x01,
}
impl Cpha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpha {
    #[inline(always)]
    fn from(val: u8) -> Cpha {
        Cpha::from_bits(val)
    }
}
impl From<Cpha> for u8 {
    #[inline(always)]
    fn from(val: Cpha) -> u8 {
        Cpha::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpol {
    #[doc = "RSPCK is low when idle"]
    Low = 0x0,
    #[doc = "RSPCK is high when idle"]
    High = 0x01,
}
impl Cpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpol {
    #[inline(always)]
    fn from(val: u8) -> Cpol {
        Cpol::from_bits(val)
    }
}
impl From<Cpol> for u8 {
    #[inline(always)]
    fn from(val: Cpol) -> u8 {
        Cpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lsbf {
    #[doc = "MSB first"]
    Msb0 = 0x0,
    #[doc = "LSB first"]
    Lsb0 = 0x01,
}
impl Lsbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsbf {
    #[inline(always)]
    fn from(val: u8) -> Lsbf {
        Lsbf::from_bits(val)
    }
}
impl From<Lsbf> for u8 {
    #[inline(always)]
    fn from(val: Lsbf) -> u8 {
        Lsbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Moifv {
    #[doc = "The level output on the MOSIn pin during MOSI idling corresponds to low."]
    Low = 0x0,
    #[doc = "The level output on the MOSIn pin during MOSI idling corresponds to high."]
    High = 0x01,
}
impl Moifv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Moifv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Moifv {
    #[inline(always)]
    fn from(val: u8) -> Moifv {
        Moifv::from_bits(val)
    }
}
impl From<Moifv> for u8 {
    #[inline(always)]
    fn from(val: Moifv) -> u8 {
        Moifv::to_bits(val)
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
    _20bits = 0x0,
    #[doc = "24 bits"]
    _24bits = 0x01,
    #[doc = "32 bits"]
    _32bits = 0x02,
    #[doc = "32 bits"]
    _RESERVED = 0x03,
    #[doc = "8 bits"]
    _8bits = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "9 bits"]
    _9bits = 0x08,
    #[doc = "10 bits"]
    _10bits = 0x09,
    #[doc = "11 bits"]
    _11bits = 0x0a,
    #[doc = "12 bits"]
    _12bits = 0x0b,
    #[doc = "13 bits"]
    _13bits = 0x0c,
    #[doc = "14 bits"]
    _14bits = 0x0d,
    #[doc = "15 bits"]
    _15bits = 0x0e,
    #[doc = "16 bits"]
    _16bits = 0x0f,
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
pub enum Spbyt {
    #[doc = "SPDR is accessed in halfword or word (SPLW is valid)."]
    Word = 0x0,
    #[doc = "SPDR is accessed in byte (SPLW is invalid)."]
    Byte = 0x01,
}
impl Spbyt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spbyt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spbyt {
    #[inline(always)]
    fn from(val: u8) -> Spbyt {
        Spbyt::from_bits(val)
    }
}
impl From<Spbyt> for u8 {
    #[inline(always)]
    fn from(val: Spbyt) -> u8 {
        Spbyt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Splw {
    #[doc = "SPDR_HA is valid to access in halfwords"]
    HalfWord = 0x0,
    #[doc = "SPDR is valid (to access in words)."]
    Word = 0x01,
}
impl Splw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Splw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Splw {
    #[inline(always)]
    fn from(val: u8) -> Splw {
        Splw::from_bits(val)
    }
}
impl From<Splw> for u8 {
    #[inline(always)]
    fn from(val: Splw) -> u8 {
        Splw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spms {
    #[doc = "SPI operation (4-wire method)"]
    Spi = 0x0,
    #[doc = "Clock synchronous operation (3-wire method)"]
    ClockSynchronous = 0x01,
}
impl Spms {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spms {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spms {
    #[inline(always)]
    fn from(val: u8) -> Spms {
        Spms::from_bits(val)
    }
}
impl From<Spms> for u8 {
    #[inline(always)]
    fn from(val: Spms) -> u8 {
        Spms::to_bits(val)
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
pub enum Sprdtd {
    #[doc = "SPDR values are read from the receive buffer"]
    RxBuf = 0x0,
    #[doc = "SPDR values are read from the transmit buffer (but only if the transmit buffer is empty)"]
    TxBuf = 0x01,
}
impl Sprdtd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sprdtd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sprdtd {
    #[inline(always)]
    fn from(val: u8) -> Sprdtd {
        Sprdtd::from_bits(val)
    }
}
impl From<Sprdtd> for u8 {
    #[inline(always)]
    fn from(val: Sprdtd) -> u8 {
        Sprdtd::to_bits(val)
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
