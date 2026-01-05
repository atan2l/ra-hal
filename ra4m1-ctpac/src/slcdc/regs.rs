#[doc = "LCD Clock Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdc0(pub u8);
impl Lcdc0 {
    #[doc = "LCD clock (LCDCL)"]
    #[must_use]
    #[inline(always)]
    pub const fn lcdc(&self) -> super::vals::Lcdc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Lcdc::from_bits(val as u8)
    }
    #[doc = "LCD clock (LCDCL)"]
    #[inline(always)]
    pub const fn set_lcdc(&mut self, val: super::vals::Lcdc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u8) & 0x3f) << 0usize);
    }
}
impl Default for Lcdc0 {
    #[inline(always)]
    fn default() -> Lcdc0 {
        Lcdc0(0)
    }
}
impl core::fmt::Debug for Lcdc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lcdc0").field("lcdc", &self.lcdc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lcdc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lcdc0 {{ lcdc: {:?} }}", self.lcdc())
    }
}
#[doc = "LCD Mode Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdm0(pub u8);
impl Lcdm0 {
    #[doc = "LCD Display Bias Method Select"]
    #[must_use]
    #[inline(always)]
    pub const fn lbas(&self) -> super::vals::Lbas {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lbas::from_bits(val as u8)
    }
    #[doc = "LCD Display Bias Method Select"]
    #[inline(always)]
    pub const fn set_lbas(&mut self, val: super::vals::Lbas) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "Time Slice of LCD Display Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ldty(&self) -> super::vals::Ldty {
        let val = (self.0 >> 2usize) & 0x07;
        super::vals::Ldty::from_bits(val as u8)
    }
    #[doc = "Time Slice of LCD Display Select"]
    #[inline(always)]
    pub const fn set_ldty(&mut self, val: super::vals::Ldty) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u8) & 0x07) << 2usize);
    }
    #[doc = "LCD display waveform selection"]
    #[must_use]
    #[inline(always)]
    pub const fn lwave(&self) -> super::vals::Lwave {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Lwave::from_bits(val as u8)
    }
    #[doc = "LCD display waveform selection"]
    #[inline(always)]
    pub const fn set_lwave(&mut self, val: super::vals::Lwave) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "LCD drive voltage generator selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mdset(&self) -> super::vals::Mdset {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Mdset::from_bits(val as u8)
    }
    #[doc = "LCD drive voltage generator selection"]
    #[inline(always)]
    pub const fn set_mdset(&mut self, val: super::vals::Mdset) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u8) & 0x03) << 6usize);
    }
}
impl Default for Lcdm0 {
    #[inline(always)]
    fn default() -> Lcdm0 {
        Lcdm0(0)
    }
}
impl core::fmt::Debug for Lcdm0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lcdm0")
            .field("lbas", &self.lbas())
            .field("ldty", &self.ldty())
            .field("lwave", &self.lwave())
            .field("mdset", &self.mdset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lcdm0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lcdm0 {{ lbas: {:?}, ldty: {:?}, lwave: {:?}, mdset: {:?} }}",
            self.lbas(),
            self.ldty(),
            self.lwave(),
            self.mdset()
        )
    }
}
#[doc = "LCD Mode Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdm1(pub u8);
impl Lcdm1 {
    #[doc = "Voltage Boosting Pin Initial Value Switching Control"]
    #[must_use]
    #[inline(always)]
    pub const fn lcdvlm(&self) -> super::vals::Lcdvlm {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lcdvlm::from_bits(val as u8)
    }
    #[doc = "Voltage Boosting Pin Initial Value Switching Control"]
    #[inline(always)]
    pub const fn set_lcdvlm(&mut self, val: super::vals::Lcdvlm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u8) & 0x03) << 1usize);
    }
    #[doc = "Display data area control"]
    #[must_use]
    #[inline(always)]
    pub const fn lcdsel(&self) -> super::vals::Lcdsel {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lcdsel::from_bits(val as u8)
    }
    #[doc = "Display data area control"]
    #[inline(always)]
    pub const fn set_lcdsel(&mut self, val: super::vals::Lcdsel) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Display data area control"]
    #[must_use]
    #[inline(always)]
    pub const fn blon(&self) -> super::vals::Blon {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Blon::from_bits(val as u8)
    }
    #[doc = "Display data area control"]
    #[inline(always)]
    pub const fn set_blon(&mut self, val: super::vals::Blon) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Voltage boost circuit or capacitor split circuit operation enable/disable"]
    #[must_use]
    #[inline(always)]
    pub const fn vlcon(&self) -> super::vals::Vlcon {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Vlcon::from_bits(val as u8)
    }
    #[doc = "Voltage boost circuit or capacitor split circuit operation enable/disable"]
    #[inline(always)]
    pub const fn set_vlcon(&mut self, val: super::vals::Vlcon) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "LCD Display Enable/Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn scoc(&self) -> super::vals::Scoc {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Scoc::from_bits(val as u8)
    }
    #[doc = "LCD Display Enable/Disable"]
    #[inline(always)]
    pub const fn set_scoc(&mut self, val: super::vals::Scoc) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "LCD Display Enable/Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn lcdon(&self) -> super::vals::Lcdon {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Lcdon::from_bits(val as u8)
    }
    #[doc = "LCD Display Enable/Disable"]
    #[inline(always)]
    pub const fn set_lcdon(&mut self, val: super::vals::Lcdon) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Lcdm1 {
    #[inline(always)]
    fn default() -> Lcdm1 {
        Lcdm1(0)
    }
}
impl core::fmt::Debug for Lcdm1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lcdm1")
            .field("lcdvlm", &self.lcdvlm())
            .field("reserved", &self.reserved())
            .field("lcdsel", &self.lcdsel())
            .field("blon", &self.blon())
            .field("vlcon", &self.vlcon())
            .field("scoc", &self.scoc())
            .field("lcdon", &self.lcdon())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lcdm1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lcdm1 {{ lcdvlm: {:?}, reserved: {=u8:?}, lcdsel: {:?}, blon: {:?}, vlcon: {:?}, scoc: {:?}, lcdon: {:?} }}",
            self.lcdvlm(),
            self.reserved(),
            self.lcdsel(),
            self.blon(),
            self.vlcon(),
            self.scoc(),
            self.lcdon()
        )
    }
}
#[doc = "LCD Display Data Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seg(pub u8);
impl Seg {
    #[doc = "LCD Display Data"]
    #[must_use]
    #[inline(always)]
    pub const fn seg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "LCD Display Data"]
    #[inline(always)]
    pub const fn set_seg(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Seg {
    #[inline(always)]
    fn default() -> Seg {
        Seg(0)
    }
}
impl core::fmt::Debug for Seg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Seg").field("seg", &self.seg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Seg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Seg {{ seg: {=u8:?} }}", self.seg())
    }
}
#[doc = "LCD Boost Level Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vlcd(pub u8);
impl Vlcd {
    #[doc = "Reference Voltage(Contrast Adjustment) Select"]
    #[must_use]
    #[inline(always)]
    pub const fn vlcd(&self) -> super::vals::Vlcd {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Vlcd::from_bits(val as u8)
    }
    #[doc = "Reference Voltage(Contrast Adjustment) Select"]
    #[inline(always)]
    pub const fn set_vlcd(&mut self, val: super::vals::Vlcd) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u8) & 0x1f) << 0usize);
    }
}
impl Default for Vlcd {
    #[inline(always)]
    fn default() -> Vlcd {
        Vlcd(0)
    }
}
impl core::fmt::Debug for Vlcd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vlcd").field("vlcd", &self.vlcd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vlcd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Vlcd {{ vlcd: {:?} }}", self.vlcd())
    }
}
