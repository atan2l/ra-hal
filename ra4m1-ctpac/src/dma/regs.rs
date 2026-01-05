#[doc = "DMAC Module Activation Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmast(pub u8);
impl Dmast {
    #[doc = "DMAC Operation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmst(&self) -> super::vals::Dmst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmst::from_bits(val as u8)
    }
    #[doc = "DMAC Operation Enable"]
    #[inline(always)]
    pub const fn set_dmst(&mut self, val: super::vals::Dmst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
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
impl Default for Dmast {
    #[inline(always)]
    fn default() -> Dmast {
        Dmast(0)
    }
}
impl core::fmt::Debug for Dmast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmast")
            .field("dmst", &self.dmst())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmast {{ dmst: {:?}, reserved: {=u8:?} }}",
            self.dmst(),
            self.reserved()
        )
    }
}
