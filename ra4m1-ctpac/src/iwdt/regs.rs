#[doc = "IWDT Refresh Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdtrr(pub u8);
impl Iwdtrr {
    #[doc = "The counter is refreshed by writing 0x00 and then writing 0xFF to this register."]
    #[must_use]
    #[inline(always)]
    pub const fn iwdtrr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The counter is refreshed by writing 0x00 and then writing 0xFF to this register."]
    #[inline(always)]
    pub const fn set_iwdtrr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Iwdtrr {
    #[inline(always)]
    fn default() -> Iwdtrr {
        Iwdtrr(0)
    }
}
impl core::fmt::Debug for Iwdtrr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iwdtrr")
            .field("iwdtrr", &self.iwdtrr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iwdtrr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iwdtrr {{ iwdtrr: {=u8:?} }}", self.iwdtrr())
    }
}
#[doc = "IWDT Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdtsr(pub u16);
impl Iwdtsr {
    #[doc = "Counter Value Value counted by the counter"]
    #[must_use]
    #[inline(always)]
    pub const fn cntval(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Counter Value Value counted by the counter"]
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
impl Default for Iwdtsr {
    #[inline(always)]
    fn default() -> Iwdtsr {
        Iwdtsr(0)
    }
}
impl core::fmt::Debug for Iwdtsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iwdtsr")
            .field("cntval", &self.cntval())
            .field("undff", &self.undff())
            .field("refef", &self.refef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iwdtsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iwdtsr {{ cntval: {=u16:?}, undff: {:?}, refef: {:?} }}",
            self.cntval(),
            self.undff(),
            self.refef()
        )
    }
}
