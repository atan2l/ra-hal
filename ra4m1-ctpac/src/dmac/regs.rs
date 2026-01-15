#[doc = "DMA Address Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmamd(pub u16);
impl Dmamd {
    #[doc = "Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings."]
    #[must_use]
    #[inline(always)]
    pub const fn dara(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings."]
    #[inline(always)]
    pub const fn set_dara(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Destination Address Update Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dm(&self) -> super::vals::Dm {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Dm::from_bits(val as u8)
    }
    #[doc = "Destination Address Update Mode"]
    #[inline(always)]
    pub const fn set_dm(&mut self, val: super::vals::Dm) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings."]
    #[must_use]
    #[inline(always)]
    pub const fn sara(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings."]
    #[inline(always)]
    pub const fn set_sara(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Source Address Update Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn sm(&self) -> super::vals::Sm {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Sm::from_bits(val as u8)
    }
    #[doc = "Source Address Update Mode"]
    #[inline(always)]
    pub const fn set_sm(&mut self, val: super::vals::Sm) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Dmamd {
    #[inline(always)]
    fn default() -> Dmamd {
        Dmamd(0)
    }
}
impl core::fmt::Debug for Dmamd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmamd")
            .field("dara", &self.dara())
            .field("reserved", &self.reserved())
            .field("dm", &self.dm())
            .field("sara", &self.sara())
            .field("reserved_2", &self.reserved_2())
            .field("sm", &self.sm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmamd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmamd {{ dara: {=u8:?}, reserved: {=bool:?}, dm: {:?}, sara: {=u8:?}, reserved_2: {=bool:?}, sm: {:?} }}",
            self.dara(),
            self.reserved(),
            self.dm(),
            self.sara(),
            self.reserved_2(),
            self.sm()
        )
    }
}
#[doc = "DMA Transfer Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmcnt(pub u8);
impl Dmcnt {
    #[doc = "DMA Transfer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transfer Enable"]
    #[inline(always)]
    pub const fn set_dte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
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
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Dmcnt {
    #[inline(always)]
    fn default() -> Dmcnt {
        Dmcnt(0)
    }
}
impl core::fmt::Debug for Dmcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmcnt")
            .field("dte", &self.dte())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmcnt {{ dte: {=bool:?}, reserved: {=u8:?} }}",
            self.dte(),
            self.reserved()
        )
    }
}
#[doc = "DMA Transfer Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmcra(pub u32);
impl Dmcra {
    #[doc = "Lower bits of transfer count"]
    #[must_use]
    #[inline(always)]
    pub const fn dmcral(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Lower bits of transfer count"]
    #[inline(always)]
    pub const fn set_dmcral(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Upper bits of transfer count"]
    #[must_use]
    #[inline(always)]
    pub const fn dmcrah(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Upper bits of transfer count"]
    #[inline(always)]
    pub const fn set_dmcrah(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for Dmcra {
    #[inline(always)]
    fn default() -> Dmcra {
        Dmcra(0)
    }
}
impl core::fmt::Debug for Dmcra {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmcra")
            .field("dmcral", &self.dmcral())
            .field("dmcrah", &self.dmcrah())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmcra {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmcra {{ dmcral: {=u16:?}, dmcrah: {=u16:?}, reserved: {=u8:?} }}",
            self.dmcral(),
            self.dmcrah(),
            self.reserved()
        )
    }
}
#[doc = "DMA Block Transfer Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmcrb(pub u16);
impl Dmcrb {
    #[doc = "Specifies the number of block transfer operations or repeat transfer operations."]
    #[must_use]
    #[inline(always)]
    pub const fn dmcrb(&self) -> super::vals::Dmcrb {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Dmcrb::from_bits(val as u16)
    }
    #[doc = "Specifies the number of block transfer operations or repeat transfer operations."]
    #[inline(always)]
    pub const fn set_dmcrb(&mut self, val: super::vals::Dmcrb) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u16) & 0xffff) << 0usize);
    }
}
impl Default for Dmcrb {
    #[inline(always)]
    fn default() -> Dmcrb {
        Dmcrb(0)
    }
}
impl core::fmt::Debug for Dmcrb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmcrb")
            .field("dmcrb", &self.dmcrb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmcrb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmcrb {{ dmcrb: {:?} }}", self.dmcrb())
    }
}
#[doc = "DMA Destination Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmdar(pub u32);
impl Dmdar {
    #[doc = "Specifies the transfer destination start address."]
    #[must_use]
    #[inline(always)]
    pub const fn dmdar(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Specifies the transfer destination start address."]
    #[inline(always)]
    pub const fn set_dmdar(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dmdar {
    #[inline(always)]
    fn default() -> Dmdar {
        Dmdar(0)
    }
}
impl core::fmt::Debug for Dmdar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmdar")
            .field("dmdar", &self.dmdar())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmdar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmdar {{ dmdar: {=u32:?} }}", self.dmdar())
    }
}
#[doc = "DMA Interrupt Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmint(pub u8);
impl Dmint {
    #[doc = "Destination Address Extended Repeat Area Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn darie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Extended Repeat Area Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_darie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Source Address Extended Repeat Area Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sarie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Extended Repeat Area Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sarie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Repeat Size End Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rptie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Repeat Size End Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rptie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Transfer Escape End Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn esie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Escape End Interrupt Enable"]
    #[inline(always)]
    pub const fn set_esie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Transfer End Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer End Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dtie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Dmint {
    #[inline(always)]
    fn default() -> Dmint {
        Dmint(0)
    }
}
impl core::fmt::Debug for Dmint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmint")
            .field("darie", &self.darie())
            .field("sarie", &self.sarie())
            .field("rptie", &self.rptie())
            .field("esie", &self.esie())
            .field("dtie", &self.dtie())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmint {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmint {{ darie: {=bool:?}, sarie: {=bool:?}, rptie: {=bool:?}, esie: {=bool:?}, dtie: {=bool:?}, reserved: {=u8:?} }}",
            self.darie(),
            self.sarie(),
            self.rptie(),
            self.esie(),
            self.dtie(),
            self.reserved()
        )
    }
}
#[doc = "DMA Offset Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmofr(pub u32);
impl Dmofr {
    #[doc = "Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
    #[must_use]
    #[inline(always)]
    pub const fn dmofr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
    #[inline(always)]
    pub const fn set_dmofr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dmofr {
    #[inline(always)]
    fn default() -> Dmofr {
        Dmofr(0)
    }
}
impl core::fmt::Debug for Dmofr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmofr")
            .field("dmofr", &self.dmofr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmofr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmofr {{ dmofr: {=u32:?} }}", self.dmofr())
    }
}
#[doc = "DMA Software Start Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmreq(pub u8);
impl Dmreq {
    #[doc = "DMA Software Start"]
    #[must_use]
    #[inline(always)]
    pub const fn swreq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Software Start"]
    #[inline(always)]
    pub const fn set_swreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u8) & 0x07) << 1usize);
    }
    #[doc = "DMA Software Start Bit Auto Clear Select"]
    #[must_use]
    #[inline(always)]
    pub const fn clrs(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Software Start Bit Auto Clear Select"]
    #[inline(always)]
    pub const fn set_clrs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Dmreq {
    #[inline(always)]
    fn default() -> Dmreq {
        Dmreq(0)
    }
}
impl core::fmt::Debug for Dmreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmreq")
            .field("swreq", &self.swreq())
            .field("reserved", &self.reserved())
            .field("clrs", &self.clrs())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmreq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmreq {{ swreq: {=bool:?}, reserved: {=u8:?}, clrs: {=bool:?}, reserved_2: {=u8:?} }}",
            self.swreq(),
            self.reserved(),
            self.clrs(),
            self.reserved_2()
        )
    }
}
#[doc = "DMA Source Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmsar(pub u32);
impl Dmsar {
    #[doc = "Specifies the transfer source start address."]
    #[must_use]
    #[inline(always)]
    pub const fn dmsar(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Specifies the transfer source start address."]
    #[inline(always)]
    pub const fn set_dmsar(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dmsar {
    #[inline(always)]
    fn default() -> Dmsar {
        Dmsar(0)
    }
}
impl core::fmt::Debug for Dmsar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmsar")
            .field("dmsar", &self.dmsar())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmsar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmsar {{ dmsar: {=u32:?} }}", self.dmsar())
    }
}
#[doc = "DMA Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmsts(pub u8);
impl Dmsts {
    #[doc = "Transfer Escape End Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn esif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Escape End Interrupt Flag"]
    #[inline(always)]
    pub const fn set_esif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u8) & 0x07) << 1usize);
    }
    #[doc = "Transfer End Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn dtif(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer End Interrupt Flag"]
    #[inline(always)]
    pub const fn set_dtif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u8) & 0x03) << 5usize);
    }
    #[doc = "DMA Active Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn act(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Active Flag"]
    #[inline(always)]
    pub const fn set_act(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dmsts {
    #[inline(always)]
    fn default() -> Dmsts {
        Dmsts(0)
    }
}
impl core::fmt::Debug for Dmsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmsts")
            .field("esif", &self.esif())
            .field("reserved", &self.reserved())
            .field("dtif", &self.dtif())
            .field("reserved_2", &self.reserved_2())
            .field("act", &self.act())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmsts {{ esif: {=bool:?}, reserved: {=u8:?}, dtif: {=bool:?}, reserved_2: {=u8:?}, act: {=bool:?} }}",
            self.esif(),
            self.reserved(),
            self.dtif(),
            self.reserved_2(),
            self.act()
        )
    }
}
#[doc = "DMA Transfer Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmtmd(pub u16);
impl Dmtmd {
    #[doc = "Transfer Request Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn dctg(&self) -> super::vals::Dctg {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Dctg::from_bits(val as u8)
    }
    #[doc = "Transfer Request Source Select"]
    #[inline(always)]
    pub const fn set_dctg(&mut self, val: super::vals::Dctg) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
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
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u16) & 0x3f) << 2usize);
    }
    #[doc = "Transfer Data Size Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sz(&self) -> super::vals::Sz {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sz::from_bits(val as u8)
    }
    #[doc = "Transfer Data Size Select"]
    #[inline(always)]
    pub const fn set_sz(&mut self, val: super::vals::Sz) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u16) & 0x03) << 10usize);
    }
    #[doc = "Repeat Area Select"]
    #[must_use]
    #[inline(always)]
    pub const fn dts(&self) -> super::vals::Dts {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Dts::from_bits(val as u8)
    }
    #[doc = "Repeat Area Select"]
    #[inline(always)]
    pub const fn set_dts(&mut self, val: super::vals::Dts) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Transfer Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn md(&self) -> super::vals::Md {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Md::from_bits(val as u8)
    }
    #[doc = "Transfer Mode Select"]
    #[inline(always)]
    pub const fn set_md(&mut self, val: super::vals::Md) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Dmtmd {
    #[inline(always)]
    fn default() -> Dmtmd {
        Dmtmd(0)
    }
}
impl core::fmt::Debug for Dmtmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmtmd")
            .field("dctg", &self.dctg())
            .field("reserved", &self.reserved())
            .field("sz", &self.sz())
            .field("reserved_2", &self.reserved_2())
            .field("dts", &self.dts())
            .field("md", &self.md())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmtmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmtmd {{ dctg: {:?}, reserved: {=u8:?}, sz: {:?}, reserved_2: {=u8:?}, dts: {:?}, md: {:?} }}",
            self.dctg(),
            self.reserved(),
            self.sz(),
            self.reserved_2(),
            self.dts(),
            self.md()
        )
    }
}
