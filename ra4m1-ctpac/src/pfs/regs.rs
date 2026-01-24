#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmnPfs(pub u32);
impl PmnPfs {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> super::vals::PortLevel {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PortLevel::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::PortLevel) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::PortLevel {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PortLevel::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::PortLevel) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::PortDirection {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PortDirection::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::PortDirection) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> super::vals::OutputType {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::OutputType::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::OutputType) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Drive Capability"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr(&self) -> super::vals::PortDrive {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PortDrive::from_bits(val as u8)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: super::vals::PortDrive) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Mode Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pmr(&self) -> super::vals::PortMode {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::PortMode::from_bits(val as u8)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub const fn set_pmr(&mut self, val: super::vals::PortMode) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn psel(&self) -> super::vals::PortFunction {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::PortFunction::from_bits(val as u8)
    }
    #[inline(always)]
    pub const fn set_psel(&mut self, val: super::vals::PortFunction) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for PmnPfs {
    #[inline(always)]
    fn default() -> PmnPfs {
        PmnPfs(0)
    }
}
impl core::fmt::Debug for PmnPfs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PmnPfs")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("pcr", &self.pcr())
            .field("ncodr", &self.ncodr())
            .field("dscr", &self.dscr())
            .field("isel", &self.isel())
            .field("asel", &self.asel())
            .field("pmr", &self.pmr())
            .field("psel", &self.psel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PmnPfs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PmnPfs {{ podr: {:?}, pidr: {:?}, pdr: {:?}, pcr: {=bool:?}, ncodr: {:?}, dscr: {:?}, isel: {=bool:?}, asel: {=bool:?}, pmr: {:?}, psel: {:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.pcr(),
            self.ncodr(),
            self.dscr(),
            self.isel(),
            self.asel(),
            self.pmr(),
            self.psel()
        )
    }
}
