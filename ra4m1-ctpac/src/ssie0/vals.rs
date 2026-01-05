#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aucke {
    #[doc = "Disables supply of AUDIO_MCK"]
    _0 = 0x0,
    #[doc = "Enables supply of AUDIO_MCK."]
    _1 = 0x01,
}
impl Aucke {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aucke {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aucke {
    #[inline(always)]
    fn from(val: u8) -> Aucke {
        Aucke::from_bits(val)
    }
}
impl From<Aucke> for u8 {
    #[inline(always)]
    fn from(val: Aucke) -> u8 {
        Aucke::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bckastp {
    #[doc = "Always outputs BCK to the SSIBCK pin"]
    _0 = 0x0,
    #[doc = "Automatically controls output of BCK to the SSIBCK pin."]
    _1 = 0x01,
}
impl Bckastp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bckastp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bckastp {
    #[inline(always)]
    fn from(val: u8) -> Bckastp {
        Bckastp::from_bits(val)
    }
}
impl From<Bckastp> for u8 {
    #[inline(always)]
    fn from(val: Bckastp) -> u8 {
        Bckastp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bckp {
    #[doc = "SSILRCK/SSIFS and SSITXD0/SSIRXD0 change at a falling edge (SSILRCK/SSIFS and SSIRXD0 are sampled at a rising edge of SSIBCK)"]
    _0 = 0x0,
    #[doc = "SSILRCK/SSIFS and SSITXD0/SSIRXD0 change at a rising edge (SSILRCK/SSIFS and SSIRXD0 are sampled at a falling edge of SSIBCK)."]
    _1 = 0x01,
}
impl Bckp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bckp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bckp {
    #[inline(always)]
    fn from(val: u8) -> Bckp {
        Bckp::from_bits(val)
    }
}
impl From<Bckp> for u8 {
    #[inline(always)]
    fn from(val: Bckp) -> u8 {
        Bckp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bsw {
    #[doc = "Disables byte swap"]
    _0 = 0x0,
    #[doc = "Enables byte swap"]
    _1 = 0x01,
}
impl Bsw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bsw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bsw {
    #[inline(always)]
    fn from(val: u8) -> Bsw {
        Bsw::from_bits(val)
    }
}
impl From<Bsw> for u8 {
    #[inline(always)]
    fn from(val: Bsw) -> u8 {
        Bsw::to_bits(val)
    }
}
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
pub enum Cks {
    #[doc = "Selects the AUDIO_CLK input"]
    _0 = 0x0,
    #[doc = "Selects the GTIOC1A (GPT output)."]
    _1 = 0x01,
}
impl Cks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cks {
        unsafe { core::mem::transmute(val & 0x01) }
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
pub enum Del {
    #[doc = "Delay of one cycle of SSIBCK between SSILRCK/SSIFS and SSITXD0/SSIRXD0"]
    _0 = 0x0,
    #[doc = "No delay between SSILRCK/SSIFS and SSITXD0/SSIRXD0 In the monaural format, this bit controls the waveform of SSILRCK/SSIFS."]
    _1 = 0x01,
}
impl Del {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Del {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Del {
    #[inline(always)]
    fn from(val: u8) -> Del {
        Del::from_bits(val)
    }
}
impl From<Del> for u8 {
    #[inline(always)]
    fn from(val: Del) -> u8 {
        Del::to_bits(val)
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
pub enum Iien {
    #[doc = "Disables idle mode interrupt output"]
    _0 = 0x0,
    #[doc = "Enables idle mode interrupt output."]
    _1 = 0x01,
}
impl Iien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iien {
    #[inline(always)]
    fn from(val: u8) -> Iien {
        Iien::from_bits(val)
    }
}
impl From<Iien> for u8 {
    #[inline(always)]
    fn from(val: Iien) -> u8 {
        Iien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iirq {
    #[doc = "In the communication state"]
    _0 = 0x0,
    #[doc = "In the idle state"]
    _1 = 0x01,
}
impl Iirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iirq {
    #[inline(always)]
    fn from(val: u8) -> Iirq {
        Iirq::from_bits(val)
    }
}
impl From<Iirq> for u8 {
    #[inline(always)]
    fn from(val: Iirq) -> u8 {
        Iirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lrckp {
    #[doc = "The initial value is at a high level The start trigger for a frame is synchronized with a falling edge of SSILRCK/SSIFS"]
    _0 = 0x0,
    #[doc = "The initial value is at a low level The start trigger for a frame is synchronized with a rising edge of SSILRCK/SSIFS."]
    _1 = 0x01,
}
impl Lrckp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lrckp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lrckp {
    #[inline(always)]
    fn from(val: u8) -> Lrckp {
        Lrckp::from_bits(val)
    }
}
impl From<Lrckp> for u8 {
    #[inline(always)]
    fn from(val: Lrckp) -> u8 {
        Lrckp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lrcont {
    #[doc = "Disables LRCK/FS continuation"]
    _0 = 0x0,
    #[doc = "Enables LRCK/FS continuation."]
    _1 = 0x01,
}
impl Lrcont {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lrcont {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lrcont {
    #[inline(always)]
    fn from(val: u8) -> Lrcont {
        Lrcont::from_bits(val)
    }
}
impl From<Lrcont> for u8 {
    #[inline(always)]
    fn from(val: Lrcont) -> u8 {
        Lrcont::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mst {
    #[doc = "Slave-mode communication"]
    _0 = 0x0,
    #[doc = "Master-mode communication."]
    _1 = 0x01,
}
impl Mst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mst {
    #[inline(always)]
    fn from(val: u8) -> Mst {
        Mst::from_bits(val)
    }
}
impl From<Mst> for u8 {
    #[inline(always)]
    fn from(val: Mst) -> u8 {
        Mst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Muen {
    #[doc = "Disables muting on the next frame boundary"]
    _0 = 0x0,
    #[doc = "Enables muting on the next frame boundary."]
    _1 = 0x01,
}
impl Muen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Muen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Muen {
    #[inline(always)]
    fn from(val: u8) -> Muen {
        Muen::from_bits(val)
    }
}
impl From<Muen> for u8 {
    #[inline(always)]
    fn from(val: Muen) -> u8 {
        Muen::to_bits(val)
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
pub enum Pdta {
    #[doc = "Left-justifies placement data (SSIFTDR, SSIFRDR)"]
    _0 = 0x0,
    #[doc = "Right-justifies placement data (SSIFTDR, SSIFRDR)."]
    _1 = 0x01,
}
impl Pdta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdta {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdta {
    #[inline(always)]
    fn from(val: u8) -> Pdta {
        Pdta::from_bits(val)
    }
}
impl From<Pdta> for u8 {
    #[inline(always)]
    fn from(val: Pdta) -> u8 {
        Pdta::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdf {
    #[doc = "The size of received data in SSIFRDR is not more than the value of SSISCR.RDFS"]
    _0 = 0x0,
    #[doc = "The size of received data in SSIFRDR is not less than the value of SSISCR.RDFS plus one."]
    _1 = 0x01,
}
impl Rdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdf {
    #[inline(always)]
    fn from(val: u8) -> Rdf {
        Rdf::from_bits(val)
    }
}
impl From<Rdf> for u8 {
    #[inline(always)]
    fn from(val: Rdf) -> u8 {
        Rdf::to_bits(val)
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
pub enum Ren {
    #[doc = "Disables the receive operation."]
    _0 = 0x0,
    #[doc = "Enables the receive operation."]
    _1 = 0x01,
}
impl Ren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ren {
    #[inline(always)]
    fn from(val: u8) -> Ren {
        Ren::from_bits(val)
    }
}
impl From<Ren> for u8 {
    #[inline(always)]
    fn from(val: Ren) -> u8 {
        Ren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfrst {
    #[doc = "Clears a receive data FIFO reset condition"]
    _0 = 0x0,
    #[doc = "Sets a receive data FIFO reset condition."]
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
pub enum Rie {
    #[doc = "Disables receive data full interrupts"]
    _0 = 0x0,
    #[doc = "Enables receive data full interrupts."]
    _1 = 0x01,
}
impl Rie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rie {
    #[inline(always)]
    fn from(val: u8) -> Rie {
        Rie::from_bits(val)
    }
}
impl From<Rie> for u8 {
    #[inline(always)]
    fn from(val: Rie) -> u8 {
        Rie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Roien {
    #[doc = "Disables receive overflow interrupt output"]
    _0 = 0x0,
    #[doc = "Enables receive overflow interrupt output."]
    _1 = 0x01,
}
impl Roien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Roien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Roien {
    #[inline(always)]
    fn from(val: u8) -> Roien {
        Roien::from_bits(val)
    }
}
impl From<Roien> for u8 {
    #[inline(always)]
    fn from(val: Roien) -> u8 {
        Roien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Roirq {
    #[doc = "No receive overflow error is generated"]
    _0 = 0x0,
    #[doc = "A receive overflow error is generated."]
    _1 = 0x01,
}
impl Roirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Roirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Roirq {
    #[inline(always)]
    fn from(val: u8) -> Roirq {
        Roirq::from_bits(val)
    }
}
impl From<Roirq> for u8 {
    #[inline(always)]
    fn from(val: Roirq) -> u8 {
        Roirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ruien {
    #[doc = "Disables receive underflow interrupt output"]
    _0 = 0x0,
    #[doc = "Enables receive underflow interrupt output."]
    _1 = 0x01,
}
impl Ruien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ruien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ruien {
    #[inline(always)]
    fn from(val: u8) -> Ruien {
        Ruien::from_bits(val)
    }
}
impl From<Ruien> for u8 {
    #[inline(always)]
    fn from(val: Ruien) -> u8 {
        Ruien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ruirq {
    #[doc = "No receive underflow error is generated"]
    _0 = 0x0,
    #[doc = "A receive underflow error is generated."]
    _1 = 0x01,
}
impl Ruirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ruirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ruirq {
    #[inline(always)]
    fn from(val: u8) -> Ruirq {
        Ruirq::from_bits(val)
    }
}
impl From<Ruirq> for u8 {
    #[inline(always)]
    fn from(val: Ruirq) -> u8 {
        Ruirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdta {
    #[doc = "Transmits and receives serial data first and then padding bits"]
    _0 = 0x0,
    #[doc = "Transmit and receives padding bits first and then serial data."]
    _1 = 0x01,
}
impl Sdta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdta {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdta {
    #[inline(always)]
    fn from(val: u8) -> Sdta {
        Sdta::from_bits(val)
    }
}
impl From<Sdta> for u8 {
    #[inline(always)]
    fn from(val: Sdta) -> u8 {
        Sdta::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdp {
    #[doc = "Padding data is at a low level"]
    _0 = 0x0,
    #[doc = "Padding data is at a high level."]
    _1 = 0x01,
}
impl Spdp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdp {
    #[inline(always)]
    fn from(val: u8) -> Spdp {
        Spdp::from_bits(val)
    }
}
impl From<Spdp> for u8 {
    #[inline(always)]
    fn from(val: Spdp) -> u8 {
        Spdp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssirst {
    #[doc = "Clears a software reset condition"]
    _0 = 0x0,
    #[doc = "Sets a software reset condition."]
    _1 = 0x01,
}
impl Ssirst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssirst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssirst {
    #[inline(always)]
    fn from(val: u8) -> Ssirst {
        Ssirst::from_bits(val)
    }
}
impl From<Ssirst> for u8 {
    #[inline(always)]
    fn from(val: Ssirst) -> u8 {
        Ssirst::to_bits(val)
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
pub enum Tde {
    #[doc = "The free space of SSIFTDR is not more than the value of SSISCR.TDES"]
    _0 = 0x0,
    #[doc = "The free space of SSIFTDR is not less than the value of SSISCR.TDES plus one."]
    _1 = 0x01,
}
impl Tde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tde {
    #[inline(always)]
    fn from(val: u8) -> Tde {
        Tde::from_bits(val)
    }
}
impl From<Tde> for u8 {
    #[inline(always)]
    fn from(val: Tde) -> u8 {
        Tde::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ten {
    #[doc = "Disables the transmit operation."]
    _0 = 0x0,
    #[doc = "Enables the transmit operation."]
    _1 = 0x01,
}
impl Ten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ten {
    #[inline(always)]
    fn from(val: u8) -> Ten {
        Ten::from_bits(val)
    }
}
impl From<Ten> for u8 {
    #[inline(always)]
    fn from(val: Ten) -> u8 {
        Ten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfrst {
    #[doc = "Clears a transmit data FIFO reset condition"]
    _0 = 0x0,
    #[doc = "Sets a transmit data FIFO reset condition."]
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
pub enum Tie {
    #[doc = "Disables transmit data empty interrupts"]
    _0 = 0x0,
    #[doc = "Enables transmit data empty interrupts."]
    _1 = 0x01,
}
impl Tie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie {
    #[inline(always)]
    fn from(val: u8) -> Tie {
        Tie::from_bits(val)
    }
}
impl From<Tie> for u8 {
    #[inline(always)]
    fn from(val: Tie) -> u8 {
        Tie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Toien {
    #[doc = "Disables transmit overflow interrupt output"]
    _0 = 0x0,
    #[doc = "Enables transmit overflow interrupt output."]
    _1 = 0x01,
}
impl Toien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Toien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Toien {
    #[inline(always)]
    fn from(val: u8) -> Toien {
        Toien::from_bits(val)
    }
}
impl From<Toien> for u8 {
    #[inline(always)]
    fn from(val: Toien) -> u8 {
        Toien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Toirq {
    #[doc = "No transmit overflow error is generated"]
    _0 = 0x0,
    #[doc = "A transmit overflow error is generated."]
    _1 = 0x01,
}
impl Toirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Toirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Toirq {
    #[inline(always)]
    fn from(val: u8) -> Toirq {
        Toirq::from_bits(val)
    }
}
impl From<Toirq> for u8 {
    #[inline(always)]
    fn from(val: Toirq) -> u8 {
        Toirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tuien {
    #[doc = "Disables transmit underflow interrupt output"]
    _0 = 0x0,
    #[doc = "Enables transmit underflow interrupt output."]
    _1 = 0x01,
}
impl Tuien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tuien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tuien {
    #[inline(always)]
    fn from(val: u8) -> Tuien {
        Tuien::from_bits(val)
    }
}
impl From<Tuien> for u8 {
    #[inline(always)]
    fn from(val: Tuien) -> u8 {
        Tuien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tuirq {
    #[doc = "No transmit underflow error is generated"]
    _0 = 0x0,
    #[doc = "A transmit underflow error is generated."]
    _1 = 0x01,
}
impl Tuirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tuirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tuirq {
    #[inline(always)]
    fn from(val: u8) -> Tuirq {
        Tuirq::from_bits(val)
    }
}
impl From<Tuirq> for u8 {
    #[inline(always)]
    fn from(val: Tuirq) -> u8 {
        Tuirq::to_bits(val)
    }
}
