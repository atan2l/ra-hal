#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssicr(pub u32);
impl Ssicr {
    #[doc = "Receive Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ren(&self) -> super::vals::Ren {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ren::from_bits(val as u8)
    }
    #[doc = "Receive Enable"]
    #[inline(always)]
    pub const fn set_ren(&mut self, val: super::vals::Ren) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ten(&self) -> super::vals::Ten {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ten::from_bits(val as u8)
    }
    #[doc = "Transmit Enable"]
    #[inline(always)]
    pub const fn set_ten(&mut self, val: super::vals::Ten) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
    pub const fn muen(&self) -> super::vals::Muen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Muen::from_bits(val as u8)
    }
    #[doc = "Mute Enable"]
    #[inline(always)]
    pub const fn set_muen(&mut self, val: super::vals::Muen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
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
    pub const fn del(&self) -> super::vals::Del {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Del::from_bits(val as u8)
    }
    #[doc = "Selects Serial Data Delay"]
    #[inline(always)]
    pub const fn set_del(&mut self, val: super::vals::Del) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Selects Placement Data Alignment"]
    #[must_use]
    #[inline(always)]
    pub const fn pdta(&self) -> super::vals::Pdta {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdta::from_bits(val as u8)
    }
    #[doc = "Selects Placement Data Alignment"]
    #[inline(always)]
    pub const fn set_pdta(&mut self, val: super::vals::Pdta) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Selects Serial Data Alignment"]
    #[must_use]
    #[inline(always)]
    pub const fn sdta(&self) -> super::vals::Sdta {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Sdta::from_bits(val as u8)
    }
    #[doc = "Selects Serial Data Alignment"]
    #[inline(always)]
    pub const fn set_sdta(&mut self, val: super::vals::Sdta) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Selects Serial Padding Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn spdp(&self) -> super::vals::Spdp {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Spdp::from_bits(val as u8)
    }
    #[doc = "Selects Serial Padding Polarity"]
    #[inline(always)]
    pub const fn set_spdp(&mut self, val: super::vals::Spdp) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn lrckp(&self) -> super::vals::Lrckp {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Lrckp::from_bits(val as u8)
    }
    #[doc = "Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal"]
    #[inline(always)]
    pub const fn set_lrckp(&mut self, val: super::vals::Lrckp) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Selects Bit Clock Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn bckp(&self) -> super::vals::Bckp {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Bckp::from_bits(val as u8)
    }
    #[doc = "Selects Bit Clock Polarity"]
    #[inline(always)]
    pub const fn set_bckp(&mut self, val: super::vals::Bckp) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Master Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mst(&self) -> super::vals::Mst {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Mst::from_bits(val as u8)
    }
    #[doc = "Master Enable"]
    #[inline(always)]
    pub const fn set_mst(&mut self, val: super::vals::Mst) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
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
    pub const fn iien(&self) -> super::vals::Iien {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Iien::from_bits(val as u8)
    }
    #[doc = "Idle Mode Interrupt Output Enable"]
    #[inline(always)]
    pub const fn set_iien(&mut self, val: super::vals::Iien) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Receive Overflow Interrupt Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn roien(&self) -> super::vals::Roien {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Roien::from_bits(val as u8)
    }
    #[doc = "Receive Overflow Interrupt Output Enable"]
    #[inline(always)]
    pub const fn set_roien(&mut self, val: super::vals::Roien) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Receive Underflow Interrupt Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ruien(&self) -> super::vals::Ruien {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Ruien::from_bits(val as u8)
    }
    #[doc = "Receive Underflow Interrupt Output Enable"]
    #[inline(always)]
    pub const fn set_ruien(&mut self, val: super::vals::Ruien) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Transmit Overflow Interrupt Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn toien(&self) -> super::vals::Toien {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Toien::from_bits(val as u8)
    }
    #[doc = "Transmit Overflow Interrupt Output Enable"]
    #[inline(always)]
    pub const fn set_toien(&mut self, val: super::vals::Toien) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Transmit Underflow Interrupt Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tuien(&self) -> super::vals::Tuien {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Tuien::from_bits(val as u8)
    }
    #[doc = "Transmit Underflow Interrupt Output Enable"]
    #[inline(always)]
    pub const fn set_tuien(&mut self, val: super::vals::Tuien) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Selects an Audio Clock for Master-mode Communication"]
    #[must_use]
    #[inline(always)]
    pub const fn cks(&self) -> super::vals::Cks {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Cks::from_bits(val as u8)
    }
    #[doc = "Selects an Audio Clock for Master-mode Communication"]
    #[inline(always)]
    pub const fn set_cks(&mut self, val: super::vals::Cks) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
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
            "Ssicr {{ ren: {:?}, ten: {:?}, reserved: {=bool:?}, muen: {:?}, ckdv: {:?}, del: {:?}, pdta: {:?}, sdta: {:?}, spdp: {:?}, lrckp: {:?}, bckp: {:?}, mst: {:?}, reserved_2: {=bool:?}, swl: {:?}, dwl: {:?}, reserved_3: {=u8:?}, iien: {:?}, roien: {:?}, ruien: {:?}, toien: {:?}, tuien: {:?}, cks: {:?}, reserved_4: {=bool:?} }}",
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
    pub const fn rfrst(&self) -> super::vals::Rfrst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rfrst::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Data Register Reset"]
    #[inline(always)]
    pub const fn set_rfrst(&mut self, val: super::vals::Rfrst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO Data Register Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn tfrst(&self) -> super::vals::Tfrst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tfrst::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Data Register Reset"]
    #[inline(always)]
    pub const fn set_tfrst(&mut self, val: super::vals::Tfrst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive Data Full Interrupt Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> super::vals::Rie {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Rie::from_bits(val as u8)
    }
    #[doc = "Receive Data Full Interrupt Output Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: super::vals::Rie) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit Data Empty Interrupt Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> super::vals::Tie {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tie::from_bits(val as u8)
    }
    #[doc = "Transmit Data Empty Interrupt Output Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: super::vals::Tie) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
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
    pub const fn bsw(&self) -> super::vals::Bsw {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Bsw::from_bits(val as u8)
    }
    #[doc = "Byte Swap Enable"]
    #[inline(always)]
    pub const fn set_bsw(&mut self, val: super::vals::Bsw) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
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
    pub const fn ssirst(&self) -> super::vals::Ssirst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ssirst::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_ssirst(&mut self, val: super::vals::Ssirst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
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
    pub const fn aucke(&self) -> super::vals::Aucke {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Aucke::from_bits(val as u8)
    }
    #[doc = "AUDIO_MCK Enable in Mastermode Communication"]
    #[inline(always)]
    pub const fn set_aucke(&mut self, val: super::vals::Aucke) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            "Ssifcr {{ rfrst: {:?}, tfrst: {:?}, rie: {:?}, tie: {:?}, reserved: {=u8:?}, bsw: {:?}, reserved_2: {=u8:?}, ssirst: {:?}, reserved_3: {=u16:?}, aucke: {:?} }}",
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
    pub const fn rdf(&self) -> super::vals::Rdf {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rdf::from_bits(val as u8)
    }
    #[doc = "Receive Data Full Flag"]
    #[inline(always)]
    pub const fn set_rdf(&mut self, val: super::vals::Rdf) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
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
    pub const fn tde(&self) -> super::vals::Tde {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Tde::from_bits(val as u8)
    }
    #[doc = "Transmit Data Empty Flag"]
    #[inline(always)]
    pub const fn set_tde(&mut self, val: super::vals::Tde) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
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
            "Ssifsr {{ rdf: {:?}, reserved: {=u8:?}, rdc: {=u8:?}, tde: {:?}, reserved_2: {=u8:?}, tdc: {=u8:?}, reserved_3: {=u8:?} }}",
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
    pub const fn iirq(&self) -> super::vals::Iirq {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Iirq::from_bits(val as u8)
    }
    #[doc = "Idle Mode Status Flag"]
    #[inline(always)]
    pub const fn set_iirq(&mut self, val: super::vals::Iirq) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Receive Overflow Error Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn roirq(&self) -> super::vals::Roirq {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Roirq::from_bits(val as u8)
    }
    #[doc = "Receive Overflow Error Status Flag"]
    #[inline(always)]
    pub const fn set_roirq(&mut self, val: super::vals::Roirq) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Receive Underflow Error Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ruirq(&self) -> super::vals::Ruirq {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Ruirq::from_bits(val as u8)
    }
    #[doc = "Receive Underflow Error Status Flag"]
    #[inline(always)]
    pub const fn set_ruirq(&mut self, val: super::vals::Ruirq) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Transmit Overflow Error Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn toirq(&self) -> super::vals::Toirq {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Toirq::from_bits(val as u8)
    }
    #[doc = "Transmit Overflow Error Status Flag"]
    #[inline(always)]
    pub const fn set_toirq(&mut self, val: super::vals::Toirq) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Transmit Underflow Error Status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tuirq(&self) -> super::vals::Tuirq {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Tuirq::from_bits(val as u8)
    }
    #[doc = "Transmit Underflow Error Status flag"]
    #[inline(always)]
    pub const fn set_tuirq(&mut self, val: super::vals::Tuirq) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
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
            "Ssisr {{ reserved: {=u32:?}, iirq: {:?}, roirq: {:?}, ruirq: {:?}, toirq: {:?}, tuirq: {:?}, reserved_2: {=u8:?} }}",
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
    pub const fn lrcont(&self) -> super::vals::Lrcont {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Lrcont::from_bits(val as u8)
    }
    #[doc = "Whether to Enable LRCK/FS Continuation"]
    #[inline(always)]
    pub const fn set_lrcont(&mut self, val: super::vals::Lrcont) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Whether to Enable Stopping BCK Output When SSIE is in Idle Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bckastp(&self) -> super::vals::Bckastp {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Bckastp::from_bits(val as u8)
    }
    #[doc = "Whether to Enable Stopping BCK Output When SSIE is in Idle Status"]
    #[inline(always)]
    pub const fn set_bckastp(&mut self, val: super::vals::Bckastp) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
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
            "Ssitdmr {{ omod: {:?}, reserved: {=u8:?}, lrcont: {:?}, bckastp: {:?}, reserved_2: {=u32:?} }}",
            self.omod(),
            self.reserved(),
            self.lrcont(),
            self.bckastp(),
            self.reserved_2()
        )
    }
}
