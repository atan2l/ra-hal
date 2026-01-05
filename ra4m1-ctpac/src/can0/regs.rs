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
    pub const fn cclks(&self) -> super::vals::Cclks {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cclks::from_bits(val as u8)
    }
    #[doc = "CAN Clock Source Selection"]
    #[inline(always)]
    pub const fn set_cclks(&mut self, val: super::vals::Cclks) {
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
            "Bcr {{ cclks: {:?}, reserved: {=u8:?}, tseg2: {:?}, reserved_2: {=bool:?}, sjw: {:?}, reserved_3: {=u8:?}, brp: {=u16:?}, reserved_4: {=u8:?}, tseg1: {:?} }}",
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
    pub const fn mbm(&self) -> super::vals::Mbm {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mbm::from_bits(val as u8)
    }
    #[doc = "CAN Mailbox Mode Select"]
    #[inline(always)]
    pub const fn set_mbm(&mut self, val: super::vals::Mbm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
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
    pub const fn mlm(&self) -> super::vals::Mlm {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mlm::from_bits(val as u8)
    }
    #[doc = "Message Lost Mode Select"]
    #[inline(always)]
    pub const fn set_mlm(&mut self, val: super::vals::Mlm) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Transmission Priority Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tpm(&self) -> super::vals::Tpm {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tpm::from_bits(val as u8)
    }
    #[doc = "Transmission Priority Mode Select"]
    #[inline(always)]
    pub const fn set_tpm(&mut self, val: super::vals::Tpm) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Time Stamp Counter Reset Command"]
    #[must_use]
    #[inline(always)]
    pub const fn tsrc(&self) -> super::vals::Tsrc {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Tsrc::from_bits(val as u8)
    }
    #[doc = "Time Stamp Counter Reset Command"]
    #[inline(always)]
    pub const fn set_tsrc(&mut self, val: super::vals::Tsrc) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
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
    pub const fn slpm(&self) -> super::vals::Slpm {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Slpm::from_bits(val as u8)
    }
    #[doc = "CAN Sleep Mode"]
    #[inline(always)]
    pub const fn set_slpm(&mut self, val: super::vals::Slpm) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
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
    pub const fn rboc(&self) -> super::vals::Rboc {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Rboc::from_bits(val as u8)
    }
    #[doc = "Forcible Return From Bus-Off"]
    #[inline(always)]
    pub const fn set_rboc(&mut self, val: super::vals::Rboc) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
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
            "Ctlr {{ mbm: {:?}, idfm: {:?}, mlm: {:?}, tpm: {:?}, tsrc: {:?}, tsps: {:?}, canm: {:?}, slpm: {:?}, bom: {:?}, rboc: {:?}, reserved: {=u8:?} }}",
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
    pub const fn sef(&self) -> super::vals::Sef {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sef::from_bits(val as u8)
    }
    #[doc = "Stuff Error Flag"]
    #[inline(always)]
    pub const fn set_sef(&mut self, val: super::vals::Sef) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Form Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fef(&self) -> super::vals::Fef {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Fef::from_bits(val as u8)
    }
    #[doc = "Form Error Flag"]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: super::vals::Fef) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "ACK Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn aef(&self) -> super::vals::Aef {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Aef::from_bits(val as u8)
    }
    #[doc = "ACK Error Flag"]
    #[inline(always)]
    pub const fn set_aef(&mut self, val: super::vals::Aef) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "CRC Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cef(&self) -> super::vals::Cef {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cef::from_bits(val as u8)
    }
    #[doc = "CRC Error Flag"]
    #[inline(always)]
    pub const fn set_cef(&mut self, val: super::vals::Cef) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Bit Error (recessive) Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn be1f(&self) -> super::vals::Be1f {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Be1f::from_bits(val as u8)
    }
    #[doc = "Bit Error (recessive) Flag"]
    #[inline(always)]
    pub const fn set_be1f(&mut self, val: super::vals::Be1f) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Bit Error (dominant) Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn be0f(&self) -> super::vals::Be0f {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Be0f::from_bits(val as u8)
    }
    #[doc = "Bit Error (dominant) Flag"]
    #[inline(always)]
    pub const fn set_be0f(&mut self, val: super::vals::Be0f) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "ACK Delimiter Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn adef(&self) -> super::vals::Adef {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Adef::from_bits(val as u8)
    }
    #[doc = "ACK Delimiter Error Flag"]
    #[inline(always)]
    pub const fn set_adef(&mut self, val: super::vals::Adef) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Error Display Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn edpm(&self) -> super::vals::Edpm {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Edpm::from_bits(val as u8)
    }
    #[doc = "Error Display Mode Select"]
    #[inline(always)]
    pub const fn set_edpm(&mut self, val: super::vals::Edpm) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
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
            "Ecsr {{ sef: {:?}, fef: {:?}, aef: {:?}, cef: {:?}, be1f: {:?}, be0f: {:?}, adef: {:?}, edpm: {:?} }}",
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
    pub const fn beie(&self) -> super::vals::Beie {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Beie::from_bits(val as u8)
    }
    #[doc = "Bus Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_beie(&mut self, val: super::vals::Beie) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Error-Warning Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ewie(&self) -> super::vals::Ewie {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ewie::from_bits(val as u8)
    }
    #[doc = "Error-Warning Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ewie(&mut self, val: super::vals::Ewie) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Error-Passive Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn epie(&self) -> super::vals::Epie {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Epie::from_bits(val as u8)
    }
    #[doc = "Error-Passive Interrupt Enable"]
    #[inline(always)]
    pub const fn set_epie(&mut self, val: super::vals::Epie) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Bus-Off Entry Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn boeie(&self) -> super::vals::Boeie {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Boeie::from_bits(val as u8)
    }
    #[doc = "Bus-Off Entry Interrupt Enable"]
    #[inline(always)]
    pub const fn set_boeie(&mut self, val: super::vals::Boeie) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Bus-Off Recovery Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn borie(&self) -> super::vals::Borie {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Borie::from_bits(val as u8)
    }
    #[doc = "Bus-Off Recovery Interrupt Enable"]
    #[inline(always)]
    pub const fn set_borie(&mut self, val: super::vals::Borie) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Overrun Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn orie(&self) -> super::vals::Orie {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Orie::from_bits(val as u8)
    }
    #[doc = "Overrun Interrupt Enable"]
    #[inline(always)]
    pub const fn set_orie(&mut self, val: super::vals::Orie) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Overload Frame Transmit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn olie(&self) -> super::vals::Olie {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Olie::from_bits(val as u8)
    }
    #[doc = "Overload Frame Transmit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_olie(&mut self, val: super::vals::Olie) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Bus Lock Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn blie(&self) -> super::vals::Blie {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Blie::from_bits(val as u8)
    }
    #[doc = "Bus Lock Interrupt Enable"]
    #[inline(always)]
    pub const fn set_blie(&mut self, val: super::vals::Blie) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
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
            "Eier {{ beie: {:?}, ewie: {:?}, epie: {:?}, boeie: {:?}, borie: {:?}, orie: {:?}, olie: {:?}, blie: {:?} }}",
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
    pub const fn beif(&self) -> super::vals::Beif {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Beif::from_bits(val as u8)
    }
    #[doc = "Bus Error Detect Flag"]
    #[inline(always)]
    pub const fn set_beif(&mut self, val: super::vals::Beif) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Error-Warning Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ewif(&self) -> super::vals::Ewif {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ewif::from_bits(val as u8)
    }
    #[doc = "Error-Warning Detect Flag"]
    #[inline(always)]
    pub const fn set_ewif(&mut self, val: super::vals::Ewif) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Error-Passive Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn epif(&self) -> super::vals::Epif {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Epif::from_bits(val as u8)
    }
    #[doc = "Error-Passive Detect Flag"]
    #[inline(always)]
    pub const fn set_epif(&mut self, val: super::vals::Epif) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Bus-Off Entry Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn boeif(&self) -> super::vals::Boeif {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Boeif::from_bits(val as u8)
    }
    #[doc = "Bus-Off Entry Detect Flag"]
    #[inline(always)]
    pub const fn set_boeif(&mut self, val: super::vals::Boeif) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Bus-Off Recovery Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn borif(&self) -> super::vals::Borif {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Borif::from_bits(val as u8)
    }
    #[doc = "Bus-Off Recovery Detect Flag"]
    #[inline(always)]
    pub const fn set_borif(&mut self, val: super::vals::Borif) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Receive Overrun Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn orif(&self) -> super::vals::Orif {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Orif::from_bits(val as u8)
    }
    #[doc = "Receive Overrun Detect Flag"]
    #[inline(always)]
    pub const fn set_orif(&mut self, val: super::vals::Orif) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Overload Frame Transmission Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn olif(&self) -> super::vals::Olif {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Olif::from_bits(val as u8)
    }
    #[doc = "Overload Frame Transmission Detect Flag"]
    #[inline(always)]
    pub const fn set_olif(&mut self, val: super::vals::Olif) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Bus Lock Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn blif(&self) -> super::vals::Blif {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Blif::from_bits(val as u8)
    }
    #[doc = "Bus Lock Detect Flag"]
    #[inline(always)]
    pub const fn set_blif(&mut self, val: super::vals::Blif) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
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
            "Eifr {{ beif: {:?}, ewif: {:?}, epif: {:?}, boeif: {:?}, borif: {:?}, orif: {:?}, olif: {:?}, blif: {:?} }}",
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
    pub const fn rtr(&self) -> super::vals::FidcrRtr {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FidcrRtr::from_bits(val as u8)
    }
    #[doc = "Remote Transmission Request"]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: super::vals::FidcrRtr) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "ID Extension"]
    #[must_use]
    #[inline(always)]
    pub const fn ide(&self) -> super::vals::FidcrIde {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FidcrIde::from_bits(val as u8)
    }
    #[doc = "ID Extension"]
    #[inline(always)]
    pub const fn set_ide(&mut self, val: super::vals::FidcrIde) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            "Fidcr {{ eid: {=u32:?}, sid: {=u16:?}, reserved: {=bool:?}, rtr: {:?}, ide: {:?} }}",
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
    pub const fn rtr(&self) -> super::vals::MbIdRtr {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MbIdRtr::from_bits(val as u8)
    }
    #[doc = "Remote Transmission Request"]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: super::vals::MbIdRtr) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "ID Extension"]
    #[must_use]
    #[inline(always)]
    pub const fn ide(&self) -> super::vals::MbIdIde {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MbIdIde::from_bits(val as u8)
    }
    #[doc = "ID Extension"]
    #[inline(always)]
    pub const fn set_ide(&mut self, val: super::vals::MbIdIde) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            "MbId {{ eid: {=u32:?}, sid: {=u16:?}, reserved: {=bool:?}, rtr: {:?}, ide: {:?} }}",
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
    pub const fn newdata(&self) -> super::vals::Newdata {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Newdata::from_bits(val as u8)
    }
    #[doc = "Reception Complete Flag"]
    #[inline(always)]
    pub const fn set_newdata(&mut self, val: super::vals::Newdata) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Reception-in-Progress Status Flag (Receive mailbox setting enabled)"]
    #[must_use]
    #[inline(always)]
    pub const fn invaldata(&self) -> super::vals::Invaldata {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Invaldata::from_bits(val as u8)
    }
    #[doc = "Reception-in-Progress Status Flag (Receive mailbox setting enabled)"]
    #[inline(always)]
    pub const fn set_invaldata(&mut self, val: super::vals::Invaldata) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Message Lost Flag (Receive mailbox setting enabled)"]
    #[must_use]
    #[inline(always)]
    pub const fn msglost(&self) -> super::vals::Msglost {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Msglost::from_bits(val as u8)
    }
    #[doc = "Message Lost Flag (Receive mailbox setting enabled)"]
    #[inline(always)]
    pub const fn set_msglost(&mut self, val: super::vals::Msglost) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
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
    pub const fn oneshot(&self) -> super::vals::MctlRxOneshot {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MctlRxOneshot::from_bits(val as u8)
    }
    #[doc = "One-Shot Enable"]
    #[inline(always)]
    pub const fn set_oneshot(&mut self, val: super::vals::MctlRxOneshot) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
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
    pub const fn recreq(&self) -> super::vals::MctlRxRecreq {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::MctlRxRecreq::from_bits(val as u8)
    }
    #[doc = "Receive Mailbox Request"]
    #[inline(always)]
    pub const fn set_recreq(&mut self, val: super::vals::MctlRxRecreq) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Transmit Mailbox Request"]
    #[must_use]
    #[inline(always)]
    pub const fn trmreq(&self) -> super::vals::MctlRxTrmreq {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::MctlRxTrmreq::from_bits(val as u8)
    }
    #[doc = "Transmit Mailbox Request"]
    #[inline(always)]
    pub const fn set_trmreq(&mut self, val: super::vals::MctlRxTrmreq) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
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
            "MctlRx {{ newdata: {:?}, invaldata: {:?}, msglost: {:?}, reserved: {=bool:?}, oneshot: {:?}, reserved_2: {=bool:?}, recreq: {:?}, trmreq: {:?} }}",
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
    pub const fn sentdata(&self) -> super::vals::Sentdata {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sentdata::from_bits(val as u8)
    }
    #[doc = "Transmission Complete Flag"]
    #[inline(always)]
    pub const fn set_sentdata(&mut self, val: super::vals::Sentdata) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Transmission-in-Progress Status Flag (Transmit mailbox setting enabled)"]
    #[must_use]
    #[inline(always)]
    pub const fn trmactive(&self) -> super::vals::Trmactive {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Trmactive::from_bits(val as u8)
    }
    #[doc = "Transmission-in-Progress Status Flag (Transmit mailbox setting enabled)"]
    #[inline(always)]
    pub const fn set_trmactive(&mut self, val: super::vals::Trmactive) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Transmission Abort Complete Flag (Transmit mailbox setting enabled)"]
    #[must_use]
    #[inline(always)]
    pub const fn trmabt(&self) -> super::vals::Trmabt {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Trmabt::from_bits(val as u8)
    }
    #[doc = "Transmission Abort Complete Flag (Transmit mailbox setting enabled)"]
    #[inline(always)]
    pub const fn set_trmabt(&mut self, val: super::vals::Trmabt) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
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
    pub const fn oneshot(&self) -> super::vals::MctlTxOneshot {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MctlTxOneshot::from_bits(val as u8)
    }
    #[doc = "One-Shot Enable"]
    #[inline(always)]
    pub const fn set_oneshot(&mut self, val: super::vals::MctlTxOneshot) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
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
    pub const fn recreq(&self) -> super::vals::MctlTxRecreq {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::MctlTxRecreq::from_bits(val as u8)
    }
    #[doc = "Receive Mailbox Request"]
    #[inline(always)]
    pub const fn set_recreq(&mut self, val: super::vals::MctlTxRecreq) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Transmit Mailbox Request"]
    #[must_use]
    #[inline(always)]
    pub const fn trmreq(&self) -> super::vals::MctlTxTrmreq {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::MctlTxTrmreq::from_bits(val as u8)
    }
    #[doc = "Transmit Mailbox Request"]
    #[inline(always)]
    pub const fn set_trmreq(&mut self, val: super::vals::MctlTxTrmreq) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
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
            "MctlTx {{ sentdata: {:?}, trmactive: {:?}, trmabt: {:?}, reserved: {=bool:?}, oneshot: {:?}, reserved_2: {=bool:?}, recreq: {:?}, trmreq: {:?} }}",
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
    pub const fn mb0(&self) -> super::vals::MierMb0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MierMb0::from_bits(val as u8)
    }
    #[doc = "mailbox 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb0(&mut self, val: super::vals::MierMb0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "mailbox 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb1(&self) -> super::vals::MierMb1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MierMb1::from_bits(val as u8)
    }
    #[doc = "mailbox 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb1(&mut self, val: super::vals::MierMb1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "mailbox 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb2(&self) -> super::vals::MierMb2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::MierMb2::from_bits(val as u8)
    }
    #[doc = "mailbox 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb2(&mut self, val: super::vals::MierMb2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "mailbox 3 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb3(&self) -> super::vals::MierMb3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::MierMb3::from_bits(val as u8)
    }
    #[doc = "mailbox 3 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb3(&mut self, val: super::vals::MierMb3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "mailbox 4 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb4(&self) -> super::vals::MierMb4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MierMb4::from_bits(val as u8)
    }
    #[doc = "mailbox 4 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb4(&mut self, val: super::vals::MierMb4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "mailbox 5 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb5(&self) -> super::vals::MierMb5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::MierMb5::from_bits(val as u8)
    }
    #[doc = "mailbox 5 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb5(&mut self, val: super::vals::MierMb5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "mailbox 6 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb6(&self) -> super::vals::MierMb6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::MierMb6::from_bits(val as u8)
    }
    #[doc = "mailbox 6 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb6(&mut self, val: super::vals::MierMb6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "mailbox 7 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb7(&self) -> super::vals::MierMb7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::MierMb7::from_bits(val as u8)
    }
    #[doc = "mailbox 7 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb7(&mut self, val: super::vals::MierMb7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "mailbox 8 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb8(&self) -> super::vals::MierMb8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::MierMb8::from_bits(val as u8)
    }
    #[doc = "mailbox 8 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb8(&mut self, val: super::vals::MierMb8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "mailbox 9 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb9(&self) -> super::vals::MierMb9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::MierMb9::from_bits(val as u8)
    }
    #[doc = "mailbox 9 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb9(&mut self, val: super::vals::MierMb9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "mailbox 10 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb10(&self) -> super::vals::MierMb10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::MierMb10::from_bits(val as u8)
    }
    #[doc = "mailbox 10 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb10(&mut self, val: super::vals::MierMb10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "mailbox 11 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb11(&self) -> super::vals::MierMb11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::MierMb11::from_bits(val as u8)
    }
    #[doc = "mailbox 11 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb11(&mut self, val: super::vals::MierMb11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "mailbox 12 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb12(&self) -> super::vals::MierMb12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::MierMb12::from_bits(val as u8)
    }
    #[doc = "mailbox 12 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb12(&mut self, val: super::vals::MierMb12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "mailbox 13 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb13(&self) -> super::vals::MierMb13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::MierMb13::from_bits(val as u8)
    }
    #[doc = "mailbox 13 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb13(&mut self, val: super::vals::MierMb13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "mailbox 14 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb14(&self) -> super::vals::MierMb14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::MierMb14::from_bits(val as u8)
    }
    #[doc = "mailbox 14 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb14(&mut self, val: super::vals::MierMb14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "mailbox 15 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb15(&self) -> super::vals::MierMb15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::MierMb15::from_bits(val as u8)
    }
    #[doc = "mailbox 15 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb15(&mut self, val: super::vals::MierMb15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "mailbox 16 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb16(&self) -> super::vals::MierMb16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::MierMb16::from_bits(val as u8)
    }
    #[doc = "mailbox 16 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb16(&mut self, val: super::vals::MierMb16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "mailbox 17 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb17(&self) -> super::vals::MierMb17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::MierMb17::from_bits(val as u8)
    }
    #[doc = "mailbox 17 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb17(&mut self, val: super::vals::MierMb17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "mailbox 18 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb18(&self) -> super::vals::MierMb18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::MierMb18::from_bits(val as u8)
    }
    #[doc = "mailbox 18 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb18(&mut self, val: super::vals::MierMb18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "mailbox 19 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb19(&self) -> super::vals::MierMb19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::MierMb19::from_bits(val as u8)
    }
    #[doc = "mailbox 19 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb19(&mut self, val: super::vals::MierMb19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "mailbox 20 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb20(&self) -> super::vals::MierMb20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::MierMb20::from_bits(val as u8)
    }
    #[doc = "mailbox 20 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb20(&mut self, val: super::vals::MierMb20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "mailbox 21 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb21(&self) -> super::vals::MierMb21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::MierMb21::from_bits(val as u8)
    }
    #[doc = "mailbox 21 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb21(&mut self, val: super::vals::MierMb21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "mailbox 22 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb22(&self) -> super::vals::MierMb22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::MierMb22::from_bits(val as u8)
    }
    #[doc = "mailbox 22 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb22(&mut self, val: super::vals::MierMb22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "mailbox 23 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb23(&self) -> super::vals::MierMb23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::MierMb23::from_bits(val as u8)
    }
    #[doc = "mailbox 23 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb23(&mut self, val: super::vals::MierMb23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "mailbox 24 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb24(&self) -> super::vals::MierMb24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::MierMb24::from_bits(val as u8)
    }
    #[doc = "mailbox 24 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb24(&mut self, val: super::vals::MierMb24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "mailbox 25 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb25(&self) -> super::vals::MierMb25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::MierMb25::from_bits(val as u8)
    }
    #[doc = "mailbox 25 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb25(&mut self, val: super::vals::MierMb25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "mailbox 26 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb26(&self) -> super::vals::MierMb26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::MierMb26::from_bits(val as u8)
    }
    #[doc = "mailbox 26 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb26(&mut self, val: super::vals::MierMb26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "mailbox 27 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb27(&self) -> super::vals::MierMb27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::MierMb27::from_bits(val as u8)
    }
    #[doc = "mailbox 27 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb27(&mut self, val: super::vals::MierMb27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "mailbox 28 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb28(&self) -> super::vals::MierMb28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::MierMb28::from_bits(val as u8)
    }
    #[doc = "mailbox 28 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb28(&mut self, val: super::vals::MierMb28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "mailbox 29 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb29(&self) -> super::vals::MierMb29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MierMb29::from_bits(val as u8)
    }
    #[doc = "mailbox 29 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb29(&mut self, val: super::vals::MierMb29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "mailbox 30 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb30(&self) -> super::vals::MierMb30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MierMb30::from_bits(val as u8)
    }
    #[doc = "mailbox 30 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb30(&mut self, val: super::vals::MierMb30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "mailbox 31 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb31(&self) -> super::vals::MierMb31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MierMb31::from_bits(val as u8)
    }
    #[doc = "mailbox 31 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb31(&mut self, val: super::vals::MierMb31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            "Mier {{ mb0: {:?}, mb1: {:?}, mb2: {:?}, mb3: {:?}, mb4: {:?}, mb5: {:?}, mb6: {:?}, mb7: {:?}, mb8: {:?}, mb9: {:?}, mb10: {:?}, mb11: {:?}, mb12: {:?}, mb13: {:?}, mb14: {:?}, mb15: {:?}, mb16: {:?}, mb17: {:?}, mb18: {:?}, mb19: {:?}, mb20: {:?}, mb21: {:?}, mb22: {:?}, mb23: {:?}, mb24: {:?}, mb25: {:?}, mb26: {:?}, mb27: {:?}, mb28: {:?}, mb29: {:?}, mb30: {:?}, mb31: {:?} }}",
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
    pub const fn mb0(&self) -> super::vals::MierFifoMb0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MierFifoMb0::from_bits(val as u8)
    }
    #[doc = "mailbox 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb0(&mut self, val: super::vals::MierFifoMb0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "mailbox 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb1(&self) -> super::vals::MierFifoMb1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MierFifoMb1::from_bits(val as u8)
    }
    #[doc = "mailbox 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb1(&mut self, val: super::vals::MierFifoMb1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "mailbox 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb2(&self) -> super::vals::MierFifoMb2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::MierFifoMb2::from_bits(val as u8)
    }
    #[doc = "mailbox 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb2(&mut self, val: super::vals::MierFifoMb2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "mailbox 3 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb3(&self) -> super::vals::MierFifoMb3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::MierFifoMb3::from_bits(val as u8)
    }
    #[doc = "mailbox 3 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb3(&mut self, val: super::vals::MierFifoMb3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "mailbox 4 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb4(&self) -> super::vals::MierFifoMb4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MierFifoMb4::from_bits(val as u8)
    }
    #[doc = "mailbox 4 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb4(&mut self, val: super::vals::MierFifoMb4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "mailbox 5 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb5(&self) -> super::vals::MierFifoMb5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::MierFifoMb5::from_bits(val as u8)
    }
    #[doc = "mailbox 5 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb5(&mut self, val: super::vals::MierFifoMb5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "mailbox 6 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb6(&self) -> super::vals::MierFifoMb6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::MierFifoMb6::from_bits(val as u8)
    }
    #[doc = "mailbox 6 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb6(&mut self, val: super::vals::MierFifoMb6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "mailbox 7 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb7(&self) -> super::vals::MierFifoMb7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::MierFifoMb7::from_bits(val as u8)
    }
    #[doc = "mailbox 7 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb7(&mut self, val: super::vals::MierFifoMb7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "mailbox 8 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb8(&self) -> super::vals::MierFifoMb8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::MierFifoMb8::from_bits(val as u8)
    }
    #[doc = "mailbox 8 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb8(&mut self, val: super::vals::MierFifoMb8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "mailbox 9 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb9(&self) -> super::vals::MierFifoMb9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::MierFifoMb9::from_bits(val as u8)
    }
    #[doc = "mailbox 9 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb9(&mut self, val: super::vals::MierFifoMb9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "mailbox 10 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb10(&self) -> super::vals::MierFifoMb10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::MierFifoMb10::from_bits(val as u8)
    }
    #[doc = "mailbox 10 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb10(&mut self, val: super::vals::MierFifoMb10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "mailbox 11 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb11(&self) -> super::vals::MierFifoMb11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::MierFifoMb11::from_bits(val as u8)
    }
    #[doc = "mailbox 11 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb11(&mut self, val: super::vals::MierFifoMb11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "mailbox 12 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb12(&self) -> super::vals::MierFifoMb12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::MierFifoMb12::from_bits(val as u8)
    }
    #[doc = "mailbox 12 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb12(&mut self, val: super::vals::MierFifoMb12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "mailbox 13 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb13(&self) -> super::vals::MierFifoMb13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::MierFifoMb13::from_bits(val as u8)
    }
    #[doc = "mailbox 13 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb13(&mut self, val: super::vals::MierFifoMb13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "mailbox 14 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb14(&self) -> super::vals::MierFifoMb14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::MierFifoMb14::from_bits(val as u8)
    }
    #[doc = "mailbox 14 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb14(&mut self, val: super::vals::MierFifoMb14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "mailbox 15 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb15(&self) -> super::vals::MierFifoMb15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::MierFifoMb15::from_bits(val as u8)
    }
    #[doc = "mailbox 15 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb15(&mut self, val: super::vals::MierFifoMb15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "mailbox 16 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb16(&self) -> super::vals::MierFifoMb16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::MierFifoMb16::from_bits(val as u8)
    }
    #[doc = "mailbox 16 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb16(&mut self, val: super::vals::MierFifoMb16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "mailbox 17 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb17(&self) -> super::vals::MierFifoMb17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::MierFifoMb17::from_bits(val as u8)
    }
    #[doc = "mailbox 17 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb17(&mut self, val: super::vals::MierFifoMb17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "mailbox 18 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb18(&self) -> super::vals::MierFifoMb18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::MierFifoMb18::from_bits(val as u8)
    }
    #[doc = "mailbox 18 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb18(&mut self, val: super::vals::MierFifoMb18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "mailbox 19 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb19(&self) -> super::vals::MierFifoMb19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::MierFifoMb19::from_bits(val as u8)
    }
    #[doc = "mailbox 19 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb19(&mut self, val: super::vals::MierFifoMb19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "mailbox 20 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb20(&self) -> super::vals::MierFifoMb20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::MierFifoMb20::from_bits(val as u8)
    }
    #[doc = "mailbox 20 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb20(&mut self, val: super::vals::MierFifoMb20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "mailbox 21 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb21(&self) -> super::vals::MierFifoMb21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::MierFifoMb21::from_bits(val as u8)
    }
    #[doc = "mailbox 21 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb21(&mut self, val: super::vals::MierFifoMb21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "mailbox 22 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb22(&self) -> super::vals::MierFifoMb22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::MierFifoMb22::from_bits(val as u8)
    }
    #[doc = "mailbox 22 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb22(&mut self, val: super::vals::MierFifoMb22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "mailbox 23 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb23(&self) -> super::vals::MierFifoMb23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::MierFifoMb23::from_bits(val as u8)
    }
    #[doc = "mailbox 23 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb23(&mut self, val: super::vals::MierFifoMb23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Transmit FIFO Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mb24(&self) -> super::vals::MierFifoMb24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::MierFifoMb24::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb24(&mut self, val: super::vals::MierFifoMb24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Transmit FIFO Interrupt Generation Timing Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mb25(&self) -> super::vals::MierFifoMb25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::MierFifoMb25::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Interrupt Generation Timing Control"]
    #[inline(always)]
    pub const fn set_mb25(&mut self, val: super::vals::MierFifoMb25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
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
    pub const fn mb28(&self) -> super::vals::MierFifoMb28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::MierFifoMb28::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mb28(&mut self, val: super::vals::MierFifoMb28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Receive FIFO Interrupt Generation Timing Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mb29(&self) -> super::vals::MierFifoMb29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MierFifoMb29::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Interrupt Generation Timing Control"]
    #[inline(always)]
    pub const fn set_mb29(&mut self, val: super::vals::MierFifoMb29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
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
            "MierFifo {{ mb0: {:?}, mb1: {:?}, mb2: {:?}, mb3: {:?}, mb4: {:?}, mb5: {:?}, mb6: {:?}, mb7: {:?}, mb8: {:?}, mb9: {:?}, mb10: {:?}, mb11: {:?}, mb12: {:?}, mb13: {:?}, mb14: {:?}, mb15: {:?}, mb16: {:?}, mb17: {:?}, mb18: {:?}, mb19: {:?}, mb20: {:?}, mb21: {:?}, mb22: {:?}, mb23: {:?}, mb24: {:?}, mb25: {:?}, reserved: {=u8:?}, mb28: {:?}, mb29: {:?} }}",
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
    pub const fn mb0(&self) -> super::vals::MkivlrMb0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MkivlrMb0::from_bits(val as u8)
    }
    #[doc = "mailbox 0 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb0(&mut self, val: super::vals::MkivlrMb0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "mailbox 1 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb1(&self) -> super::vals::MkivlrMb1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MkivlrMb1::from_bits(val as u8)
    }
    #[doc = "mailbox 1 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb1(&mut self, val: super::vals::MkivlrMb1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "mailbox 2 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb2(&self) -> super::vals::MkivlrMb2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::MkivlrMb2::from_bits(val as u8)
    }
    #[doc = "mailbox 2 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb2(&mut self, val: super::vals::MkivlrMb2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "mailbox 3 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb3(&self) -> super::vals::MkivlrMb3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::MkivlrMb3::from_bits(val as u8)
    }
    #[doc = "mailbox 3 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb3(&mut self, val: super::vals::MkivlrMb3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "mailbox 4 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb4(&self) -> super::vals::MkivlrMb4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MkivlrMb4::from_bits(val as u8)
    }
    #[doc = "mailbox 4 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb4(&mut self, val: super::vals::MkivlrMb4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "mailbox 5 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb5(&self) -> super::vals::MkivlrMb5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::MkivlrMb5::from_bits(val as u8)
    }
    #[doc = "mailbox 5 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb5(&mut self, val: super::vals::MkivlrMb5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "mailbox 6 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb6(&self) -> super::vals::MkivlrMb6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::MkivlrMb6::from_bits(val as u8)
    }
    #[doc = "mailbox 6 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb6(&mut self, val: super::vals::MkivlrMb6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "mailbox 7 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb7(&self) -> super::vals::MkivlrMb7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::MkivlrMb7::from_bits(val as u8)
    }
    #[doc = "mailbox 7 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb7(&mut self, val: super::vals::MkivlrMb7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "mailbox 8 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb8(&self) -> super::vals::MkivlrMb8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::MkivlrMb8::from_bits(val as u8)
    }
    #[doc = "mailbox 8 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb8(&mut self, val: super::vals::MkivlrMb8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "mailbox 9 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb9(&self) -> super::vals::MkivlrMb9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::MkivlrMb9::from_bits(val as u8)
    }
    #[doc = "mailbox 9 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb9(&mut self, val: super::vals::MkivlrMb9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "mailbox 10 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb10(&self) -> super::vals::MkivlrMb10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::MkivlrMb10::from_bits(val as u8)
    }
    #[doc = "mailbox 10 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb10(&mut self, val: super::vals::MkivlrMb10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "mailbox 11 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb11(&self) -> super::vals::MkivlrMb11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::MkivlrMb11::from_bits(val as u8)
    }
    #[doc = "mailbox 11 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb11(&mut self, val: super::vals::MkivlrMb11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "mailbox 12 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb12(&self) -> super::vals::MkivlrMb12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::MkivlrMb12::from_bits(val as u8)
    }
    #[doc = "mailbox 12 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb12(&mut self, val: super::vals::MkivlrMb12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "mailbox 13 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb13(&self) -> super::vals::MkivlrMb13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::MkivlrMb13::from_bits(val as u8)
    }
    #[doc = "mailbox 13 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb13(&mut self, val: super::vals::MkivlrMb13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "mailbox 14 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb14(&self) -> super::vals::MkivlrMb14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::MkivlrMb14::from_bits(val as u8)
    }
    #[doc = "mailbox 14 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb14(&mut self, val: super::vals::MkivlrMb14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "mailbox 15 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb15(&self) -> super::vals::MkivlrMb15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::MkivlrMb15::from_bits(val as u8)
    }
    #[doc = "mailbox 15 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb15(&mut self, val: super::vals::MkivlrMb15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "mailbox 16 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb16(&self) -> super::vals::MkivlrMb16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::MkivlrMb16::from_bits(val as u8)
    }
    #[doc = "mailbox 16 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb16(&mut self, val: super::vals::MkivlrMb16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "mailbox 17 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb17(&self) -> super::vals::MkivlrMb17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::MkivlrMb17::from_bits(val as u8)
    }
    #[doc = "mailbox 17 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb17(&mut self, val: super::vals::MkivlrMb17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "mailbox 18 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb18(&self) -> super::vals::MkivlrMb18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::MkivlrMb18::from_bits(val as u8)
    }
    #[doc = "mailbox 18 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb18(&mut self, val: super::vals::MkivlrMb18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "mailbox 19 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb19(&self) -> super::vals::MkivlrMb19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::MkivlrMb19::from_bits(val as u8)
    }
    #[doc = "mailbox 19 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb19(&mut self, val: super::vals::MkivlrMb19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "mailbox 20 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb20(&self) -> super::vals::MkivlrMb20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::MkivlrMb20::from_bits(val as u8)
    }
    #[doc = "mailbox 20 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb20(&mut self, val: super::vals::MkivlrMb20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "mailbox 21 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb21(&self) -> super::vals::MkivlrMb21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::MkivlrMb21::from_bits(val as u8)
    }
    #[doc = "mailbox 21 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb21(&mut self, val: super::vals::MkivlrMb21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "mailbox 22 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb22(&self) -> super::vals::MkivlrMb22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::MkivlrMb22::from_bits(val as u8)
    }
    #[doc = "mailbox 22 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb22(&mut self, val: super::vals::MkivlrMb22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "mailbox 23 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb23(&self) -> super::vals::MkivlrMb23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::MkivlrMb23::from_bits(val as u8)
    }
    #[doc = "mailbox 23 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb23(&mut self, val: super::vals::MkivlrMb23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "mailbox 24 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb24(&self) -> super::vals::MkivlrMb24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::MkivlrMb24::from_bits(val as u8)
    }
    #[doc = "mailbox 24 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb24(&mut self, val: super::vals::MkivlrMb24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "mailbox 25 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb25(&self) -> super::vals::MkivlrMb25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::MkivlrMb25::from_bits(val as u8)
    }
    #[doc = "mailbox 25 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb25(&mut self, val: super::vals::MkivlrMb25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "mailbox 26 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb26(&self) -> super::vals::MkivlrMb26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::MkivlrMb26::from_bits(val as u8)
    }
    #[doc = "mailbox 26 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb26(&mut self, val: super::vals::MkivlrMb26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "mailbox 27 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb27(&self) -> super::vals::MkivlrMb27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::MkivlrMb27::from_bits(val as u8)
    }
    #[doc = "mailbox 27 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb27(&mut self, val: super::vals::MkivlrMb27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "mailbox 28 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb28(&self) -> super::vals::MkivlrMb28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::MkivlrMb28::from_bits(val as u8)
    }
    #[doc = "mailbox 28 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb28(&mut self, val: super::vals::MkivlrMb28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "mailbox 29 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb29(&self) -> super::vals::MkivlrMb29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MkivlrMb29::from_bits(val as u8)
    }
    #[doc = "mailbox 29 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb29(&mut self, val: super::vals::MkivlrMb29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "mailbox 30 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb30(&self) -> super::vals::MkivlrMb30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MkivlrMb30::from_bits(val as u8)
    }
    #[doc = "mailbox 30 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb30(&mut self, val: super::vals::MkivlrMb30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "mailbox 31 Mask Invalid"]
    #[must_use]
    #[inline(always)]
    pub const fn mb31(&self) -> super::vals::MkivlrMb31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MkivlrMb31::from_bits(val as u8)
    }
    #[doc = "mailbox 31 Mask Invalid"]
    #[inline(always)]
    pub const fn set_mb31(&mut self, val: super::vals::MkivlrMb31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            "Mkivlr {{ mb0: {:?}, mb1: {:?}, mb2: {:?}, mb3: {:?}, mb4: {:?}, mb5: {:?}, mb6: {:?}, mb7: {:?}, mb8: {:?}, mb9: {:?}, mb10: {:?}, mb11: {:?}, mb12: {:?}, mb13: {:?}, mb14: {:?}, mb15: {:?}, mb16: {:?}, mb17: {:?}, mb18: {:?}, mb19: {:?}, mb20: {:?}, mb21: {:?}, mb22: {:?}, mb23: {:?}, mb24: {:?}, mb25: {:?}, mb26: {:?}, mb27: {:?}, mb28: {:?}, mb29: {:?}, mb30: {:?}, mb31: {:?} }}",
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
    pub const fn sest(&self) -> super::vals::Sest {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sest::from_bits(val as u8)
    }
    #[doc = "Search Result Status"]
    #[inline(always)]
    pub const fn set_sest(&mut self, val: super::vals::Sest) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
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
            "Mssr {{ mbnst: {=u8:?}, reserved: {=u8:?}, sest: {:?} }}",
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
    pub const fn rfe(&self) -> super::vals::Rfe {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rfe::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Enable"]
    #[inline(always)]
    pub const fn set_rfe(&mut self, val: super::vals::Rfe) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
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
    pub const fn rfmlf(&self) -> super::vals::Rfmlf {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Rfmlf::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Message Lost Flag"]
    #[inline(always)]
    pub const fn set_rfmlf(&mut self, val: super::vals::Rfmlf) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Receive FIFO Full Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rffst(&self) -> super::vals::Rffst {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Rffst::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Full Status Flag"]
    #[inline(always)]
    pub const fn set_rffst(&mut self, val: super::vals::Rffst) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Receive FIFO Buffer Warning Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rfwst(&self) -> super::vals::Rfwst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Rfwst::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Buffer Warning Status Flag"]
    #[inline(always)]
    pub const fn set_rfwst(&mut self, val: super::vals::Rfwst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Receive FIFO Empty Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rfest(&self) -> super::vals::Rfest {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Rfest::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Empty Status Flag"]
    #[inline(always)]
    pub const fn set_rfest(&mut self, val: super::vals::Rfest) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
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
            "Rfcr {{ rfe: {:?}, rfust: {:?}, rfmlf: {:?}, rffst: {:?}, rfwst: {:?}, rfest: {:?} }}",
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
    pub const fn ndst(&self) -> super::vals::Ndst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ndst::from_bits(val as u8)
    }
    #[doc = "NEWDATA Status Flag"]
    #[inline(always)]
    pub const fn set_ndst(&mut self, val: super::vals::Ndst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "SENTDATA Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sdst(&self) -> super::vals::Sdst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sdst::from_bits(val as u8)
    }
    #[doc = "SENTDATA Status Flag"]
    #[inline(always)]
    pub const fn set_sdst(&mut self, val: super::vals::Sdst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rfst(&self) -> super::vals::Rfst {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Rfst::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Status Flag"]
    #[inline(always)]
    pub const fn set_rfst(&mut self, val: super::vals::Rfst) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Transmit FIFO Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tfst(&self) -> super::vals::Tfst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tfst::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Status Flag"]
    #[inline(always)]
    pub const fn set_tfst(&mut self, val: super::vals::Tfst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Normal Mailbox Message Lost Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn nmlst(&self) -> super::vals::Nmlst {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Nmlst::from_bits(val as u8)
    }
    #[doc = "Normal Mailbox Message Lost Status Flag"]
    #[inline(always)]
    pub const fn set_nmlst(&mut self, val: super::vals::Nmlst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "FIFO Mailbox Message Lost Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fmlst(&self) -> super::vals::Fmlst {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Fmlst::from_bits(val as u8)
    }
    #[doc = "FIFO Mailbox Message Lost Status Flag"]
    #[inline(always)]
    pub const fn set_fmlst(&mut self, val: super::vals::Fmlst) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Transmission Abort Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tabst(&self) -> super::vals::Tabst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Tabst::from_bits(val as u8)
    }
    #[doc = "Transmission Abort Status Flag"]
    #[inline(always)]
    pub const fn set_tabst(&mut self, val: super::vals::Tabst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Error Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn est(&self) -> super::vals::Est {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Est::from_bits(val as u8)
    }
    #[doc = "Error Status Flag"]
    #[inline(always)]
    pub const fn set_est(&mut self, val: super::vals::Est) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "CAN Reset Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rstst(&self) -> super::vals::Rstst {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Rstst::from_bits(val as u8)
    }
    #[doc = "CAN Reset Status Flag"]
    #[inline(always)]
    pub const fn set_rstst(&mut self, val: super::vals::Rstst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "CAN Halt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn hltst(&self) -> super::vals::Hltst {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Hltst::from_bits(val as u8)
    }
    #[doc = "CAN Halt Status Flag"]
    #[inline(always)]
    pub const fn set_hltst(&mut self, val: super::vals::Hltst) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "CAN Sleep Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn slpst(&self) -> super::vals::Slpst {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Slpst::from_bits(val as u8)
    }
    #[doc = "CAN Sleep Status Flag"]
    #[inline(always)]
    pub const fn set_slpst(&mut self, val: super::vals::Slpst) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Error-Passive Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn epst(&self) -> super::vals::Epst {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Epst::from_bits(val as u8)
    }
    #[doc = "Error-Passive Status Flag"]
    #[inline(always)]
    pub const fn set_epst(&mut self, val: super::vals::Epst) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Bus-Off Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bost(&self) -> super::vals::Bost {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Bost::from_bits(val as u8)
    }
    #[doc = "Bus-Off Status Flag"]
    #[inline(always)]
    pub const fn set_bost(&mut self, val: super::vals::Bost) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Transmit Status Flag (transmitter)"]
    #[must_use]
    #[inline(always)]
    pub const fn trmst(&self) -> super::vals::Trmst {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Trmst::from_bits(val as u8)
    }
    #[doc = "Transmit Status Flag (transmitter)"]
    #[inline(always)]
    pub const fn set_trmst(&mut self, val: super::vals::Trmst) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Receive Status Flag (receiver)"]
    #[must_use]
    #[inline(always)]
    pub const fn recst(&self) -> super::vals::Recst {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Recst::from_bits(val as u8)
    }
    #[doc = "Receive Status Flag (receiver)"]
    #[inline(always)]
    pub const fn set_recst(&mut self, val: super::vals::Recst) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
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
            "Str {{ ndst: {:?}, sdst: {:?}, rfst: {:?}, tfst: {:?}, nmlst: {:?}, fmlst: {:?}, tabst: {:?}, est: {:?}, rstst: {:?}, hltst: {:?}, slpst: {:?}, epst: {:?}, bost: {:?}, trmst: {:?}, recst: {:?}, reserved: {=bool:?} }}",
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
    pub const fn tste(&self) -> super::vals::Tste {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tste::from_bits(val as u8)
    }
    #[doc = "CAN Test Mode Enable"]
    #[inline(always)]
    pub const fn set_tste(&mut self, val: super::vals::Tste) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
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
            "Tcr {{ tste: {:?}, tstm: {:?}, reserved: {=u8:?} }}",
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
    pub const fn tfe(&self) -> super::vals::Tfe {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tfe::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Enable"]
    #[inline(always)]
    pub const fn set_tfe(&mut self, val: super::vals::Tfe) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
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
    pub const fn tffst(&self) -> super::vals::Tffst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Tffst::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Full Status"]
    #[inline(always)]
    pub const fn set_tffst(&mut self, val: super::vals::Tffst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Transmit FIFO Empty Status"]
    #[must_use]
    #[inline(always)]
    pub const fn tfest(&self) -> super::vals::Tfest {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Tfest::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Empty Status"]
    #[inline(always)]
    pub const fn set_tfest(&mut self, val: super::vals::Tfest) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
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
            "Tfcr {{ tfe: {:?}, tfust: {:?}, reserved: {=u8:?}, tffst: {:?}, tfest: {:?} }}",
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
