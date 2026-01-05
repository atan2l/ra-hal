#[doc = "WDT Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtcr(pub u16);
impl Wdtcr {
    #[doc = "Timeout Period Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn tops(&self) -> super::vals::Tops {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Tops::from_bits(val as u8)
    }
    #[doc = "Timeout Period Selection"]
    #[inline(always)]
    pub const fn set_tops(&mut self, val: super::vals::Tops) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
    }
    #[doc = "Clock Division Ratio Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cks(&self) -> super::vals::Cks {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Cks::from_bits(val as u8)
    }
    #[doc = "Clock Division Ratio Selection"]
    #[inline(always)]
    pub const fn set_cks(&mut self, val: super::vals::Cks) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u16) & 0x0f) << 4usize);
    }
    #[doc = "Window End Position Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn rpes(&self) -> super::vals::Rpes {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Rpes::from_bits(val as u8)
    }
    #[doc = "Window End Position Selection"]
    #[inline(always)]
    pub const fn set_rpes(&mut self, val: super::vals::Rpes) {
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
    #[doc = "Window Start Position Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn rpss(&self) -> super::vals::Rpss {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Rpss::from_bits(val as u8)
    }
    #[doc = "Window Start Position Selection"]
    #[inline(always)]
    pub const fn set_rpss(&mut self, val: super::vals::Rpss) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
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
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
    }
}
impl Default for Wdtcr {
    #[inline(always)]
    fn default() -> Wdtcr {
        Wdtcr(0)
    }
}
impl core::fmt::Debug for Wdtcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdtcr")
            .field("tops", &self.tops())
            .field("reserved", &self.reserved())
            .field("cks", &self.cks())
            .field("rpes", &self.rpes())
            .field("reserved_2", &self.reserved_2())
            .field("rpss", &self.rpss())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdtcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wdtcr {{ tops: {:?}, reserved: {=u8:?}, cks: {:?}, rpes: {:?}, reserved_2: {=u8:?}, rpss: {:?}, reserved_3: {=u8:?} }}",
            self.tops(),
            self.reserved(),
            self.cks(),
            self.rpes(),
            self.reserved_2(),
            self.rpss(),
            self.reserved_3()
        )
    }
}
#[doc = "WDT Count Stop Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtcstpr(pub u8);
impl Wdtcstpr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
    }
    #[doc = "Sleep-Mode Count Stop Control"]
    #[must_use]
    #[inline(always)]
    pub const fn slcstp(&self) -> super::vals::Slcstp {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Slcstp::from_bits(val as u8)
    }
    #[doc = "Sleep-Mode Count Stop Control"]
    #[inline(always)]
    pub const fn set_slcstp(&mut self, val: super::vals::Slcstp) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Wdtcstpr {
    #[inline(always)]
    fn default() -> Wdtcstpr {
        Wdtcstpr(0)
    }
}
impl core::fmt::Debug for Wdtcstpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdtcstpr")
            .field("reserved", &self.reserved())
            .field("slcstp", &self.slcstp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdtcstpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wdtcstpr {{ reserved: {=u8:?}, slcstp: {:?} }}",
            self.reserved(),
            self.slcstp()
        )
    }
}
#[doc = "WDT Reset Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtrcr(pub u8);
impl Wdtrcr {
    #[doc = "Reset Interrupt Request Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn rstirqs(&self) -> super::vals::Rstirqs {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Rstirqs::from_bits(val as u8)
    }
    #[doc = "Reset Interrupt Request Selection"]
    #[inline(always)]
    pub const fn set_rstirqs(&mut self, val: super::vals::Rstirqs) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Wdtrcr {
    #[inline(always)]
    fn default() -> Wdtrcr {
        Wdtrcr(0)
    }
}
impl core::fmt::Debug for Wdtrcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdtrcr")
            .field("rstirqs", &self.rstirqs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdtrcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wdtrcr {{ rstirqs: {:?} }}", self.rstirqs())
    }
}
#[doc = "WDT Refresh Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtrr(pub u8);
impl Wdtrr {
    #[doc = "WDTRR is an 8-bit register that refreshes the down-counter of the WDT."]
    #[must_use]
    #[inline(always)]
    pub const fn wdtrr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "WDTRR is an 8-bit register that refreshes the down-counter of the WDT."]
    #[inline(always)]
    pub const fn set_wdtrr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Wdtrr {
    #[inline(always)]
    fn default() -> Wdtrr {
        Wdtrr(0)
    }
}
impl core::fmt::Debug for Wdtrr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdtrr")
            .field("wdtrr", &self.wdtrr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdtrr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wdtrr {{ wdtrr: {=u8:?} }}", self.wdtrr())
    }
}
#[doc = "WDT Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtsr(pub u16);
impl Wdtsr {
    #[doc = "Down-Counter Value Value counted by the down-counter"]
    #[must_use]
    #[inline(always)]
    pub const fn cntval(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Down-Counter Value Value counted by the down-counter"]
    #[inline(always)]
    pub const fn set_cntval(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u16) & 0x3fff) << 0usize);
    }
    #[doc = "Underflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn undff(&self) -> super::vals::Undff {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Undff::from_bits(val as u8)
    }
    #[doc = "Underflow Flag"]
    #[inline(always)]
    pub const fn set_undff(&mut self, val: super::vals::Undff) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Refresh Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn refef(&self) -> super::vals::Refef {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Refef::from_bits(val as u8)
    }
    #[doc = "Refresh Error Flag"]
    #[inline(always)]
    pub const fn set_refef(&mut self, val: super::vals::Refef) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Wdtsr {
    #[inline(always)]
    fn default() -> Wdtsr {
        Wdtsr(0)
    }
}
impl core::fmt::Debug for Wdtsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdtsr")
            .field("cntval", &self.cntval())
            .field("undff", &self.undff())
            .field("refef", &self.refef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdtsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wdtsr {{ cntval: {=u16:?}, undff: {:?}, refef: {:?} }}",
            self.cntval(),
            self.undff(),
            self.refef()
        )
    }
}
