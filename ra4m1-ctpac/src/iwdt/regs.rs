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
    pub const fn undff(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Underflow Flag"]
    #[inline(always)]
    pub const fn set_undff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Refresh Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn refef(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Refresh Error Flag"]
    #[inline(always)]
    pub const fn set_refef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
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
            "Iwdtsr {{ cntval: {=u16:?}, undff: {=bool:?}, refef: {=bool:?} }}",
            self.cntval(),
            self.undff(),
            self.refef()
        )
    }
}
