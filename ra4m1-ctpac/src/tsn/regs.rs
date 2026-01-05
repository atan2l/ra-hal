#[doc = "Temperature Sensor Calibration Data Register H"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscdrh(pub u8);
impl Tscdrh {
    #[doc = "The calibration data stores the higher 8 bits of the converted value."]
    #[must_use]
    #[inline(always)]
    pub const fn tscdrh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The calibration data stores the higher 8 bits of the converted value."]
    #[inline(always)]
    pub const fn set_tscdrh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Tscdrh {
    #[inline(always)]
    fn default() -> Tscdrh {
        Tscdrh(0)
    }
}
impl core::fmt::Debug for Tscdrh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tscdrh")
            .field("tscdrh", &self.tscdrh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tscdrh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tscdrh {{ tscdrh: {=u8:?} }}", self.tscdrh())
    }
}
#[doc = "Temperature Sensor Calibration Data Register L"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscdrl(pub u8);
impl Tscdrl {
    #[doc = "The calibration data stores the lower 8 bits of the converted value."]
    #[must_use]
    #[inline(always)]
    pub const fn tscdrl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The calibration data stores the lower 8 bits of the converted value."]
    #[inline(always)]
    pub const fn set_tscdrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Tscdrl {
    #[inline(always)]
    fn default() -> Tscdrl {
        Tscdrl(0)
    }
}
impl core::fmt::Debug for Tscdrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tscdrl")
            .field("tscdrl", &self.tscdrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tscdrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tscdrl {{ tscdrl: {=u8:?} }}", self.tscdrl())
    }
}
