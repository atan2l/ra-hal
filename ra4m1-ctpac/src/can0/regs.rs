#[doc = "Acceptance Filter Support Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afsr(pub u16);
impl Afsr {
    #[doc = "After the standard ID of a received message is written, the value converted for data table search can be read."]
    #[must_use]
    #[inline(always)]
    pub const fn afsr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "After the standard ID of a received message is written, the value converted for data table search can be read."]
    #[inline(always)]
    pub const fn set_afsr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Afsr {
    #[inline(always)]
    fn default() -> Afsr {
        Afsr(0)
    }
}
impl core::fmt::Debug for Afsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Afsr").field("afsr", &self.afsr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Afsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Afsr {{ afsr: {=u16:?} }}", self.afsr())
    }
}
#[doc = "Bit Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcr(pub u32);
impl Bcr {
    #[doc = "CAN Clock Source Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cclks(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CAN Clock Source Selection"]
    #[inline(always)]
    pub const fn set_cclks(&mut self, val: bool) {
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
    #[doc = "Time Segment 2 Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tseg2(&self) -> super::vals::Tseg2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tseg2::from_bits(val as u8)
    }
    #[doc = "Time Segment 2 Control"]
    #[inline(always)]
    pub const fn set_tseg2(&mut self, val: super::vals::Tseg2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Resynchronization Jump Width Control"]
    #[must_use]
    #[inline(always)]
    pub const fn sjw(&self) -> super::vals::Sjw {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Sjw::from_bits(val as u8)
    }
    #[doc = "Resynchronization Jump Width Control"]
    #[inline(always)]
    pub const fn set_sjw(&mut self, val: super::vals::Sjw) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Prescaler Division Ratio Select . These bits set the frequency of the CAN communication clock (fCANCLK)."]
    #[must_use]
    #[inline(always)]
    pub const fn brp(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Prescaler Division Ratio Select . These bits set the frequency of the CAN communication clock (fCANCLK)."]
    #[inline(always)]
    pub const fn set_brp(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Time Segment 1 Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tseg1(&self) -> super::vals::Tseg1 {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Tseg1::from_bits(val as u8)
    }
    #[doc = "Time Segment 1 Control"]
    #[inline(always)]
    pub const fn set_tseg1(&mut self, val: super::vals::Tseg1) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Bcr {
    #[inline(always)]
    fn default() -> Bcr {
        Bcr(0)
    }
}
impl core::fmt::Debug for Bcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bcr")
            .field("cclks", &self.cclks())
            .field("reserved", &self.reserved())
            .field("tseg2", &self.tseg2())
            .field("reserved_2", &self.reserved_2())
            .field("sjw", &self.sjw())
            .field("reserved_3", &self.reserved_3())
            .field("brp", &self.brp())
            .field("reserved_4", &self.reserved_4())
            .field("tseg1", &self.tseg1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bcr {{ cclks: {=bool:?}, reserved: {=u8:?}, tseg2: {:?}, reserved_2: {=bool:?}, sjw: {:?}, reserved_3: {=u8:?}, brp: {=u16:?}, reserved_4: {=u8:?}, tseg1: {:?} }}",
            self.cclks(),
            self.reserved(),
            self.tseg2(),
            self.reserved_2(),
            self.sjw(),
            self.reserved_3(),
            self.brp(),
            self.reserved_4(),
            self.tseg1()
        )
    }
}
#[doc = "Channel Search Support Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cssr(pub u8);
impl Cssr {
    #[doc = "When the value for the channel search is input, the channel number is output to MSSR."]
    #[must_use]
    #[inline(always)]
    pub const fn cssr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "When the value for the channel search is input, the channel number is output to MSSR."]
    #[inline(always)]
    pub const fn set_cssr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Cssr {
    #[inline(always)]
    fn default() -> Cssr {
        Cssr(0)
    }
}
impl core::fmt::Debug for Cssr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cssr").field("cssr", &self.cssr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cssr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cssr {{ cssr: {=u8:?} }}", self.cssr())
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctlr(pub u16);
impl Ctlr {
    #[doc = "CAN Mailbox Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mbm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CAN Mailbox Mode Select"]
    #[inline(always)]
    pub const fn set_mbm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "ID Format Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn idfm(&self) -> super::vals::Idfm {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Idfm::from_bits(val as u8)
    }
    #[doc = "ID Format Mode Select"]
    #[inline(always)]
    pub const fn set_idfm(&mut self, val: super::vals::Idfm) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u16) & 0x03) << 1usize);
    }
    #[doc = "Message Lost Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mlm(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Message Lost Mode Select"]
    #[inline(always)]
    pub const fn set_mlm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Transmission Priority Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tpm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Priority Mode Select"]
    #[inline(always)]
    pub const fn set_tpm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Time Stamp Counter Reset Command"]
    #[must_use]
    #[inline(always)]
    pub const fn tsrc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Time Stamp Counter Reset Command"]
    #[inline(always)]
    pub const fn set_tsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Time Stamp Prescaler Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tsps(&self) -> super::vals::Tsps {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Tsps::from_bits(val as u8)
    }
    #[doc = "Time Stamp Prescaler Select"]
    #[inline(always)]
    pub const fn set_tsps(&mut self, val: super::vals::Tsps) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "CAN Operating Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn canm(&self) -> super::vals::Canm {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Canm::from_bits(val as u8)
    }
    #[doc = "CAN Operating Mode Select"]
    #[inline(always)]
    pub const fn set_canm(&mut self, val: super::vals::Canm) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "CAN Sleep Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn slpm(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CAN Sleep Mode"]
    #[inline(always)]
    pub const fn set_slpm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Bus-Off Recovery Mode by a program request"]
    #[must_use]
    #[inline(always)]
    pub const fn bom(&self) -> super::vals::Bom {
        let val = (self.0 >> 11usize) & 0x03;
        super::vals::Bom::from_bits(val as u8)
    }
    #[doc = "Bus-Off Recovery Mode by a program request"]
    #[inline(always)]
    pub const fn set_bom(&mut self, val: super::vals::Bom) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u16) & 0x03) << 11usize);
    }
    #[doc = "Forcible Return From Bus-Off"]
    #[must_use]
    #[inline(always)]
    pub const fn rboc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Forcible Return From Bus-Off"]
    #[inline(always)]
    pub const fn set_rboc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
    }
}
impl Default for Ctlr {
    #[inline(always)]
    fn default() -> Ctlr {
        Ctlr(0)
    }
}
impl core::fmt::Debug for Ctlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctlr")
            .field("mbm", &self.mbm())
            .field("idfm", &self.idfm())
            .field("mlm", &self.mlm())
            .field("tpm", &self.tpm())
            .field("tsrc", &self.tsrc())
            .field("tsps", &self.tsps())
            .field("canm", &self.canm())
            .field("slpm", &self.slpm())
            .field("bom", &self.bom())
            .field("rboc", &self.rboc())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctlr {{ mbm: {=bool:?}, idfm: {:?}, mlm: {=bool:?}, tpm: {=bool:?}, tsrc: {=bool:?}, tsps: {:?}, canm: {:?}, slpm: {=bool:?}, bom: {:?}, rboc: {=bool:?}, reserved: {=u8:?} }}",
            self.mbm(),
            self.idfm(),
            self.mlm(),
            self.tpm(),
            self.tsrc(),
            self.tsps(),
            self.canm(),
            self.slpm(),
            self.bom(),
            self.rboc(),
            self.reserved()
        )
    }
}
#[doc = "Error Code Store Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecsr(pub u8);
impl Ecsr {
    #[doc = "Stuff Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sef(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Stuff Error Flag"]
    #[inline(always)]
    pub const fn set_sef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Form Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fef(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Form Error Flag"]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "ACK Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn aef(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ACK Error Flag"]
    #[inline(always)]
    pub const fn set_aef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "CRC Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cef(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Error Flag"]
    #[inline(always)]
    pub const fn set_cef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Bit Error (recessive) Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn be1f(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Error (recessive) Flag"]
    #[inline(always)]
    pub const fn set_be1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Bit Error (dominant) Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn be0f(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Error (dominant) Flag"]
    #[inline(always)]
    pub const fn set_be0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "ACK Delimiter Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn adef(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "ACK Delimiter Error Flag"]
    #[inline(always)]
    pub const fn set_adef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Error Display Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn edpm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Error Display Mode Select"]
    #[inline(always)]
    pub const fn set_edpm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Ecsr {
    #[inline(always)]
    fn default() -> Ecsr {
        Ecsr(0)
    }
}
impl core::fmt::Debug for Ecsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ecsr")
            .field("sef", &self.sef())
            .field("fef", &self.fef())
            .field("aef", &self.aef())
            .field("cef", &self.cef())
            .field("be1f", &self.be1f())
            .field("be0f", &self.be0f())
            .field("adef", &self.adef())
            .field("edpm", &self.edpm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ecsr {{ sef: {=bool:?}, fef: {=bool:?}, aef: {=bool:?}, cef: {=bool:?}, be1f: {=bool:?}, be0f: {=bool:?}, adef: {=bool:?}, edpm: {=bool:?} }}",
            self.sef(),
            self.fef(),
            self.aef(),
            self.cef(),
            self.be1f(),
            self.be0f(),
            self.adef(),
            self.edpm()
        )
    }
}
#[doc = "Error Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eier(pub u8);
impl Eier {
    #[doc = "Bus Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn beie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_beie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Error-Warning Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ewie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Error-Warning Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ewie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Error-Passive Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn epie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error-Passive Interrupt Enable"]
    #[inline(always)]
    pub const fn set_epie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Bus-Off Entry Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn boeie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bus-Off Entry Interrupt Enable"]
    #[inline(always)]
    pub const fn set_boeie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Bus-Off Recovery Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn borie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Bus-Off Recovery Interrupt Enable"]
    #[inline(always)]
    pub const fn set_borie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Overrun Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn orie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Interrupt Enable"]
    #[inline(always)]
    pub const fn set_orie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Overload Frame Transmit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn olie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Overload Frame Transmit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_olie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Bus Lock Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn blie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Lock Interrupt Enable"]
    #[inline(always)]
    pub const fn set_blie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Eier {
    #[inline(always)]
    fn default() -> Eier {
        Eier(0)
    }
}
impl core::fmt::Debug for Eier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eier")
            .field("beie", &self.beie())
            .field("ewie", &self.ewie())
            .field("epie", &self.epie())
            .field("boeie", &self.boeie())
            .field("borie", &self.borie())
            .field("orie", &self.orie())
            .field("olie", &self.olie())
            .field("blie", &self.blie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eier {{ beie: {=bool:?}, ewie: {=bool:?}, epie: {=bool:?}, boeie: {=bool:?}, borie: {=bool:?}, orie: {=bool:?}, olie: {=bool:?}, blie: {=bool:?} }}",
            self.beie(),
            self.ewie(),
            self.epie(),
            self.boeie(),
            self.borie(),
            self.orie(),
            self.olie(),
            self.blie()
        )
    }
}
#[doc = "Error Interrupt Factor Judge Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eifr(pub u8);
impl Eifr {
    #[doc = "Bus Error Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn beif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Error Detect Flag"]
    #[inline(always)]
    pub const fn set_beif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Error-Warning Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ewif(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Error-Warning Detect Flag"]
    #[inline(always)]
    pub const fn set_ewif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Error-Passive Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn epif(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error-Passive Detect Flag"]
    #[inline(always)]
    pub const fn set_epif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Bus-Off Entry Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn boeif(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bus-Off Entry Detect Flag"]
    #[inline(always)]
    pub const fn set_boeif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Bus-Off Recovery Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn borif(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Bus-Off Recovery Detect Flag"]
    #[inline(always)]
    pub const fn set_borif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Receive Overrun Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn orif(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Overrun Detect Flag"]
    #[inline(always)]
    pub const fn set_orif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Overload Frame Transmission Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn olif(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Overload Frame Transmission Detect Flag"]
    #[inline(always)]
    pub const fn set_olif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Bus Lock Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn blif(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Lock Detect Flag"]
    #[inline(always)]
    pub const fn set_blif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Eifr {
    #[inline(always)]
    fn default() -> Eifr {
        Eifr(0)
    }
}
impl core::fmt::Debug for Eifr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eifr")
            .field("beif", &self.beif())
            .field("ewif", &self.ewif())
            .field("epif", &self.epif())
            .field("boeif", &self.boeif())
            .field("borif", &self.borif())
            .field("orif", &self.orif())
            .field("olif", &self.olif())
            .field("blif", &self.blif())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eifr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eifr {{ beif: {=bool:?}, ewif: {=bool:?}, epif: {=bool:?}, boeif: {=bool:?}, borif: {=bool:?}, orif: {=bool:?}, olif: {=bool:?}, blif: {=bool:?} }}",
            self.beif(),
            self.ewif(),
            self.epif(),
            self.boeif(),
            self.borif(),
            self.orif(),
            self.olif(),
            self.blif()
        )
    }
}
#[doc = "FIFO Received ID Compare Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fidcr(pub u32);
impl Fidcr {
    #[doc = "Extended ID"]
    #[must_use]
    #[inline(always)]
    pub const fn eid(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Extended ID"]
    #[inline(always)]
    pub const fn set_eid(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Standard ID"]
    #[must_use]
    #[inline(always)]
    pub const fn sid(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Standard ID"]
    #[inline(always)]
    pub const fn set_sid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Remote Transmission Request"]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Transmission Request"]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "ID Extension"]
    #[must_use]
    #[inline(always)]
    pub const fn ide(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "ID Extension"]
    #[inline(always)]
    pub const fn set_ide(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Fidcr {
    #[inline(always)]
    fn default() -> Fidcr {
        Fidcr(0)
    }
}
impl core::fmt::Debug for Fidcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fidcr")
            .field("eid", &self.eid())
            .field("sid", &self.sid())
            .field("reserved", &self.reserved())
            .field("rtr", &self.rtr())
            .field("ide", &self.ide())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fidcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fidcr {{ eid: {=u32:?}, sid: {=u16:?}, reserved: {=bool:?}, rtr: {=bool:?}, ide: {=bool:?} }}",
            self.eid(),
            self.sid(),
            self.reserved(),
            self.rtr(),
            self.ide()
        )
    }
}
#[doc = "Mailbox Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD0(pub u8);
impl MbD0 {
    #[doc = "Data Bytes 0. DATA0 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Bytes 0. DATA0 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for MbD0 {
    #[inline(always)]
    fn default() -> MbD0 {
        MbD0(0)
    }
}
impl core::fmt::Debug for MbD0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbD0")
            .field("data0", &self.data0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbD0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MbD0 {{ data0: {=u8:?} }}", self.data0())
    }
}
#[doc = "Mailbox Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD1(pub u8);
impl MbD1 {
    #[doc = "Data Bytes 1 DATA1 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Bytes 1 DATA1 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for MbD1 {
    #[inline(always)]
    fn default() -> MbD1 {
        MbD1(0)
    }
}
impl core::fmt::Debug for MbD1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbD1")
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbD1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MbD1 {{ data1: {=u8:?} }}", self.data1())
    }
}
#[doc = "Mailbox Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD2(pub u8);
impl MbD2 {
    #[doc = "Data Bytes 2 DATA2 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Bytes 2 DATA2 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for MbD2 {
    #[inline(always)]
    fn default() -> MbD2 {
        MbD2(0)
    }
}
impl core::fmt::Debug for MbD2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbD2")
            .field("data2", &self.data2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbD2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MbD2 {{ data2: {=u8:?} }}", self.data2())
    }
}
#[doc = "Mailbox Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD3(pub u8);
impl MbD3 {
    #[doc = "Data Bytes 3 DATA3 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Bytes 3 DATA3 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for MbD3 {
    #[inline(always)]
    fn default() -> MbD3 {
        MbD3(0)
    }
}
impl core::fmt::Debug for MbD3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbD3")
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbD3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MbD3 {{ data3: {=u8:?} }}", self.data3())
    }
}
#[doc = "Mailbox Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD4(pub u8);
impl MbD4 {
    #[doc = "Data Bytes 4 DATA4 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Bytes 4 DATA4 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for MbD4 {
    #[inline(always)]
    fn default() -> MbD4 {
        MbD4(0)
    }
}
impl core::fmt::Debug for MbD4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbD4")
            .field("data4", &self.data4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbD4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MbD4 {{ data4: {=u8:?} }}", self.data4())
    }
}
#[doc = "Mailbox Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD5(pub u8);
impl MbD5 {
    #[doc = "Data Bytes 5 DATA5 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Bytes 5 DATA5 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for MbD5 {
    #[inline(always)]
    fn default() -> MbD5 {
        MbD5(0)
    }
}
impl core::fmt::Debug for MbD5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbD5")
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbD5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MbD5 {{ data5: {=u8:?} }}", self.data5())
    }
}
#[doc = "Mailbox Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD6(pub u8);
impl MbD6 {
    #[doc = "Data Bytes 6 DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Bytes 6 DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for MbD6 {
    #[inline(always)]
    fn default() -> MbD6 {
        MbD6(0)
    }
}
impl core::fmt::Debug for MbD6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbD6")
            .field("data6", &self.data6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbD6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MbD6 {{ data6: {=u8:?} }}", self.data6())
    }
}
#[doc = "Mailbox Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD7(pub u8);
impl MbD7 {
    #[doc = "Data Bytes 7 DATA7 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Bytes 7 DATA7 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for MbD7 {
    #[inline(always)]
    fn default() -> MbD7 {
        MbD7(0)
    }
}
impl core::fmt::Debug for MbD7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbD7")
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbD7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MbD7 {{ data7: {=u8:?} }}", self.data7())
    }
}
#[doc = "Mailbox Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbDl(pub u16);
impl MbDl {
    #[doc = "Data Length Code"]
    #[must_use]
    #[inline(always)]
    pub const fn dlc(&self) -> super::vals::Dlc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Dlc::from_bits(val as u8)
    }
    #[doc = "Data Length Code"]
    #[inline(always)]
    pub const fn set_dlc(&mut self, val: super::vals::Dlc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u16) & 0x0fff) << 4usize);
    }
}
impl Default for MbDl {
    #[inline(always)]
    fn default() -> MbDl {
        MbDl(0)
    }
}
impl core::fmt::Debug for MbDl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbDl")
            .field("dlc", &self.dlc())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbDl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MbDl {{ dlc: {:?}, reserved: {=u16:?} }}",
            self.dlc(),
            self.reserved()
        )
    }
}
#[doc = "Mailbox Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbId(pub u32);
impl MbId {
    #[doc = "Extended ID"]
    #[must_use]
    #[inline(always)]
    pub const fn eid(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Extended ID"]
    #[inline(always)]
    pub const fn set_eid(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Standard ID"]
    #[must_use]
    #[inline(always)]
    pub const fn sid(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Standard ID"]
    #[inline(always)]
    pub const fn set_sid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Remote Transmission Request"]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Transmission Request"]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "ID Extension"]
    #[must_use]
    #[inline(always)]
    pub const fn ide(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "ID Extension"]
    #[inline(always)]
    pub const fn set_ide(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MbId {
    #[inline(always)]
    fn default() -> MbId {
        MbId(0)
    }
}
impl core::fmt::Debug for MbId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbId")
            .field("eid", &self.eid())
            .field("sid", &self.sid())
            .field("reserved", &self.reserved())
            .field("rtr", &self.rtr())
            .field("ide", &self.ide())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MbId {{ eid: {=u32:?}, sid: {=u16:?}, reserved: {=bool:?}, rtr: {=bool:?}, ide: {=bool:?} }}",
            self.eid(),
            self.sid(),
            self.reserved(),
            self.rtr(),
            self.ide()
        )
    }
}
#[doc = "Mailbox Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbTs(pub u16);
impl MbTs {
    #[doc = "Time Stamp Higher Byte Bits TSL\\[7:0\\] store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[must_use]
    #[inline(always)]
    pub const fn tsl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Time Stamp Higher Byte Bits TSL\\[7:0\\] store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    pub const fn set_tsl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Time Stamp Lower Byte Bits TSH\\[7:0\\] store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[must_use]
    #[inline(always)]
    pub const fn tsh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Time Stamp Lower Byte Bits TSH\\[7:0\\] store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    pub const fn set_tsh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for MbTs {
    #[inline(always)]
    fn default() -> MbTs {
        MbTs(0)
    }
}
impl core::fmt::Debug for MbTs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbTs")
            .field("tsl", &self.tsl())
            .field("tsh", &self.tsh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbTs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MbTs {{ tsl: {=u8:?}, tsh: {=u8:?} }}",
            self.tsl(),
            self.tsh()
        )
    }
}
#[doc = "Message Control Register for Receive"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MctlRx(pub u8);
impl MctlRx {
    #[doc = "Reception Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn newdata(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reception Complete Flag"]
    #[inline(always)]
    pub const fn set_newdata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Reception-in-Progress Status Flag (Receive mailbox setting enabled)"]
    #[must_use]
    #[inline(always)]
    pub const fn invaldata(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reception-in-Progress Status Flag (Receive mailbox setting enabled)"]
    #[inline(always)]
    pub const fn set_invaldata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Message Lost Flag (Receive mailbox setting enabled)"]
    #[must_use]
    #[inline(always)]
    pub const fn msglost(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Message Lost Flag (Receive mailbox setting enabled)"]
    #[inline(always)]
    pub const fn set_msglost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "One-Shot Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshot(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "One-Shot Enable"]
    #[inline(always)]
    pub const fn set_oneshot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Receive Mailbox Request"]
    #[must_use]
    #[inline(always)]
    pub const fn recreq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Mailbox Request"]
    #[inline(always)]
    pub const fn set_recreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Transmit Mailbox Request"]
    #[must_use]
    #[inline(always)]
    pub const fn trmreq(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Mailbox Request"]
    #[inline(always)]
    pub const fn set_trmreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for MctlRx {
    #[inline(always)]
    fn default() -> MctlRx {
        MctlRx(0)
    }
}
impl core::fmt::Debug for MctlRx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MctlRx")
            .field("newdata", &self.newdata())
            .field("invaldata", &self.invaldata())
            .field("msglost", &self.msglost())
            .field("reserved", &self.reserved())
            .field("oneshot", &self.oneshot())
            .field("reserved_2", &self.reserved_2())
            .field("recreq", &self.recreq())
            .field("trmreq", &self.trmreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MctlRx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MctlRx {{ newdata: {=bool:?}, invaldata: {=bool:?}, msglost: {=bool:?}, reserved: {=bool:?}, oneshot: {=bool:?}, reserved_2: {=bool:?}, recreq: {=bool:?}, trmreq: {=bool:?} }}",
            self.newdata(),
            self.invaldata(),
            self.msglost(),
            self.reserved(),
            self.oneshot(),
            self.reserved_2(),
            self.recreq(),
            self.trmreq()
        )
    }
}
#[doc = "Message Control Register for Transmit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MctlTx(pub u8);
impl MctlTx {
    #[doc = "Transmission Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sentdata(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Complete Flag"]
    #[inline(always)]
    pub const fn set_sentdata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Transmission-in-Progress Status Flag (Transmit mailbox setting enabled)"]
    #[must_use]
    #[inline(always)]
    pub const fn trmactive(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission-in-Progress Status Flag (Transmit mailbox setting enabled)"]
    #[inline(always)]
    pub const fn set_trmactive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Transmission Abort Complete Flag (Transmit mailbox setting enabled)"]
    #[must_use]
    #[inline(always)]
    pub const fn trmabt(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Abort Complete Flag (Transmit mailbox setting enabled)"]
    #[inline(always)]
    pub const fn set_trmabt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "One-Shot Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshot(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "One-Shot Enable"]
    #[inline(always)]
    pub const fn set_oneshot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Receive Mailbox Request"]
    #[must_use]
    #[inline(always)]
    pub const fn recreq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Mailbox Request"]
    #[inline(always)]
    pub const fn set_recreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Transmit Mailbox Request"]
    #[must_use]
    #[inline(always)]
    pub const fn trmreq(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Mailbox Request"]
    #[inline(always)]
    pub const fn set_trmreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for MctlTx {
    #[inline(always)]
    fn default() -> MctlTx {
        MctlTx(0)
    }
}
impl core::fmt::Debug for MctlTx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MctlTx")
            .field("sentdata", &self.sentdata())
            .field("trmactive", &self.trmactive())
            .field("trmabt", &self.trmabt())
            .field("reserved", &self.reserved())
            .field("oneshot", &self.oneshot())
            .field("reserved_2", &self.reserved_2())
            .field("recreq", &self.recreq())
            .field("trmreq", &self.trmreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MctlTx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MctlTx {{ sentdata: {=bool:?}, trmactive: {=bool:?}, trmabt: {=bool:?}, reserved: {=bool:?}, oneshot: {=bool:?}, reserved_2: {=bool:?}, recreq: {=bool:?}, trmreq: {=bool:?} }}",
            self.sentdata(),
            self.trmactive(),
            self.trmabt(),
            self.reserved(),
            self.oneshot(),
            self.reserved_2(),
            self.recreq(),
            self.trmreq()
        )
    }
}
#[doc = "Mailbox Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mier(pub u32);
impl Mier {
    #[doc = "mailbox 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "mailbox 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "mailbox 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "mailbox 3 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 3 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "mailbox 4 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 4 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "mailbox 5 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 5 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "mailbox 6 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 6 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "mailbox 7 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 7 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "mailbox 8 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 8 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "mailbox 9 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 9 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "mailbox 10 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 10 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "mailbox 11 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 11 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "mailbox 12 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 12 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "mailbox 13 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 13 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "mailbox 14 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 14 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "mailbox 15 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 15 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "mailbox 16 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 16 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "mailbox 17 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 17 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "mailbox 18 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 18 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "mailbox 19 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 19 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "mailbox 20 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 20 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "mailbox 21 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 21 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "mailbox 22 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 22 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "mailbox 23 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 23 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "mailbox 24 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 24 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "mailbox 25 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 25 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "mailbox 26 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 26 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "mailbox 27 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 27 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "mailbox 28 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 28 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "mailbox 29 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 29 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "mailbox 30 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 30 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "mailbox 31 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 31 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mier {
    #[inline(always)]
    fn default() -> Mier {
        Mier(0)
    }
}
impl core::fmt::Debug for Mier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mier")
            .field("mb0", &self.mb0())
            .field("mb1", &self.mb1())
            .field("mb2", &self.mb2())
            .field("mb3", &self.mb3())
            .field("mb4", &self.mb4())
            .field("mb5", &self.mb5())
            .field("mb6", &self.mb6())
            .field("mb7", &self.mb7())
            .field("mb8", &self.mb8())
            .field("mb9", &self.mb9())
            .field("mb10", &self.mb10())
            .field("mb11", &self.mb11())
            .field("mb12", &self.mb12())
            .field("mb13", &self.mb13())
            .field("mb14", &self.mb14())
            .field("mb15", &self.mb15())
            .field("mb16", &self.mb16())
            .field("mb17", &self.mb17())
            .field("mb18", &self.mb18())
            .field("mb19", &self.mb19())
            .field("mb20", &self.mb20())
            .field("mb21", &self.mb21())
            .field("mb22", &self.mb22())
            .field("mb23", &self.mb23())
            .field("mb24", &self.mb24())
            .field("mb25", &self.mb25())
            .field("mb26", &self.mb26())
            .field("mb27", &self.mb27())
            .field("mb28", &self.mb28())
            .field("mb29", &self.mb29())
            .field("mb30", &self.mb30())
            .field("mb31", &self.mb31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mier {{ mb0: {=bool:?}, mb1: {=bool:?}, mb2: {=bool:?}, mb3: {=bool:?}, mb4: {=bool:?}, mb5: {=bool:?}, mb6: {=bool:?}, mb7: {=bool:?}, mb8: {=bool:?}, mb9: {=bool:?}, mb10: {=bool:?}, mb11: {=bool:?}, mb12: {=bool:?}, mb13: {=bool:?}, mb14: {=bool:?}, mb15: {=bool:?}, mb16: {=bool:?}, mb17: {=bool:?}, mb18: {=bool:?}, mb19: {=bool:?}, mb20: {=bool:?}, mb21: {=bool:?}, mb22: {=bool:?}, mb23: {=bool:?}, mb24: {=bool:?}, mb25: {=bool:?}, mb26: {=bool:?}, mb27: {=bool:?}, mb28: {=bool:?}, mb29: {=bool:?}, mb30: {=bool:?}, mb31: {=bool:?} }}",
            self.mb0(),
            self.mb1(),
            self.mb2(),
            self.mb3(),
            self.mb4(),
            self.mb5(),
            self.mb6(),
            self.mb7(),
            self.mb8(),
            self.mb9(),
            self.mb10(),
            self.mb11(),
            self.mb12(),
            self.mb13(),
            self.mb14(),
            self.mb15(),
            self.mb16(),
            self.mb17(),
            self.mb18(),
            self.mb19(),
            self.mb20(),
            self.mb21(),
            self.mb22(),
            self.mb23(),
            self.mb24(),
            self.mb25(),
            self.mb26(),
            self.mb27(),
            self.mb28(),
            self.mb29(),
            self.mb30(),
            self.mb31()
        )
    }
}
#[doc = "Mailbox Interrupt Enable Register for FIFO Mailbox Mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MierFifo(pub u32);
impl MierFifo {
    #[doc = "mailbox 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "mailbox 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "mailbox 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "mailbox 3 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 3 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "mailbox 4 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 4 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "mailbox 5 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 5 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "mailbox 6 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 6 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "mailbox 7 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 7 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "mailbox 8 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 8 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "mailbox 9 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 9 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "mailbox 10 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 10 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "mailbox 11 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 11 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "mailbox 12 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 12 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "mailbox 13 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 13 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "mailbox 14 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 14 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "mailbox 15 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 15 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "mailbox 16 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 16 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "mailbox 17 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 17 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "mailbox 18 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 18 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "mailbox 19 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 19 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "mailbox 20 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 20 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "mailbox 21 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 21 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "mailbox 22 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 22 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "mailbox 23 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 23 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Transmit FIFO Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Transmit FIFO Interrupt Generation Timing Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mb25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Interrupt Generation Timing Control"]
    #[inline(always)]
    pub const fn set_mb25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Receive FIFO Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Receive FIFO Interrupt Generation Timing Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mb29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Interrupt Generation Timing Control"]
    #[inline(always)]
    pub const fn set_mb29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for MierFifo {
    #[inline(always)]
    fn default() -> MierFifo {
        MierFifo(0)
    }
}
impl core::fmt::Debug for MierFifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MierFifo")
            .field("mb0", &self.mb0())
            .field("mb1", &self.mb1())
            .field("mb2", &self.mb2())
            .field("mb3", &self.mb3())
            .field("mb4", &self.mb4())
            .field("mb5", &self.mb5())
            .field("mb6", &self.mb6())
            .field("mb7", &self.mb7())
            .field("mb8", &self.mb8())
            .field("mb9", &self.mb9())
            .field("mb10", &self.mb10())
            .field("mb11", &self.mb11())
            .field("mb12", &self.mb12())
            .field("mb13", &self.mb13())
            .field("mb14", &self.mb14())
            .field("mb15", &self.mb15())
            .field("mb16", &self.mb16())
            .field("mb17", &self.mb17())
            .field("mb18", &self.mb18())
            .field("mb19", &self.mb19())
            .field("mb20", &self.mb20())
            .field("mb21", &self.mb21())
            .field("mb22", &self.mb22())
            .field("mb23", &self.mb23())
            .field("mb24", &self.mb24())
            .field("mb25", &self.mb25())
            .field("reserved", &self.reserved())
            .field("mb28", &self.mb28())
            .field("mb29", &self.mb29())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MierFifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MierFifo {{ mb0: {=bool:?}, mb1: {=bool:?}, mb2: {=bool:?}, mb3: {=bool:?}, mb4: {=bool:?}, mb5: {=bool:?}, mb6: {=bool:?}, mb7: {=bool:?}, mb8: {=bool:?}, mb9: {=bool:?}, mb10: {=bool:?}, mb11: {=bool:?}, mb12: {=bool:?}, mb13: {=bool:?}, mb14: {=bool:?}, mb15: {=bool:?}, mb16: {=bool:?}, mb17: {=bool:?}, mb18: {=bool:?}, mb19: {=bool:?}, mb20: {=bool:?}, mb21: {=bool:?}, mb22: {=bool:?}, mb23: {=bool:?}, mb24: {=bool:?}, mb25: {=bool:?}, reserved: {=u8:?}, mb28: {=bool:?}, mb29: {=bool:?} }}",
            self.mb0(),
            self.mb1(),
            self.mb2(),
            self.mb3(),
            self.mb4(),
            self.mb5(),
            self.mb6(),
            self.mb7(),
            self.mb8(),
            self.mb9(),
            self.mb10(),
            self.mb11(),
            self.mb12(),
            self.mb13(),
            self.mb14(),
            self.mb15(),
            self.mb16(),
            self.mb17(),
            self.mb18(),
            self.mb19(),
            self.mb20(),
            self.mb21(),
            self.mb22(),
            self.mb23(),
            self.mb24(),
            self.mb25(),
            self.reserved(),
            self.mb28(),
            self.mb29()
        )
    }
}
#[doc = "Mask Invalid Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mkivlr(pub u32);
impl Mkivlr {
    #[doc = "mailbox 0 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 0 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "mailbox 1 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 1 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "mailbox 2 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 2 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "mailbox 3 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 3 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "mailbox 4 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 4 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "mailbox 5 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 5 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "mailbox 6 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 6 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "mailbox 7 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 7 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "mailbox 8 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 8 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "mailbox 9 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 9 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "mailbox 10 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 10 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "mailbox 11 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 11 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "mailbox 12 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 12 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "mailbox 13 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 13 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "mailbox 14 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 14 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "mailbox 15 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 15 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "mailbox 16 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 16 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "mailbox 17 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 17 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "mailbox 18 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 18 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "mailbox 19 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 19 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "mailbox 20 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 20 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "mailbox 21 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 21 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "mailbox 22 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 22 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "mailbox 23 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 23 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "mailbox 24 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 24 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "mailbox 25 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 25 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "mailbox 26 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 26 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "mailbox 27 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 27 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "mailbox 28 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 28 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "mailbox 29 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 29 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "mailbox 30 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 30 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "mailbox 31 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "mailbox 31 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mkivlr {
    #[inline(always)]
    fn default() -> Mkivlr {
        Mkivlr(0)
    }
}
impl core::fmt::Debug for Mkivlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mkivlr")
            .field("mb0", &self.mb0())
            .field("mb1", &self.mb1())
            .field("mb2", &self.mb2())
            .field("mb3", &self.mb3())
            .field("mb4", &self.mb4())
            .field("mb5", &self.mb5())
            .field("mb6", &self.mb6())
            .field("mb7", &self.mb7())
            .field("mb8", &self.mb8())
            .field("mb9", &self.mb9())
            .field("mb10", &self.mb10())
            .field("mb11", &self.mb11())
            .field("mb12", &self.mb12())
            .field("mb13", &self.mb13())
            .field("mb14", &self.mb14())
            .field("mb15", &self.mb15())
            .field("mb16", &self.mb16())
            .field("mb17", &self.mb17())
            .field("mb18", &self.mb18())
            .field("mb19", &self.mb19())
            .field("mb20", &self.mb20())
            .field("mb21", &self.mb21())
            .field("mb22", &self.mb22())
            .field("mb23", &self.mb23())
            .field("mb24", &self.mb24())
            .field("mb25", &self.mb25())
            .field("mb26", &self.mb26())
            .field("mb27", &self.mb27())
            .field("mb28", &self.mb28())
            .field("mb29", &self.mb29())
            .field("mb30", &self.mb30())
            .field("mb31", &self.mb31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mkivlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mkivlr {{ mb0: {=bool:?}, mb1: {=bool:?}, mb2: {=bool:?}, mb3: {=bool:?}, mb4: {=bool:?}, mb5: {=bool:?}, mb6: {=bool:?}, mb7: {=bool:?}, mb8: {=bool:?}, mb9: {=bool:?}, mb10: {=bool:?}, mb11: {=bool:?}, mb12: {=bool:?}, mb13: {=bool:?}, mb14: {=bool:?}, mb15: {=bool:?}, mb16: {=bool:?}, mb17: {=bool:?}, mb18: {=bool:?}, mb19: {=bool:?}, mb20: {=bool:?}, mb21: {=bool:?}, mb22: {=bool:?}, mb23: {=bool:?}, mb24: {=bool:?}, mb25: {=bool:?}, mb26: {=bool:?}, mb27: {=bool:?}, mb28: {=bool:?}, mb29: {=bool:?}, mb30: {=bool:?}, mb31: {=bool:?} }}",
            self.mb0(),
            self.mb1(),
            self.mb2(),
            self.mb3(),
            self.mb4(),
            self.mb5(),
            self.mb6(),
            self.mb7(),
            self.mb8(),
            self.mb9(),
            self.mb10(),
            self.mb11(),
            self.mb12(),
            self.mb13(),
            self.mb14(),
            self.mb15(),
            self.mb16(),
            self.mb17(),
            self.mb18(),
            self.mb19(),
            self.mb20(),
            self.mb21(),
            self.mb22(),
            self.mb23(),
            self.mb24(),
            self.mb25(),
            self.mb26(),
            self.mb27(),
            self.mb28(),
            self.mb29(),
            self.mb30(),
            self.mb31()
        )
    }
}
#[doc = "Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mkr(pub u32);
impl Mkr {
    #[doc = "Extended ID"]
    #[must_use]
    #[inline(always)]
    pub const fn eid(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Extended ID"]
    #[inline(always)]
    pub const fn set_eid(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Standard ID"]
    #[must_use]
    #[inline(always)]
    pub const fn sid(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Standard ID"]
    #[inline(always)]
    pub const fn set_sid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Mkr {
    #[inline(always)]
    fn default() -> Mkr {
        Mkr(0)
    }
}
impl core::fmt::Debug for Mkr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mkr")
            .field("eid", &self.eid())
            .field("sid", &self.sid())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mkr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mkr {{ eid: {=u32:?}, sid: {=u16:?}, reserved: {=u8:?} }}",
            self.eid(),
            self.sid(),
            self.reserved()
        )
    }
}
#[doc = "Mailbox Search Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msmr(pub u8);
impl Msmr {
    #[doc = "Mailbox Search Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mbsm(&self) -> super::vals::Mbsm {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Mbsm::from_bits(val as u8)
    }
    #[doc = "Mailbox Search Mode Select"]
    #[inline(always)]
    pub const fn set_mbsm(&mut self, val: super::vals::Mbsm) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
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
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for Msmr {
    #[inline(always)]
    fn default() -> Msmr {
        Msmr(0)
    }
}
impl core::fmt::Debug for Msmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msmr")
            .field("mbsm", &self.mbsm())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Msmr {{ mbsm: {:?}, reserved: {=u8:?} }}",
            self.mbsm(),
            self.reserved()
        )
    }
}
#[doc = "Mailbox Search Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mssr(pub u8);
impl Mssr {
    #[doc = "Search Result Mailbox Number Status These bits output the smallest mailbox number that is searched in each mode of MSMR."]
    #[must_use]
    #[inline(always)]
    pub const fn mbnst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Search Result Mailbox Number Status These bits output the smallest mailbox number that is searched in each mode of MSMR."]
    #[inline(always)]
    pub const fn set_mbnst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "These bits are read as 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u8) & 0x03) << 5usize);
    }
    #[doc = "Search Result Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sest(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Search Result Status"]
    #[inline(always)]
    pub const fn set_sest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Mssr {
    #[inline(always)]
    fn default() -> Mssr {
        Mssr(0)
    }
}
impl core::fmt::Debug for Mssr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mssr")
            .field("mbnst", &self.mbnst())
            .field("reserved", &self.reserved())
            .field("sest", &self.sest())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mssr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mssr {{ mbnst: {=u8:?}, reserved: {=u8:?}, sest: {=bool:?} }}",
            self.mbnst(),
            self.reserved(),
            self.sest()
        )
    }
}
#[doc = "Receive Error Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Recr(pub u8);
impl Recr {
    #[doc = "Receive error count function RECR increments or decrements the counter value according to the error status of the CAN module during reception."]
    #[must_use]
    #[inline(always)]
    pub const fn recr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive error count function RECR increments or decrements the counter value according to the error status of the CAN module during reception."]
    #[inline(always)]
    pub const fn set_recr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Recr {
    #[inline(always)]
    fn default() -> Recr {
        Recr(0)
    }
}
impl core::fmt::Debug for Recr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Recr").field("recr", &self.recr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Recr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Recr {{ recr: {=u8:?} }}", self.recr())
    }
}
#[doc = "Receive FIFO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfcr(pub u8);
impl Rfcr {
    #[doc = "Receive FIFO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rfe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Enable"]
    #[inline(always)]
    pub const fn set_rfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Receive FIFO Unread Message Number Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rfust(&self) -> super::vals::Rfust {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Rfust::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Unread Message Number Status"]
    #[inline(always)]
    pub const fn set_rfust(&mut self, val: super::vals::Rfust) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u8) & 0x07) << 1usize);
    }
    #[doc = "Receive FIFO Message Lost Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rfmlf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Message Lost Flag"]
    #[inline(always)]
    pub const fn set_rfmlf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Receive FIFO Full Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rffst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Full Status Flag"]
    #[inline(always)]
    pub const fn set_rffst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Receive FIFO Buffer Warning Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rfwst(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Buffer Warning Status Flag"]
    #[inline(always)]
    pub const fn set_rfwst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Receive FIFO Empty Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rfest(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Empty Status Flag"]
    #[inline(always)]
    pub const fn set_rfest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Rfcr {
    #[inline(always)]
    fn default() -> Rfcr {
        Rfcr(0)
    }
}
impl core::fmt::Debug for Rfcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rfcr")
            .field("rfe", &self.rfe())
            .field("rfust", &self.rfust())
            .field("rfmlf", &self.rfmlf())
            .field("rffst", &self.rffst())
            .field("rfwst", &self.rfwst())
            .field("rfest", &self.rfest())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rfcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rfcr {{ rfe: {=bool:?}, rfust: {:?}, rfmlf: {=bool:?}, rffst: {=bool:?}, rfwst: {=bool:?}, rfest: {=bool:?} }}",
            self.rfe(),
            self.rfust(),
            self.rfmlf(),
            self.rffst(),
            self.rfwst(),
            self.rfest()
        )
    }
}
#[doc = "Receive FIFO Pointer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfpcr(pub u8);
impl Rfpcr {
    #[doc = "The CPU-side pointer for the receive FIFO is incremented by writing FFh to RFPCR."]
    #[must_use]
    #[inline(always)]
    pub const fn rfpcr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The CPU-side pointer for the receive FIFO is incremented by writing FFh to RFPCR."]
    #[inline(always)]
    pub const fn set_rfpcr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Rfpcr {
    #[inline(always)]
    fn default() -> Rfpcr {
        Rfpcr(0)
    }
}
impl core::fmt::Debug for Rfpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rfpcr")
            .field("rfpcr", &self.rfpcr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rfpcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rfpcr {{ rfpcr: {=u8:?} }}", self.rfpcr())
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Str(pub u16);
impl Str {
    #[doc = "NEWDATA Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ndst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NEWDATA Status Flag"]
    #[inline(always)]
    pub const fn set_ndst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "SENTDATA Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sdst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SENTDATA Status Flag"]
    #[inline(always)]
    pub const fn set_sdst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rfst(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Status Flag"]
    #[inline(always)]
    pub const fn set_rfst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Transmit FIFO Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tfst(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Status Flag"]
    #[inline(always)]
    pub const fn set_tfst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Normal Mailbox Message Lost Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn nmlst(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Normal Mailbox Message Lost Status Flag"]
    #[inline(always)]
    pub const fn set_nmlst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "FIFO Mailbox Message Lost Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fmlst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Mailbox Message Lost Status Flag"]
    #[inline(always)]
    pub const fn set_fmlst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Transmission Abort Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tabst(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Abort Status Flag"]
    #[inline(always)]
    pub const fn set_tabst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Error Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn est(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Error Status Flag"]
    #[inline(always)]
    pub const fn set_est(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "CAN Reset Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rstst(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CAN Reset Status Flag"]
    #[inline(always)]
    pub const fn set_rstst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "CAN Halt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn hltst(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CAN Halt Status Flag"]
    #[inline(always)]
    pub const fn set_hltst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "CAN Sleep Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn slpst(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CAN Sleep Status Flag"]
    #[inline(always)]
    pub const fn set_slpst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Error-Passive Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn epst(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Error-Passive Status Flag"]
    #[inline(always)]
    pub const fn set_epst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Bus-Off Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bost(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Bus-Off Status Flag"]
    #[inline(always)]
    pub const fn set_bost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Transmit Status Flag (transmitter)"]
    #[must_use]
    #[inline(always)]
    pub const fn trmst(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Status Flag (transmitter)"]
    #[inline(always)]
    pub const fn set_trmst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Receive Status Flag (receiver)"]
    #[must_use]
    #[inline(always)]
    pub const fn recst(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Status Flag (receiver)"]
    #[inline(always)]
    pub const fn set_recst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "This bit is read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Str {
    #[inline(always)]
    fn default() -> Str {
        Str(0)
    }
}
impl core::fmt::Debug for Str {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Str")
            .field("ndst", &self.ndst())
            .field("sdst", &self.sdst())
            .field("rfst", &self.rfst())
            .field("tfst", &self.tfst())
            .field("nmlst", &self.nmlst())
            .field("fmlst", &self.fmlst())
            .field("tabst", &self.tabst())
            .field("est", &self.est())
            .field("rstst", &self.rstst())
            .field("hltst", &self.hltst())
            .field("slpst", &self.slpst())
            .field("epst", &self.epst())
            .field("bost", &self.bost())
            .field("trmst", &self.trmst())
            .field("recst", &self.recst())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Str {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Str {{ ndst: {=bool:?}, sdst: {=bool:?}, rfst: {=bool:?}, tfst: {=bool:?}, nmlst: {=bool:?}, fmlst: {=bool:?}, tabst: {=bool:?}, est: {=bool:?}, rstst: {=bool:?}, hltst: {=bool:?}, slpst: {=bool:?}, epst: {=bool:?}, bost: {=bool:?}, trmst: {=bool:?}, recst: {=bool:?}, reserved: {=bool:?} }}",
            self.ndst(),
            self.sdst(),
            self.rfst(),
            self.tfst(),
            self.nmlst(),
            self.fmlst(),
            self.tabst(),
            self.est(),
            self.rstst(),
            self.hltst(),
            self.slpst(),
            self.epst(),
            self.bost(),
            self.trmst(),
            self.recst(),
            self.reserved()
        )
    }
}
#[doc = "Test Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u8);
impl Tcr {
    #[doc = "CAN Test Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tste(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CAN Test Mode Enable"]
    #[inline(always)]
    pub const fn set_tste(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "CAN Test Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tstm(&self) -> super::vals::Tstm {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Tstm::from_bits(val as u8)
    }
    #[doc = "CAN Test Mode Select"]
    #[inline(always)]
    pub const fn set_tstm(&mut self, val: super::vals::Tstm) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u8) & 0x03) << 1usize);
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
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for Tcr {
    #[inline(always)]
    fn default() -> Tcr {
        Tcr(0)
    }
}
impl core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr")
            .field("tste", &self.tste())
            .field("tstm", &self.tstm())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr {{ tste: {=bool:?}, tstm: {:?}, reserved: {=u8:?} }}",
            self.tste(),
            self.tstm(),
            self.reserved()
        )
    }
}
#[doc = "Transmit Error Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tecr(pub u8);
impl Tecr {
    #[doc = "Transmit error count function TECR increments or decrements the counter value according to the error status of the CAN module during transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn tecr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit error count function TECR increments or decrements the counter value according to the error status of the CAN module during transmission."]
    #[inline(always)]
    pub const fn set_tecr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Tecr {
    #[inline(always)]
    fn default() -> Tecr {
        Tecr(0)
    }
}
impl core::fmt::Debug for Tecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tecr").field("tecr", &self.tecr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tecr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tecr {{ tecr: {=u8:?} }}", self.tecr())
    }
}
#[doc = "Transmit FIFO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfcr(pub u8);
impl Tfcr {
    #[doc = "Transmit FIFO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tfe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Enable"]
    #[inline(always)]
    pub const fn set_tfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO Unsent Message Number Status"]
    #[must_use]
    #[inline(always)]
    pub const fn tfust(&self) -> super::vals::Tfust {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Tfust::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Unsent Message Number Status"]
    #[inline(always)]
    pub const fn set_tfust(&mut self, val: super::vals::Tfust) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u8) & 0x07) << 1usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Transmit FIFO Full Status"]
    #[must_use]
    #[inline(always)]
    pub const fn tffst(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Full Status"]
    #[inline(always)]
    pub const fn set_tffst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Transmit FIFO Empty Status"]
    #[must_use]
    #[inline(always)]
    pub const fn tfest(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Empty Status"]
    #[inline(always)]
    pub const fn set_tfest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Tfcr {
    #[inline(always)]
    fn default() -> Tfcr {
        Tfcr(0)
    }
}
impl core::fmt::Debug for Tfcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tfcr")
            .field("tfe", &self.tfe())
            .field("tfust", &self.tfust())
            .field("reserved", &self.reserved())
            .field("tffst", &self.tffst())
            .field("tfest", &self.tfest())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tfcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tfcr {{ tfe: {=bool:?}, tfust: {:?}, reserved: {=u8:?}, tffst: {=bool:?}, tfest: {=bool:?} }}",
            self.tfe(),
            self.tfust(),
            self.reserved(),
            self.tffst(),
            self.tfest()
        )
    }
}
#[doc = "Transmit FIFO Pointer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfpcr(pub u8);
impl Tfpcr {
    #[doc = "The CPU-side pointer for the transmit FIFO is incremented by writing FFh to TFPCR."]
    #[must_use]
    #[inline(always)]
    pub const fn tfpcr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The CPU-side pointer for the transmit FIFO is incremented by writing FFh to TFPCR."]
    #[inline(always)]
    pub const fn set_tfpcr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Tfpcr {
    #[inline(always)]
    fn default() -> Tfpcr {
        Tfpcr(0)
    }
}
impl core::fmt::Debug for Tfpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tfpcr")
            .field("tfpcr", &self.tfpcr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tfpcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tfpcr {{ tfpcr: {=u8:?} }}", self.tfpcr())
    }
}
#[doc = "Time Stamp Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsr(pub u16);
impl Tsr {
    #[doc = "Free-running counter value for the time stamp function"]
    #[must_use]
    #[inline(always)]
    pub const fn tsr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Free-running counter value for the time stamp function"]
    #[inline(always)]
    pub const fn set_tsr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tsr {
    #[inline(always)]
    fn default() -> Tsr {
        Tsr(0)
    }
}
impl core::fmt::Debug for Tsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsr").field("tsr", &self.tsr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsr {{ tsr: {=u16:?} }}", self.tsr())
    }
}
