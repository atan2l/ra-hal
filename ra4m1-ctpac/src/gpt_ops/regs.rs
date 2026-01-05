#[doc = "Output Phase Switching Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opscr(pub u32);
impl Opscr {
    #[doc = "Input Phase Soft Setting WF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn uf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Input Phase Soft Setting WF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub const fn set_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Input Phase Soft Setting VF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn vf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Input Phase Soft Setting VF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub const fn set_vf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Input Phase Soft Setting UF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn wf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Input Phase Soft Setting UF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub const fn set_wf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Input U-Phase Monitor This bit monitors the state of the input phase. OPSCR.FB=0:External input monitoring by PCLK OPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[must_use]
    #[inline(always)]
    pub const fn u(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Input U-Phase Monitor This bit monitors the state of the input phase. OPSCR.FB=0:External input monitoring by PCLK OPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[inline(always)]
    pub const fn set_u(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Input V-Phase Monitor This bit monitors the state of the input phase. OPSCR.FB=0:External input monitoring by PCLK OPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[must_use]
    #[inline(always)]
    pub const fn v(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Input V-Phase Monitor This bit monitors the state of the input phase. OPSCR.FB=0:External input monitoring by PCLK OPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[inline(always)]
    pub const fn set_v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input W-Phase Monitor This bit monitors the state of the input phase. OPSCR.FB=0:External input monitoring by PCLK OPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[must_use]
    #[inline(always)]
    pub const fn w(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input W-Phase Monitor This bit monitors the state of the input phase. OPSCR.FB=0:External input monitoring by PCLK OPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[inline(always)]
    pub const fn set_w(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable-Phase Output Control"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> super::vals::En {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Enable-Phase Output Control"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "External Feedback Signal Enable This bit selects the input phase from the software settings and external input."]
    #[must_use]
    #[inline(always)]
    pub const fn fb(&self) -> super::vals::Fb {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Fb::from_bits(val as u8)
    }
    #[doc = "External Feedback Signal Enable This bit selects the input phase from the software settings and external input."]
    #[inline(always)]
    pub const fn set_fb(&mut self, val: super::vals::Fb) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Positive-Phase Output (P) Control"]
    #[must_use]
    #[inline(always)]
    pub const fn p(&self) -> super::vals::P {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::P::from_bits(val as u8)
    }
    #[doc = "Positive-Phase Output (P) Control"]
    #[inline(always)]
    pub const fn set_p(&mut self, val: super::vals::P) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Negative-Phase Output (N) Control"]
    #[must_use]
    #[inline(always)]
    pub const fn n(&self) -> super::vals::N {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::N::from_bits(val as u8)
    }
    #[doc = "Negative-Phase Output (N) Control"]
    #[inline(always)]
    pub const fn set_n(&mut self, val: super::vals::N) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Invert-Phase Output Control"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Inv {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Inv::from_bits(val as u8)
    }
    #[doc = "Invert-Phase Output Control"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Inv) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Output phase rotation direction reversal"]
    #[must_use]
    #[inline(always)]
    pub const fn rv(&self) -> super::vals::Rv {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Rv::from_bits(val as u8)
    }
    #[doc = "Output phase rotation direction reversal"]
    #[inline(always)]
    pub const fn set_rv(&mut self, val: super::vals::Rv) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Input phase alignment"]
    #[must_use]
    #[inline(always)]
    pub const fn align(&self) -> super::vals::Align {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Align::from_bits(val as u8)
    }
    #[doc = "Input phase alignment"]
    #[inline(always)]
    pub const fn set_align(&mut self, val: super::vals::Align) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Output disabled source selection"]
    #[must_use]
    #[inline(always)]
    pub const fn grp(&self) -> super::vals::Grp {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Grp::from_bits(val as u8)
    }
    #[doc = "Output disabled source selection"]
    #[inline(always)]
    pub const fn set_grp(&mut self, val: super::vals::Grp) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Group output disable function"]
    #[must_use]
    #[inline(always)]
    pub const fn godf(&self) -> super::vals::Godf {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Godf::from_bits(val as u8)
    }
    #[doc = "Group output disable function"]
    #[inline(always)]
    pub const fn set_godf(&mut self, val: super::vals::Godf) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_5(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "External Input Noise Filter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nfen(&self) -> super::vals::Nfen {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Nfen::from_bits(val as u8)
    }
    #[doc = "External Input Noise Filter Enable"]
    #[inline(always)]
    pub const fn set_nfen(&mut self, val: super::vals::Nfen) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "External Input Noise Filter Clock selection Noise filter sampling clock setting of the external input."]
    #[must_use]
    #[inline(always)]
    pub const fn nfcs(&self) -> super::vals::Nfcs {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Nfcs::from_bits(val as u8)
    }
    #[doc = "External Input Noise Filter Clock selection Noise filter sampling clock setting of the external input."]
    #[inline(always)]
    pub const fn set_nfcs(&mut self, val: super::vals::Nfcs) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Opscr {
    #[inline(always)]
    fn default() -> Opscr {
        Opscr(0)
    }
}
impl core::fmt::Debug for Opscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opscr")
            .field("uf", &self.uf())
            .field("vf", &self.vf())
            .field("wf", &self.wf())
            .field("reserved", &self.reserved())
            .field("u", &self.u())
            .field("v", &self.v())
            .field("w", &self.w())
            .field("reserved_2", &self.reserved_2())
            .field("en", &self.en())
            .field("reserved_3", &self.reserved_3())
            .field("fb", &self.fb())
            .field("p", &self.p())
            .field("n", &self.n())
            .field("inv", &self.inv())
            .field("rv", &self.rv())
            .field("align", &self.align())
            .field("reserved_4", &self.reserved_4())
            .field("grp", &self.grp())
            .field("godf", &self.godf())
            .field("reserved_5", &self.reserved_5())
            .field("nfen", &self.nfen())
            .field("nfcs", &self.nfcs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opscr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Opscr {{ uf: {=bool:?}, vf: {=bool:?}, wf: {=bool:?}, reserved: {=bool:?}, u: {=bool:?}, v: {=bool:?}, w: {=bool:?}, reserved_2: {=bool:?}, en: {:?}, reserved_3: {=u8:?}, fb: {:?}, p: {:?}, n: {:?}, inv: {:?}, rv: {:?}, align: {:?}, reserved_4: {=u8:?}, grp: {:?}, godf: {:?}, reserved_5: {=u8:?}, nfen: {:?}, nfcs: {:?} }}",
            self.uf(),
            self.vf(),
            self.wf(),
            self.reserved(),
            self.u(),
            self.v(),
            self.w(),
            self.reserved_2(),
            self.en(),
            self.reserved_3(),
            self.fb(),
            self.p(),
            self.n(),
            self.inv(),
            self.rv(),
            self.align(),
            self.reserved_4(),
            self.grp(),
            self.godf(),
            self.reserved_5(),
            self.nfen(),
            self.nfcs()
        )
    }
}
