#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssicr(pub u32);
impl Ssicr {
    #[doc = "Receive Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Enable"]
    #[inline(always)]
    pub const fn set_ren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Enable"]
    #[inline(always)]
    pub const fn set_ten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Mute Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn muen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Mute Enable"]
    #[inline(always)]
    pub const fn set_muen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Selects Bit Clock Division Ratio"]
    #[must_use]
    #[inline(always)]
    pub const fn ckdv(&self) -> super::vals::Ckdv {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Ckdv::from_bits(val as u8)
    }
    #[doc = "Selects Bit Clock Division Ratio"]
    #[inline(always)]
    pub const fn set_ckdv(&mut self, val: super::vals::Ckdv) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Selects Serial Data Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn del(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Selects Serial Data Delay"]
    #[inline(always)]
    pub const fn set_del(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Selects Placement Data Alignment"]
    #[must_use]
    #[inline(always)]
    pub const fn pdta(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Selects Placement Data Alignment"]
    #[inline(always)]
    pub const fn set_pdta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Selects Serial Data Alignment"]
    #[must_use]
    #[inline(always)]
    pub const fn sdta(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Selects Serial Data Alignment"]
    #[inline(always)]
    pub const fn set_sdta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Selects Serial Padding Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn spdp(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Selects Serial Padding Polarity"]
    #[inline(always)]
    pub const fn set_spdp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn lrckp(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal"]
    #[inline(always)]
    pub const fn set_lrckp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Selects Bit Clock Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn bckp(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Selects Bit Clock Polarity"]
    #[inline(always)]
    pub const fn set_bckp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Master Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mst(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Master Enable"]
    #[inline(always)]
    pub const fn set_mst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Selects System Word Length"]
    #[must_use]
    #[inline(always)]
    pub const fn swl(&self) -> super::vals::Swl {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Swl::from_bits(val as u8)
    }
    #[doc = "Selects System Word Length"]
    #[inline(always)]
    pub const fn set_swl(&mut self, val: super::vals::Swl) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Selects Data Word Length"]
    #[must_use]
    #[inline(always)]
    pub const fn dwl(&self) -> super::vals::Dwl {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Dwl::from_bits(val as u8)
    }
    #[doc = "Selects Data Word Length"]
    #[inline(always)]
    pub const fn set_dwl(&mut self, val: super::vals::Dwl) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "Idle Mode Interrupt Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iien(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Idle Mode Interrupt Output Enable"]
    #[inline(always)]
    pub const fn set_iien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Receive Overflow Interrupt Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn roien(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Overflow Interrupt Output Enable"]
    #[inline(always)]
    pub const fn set_roien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Receive Underflow Interrupt Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ruien(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Underflow Interrupt Output Enable"]
    #[inline(always)]
    pub const fn set_ruien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Transmit Overflow Interrupt Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn toien(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Overflow Interrupt Output Enable"]
    #[inline(always)]
    pub const fn set_toien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Transmit Underflow Interrupt Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tuien(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Underflow Interrupt Output Enable"]
    #[inline(always)]
    pub const fn set_tuien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Selects an Audio Clock for Master-mode Communication"]
    #[must_use]
    #[inline(always)]
    pub const fn cks(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Selects an Audio Clock for Master-mode Communication"]
    #[inline(always)]
    pub const fn set_cks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ssicr {
    #[inline(always)]
    fn default() -> Ssicr {
        Ssicr(0)
    }
}
impl core::fmt::Debug for Ssicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssicr")
            .field("ren", &self.ren())
            .field("ten", &self.ten())
            .field("reserved", &self.reserved())
            .field("muen", &self.muen())
            .field("ckdv", &self.ckdv())
            .field("del", &self.del())
            .field("pdta", &self.pdta())
            .field("sdta", &self.sdta())
            .field("spdp", &self.spdp())
            .field("lrckp", &self.lrckp())
            .field("bckp", &self.bckp())
            .field("mst", &self.mst())
            .field("reserved_2", &self.reserved_2())
            .field("swl", &self.swl())
            .field("dwl", &self.dwl())
            .field("reserved_3", &self.reserved_3())
            .field("iien", &self.iien())
            .field("roien", &self.roien())
            .field("ruien", &self.ruien())
            .field("toien", &self.toien())
            .field("tuien", &self.tuien())
            .field("cks", &self.cks())
            .field("reserved_4", &self.reserved_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssicr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssicr {{ ren: {=bool:?}, ten: {=bool:?}, reserved: {=bool:?}, muen: {=bool:?}, ckdv: {:?}, del: {=bool:?}, pdta: {=bool:?}, sdta: {=bool:?}, spdp: {=bool:?}, lrckp: {=bool:?}, bckp: {=bool:?}, mst: {=bool:?}, reserved_2: {=bool:?}, swl: {:?}, dwl: {:?}, reserved_3: {=u8:?}, iien: {=bool:?}, roien: {=bool:?}, ruien: {=bool:?}, toien: {=bool:?}, tuien: {=bool:?}, cks: {=bool:?}, reserved_4: {=bool:?} }}",
            self.ren(),
            self.ten(),
            self.reserved(),
            self.muen(),
            self.ckdv(),
            self.del(),
            self.pdta(),
            self.sdta(),
            self.spdp(),
            self.lrckp(),
            self.bckp(),
            self.mst(),
            self.reserved_2(),
            self.swl(),
            self.dwl(),
            self.reserved_3(),
            self.iien(),
            self.roien(),
            self.ruien(),
            self.toien(),
            self.tuien(),
            self.cks(),
            self.reserved_4()
        )
    }
}
#[doc = "FIFO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssifcr(pub u32);
impl Ssifcr {
    #[doc = "Receive FIFO Data Register Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn rfrst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Data Register Reset"]
    #[inline(always)]
    pub const fn set_rfrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO Data Register Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn tfrst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Data Register Reset"]
    #[inline(always)]
    pub const fn set_tfrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive Data Full Interrupt Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Full Interrupt Output Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit Data Empty Interrupt Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Empty Interrupt Output Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 4usize)) | (((val as u32) & 0x7f) << 4usize);
    }
    #[doc = "Byte Swap Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bsw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Byte Swap Enable"]
    #[inline(always)]
    pub const fn set_bsw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn ssirst(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_ssirst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits are read as 00000000000000. The write value should be 00000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x3fff;
        val as u16
    }
    #[doc = "These bits are read as 00000000000000. The write value should be 00000000000000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 17usize)) | (((val as u32) & 0x3fff) << 17usize);
    }
    #[doc = "AUDIO_MCK Enable in Mastermode Communication"]
    #[must_use]
    #[inline(always)]
    pub const fn aucke(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AUDIO_MCK Enable in Mastermode Communication"]
    #[inline(always)]
    pub const fn set_aucke(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ssifcr {
    #[inline(always)]
    fn default() -> Ssifcr {
        Ssifcr(0)
    }
}
impl core::fmt::Debug for Ssifcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssifcr")
            .field("rfrst", &self.rfrst())
            .field("tfrst", &self.tfrst())
            .field("rie", &self.rie())
            .field("tie", &self.tie())
            .field("reserved", &self.reserved())
            .field("bsw", &self.bsw())
            .field("reserved_2", &self.reserved_2())
            .field("ssirst", &self.ssirst())
            .field("reserved_3", &self.reserved_3())
            .field("aucke", &self.aucke())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssifcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssifcr {{ rfrst: {=bool:?}, tfrst: {=bool:?}, rie: {=bool:?}, tie: {=bool:?}, reserved: {=u8:?}, bsw: {=bool:?}, reserved_2: {=u8:?}, ssirst: {=bool:?}, reserved_3: {=u16:?}, aucke: {=bool:?} }}",
            self.rfrst(),
            self.tfrst(),
            self.rie(),
            self.tie(),
            self.reserved(),
            self.bsw(),
            self.reserved_2(),
            self.ssirst(),
            self.reserved_3(),
            self.aucke()
        )
    }
}
#[doc = "Receive FIFO Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssifrdr(pub u32);
impl Ssifrdr {
    #[doc = "Receive FIFO data."]
    #[must_use]
    #[inline(always)]
    pub const fn ssifrdr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive FIFO data."]
    #[inline(always)]
    pub const fn set_ssifrdr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ssifrdr {
    #[inline(always)]
    fn default() -> Ssifrdr {
        Ssifrdr(0)
    }
}
impl core::fmt::Debug for Ssifrdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssifrdr")
            .field("ssifrdr", &self.ssifrdr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssifrdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ssifrdr {{ ssifrdr: {=u32:?} }}", self.ssifrdr())
    }
}
#[doc = "FIFO Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssifsr(pub u32);
impl Ssifsr {
    #[doc = "Receive Data Full Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rdf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Full Flag"]
    #[inline(always)]
    pub const fn set_rdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "Number of Receive FIFO Data Indication Flag Number of receive FIFO data indication flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rdc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Receive FIFO Data Indication Flag Number of receive FIFO data indication flag."]
    #[inline(always)]
    pub const fn set_rdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Transmit Data Empty Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tde(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Empty Flag"]
    #[inline(always)]
    pub const fn set_tde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "Number of Transmit FIFO Data Indication Flag Number of transmit FIFO data indication flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tdc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Transmit FIFO Data Indication Flag Number of transmit FIFO data indication flag."]
    #[inline(always)]
    pub const fn set_tdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ssifsr {
    #[inline(always)]
    fn default() -> Ssifsr {
        Ssifsr(0)
    }
}
impl core::fmt::Debug for Ssifsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssifsr")
            .field("rdf", &self.rdf())
            .field("reserved", &self.reserved())
            .field("rdc", &self.rdc())
            .field("tde", &self.tde())
            .field("reserved_2", &self.reserved_2())
            .field("tdc", &self.tdc())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssifsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssifsr {{ rdf: {=bool:?}, reserved: {=u8:?}, rdc: {=u8:?}, tde: {=bool:?}, reserved_2: {=u8:?}, tdc: {=u8:?}, reserved_3: {=u8:?} }}",
            self.rdf(),
            self.reserved(),
            self.rdc(),
            self.tde(),
            self.reserved_2(),
            self.tdc(),
            self.reserved_3()
        )
    }
}
#[doc = "Transmit FIFO Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssiftdr(pub u32);
impl Ssiftdr {
    #[doc = "Transmit FIFO Data"]
    #[must_use]
    #[inline(always)]
    pub const fn ssiftdr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit FIFO Data"]
    #[inline(always)]
    pub const fn set_ssiftdr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ssiftdr {
    #[inline(always)]
    fn default() -> Ssiftdr {
        Ssiftdr(0)
    }
}
impl core::fmt::Debug for Ssiftdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssiftdr")
            .field("ssiftdr", &self.ssiftdr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssiftdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ssiftdr {{ ssiftdr: {=u32:?} }}", self.ssiftdr())
    }
}
#[doc = "Status Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssiscr(pub u32);
impl Ssiscr {
    #[doc = "RDF Setting Condition Select"]
    #[must_use]
    #[inline(always)]
    pub const fn rdfs(&self) -> super::vals::Rdfs {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Rdfs::from_bits(val as u8)
    }
    #[doc = "RDF Setting Condition Select"]
    #[inline(always)]
    pub const fn set_rdfs(&mut self, val: super::vals::Rdfs) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
    #[doc = "TDE Setting Condition Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tdes(&self) -> super::vals::Tdes {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tdes::from_bits(val as u8)
    }
    #[doc = "TDE Setting Condition Select"]
    #[inline(always)]
    pub const fn set_tdes(&mut self, val: super::vals::Tdes) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ssiscr {
    #[inline(always)]
    fn default() -> Ssiscr {
        Ssiscr(0)
    }
}
impl core::fmt::Debug for Ssiscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssiscr")
            .field("rdfs", &self.rdfs())
            .field("reserved", &self.reserved())
            .field("tdes", &self.tdes())
            .field("reserved_2", &self.reserved_2())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssiscr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssiscr {{ rdfs: {:?}, reserved: {=u8:?}, tdes: {:?}, reserved_2: {=u8:?}, reserved_3: {=u16:?} }}",
            self.rdfs(),
            self.reserved(),
            self.tdes(),
            self.reserved_2(),
            self.reserved_3()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssisr(pub u32);
impl Ssisr {
    #[doc = "These bits are read as 0000000000000000000000000. The write value should be 0000000000000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "These bits are read as 0000000000000000000000000. The write value should be 0000000000000000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
    }
    #[doc = "Idle Mode Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn iirq(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Idle Mode Status Flag"]
    #[inline(always)]
    pub const fn set_iirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Receive Overflow Error Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn roirq(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Overflow Error Status Flag"]
    #[inline(always)]
    pub const fn set_roirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Receive Underflow Error Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ruirq(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Underflow Error Status Flag"]
    #[inline(always)]
    pub const fn set_ruirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Transmit Overflow Error Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn toirq(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Overflow Error Status Flag"]
    #[inline(always)]
    pub const fn set_toirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Transmit Underflow Error Status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tuirq(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Underflow Error Status flag"]
    #[inline(always)]
    pub const fn set_tuirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ssisr {
    #[inline(always)]
    fn default() -> Ssisr {
        Ssisr(0)
    }
}
impl core::fmt::Debug for Ssisr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssisr")
            .field("reserved", &self.reserved())
            .field("iirq", &self.iirq())
            .field("roirq", &self.roirq())
            .field("ruirq", &self.ruirq())
            .field("toirq", &self.toirq())
            .field("tuirq", &self.tuirq())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssisr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssisr {{ reserved: {=u32:?}, iirq: {=bool:?}, roirq: {=bool:?}, ruirq: {=bool:?}, toirq: {=bool:?}, tuirq: {=bool:?}, reserved_2: {=u8:?} }}",
            self.reserved(),
            self.iirq(),
            self.roirq(),
            self.ruirq(),
            self.toirq(),
            self.tuirq(),
            self.reserved_2()
        )
    }
}
#[doc = "TDM Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssitdmr(pub u32);
impl Ssitdmr {
    #[doc = "Audio Format Select"]
    #[must_use]
    #[inline(always)]
    pub const fn omod(&self) -> super::vals::Omod {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Omod::from_bits(val as u8)
    }
    #[doc = "Audio Format Select"]
    #[inline(always)]
    pub const fn set_omod(&mut self, val: super::vals::Omod) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "Whether to Enable LRCK/FS Continuation"]
    #[must_use]
    #[inline(always)]
    pub const fn lrcont(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Whether to Enable LRCK/FS Continuation"]
    #[inline(always)]
    pub const fn set_lrcont(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Whether to Enable Stopping BCK Output When SSIE is in Idle Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bckastp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Whether to Enable Stopping BCK Output When SSIE is in Idle Status"]
    #[inline(always)]
    pub const fn set_bckastp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "These bits are read as 0000000000000000000000. The write value should be 0000000000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "These bits are read as 0000000000000000000000. The write value should be 0000000000000000000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for Ssitdmr {
    #[inline(always)]
    fn default() -> Ssitdmr {
        Ssitdmr(0)
    }
}
impl core::fmt::Debug for Ssitdmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssitdmr")
            .field("omod", &self.omod())
            .field("reserved", &self.reserved())
            .field("lrcont", &self.lrcont())
            .field("bckastp", &self.bckastp())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssitdmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssitdmr {{ omod: {:?}, reserved: {=u8:?}, lrcont: {=bool:?}, bckastp: {=bool:?}, reserved_2: {=u32:?} }}",
            self.omod(),
            self.reserved(),
            self.lrcont(),
            self.bckastp(),
            self.reserved_2()
        )
    }
}
