#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McuVersion(pub u32);
impl McuVersion {
    #[must_use]
    #[inline(always)]
    pub const fn mcuver(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_mcuver(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for McuVersion {
    #[inline(always)]
    fn default() -> McuVersion {
        McuVersion(0)
    }
}
impl core::fmt::Debug for McuVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("McuVersion")
            .field("mcuver", &self.mcuver())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for McuVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "McuVersion {{ mcuver: {=u8:?} }}", self.mcuver())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PartNumber(pub u32);
impl PartNumber {
    #[must_use]
    #[inline(always)]
    pub const fn pn(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_pn(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PartNumber {
    #[inline(always)]
    fn default() -> PartNumber {
        PartNumber(0)
    }
}
impl core::fmt::Debug for PartNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PartNumber")
            .field("pn", &self.pn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PartNumber {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PartNumber {{ pn: {=u32:?} }}", self.pn())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uid(pub u32);
impl Uid {
    #[must_use]
    #[inline(always)]
    pub const fn uid(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_uid(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Uid {
    #[inline(always)]
    fn default() -> Uid {
        Uid(0)
    }
}
impl core::fmt::Debug for Uid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uid").field("uid", &self.uid()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uid {{ uid: {=u32:?} }}", self.uid())
    }
}
