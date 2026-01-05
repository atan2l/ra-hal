#[doc = "DOC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Docr(pub u8);
impl Docr {
    #[doc = "Operating Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn oms(&self) -> super::vals::Oms {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Oms::from_bits(val as u8)
    }
    #[doc = "Operating Mode Select"]
    #[inline(always)]
    pub const fn set_oms(&mut self, val: super::vals::Oms) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "Detection Condition Select"]
    #[must_use]
    #[inline(always)]
    pub const fn dcsel(&self) -> super::vals::Dcsel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dcsel::from_bits(val as u8)
    }
    #[doc = "Detection Condition Select"]
    #[inline(always)]
    pub const fn set_dcsel(&mut self, val: super::vals::Dcsel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
    }
    #[doc = "Data Operation Circuit Flag Indicates the result of an operation."]
    #[must_use]
    #[inline(always)]
    pub const fn dopcf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Data Operation Circuit Flag Indicates the result of an operation."]
    #[inline(always)]
    pub const fn set_dopcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "DOPCF Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn dopcfcl(&self) -> super::vals::Dopcfcl {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dopcfcl::from_bits(val as u8)
    }
    #[doc = "DOPCF Clear"]
    #[inline(always)]
    pub const fn set_dopcfcl(&mut self, val: super::vals::Dopcfcl) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Docr {
    #[inline(always)]
    fn default() -> Docr {
        Docr(0)
    }
}
impl core::fmt::Debug for Docr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Docr")
            .field("oms", &self.oms())
            .field("dcsel", &self.dcsel())
            .field("reserved", &self.reserved())
            .field("dopcf", &self.dopcf())
            .field("dopcfcl", &self.dopcfcl())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Docr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Docr {{ oms: {:?}, dcsel: {:?}, reserved: {=u8:?}, dopcf: {=bool:?}, dopcfcl: {:?}, reserved_2: {=bool:?} }}",
            self.oms(),
            self.dcsel(),
            self.reserved(),
            self.dopcf(),
            self.dopcfcl(),
            self.reserved_2()
        )
    }
}
#[doc = "DOC Data Input Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dodir(pub u16);
impl Dodir {
    #[doc = "16-bit read-write register in which 16-bit data for use in the operations are stored."]
    #[must_use]
    #[inline(always)]
    pub const fn dodir(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "16-bit read-write register in which 16-bit data for use in the operations are stored."]
    #[inline(always)]
    pub const fn set_dodir(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Dodir {
    #[inline(always)]
    fn default() -> Dodir {
        Dodir(0)
    }
}
impl core::fmt::Debug for Dodir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dodir")
            .field("dodir", &self.dodir())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dodir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dodir {{ dodir: {=u16:?} }}", self.dodir())
    }
}
#[doc = "DOC Data Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dodsr(pub u16);
impl Dodsr {
    #[doc = "This register stores 16-bit data for use as a reference in data comparison mode. This register also stores the results of operations in data addition and data subtraction modes."]
    #[must_use]
    #[inline(always)]
    pub const fn dodsr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This register stores 16-bit data for use as a reference in data comparison mode. This register also stores the results of operations in data addition and data subtraction modes."]
    #[inline(always)]
    pub const fn set_dodsr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Dodsr {
    #[inline(always)]
    fn default() -> Dodsr {
        Dodsr(0)
    }
}
impl core::fmt::Debug for Dodsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dodsr")
            .field("dodsr", &self.dodsr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dodsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dodsr {{ dodsr: {=u16:?} }}", self.dodsr())
    }
}
