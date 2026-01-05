#[doc = "Write-Protect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwpr(pub u8);
impl Pwpr {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u8) & 0x3f) << 0usize);
    }
    #[doc = "PFS Register Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pfswe(&self) -> super::vals::Pfswe {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pfswe::from_bits(val as u8)
    }
    #[doc = "PFS Register Write Enable"]
    #[inline(always)]
    pub const fn set_pfswe(&mut self, val: super::vals::Pfswe) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "PFSWE Bit Write Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn b0wi(&self) -> super::vals::B0wi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::B0wi::from_bits(val as u8)
    }
    #[doc = "PFSWE Bit Write Disable"]
    #[inline(always)]
    pub const fn set_b0wi(&mut self, val: super::vals::B0wi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Pwpr {
    #[inline(always)]
    fn default() -> Pwpr {
        Pwpr(0)
    }
}
impl core::fmt::Debug for Pwpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwpr")
            .field("reserved", &self.reserved())
            .field("pfswe", &self.pfswe())
            .field("b0wi", &self.b0wi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwpr {{ reserved: {=u8:?}, pfswe: {:?}, b0wi: {:?} }}",
            self.reserved(),
            self.pfswe(),
            self.b0wi()
        )
    }
}
