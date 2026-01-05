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
pub enum Cpha {
    #[doc = "Data sampling on odd edge, data variation on even edge"]
    _0 = 0x0,
    #[doc = "Data variation on odd edge, data sampling on even edge"]
    _1 = 0x01,
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
    _0 = 0x0,
    #[doc = "RSPCK is high when idle"]
    _1 = 0x01,
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
pub enum Idlnf {
    #[doc = "SPI is in the idle state"]
    _0 = 0x0,
    #[doc = "SPI is in the transfer state"]
    _1 = 0x01,
}
impl Idlnf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idlnf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idlnf {
    #[inline(always)]
    fn from(val: u8) -> Idlnf {
        Idlnf::from_bits(val)
    }
}
impl From<Idlnf> for u8 {
    #[inline(always)]
    fn from(val: Idlnf) -> u8 {
        Idlnf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lsbf {
    #[doc = "MSB first"]
    _0 = 0x0,
    #[doc = "LSB first"]
    _1 = 0x01,
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
pub enum Modf {
    #[doc = "Neither mode fault error nor underrun error occurs"]
    _0 = 0x0,
    #[doc = "A mode fault error or an underrun error occurs."]
    _1 = 0x01,
}
impl Modf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Modf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Modf {
    #[inline(always)]
    fn from(val: u8) -> Modf {
        Modf::from_bits(val)
    }
}
impl From<Modf> for u8 {
    #[inline(always)]
    fn from(val: Modf) -> u8 {
        Modf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Modfen {
    #[doc = "Disables the detection of mode fault error"]
    _0 = 0x0,
    #[doc = "Enables the detection of mode fault error"]
    _1 = 0x01,
}
impl Modfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Modfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Modfen {
    #[inline(always)]
    fn from(val: u8) -> Modfen {
        Modfen::from_bits(val)
    }
}
impl From<Modfen> for u8 {
    #[inline(always)]
    fn from(val: Modfen) -> u8 {
        Modfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Moife {
    #[doc = "MOSI output value equals final data from previous transfer"]
    _0 = 0x0,
    #[doc = "MOSI output value equals the value set in the MOIFV bit"]
    _1 = 0x01,
}
impl Moife {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Moife {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Moife {
    #[inline(always)]
    fn from(val: u8) -> Moife {
        Moife::from_bits(val)
    }
}
impl From<Moife> for u8 {
    #[inline(always)]
    fn from(val: Moife) -> u8 {
        Moife::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Moifv {
    #[doc = "The level output on the MOSIn pin during MOSI idling corresponds to low."]
    _0 = 0x0,
    #[doc = "The level output on the MOSIn pin during MOSI idling corresponds to high."]
    _1 = 0x01,
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
pub enum Mstr {
    #[doc = "Slave mode"]
    _0 = 0x0,
    #[doc = "Master mode"]
    _1 = 0x01,
}
impl Mstr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstr {
    #[inline(always)]
    fn from(val: u8) -> Mstr {
        Mstr::from_bits(val)
    }
}
impl From<Mstr> for u8 {
    #[inline(always)]
    fn from(val: Mstr) -> u8 {
        Mstr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ovrf {
    #[doc = "No overrun error occurs"]
    _0 = 0x0,
    #[doc = "An overrun error occurs"]
    _1 = 0x01,
}
impl Ovrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ovrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ovrf {
    #[inline(always)]
    fn from(val: u8) -> Ovrf {
        Ovrf::from_bits(val)
    }
}
impl From<Ovrf> for u8 {
    #[inline(always)]
    fn from(val: Ovrf) -> u8 {
        Ovrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Perf {
    #[doc = "No parity error occurs"]
    _0 = 0x0,
    #[doc = "A parity error occurs"]
    _1 = 0x01,
}
impl Perf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Perf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Perf {
    #[inline(always)]
    fn from(val: u8) -> Perf {
        Perf::from_bits(val)
    }
}
impl From<Perf> for u8 {
    #[inline(always)]
    fn from(val: Perf) -> u8 {
        Perf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pte {
    #[doc = "Disables the self-diagnosis function of the parity circuit"]
    _0 = 0x0,
    #[doc = "Enables the self-diagnosis function of the parity circuit"]
    _1 = 0x01,
}
impl Pte {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pte {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pte {
    #[inline(always)]
    fn from(val: u8) -> Pte {
        Pte::from_bits(val)
    }
}
impl From<Pte> for u8 {
    #[inline(always)]
    fn from(val: Pte) -> u8 {
        Pte::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckase {
    #[doc = "Disables the RSPCK auto-stop function"]
    _0 = 0x0,
    #[doc = "Enables the RSPCK auto-stop function"]
    _1 = 0x01,
}
impl Sckase {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckase {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckase {
    #[inline(always)]
    fn from(val: u8) -> Sckase {
        Sckase::from_bits(val)
    }
}
impl From<Sckase> for u8 {
    #[inline(always)]
    fn from(val: Sckase) -> u8 {
        Sckase::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckden {
    #[doc = "An RSPCK delay of 1 RSPCK"]
    _0 = 0x0,
    #[doc = "An RSPCK delay is equal to the setting of the RSPI clock delay register (SPCKD)"]
    _1 = 0x01,
}
impl Sckden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckden {
    #[inline(always)]
    fn from(val: u8) -> Sckden {
        Sckden::from_bits(val)
    }
}
impl From<Sckden> for u8 {
    #[inline(always)]
    fn from(val: Sckden) -> u8 {
        Sckden::to_bits(val)
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
pub enum Slnden {
    #[doc = "An SSL negation delay of 1 RSPCK"]
    _0 = 0x0,
    #[doc = "An SSL negation delay is equal to the setting of the RSPI slave select negation delay register (SSLND)"]
    _1 = 0x01,
}
impl Slnden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slnden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slnden {
    #[inline(always)]
    fn from(val: u8) -> Slnden {
        Slnden::from_bits(val)
    }
}
impl From<Slnden> for u8 {
    #[inline(always)]
    fn from(val: Slnden) -> u8 {
        Slnden::to_bits(val)
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
pub enum Spe {
    #[doc = "Disables the SPI function"]
    _0 = 0x0,
    #[doc = "Enables the SPI function"]
    _1 = 0x01,
}
impl Spe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spe {
    #[inline(always)]
    fn from(val: u8) -> Spe {
        Spe::from_bits(val)
    }
}
impl From<Spe> for u8 {
    #[inline(always)]
    fn from(val: Spe) -> u8 {
        Spe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Speie {
    #[doc = "Disables the generation of SPI error interrupt requests"]
    _0 = 0x0,
    #[doc = "Enables the generation of SPI error interrupt requests"]
    _1 = 0x01,
}
impl Speie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Speie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Speie {
    #[inline(always)]
    fn from(val: u8) -> Speie {
        Speie::from_bits(val)
    }
}
impl From<Speie> for u8 {
    #[inline(always)]
    fn from(val: Speie) -> u8 {
        Speie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spiie {
    #[doc = "Disables the generation of idle interrupt requests"]
    _0 = 0x0,
    #[doc = "Enables the generation of idle interrupt requests"]
    _1 = 0x01,
}
impl Spiie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spiie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spiie {
    #[inline(always)]
    fn from(val: u8) -> Spiie {
        Spiie::from_bits(val)
    }
}
impl From<Spiie> for u8 {
    #[inline(always)]
    fn from(val: Spiie) -> u8 {
        Spiie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Splp {
    #[doc = "Normal mode"]
    _0 = 0x0,
    #[doc = "Loopback mode (data is inverted for transmission)"]
    _1 = 0x01,
}
impl Splp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Splp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Splp {
    #[inline(always)]
    fn from(val: u8) -> Splp {
        Splp::from_bits(val)
    }
}
impl From<Splp> for u8 {
    #[inline(always)]
    fn from(val: Splp) -> u8 {
        Splp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Splp2 {
    #[doc = "Normal mode"]
    _0 = 0x0,
    #[doc = "Loopback mode (data is not inverted for transmission)"]
    _1 = 0x01,
}
impl Splp2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Splp2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Splp2 {
    #[inline(always)]
    fn from(val: u8) -> Splp2 {
        Splp2::from_bits(val)
    }
}
impl From<Splp2> for u8 {
    #[inline(always)]
    fn from(val: Splp2) -> u8 {
        Splp2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Splw {
    #[doc = "SPDR_HA is valid to access in halfwords"]
    _0 = 0x0,
    #[doc = "SPDR is valid (to access in words)."]
    _1 = 0x01,
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
    _0 = 0x0,
    #[doc = "Clock synchronous operation (3-wire method)"]
    _1 = 0x01,
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
pub enum Spnden {
    #[doc = "A next-access delay of 1 RSPCK + 2 PCLK"]
    _0 = 0x0,
    #[doc = "A next-access delay is equal to the setting of the RSPI next-access delay register (SPND)"]
    _1 = 0x01,
}
impl Spnden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spnden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spnden {
    #[inline(always)]
    fn from(val: u8) -> Spnden {
        Spnden::from_bits(val)
    }
}
impl From<Spnden> for u8 {
    #[inline(always)]
    fn from(val: Spnden) -> u8 {
        Spnden::to_bits(val)
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
pub enum Spoe {
    #[doc = "Selects even parity for use in transmission and reception"]
    _0 = 0x0,
    #[doc = "Selects odd parity for use in transmission and reception"]
    _1 = 0x01,
}
impl Spoe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spoe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spoe {
    #[inline(always)]
    fn from(val: u8) -> Spoe {
        Spoe::from_bits(val)
    }
}
impl From<Spoe> for u8 {
    #[inline(always)]
    fn from(val: Spoe) -> u8 {
        Spoe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sppe {
    #[doc = "Does not add the parity bit to transmit data and does not check the parity bit of receive data"]
    _0 = 0x0,
    #[doc = "Adds the parity bit to transmit data and checks the parity bit of receive data (when SPCR.TXMD = 0) / Adds the parity bit to transmit data but does not check the parity bit of receive data (when SPCR.TXMD = 1)"]
    _1 = 0x01,
}
impl Sppe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sppe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sppe {
    #[inline(always)]
    fn from(val: u8) -> Sppe {
        Sppe::from_bits(val)
    }
}
impl From<Sppe> for u8 {
    #[inline(always)]
    fn from(val: Sppe) -> u8 {
        Sppe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sprdtd {
    #[doc = "SPDR values are read from the receive buffer"]
    _0 = 0x0,
    #[doc = "SPDR values are read from the transmit buffer (but only if the transmit buffer is empty)"]
    _1 = 0x01,
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
pub enum Sprf {
    #[doc = "No valid data in SPDR"]
    _0 = 0x0,
    #[doc = "Valid data found in SPDR"]
    _1 = 0x01,
}
impl Sprf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sprf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sprf {
    #[inline(always)]
    fn from(val: u8) -> Sprf {
        Sprf::from_bits(val)
    }
}
impl From<Sprf> for u8 {
    #[inline(always)]
    fn from(val: Sprf) -> u8 {
        Sprf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sprie {
    #[doc = "Disables the generation of SPI receive buffer full interrupt requests"]
    _0 = 0x0,
    #[doc = "Enables the generation of SPI receive buffer full interrupt requests"]
    _1 = 0x01,
}
impl Sprie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sprie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sprie {
    #[inline(always)]
    fn from(val: u8) -> Sprie {
        Sprie::from_bits(val)
    }
}
impl From<Sprie> for u8 {
    #[inline(always)]
    fn from(val: Sprie) -> u8 {
        Sprie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sptef {
    #[doc = "Data found in the transmit buffer"]
    _0 = 0x0,
    #[doc = "No data in the transmit buffer"]
    _1 = 0x01,
}
impl Sptef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sptef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sptef {
    #[inline(always)]
    fn from(val: u8) -> Sptef {
        Sptef::from_bits(val)
    }
}
impl From<Sptef> for u8 {
    #[inline(always)]
    fn from(val: Sptef) -> u8 {
        Sptef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sptie {
    #[doc = "Disables the generation of transmit buffer empty interrupt requests"]
    _0 = 0x0,
    #[doc = "Enables the generation of transmit buffer empty interrupt requests"]
    _1 = 0x01,
}
impl Sptie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sptie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sptie {
    #[inline(always)]
    fn from(val: u8) -> Sptie {
        Sptie::from_bits(val)
    }
}
impl From<Sptie> for u8 {
    #[inline(always)]
    fn from(val: Sptie) -> u8 {
        Sptie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssl0p {
    #[doc = "SSL0 signal is active low"]
    _0 = 0x0,
    #[doc = "SSL0 signal is active high"]
    _1 = 0x01,
}
impl Ssl0p {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssl0p {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssl0p {
    #[inline(always)]
    fn from(val: u8) -> Ssl0p {
        Ssl0p::from_bits(val)
    }
}
impl From<Ssl0p> for u8 {
    #[inline(always)]
    fn from(val: Ssl0p) -> u8 {
        Ssl0p::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssl1p {
    #[doc = "SSL1 signal is active low"]
    _0 = 0x0,
    #[doc = "SSL1 signal is active high"]
    _1 = 0x01,
}
impl Ssl1p {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssl1p {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssl1p {
    #[inline(always)]
    fn from(val: u8) -> Ssl1p {
        Ssl1p::from_bits(val)
    }
}
impl From<Ssl1p> for u8 {
    #[inline(always)]
    fn from(val: Ssl1p) -> u8 {
        Ssl1p::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssl2p {
    #[doc = "SSL2 signal is active low"]
    _0 = 0x0,
    #[doc = "SSL2 signal is active high"]
    _1 = 0x01,
}
impl Ssl2p {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssl2p {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssl2p {
    #[inline(always)]
    fn from(val: u8) -> Ssl2p {
        Ssl2p::from_bits(val)
    }
}
impl From<Ssl2p> for u8 {
    #[inline(always)]
    fn from(val: Ssl2p) -> u8 {
        Ssl2p::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssl3p {
    #[doc = "SSL3 signal is active low"]
    _0 = 0x0,
    #[doc = "SSL3 signal is active high"]
    _1 = 0x01,
}
impl Ssl3p {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssl3p {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssl3p {
    #[inline(always)]
    fn from(val: u8) -> Ssl3p {
        Ssl3p::from_bits(val)
    }
}
impl From<Ssl3p> for u8 {
    #[inline(always)]
    fn from(val: Ssl3p) -> u8 {
        Ssl3p::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txmd {
    #[doc = "Full-duplex synchronous serial communications"]
    _0 = 0x0,
    #[doc = "Serial communications consisting of only transmit operations"]
    _1 = 0x01,
}
impl Txmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txmd {
    #[inline(always)]
    fn from(val: u8) -> Txmd {
        Txmd::from_bits(val)
    }
}
impl From<Txmd> for u8 {
    #[inline(always)]
    fn from(val: Txmd) -> u8 {
        Txmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Udrf {
    #[doc = "A mode fault error occurs (MODF=1)"]
    _0 = 0x0,
    #[doc = "An underrun error occurs (MODF=1)"]
    _1 = 0x01,
}
impl Udrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Udrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Udrf {
    #[inline(always)]
    fn from(val: u8) -> Udrf {
        Udrf::from_bits(val)
    }
}
impl From<Udrf> for u8 {
    #[inline(always)]
    fn from(val: Udrf) -> u8 {
        Udrf::to_bits(val)
    }
}
