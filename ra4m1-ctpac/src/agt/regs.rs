#[doc = "AGT Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agt(pub u16);
impl Agt {
    #[doc = "16bit counter and reload register NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, the 16-bit counter is forcibly stopped and set to FFFFH."]
    #[must_use]
    #[inline(always)]
    pub const fn agt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "16bit counter and reload register NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, the 16-bit counter is forcibly stopped and set to FFFFH."]
    #[inline(always)]
    pub const fn set_agt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Agt {
    #[inline(always)]
    fn default() -> Agt {
        Agt(0)
    }
}
impl core::fmt::Debug for Agt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Agt").field("agt", &self.agt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Agt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Agt {{ agt: {=u16:?} }}", self.agt())
    }
}
#[doc = "AGT Compare Match A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtcma(pub u16);
impl Agtcma {
    #[doc = "AGT Compare Match A data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH"]
    #[must_use]
    #[inline(always)]
    pub const fn agtcma(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "AGT Compare Match A data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH"]
    #[inline(always)]
    pub const fn set_agtcma(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Agtcma {
    #[inline(always)]
    fn default() -> Agtcma {
        Agtcma(0)
    }
}
impl core::fmt::Debug for Agtcma {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Agtcma")
            .field("agtcma", &self.agtcma())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Agtcma {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Agtcma {{ agtcma: {=u16:?} }}", self.agtcma())
    }
}
#[doc = "AGT Compare Match B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtcmb(pub u16);
impl Agtcmb {
    #[doc = "AGT Compare Match B data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH"]
    #[must_use]
    #[inline(always)]
    pub const fn agtcmb(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "AGT Compare Match B data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH"]
    #[inline(always)]
    pub const fn set_agtcmb(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Agtcmb {
    #[inline(always)]
    fn default() -> Agtcmb {
        Agtcmb(0)
    }
}
impl core::fmt::Debug for Agtcmb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Agtcmb")
            .field("agtcmb", &self.agtcmb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Agtcmb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Agtcmb {{ agtcmb: {=u16:?} }}", self.agtcmb())
    }
}
#[doc = "AGT Compare Match Function Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtcmsr(pub u8);
impl Agtcmsr {
    #[doc = "Compare match A register enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcmea(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compare match A register enable"]
    #[inline(always)]
    pub const fn set_tcmea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "AGTOA output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn toea(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "AGTOA output enable"]
    #[inline(always)]
    pub const fn set_toea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "AGTOA polarity select"]
    #[must_use]
    #[inline(always)]
    pub const fn topola(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AGTOA polarity select"]
    #[inline(always)]
    pub const fn set_topola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Compare match B register enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcmeb(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Compare match B register enable"]
    #[inline(always)]
    pub const fn set_tcmeb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "AGTOB output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn toeb(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "AGTOB output enable"]
    #[inline(always)]
    pub const fn set_toeb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "AGTOB polarity select"]
    #[must_use]
    #[inline(always)]
    pub const fn topolb(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "AGTOB polarity select"]
    #[inline(always)]
    pub const fn set_topolb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
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
impl Default for Agtcmsr {
    #[inline(always)]
    fn default() -> Agtcmsr {
        Agtcmsr(0)
    }
}
impl core::fmt::Debug for Agtcmsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Agtcmsr")
            .field("tcmea", &self.tcmea())
            .field("toea", &self.toea())
            .field("topola", &self.topola())
            .field("reserved", &self.reserved())
            .field("tcmeb", &self.tcmeb())
            .field("toeb", &self.toeb())
            .field("topolb", &self.topolb())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Agtcmsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Agtcmsr {{ tcmea: {=bool:?}, toea: {=bool:?}, topola: {=bool:?}, reserved: {=bool:?}, tcmeb: {=bool:?}, toeb: {=bool:?}, topolb: {=bool:?}, reserved_2: {=bool:?} }}",
            self.tcmea(),
            self.toea(),
            self.topola(),
            self.reserved(),
            self.tcmeb(),
            self.toeb(),
            self.topolb(),
            self.reserved_2()
        )
    }
}
#[doc = "AGT Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtcr(pub u8);
impl Agtcr {
    #[doc = "AGT count start"]
    #[must_use]
    #[inline(always)]
    pub const fn tstart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AGT count start"]
    #[inline(always)]
    pub const fn set_tstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "AGT count status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcstf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "AGT count status flag"]
    #[inline(always)]
    pub const fn set_tcstf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "AGT count forced stop"]
    #[must_use]
    #[inline(always)]
    pub const fn tstop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AGT count forced stop"]
    #[inline(always)]
    pub const fn set_tstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Active edge judgment flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tedgf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Active edge judgment flag"]
    #[inline(always)]
    pub const fn set_tedgf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Underflow flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tundf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Underflow flag"]
    #[inline(always)]
    pub const fn set_tundf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Compare match A flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcmaf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Compare match A flag"]
    #[inline(always)]
    pub const fn set_tcmaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Compare match B flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcmbf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Compare match B flag"]
    #[inline(always)]
    pub const fn set_tcmbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Agtcr {
    #[inline(always)]
    fn default() -> Agtcr {
        Agtcr(0)
    }
}
impl core::fmt::Debug for Agtcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Agtcr")
            .field("tstart", &self.tstart())
            .field("tcstf", &self.tcstf())
            .field("tstop", &self.tstop())
            .field("reserved", &self.reserved())
            .field("tedgf", &self.tedgf())
            .field("tundf", &self.tundf())
            .field("tcmaf", &self.tcmaf())
            .field("tcmbf", &self.tcmbf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Agtcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Agtcr {{ tstart: {=bool:?}, tcstf: {=bool:?}, tstop: {=bool:?}, reserved: {=bool:?}, tedgf: {=bool:?}, tundf: {=bool:?}, tcmaf: {=bool:?}, tcmbf: {=bool:?} }}",
            self.tstart(),
            self.tcstf(),
            self.tstop(),
            self.reserved(),
            self.tedgf(),
            self.tundf(),
            self.tcmaf(),
            self.tcmbf()
        )
    }
}
#[doc = "AGT I/O Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtioc(pub u8);
impl Agtioc {
    #[doc = "I/O polarity switch Function varies depending on the operating mode."]
    #[must_use]
    #[inline(always)]
    pub const fn tedgsel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I/O polarity switch Function varies depending on the operating mode."]
    #[inline(always)]
    pub const fn set_tedgsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "AGTOn output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn toe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AGTOn output enable"]
    #[inline(always)]
    pub const fn set_toe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Input filter"]
    #[must_use]
    #[inline(always)]
    pub const fn tipf(&self) -> super::vals::Tipf {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Tipf::from_bits(val as u8)
    }
    #[doc = "Input filter"]
    #[inline(always)]
    pub const fn set_tipf(&mut self, val: super::vals::Tipf) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
    }
    #[doc = "Count control"]
    #[must_use]
    #[inline(always)]
    pub const fn tiogt(&self) -> super::vals::Tiogt {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Tiogt::from_bits(val as u8)
    }
    #[doc = "Count control"]
    #[inline(always)]
    pub const fn set_tiogt(&mut self, val: super::vals::Tiogt) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u8) & 0x03) << 6usize);
    }
}
impl Default for Agtioc {
    #[inline(always)]
    fn default() -> Agtioc {
        Agtioc(0)
    }
}
impl core::fmt::Debug for Agtioc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Agtioc")
            .field("tedgsel", &self.tedgsel())
            .field("reserved", &self.reserved())
            .field("toe", &self.toe())
            .field("reserved_2", &self.reserved_2())
            .field("tipf", &self.tipf())
            .field("tiogt", &self.tiogt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Agtioc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Agtioc {{ tedgsel: {=bool:?}, reserved: {=bool:?}, toe: {=bool:?}, reserved_2: {=bool:?}, tipf: {:?}, tiogt: {:?} }}",
            self.tedgsel(),
            self.reserved(),
            self.toe(),
            self.reserved_2(),
            self.tipf(),
            self.tiogt()
        )
    }
}
#[doc = "AGT Pin Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtiosel(pub u8);
impl Agtiosel {
    #[doc = "AGTIO pin select"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Sel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sel::from_bits(val as u8)
    }
    #[doc = "AGTIO pin select"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Sel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
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
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
    }
    #[doc = "AGTIO input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ties(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AGTIO input enable"]
    #[inline(always)]
    pub const fn set_ties(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Agtiosel {
    #[inline(always)]
    fn default() -> Agtiosel {
        Agtiosel(0)
    }
}
impl core::fmt::Debug for Agtiosel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Agtiosel")
            .field("sel", &self.sel())
            .field("reserved", &self.reserved())
            .field("ties", &self.ties())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Agtiosel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Agtiosel {{ sel: {:?}, reserved: {=u8:?}, ties: {=bool:?}, reserved_2: {=u8:?} }}",
            self.sel(),
            self.reserved(),
            self.ties(),
            self.reserved_2()
        )
    }
}
#[doc = "AGT Event Pin Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtisr(pub u8);
impl Agtisr {
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
    }
    #[doc = "AGTEE polarty selection"]
    #[must_use]
    #[inline(always)]
    pub const fn eeps(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AGTEE polarty selection"]
    #[inline(always)]
    pub const fn set_eeps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for Agtisr {
    #[inline(always)]
    fn default() -> Agtisr {
        Agtisr(0)
    }
}
impl core::fmt::Debug for Agtisr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Agtisr")
            .field("reserved", &self.reserved())
            .field("eeps", &self.eeps())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Agtisr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Agtisr {{ reserved: {=u8:?}, eeps: {=bool:?}, reserved_2: {=u8:?} }}",
            self.reserved(),
            self.eeps(),
            self.reserved_2()
        )
    }
}
#[doc = "AGT Mode Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtmr1(pub u8);
impl Agtmr1 {
    #[doc = "Operating mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tmod(&self) -> super::vals::Tmod {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Tmod::from_bits(val as u8)
    }
    #[doc = "Operating mode"]
    #[inline(always)]
    pub const fn set_tmod(&mut self, val: super::vals::Tmod) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u8) & 0x07) << 0usize);
    }
    #[doc = "Edge polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn tedgpl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Edge polarity"]
    #[inline(always)]
    pub const fn set_tedgpl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Count source"]
    #[must_use]
    #[inline(always)]
    pub const fn tck(&self) -> super::vals::Tck {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Tck::from_bits(val as u8)
    }
    #[doc = "Count source"]
    #[inline(always)]
    pub const fn set_tck(&mut self, val: super::vals::Tck) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u8) & 0x07) << 4usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Agtmr1 {
    #[inline(always)]
    fn default() -> Agtmr1 {
        Agtmr1(0)
    }
}
impl core::fmt::Debug for Agtmr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Agtmr1")
            .field("tmod", &self.tmod())
            .field("tedgpl", &self.tedgpl())
            .field("tck", &self.tck())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Agtmr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Agtmr1 {{ tmod: {:?}, tedgpl: {=bool:?}, tck: {:?}, reserved: {=bool:?} }}",
            self.tmod(),
            self.tedgpl(),
            self.tck(),
            self.reserved()
        )
    }
}
#[doc = "AGT Mode Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtmr2(pub u8);
impl Agtmr2 {
    #[doc = "AGTLCLK/AGTSCLK count source clock frequency division ratio"]
    #[must_use]
    #[inline(always)]
    pub const fn cks(&self) -> super::vals::Cks {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cks::from_bits(val as u8)
    }
    #[doc = "AGTLCLK/AGTSCLK count source clock frequency division ratio"]
    #[inline(always)]
    pub const fn set_cks(&mut self, val: super::vals::Cks) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u8) & 0x07) << 0usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u8) & 0x0f) << 3usize);
    }
    #[doc = "Low Power Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub const fn set_lpm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Agtmr2 {
    #[inline(always)]
    fn default() -> Agtmr2 {
        Agtmr2(0)
    }
}
impl core::fmt::Debug for Agtmr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Agtmr2")
            .field("cks", &self.cks())
            .field("reserved", &self.reserved())
            .field("lpm", &self.lpm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Agtmr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Agtmr2 {{ cks: {:?}, reserved: {=u8:?}, lpm: {=bool:?} }}",
            self.cks(),
            self.reserved(),
            self.lpm()
        )
    }
}
