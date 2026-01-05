#[doc = "POEG Group %s Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Poegg(pub u32);
impl Poegg {
    #[doc = "Port Input Detection Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pidf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Detection Flag"]
    #[inline(always)]
    pub const fn set_pidf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Output-disable Request Detection Flag from GPT"]
    #[must_use]
    #[inline(always)]
    pub const fn iocf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Output-disable Request Detection Flag from GPT"]
    #[inline(always)]
    pub const fn set_iocf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Oscillation Stop Detection Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ostpf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub const fn set_ostpf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Software Stop Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ssf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Software Stop Flag"]
    #[inline(always)]
    pub const fn set_ssf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Input Detection Enable Note: Can be modified only once after a reset."]
    #[must_use]
    #[inline(always)]
    pub const fn pide(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Detection Enable Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub const fn set_pide(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Output-disable Request Enable from GPT Note: Can be modified only once after a reset."]
    #[must_use]
    #[inline(always)]
    pub const fn ioce(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Output-disable Request Enable from GPT Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub const fn set_ioce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Oscillation Stop Detection Enable Note: Can be modified only once after a reset."]
    #[must_use]
    #[inline(always)]
    pub const fn ostpe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillation Stop Detection Enable Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub const fn set_ostpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GTETRG Input Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn st(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRG Input Status Flag"]
    #[inline(always)]
    pub const fn set_st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits are read as 00000000000. The write value should be 00000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x07ff;
        val as u16
    }
    #[doc = "These bits are read as 00000000000. The write value should be 00000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 17usize)) | (((val as u32) & 0x07ff) << 17usize);
    }
    #[doc = "GTETRG Input Reverse"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRG Input Reverse"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Noise Filter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nfen(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Noise Filter Enable"]
    #[inline(always)]
    pub const fn set_nfen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Noise Filter Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn nfcs(&self) -> super::vals::Nfcs {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Nfcs::from_bits(val as u8)
    }
    #[doc = "Noise Filter Clock Select"]
    #[inline(always)]
    pub const fn set_nfcs(&mut self, val: super::vals::Nfcs) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Poegg {
    #[inline(always)]
    fn default() -> Poegg {
        Poegg(0)
    }
}
impl core::fmt::Debug for Poegg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Poegg")
            .field("pidf", &self.pidf())
            .field("iocf", &self.iocf())
            .field("ostpf", &self.ostpf())
            .field("ssf", &self.ssf())
            .field("pide", &self.pide())
            .field("ioce", &self.ioce())
            .field("ostpe", &self.ostpe())
            .field("st", &self.st())
            .field("reserved", &self.reserved())
            .field("inv", &self.inv())
            .field("nfen", &self.nfen())
            .field("nfcs", &self.nfcs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Poegg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Poegg {{ pidf: {=bool:?}, iocf: {=bool:?}, ostpf: {=bool:?}, ssf: {=bool:?}, pide: {=bool:?}, ioce: {=bool:?}, ostpe: {=bool:?}, st: {=bool:?}, reserved: {=u16:?}, inv: {=bool:?}, nfen: {=bool:?}, nfcs: {:?} }}",
            self.pidf(),
            self.iocf(),
            self.ostpf(),
            self.ssf(),
            self.pide(),
            self.ioce(),
            self.ostpe(),
            self.st(),
            self.reserved(),
            self.inv(),
            self.nfen(),
            self.nfcs()
        )
    }
}
