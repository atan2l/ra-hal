#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Atrepm {
    #[doc = "Auto response disabled."]
    _0 = 0x0,
    #[doc = "Auto response enabled."]
    _1 = 0x01,
}
impl Atrepm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Atrepm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Atrepm {
    #[inline(always)]
    fn from(val: u8) -> Atrepm {
        Atrepm::from_bits(val)
    }
}
impl From<Atrepm> for u8 {
    #[inline(always)]
    fn from(val: Atrepm) -> u8 {
        Atrepm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Attch {
    #[doc = "ATTCH interrupts are not generated."]
    _0 = 0x0,
    #[doc = "ATTCH interrupts are generated."]
    _1 = 0x01,
}
impl Attch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Attch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Attch {
    #[inline(always)]
    fn from(val: u8) -> Attch {
        Attch::from_bits(val)
    }
}
impl From<Attch> for u8 {
    #[inline(always)]
    fn from(val: Attch) -> u8 {
        Attch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Attche {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Attche {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Attche {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Attche {
    #[inline(always)]
    fn from(val: u8) -> Attche {
        Attche::from_bits(val)
    }
}
impl From<Attche> for u8 {
    #[inline(always)]
    fn from(val: Attche) -> u8 {
        Attche::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Batchge0 {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled"]
    _1 = 0x01,
}
impl Batchge0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Batchge0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Batchge0 {
    #[inline(always)]
    fn from(val: u8) -> Batchge0 {
        Batchge0::from_bits(val)
    }
}
impl From<Batchge0> for u8 {
    #[inline(always)]
    fn from(val: Batchge0) -> u8 {
        Batchge0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bchg {
    #[doc = "BCHG interrupts are not generated."]
    _0 = 0x0,
    #[doc = "BCHG interrupts are generated."]
    _1 = 0x01,
}
impl Bchg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bchg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bchg {
    #[inline(always)]
    fn from(val: u8) -> Bchg {
        Bchg::from_bits(val)
    }
}
impl From<Bchg> for u8 {
    #[inline(always)]
    fn from(val: Bchg) -> u8 {
        Bchg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bchge {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Bchge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bchge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bchge {
    #[inline(always)]
    fn from(val: u8) -> Bchge {
        Bchge::from_bits(val)
    }
}
impl From<Bchge> for u8 {
    #[inline(always)]
    fn from(val: Bchge) -> u8 {
        Bchge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bemp {
    #[doc = "BEMP interrupts are not generated."]
    _0 = 0x0,
    #[doc = "BEMP interrupts are generated."]
    _1 = 0x01,
}
impl Bemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bemp {
    #[inline(always)]
    fn from(val: u8) -> Bemp {
        Bemp::from_bits(val)
    }
}
impl From<Bemp> for u8 {
    #[inline(always)]
    fn from(val: Bemp) -> u8 {
        Bemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bempe {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Bempe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bempe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bempe {
    #[inline(always)]
    fn from(val: u8) -> Bempe {
        Bempe::from_bits(val)
    }
}
impl From<Bempe> for u8 {
    #[inline(always)]
    fn from(val: Bempe) -> u8 {
        Bempe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfre {
    #[doc = "BRDY interrupt upon transmitting or receiving data"]
    _0 = 0x0,
    #[doc = "BRDY interrupt upon completion of reading data"]
    _1 = 0x01,
}
impl Bfre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfre {
    #[inline(always)]
    fn from(val: u8) -> Bfre {
        Bfre::from_bits(val)
    }
}
impl From<Bfre> for u8 {
    #[inline(always)]
    fn from(val: Bfre) -> u8 {
        Bfre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brdy {
    #[doc = "BRDY interrupts are not generated."]
    _0 = 0x0,
    #[doc = "BRDY interrupts are generated."]
    _1 = 0x01,
}
impl Brdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brdy {
    #[inline(always)]
    fn from(val: u8) -> Brdy {
        Brdy::from_bits(val)
    }
}
impl From<Brdy> for u8 {
    #[inline(always)]
    fn from(val: Brdy) -> u8 {
        Brdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Brdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brdye {
    #[inline(always)]
    fn from(val: u8) -> Brdye {
        Brdye::from_bits(val)
    }
}
impl From<Brdye> for u8 {
    #[inline(always)]
    fn from(val: Brdye) -> u8 {
        Brdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brdym {
    #[doc = "BRDY flag cleared by software"]
    _0 = 0x0,
    #[doc = "BRDY flag cleared by the USBFS through a data read from the FIFO buffer or data write to the FIFO buffer."]
    _1 = 0x01,
}
impl Brdym {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brdym {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brdym {
    #[inline(always)]
    fn from(val: u8) -> Brdym {
        Brdym::from_bits(val)
    }
}
impl From<Brdym> for u8 {
    #[inline(always)]
    fn from(val: Brdym) -> u8 {
        Brdym::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccpl {
    #[doc = "Invalid"]
    _0 = 0x0,
    #[doc = "Completion of control transfer is enabled."]
    _1 = 0x01,
}
impl Ccpl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccpl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccpl {
    #[inline(always)]
    fn from(val: u8) -> Ccpl {
        Ccpl::from_bits(val)
    }
}
impl From<Ccpl> for u8 {
    #[inline(always)]
    fn from(val: Ccpl) -> u8 {
        Ccpl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CfifoctrBclr {
    #[doc = "Does not operate"]
    _0 = 0x0,
    #[doc = "FIFO buffer cleared on the CPU side."]
    _1 = 0x01,
}
impl CfifoctrBclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CfifoctrBclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CfifoctrBclr {
    #[inline(always)]
    fn from(val: u8) -> CfifoctrBclr {
        CfifoctrBclr::from_bits(val)
    }
}
impl From<CfifoctrBclr> for u8 {
    #[inline(always)]
    fn from(val: CfifoctrBclr) -> u8 {
        CfifoctrBclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CfifoctrBval {
    #[doc = "Invalid"]
    _0 = 0x0,
    #[doc = "Writing ended"]
    _1 = 0x01,
}
impl CfifoctrBval {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CfifoctrBval {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CfifoctrBval {
    #[inline(always)]
    fn from(val: u8) -> CfifoctrBval {
        CfifoctrBval::from_bits(val)
    }
}
impl From<CfifoctrBval> for u8 {
    #[inline(always)]
    fn from(val: CfifoctrBval) -> u8 {
        CfifoctrBval::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CfifoctrFrdy {
    #[doc = "FIFO port access is disabled."]
    _0 = 0x0,
    #[doc = "FIFO port access is enabled."]
    _1 = 0x01,
}
impl CfifoctrFrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CfifoctrFrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CfifoctrFrdy {
    #[inline(always)]
    fn from(val: u8) -> CfifoctrFrdy {
        CfifoctrFrdy::from_bits(val)
    }
}
impl From<CfifoctrFrdy> for u8 {
    #[inline(always)]
    fn from(val: CfifoctrFrdy) -> u8 {
        CfifoctrFrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CfifoselBigend {
    #[doc = "Little endian"]
    _0 = 0x0,
    #[doc = "Big endian"]
    _1 = 0x01,
}
impl CfifoselBigend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CfifoselBigend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CfifoselBigend {
    #[inline(always)]
    fn from(val: u8) -> CfifoselBigend {
        CfifoselBigend::from_bits(val)
    }
}
impl From<CfifoselBigend> for u8 {
    #[inline(always)]
    fn from(val: CfifoselBigend) -> u8 {
        CfifoselBigend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CfifoselCurpipe {
    #[doc = "DCP (Default control pipe)"]
    _0000 = 0x0,
    #[doc = "Pipe 1"]
    _0001 = 0x01,
    #[doc = "Pipe 2"]
    _0010 = 0x02,
    #[doc = "Pipe 3"]
    _0011 = 0x03,
    #[doc = "Pipe 4"]
    _0100 = 0x04,
    #[doc = "Pipe 5"]
    _0101 = 0x05,
    #[doc = "Pipe 6"]
    _0110 = 0x06,
    #[doc = "Pipe 7"]
    _0111 = 0x07,
    #[doc = "Pipe 8"]
    _1000 = 0x08,
    #[doc = "Pipe 9"]
    _1001 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CfifoselCurpipe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CfifoselCurpipe {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CfifoselCurpipe {
    #[inline(always)]
    fn from(val: u8) -> CfifoselCurpipe {
        CfifoselCurpipe::from_bits(val)
    }
}
impl From<CfifoselCurpipe> for u8 {
    #[inline(always)]
    fn from(val: CfifoselCurpipe) -> u8 {
        CfifoselCurpipe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CfifoselMbw {
    #[doc = "8-bit width"]
    _0 = 0x0,
    #[doc = "16-bit width"]
    _1 = 0x01,
}
impl CfifoselMbw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CfifoselMbw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CfifoselMbw {
    #[inline(always)]
    fn from(val: u8) -> CfifoselMbw {
        CfifoselMbw::from_bits(val)
    }
}
impl From<CfifoselMbw> for u8 {
    #[inline(always)]
    fn from(val: CfifoselMbw) -> u8 {
        CfifoselMbw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CfifoselRcnt {
    #[doc = "The DTLN\\[8:0\\] bits (CFIFOCRT.DTLN\\[8:0\\], D0FIFOCRT.DTLN\\[8:0\\], D1FIFOCRT.DTLN\\[8:0\\]) are cleared when all of the receive data has been read from the CFIFO.(In double buffer mode, the DTLN\\[8:0\\] bit value is cleared when all the data has been read from only a single plane.)"]
    _0 = 0x0,
    #[doc = "The DTLN\\[8:0\\] bits are decremented each time the receive data is read from the CFIFO."]
    _1 = 0x01,
}
impl CfifoselRcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CfifoselRcnt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CfifoselRcnt {
    #[inline(always)]
    fn from(val: u8) -> CfifoselRcnt {
        CfifoselRcnt::from_bits(val)
    }
}
impl From<CfifoselRcnt> for u8 {
    #[inline(always)]
    fn from(val: CfifoselRcnt) -> u8 {
        CfifoselRcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CfifoselRew {
    #[doc = "The buffer pointer is not rewound."]
    _0 = 0x0,
    #[doc = "The buffer pointer is rewound."]
    _1 = 0x01,
}
impl CfifoselRew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CfifoselRew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CfifoselRew {
    #[inline(always)]
    fn from(val: u8) -> CfifoselRew {
        CfifoselRew::from_bits(val)
    }
}
impl From<CfifoselRew> for u8 {
    #[inline(always)]
    fn from(val: CfifoselRew) -> u8 {
        CfifoselRew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chgdetsts0 {
    #[doc = "Not detected"]
    _0 = 0x0,
    #[doc = "Detected"]
    _1 = 0x01,
}
impl Chgdetsts0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chgdetsts0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chgdetsts0 {
    #[inline(always)]
    fn from(val: u8) -> Chgdetsts0 {
        Chgdetsts0::from_bits(val)
    }
}
impl From<Chgdetsts0> for u8 {
    #[inline(always)]
    fn from(val: Chgdetsts0) -> u8 {
        Chgdetsts0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cnen {
    #[doc = "Single end receiver disabled"]
    _0 = 0x0,
    #[doc = "Single end receiver enabled"]
    _1 = 0x01,
}
impl Cnen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cnen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cnen {
    #[inline(always)]
    fn from(val: u8) -> Cnen {
        Cnen::from_bits(val)
    }
}
impl From<Cnen> for u8 {
    #[inline(always)]
    fn from(val: Cnen) -> u8 {
        Cnen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crce {
    #[doc = "No error"]
    _0 = 0x0,
    #[doc = "An error occurred"]
    _1 = 0x01,
}
impl Crce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crce {
    #[inline(always)]
    fn from(val: u8) -> Crce {
        Crce::from_bits(val)
    }
}
impl From<Crce> for u8 {
    #[inline(always)]
    fn from(val: Crce) -> u8 {
        Crce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctre {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Ctre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctre {
    #[inline(always)]
    fn from(val: u8) -> Ctre {
        Ctre::from_bits(val)
    }
}
impl From<Ctre> for u8 {
    #[inline(always)]
    fn from(val: Ctre) -> u8 {
        Ctre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrt {
    #[doc = "Control transfer stage transition interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Control transfer stage transition interrupts are generated."]
    _1 = 0x01,
}
impl Ctrt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrt {
    #[inline(always)]
    fn from(val: u8) -> Ctrt {
        Ctrt::from_bits(val)
    }
}
impl From<Ctrt> for u8 {
    #[inline(always)]
    fn from(val: Ctrt) -> u8 {
        Ctrt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsq {
    #[doc = "Idle or setup stage"]
    _000 = 0x0,
    #[doc = "Control read data stage"]
    _001 = 0x01,
    #[doc = "Control read status stage"]
    _010 = 0x02,
    #[doc = "Control write data stage"]
    _011 = 0x03,
    #[doc = "Control write status stage"]
    _100 = 0x04,
    #[doc = "Control write (no data) status stage"]
    _101 = 0x05,
    #[doc = "Control transfer sequence error"]
    _110 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctsq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsq {
    #[inline(always)]
    fn from(val: u8) -> Ctsq {
        Ctsq::from_bits(val)
    }
}
impl From<Ctsq> for u8 {
    #[inline(always)]
    fn from(val: Ctsq) -> u8 {
        Ctsq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D0fifoctrBclr {
    #[doc = "Does not operate"]
    _0 = 0x0,
    #[doc = "FIFO buffer cleared on the CPU side."]
    _1 = 0x01,
}
impl D0fifoctrBclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D0fifoctrBclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D0fifoctrBclr {
    #[inline(always)]
    fn from(val: u8) -> D0fifoctrBclr {
        D0fifoctrBclr::from_bits(val)
    }
}
impl From<D0fifoctrBclr> for u8 {
    #[inline(always)]
    fn from(val: D0fifoctrBclr) -> u8 {
        D0fifoctrBclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D0fifoctrBval {
    #[doc = "Invalid"]
    _0 = 0x0,
    #[doc = "Writing ended"]
    _1 = 0x01,
}
impl D0fifoctrBval {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D0fifoctrBval {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D0fifoctrBval {
    #[inline(always)]
    fn from(val: u8) -> D0fifoctrBval {
        D0fifoctrBval::from_bits(val)
    }
}
impl From<D0fifoctrBval> for u8 {
    #[inline(always)]
    fn from(val: D0fifoctrBval) -> u8 {
        D0fifoctrBval::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D0fifoctrFrdy {
    #[doc = "FIFO port access is disabled."]
    _0 = 0x0,
    #[doc = "FIFO port access is enabled."]
    _1 = 0x01,
}
impl D0fifoctrFrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D0fifoctrFrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D0fifoctrFrdy {
    #[inline(always)]
    fn from(val: u8) -> D0fifoctrFrdy {
        D0fifoctrFrdy::from_bits(val)
    }
}
impl From<D0fifoctrFrdy> for u8 {
    #[inline(always)]
    fn from(val: D0fifoctrFrdy) -> u8 {
        D0fifoctrFrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D0fifoselBigend {
    #[doc = "Little endian"]
    _0 = 0x0,
    #[doc = "Big endian"]
    _1 = 0x01,
}
impl D0fifoselBigend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D0fifoselBigend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D0fifoselBigend {
    #[inline(always)]
    fn from(val: u8) -> D0fifoselBigend {
        D0fifoselBigend::from_bits(val)
    }
}
impl From<D0fifoselBigend> for u8 {
    #[inline(always)]
    fn from(val: D0fifoselBigend) -> u8 {
        D0fifoselBigend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D0fifoselCurpipe {
    #[doc = "DCP (Default control pipe)"]
    _0000 = 0x0,
    #[doc = "Pipe 1"]
    _0001 = 0x01,
    #[doc = "Pipe 2"]
    _0010 = 0x02,
    #[doc = "Pipe 3"]
    _0011 = 0x03,
    #[doc = "Pipe 4"]
    _0100 = 0x04,
    #[doc = "Pipe 5"]
    _0101 = 0x05,
    #[doc = "Pipe 6"]
    _0110 = 0x06,
    #[doc = "Pipe 7"]
    _0111 = 0x07,
    #[doc = "Pipe 8"]
    _1000 = 0x08,
    #[doc = "Pipe 9"]
    _1001 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl D0fifoselCurpipe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D0fifoselCurpipe {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D0fifoselCurpipe {
    #[inline(always)]
    fn from(val: u8) -> D0fifoselCurpipe {
        D0fifoselCurpipe::from_bits(val)
    }
}
impl From<D0fifoselCurpipe> for u8 {
    #[inline(always)]
    fn from(val: D0fifoselCurpipe) -> u8 {
        D0fifoselCurpipe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D0fifoselDclrm {
    #[doc = "Auto buffer clear mode is disabled."]
    _0 = 0x0,
    #[doc = "Auto buffer clear mode is enabled."]
    _1 = 0x01,
}
impl D0fifoselDclrm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D0fifoselDclrm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D0fifoselDclrm {
    #[inline(always)]
    fn from(val: u8) -> D0fifoselDclrm {
        D0fifoselDclrm::from_bits(val)
    }
}
impl From<D0fifoselDclrm> for u8 {
    #[inline(always)]
    fn from(val: D0fifoselDclrm) -> u8 {
        D0fifoselDclrm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D0fifoselDreqe {
    #[doc = "DMA/DTC transfer request is disabled."]
    _0 = 0x0,
    #[doc = "DMA/DTC transfer request is enabled."]
    _1 = 0x01,
}
impl D0fifoselDreqe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D0fifoselDreqe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D0fifoselDreqe {
    #[inline(always)]
    fn from(val: u8) -> D0fifoselDreqe {
        D0fifoselDreqe::from_bits(val)
    }
}
impl From<D0fifoselDreqe> for u8 {
    #[inline(always)]
    fn from(val: D0fifoselDreqe) -> u8 {
        D0fifoselDreqe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D0fifoselMbw {
    #[doc = "8-bit width"]
    _0 = 0x0,
    #[doc = "16-bit width"]
    _1 = 0x01,
}
impl D0fifoselMbw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D0fifoselMbw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D0fifoselMbw {
    #[inline(always)]
    fn from(val: u8) -> D0fifoselMbw {
        D0fifoselMbw::from_bits(val)
    }
}
impl From<D0fifoselMbw> for u8 {
    #[inline(always)]
    fn from(val: D0fifoselMbw) -> u8 {
        D0fifoselMbw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D0fifoselRcnt {
    #[doc = "The DTLN\\[8:0\\] bits (CFIFOCRT.DTLN\\[8:0\\], D0FIFOCRT.DTLN\\[8:0\\], D1FIFOCRT.DTLN\\[8:0\\]) are cleared when all of the receive data has been read from the DnFIFO.(In double buffer mode, the DTLN bit Value is cleared when all the data has been read from only a single plane.)"]
    _0 = 0x0,
    #[doc = "The DTLN\\[8:0\\] bits are decremented each time the receive data is read from the DnFIFO. (n = 0, 1)"]
    _1 = 0x01,
}
impl D0fifoselRcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D0fifoselRcnt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D0fifoselRcnt {
    #[inline(always)]
    fn from(val: u8) -> D0fifoselRcnt {
        D0fifoselRcnt::from_bits(val)
    }
}
impl From<D0fifoselRcnt> for u8 {
    #[inline(always)]
    fn from(val: D0fifoselRcnt) -> u8 {
        D0fifoselRcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D0fifoselRew {
    #[doc = "The buffer pointer is not rewound."]
    _0 = 0x0,
    #[doc = "The buffer pointer is rewound."]
    _1 = 0x01,
}
impl D0fifoselRew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D0fifoselRew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D0fifoselRew {
    #[inline(always)]
    fn from(val: u8) -> D0fifoselRew {
        D0fifoselRew::from_bits(val)
    }
}
impl From<D0fifoselRew> for u8 {
    #[inline(always)]
    fn from(val: D0fifoselRew) -> u8 {
        D0fifoselRew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D1fifoctrBclr {
    #[doc = "Does not operate"]
    _0 = 0x0,
    #[doc = "FIFO buffer cleared on the CPU side."]
    _1 = 0x01,
}
impl D1fifoctrBclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D1fifoctrBclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D1fifoctrBclr {
    #[inline(always)]
    fn from(val: u8) -> D1fifoctrBclr {
        D1fifoctrBclr::from_bits(val)
    }
}
impl From<D1fifoctrBclr> for u8 {
    #[inline(always)]
    fn from(val: D1fifoctrBclr) -> u8 {
        D1fifoctrBclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D1fifoctrBval {
    #[doc = "Invalid"]
    _0 = 0x0,
    #[doc = "Writing ended"]
    _1 = 0x01,
}
impl D1fifoctrBval {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D1fifoctrBval {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D1fifoctrBval {
    #[inline(always)]
    fn from(val: u8) -> D1fifoctrBval {
        D1fifoctrBval::from_bits(val)
    }
}
impl From<D1fifoctrBval> for u8 {
    #[inline(always)]
    fn from(val: D1fifoctrBval) -> u8 {
        D1fifoctrBval::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D1fifoctrFrdy {
    #[doc = "FIFO port access is disabled."]
    _0 = 0x0,
    #[doc = "FIFO port access is enabled."]
    _1 = 0x01,
}
impl D1fifoctrFrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D1fifoctrFrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D1fifoctrFrdy {
    #[inline(always)]
    fn from(val: u8) -> D1fifoctrFrdy {
        D1fifoctrFrdy::from_bits(val)
    }
}
impl From<D1fifoctrFrdy> for u8 {
    #[inline(always)]
    fn from(val: D1fifoctrFrdy) -> u8 {
        D1fifoctrFrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D1fifoselBigend {
    #[doc = "Little endian"]
    _0 = 0x0,
    #[doc = "Big endian"]
    _1 = 0x01,
}
impl D1fifoselBigend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D1fifoselBigend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D1fifoselBigend {
    #[inline(always)]
    fn from(val: u8) -> D1fifoselBigend {
        D1fifoselBigend::from_bits(val)
    }
}
impl From<D1fifoselBigend> for u8 {
    #[inline(always)]
    fn from(val: D1fifoselBigend) -> u8 {
        D1fifoselBigend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D1fifoselCurpipe {
    #[doc = "DCP (Default control pipe)"]
    _0000 = 0x0,
    #[doc = "Pipe 1"]
    _0001 = 0x01,
    #[doc = "Pipe 2"]
    _0010 = 0x02,
    #[doc = "Pipe 3"]
    _0011 = 0x03,
    #[doc = "Pipe 4"]
    _0100 = 0x04,
    #[doc = "Pipe 5"]
    _0101 = 0x05,
    #[doc = "Pipe 6"]
    _0110 = 0x06,
    #[doc = "Pipe 7"]
    _0111 = 0x07,
    #[doc = "Pipe 8"]
    _1000 = 0x08,
    #[doc = "Pipe 9"]
    _1001 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl D1fifoselCurpipe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D1fifoselCurpipe {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D1fifoselCurpipe {
    #[inline(always)]
    fn from(val: u8) -> D1fifoselCurpipe {
        D1fifoselCurpipe::from_bits(val)
    }
}
impl From<D1fifoselCurpipe> for u8 {
    #[inline(always)]
    fn from(val: D1fifoselCurpipe) -> u8 {
        D1fifoselCurpipe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D1fifoselDclrm {
    #[doc = "Auto buffer clear mode is disabled."]
    _0 = 0x0,
    #[doc = "Auto buffer clear mode is enabled."]
    _1 = 0x01,
}
impl D1fifoselDclrm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D1fifoselDclrm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D1fifoselDclrm {
    #[inline(always)]
    fn from(val: u8) -> D1fifoselDclrm {
        D1fifoselDclrm::from_bits(val)
    }
}
impl From<D1fifoselDclrm> for u8 {
    #[inline(always)]
    fn from(val: D1fifoselDclrm) -> u8 {
        D1fifoselDclrm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D1fifoselDreqe {
    #[doc = "DMA/DTC transfer request is disabled."]
    _0 = 0x0,
    #[doc = "DMA/DTC transfer request is enabled."]
    _1 = 0x01,
}
impl D1fifoselDreqe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D1fifoselDreqe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D1fifoselDreqe {
    #[inline(always)]
    fn from(val: u8) -> D1fifoselDreqe {
        D1fifoselDreqe::from_bits(val)
    }
}
impl From<D1fifoselDreqe> for u8 {
    #[inline(always)]
    fn from(val: D1fifoselDreqe) -> u8 {
        D1fifoselDreqe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D1fifoselMbw {
    #[doc = "8-bit width"]
    _0 = 0x0,
    #[doc = "16-bit width"]
    _1 = 0x01,
}
impl D1fifoselMbw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D1fifoselMbw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D1fifoselMbw {
    #[inline(always)]
    fn from(val: u8) -> D1fifoselMbw {
        D1fifoselMbw::from_bits(val)
    }
}
impl From<D1fifoselMbw> for u8 {
    #[inline(always)]
    fn from(val: D1fifoselMbw) -> u8 {
        D1fifoselMbw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D1fifoselRcnt {
    #[doc = "The DTLN\\[8:0\\] bits (CFIFOCRT.DTLN\\[8:0\\], D0FIFOCRT.DTLN\\[8:0\\], D1FIFOCRT.DTLN\\[8:0\\]) are cleared when all of the receive data has been read from the DnFIFO.(In double buffer mode, the DTLN bit Value is cleared when all the data has been read from only a single plane.)"]
    _0 = 0x0,
    #[doc = "The DTLN\\[8:0\\] bits are decremented each time the receive data is read from the DnFIFO. (n = 0, 1)"]
    _1 = 0x01,
}
impl D1fifoselRcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D1fifoselRcnt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D1fifoselRcnt {
    #[inline(always)]
    fn from(val: u8) -> D1fifoselRcnt {
        D1fifoselRcnt::from_bits(val)
    }
}
impl From<D1fifoselRcnt> for u8 {
    #[inline(always)]
    fn from(val: D1fifoselRcnt) -> u8 {
        D1fifoselRcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D1fifoselRew {
    #[doc = "The buffer pointer is not rewound."]
    _0 = 0x0,
    #[doc = "The buffer pointer is rewound."]
    _1 = 0x01,
}
impl D1fifoselRew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D1fifoselRew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D1fifoselRew {
    #[inline(always)]
    fn from(val: u8) -> D1fifoselRew {
        D1fifoselRew::from_bits(val)
    }
}
impl From<D1fifoselRew> for u8 {
    #[inline(always)]
    fn from(val: D1fifoselRew) -> u8 {
        D1fifoselRew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dblb {
    #[doc = "Single buffer"]
    _0 = 0x0,
    #[doc = "Double buffer"]
    _1 = 0x01,
}
impl Dblb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dblb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dblb {
    #[inline(always)]
    fn from(val: u8) -> Dblb {
        Dblb::from_bits(val)
    }
}
impl From<Dblb> for u8 {
    #[inline(always)]
    fn from(val: Dblb) -> u8 {
        Dblb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcfm {
    #[doc = "Device controller selected"]
    _0 = 0x0,
    #[doc = "Host controller selected."]
    _1 = 0x01,
}
impl Dcfm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcfm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcfm {
    #[inline(always)]
    fn from(val: u8) -> Dcfm {
        Dcfm::from_bits(val)
    }
}
impl From<Dcfm> for u8 {
    #[inline(always)]
    fn from(val: Dcfm) -> u8 {
        Dcfm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcpcfgDir {
    #[doc = "Data receiving direction"]
    _0 = 0x0,
    #[doc = "Data transmitting direction"]
    _1 = 0x01,
}
impl DcpcfgDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcpcfgDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcpcfgDir {
    #[inline(always)]
    fn from(val: u8) -> DcpcfgDir {
        DcpcfgDir::from_bits(val)
    }
}
impl From<DcpcfgDir> for u8 {
    #[inline(always)]
    fn from(val: DcpcfgDir) -> u8 {
        DcpcfgDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcpcfgShtnak {
    #[doc = "Pipe continued at the end of transfer"]
    _0 = 0x0,
    #[doc = "Pipe disabled at the end of transfer"]
    _1 = 0x01,
}
impl DcpcfgShtnak {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcpcfgShtnak {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcpcfgShtnak {
    #[inline(always)]
    fn from(val: u8) -> DcpcfgShtnak {
        DcpcfgShtnak::from_bits(val)
    }
}
impl From<DcpcfgShtnak> for u8 {
    #[inline(always)]
    fn from(val: DcpcfgShtnak) -> u8 {
        DcpcfgShtnak::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcpctrBsts {
    #[doc = "Buffer access is disabled."]
    _0 = 0x0,
    #[doc = "Buffer access is enabled."]
    _1 = 0x01,
}
impl DcpctrBsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcpctrBsts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcpctrBsts {
    #[inline(always)]
    fn from(val: u8) -> DcpctrBsts {
        DcpctrBsts::from_bits(val)
    }
}
impl From<DcpctrBsts> for u8 {
    #[inline(always)]
    fn from(val: DcpctrBsts) -> u8 {
        DcpctrBsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcpctrPbusy {
    #[doc = "DCP is not used for the transaction."]
    _0 = 0x0,
    #[doc = "DCP is used for the transaction."]
    _1 = 0x01,
}
impl DcpctrPbusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcpctrPbusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcpctrPbusy {
    #[inline(always)]
    fn from(val: u8) -> DcpctrPbusy {
        DcpctrPbusy::from_bits(val)
    }
}
impl From<DcpctrPbusy> for u8 {
    #[inline(always)]
    fn from(val: DcpctrPbusy) -> u8 {
        DcpctrPbusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcpctrPid {
    #[doc = "NAK response"]
    _00 = 0x0,
    #[doc = "BUF response (depending on the buffer state)"]
    _01 = 0x01,
    #[doc = "STALL response"]
    _10 = 0x02,
    #[doc = "STALL response"]
    _11 = 0x03,
}
impl DcpctrPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcpctrPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcpctrPid {
    #[inline(always)]
    fn from(val: u8) -> DcpctrPid {
        DcpctrPid::from_bits(val)
    }
}
impl From<DcpctrPid> for u8 {
    #[inline(always)]
    fn from(val: DcpctrPid) -> u8 {
        DcpctrPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcpctrSqclr {
    #[doc = "Invalid"]
    _0 = 0x0,
    #[doc = "Specifies DATA0."]
    _1 = 0x01,
}
impl DcpctrSqclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcpctrSqclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcpctrSqclr {
    #[inline(always)]
    fn from(val: u8) -> DcpctrSqclr {
        DcpctrSqclr::from_bits(val)
    }
}
impl From<DcpctrSqclr> for u8 {
    #[inline(always)]
    fn from(val: DcpctrSqclr) -> u8 {
        DcpctrSqclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcpctrSqmon {
    #[doc = "DATA0"]
    _0 = 0x0,
    #[doc = "DATA1"]
    _1 = 0x01,
}
impl DcpctrSqmon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcpctrSqmon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcpctrSqmon {
    #[inline(always)]
    fn from(val: u8) -> DcpctrSqmon {
        DcpctrSqmon::from_bits(val)
    }
}
impl From<DcpctrSqmon> for u8 {
    #[inline(always)]
    fn from(val: DcpctrSqmon) -> u8 {
        DcpctrSqmon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcpctrSqset {
    #[doc = "Invalid"]
    _0 = 0x0,
    #[doc = "Specifies DATA1."]
    _1 = 0x01,
}
impl DcpctrSqset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcpctrSqset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcpctrSqset {
    #[inline(always)]
    fn from(val: u8) -> DcpctrSqset {
        DcpctrSqset::from_bits(val)
    }
}
impl From<DcpctrSqset> for u8 {
    #[inline(always)]
    fn from(val: DcpctrSqset) -> u8 {
        DcpctrSqset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcpmaxpDevsel {
    #[doc = "Address 0000"]
    _0000 = 0x0,
    #[doc = "Address 0001"]
    _0001 = 0x01,
    #[doc = "Address 0010"]
    _0010 = 0x02,
    #[doc = "Address 0011"]
    _0011 = 0x03,
    #[doc = "Address 0100"]
    _0100 = 0x04,
    #[doc = "Address 0101"]
    _0101 = 0x05,
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
impl DcpmaxpDevsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcpmaxpDevsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcpmaxpDevsel {
    #[inline(always)]
    fn from(val: u8) -> DcpmaxpDevsel {
        DcpmaxpDevsel::from_bits(val)
    }
}
impl From<DcpmaxpDevsel> for u8 {
    #[inline(always)]
    fn from(val: DcpmaxpDevsel) -> u8 {
        DcpmaxpDevsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmrpu {
    #[doc = "Line pull-up disabled"]
    _0 = 0x0,
    #[doc = "Line pull-up enabled."]
    _1 = 0x01,
}
impl Dmrpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmrpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmrpu {
    #[inline(always)]
    fn from(val: u8) -> Dmrpu {
        Dmrpu::from_bits(val)
    }
}
impl From<Dmrpu> for u8 {
    #[inline(always)]
    fn from(val: Dmrpu) -> u8 {
        Dmrpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dprpu {
    #[doc = "Line pull-down disabled"]
    _0 = 0x0,
    #[doc = "Line pull-down enabled."]
    _1 = 0x01,
}
impl Dprpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dprpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dprpu {
    #[inline(always)]
    fn from(val: u8) -> Dprpu {
        Dprpu::from_bits(val)
    }
}
impl From<Dprpu> for u8 {
    #[inline(always)]
    fn from(val: Dprpu) -> u8 {
        Dprpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Drpd {
    #[doc = "Line pull-down disabled"]
    _0 = 0x0,
    #[doc = "Line pull-down enabled."]
    _1 = 0x01,
}
impl Drpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Drpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Drpd {
    #[inline(always)]
    fn from(val: u8) -> Drpd {
        Drpd::from_bits(val)
    }
}
impl From<Drpd> for u8 {
    #[inline(always)]
    fn from(val: Drpd) -> u8 {
        Drpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtch {
    #[doc = "DTCH interrupts are not generated."]
    _0 = 0x0,
    #[doc = "DTCH interrupts are generated."]
    _1 = 0x01,
}
impl Dtch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtch {
    #[inline(always)]
    fn from(val: u8) -> Dtch {
        Dtch::from_bits(val)
    }
}
impl From<Dtch> for u8 {
    #[inline(always)]
    fn from(val: Dtch) -> u8 {
        Dtch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtche {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Dtche {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtche {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtche {
    #[inline(always)]
    fn from(val: u8) -> Dtche {
        Dtche::from_bits(val)
    }
}
impl From<Dtche> for u8 {
    #[inline(always)]
    fn from(val: Dtche) -> u8 {
        Dtche::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dvse {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Dvse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dvse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dvse {
    #[inline(always)]
    fn from(val: u8) -> Dvse {
        Dvse::from_bits(val)
    }
}
impl From<Dvse> for u8 {
    #[inline(always)]
    fn from(val: Dvse) -> u8 {
        Dvse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dvsq {
    #[doc = "Powered state"]
    _000 = 0x0,
    #[doc = "Default state"]
    _001 = 0x01,
    #[doc = "Address state"]
    _010 = 0x02,
    #[doc = "Configured state"]
    _011 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Dvsq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dvsq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dvsq {
    #[inline(always)]
    fn from(val: u8) -> Dvsq {
        Dvsq::from_bits(val)
    }
}
impl From<Dvsq> for u8 {
    #[inline(always)]
    fn from(val: Dvsq) -> u8 {
        Dvsq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dvst {
    #[doc = "Device state transition interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Device state transition interrupts are generated."]
    _1 = 0x01,
}
impl Dvst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dvst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dvst {
    #[inline(always)]
    fn from(val: u8) -> Dvst {
        Dvst::from_bits(val)
    }
}
impl From<Dvst> for u8 {
    #[inline(always)]
    fn from(val: Dvst) -> u8 {
        Dvst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edgests {
    #[doc = "before stopping the clock supply to the USB module"]
    _0 = 0x0,
    #[doc = "the edge interrupt output signal is in the middle of the edge processing"]
    _1 = 0x01,
}
impl Edgests {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edgests {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edgests {
    #[inline(always)]
    fn from(val: u8) -> Edgests {
        Edgests::from_bits(val)
    }
}
impl From<Edgests> for u8 {
    #[inline(always)]
    fn from(val: Edgests) -> u8 {
        Edgests::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eoferr {
    #[doc = "EOFERR interrupts are not generated."]
    _0 = 0x0,
    #[doc = "EOFERR interrupts are generated."]
    _1 = 0x01,
}
impl Eoferr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eoferr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eoferr {
    #[inline(always)]
    fn from(val: u8) -> Eoferr {
        Eoferr::from_bits(val)
    }
}
impl From<Eoferr> for u8 {
    #[inline(always)]
    fn from(val: Eoferr) -> u8 {
        Eoferr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eoferre {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Eoferre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eoferre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eoferre {
    #[inline(always)]
    fn from(val: u8) -> Eoferre {
        Eoferre::from_bits(val)
    }
}
impl From<Eoferre> for u8 {
    #[inline(always)]
    fn from(val: Eoferre) -> u8 {
        Eoferre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Exicen {
    #[doc = "External USB_EXICEN pin outputs low"]
    _0 = 0x0,
    #[doc = "External USB_EXICEN pin outputs high"]
    _1 = 0x01,
}
impl Exicen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Exicen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Exicen {
    #[inline(always)]
    fn from(val: u8) -> Exicen {
        Exicen::from_bits(val)
    }
}
impl From<Exicen> for u8 {
    #[inline(always)]
    fn from(val: Exicen) -> u8 {
        Exicen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hnpbtoa {
    #[doc = "Normal Operation"]
    _0 = 0x0,
    #[doc = "Switching from device B to device A is enabled"]
    _1 = 0x01,
}
impl Hnpbtoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hnpbtoa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hnpbtoa {
    #[inline(always)]
    fn from(val: u8) -> Hnpbtoa {
        Hnpbtoa::from_bits(val)
    }
}
impl From<Hnpbtoa> for u8 {
    #[inline(always)]
    fn from(val: Hnpbtoa) -> u8 {
        Hnpbtoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Htact {
    #[doc = "Host sequencer completely stopped"]
    _0 = 0x0,
    #[doc = "Host sequencer not completely stopped."]
    _1 = 0x01,
}
impl Htact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Htact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Htact {
    #[inline(always)]
    fn from(val: u8) -> Htact {
        Htact::from_bits(val)
    }
}
impl From<Htact> for u8 {
    #[inline(always)]
    fn from(val: Htact) -> u8 {
        Htact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idmon {
    #[doc = "USB0_ID pin is low"]
    _0 = 0x0,
    #[doc = "USB0_ID pin is high"]
    _1 = 0x01,
}
impl Idmon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idmon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idmon {
    #[inline(always)]
    fn from(val: u8) -> Idmon {
        Idmon::from_bits(val)
    }
}
impl From<Idmon> for u8 {
    #[inline(always)]
    fn from(val: Idmon) -> u8 {
        Idmon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idmsinke0 {
    #[doc = "Detection off"]
    _0 = 0x0,
    #[doc = "Detection on ( Comparator and sink current on )"]
    _1 = 0x01,
}
impl Idmsinke0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idmsinke0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idmsinke0 {
    #[inline(always)]
    fn from(val: u8) -> Idmsinke0 {
        Idmsinke0::from_bits(val)
    }
}
impl From<Idmsinke0> for u8 {
    #[inline(always)]
    fn from(val: Idmsinke0) -> u8 {
        Idmsinke0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idpsinke0 {
    #[doc = "Detection off"]
    _0 = 0x0,
    #[doc = "Detection on ( Comparator and sink current on )"]
    _1 = 0x01,
}
impl Idpsinke0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idpsinke0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idpsinke0 {
    #[inline(always)]
    fn from(val: u8) -> Idpsinke0 {
        Idpsinke0::from_bits(val)
    }
}
impl From<Idpsinke0> for u8 {
    #[inline(always)]
    fn from(val: Idpsinke0) -> u8 {
        Idpsinke0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idpsrce0 {
    #[doc = "Stop"]
    _0 = 0x0,
    #[doc = "10uA output"]
    _1 = 0x01,
}
impl Idpsrce0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idpsrce0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idpsrce0 {
    #[inline(always)]
    fn from(val: u8) -> Idpsrce0 {
        Idpsrce0::from_bits(val)
    }
}
impl From<Idpsrce0> for u8 {
    #[inline(always)]
    fn from(val: Idpsrce0) -> u8 {
        Idpsrce0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ifis {
    #[doc = "The buffer is not flushed."]
    _0 = 0x0,
    #[doc = "The buffer is flushed."]
    _1 = 0x01,
}
impl Ifis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ifis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ifis {
    #[inline(always)]
    fn from(val: u8) -> Ifis {
        Ifis::from_bits(val)
    }
}
impl From<Ifis> for u8 {
    #[inline(always)]
    fn from(val: Ifis) -> u8 {
        Ifis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inbufm {
    #[doc = "No data to be transmitted is in the FIFO buffer"]
    _0 = 0x0,
    #[doc = "Data to be transmitted is in the FIFO buffer"]
    _1 = 0x01,
}
impl Inbufm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inbufm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inbufm {
    #[inline(always)]
    fn from(val: u8) -> Inbufm {
        Inbufm::from_bits(val)
    }
}
impl From<Inbufm> for u8 {
    #[inline(always)]
    fn from(val: Inbufm) -> u8 {
        Inbufm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isel {
    #[doc = "Reading from the buffer memory is selected"]
    _0 = 0x0,
    #[doc = "Writing to the buffer memory is selected"]
    _1 = 0x01,
}
impl Isel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isel {
    #[inline(always)]
    fn from(val: u8) -> Isel {
        Isel::from_bits(val)
    }
}
impl From<Isel> for u8 {
    #[inline(always)]
    fn from(val: Isel) -> u8 {
        Isel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lnst {
    #[doc = "SE0"]
    _00 = 0x0,
    #[doc = "K-State (FS) / J-State(LS)"]
    _01 = 0x01,
    #[doc = "J-State(FS) / K-State(LS)"]
    _10 = 0x02,
    #[doc = "SE1"]
    _11 = 0x03,
}
impl Lnst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lnst {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lnst {
    #[inline(always)]
    fn from(val: u8) -> Lnst {
        Lnst::from_bits(val)
    }
}
impl From<Lnst> for u8 {
    #[inline(always)]
    fn from(val: Lnst) -> u8 {
        Lnst::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mxps(u8);
impl Mxps {
    #[doc = "8 bytes"]
    pub const _0X08: Self = Self(0x08);
    #[doc = "16 bytes"]
    pub const _0X10: Self = Self(0x10);
    #[doc = "24 bytes"]
    pub const _0X18: Self = Self(0x18);
    #[doc = "32 bytes"]
    pub const _0X20: Self = Self(0x20);
    #[doc = "40 bytes"]
    pub const _0X28: Self = Self(0x28);
    #[doc = "48 bytes"]
    pub const _0X30: Self = Self(0x30);
    #[doc = "56 bytes"]
    pub const _0X38: Self = Self(0x38);
    #[doc = "64 bytes"]
    pub const _0X40: Self = Self(0x40);
    #[doc = "72 bytes"]
    pub const _0X48: Self = Self(0x48);
    #[doc = "80 bytes"]
    pub const _0X50: Self = Self(0x50);
    #[doc = "88 bytes"]
    pub const _0X58: Self = Self(0x58);
    #[doc = "96 bytes"]
    pub const _0X60: Self = Self(0x60);
    #[doc = "104 bytes"]
    pub const _0X68: Self = Self(0x68);
    #[doc = "112 bytes"]
    pub const _0X70: Self = Self(0x70);
    #[doc = "120 bytes"]
    pub const _0X78: Self = Self(0x78);
}
impl Mxps {
    pub const fn from_bits(val: u8) -> Mxps {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Mxps {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x08 => f.write_str("_0X08"),
            0x10 => f.write_str("_0X10"),
            0x18 => f.write_str("_0X18"),
            0x20 => f.write_str("_0X20"),
            0x28 => f.write_str("_0X28"),
            0x30 => f.write_str("_0X30"),
            0x38 => f.write_str("_0X38"),
            0x40 => f.write_str("_0X40"),
            0x48 => f.write_str("_0X48"),
            0x50 => f.write_str("_0X50"),
            0x58 => f.write_str("_0X58"),
            0x60 => f.write_str("_0X60"),
            0x68 => f.write_str("_0X68"),
            0x70 => f.write_str("_0X70"),
            0x78 => f.write_str("_0X78"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mxps {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x08 => defmt::write!(f, "_0X08"),
            0x10 => defmt::write!(f, "_0X10"),
            0x18 => defmt::write!(f, "_0X18"),
            0x20 => defmt::write!(f, "_0X20"),
            0x28 => defmt::write!(f, "_0X28"),
            0x30 => defmt::write!(f, "_0X30"),
            0x38 => defmt::write!(f, "_0X38"),
            0x40 => defmt::write!(f, "_0X40"),
            0x48 => defmt::write!(f, "_0X48"),
            0x50 => defmt::write!(f, "_0X50"),
            0x58 => defmt::write!(f, "_0X58"),
            0x60 => defmt::write!(f, "_0X60"),
            0x68 => defmt::write!(f, "_0X68"),
            0x70 => defmt::write!(f, "_0X70"),
            0x78 => defmt::write!(f, "_0X78"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Mxps {
    #[inline(always)]
    fn from(val: u8) -> Mxps {
        Mxps::from_bits(val)
    }
}
impl From<Mxps> for u8 {
    #[inline(always)]
    fn from(val: Mxps) -> u8 {
        Mxps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nrdy {
    #[doc = "NRDY interrupts are not generated."]
    _0 = 0x0,
    #[doc = "NRDY interrupts are generated."]
    _1 = 0x01,
}
impl Nrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nrdy {
    #[inline(always)]
    fn from(val: u8) -> Nrdy {
        Nrdy::from_bits(val)
    }
}
impl From<Nrdy> for u8 {
    #[inline(always)]
    fn from(val: Nrdy) -> u8 {
        Nrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nrdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Nrdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nrdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nrdye {
    #[inline(always)]
    fn from(val: u8) -> Nrdye {
        Nrdye::from_bits(val)
    }
}
impl From<Nrdye> for u8 {
    #[inline(always)]
    fn from(val: Nrdye) -> u8 {
        Nrdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ovrcr {
    #[doc = "OVRCR interrupts are not generated."]
    _0 = 0x0,
    #[doc = "OVRCR interrupts are generated."]
    _1 = 0x01,
}
impl Ovrcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ovrcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ovrcr {
    #[inline(always)]
    fn from(val: u8) -> Ovrcr {
        Ovrcr::from_bits(val)
    }
}
impl From<Ovrcr> for u8 {
    #[inline(always)]
    fn from(val: Ovrcr) -> u8 {
        Ovrcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ovrcre {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Ovrcre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ovrcre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ovrcre {
    #[inline(always)]
    fn from(val: u8) -> Ovrcre {
        Ovrcre::from_bits(val)
    }
}
impl From<Ovrcre> for u8 {
    #[inline(always)]
    fn from(val: Ovrcre) -> u8 {
        Ovrcre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ovrn {
    #[doc = "No error"]
    _0 = 0x0,
    #[doc = "An error occurred"]
    _1 = 0x01,
}
impl Ovrn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ovrn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ovrn {
    #[inline(always)]
    fn from(val: u8) -> Ovrn {
        Ovrn::from_bits(val)
    }
}
impl From<Ovrn> for u8 {
    #[inline(always)]
    fn from(val: Ovrn) -> u8 {
        Ovrn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pddetint0 {
    #[doc = "PDDET0 detection interrupts are not generated."]
    _0 = 0x0,
    #[doc = "PDDET0 detection interrupts are generated."]
    _1 = 0x01,
}
impl Pddetint0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pddetint0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pddetint0 {
    #[inline(always)]
    fn from(val: u8) -> Pddetint0 {
        Pddetint0::from_bits(val)
    }
}
impl From<Pddetint0> for u8 {
    #[inline(always)]
    fn from(val: Pddetint0) -> u8 {
        Pddetint0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pddetinte0 {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pddetinte0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pddetinte0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pddetinte0 {
    #[inline(always)]
    fn from(val: u8) -> Pddetinte0 {
        Pddetinte0::from_bits(val)
    }
}
impl From<Pddetinte0> for u8 {
    #[inline(always)]
    fn from(val: Pddetinte0) -> u8 {
        Pddetinte0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pddetsts0 {
    #[doc = "Not detected"]
    _0 = 0x0,
    #[doc = "Detected"]
    _1 = 0x01,
}
impl Pddetsts0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pddetsts0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pddetsts0 {
    #[inline(always)]
    fn from(val: u8) -> Pddetsts0 {
        Pddetsts0::from_bits(val)
    }
}
impl From<Pddetsts0> for u8 {
    #[inline(always)]
    fn from(val: Pddetsts0) -> u8 {
        Pddetsts0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe0bemp {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe0bemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe0bemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe0bemp {
    #[inline(always)]
    fn from(val: u8) -> Pipe0bemp {
        Pipe0bemp::from_bits(val)
    }
}
impl From<Pipe0bemp> for u8 {
    #[inline(always)]
    fn from(val: Pipe0bemp) -> u8 {
        Pipe0bemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe0bempe {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe0bempe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe0bempe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe0bempe {
    #[inline(always)]
    fn from(val: u8) -> Pipe0bempe {
        Pipe0bempe::from_bits(val)
    }
}
impl From<Pipe0bempe> for u8 {
    #[inline(always)]
    fn from(val: Pipe0bempe) -> u8 {
        Pipe0bempe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe0brdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe0brdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe0brdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe0brdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe0brdy {
        Pipe0brdy::from_bits(val)
    }
}
impl From<Pipe0brdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe0brdy) -> u8 {
        Pipe0brdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe0brdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe0brdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe0brdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe0brdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe0brdye {
        Pipe0brdye::from_bits(val)
    }
}
impl From<Pipe0brdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe0brdye) -> u8 {
        Pipe0brdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe0nrdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe0nrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe0nrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe0nrdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe0nrdy {
        Pipe0nrdy::from_bits(val)
    }
}
impl From<Pipe0nrdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe0nrdy) -> u8 {
        Pipe0nrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe0nrdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe0nrdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe0nrdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe0nrdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe0nrdye {
        Pipe0nrdye::from_bits(val)
    }
}
impl From<Pipe0nrdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe0nrdye) -> u8 {
        Pipe0nrdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe1bemp {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe1bemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe1bemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe1bemp {
    #[inline(always)]
    fn from(val: u8) -> Pipe1bemp {
        Pipe1bemp::from_bits(val)
    }
}
impl From<Pipe1bemp> for u8 {
    #[inline(always)]
    fn from(val: Pipe1bemp) -> u8 {
        Pipe1bemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe1bempe {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe1bempe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe1bempe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe1bempe {
    #[inline(always)]
    fn from(val: u8) -> Pipe1bempe {
        Pipe1bempe::from_bits(val)
    }
}
impl From<Pipe1bempe> for u8 {
    #[inline(always)]
    fn from(val: Pipe1bempe) -> u8 {
        Pipe1bempe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe1brdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe1brdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe1brdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe1brdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe1brdy {
        Pipe1brdy::from_bits(val)
    }
}
impl From<Pipe1brdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe1brdy) -> u8 {
        Pipe1brdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe1brdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe1brdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe1brdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe1brdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe1brdye {
        Pipe1brdye::from_bits(val)
    }
}
impl From<Pipe1brdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe1brdye) -> u8 {
        Pipe1brdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe1nrdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe1nrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe1nrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe1nrdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe1nrdy {
        Pipe1nrdy::from_bits(val)
    }
}
impl From<Pipe1nrdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe1nrdy) -> u8 {
        Pipe1nrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe1nrdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe1nrdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe1nrdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe1nrdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe1nrdye {
        Pipe1nrdye::from_bits(val)
    }
}
impl From<Pipe1nrdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe1nrdye) -> u8 {
        Pipe1nrdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe2bemp {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe2bemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe2bemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe2bemp {
    #[inline(always)]
    fn from(val: u8) -> Pipe2bemp {
        Pipe2bemp::from_bits(val)
    }
}
impl From<Pipe2bemp> for u8 {
    #[inline(always)]
    fn from(val: Pipe2bemp) -> u8 {
        Pipe2bemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe2bempe {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe2bempe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe2bempe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe2bempe {
    #[inline(always)]
    fn from(val: u8) -> Pipe2bempe {
        Pipe2bempe::from_bits(val)
    }
}
impl From<Pipe2bempe> for u8 {
    #[inline(always)]
    fn from(val: Pipe2bempe) -> u8 {
        Pipe2bempe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe2brdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe2brdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe2brdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe2brdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe2brdy {
        Pipe2brdy::from_bits(val)
    }
}
impl From<Pipe2brdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe2brdy) -> u8 {
        Pipe2brdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe2brdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe2brdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe2brdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe2brdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe2brdye {
        Pipe2brdye::from_bits(val)
    }
}
impl From<Pipe2brdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe2brdye) -> u8 {
        Pipe2brdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe2nrdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe2nrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe2nrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe2nrdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe2nrdy {
        Pipe2nrdy::from_bits(val)
    }
}
impl From<Pipe2nrdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe2nrdy) -> u8 {
        Pipe2nrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe2nrdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe2nrdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe2nrdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe2nrdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe2nrdye {
        Pipe2nrdye::from_bits(val)
    }
}
impl From<Pipe2nrdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe2nrdye) -> u8 {
        Pipe2nrdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe3bemp {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe3bemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe3bemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe3bemp {
    #[inline(always)]
    fn from(val: u8) -> Pipe3bemp {
        Pipe3bemp::from_bits(val)
    }
}
impl From<Pipe3bemp> for u8 {
    #[inline(always)]
    fn from(val: Pipe3bemp) -> u8 {
        Pipe3bemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe3bempe {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe3bempe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe3bempe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe3bempe {
    #[inline(always)]
    fn from(val: u8) -> Pipe3bempe {
        Pipe3bempe::from_bits(val)
    }
}
impl From<Pipe3bempe> for u8 {
    #[inline(always)]
    fn from(val: Pipe3bempe) -> u8 {
        Pipe3bempe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe3brdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe3brdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe3brdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe3brdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe3brdy {
        Pipe3brdy::from_bits(val)
    }
}
impl From<Pipe3brdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe3brdy) -> u8 {
        Pipe3brdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe3brdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe3brdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe3brdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe3brdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe3brdye {
        Pipe3brdye::from_bits(val)
    }
}
impl From<Pipe3brdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe3brdye) -> u8 {
        Pipe3brdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe3nrdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe3nrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe3nrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe3nrdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe3nrdy {
        Pipe3nrdy::from_bits(val)
    }
}
impl From<Pipe3nrdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe3nrdy) -> u8 {
        Pipe3nrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe3nrdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe3nrdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe3nrdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe3nrdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe3nrdye {
        Pipe3nrdye::from_bits(val)
    }
}
impl From<Pipe3nrdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe3nrdye) -> u8 {
        Pipe3nrdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe4bemp {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe4bemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe4bemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe4bemp {
    #[inline(always)]
    fn from(val: u8) -> Pipe4bemp {
        Pipe4bemp::from_bits(val)
    }
}
impl From<Pipe4bemp> for u8 {
    #[inline(always)]
    fn from(val: Pipe4bemp) -> u8 {
        Pipe4bemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe4bempe {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe4bempe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe4bempe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe4bempe {
    #[inline(always)]
    fn from(val: u8) -> Pipe4bempe {
        Pipe4bempe::from_bits(val)
    }
}
impl From<Pipe4bempe> for u8 {
    #[inline(always)]
    fn from(val: Pipe4bempe) -> u8 {
        Pipe4bempe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe4brdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe4brdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe4brdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe4brdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe4brdy {
        Pipe4brdy::from_bits(val)
    }
}
impl From<Pipe4brdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe4brdy) -> u8 {
        Pipe4brdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe4brdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe4brdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe4brdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe4brdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe4brdye {
        Pipe4brdye::from_bits(val)
    }
}
impl From<Pipe4brdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe4brdye) -> u8 {
        Pipe4brdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe4nrdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe4nrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe4nrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe4nrdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe4nrdy {
        Pipe4nrdy::from_bits(val)
    }
}
impl From<Pipe4nrdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe4nrdy) -> u8 {
        Pipe4nrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe4nrdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe4nrdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe4nrdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe4nrdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe4nrdye {
        Pipe4nrdye::from_bits(val)
    }
}
impl From<Pipe4nrdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe4nrdye) -> u8 {
        Pipe4nrdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe5bemp {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe5bemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe5bemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe5bemp {
    #[inline(always)]
    fn from(val: u8) -> Pipe5bemp {
        Pipe5bemp::from_bits(val)
    }
}
impl From<Pipe5bemp> for u8 {
    #[inline(always)]
    fn from(val: Pipe5bemp) -> u8 {
        Pipe5bemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe5bempe {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe5bempe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe5bempe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe5bempe {
    #[inline(always)]
    fn from(val: u8) -> Pipe5bempe {
        Pipe5bempe::from_bits(val)
    }
}
impl From<Pipe5bempe> for u8 {
    #[inline(always)]
    fn from(val: Pipe5bempe) -> u8 {
        Pipe5bempe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe5brdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe5brdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe5brdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe5brdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe5brdy {
        Pipe5brdy::from_bits(val)
    }
}
impl From<Pipe5brdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe5brdy) -> u8 {
        Pipe5brdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe5brdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe5brdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe5brdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe5brdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe5brdye {
        Pipe5brdye::from_bits(val)
    }
}
impl From<Pipe5brdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe5brdye) -> u8 {
        Pipe5brdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe5nrdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe5nrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe5nrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe5nrdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe5nrdy {
        Pipe5nrdy::from_bits(val)
    }
}
impl From<Pipe5nrdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe5nrdy) -> u8 {
        Pipe5nrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe5nrdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe5nrdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe5nrdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe5nrdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe5nrdye {
        Pipe5nrdye::from_bits(val)
    }
}
impl From<Pipe5nrdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe5nrdye) -> u8 {
        Pipe5nrdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe6bemp {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe6bemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe6bemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe6bemp {
    #[inline(always)]
    fn from(val: u8) -> Pipe6bemp {
        Pipe6bemp::from_bits(val)
    }
}
impl From<Pipe6bemp> for u8 {
    #[inline(always)]
    fn from(val: Pipe6bemp) -> u8 {
        Pipe6bemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe6bempe {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe6bempe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe6bempe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe6bempe {
    #[inline(always)]
    fn from(val: u8) -> Pipe6bempe {
        Pipe6bempe::from_bits(val)
    }
}
impl From<Pipe6bempe> for u8 {
    #[inline(always)]
    fn from(val: Pipe6bempe) -> u8 {
        Pipe6bempe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe6brdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe6brdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe6brdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe6brdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe6brdy {
        Pipe6brdy::from_bits(val)
    }
}
impl From<Pipe6brdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe6brdy) -> u8 {
        Pipe6brdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe6brdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe6brdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe6brdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe6brdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe6brdye {
        Pipe6brdye::from_bits(val)
    }
}
impl From<Pipe6brdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe6brdye) -> u8 {
        Pipe6brdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe6nrdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe6nrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe6nrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe6nrdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe6nrdy {
        Pipe6nrdy::from_bits(val)
    }
}
impl From<Pipe6nrdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe6nrdy) -> u8 {
        Pipe6nrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe6nrdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe6nrdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe6nrdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe6nrdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe6nrdye {
        Pipe6nrdye::from_bits(val)
    }
}
impl From<Pipe6nrdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe6nrdye) -> u8 {
        Pipe6nrdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe7bemp {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe7bemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe7bemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe7bemp {
    #[inline(always)]
    fn from(val: u8) -> Pipe7bemp {
        Pipe7bemp::from_bits(val)
    }
}
impl From<Pipe7bemp> for u8 {
    #[inline(always)]
    fn from(val: Pipe7bemp) -> u8 {
        Pipe7bemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe7bempe {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe7bempe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe7bempe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe7bempe {
    #[inline(always)]
    fn from(val: u8) -> Pipe7bempe {
        Pipe7bempe::from_bits(val)
    }
}
impl From<Pipe7bempe> for u8 {
    #[inline(always)]
    fn from(val: Pipe7bempe) -> u8 {
        Pipe7bempe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe7brdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe7brdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe7brdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe7brdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe7brdy {
        Pipe7brdy::from_bits(val)
    }
}
impl From<Pipe7brdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe7brdy) -> u8 {
        Pipe7brdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe7brdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe7brdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe7brdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe7brdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe7brdye {
        Pipe7brdye::from_bits(val)
    }
}
impl From<Pipe7brdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe7brdye) -> u8 {
        Pipe7brdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe7nrdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe7nrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe7nrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe7nrdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe7nrdy {
        Pipe7nrdy::from_bits(val)
    }
}
impl From<Pipe7nrdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe7nrdy) -> u8 {
        Pipe7nrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe7nrdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe7nrdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe7nrdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe7nrdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe7nrdye {
        Pipe7nrdye::from_bits(val)
    }
}
impl From<Pipe7nrdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe7nrdye) -> u8 {
        Pipe7nrdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe8bemp {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe8bemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe8bemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe8bemp {
    #[inline(always)]
    fn from(val: u8) -> Pipe8bemp {
        Pipe8bemp::from_bits(val)
    }
}
impl From<Pipe8bemp> for u8 {
    #[inline(always)]
    fn from(val: Pipe8bemp) -> u8 {
        Pipe8bemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe8bempe {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe8bempe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe8bempe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe8bempe {
    #[inline(always)]
    fn from(val: u8) -> Pipe8bempe {
        Pipe8bempe::from_bits(val)
    }
}
impl From<Pipe8bempe> for u8 {
    #[inline(always)]
    fn from(val: Pipe8bempe) -> u8 {
        Pipe8bempe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe8brdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe8brdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe8brdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe8brdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe8brdy {
        Pipe8brdy::from_bits(val)
    }
}
impl From<Pipe8brdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe8brdy) -> u8 {
        Pipe8brdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe8brdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe8brdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe8brdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe8brdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe8brdye {
        Pipe8brdye::from_bits(val)
    }
}
impl From<Pipe8brdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe8brdye) -> u8 {
        Pipe8brdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe8nrdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe8nrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe8nrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe8nrdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe8nrdy {
        Pipe8nrdy::from_bits(val)
    }
}
impl From<Pipe8nrdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe8nrdy) -> u8 {
        Pipe8nrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe8nrdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe8nrdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe8nrdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe8nrdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe8nrdye {
        Pipe8nrdye::from_bits(val)
    }
}
impl From<Pipe8nrdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe8nrdye) -> u8 {
        Pipe8nrdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe9bemp {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe9bemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe9bemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe9bemp {
    #[inline(always)]
    fn from(val: u8) -> Pipe9bemp {
        Pipe9bemp::from_bits(val)
    }
}
impl From<Pipe9bemp> for u8 {
    #[inline(always)]
    fn from(val: Pipe9bemp) -> u8 {
        Pipe9bemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe9bempe {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe9bempe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe9bempe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe9bempe {
    #[inline(always)]
    fn from(val: u8) -> Pipe9bempe {
        Pipe9bempe::from_bits(val)
    }
}
impl From<Pipe9bempe> for u8 {
    #[inline(always)]
    fn from(val: Pipe9bempe) -> u8 {
        Pipe9bempe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe9brdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe9brdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe9brdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe9brdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe9brdy {
        Pipe9brdy::from_bits(val)
    }
}
impl From<Pipe9brdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe9brdy) -> u8 {
        Pipe9brdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe9brdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe9brdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe9brdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe9brdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe9brdye {
        Pipe9brdye::from_bits(val)
    }
}
impl From<Pipe9brdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe9brdye) -> u8 {
        Pipe9brdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe9nrdy {
    #[doc = "Interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Interrupts are generated."]
    _1 = 0x01,
}
impl Pipe9nrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe9nrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe9nrdy {
    #[inline(always)]
    fn from(val: u8) -> Pipe9nrdy {
        Pipe9nrdy::from_bits(val)
    }
}
impl From<Pipe9nrdy> for u8 {
    #[inline(always)]
    fn from(val: Pipe9nrdy) -> u8 {
        Pipe9nrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipe9nrdye {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Pipe9nrdye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipe9nrdye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipe9nrdye {
    #[inline(always)]
    fn from(val: u8) -> Pipe9nrdye {
        Pipe9nrdye::from_bits(val)
    }
}
impl From<Pipe9nrdye> for u8 {
    #[inline(always)]
    fn from(val: Pipe9nrdye) -> u8 {
        Pipe9nrdye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PipecfgDir {
    #[doc = "Receiving direction"]
    _0 = 0x0,
    #[doc = "Transmitting direction"]
    _1 = 0x01,
}
impl PipecfgDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PipecfgDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PipecfgDir {
    #[inline(always)]
    fn from(val: u8) -> PipecfgDir {
        PipecfgDir::from_bits(val)
    }
}
impl From<PipecfgDir> for u8 {
    #[inline(always)]
    fn from(val: PipecfgDir) -> u8 {
        PipecfgDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PipecfgShtnak {
    #[doc = "Continue pipe operation after transfer ends"]
    _0 = 0x0,
    #[doc = "Disable pipe operation after transfer ends."]
    _1 = 0x01,
}
impl PipecfgShtnak {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PipecfgShtnak {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PipecfgShtnak {
    #[inline(always)]
    fn from(val: u8) -> PipecfgShtnak {
        PipecfgShtnak::from_bits(val)
    }
}
impl From<PipecfgShtnak> for u8 {
    #[inline(always)]
    fn from(val: PipecfgShtnak) -> u8 {
        PipecfgShtnak::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipectr2Aclrm {
    #[doc = "Auto buffer clear mode is disabled."]
    _0 = 0x0,
    #[doc = "Auto buffer clear mode is enabled (all buffers are initialized)"]
    _1 = 0x01,
}
impl Pipectr2Aclrm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipectr2Aclrm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipectr2Aclrm {
    #[inline(always)]
    fn from(val: u8) -> Pipectr2Aclrm {
        Pipectr2Aclrm::from_bits(val)
    }
}
impl From<Pipectr2Aclrm> for u8 {
    #[inline(always)]
    fn from(val: Pipectr2Aclrm) -> u8 {
        Pipectr2Aclrm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipectr2Bsts {
    #[doc = "Buffer access is disabled."]
    _0 = 0x0,
    #[doc = "Buffer access is enabled."]
    _1 = 0x01,
}
impl Pipectr2Bsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipectr2Bsts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipectr2Bsts {
    #[inline(always)]
    fn from(val: u8) -> Pipectr2Bsts {
        Pipectr2Bsts::from_bits(val)
    }
}
impl From<Pipectr2Bsts> for u8 {
    #[inline(always)]
    fn from(val: Pipectr2Bsts) -> u8 {
        Pipectr2Bsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipectr2Pbusy {
    #[doc = "Pipe n not in use for the transaction"]
    _0 = 0x0,
    #[doc = "Pipe n in use for the transaction."]
    _1 = 0x01,
}
impl Pipectr2Pbusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipectr2Pbusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipectr2Pbusy {
    #[inline(always)]
    fn from(val: u8) -> Pipectr2Pbusy {
        Pipectr2Pbusy::from_bits(val)
    }
}
impl From<Pipectr2Pbusy> for u8 {
    #[inline(always)]
    fn from(val: Pipectr2Pbusy) -> u8 {
        Pipectr2Pbusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipectr2Pid {
    #[doc = "NAK response"]
    _00 = 0x0,
    #[doc = "BUF response (depending on the buffer state)"]
    _01 = 0x01,
    #[doc = "STALL response"]
    _10 = 0x02,
    #[doc = "STALL response"]
    _11 = 0x03,
}
impl Pipectr2Pid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipectr2Pid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipectr2Pid {
    #[inline(always)]
    fn from(val: u8) -> Pipectr2Pid {
        Pipectr2Pid::from_bits(val)
    }
}
impl From<Pipectr2Pid> for u8 {
    #[inline(always)]
    fn from(val: Pipectr2Pid) -> u8 {
        Pipectr2Pid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipectr2Sqclr {
    #[doc = "Invalid"]
    _0 = 0x0,
    #[doc = "Specifies DATA0."]
    _1 = 0x01,
}
impl Pipectr2Sqclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipectr2Sqclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipectr2Sqclr {
    #[inline(always)]
    fn from(val: u8) -> Pipectr2Sqclr {
        Pipectr2Sqclr::from_bits(val)
    }
}
impl From<Pipectr2Sqclr> for u8 {
    #[inline(always)]
    fn from(val: Pipectr2Sqclr) -> u8 {
        Pipectr2Sqclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipectr2Sqmon {
    #[doc = "DATA0"]
    _0 = 0x0,
    #[doc = "DATA1"]
    _1 = 0x01,
}
impl Pipectr2Sqmon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipectr2Sqmon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipectr2Sqmon {
    #[inline(always)]
    fn from(val: u8) -> Pipectr2Sqmon {
        Pipectr2Sqmon::from_bits(val)
    }
}
impl From<Pipectr2Sqmon> for u8 {
    #[inline(always)]
    fn from(val: Pipectr2Sqmon) -> u8 {
        Pipectr2Sqmon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipectr2Sqset {
    #[doc = "Invalid"]
    _0 = 0x0,
    #[doc = "Specifies DATA1."]
    _1 = 0x01,
}
impl Pipectr2Sqset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipectr2Sqset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipectr2Sqset {
    #[inline(always)]
    fn from(val: u8) -> Pipectr2Sqset {
        Pipectr2Sqset::from_bits(val)
    }
}
impl From<Pipectr2Sqset> for u8 {
    #[inline(always)]
    fn from(val: Pipectr2Sqset) -> u8 {
        Pipectr2Sqset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PipectrAclrm {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled (all buffers are initialized)"]
    _1 = 0x01,
}
impl PipectrAclrm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PipectrAclrm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PipectrAclrm {
    #[inline(always)]
    fn from(val: u8) -> PipectrAclrm {
        PipectrAclrm::from_bits(val)
    }
}
impl From<PipectrAclrm> for u8 {
    #[inline(always)]
    fn from(val: PipectrAclrm) -> u8 {
        PipectrAclrm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PipectrBsts {
    #[doc = "Buffer access by the CPU is disabled."]
    _0 = 0x0,
    #[doc = "Buffer access by the CPU is enabled."]
    _1 = 0x01,
}
impl PipectrBsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PipectrBsts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PipectrBsts {
    #[inline(always)]
    fn from(val: u8) -> PipectrBsts {
        PipectrBsts::from_bits(val)
    }
}
impl From<PipectrBsts> for u8 {
    #[inline(always)]
    fn from(val: PipectrBsts) -> u8 {
        PipectrBsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PipectrPbusy {
    #[doc = "Pipe n not in use for the transaction"]
    _0 = 0x0,
    #[doc = "Pipe n in use for the transaction."]
    _1 = 0x01,
}
impl PipectrPbusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PipectrPbusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PipectrPbusy {
    #[inline(always)]
    fn from(val: u8) -> PipectrPbusy {
        PipectrPbusy::from_bits(val)
    }
}
impl From<PipectrPbusy> for u8 {
    #[inline(always)]
    fn from(val: PipectrPbusy) -> u8 {
        PipectrPbusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PipectrPid {
    #[doc = "NAK response"]
    _00 = 0x0,
    #[doc = "BUF response (depending on the buffer state)"]
    _01 = 0x01,
    #[doc = "STALL response"]
    _10 = 0x02,
    #[doc = "STALL response"]
    _11 = 0x03,
}
impl PipectrPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PipectrPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PipectrPid {
    #[inline(always)]
    fn from(val: u8) -> PipectrPid {
        PipectrPid::from_bits(val)
    }
}
impl From<PipectrPid> for u8 {
    #[inline(always)]
    fn from(val: PipectrPid) -> u8 {
        PipectrPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PipectrSqclr {
    #[doc = "Write disabled"]
    _0 = 0x0,
    #[doc = "Specifies DATA0."]
    _1 = 0x01,
}
impl PipectrSqclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PipectrSqclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PipectrSqclr {
    #[inline(always)]
    fn from(val: u8) -> PipectrSqclr {
        PipectrSqclr::from_bits(val)
    }
}
impl From<PipectrSqclr> for u8 {
    #[inline(always)]
    fn from(val: PipectrSqclr) -> u8 {
        PipectrSqclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PipectrSqmon {
    #[doc = "DATA0"]
    _0 = 0x0,
    #[doc = "DATA1"]
    _1 = 0x01,
}
impl PipectrSqmon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PipectrSqmon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PipectrSqmon {
    #[inline(always)]
    fn from(val: u8) -> PipectrSqmon {
        PipectrSqmon::from_bits(val)
    }
}
impl From<PipectrSqmon> for u8 {
    #[inline(always)]
    fn from(val: PipectrSqmon) -> u8 {
        PipectrSqmon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PipectrSqset {
    #[doc = "Write disabled"]
    _0 = 0x0,
    #[doc = "Specifies DATA1."]
    _1 = 0x01,
}
impl PipectrSqset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PipectrSqset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PipectrSqset {
    #[inline(always)]
    fn from(val: u8) -> PipectrSqset {
        PipectrSqset::from_bits(val)
    }
}
impl From<PipectrSqset> for u8 {
    #[inline(always)]
    fn from(val: PipectrSqset) -> u8 {
        PipectrSqset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PipemaxpDevsel {
    #[doc = "Address 0000"]
    _0000 = 0x0,
    #[doc = "Address 0001"]
    _0001 = 0x01,
    #[doc = "Address 0010"]
    _0010 = 0x02,
    #[doc = "Address 0011"]
    _0011 = 0x03,
    #[doc = "Address 0100"]
    _0100 = 0x04,
    #[doc = "Address 0101"]
    _0101 = 0x05,
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
impl PipemaxpDevsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PipemaxpDevsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PipemaxpDevsel {
    #[inline(always)]
    fn from(val: u8) -> PipemaxpDevsel {
        PipemaxpDevsel::from_bits(val)
    }
}
impl From<PipemaxpDevsel> for u8 {
    #[inline(always)]
    fn from(val: PipemaxpDevsel) -> u8 {
        PipemaxpDevsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipesel {
    #[doc = "No pipe selected"]
    _0000 = 0x0,
    #[doc = "PIPE1"]
    _0001 = 0x01,
    #[doc = "PIPE2"]
    _0010 = 0x02,
    #[doc = "PIPE3"]
    _0011 = 0x03,
    #[doc = "PIPE4"]
    _0100 = 0x04,
    #[doc = "PIPE5"]
    _0101 = 0x05,
    #[doc = "PIPE6"]
    _0110 = 0x06,
    #[doc = "PIPE7"]
    _0111 = 0x07,
    #[doc = "PIPE8"]
    _1000 = 0x08,
    #[doc = "PIPE9"]
    _1001 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pipesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipesel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipesel {
    #[inline(always)]
    fn from(val: u8) -> Pipesel {
        Pipesel::from_bits(val)
    }
}
impl From<Pipesel> for u8 {
    #[inline(always)]
    fn from(val: Pipesel) -> u8 {
        Pipesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resm {
    #[doc = "Resume interrupts are not generated."]
    _0 = 0x0,
    #[doc = "Resume interrupts are generated."]
    _1 = 0x01,
}
impl Resm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resm {
    #[inline(always)]
    fn from(val: u8) -> Resm {
        Resm::from_bits(val)
    }
}
impl From<Resm> for u8 {
    #[inline(always)]
    fn from(val: Resm) -> u8 {
        Resm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resume {
    #[doc = "Resume signal is not output."]
    _0 = 0x0,
    #[doc = "Resume signal is output."]
    _1 = 0x01,
}
impl Resume {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resume {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resume {
    #[inline(always)]
    fn from(val: u8) -> Resume {
        Resume::from_bits(val)
    }
}
impl From<Resume> for u8 {
    #[inline(always)]
    fn from(val: Resume) -> u8 {
        Resume::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rhst {
    #[doc = "Communication speed not determined"]
    _000 = 0x0,
    #[doc = "Low-speed connection(When the host controller is selected) /USB bus reset in progress( When the function controller is selected)"]
    _001 = 0x01,
    #[doc = "Full-speed connection(When the host controller is selected) /USB bus reset in progress or full-speed connection(When the function controller is selected)"]
    _010 = 0x02,
    #[doc = "Setting prohibited"]
    _011 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Rhst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rhst {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rhst {
    #[inline(always)]
    fn from(val: u8) -> Rhst {
        Rhst::from_bits(val)
    }
}
impl From<Rhst> for u8 {
    #[inline(always)]
    fn from(val: Rhst) -> u8 {
        Rhst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rpdme0 {
    #[doc = "Pull-down off"]
    _0 = 0x0,
    #[doc = "Pull-down on"]
    _1 = 0x01,
}
impl Rpdme0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rpdme0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rpdme0 {
    #[inline(always)]
    fn from(val: u8) -> Rpdme0 {
        Rpdme0::from_bits(val)
    }
}
impl From<Rpdme0> for u8 {
    #[inline(always)]
    fn from(val: Rpdme0) -> u8 {
        Rpdme0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rsme {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Rsme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rsme {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rsme {
    #[inline(always)]
    fn from(val: u8) -> Rsme {
        Rsme::from_bits(val)
    }
}
impl From<Rsme> for u8 {
    #[inline(always)]
    fn from(val: Rsme) -> u8 {
        Rsme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwupe {
    #[doc = "Downstream port wakeup is disabled."]
    _0 = 0x0,
    #[doc = "Downstream port wakeup is enabled."]
    _1 = 0x01,
}
impl Rwupe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwupe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwupe {
    #[inline(always)]
    fn from(val: u8) -> Rwupe {
        Rwupe::from_bits(val)
    }
}
impl From<Rwupe> for u8 {
    #[inline(always)]
    fn from(val: Rwupe) -> u8 {
        Rwupe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sack {
    #[doc = "SACK interrupts are not generated."]
    _0 = 0x0,
    #[doc = "SACK interrupts are generated."]
    _1 = 0x01,
}
impl Sack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sack {
    #[inline(always)]
    fn from(val: u8) -> Sack {
        Sack::from_bits(val)
    }
}
impl From<Sack> for u8 {
    #[inline(always)]
    fn from(val: Sack) -> u8 {
        Sack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sacke {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Sacke {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sacke {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sacke {
    #[inline(always)]
    fn from(val: u8) -> Sacke {
        Sacke::from_bits(val)
    }
}
impl From<Sacke> for u8 {
    #[inline(always)]
    fn from(val: Sacke) -> u8 {
        Sacke::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scke {
    #[doc = "Clock supply to the USBFS stopped"]
    _0 = 0x0,
    #[doc = "Clock supply to the USBFS enabled."]
    _1 = 0x01,
}
impl Scke {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scke {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scke {
    #[inline(always)]
    fn from(val: u8) -> Scke {
        Scke::from_bits(val)
    }
}
impl From<Scke> for u8 {
    #[inline(always)]
    fn from(val: Scke) -> u8 {
        Scke::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sign {
    #[doc = "SIGN interrupts are not generated."]
    _0 = 0x0,
    #[doc = "SIGN interrupts are generated."]
    _1 = 0x01,
}
impl Sign {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sign {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sign {
    #[inline(always)]
    fn from(val: u8) -> Sign {
        Sign::from_bits(val)
    }
}
impl From<Sign> for u8 {
    #[inline(always)]
    fn from(val: Sign) -> u8 {
        Sign::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Signe {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Signe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Signe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Signe {
    #[inline(always)]
    fn from(val: u8) -> Signe {
        Signe::from_bits(val)
    }
}
impl From<Signe> for u8 {
    #[inline(always)]
    fn from(val: Signe) -> u8 {
        Signe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sofe {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Sofe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sofe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sofe {
    #[inline(always)]
    fn from(val: u8) -> Sofe {
        Sofe::from_bits(val)
    }
}
impl From<Sofe> for u8 {
    #[inline(always)]
    fn from(val: Sofe) -> u8 {
        Sofe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sofr {
    #[doc = "SOF interrupts are not generated."]
    _0 = 0x0,
    #[doc = "SOF interrupts are generated."]
    _1 = 0x01,
}
impl Sofr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sofr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sofr {
    #[inline(always)]
    fn from(val: u8) -> Sofr {
        Sofr::from_bits(val)
    }
}
impl From<Sofr> for u8 {
    #[inline(always)]
    fn from(val: Sofr) -> u8 {
        Sofr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sureq {
    #[doc = "Invalid"]
    _0 = 0x0,
    #[doc = "Transmits the setup packet."]
    _1 = 0x01,
}
impl Sureq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sureq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sureq {
    #[inline(always)]
    fn from(val: u8) -> Sureq {
        Sureq::from_bits(val)
    }
}
impl From<Sureq> for u8 {
    #[inline(always)]
    fn from(val: Sureq) -> u8 {
        Sureq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sureqclr {
    #[doc = "Invalid"]
    _0 = 0x0,
    #[doc = "Clears the SUREQ bit to 0."]
    _1 = 0x01,
}
impl Sureqclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sureqclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sureqclr {
    #[inline(always)]
    fn from(val: u8) -> Sureqclr {
        Sureqclr::from_bits(val)
    }
}
impl From<Sureqclr> for u8 {
    #[inline(always)]
    fn from(val: Sureqclr) -> u8 {
        Sureqclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trclr {
    #[doc = "Invalid"]
    _0 = 0x0,
    #[doc = "The current counter value is cleared."]
    _1 = 0x01,
}
impl Trclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trclr {
    #[inline(always)]
    fn from(val: u8) -> Trclr {
        Trclr::from_bits(val)
    }
}
impl From<Trclr> for u8 {
    #[inline(always)]
    fn from(val: Trclr) -> u8 {
        Trclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trenb {
    #[doc = "Transaction counter is disabled."]
    _0 = 0x0,
    #[doc = "Transaction counter is enabled."]
    _1 = 0x01,
}
impl Trenb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trenb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trenb {
    #[inline(always)]
    fn from(val: u8) -> Trenb {
        Trenb::from_bits(val)
    }
}
impl From<Trenb> for u8 {
    #[inline(always)]
    fn from(val: Trenb) -> u8 {
        Trenb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trnensel {
    #[doc = "Not low-speed communication"]
    _0 = 0x0,
    #[doc = "Low-speed communication."]
    _1 = 0x01,
}
impl Trnensel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trnensel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trnensel {
    #[inline(always)]
    fn from(val: u8) -> Trnensel {
        Trnensel::from_bits(val)
    }
}
impl From<Trnensel> for u8 {
    #[inline(always)]
    fn from(val: Trnensel) -> u8 {
        Trnensel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Type {
    #[doc = "Pipe not used"]
    _00 = 0x0,
    #[doc = "Bulk transfer(PIPE1 and PIPE5) /Setting prohibited(PIPE6 to PIPE9)"]
    _01 = 0x01,
    #[doc = "Setting prohibited(PIPE1 and PIPE5) /Interrupt transfer(PIPE6 to PIPE9)"]
    _10 = 0x02,
    #[doc = "Isochronous transfer(PIPE1 and PIPE2) /Setting prohibited(PIPE3 to PIPE9)"]
    _11 = 0x03,
}
impl Type {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Type {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Type {
    #[inline(always)]
    fn from(val: u8) -> Type {
        Type::from_bits(val)
    }
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(val: Type) -> u8 {
        Type::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uact {
    #[doc = "Downstream port is disabled (SOF transmission is disabled)."]
    _0 = 0x0,
    #[doc = "Downstream port is enabled (SOF transmission is enabled)."]
    _1 = 0x01,
}
impl Uact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uact {
    #[inline(always)]
    fn from(val: u8) -> Uact {
        Uact::from_bits(val)
    }
}
impl From<Uact> for u8 {
    #[inline(always)]
    fn from(val: Uact) -> u8 {
        Uact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbe {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled."]
    _1 = 0x01,
}
impl Usbe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbe {
    #[inline(always)]
    fn from(val: u8) -> Usbe {
        Usbe::from_bits(val)
    }
}
impl From<Usbe> for u8 {
    #[inline(always)]
    fn from(val: Usbe) -> u8 {
        Usbe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbrst {
    #[doc = "USB bus reset signal is not output."]
    _0 = 0x0,
    #[doc = "USB bus reset signal is output."]
    _1 = 0x01,
}
impl Usbrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbrst {
    #[inline(always)]
    fn from(val: u8) -> Usbrst {
        Usbrst::from_bits(val)
    }
}
impl From<Usbrst> for u8 {
    #[inline(always)]
    fn from(val: Usbrst) -> u8 {
        Usbrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbspd {
    #[doc = "DEVADDn is not used"]
    _00 = 0x0,
    #[doc = "Low speed"]
    _01 = 0x01,
    #[doc = "Full speed"]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Usbspd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbspd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbspd {
    #[inline(always)]
    fn from(val: u8) -> Usbspd {
        Usbspd::from_bits(val)
    }
}
impl From<Usbspd> for u8 {
    #[inline(always)]
    fn from(val: Usbspd) -> u8 {
        Usbspd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Valid {
    #[doc = "Setup packet is not received"]
    _0 = 0x0,
    #[doc = "Setup packet is received"]
    _1 = 0x01,
}
impl Valid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Valid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Valid {
    #[inline(always)]
    fn from(val: u8) -> Valid {
        Valid::from_bits(val)
    }
}
impl From<Valid> for u8 {
    #[inline(always)]
    fn from(val: Valid) -> u8 {
        Valid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbint {
    #[doc = "VBUS interrupts are not generated."]
    _0 = 0x0,
    #[doc = "VBUS interrupts are generated."]
    _1 = 0x01,
}
impl Vbint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbint {
    #[inline(always)]
    fn from(val: u8) -> Vbint {
        Vbint::from_bits(val)
    }
}
impl From<Vbint> for u8 {
    #[inline(always)]
    fn from(val: Vbint) -> u8 {
        Vbint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbse {
    #[doc = "Interrupt output disabled"]
    _0 = 0x0,
    #[doc = "Interrupt output enabled"]
    _1 = 0x01,
}
impl Vbse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbse {
    #[inline(always)]
    fn from(val: u8) -> Vbse {
        Vbse::from_bits(val)
    }
}
impl From<Vbse> for u8 {
    #[inline(always)]
    fn from(val: Vbse) -> u8 {
        Vbse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbsts {
    #[doc = "USB_VBUS pin is low."]
    _0 = 0x0,
    #[doc = "USB_VBUS pin is high."]
    _1 = 0x01,
}
impl Vbsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbsts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbsts {
    #[inline(always)]
    fn from(val: u8) -> Vbsts {
        Vbsts::from_bits(val)
    }
}
impl From<Vbsts> for u8 {
    #[inline(always)]
    fn from(val: Vbsts) -> u8 {
        Vbsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbusen {
    #[doc = "External USB_VBUSEN pin outputs low"]
    _0 = 0x0,
    #[doc = "External USB_VBUSEN pin outputs high"]
    _1 = 0x01,
}
impl Vbusen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbusen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbusen {
    #[inline(always)]
    fn from(val: u8) -> Vbusen {
        Vbusen::from_bits(val)
    }
}
impl From<Vbusen> for u8 {
    #[inline(always)]
    fn from(val: Vbusen) -> u8 {
        Vbusen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vdcen {
    #[doc = "USB regulator off"]
    _0 = 0x0,
    #[doc = "USB regulator on"]
    _1 = 0x01,
}
impl Vdcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vdcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vdcen {
    #[inline(always)]
    fn from(val: u8) -> Vdcen {
        Vdcen::from_bits(val)
    }
}
impl From<Vdcen> for u8 {
    #[inline(always)]
    fn from(val: Vdcen) -> u8 {
        Vdcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vddusbe {
    #[doc = "USB reference power supply circuit off"]
    _0 = 0x0,
    #[doc = "USB reference power supply circuit on"]
    _1 = 0x01,
}
impl Vddusbe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vddusbe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vddusbe {
    #[inline(always)]
    fn from(val: u8) -> Vddusbe {
        Vddusbe::from_bits(val)
    }
}
impl From<Vddusbe> for u8 {
    #[inline(always)]
    fn from(val: Vddusbe) -> u8 {
        Vddusbe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vdmsrce0 {
    #[doc = "Stop"]
    _0 = 0x0,
    #[doc = "0.6V output"]
    _1 = 0x01,
}
impl Vdmsrce0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vdmsrce0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vdmsrce0 {
    #[inline(always)]
    fn from(val: u8) -> Vdmsrce0 {
        Vdmsrce0::from_bits(val)
    }
}
impl From<Vdmsrce0> for u8 {
    #[inline(always)]
    fn from(val: Vdmsrce0) -> u8 {
        Vdmsrce0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vdpsrce0 {
    #[doc = "Stop"]
    _0 = 0x0,
    #[doc = "0.6V output"]
    _1 = 0x01,
}
impl Vdpsrce0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vdpsrce0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vdpsrce0 {
    #[inline(always)]
    fn from(val: u8) -> Vdpsrce0 {
        Vdpsrce0::from_bits(val)
    }
}
impl From<Vdpsrce0> for u8 {
    #[inline(always)]
    fn from(val: Vdpsrce0) -> u8 {
        Vdpsrce0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wkup {
    #[doc = "Remote wakeup signal is not output."]
    _0 = 0x0,
    #[doc = "Remote wakeup signal is output."]
    _1 = 0x01,
}
impl Wkup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wkup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wkup {
    #[inline(always)]
    fn from(val: u8) -> Wkup {
        Wkup::from_bits(val)
    }
}
impl From<Wkup> for u8 {
    #[inline(always)]
    fn from(val: Wkup) -> u8 {
        Wkup::to_bits(val)
    }
}
