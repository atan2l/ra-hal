#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Abcs {
    #[doc = "Selects 16 base clock cycles for 1-bit period."]
    _0 = 0x0,
    #[doc = "Selects 8 base clock cycles for 1-bit period."]
    _1 = 0x01,
}
impl Abcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Abcs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Abcs {
    #[inline(always)]
    fn from(val: u8) -> Abcs {
        Abcs::from_bits(val)
    }
}
impl From<Abcs> for u8 {
    #[inline(always)]
    fn from(val: Abcs) -> u8 {
        Abcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Abcse {
    #[doc = "Clock cycles for 1-bit period is decided with combination between BGDM and ABCS in SEMR."]
    _0 = 0x0,
    #[doc = "Baud rate is 6 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator."]
    _1 = 0x01,
}
impl Abcse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Abcse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Abcse {
    #[inline(always)]
    fn from(val: u8) -> Abcse {
        Abcse::from_bits(val)
    }
}
impl From<Abcse> for u8 {
    #[inline(always)]
    fn from(val: Abcse) -> u8 {
        Abcse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bcp {
    #[doc = "93 clock cycles(S=93) (SCMR.BCP2=0) / 32 clock cycles(S=32) (SCMR.BCP2=1)"]
    _00 = 0x0,
    #[doc = "128 clock cycles(S=128) (SCMR.BCP2=0) / 64 clock cycles(S=64) (SCMR.BCP2=1)"]
    _01 = 0x01,
    #[doc = "186 clock cycles(S=186) (SCMR.BCP2=0) / 372 clock cycles(S=372) (SCMR.BCP2=1)"]
    _10 = 0x02,
    #[doc = "512 clock cycles(S=512) (SCMR.BCP2=0) / 256 clock cycles(S=256) (SCMR.BCP2=1)"]
    _11 = 0x03,
}
impl Bcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bcp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bcp {
    #[inline(always)]
    fn from(val: u8) -> Bcp {
        Bcp::from_bits(val)
    }
}
impl From<Bcp> for u8 {
    #[inline(always)]
    fn from(val: Bcp) -> u8 {
        Bcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bcp2 {
    #[doc = "S=93(SMR.BCP\\[1:0\\]=00), 128(SMR.BCP\\[1:0\\]=01), 186(SMR.BCP\\[1:0\\]=10), 512(SMR.BCP\\[1:0\\]=11)"]
    _0 = 0x0,
    #[doc = "S=32(SMR.BCP\\[1:0\\]=00), 64(SMR.BCP\\[1:0\\]=01), 372(SMR.BCP\\[1:0\\]=10), 256(SMR.BCP\\[1:0\\]=11)"]
    _1 = 0x01,
}
impl Bcp2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bcp2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bcp2 {
    #[inline(always)]
    fn from(val: u8) -> Bcp2 {
        Bcp2::from_bits(val)
    }
}
impl From<Bcp2> for u8 {
    #[inline(always)]
    fn from(val: Bcp2) -> u8 {
        Bcp2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bgdm {
    #[doc = "Baud rate generator outputs the clock with normal frequency."]
    _0 = 0x0,
    #[doc = "Baud rate generator outputs the clock with doubled frequency."]
    _1 = 0x01,
}
impl Bgdm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bgdm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bgdm {
    #[inline(always)]
    fn from(val: u8) -> Bgdm {
        Bgdm::from_bits(val)
    }
}
impl From<Bgdm> for u8 {
    #[inline(always)]
    fn from(val: Bgdm) -> u8 {
        Bgdm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Blk {
    #[doc = "Normal mode operation"]
    _0 = 0x0,
    #[doc = "Block transfer mode operation"]
    _1 = 0x01,
}
impl Blk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Blk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Blk {
    #[inline(always)]
    fn from(val: u8) -> Blk {
        Blk::from_bits(val)
    }
}
impl From<Blk> for u8 {
    #[inline(always)]
    fn from(val: Blk) -> u8 {
        Blk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brme {
    #[doc = "Bit rate modulation function is disabled."]
    _0 = 0x0,
    #[doc = "Bit rate modulation function is enabled."]
    _1 = 0x01,
}
impl Brme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brme {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brme {
    #[inline(always)]
    fn from(val: u8) -> Brme {
        Brme::from_bits(val)
    }
}
impl From<Brme> for u8 {
    #[inline(always)]
    fn from(val: Brme) -> u8 {
        Brme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chr {
    #[doc = "Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 8bit data length(SCMR.CHR1=1)"]
    _0 = 0x0,
    #[doc = "Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 7bit data length(SCMR.CHR1=1)"]
    _1 = 0x01,
}
impl Chr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chr {
    #[inline(always)]
    fn from(val: u8) -> Chr {
        Chr::from_bits(val)
    }
}
impl From<Chr> for u8 {
    #[inline(always)]
    fn from(val: Chr) -> u8 {
        Chr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chr1 {
    #[doc = "Transmit/receive in 9-bit data length"]
    _0 = 0x0,
    #[doc = "Transmit/receive in 8-bit data length(SMR.CHR=0) / in 7bit data length(SMR.CHR=1)"]
    _1 = 0x01,
}
impl Chr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chr1 {
    #[inline(always)]
    fn from(val: u8) -> Chr1 {
        Chr1::from_bits(val)
    }
}
impl From<Chr1> for u8 {
    #[inline(always)]
    fn from(val: Chr1) -> u8 {
        Chr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ckph {
    #[doc = "Clock is not delayed."]
    _0 = 0x0,
    #[doc = "Clock is delayed."]
    _1 = 0x01,
}
impl Ckph {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ckph {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ckph {
    #[inline(always)]
    fn from(val: u8) -> Ckph {
        Ckph::from_bits(val)
    }
}
impl From<Ckph> for u8 {
    #[inline(always)]
    fn from(val: Ckph) -> u8 {
        Ckph::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ckpol {
    #[doc = "Clock polarity is not inverted."]
    _0 = 0x0,
    #[doc = "Clock polarity is inverted"]
    _1 = 0x01,
}
impl Ckpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ckpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ckpol {
    #[inline(always)]
    fn from(val: u8) -> Ckpol {
        Ckpol::from_bits(val)
    }
}
impl From<Ckpol> for u8 {
    #[inline(always)]
    fn from(val: Ckpol) -> u8 {
        Ckpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cm {
    #[doc = "Asynchronous mode or simple I2C mode"]
    _0 = 0x0,
    #[doc = "Clock synchronous mode"]
    _1 = 0x01,
}
impl Cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm {
    #[inline(always)]
    fn from(val: u8) -> Cm {
        Cm::from_bits(val)
    }
}
impl From<Cm> for u8 {
    #[inline(always)]
    fn from(val: Cm) -> u8 {
        Cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctse {
    #[doc = "CTS function is disabled (RTS output function is enabled)."]
    _0 = 0x0,
    #[doc = "CTS function is enabled."]
    _1 = 0x01,
}
impl Ctse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctse {
    #[inline(always)]
    fn from(val: u8) -> Ctse {
        Ctse::from_bits(val)
    }
}
impl From<Ctse> for u8 {
    #[inline(always)]
    fn from(val: Ctse) -> u8 {
        Ctse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcme {
    #[doc = "Address match function is disabled."]
    _0 = 0x0,
    #[doc = "Address match function is enabled"]
    _1 = 0x01,
}
impl Dcme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcme {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcme {
    #[inline(always)]
    fn from(val: u8) -> Dcme {
        Dcme::from_bits(val)
    }
}
impl From<Dcme> for u8 {
    #[inline(always)]
    fn from(val: Dcme) -> u8 {
        Dcme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcmf {
    #[doc = "No matched"]
    _0 = 0x0,
    #[doc = "Matched"]
    _1 = 0x01,
}
impl Dcmf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcmf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcmf {
    #[inline(always)]
    fn from(val: u8) -> Dcmf {
        Dcmf::from_bits(val)
    }
}
impl From<Dcmf> for u8 {
    #[inline(always)]
    fn from(val: Dcmf) -> u8 {
        Dcmf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dfer {
    #[doc = "No framing error occurred"]
    _0 = 0x0,
    #[doc = "A framing error has occurred"]
    _1 = 0x01,
}
impl Dfer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfer {
    #[inline(always)]
    fn from(val: u8) -> Dfer {
        Dfer::from_bits(val)
    }
}
impl From<Dfer> for u8 {
    #[inline(always)]
    fn from(val: Dfer) -> u8 {
        Dfer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dper {
    #[doc = "No parity error occurred"]
    _0 = 0x0,
    #[doc = "A parity error has occurred"]
    _1 = 0x01,
}
impl Dper {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dper {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dper {
    #[inline(always)]
    fn from(val: u8) -> Dper {
        Dper::from_bits(val)
    }
}
impl From<Dper> for u8 {
    #[inline(always)]
    fn from(val: Dper) -> u8 {
        Dper::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dres {
    #[doc = "reception data full interrupt (RXI)"]
    _0 = 0x0,
    #[doc = "receive error interrupt (ERI)"]
    _1 = 0x01,
}
impl Dres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dres {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dres {
    #[inline(always)]
    fn from(val: u8) -> Dres {
        Dres::from_bits(val)
    }
}
impl From<Dres> for u8 {
    #[inline(always)]
    fn from(val: Dres) -> u8 {
        Dres::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ers {
    #[doc = "Low error signal not responded"]
    _0 = 0x0,
    #[doc = "Low error signal responded"]
    _1 = 0x01,
}
impl Ers {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ers {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ers {
    #[inline(always)]
    fn from(val: u8) -> Ers {
        Ers::from_bits(val)
    }
}
impl From<Ers> for u8 {
    #[inline(always)]
    fn from(val: Ers) -> u8 {
        Ers::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fm {
    #[doc = "Non-FIFO mode(Selects o TDR/RDR for communication)"]
    _0 = 0x0,
    #[doc = "FIFO mode (Selects to FTDRH and FTDRL/FRDRH and FRDRL for communication)"]
    _1 = 0x01,
}
impl Fm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fm {
    #[inline(always)]
    fn from(val: u8) -> Fm {
        Fm::from_bits(val)
    }
}
impl From<Fm> for u8 {
    #[inline(always)]
    fn from(val: Fm) -> u8 {
        Fm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrdrhDr {
    #[doc = "Receiving is in progress, or no received data has remained in FRDRH and FRDRL after normally completed receiving."]
    _0 = 0x0,
    #[doc = "Next receive data has not been received for a period after normal completed receiving."]
    _1 = 0x01,
}
impl FrdrhDr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrdrhDr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrdrhDr {
    #[inline(always)]
    fn from(val: u8) -> FrdrhDr {
        FrdrhDr::from_bits(val)
    }
}
impl From<FrdrhDr> for u8 {
    #[inline(always)]
    fn from(val: FrdrhDr) -> u8 {
        FrdrhDr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrdrhFer {
    #[doc = "No framing error occurred at the first data of FRDRH and FRDRL"]
    _0 = 0x0,
    #[doc = "A framing error has occurred at the first data of FRDRH and FRDRL"]
    _1 = 0x01,
}
impl FrdrhFer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrdrhFer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrdrhFer {
    #[inline(always)]
    fn from(val: u8) -> FrdrhFer {
        FrdrhFer::from_bits(val)
    }
}
impl From<FrdrhFer> for u8 {
    #[inline(always)]
    fn from(val: FrdrhFer) -> u8 {
        FrdrhFer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrdrhMpb {
    #[doc = "Data transmission cycles"]
    _0 = 0x0,
    #[doc = "ID transmission cycles"]
    _1 = 0x01,
}
impl FrdrhMpb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrdrhMpb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrdrhMpb {
    #[inline(always)]
    fn from(val: u8) -> FrdrhMpb {
        FrdrhMpb::from_bits(val)
    }
}
impl From<FrdrhMpb> for u8 {
    #[inline(always)]
    fn from(val: FrdrhMpb) -> u8 {
        FrdrhMpb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrdrhOrer {
    #[doc = "No overrun error occurred"]
    _0 = 0x0,
    #[doc = "An overrun error has occurred"]
    _1 = 0x01,
}
impl FrdrhOrer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrdrhOrer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrdrhOrer {
    #[inline(always)]
    fn from(val: u8) -> FrdrhOrer {
        FrdrhOrer::from_bits(val)
    }
}
impl From<FrdrhOrer> for u8 {
    #[inline(always)]
    fn from(val: FrdrhOrer) -> u8 {
        FrdrhOrer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrdrhPer {
    #[doc = "No parity error occurred at the first data of FRDRH and FRDRL"]
    _0 = 0x0,
    #[doc = "A parity error has occurred at the first data of FRDRH and FRDRL"]
    _1 = 0x01,
}
impl FrdrhPer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrdrhPer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrdrhPer {
    #[inline(always)]
    fn from(val: u8) -> FrdrhPer {
        FrdrhPer::from_bits(val)
    }
}
impl From<FrdrhPer> for u8 {
    #[inline(always)]
    fn from(val: FrdrhPer) -> u8 {
        FrdrhPer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrdrhRdf {
    #[doc = "The quantity of receive data written in FRDRH and FRDRL falls below the specified receive triggering number."]
    _0 = 0x0,
    #[doc = "The quantity of receive data written in FRDRH and FRDRL is equal to or greater than the specified receive triggering number."]
    _1 = 0x01,
}
impl FrdrhRdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrdrhRdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrdrhRdf {
    #[inline(always)]
    fn from(val: u8) -> FrdrhRdf {
        FrdrhRdf::from_bits(val)
    }
}
impl From<FrdrhRdf> for u8 {
    #[inline(always)]
    fn from(val: FrdrhRdf) -> u8 {
        FrdrhRdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrdrhlDr {
    #[doc = "Receiving is in progress, or no received data has remained in FRDRH and FRDRL after normally completed receiving."]
    _0 = 0x0,
    #[doc = "Next receive data has not been received for a period after normal completed receiving."]
    _1 = 0x01,
}
impl FrdrhlDr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrdrhlDr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrdrhlDr {
    #[inline(always)]
    fn from(val: u8) -> FrdrhlDr {
        FrdrhlDr::from_bits(val)
    }
}
impl From<FrdrhlDr> for u8 {
    #[inline(always)]
    fn from(val: FrdrhlDr) -> u8 {
        FrdrhlDr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrdrhlFer {
    #[doc = "No framing error occurred at the first data of FRDRH and FRDRL."]
    _0 = 0x0,
    #[doc = "A framing error has occurred at the first data of FRDRH and FRDRL."]
    _1 = 0x01,
}
impl FrdrhlFer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrdrhlFer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrdrhlFer {
    #[inline(always)]
    fn from(val: u8) -> FrdrhlFer {
        FrdrhlFer::from_bits(val)
    }
}
impl From<FrdrhlFer> for u8 {
    #[inline(always)]
    fn from(val: FrdrhlFer) -> u8 {
        FrdrhlFer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrdrhlMpb {
    #[doc = "Data transmission cycles"]
    _0 = 0x0,
    #[doc = "ID transmission cycles"]
    _1 = 0x01,
}
impl FrdrhlMpb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrdrhlMpb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrdrhlMpb {
    #[inline(always)]
    fn from(val: u8) -> FrdrhlMpb {
        FrdrhlMpb::from_bits(val)
    }
}
impl From<FrdrhlMpb> for u8 {
    #[inline(always)]
    fn from(val: FrdrhlMpb) -> u8 {
        FrdrhlMpb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrdrhlOrer {
    #[doc = "No overrun error occurred."]
    _0 = 0x0,
    #[doc = "An overrun error has occurred."]
    _1 = 0x01,
}
impl FrdrhlOrer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrdrhlOrer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrdrhlOrer {
    #[inline(always)]
    fn from(val: u8) -> FrdrhlOrer {
        FrdrhlOrer::from_bits(val)
    }
}
impl From<FrdrhlOrer> for u8 {
    #[inline(always)]
    fn from(val: FrdrhlOrer) -> u8 {
        FrdrhlOrer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrdrhlPer {
    #[doc = "No parity error occurred at the first data of FRDRH and FRDRL."]
    _0 = 0x0,
    #[doc = "A parity error has occurred at the first data of FRDRH and FRDRL."]
    _1 = 0x01,
}
impl FrdrhlPer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrdrhlPer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrdrhlPer {
    #[inline(always)]
    fn from(val: u8) -> FrdrhlPer {
        FrdrhlPer::from_bits(val)
    }
}
impl From<FrdrhlPer> for u8 {
    #[inline(always)]
    fn from(val: FrdrhlPer) -> u8 {
        FrdrhlPer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrdrhlRdf {
    #[doc = "The quantity of receive data written in FRDRH and FRDRL falls below the specified receive triggering number."]
    _0 = 0x0,
    #[doc = "The quantity of receive data written in FRDRH and FRDRL is equal to or greater than the specified receive triggering number."]
    _1 = 0x01,
}
impl FrdrhlRdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrdrhlRdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrdrhlRdf {
    #[inline(always)]
    fn from(val: u8) -> FrdrhlRdf {
        FrdrhlRdf::from_bits(val)
    }
}
impl From<FrdrhlRdf> for u8 {
    #[inline(always)]
    fn from(val: FrdrhlRdf) -> u8 {
        FrdrhlRdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FtdrhMpbt {
    #[doc = "Data transmission cycles"]
    _0 = 0x0,
    #[doc = "ID transmission cycles"]
    _1 = 0x01,
}
impl FtdrhMpbt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FtdrhMpbt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FtdrhMpbt {
    #[inline(always)]
    fn from(val: u8) -> FtdrhMpbt {
        FtdrhMpbt::from_bits(val)
    }
}
impl From<FtdrhMpbt> for u8 {
    #[inline(always)]
    fn from(val: FtdrhMpbt) -> u8 {
        FtdrhMpbt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FtdrhlMpbt {
    #[doc = "Data transmission cycles"]
    _0 = 0x0,
    #[doc = "ID transmission cycles"]
    _1 = 0x01,
}
impl FtdrhlMpbt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FtdrhlMpbt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FtdrhlMpbt {
    #[inline(always)]
    fn from(val: u8) -> FtdrhlMpbt {
        FtdrhlMpbt::from_bits(val)
    }
}
impl From<FtdrhlMpbt> for u8 {
    #[inline(always)]
    fn from(val: FtdrhlMpbt) -> u8 {
        FtdrhlMpbt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gm {
    #[doc = "Normal mode operation"]
    _0 = 0x0,
    #[doc = "GSM mode operation"]
    _1 = 0x01,
}
impl Gm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gm {
    #[inline(always)]
    fn from(val: u8) -> Gm {
        Gm::from_bits(val)
    }
}
impl From<Gm> for u8 {
    #[inline(always)]
    fn from(val: Gm) -> u8 {
        Gm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idsel {
    #[doc = "Always compare data regardless of the value of the MPB bit."]
    _0 = 0x0,
    #[doc = "Compare data when the MPB bit is 1 (ID frame) only."]
    _1 = 0x01,
}
impl Idsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idsel {
    #[inline(always)]
    fn from(val: u8) -> Idsel {
        Idsel::from_bits(val)
    }
}
impl From<Idsel> for u8 {
    #[inline(always)]
    fn from(val: Idsel) -> u8 {
        Idsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicackr {
    #[doc = "ACK received"]
    _0 = 0x0,
    #[doc = "NACK received"]
    _1 = 0x01,
}
impl Iicackr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicackr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicackr {
    #[inline(always)]
    fn from(val: u8) -> Iicackr {
        Iicackr::from_bits(val)
    }
}
impl From<Iicackr> for u8 {
    #[inline(always)]
    fn from(val: Iicackr) -> u8 {
        Iicackr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicackt {
    #[doc = "ACK transmission"]
    _0 = 0x0,
    #[doc = "NACK transmission and reception of ACK/NACK"]
    _1 = 0x01,
}
impl Iicackt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicackt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicackt {
    #[inline(always)]
    fn from(val: u8) -> Iicackt {
        Iicackt::from_bits(val)
    }
}
impl From<Iicackt> for u8 {
    #[inline(always)]
    fn from(val: Iicackt) -> u8 {
        Iicackt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iiccsc {
    #[doc = "No synchronization with the clock signal"]
    _0 = 0x0,
    #[doc = "Synchronization with the clock signal"]
    _1 = 0x01,
}
impl Iiccsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iiccsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iiccsc {
    #[inline(always)]
    fn from(val: u8) -> Iiccsc {
        Iiccsc::from_bits(val)
    }
}
impl From<Iiccsc> for u8 {
    #[inline(always)]
    fn from(val: Iiccsc) -> u8 {
        Iiccsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicdl {
    #[doc = "No output delay"]
    _00000 = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Iicdl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicdl {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicdl {
    #[inline(always)]
    fn from(val: u8) -> Iicdl {
        Iicdl::from_bits(val)
    }
}
impl From<Iicdl> for u8 {
    #[inline(always)]
    fn from(val: Iicdl) -> u8 {
        Iicdl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicintm {
    #[doc = "Use ACK/NACK interrupts."]
    _0 = 0x0,
    #[doc = "Use reception and transmission interrupts"]
    _1 = 0x01,
}
impl Iicintm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicintm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicintm {
    #[inline(always)]
    fn from(val: u8) -> Iicintm {
        Iicintm::from_bits(val)
    }
}
impl From<Iicintm> for u8 {
    #[inline(always)]
    fn from(val: Iicintm) -> u8 {
        Iicintm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicm {
    #[doc = "Asynchronous mode, Multi-processor mode, Clock synchronous mode(SCMR.SMIF=0) /Smart card interface mode(SCMR.SMIF=1)"]
    _0 = 0x0,
    #[doc = "Simple I2C mode(SCMR.SMIF=0) / Setting prohibited.(SCMR.SMIF=1)"]
    _1 = 0x01,
}
impl Iicm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicm {
    #[inline(always)]
    fn from(val: u8) -> Iicm {
        Iicm::from_bits(val)
    }
}
impl From<Iicm> for u8 {
    #[inline(always)]
    fn from(val: Iicm) -> u8 {
        Iicm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicrstareq {
    #[doc = "A restart condition is not generated."]
    _0 = 0x0,
    #[doc = "A restart condition is generated."]
    _1 = 0x01,
}
impl Iicrstareq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicrstareq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicrstareq {
    #[inline(always)]
    fn from(val: u8) -> Iicrstareq {
        Iicrstareq::from_bits(val)
    }
}
impl From<Iicrstareq> for u8 {
    #[inline(always)]
    fn from(val: Iicrstareq) -> u8 {
        Iicrstareq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicscls {
    #[doc = "Serial clock output"]
    _00 = 0x0,
    #[doc = "Generate a start, restart, or stop condition."]
    _01 = 0x01,
    #[doc = "Output the low level on the SSCLn pin."]
    _10 = 0x02,
    #[doc = "Place the SSCLn pin in the high-impedance state."]
    _11 = 0x03,
}
impl Iicscls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicscls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicscls {
    #[inline(always)]
    fn from(val: u8) -> Iicscls {
        Iicscls::from_bits(val)
    }
}
impl From<Iicscls> for u8 {
    #[inline(always)]
    fn from(val: Iicscls) -> u8 {
        Iicscls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicsdas {
    #[doc = "Serial data output"]
    _00 = 0x0,
    #[doc = "Generate a start, restart, or stop condition."]
    _01 = 0x01,
    #[doc = "Output the low level on the SSDAn pin."]
    _10 = 0x02,
    #[doc = "Place the SSDAn pin in the high-impedance state."]
    _11 = 0x03,
}
impl Iicsdas {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicsdas {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicsdas {
    #[inline(always)]
    fn from(val: u8) -> Iicsdas {
        Iicsdas::from_bits(val)
    }
}
impl From<Iicsdas> for u8 {
    #[inline(always)]
    fn from(val: Iicsdas) -> u8 {
        Iicsdas::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicstareq {
    #[doc = "A start condition is not generated."]
    _0 = 0x0,
    #[doc = "A start condition is generated."]
    _1 = 0x01,
}
impl Iicstareq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicstareq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicstareq {
    #[inline(always)]
    fn from(val: u8) -> Iicstareq {
        Iicstareq::from_bits(val)
    }
}
impl From<Iicstareq> for u8 {
    #[inline(always)]
    fn from(val: Iicstareq) -> u8 {
        Iicstareq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicstif {
    #[doc = "There are no requests for generating conditions or a condition is being generated."]
    _0 = 0x0,
    #[doc = "A start, restart, or stop condition is completely generated."]
    _1 = 0x01,
}
impl Iicstif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicstif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicstif {
    #[inline(always)]
    fn from(val: u8) -> Iicstif {
        Iicstif::from_bits(val)
    }
}
impl From<Iicstif> for u8 {
    #[inline(always)]
    fn from(val: Iicstif) -> u8 {
        Iicstif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicstpreq {
    #[doc = "A stop condition is not generated."]
    _0 = 0x0,
    #[doc = "A stop condition is generated."]
    _1 = 0x01,
}
impl Iicstpreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicstpreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicstpreq {
    #[inline(always)]
    fn from(val: u8) -> Iicstpreq {
        Iicstpreq::from_bits(val)
    }
}
impl From<Iicstpreq> for u8 {
    #[inline(always)]
    fn from(val: Iicstpreq) -> u8 {
        Iicstpreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LsrOrer {
    #[doc = "No overrun error occurred"]
    _0 = 0x0,
    #[doc = "An overrun error has occurred"]
    _1 = 0x01,
}
impl LsrOrer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LsrOrer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LsrOrer {
    #[inline(always)]
    fn from(val: u8) -> LsrOrer {
        LsrOrer::from_bits(val)
    }
}
impl From<LsrOrer> for u8 {
    #[inline(always)]
    fn from(val: LsrOrer) -> u8 {
        LsrOrer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mff {
    #[doc = "No mode fault error"]
    _0 = 0x0,
    #[doc = "Mode fault error"]
    _1 = 0x01,
}
impl Mff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mff {
    #[inline(always)]
    fn from(val: u8) -> Mff {
        Mff::from_bits(val)
    }
}
impl From<Mff> for u8 {
    #[inline(always)]
    fn from(val: Mff) -> u8 {
        Mff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mp {
    #[doc = "Multi-processor communications function is disabled"]
    _0 = 0x0,
    #[doc = "Multi-processor communications function is enabled"]
    _1 = 0x01,
}
impl Mp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mp {
    #[inline(always)]
    fn from(val: u8) -> Mp {
        Mp::from_bits(val)
    }
}
impl From<Mp> for u8 {
    #[inline(always)]
    fn from(val: Mp) -> u8 {
        Mp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mpie {
    #[doc = "Normal reception"]
    _0 = 0x0,
    #[doc = "When the data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF,ORER and FER in SSR to 1 is disabled. When the data with the multiprocessor bit set to 1 is received, the MPIE bit is automatically cleared to 0, and normal reception is resumed."]
    _1 = 0x01,
}
impl Mpie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mpie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mpie {
    #[inline(always)]
    fn from(val: u8) -> Mpie {
        Mpie::from_bits(val)
    }
}
impl From<Mpie> for u8 {
    #[inline(always)]
    fn from(val: Mpie) -> u8 {
        Mpie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mss {
    #[doc = "Transmission is through the TXDn pin and reception is through the RXDn pin (master mode)."]
    _0 = 0x0,
    #[doc = "Reception is through the TXDn pin and transmission is through the RXDn pin (slave mode)."]
    _1 = 0x01,
}
impl Mss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mss {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mss {
    #[inline(always)]
    fn from(val: u8) -> Mss {
        Mss::from_bits(val)
    }
}
impl From<Mss> for u8 {
    #[inline(always)]
    fn from(val: Mss) -> u8 {
        Mss::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nfcs {
    #[doc = "The clock signal divided by 1 is used with the noise filter.(In asynchronous mode)"]
    _000 = 0x0,
    #[doc = "The clock signal divided by 1 is used with the noise filter.(In simple I2C mode)"]
    _001 = 0x01,
    #[doc = "The clock signal divided by 2 is used with the noise filter.(In simple I2C mode)"]
    _010 = 0x02,
    #[doc = "The clock signal divided by 4 is used with the noise filter.(In simple I2C mode)"]
    _011 = 0x03,
    #[doc = "The clock signal divided by 8 is used with the noise filter.(In simple I2C mode)"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Nfcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfcs {
        unsafe { core::mem::transmute(val & 0x07) }
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
    #[doc = "Noise cancellation function for the RXDn/SSCLn and SSDAn input signal is disabled."]
    _0 = 0x0,
    #[doc = "Noise cancellation function for the RXDn/SSCLn and SSDAn input signal is enabled."]
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
pub enum Rfrst {
    #[doc = "The number of data stored in FRDRH and FRDRL register are NOT made 0"]
    _0 = 0x0,
    #[doc = "The number of data stored in FRDRH and FRDRL register are made 0"]
    _1 = 0x01,
}
impl Rfrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfrst {
    #[inline(always)]
    fn from(val: u8) -> Rfrst {
        Rfrst::from_bits(val)
    }
}
impl From<Rfrst> for u8 {
    #[inline(always)]
    fn from(val: Rfrst) -> u8 {
        Rfrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstrg {
    #[doc = "Trigger number 0"]
    _0000 = 0x0,
    _RESERVED_1 = 0x01,
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
impl Rstrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstrg {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstrg {
    #[inline(always)]
    fn from(val: u8) -> Rstrg {
        Rstrg::from_bits(val)
    }
}
impl From<Rstrg> for u8 {
    #[inline(always)]
    fn from(val: Rstrg) -> u8 {
        Rstrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtrg {
    #[doc = "Trigger number 0"]
    _0000 = 0x0,
    _RESERVED_1 = 0x01,
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
impl Rtrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtrg {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtrg {
    #[inline(always)]
    fn from(val: u8) -> Rtrg {
        Rtrg::from_bits(val)
    }
}
impl From<Rtrg> for u8 {
    #[inline(always)]
    fn from(val: Rtrg) -> u8 {
        Rtrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxdesel {
    #[doc = "The low level on the RXDn pin is detected as the start bit."]
    _0 = 0x0,
    #[doc = "A falling edge on the RXDn pin is detected as the start bit."]
    _1 = 0x01,
}
impl Rxdesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxdesel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxdesel {
    #[inline(always)]
    fn from(val: u8) -> Rxdesel {
        Rxdesel::from_bits(val)
    }
}
impl From<Rxdesel> for u8 {
    #[inline(always)]
    fn from(val: Rxdesel) -> u8 {
        Rxdesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxdmon {
    #[doc = "RXD pin is low."]
    _0 = 0x0,
    #[doc = "RXD pin is high."]
    _1 = 0x01,
}
impl Rxdmon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxdmon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxdmon {
    #[inline(always)]
    fn from(val: u8) -> Rxdmon {
        Rxdmon::from_bits(val)
    }
}
impl From<Rxdmon> for u8 {
    #[inline(always)]
    fn from(val: Rxdmon) -> u8 {
        Rxdmon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrCke {
    #[doc = "The SCKn pin is available for use as an I/O port in accord with the I/O port settings.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    _00 = 0x0,
    #[doc = "The clock with the same frequency as the bit rate is output from the SCKn pin.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    _01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl ScrCke {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrCke {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrCke {
    #[inline(always)]
    fn from(val: u8) -> ScrCke {
        ScrCke::from_bits(val)
    }
}
impl From<ScrCke> for u8 {
    #[inline(always)]
    fn from(val: ScrCke) -> u8 {
        ScrCke::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrRe {
    #[doc = "Serial reception is disabled"]
    _0 = 0x0,
    #[doc = "Serial reception is enabled"]
    _1 = 0x01,
}
impl ScrRe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrRe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrRe {
    #[inline(always)]
    fn from(val: u8) -> ScrRe {
        ScrRe::from_bits(val)
    }
}
impl From<ScrRe> for u8 {
    #[inline(always)]
    fn from(val: ScrRe) -> u8 {
        ScrRe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrRie {
    #[doc = "SCI_RXI and SCI_ERI interrupt requests are disabled"]
    _0 = 0x0,
    #[doc = "SCI_RXI and SCI_ERI interrupt requests are enabled"]
    _1 = 0x01,
}
impl ScrRie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrRie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrRie {
    #[inline(always)]
    fn from(val: u8) -> ScrRie {
        ScrRie::from_bits(val)
    }
}
impl From<ScrRie> for u8 {
    #[inline(always)]
    fn from(val: ScrRie) -> u8 {
        ScrRie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrSmciCke {
    #[doc = "Output disabled(SMR_SMCI.GM=0) / Output fixed low(SMR_SMCI.GM=1)"]
    _00 = 0x0,
    #[doc = "Clock Output"]
    _01 = 0x01,
    #[doc = "Setting prohibited(SMR_SMCI.GM=0) / Output fixed High(SMR_SMCI.GM=1)"]
    _10 = 0x02,
    #[doc = "Setting prohibited(SMR_SMCI.GM=0) / Clock Output(SMR_SMCI.GM=1)"]
    _11 = 0x03,
}
impl ScrSmciCke {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrSmciCke {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrSmciCke {
    #[inline(always)]
    fn from(val: u8) -> ScrSmciCke {
        ScrSmciCke::from_bits(val)
    }
}
impl From<ScrSmciCke> for u8 {
    #[inline(always)]
    fn from(val: ScrSmciCke) -> u8 {
        ScrSmciCke::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrSmciRe {
    #[doc = "Serial reception is disabled"]
    _0 = 0x0,
    #[doc = "Serial reception is enabled"]
    _1 = 0x01,
}
impl ScrSmciRe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrSmciRe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrSmciRe {
    #[inline(always)]
    fn from(val: u8) -> ScrSmciRe {
        ScrSmciRe::from_bits(val)
    }
}
impl From<ScrSmciRe> for u8 {
    #[inline(always)]
    fn from(val: ScrSmciRe) -> u8 {
        ScrSmciRe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrSmciRie {
    #[doc = "SCI_RXI and SCI_ERI interrupt requests are disabled"]
    _0 = 0x0,
    #[doc = "SCI_RXI and SCI_ERI interrupt requests are enabled"]
    _1 = 0x01,
}
impl ScrSmciRie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrSmciRie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrSmciRie {
    #[inline(always)]
    fn from(val: u8) -> ScrSmciRie {
        ScrSmciRie::from_bits(val)
    }
}
impl From<ScrSmciRie> for u8 {
    #[inline(always)]
    fn from(val: ScrSmciRie) -> u8 {
        ScrSmciRie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrSmciTe {
    #[doc = "Serial transmission is disabled"]
    _0 = 0x0,
    #[doc = "Serial transmission is enabled"]
    _1 = 0x01,
}
impl ScrSmciTe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrSmciTe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrSmciTe {
    #[inline(always)]
    fn from(val: u8) -> ScrSmciTe {
        ScrSmciTe::from_bits(val)
    }
}
impl From<ScrSmciTe> for u8 {
    #[inline(always)]
    fn from(val: ScrSmciTe) -> u8 {
        ScrSmciTe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrSmciTie {
    #[doc = "A SCI_TXI interrupt request is disabled"]
    _0 = 0x0,
    #[doc = "A SCI_TXI interrupt request is enabled"]
    _1 = 0x01,
}
impl ScrSmciTie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrSmciTie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrSmciTie {
    #[inline(always)]
    fn from(val: u8) -> ScrSmciTie {
        ScrSmciTie::from_bits(val)
    }
}
impl From<ScrSmciTie> for u8 {
    #[inline(always)]
    fn from(val: ScrSmciTie) -> u8 {
        ScrSmciTie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrTe {
    #[doc = "Serial transmission is disabled"]
    _0 = 0x0,
    #[doc = "Serial transmission is enabled"]
    _1 = 0x01,
}
impl ScrTe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrTe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrTe {
    #[inline(always)]
    fn from(val: u8) -> ScrTe {
        ScrTe::from_bits(val)
    }
}
impl From<ScrTe> for u8 {
    #[inline(always)]
    fn from(val: ScrTe) -> u8 {
        ScrTe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrTie {
    #[doc = "SCI_TXI interrupt request is disabled"]
    _0 = 0x0,
    #[doc = "SCI_TXI interrupt request is enabled"]
    _1 = 0x01,
}
impl ScrTie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrTie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrTie {
    #[inline(always)]
    fn from(val: u8) -> ScrTie {
        ScrTie::from_bits(val)
    }
}
impl From<ScrTie> for u8 {
    #[inline(always)]
    fn from(val: ScrTie) -> u8 {
        ScrTie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdir {
    #[doc = "Transfer with LSB first"]
    _0 = 0x0,
    #[doc = "Transfer with MSB first"]
    _1 = 0x01,
}
impl Sdir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdir {
    #[inline(always)]
    fn from(val: u8) -> Sdir {
        Sdir::from_bits(val)
    }
}
impl From<Sdir> for u8 {
    #[inline(always)]
    fn from(val: Sdir) -> u8 {
        Sdir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinv {
    #[doc = "TDR contents are transmitted as they are. Receive data is stored as it is in RDR."]
    _0 = 0x0,
    #[doc = "TDR contents are inverted before being transmitted. Receive data is stored in inverted form in RDR."]
    _1 = 0x01,
}
impl Sinv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinv {
    #[inline(always)]
    fn from(val: u8) -> Sinv {
        Sinv::from_bits(val)
    }
}
impl From<Sinv> for u8 {
    #[inline(always)]
    fn from(val: Sinv) -> u8 {
        Sinv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smif {
    #[doc = "Non-smart card interface mode(Asynchronous mode, clock synchronous mode, simple SPI mode, or simple I2C mode)"]
    _0 = 0x0,
    #[doc = "Smart card interface mode"]
    _1 = 0x01,
}
impl Smif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smif {
    #[inline(always)]
    fn from(val: u8) -> Smif {
        Smif::from_bits(val)
    }
}
impl From<Smif> for u8 {
    #[inline(always)]
    fn from(val: Smif) -> u8 {
        Smif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmrCks {
    #[doc = "PCLK clock"]
    _00 = 0x0,
    #[doc = "PCLK/4 clock"]
    _01 = 0x01,
    #[doc = "PCLK/16 clock"]
    _10 = 0x02,
    #[doc = "PCLK/64 clock"]
    _11 = 0x03,
}
impl SmrCks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmrCks {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmrCks {
    #[inline(always)]
    fn from(val: u8) -> SmrCks {
        SmrCks::from_bits(val)
    }
}
impl From<SmrCks> for u8 {
    #[inline(always)]
    fn from(val: SmrCks) -> u8 {
        SmrCks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmrPe {
    #[doc = "Parity bit addition is not performed (transmitting) / Parity bit checking is not performed ( receiving )"]
    _0 = 0x0,
    #[doc = "The parity bit is added (transmitting) / The parity bit is checked (receiving)"]
    _1 = 0x01,
}
impl SmrPe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmrPe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmrPe {
    #[inline(always)]
    fn from(val: u8) -> SmrPe {
        SmrPe::from_bits(val)
    }
}
impl From<SmrPe> for u8 {
    #[inline(always)]
    fn from(val: SmrPe) -> u8 {
        SmrPe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmrPm {
    #[doc = "Selects even parity"]
    _0 = 0x0,
    #[doc = "Selects odd parity"]
    _1 = 0x01,
}
impl SmrPm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmrPm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmrPm {
    #[inline(always)]
    fn from(val: u8) -> SmrPm {
        SmrPm::from_bits(val)
    }
}
impl From<SmrPm> for u8 {
    #[inline(always)]
    fn from(val: SmrPm) -> u8 {
        SmrPm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmrSmciCks {
    #[doc = "PCLK clock"]
    _00 = 0x0,
    #[doc = "PCLK/4 clock"]
    _01 = 0x01,
    #[doc = "PCLK/16 clock"]
    _10 = 0x02,
    #[doc = "PCLK/64 clock"]
    _11 = 0x03,
}
impl SmrSmciCks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmrSmciCks {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmrSmciCks {
    #[inline(always)]
    fn from(val: u8) -> SmrSmciCks {
        SmrSmciCks::from_bits(val)
    }
}
impl From<SmrSmciCks> for u8 {
    #[inline(always)]
    fn from(val: SmrSmciCks) -> u8 {
        SmrSmciCks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmrSmciPe {
    #[doc = "Setting Prohibited"]
    _0 = 0x0,
    #[doc = "Set this bit to 1 in smart card interface mode."]
    _1 = 0x01,
}
impl SmrSmciPe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmrSmciPe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmrSmciPe {
    #[inline(always)]
    fn from(val: u8) -> SmrSmciPe {
        SmrSmciPe::from_bits(val)
    }
}
impl From<SmrSmciPe> for u8 {
    #[inline(always)]
    fn from(val: SmrSmciPe) -> u8 {
        SmrSmciPe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmrSmciPm {
    #[doc = "Selects even parity"]
    _0 = 0x0,
    #[doc = "Selects odd parity"]
    _1 = 0x01,
}
impl SmrSmciPm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmrSmciPm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmrSmciPm {
    #[inline(always)]
    fn from(val: u8) -> SmrSmciPm {
        SmrSmciPm::from_bits(val)
    }
}
impl From<SmrSmciPm> for u8 {
    #[inline(always)]
    fn from(val: SmrSmciPm) -> u8 {
        SmrSmciPm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spb2dt {
    #[doc = "Low level is output on TXD pin"]
    _0 = 0x0,
    #[doc = "High level is output on TXD pin"]
    _1 = 0x01,
}
impl Spb2dt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spb2dt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spb2dt {
    #[inline(always)]
    fn from(val: u8) -> Spb2dt {
        Spb2dt::from_bits(val)
    }
}
impl From<Spb2dt> for u8 {
    #[inline(always)]
    fn from(val: Spb2dt) -> u8 {
        Spb2dt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spb2io {
    #[doc = "The value of SPB2DT bit is not output in TXD pin."]
    _0 = 0x0,
    #[doc = "The value of SPB2DT bit is output in TXD pin."]
    _1 = 0x01,
}
impl Spb2io {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spb2io {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spb2io {
    #[inline(always)]
    fn from(val: u8) -> Spb2io {
        Spb2io::from_bits(val)
    }
}
impl From<Spb2io> for u8 {
    #[inline(always)]
    fn from(val: Spb2io) -> u8 {
        Spb2io::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sse {
    #[doc = "SSn pin function is disabled."]
    _0 = 0x0,
    #[doc = "SSn pin function is enabled."]
    _1 = 0x01,
}
impl Sse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sse {
    #[inline(always)]
    fn from(val: u8) -> Sse {
        Sse::from_bits(val)
    }
}
impl From<Sse> for u8 {
    #[inline(always)]
    fn from(val: Sse) -> u8 {
        Sse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrFer {
    #[doc = "No framing error occurred"]
    _0 = 0x0,
    #[doc = "A framing error has occurred"]
    _1 = 0x01,
}
impl SsrFer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrFer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrFer {
    #[inline(always)]
    fn from(val: u8) -> SsrFer {
        SsrFer::from_bits(val)
    }
}
impl From<SsrFer> for u8 {
    #[inline(always)]
    fn from(val: SsrFer) -> u8 {
        SsrFer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrFifoDr {
    #[doc = "Receiving is in progress, or no received data has remained in FRDR after normally completed receiving.(receive FIFO is empty)"]
    _0 = 0x0,
    #[doc = "Next receive data has not been received for a period after normal completed receiving, , when data is stored in FIFO to equal or less than receive triggering number."]
    _1 = 0x01,
}
impl SsrFifoDr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrFifoDr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrFifoDr {
    #[inline(always)]
    fn from(val: u8) -> SsrFifoDr {
        SsrFifoDr::from_bits(val)
    }
}
impl From<SsrFifoDr> for u8 {
    #[inline(always)]
    fn from(val: SsrFifoDr) -> u8 {
        SsrFifoDr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrFifoFer {
    #[doc = "No framing error occurred."]
    _0 = 0x0,
    #[doc = "A framing error has occurred."]
    _1 = 0x01,
}
impl SsrFifoFer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrFifoFer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrFifoFer {
    #[inline(always)]
    fn from(val: u8) -> SsrFifoFer {
        SsrFifoFer::from_bits(val)
    }
}
impl From<SsrFifoFer> for u8 {
    #[inline(always)]
    fn from(val: SsrFifoFer) -> u8 {
        SsrFifoFer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrFifoOrer {
    #[doc = "No overrun error occurred"]
    _0 = 0x0,
    #[doc = "An overrun error has occurred"]
    _1 = 0x01,
}
impl SsrFifoOrer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrFifoOrer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrFifoOrer {
    #[inline(always)]
    fn from(val: u8) -> SsrFifoOrer {
        SsrFifoOrer::from_bits(val)
    }
}
impl From<SsrFifoOrer> for u8 {
    #[inline(always)]
    fn from(val: SsrFifoOrer) -> u8 {
        SsrFifoOrer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrFifoPer {
    #[doc = "No parity error occurred."]
    _0 = 0x0,
    #[doc = "A parity error has occurred."]
    _1 = 0x01,
}
impl SsrFifoPer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrFifoPer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrFifoPer {
    #[inline(always)]
    fn from(val: u8) -> SsrFifoPer {
        SsrFifoPer::from_bits(val)
    }
}
impl From<SsrFifoPer> for u8 {
    #[inline(always)]
    fn from(val: SsrFifoPer) -> u8 {
        SsrFifoPer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrFifoRdf {
    #[doc = "The quantity of receive data written in FRDR falls below the specified receive triggering number."]
    _0 = 0x0,
    #[doc = "The quantity of receive data written in FRDR is equal to or greater than the specified receive triggering number."]
    _1 = 0x01,
}
impl SsrFifoRdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrFifoRdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrFifoRdf {
    #[inline(always)]
    fn from(val: u8) -> SsrFifoRdf {
        SsrFifoRdf::from_bits(val)
    }
}
impl From<SsrFifoRdf> for u8 {
    #[inline(always)]
    fn from(val: SsrFifoRdf) -> u8 {
        SsrFifoRdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrFifoTend {
    #[doc = "A character is being transmitted or standing by for transmission."]
    _0 = 0x0,
    #[doc = "Character transfer has been completed."]
    _1 = 0x01,
}
impl SsrFifoTend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrFifoTend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrFifoTend {
    #[inline(always)]
    fn from(val: u8) -> SsrFifoTend {
        SsrFifoTend::from_bits(val)
    }
}
impl From<SsrFifoTend> for u8 {
    #[inline(always)]
    fn from(val: SsrFifoTend) -> u8 {
        SsrFifoTend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrMpb {
    #[doc = "Data transmission cycles"]
    _0 = 0x0,
    #[doc = "ID transmission cycles"]
    _1 = 0x01,
}
impl SsrMpb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrMpb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrMpb {
    #[inline(always)]
    fn from(val: u8) -> SsrMpb {
        SsrMpb::from_bits(val)
    }
}
impl From<SsrMpb> for u8 {
    #[inline(always)]
    fn from(val: SsrMpb) -> u8 {
        SsrMpb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrMpbt {
    #[doc = "Data transmission cycles"]
    _0 = 0x0,
    #[doc = "ID transmission cycles"]
    _1 = 0x01,
}
impl SsrMpbt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrMpbt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrMpbt {
    #[inline(always)]
    fn from(val: u8) -> SsrMpbt {
        SsrMpbt::from_bits(val)
    }
}
impl From<SsrMpbt> for u8 {
    #[inline(always)]
    fn from(val: SsrMpbt) -> u8 {
        SsrMpbt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrOrer {
    #[doc = "No overrun error occurred"]
    _0 = 0x0,
    #[doc = "An overrun error has occurred"]
    _1 = 0x01,
}
impl SsrOrer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrOrer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrOrer {
    #[inline(always)]
    fn from(val: u8) -> SsrOrer {
        SsrOrer::from_bits(val)
    }
}
impl From<SsrOrer> for u8 {
    #[inline(always)]
    fn from(val: SsrOrer) -> u8 {
        SsrOrer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrPer {
    #[doc = "No parity error occurred"]
    _0 = 0x0,
    #[doc = "A parity error has occurred"]
    _1 = 0x01,
}
impl SsrPer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrPer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrPer {
    #[inline(always)]
    fn from(val: u8) -> SsrPer {
        SsrPer::from_bits(val)
    }
}
impl From<SsrPer> for u8 {
    #[inline(always)]
    fn from(val: SsrPer) -> u8 {
        SsrPer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrRdrf {
    #[doc = "No received data is in RDR register"]
    _0 = 0x0,
    #[doc = "Received data is in RDR register"]
    _1 = 0x01,
}
impl SsrRdrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrRdrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrRdrf {
    #[inline(always)]
    fn from(val: u8) -> SsrRdrf {
        SsrRdrf::from_bits(val)
    }
}
impl From<SsrRdrf> for u8 {
    #[inline(always)]
    fn from(val: SsrRdrf) -> u8 {
        SsrRdrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrSmciOrer {
    #[doc = "No overrun error occurred"]
    _0 = 0x0,
    #[doc = "An overrun error has occurred"]
    _1 = 0x01,
}
impl SsrSmciOrer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrSmciOrer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrSmciOrer {
    #[inline(always)]
    fn from(val: u8) -> SsrSmciOrer {
        SsrSmciOrer::from_bits(val)
    }
}
impl From<SsrSmciOrer> for u8 {
    #[inline(always)]
    fn from(val: SsrSmciOrer) -> u8 {
        SsrSmciOrer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrSmciPer {
    #[doc = "No parity error occurred"]
    _0 = 0x0,
    #[doc = "A parity error has occurred"]
    _1 = 0x01,
}
impl SsrSmciPer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrSmciPer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrSmciPer {
    #[inline(always)]
    fn from(val: u8) -> SsrSmciPer {
        SsrSmciPer::from_bits(val)
    }
}
impl From<SsrSmciPer> for u8 {
    #[inline(always)]
    fn from(val: SsrSmciPer) -> u8 {
        SsrSmciPer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrSmciRdrf {
    #[doc = "No received data is in RDR register"]
    _0 = 0x0,
    #[doc = "Received data is in RDR register"]
    _1 = 0x01,
}
impl SsrSmciRdrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrSmciRdrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrSmciRdrf {
    #[inline(always)]
    fn from(val: u8) -> SsrSmciRdrf {
        SsrSmciRdrf::from_bits(val)
    }
}
impl From<SsrSmciRdrf> for u8 {
    #[inline(always)]
    fn from(val: SsrSmciRdrf) -> u8 {
        SsrSmciRdrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrSmciTdre {
    #[doc = "Transmit data is in TDR register"]
    _0 = 0x0,
    #[doc = "No transmit data is in TDR register"]
    _1 = 0x01,
}
impl SsrSmciTdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrSmciTdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrSmciTdre {
    #[inline(always)]
    fn from(val: u8) -> SsrSmciTdre {
        SsrSmciTdre::from_bits(val)
    }
}
impl From<SsrSmciTdre> for u8 {
    #[inline(always)]
    fn from(val: SsrSmciTdre) -> u8 {
        SsrSmciTdre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrSmciTend {
    #[doc = "A character is being transmitted."]
    _0 = 0x0,
    #[doc = "Character transfer has been completed."]
    _1 = 0x01,
}
impl SsrSmciTend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrSmciTend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrSmciTend {
    #[inline(always)]
    fn from(val: u8) -> SsrSmciTend {
        SsrSmciTend::from_bits(val)
    }
}
impl From<SsrSmciTend> for u8 {
    #[inline(always)]
    fn from(val: SsrSmciTend) -> u8 {
        SsrSmciTend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrTdre {
    #[doc = "Transmit data is in TDR register"]
    _0 = 0x0,
    #[doc = "No transmit data is in TDR register"]
    _1 = 0x01,
}
impl SsrTdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrTdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrTdre {
    #[inline(always)]
    fn from(val: u8) -> SsrTdre {
        SsrTdre::from_bits(val)
    }
}
impl From<SsrTdre> for u8 {
    #[inline(always)]
    fn from(val: SsrTdre) -> u8 {
        SsrTdre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrTend {
    #[doc = "A character is being transmitted."]
    _0 = 0x0,
    #[doc = "Character transfer has been completed."]
    _1 = 0x01,
}
impl SsrTend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrTend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrTend {
    #[inline(always)]
    fn from(val: u8) -> SsrTend {
        SsrTend::from_bits(val)
    }
}
impl From<SsrTend> for u8 {
    #[inline(always)]
    fn from(val: SsrTend) -> u8 {
        SsrTend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stop {
    #[doc = "1 stop bit"]
    _0 = 0x0,
    #[doc = "2 stop bits"]
    _1 = 0x01,
}
impl Stop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stop {
    #[inline(always)]
    fn from(val: u8) -> Stop {
        Stop::from_bits(val)
    }
}
impl From<Stop> for u8 {
    #[inline(always)]
    fn from(val: Stop) -> u8 {
        Stop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdfe {
    #[doc = "The quantity of transmit data written in FTDR exceeds the specified transmit triggering number."]
    _0 = 0x0,
    #[doc = "The quantity of transmit data written in FTDR is equal to or less than the specified transmit triggering number"]
    _1 = 0x01,
}
impl Tdfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdfe {
    #[inline(always)]
    fn from(val: u8) -> Tdfe {
        Tdfe::from_bits(val)
    }
}
impl From<Tdfe> for u8 {
    #[inline(always)]
    fn from(val: Tdfe) -> u8 {
        Tdfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Teie {
    #[doc = "SCI_TEI interrupt request is disabled"]
    _0 = 0x0,
    #[doc = "SCI_TEI interrupt request is enabled"]
    _1 = 0x01,
}
impl Teie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Teie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Teie {
    #[inline(always)]
    fn from(val: u8) -> Teie {
        Teie::from_bits(val)
    }
}
impl From<Teie> for u8 {
    #[inline(always)]
    fn from(val: Teie) -> u8 {
        Teie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfrst {
    #[doc = "The number of data stored in FTDRH and FTDRL register are NOT made 0"]
    _0 = 0x0,
    #[doc = "The number of data stored in FTDRH and FTDRL register are made 0"]
    _1 = 0x01,
}
impl Tfrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfrst {
    #[inline(always)]
    fn from(val: u8) -> Tfrst {
        Tfrst::from_bits(val)
    }
}
impl From<Tfrst> for u8 {
    #[inline(always)]
    fn from(val: Tfrst) -> u8 {
        Tfrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ttrg {
    #[doc = "Trigger number 0"]
    _0000 = 0x0,
    _RESERVED_1 = 0x01,
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
impl Ttrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ttrg {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ttrg {
    #[inline(always)]
    fn from(val: u8) -> Ttrg {
        Ttrg::from_bits(val)
    }
}
impl From<Ttrg> for u8 {
    #[inline(always)]
    fn from(val: Ttrg) -> u8 {
        Ttrg::to_bits(val)
    }
}
