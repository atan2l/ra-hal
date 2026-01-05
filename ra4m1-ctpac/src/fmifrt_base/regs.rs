#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Base(pub u32);
impl Base {
    #[must_use]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Base {
    #[inline(always)]
    fn default() -> Base {
        Base(0)
    }
}
impl core::fmt::Debug for Base {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Base").field("base", &self.base()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Base {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Base {{ base: {=u32:?} }}", self.base())
    }
}
