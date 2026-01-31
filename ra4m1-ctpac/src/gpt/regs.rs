#[doc = "General PWM Timer Buffer Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtber(pub u32);
impl Gtber {
    #[doc = "BD\\[1\\]: GTPR Buffer Operation Disable BD\\[0\\]: GTCCR Buffer Operation Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn bd(&self) -> super::vals::Bd {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bd::from_bits(val as u8)
    }
    #[doc = "BD\\[1\\]: GTPR Buffer Operation Disable BD\\[0\\]: GTCCR Buffer Operation Disable"]
    #[inline(always)]
    pub const fn set_bd(&mut self, val: super::vals::Bd) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GTCCRA Buffer Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn ccra(&self) -> super::vals::Ccra {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ccra::from_bits(val as u8)
    }
    #[doc = "GTCCRA Buffer Operation"]
    #[inline(always)]
    pub const fn set_ccra(&mut self, val: super::vals::Ccra) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "GTCCRB Buffer Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn ccrb(&self) -> super::vals::Ccrb {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Ccrb::from_bits(val as u8)
    }
    #[doc = "GTCCRB Buffer Operation"]
    #[inline(always)]
    pub const fn set_ccrb(&mut self, val: super::vals::Ccrb) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "GTPR Buffer Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn pr(&self) -> super::vals::Pr {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Pr::from_bits(val as u8)
    }
    #[doc = "GTPR Buffer Operation"]
    #[inline(always)]
    pub const fn set_pr(&mut self, val: super::vals::Pr) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "GTCCRA and GTCCRB Forcible Buffer Operation This bit is read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ccrswt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "GTCCRA and GTCCRB Forcible Buffer Operation This bit is read as 0."]
    #[inline(always)]
    pub const fn set_ccrswt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Gtber {
    #[inline(always)]
    fn default() -> Gtber {
        Gtber(0)
    }
}
impl core::fmt::Debug for Gtber {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtber")
            .field("bd", &self.bd())
            .field("ccra", &self.ccra())
            .field("ccrb", &self.ccrb())
            .field("pr", &self.pr())
            .field("ccrswt", &self.ccrswt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtber {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtber {{ bd: {:?}, ccra: {:?}, ccrb: {:?}, pr: {:?}, ccrswt: {=bool:?} }}",
            self.bd(),
            self.ccra(),
            self.ccrb(),
            self.pr(),
            self.ccrswt()
        )
    }
}
#[doc = "General PWM Timer Software Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtclr(pub u32);
impl Gtclr {
    #[doc = "Channel 0 GTCNT Count Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cclr(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 GTCNT Count Clear"]
    #[inline(always)]
    pub const fn set_cclr(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "The write value should be 000000000000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "The write value should be 000000000000000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Gtclr {
    #[inline(always)]
    fn default() -> Gtclr {
        Gtclr(0)
    }
}
impl core::fmt::Debug for Gtclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtclr")
            .field("cclr[0]", &self.cclr(0usize))
            .field("cclr[1]", &self.cclr(1usize))
            .field("cclr[2]", &self.cclr(2usize))
            .field("cclr[3]", &self.cclr(3usize))
            .field("cclr[4]", &self.cclr(4usize))
            .field("cclr[5]", &self.cclr(5usize))
            .field("cclr[6]", &self.cclr(6usize))
            .field("cclr[7]", &self.cclr(7usize))
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtclr {{ cclr[0]: {=bool:?}, cclr[1]: {=bool:?}, cclr[2]: {=bool:?}, cclr[3]: {=bool:?}, cclr[4]: {=bool:?}, cclr[5]: {=bool:?}, cclr[6]: {=bool:?}, cclr[7]: {=bool:?}, reserved: {=u32:?} }}",
            self.cclr(0usize),
            self.cclr(1usize),
            self.cclr(2usize),
            self.cclr(3usize),
            self.cclr(4usize),
            self.cclr(5usize),
            self.cclr(6usize),
            self.cclr(7usize),
            self.reserved()
        )
    }
}
#[doc = "General PWM Timer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtcr(pub u32);
impl Gtcr {
    #[doc = "Count Start"]
    #[must_use]
    #[inline(always)]
    pub const fn cst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Count Start"]
    #[inline(always)]
    pub const fn set_cst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x7fff;
        val as u16
    }
    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 1usize)) | (((val as u32) & 0x7fff) << 1usize);
    }
    #[doc = "Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn md(&self) -> super::vals::Mode {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Mode Select"]
    #[inline(always)]
    pub const fn set_md(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "Timer Prescaler Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tpcs(&self) -> super::vals::Tpcs {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Tpcs::from_bits(val as u8)
    }
    #[doc = "Timer Prescaler Select"]
    #[inline(always)]
    pub const fn set_tpcs(&mut self, val: super::vals::Tpcs) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Gtcr {
    #[inline(always)]
    fn default() -> Gtcr {
        Gtcr(0)
    }
}
impl core::fmt::Debug for Gtcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtcr")
            .field("cst", &self.cst())
            .field("reserved", &self.reserved())
            .field("md", &self.md())
            .field("reserved_2", &self.reserved_2())
            .field("tpcs", &self.tpcs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtcr {{ cst: {=bool:?}, reserved: {=u16:?}, md: {:?}, reserved_2: {=u8:?}, tpcs: {:?} }}",
            self.cst(),
            self.reserved(),
            self.md(),
            self.reserved_2(),
            self.tpcs()
        )
    }
}
#[doc = "General PWM Timer Clear Source Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtcsr(pub u32);
impl Gtcsr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn csgtrgar(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_csgtrgar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn csgtrgaf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_csgtrgaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn csgtrgbr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_csgtrgbr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn csgtrgbf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_csgtrgbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscarbl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscarbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscarbh(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscarbh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscafbl(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscafbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscafbh(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscafbh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscbral(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscbral(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscbrah(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscbrah(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscbfal(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscbfal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscbfah(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscbfah(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ELC_GPTA Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselca(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTA Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ELC_GPTB Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselcb(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTB Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselcb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "ELC_GPTC Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselcc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTC Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ELC_GPTD Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselcd(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTD Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ELC_GPTE Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselce(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTE Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ELC_GPTF Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselcf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTF Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ELC_GPTG Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselcg(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTG Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselcg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ELCH Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselch(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "ELCH Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "Software Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cclr(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Gtcsr {
    #[inline(always)]
    fn default() -> Gtcsr {
        Gtcsr(0)
    }
}
impl core::fmt::Debug for Gtcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtcsr")
            .field("csgtrgar", &self.csgtrgar())
            .field("csgtrgaf", &self.csgtrgaf())
            .field("csgtrgbr", &self.csgtrgbr())
            .field("csgtrgbf", &self.csgtrgbf())
            .field("reserved", &self.reserved())
            .field("cscarbl", &self.cscarbl())
            .field("cscarbh", &self.cscarbh())
            .field("cscafbl", &self.cscafbl())
            .field("cscafbh", &self.cscafbh())
            .field("cscbral", &self.cscbral())
            .field("cscbrah", &self.cscbrah())
            .field("cscbfal", &self.cscbfal())
            .field("cscbfah", &self.cscbfah())
            .field("cselca", &self.cselca())
            .field("cselcb", &self.cselcb())
            .field("cselcc", &self.cselcc())
            .field("cselcd", &self.cselcd())
            .field("cselce", &self.cselce())
            .field("cselcf", &self.cselcf())
            .field("cselcg", &self.cselcg())
            .field("cselch", &self.cselch())
            .field("reserved_2", &self.reserved_2())
            .field("cclr", &self.cclr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtcsr {{ csgtrgar: {=bool:?}, csgtrgaf: {=bool:?}, csgtrgbr: {=bool:?}, csgtrgbf: {=bool:?}, reserved: {=u8:?}, cscarbl: {=bool:?}, cscarbh: {=bool:?}, cscafbl: {=bool:?}, cscafbh: {=bool:?}, cscbral: {=bool:?}, cscbrah: {=bool:?}, cscbfal: {=bool:?}, cscbfah: {=bool:?}, cselca: {=bool:?}, cselcb: {=bool:?}, cselcc: {=bool:?}, cselcd: {=bool:?}, cselce: {=bool:?}, cselcf: {=bool:?}, cselcg: {=bool:?}, cselch: {=bool:?}, reserved_2: {=u8:?}, cclr: {=bool:?} }}",
            self.csgtrgar(),
            self.csgtrgaf(),
            self.csgtrgbr(),
            self.csgtrgbf(),
            self.reserved(),
            self.cscarbl(),
            self.cscarbh(),
            self.cscafbl(),
            self.cscafbh(),
            self.cscbral(),
            self.cscbrah(),
            self.cscbfal(),
            self.cscbfah(),
            self.cselca(),
            self.cselcb(),
            self.cselcc(),
            self.cselcd(),
            self.cselce(),
            self.cselcf(),
            self.cselcg(),
            self.cselch(),
            self.reserved_2(),
            self.cclr()
        )
    }
}
#[doc = "General PWM Timer Down Count Source Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdnsr(pub u32);
impl Gtdnsr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dsgtrgar(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dsgtrgar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dsgtrgaf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dsgtrgaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dsgtrgbr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dsgtrgbr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dsgtrgbf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dsgtrgbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscarbl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscarbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscarbh(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscarbh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscafbl(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscafbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscafbh(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscafbh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscbral(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscbral(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscbrah(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscbrah(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscbfal(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscbfal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscbfah(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscbfah(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ELC_GPTA Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselca(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTA Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ELC_GPTB Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselcb(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTB Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselcb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "ELC_GPTC Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselcc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTC Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ELC_GPTD Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselcd(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTD Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ELC_GPTE Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselce(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTE Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ELC_GPTF Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselcf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTF Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ELC_GPTG Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselcg(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTG Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselcg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ELCH Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselch(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "ELCH Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Gtdnsr {
    #[inline(always)]
    fn default() -> Gtdnsr {
        Gtdnsr(0)
    }
}
impl core::fmt::Debug for Gtdnsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtdnsr")
            .field("dsgtrgar", &self.dsgtrgar())
            .field("dsgtrgaf", &self.dsgtrgaf())
            .field("dsgtrgbr", &self.dsgtrgbr())
            .field("dsgtrgbf", &self.dsgtrgbf())
            .field("dscarbl", &self.dscarbl())
            .field("dscarbh", &self.dscarbh())
            .field("dscafbl", &self.dscafbl())
            .field("dscafbh", &self.dscafbh())
            .field("dscbral", &self.dscbral())
            .field("dscbrah", &self.dscbrah())
            .field("dscbfal", &self.dscbfal())
            .field("dscbfah", &self.dscbfah())
            .field("dselca", &self.dselca())
            .field("dselcb", &self.dselcb())
            .field("dselcc", &self.dselcc())
            .field("dselcd", &self.dselcd())
            .field("dselce", &self.dselce())
            .field("dselcf", &self.dselcf())
            .field("dselcg", &self.dselcg())
            .field("dselch", &self.dselch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtdnsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtdnsr {{ dsgtrgar: {=bool:?}, dsgtrgaf: {=bool:?}, dsgtrgbr: {=bool:?}, dsgtrgbf: {=bool:?}, dscarbl: {=bool:?}, dscarbh: {=bool:?}, dscafbl: {=bool:?}, dscafbh: {=bool:?}, dscbral: {=bool:?}, dscbrah: {=bool:?}, dscbfal: {=bool:?}, dscbfah: {=bool:?}, dselca: {=bool:?}, dselcb: {=bool:?}, dselcc: {=bool:?}, dselcd: {=bool:?}, dselce: {=bool:?}, dselcf: {=bool:?}, dselcg: {=bool:?}, dselch: {=bool:?} }}",
            self.dsgtrgar(),
            self.dsgtrgaf(),
            self.dsgtrgbr(),
            self.dsgtrgbf(),
            self.dscarbl(),
            self.dscarbh(),
            self.dscafbl(),
            self.dscafbh(),
            self.dscbral(),
            self.dscbrah(),
            self.dscbfal(),
            self.dscbfah(),
            self.dselca(),
            self.dselcb(),
            self.dselcc(),
            self.dselcd(),
            self.dselce(),
            self.dselcf(),
            self.dselcg(),
            self.dselch()
        )
    }
}
#[doc = "General PWM Timer Dead Time Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdtcr(pub u32);
impl Gtdtcr {
    #[doc = "Negative-Phase Waveform Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn tde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Negative-Phase Waveform Setting"]
    #[inline(always)]
    pub const fn set_tde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_5(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "These bits are read as 00000000000000000000000. The write value should be 00000000000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_6(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "These bits are read as 00000000000000000000000. The write value should be 00000000000000000000000."]
    #[inline(always)]
    pub const fn set_reserved_6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for Gtdtcr {
    #[inline(always)]
    fn default() -> Gtdtcr {
        Gtdtcr(0)
    }
}
impl core::fmt::Debug for Gtdtcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtdtcr")
            .field("tde", &self.tde())
            .field("reserved", &self.reserved())
            .field("reserved_2", &self.reserved_2())
            .field("reserved_3", &self.reserved_3())
            .field("reserved_4", &self.reserved_4())
            .field("reserved_5", &self.reserved_5())
            .field("reserved_6", &self.reserved_6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtdtcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtdtcr {{ tde: {=bool:?}, reserved: {=u8:?}, reserved_2: {=bool:?}, reserved_3: {=bool:?}, reserved_4: {=u8:?}, reserved_5: {=bool:?}, reserved_6: {=u32:?} }}",
            self.tde(),
            self.reserved(),
            self.reserved_2(),
            self.reserved_3(),
            self.reserved_4(),
            self.reserved_5(),
            self.reserved_6()
        )
    }
}
#[doc = "General PWM Timer Dead Time Value Register U"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdvu(pub u32);
impl Gtdvu {
    #[doc = "Dead Time Value Register U"]
    #[must_use]
    #[inline(always)]
    pub const fn gtdvu(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Dead Time Value Register U"]
    #[inline(always)]
    pub const fn set_gtdvu(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gtdvu {
    #[inline(always)]
    fn default() -> Gtdvu {
        Gtdvu(0)
    }
}
impl core::fmt::Debug for Gtdvu {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtdvu")
            .field("gtdvu", &self.gtdvu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtdvu {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gtdvu {{ gtdvu: {=u32:?} }}", self.gtdvu())
    }
}
#[doc = "General PWM Timer Input Capture Source Select Register A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gticasr(pub u32);
impl Gticasr {
    #[doc = "GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asgtrgar(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_asgtrgar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asgtrgaf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_asgtrgaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asgtrgbr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_asgtrgbr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asgtrgbf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_asgtrgbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascarbl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascarbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascarbh(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascarbh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascafbl(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascafbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascafbh(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascafbh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascbral(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascbral(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascbrah(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascbrah(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascbfal(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascbfal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascbfah(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascbfah(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ELC_GPTA Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselca(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTA Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ELC_GPTB Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselcb(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTB Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselcb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "ELC_GPTC Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselcc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTC Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ELC_GPTD Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselcd(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTD Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ELC_GPTE Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselce(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTE Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ELC_GPTF Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselcf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTF Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ELC_GPTG Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselcg(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTG Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselcg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ELCH Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselch(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "ELCH Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Gticasr {
    #[inline(always)]
    fn default() -> Gticasr {
        Gticasr(0)
    }
}
impl core::fmt::Debug for Gticasr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gticasr")
            .field("asgtrgar", &self.asgtrgar())
            .field("asgtrgaf", &self.asgtrgaf())
            .field("asgtrgbr", &self.asgtrgbr())
            .field("asgtrgbf", &self.asgtrgbf())
            .field("ascarbl", &self.ascarbl())
            .field("ascarbh", &self.ascarbh())
            .field("ascafbl", &self.ascafbl())
            .field("ascafbh", &self.ascafbh())
            .field("ascbral", &self.ascbral())
            .field("ascbrah", &self.ascbrah())
            .field("ascbfal", &self.ascbfal())
            .field("ascbfah", &self.ascbfah())
            .field("aselca", &self.aselca())
            .field("aselcb", &self.aselcb())
            .field("aselcc", &self.aselcc())
            .field("aselcd", &self.aselcd())
            .field("aselce", &self.aselce())
            .field("aselcf", &self.aselcf())
            .field("aselcg", &self.aselcg())
            .field("aselch", &self.aselch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gticasr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gticasr {{ asgtrgar: {=bool:?}, asgtrgaf: {=bool:?}, asgtrgbr: {=bool:?}, asgtrgbf: {=bool:?}, ascarbl: {=bool:?}, ascarbh: {=bool:?}, ascafbl: {=bool:?}, ascafbh: {=bool:?}, ascbral: {=bool:?}, ascbrah: {=bool:?}, ascbfal: {=bool:?}, ascbfah: {=bool:?}, aselca: {=bool:?}, aselcb: {=bool:?}, aselcc: {=bool:?}, aselcd: {=bool:?}, aselce: {=bool:?}, aselcf: {=bool:?}, aselcg: {=bool:?}, aselch: {=bool:?} }}",
            self.asgtrgar(),
            self.asgtrgaf(),
            self.asgtrgbr(),
            self.asgtrgbf(),
            self.ascarbl(),
            self.ascarbh(),
            self.ascafbl(),
            self.ascafbh(),
            self.ascbral(),
            self.ascbrah(),
            self.ascbfal(),
            self.ascbfah(),
            self.aselca(),
            self.aselcb(),
            self.aselcc(),
            self.aselcd(),
            self.aselce(),
            self.aselcf(),
            self.aselcg(),
            self.aselch()
        )
    }
}
#[doc = "General PWM Timer Input Capture Source Select Register B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gticbsr(pub u32);
impl Gticbsr {
    #[doc = "GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bsgtrgar(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bsgtrgar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bsgtrgaf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bsgtrgaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bsgtrgbr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bsgtrgbr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bsgtrgbf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bsgtrgbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscarbl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscarbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscarbh(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscarbh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscafbl(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscafbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscafbh(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscafbh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscbral(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscbral(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscbrah(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscbrah(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscbfal(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscbfal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscbfah(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscbfah(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ELC_GPTA Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselca(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTA Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ELC_GPTB Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselcb(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTB Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselcb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "ELC_GPTC Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselcc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTC Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ELC_GPTD Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselcd(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTD Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ELC_GPTE Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselce(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTE Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ELC_GPTF Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselcf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTF Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ELC_GPTG Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselcg(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTG Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselcg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ELCH Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselch(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "ELCH Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Gticbsr {
    #[inline(always)]
    fn default() -> Gticbsr {
        Gticbsr(0)
    }
}
impl core::fmt::Debug for Gticbsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gticbsr")
            .field("bsgtrgar", &self.bsgtrgar())
            .field("bsgtrgaf", &self.bsgtrgaf())
            .field("bsgtrgbr", &self.bsgtrgbr())
            .field("bsgtrgbf", &self.bsgtrgbf())
            .field("bscarbl", &self.bscarbl())
            .field("bscarbh", &self.bscarbh())
            .field("bscafbl", &self.bscafbl())
            .field("bscafbh", &self.bscafbh())
            .field("bscbral", &self.bscbral())
            .field("bscbrah", &self.bscbrah())
            .field("bscbfal", &self.bscbfal())
            .field("bscbfah", &self.bscbfah())
            .field("bselca", &self.bselca())
            .field("bselcb", &self.bselcb())
            .field("bselcc", &self.bselcc())
            .field("bselcd", &self.bselcd())
            .field("bselce", &self.bselce())
            .field("bselcf", &self.bselcf())
            .field("bselcg", &self.bselcg())
            .field("bselch", &self.bselch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gticbsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gticbsr {{ bsgtrgar: {=bool:?}, bsgtrgaf: {=bool:?}, bsgtrgbr: {=bool:?}, bsgtrgbf: {=bool:?}, bscarbl: {=bool:?}, bscarbh: {=bool:?}, bscafbl: {=bool:?}, bscafbh: {=bool:?}, bscbral: {=bool:?}, bscbrah: {=bool:?}, bscbfal: {=bool:?}, bscbfah: {=bool:?}, bselca: {=bool:?}, bselcb: {=bool:?}, bselcc: {=bool:?}, bselcd: {=bool:?}, bselce: {=bool:?}, bselcf: {=bool:?}, bselcg: {=bool:?}, bselch: {=bool:?} }}",
            self.bsgtrgar(),
            self.bsgtrgaf(),
            self.bsgtrgbr(),
            self.bsgtrgbf(),
            self.bscarbl(),
            self.bscarbh(),
            self.bscafbl(),
            self.bscafbh(),
            self.bscbral(),
            self.bscbrah(),
            self.bscbfal(),
            self.bscbfah(),
            self.bselca(),
            self.bselcb(),
            self.bselcc(),
            self.bselcd(),
            self.bselce(),
            self.bselcf(),
            self.bselcg(),
            self.bselch()
        )
    }
}
#[doc = "General PWM Timer Interrupt Output Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtintad(pub u32);
impl Gtintad {
    #[doc = "These bits are read as 000000000000000000000000. The write value should be 000000000000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "These bits are read as 000000000000000000000000. The write value should be 000000000000000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Output Disable Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn grp(&self) -> super::vals::Grp {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Grp::from_bits(val as u8)
    }
    #[doc = "Output Disable Source Select"]
    #[inline(always)]
    pub const fn set_grp(&mut self, val: super::vals::Grp) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "Same Time Output Level High Disable Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn grpabh(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub const fn set_grpabh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Same Time Output Level Low Disable Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn grpabl(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub const fn set_grpabl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Gtintad {
    #[inline(always)]
    fn default() -> Gtintad {
        Gtintad(0)
    }
}
impl core::fmt::Debug for Gtintad {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtintad")
            .field("reserved", &self.reserved())
            .field("grp", &self.grp())
            .field("reserved_2", &self.reserved_2())
            .field("grpabh", &self.grpabh())
            .field("grpabl", &self.grpabl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtintad {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtintad {{ reserved: {=u32:?}, grp: {:?}, reserved_2: {=u8:?}, grpabh: {=bool:?}, grpabl: {=bool:?} }}",
            self.reserved(),
            self.grp(),
            self.reserved_2(),
            self.grpabh(),
            self.grpabl()
        )
    }
}
#[doc = "General PWM Timer I/O Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtior(pub u32);
impl Gtior {
    #[doc = "GTIOCA Pin Function Select"]
    #[must_use]
    #[inline(always)]
    pub const fn gtioa(&self) -> super::vals::Gtioa {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Gtioa::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Function Select"]
    #[inline(always)]
    pub const fn set_gtioa(&mut self, val: super::vals::Gtioa) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GTIOCA Pin Output Value Setting at the Count Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn oadflt(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub const fn set_oadflt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GTIOCA Pin Output Setting at the Start/Stop Count"]
    #[must_use]
    #[inline(always)]
    pub const fn oahld(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub const fn set_oahld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GTIOCA Pin Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oae(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Output Enable"]
    #[inline(always)]
    pub const fn set_oae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Disable Value Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn oadf(&self) -> super::vals::Oadf {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Oadf::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Disable Value Setting"]
    #[inline(always)]
    pub const fn set_oadf(&mut self, val: super::vals::Oadf) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "Noise Filter A Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nfaen(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Noise Filter A Enable"]
    #[inline(always)]
    pub const fn set_nfaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Noise Filter A Sampling Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn nfcsa(&self) -> super::vals::Nfcsa {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Nfcsa::from_bits(val as u8)
    }
    #[doc = "Noise Filter A Sampling Clock Select"]
    #[inline(always)]
    pub const fn set_nfcsa(&mut self, val: super::vals::Nfcsa) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "GTIOCB Pin Function Select"]
    #[must_use]
    #[inline(always)]
    pub const fn gtiob(&self) -> super::vals::Gtiob {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Gtiob::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Function Select"]
    #[inline(always)]
    pub const fn set_gtiob(&mut self, val: super::vals::Gtiob) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "GTIOCB Pin Output Value Setting at the Count Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn obdflt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub const fn set_obdflt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "GTIOCB Pin Output Setting at the Start/Stop Count"]
    #[must_use]
    #[inline(always)]
    pub const fn obhld(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub const fn set_obhld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GTIOCB Pin Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn obe(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Output Enable"]
    #[inline(always)]
    pub const fn set_obe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "GTIOCB Pin Disable Value Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn obdf(&self) -> super::vals::Obdf {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::Obdf::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Disable Value Setting"]
    #[inline(always)]
    pub const fn set_obdf(&mut self, val: super::vals::Obdf) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "Noise Filter B Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nfben(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Noise Filter B Enable"]
    #[inline(always)]
    pub const fn set_nfben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Noise Filter B Sampling Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn nfcsb(&self) -> super::vals::Nfcsb {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Nfcsb::from_bits(val as u8)
    }
    #[doc = "Noise Filter B Sampling Clock Select"]
    #[inline(always)]
    pub const fn set_nfcsb(&mut self, val: super::vals::Nfcsb) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Gtior {
    #[inline(always)]
    fn default() -> Gtior {
        Gtior(0)
    }
}
impl core::fmt::Debug for Gtior {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtior")
            .field("gtioa", &self.gtioa())
            .field("reserved", &self.reserved())
            .field("oadflt", &self.oadflt())
            .field("oahld", &self.oahld())
            .field("oae", &self.oae())
            .field("oadf", &self.oadf())
            .field("reserved_2", &self.reserved_2())
            .field("nfaen", &self.nfaen())
            .field("nfcsa", &self.nfcsa())
            .field("gtiob", &self.gtiob())
            .field("reserved_3", &self.reserved_3())
            .field("obdflt", &self.obdflt())
            .field("obhld", &self.obhld())
            .field("obe", &self.obe())
            .field("obdf", &self.obdf())
            .field("reserved_4", &self.reserved_4())
            .field("nfben", &self.nfben())
            .field("nfcsb", &self.nfcsb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtior {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtior {{ gtioa: {:?}, reserved: {=bool:?}, oadflt: {=bool:?}, oahld: {=bool:?}, oae: {=bool:?}, oadf: {:?}, reserved_2: {=u8:?}, nfaen: {=bool:?}, nfcsa: {:?}, gtiob: {:?}, reserved_3: {=bool:?}, obdflt: {=bool:?}, obhld: {=bool:?}, obe: {=bool:?}, obdf: {:?}, reserved_4: {=u8:?}, nfben: {=bool:?}, nfcsb: {:?} }}",
            self.gtioa(),
            self.reserved(),
            self.oadflt(),
            self.oahld(),
            self.oae(),
            self.oadf(),
            self.reserved_2(),
            self.nfaen(),
            self.nfcsa(),
            self.gtiob(),
            self.reserved_3(),
            self.obdflt(),
            self.obhld(),
            self.obe(),
            self.obdf(),
            self.reserved_4(),
            self.nfben(),
            self.nfcsb()
        )
    }
}
#[doc = "General PWM Timer Cycle Setting Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpbr(pub u32);
impl Gtpbr {
    #[doc = "Cycle Setting Buffer Register"]
    #[must_use]
    #[inline(always)]
    pub const fn gtpbr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cycle Setting Buffer Register"]
    #[inline(always)]
    pub const fn set_gtpbr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gtpbr {
    #[inline(always)]
    fn default() -> Gtpbr {
        Gtpbr(0)
    }
}
impl core::fmt::Debug for Gtpbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtpbr")
            .field("gtpbr", &self.gtpbr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtpbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gtpbr {{ gtpbr: {=u32:?} }}", self.gtpbr())
    }
}
#[doc = "General PWM Timer Stop Source Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpsr(pub u32);
impl Gtpsr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn psgtrgar(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_psgtrgar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn psgtrgaf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_psgtrgaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn psgtrgbr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_psgtrgbr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn psgtrgbf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_psgtrgbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscarbl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscarbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscarbh(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscarbh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscafbl(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscafbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscafbh(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscafbh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscbral(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscbral(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscbrah(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscbrah(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscbfal(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscbfal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscbfah(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscbfah(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ELC_GPTA Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselca(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTA Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ELC_GPTB Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselcb(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTB Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselcb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "ELC_GPTC Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselcc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTC Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ELC_GPTD Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselcd(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTD Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ELC_GPTE Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselce(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTE Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ELC_GPTF Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselcf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTF Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ELC_GPTG Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselcg(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTG Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselcg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ELCH Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselch(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "ELCH Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "Software Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cstop(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_cstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Gtpsr {
    #[inline(always)]
    fn default() -> Gtpsr {
        Gtpsr(0)
    }
}
impl core::fmt::Debug for Gtpsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtpsr")
            .field("psgtrgar", &self.psgtrgar())
            .field("psgtrgaf", &self.psgtrgaf())
            .field("psgtrgbr", &self.psgtrgbr())
            .field("psgtrgbf", &self.psgtrgbf())
            .field("reserved", &self.reserved())
            .field("pscarbl", &self.pscarbl())
            .field("pscarbh", &self.pscarbh())
            .field("pscafbl", &self.pscafbl())
            .field("pscafbh", &self.pscafbh())
            .field("pscbral", &self.pscbral())
            .field("pscbrah", &self.pscbrah())
            .field("pscbfal", &self.pscbfal())
            .field("pscbfah", &self.pscbfah())
            .field("pselca", &self.pselca())
            .field("pselcb", &self.pselcb())
            .field("pselcc", &self.pselcc())
            .field("pselcd", &self.pselcd())
            .field("pselce", &self.pselce())
            .field("pselcf", &self.pselcf())
            .field("pselcg", &self.pselcg())
            .field("pselch", &self.pselch())
            .field("reserved_2", &self.reserved_2())
            .field("cstop", &self.cstop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtpsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtpsr {{ psgtrgar: {=bool:?}, psgtrgaf: {=bool:?}, psgtrgbr: {=bool:?}, psgtrgbf: {=bool:?}, reserved: {=u8:?}, pscarbl: {=bool:?}, pscarbh: {=bool:?}, pscafbl: {=bool:?}, pscafbh: {=bool:?}, pscbral: {=bool:?}, pscbrah: {=bool:?}, pscbfal: {=bool:?}, pscbfah: {=bool:?}, pselca: {=bool:?}, pselcb: {=bool:?}, pselcc: {=bool:?}, pselcd: {=bool:?}, pselce: {=bool:?}, pselcf: {=bool:?}, pselcg: {=bool:?}, pselch: {=bool:?}, reserved_2: {=u8:?}, cstop: {=bool:?} }}",
            self.psgtrgar(),
            self.psgtrgaf(),
            self.psgtrgbr(),
            self.psgtrgbf(),
            self.reserved(),
            self.pscarbl(),
            self.pscarbh(),
            self.pscafbl(),
            self.pscafbh(),
            self.pscbral(),
            self.pscbrah(),
            self.pscbfal(),
            self.pscbfah(),
            self.pselca(),
            self.pselcb(),
            self.pselcc(),
            self.pselcd(),
            self.pselce(),
            self.pselcf(),
            self.pselcg(),
            self.pselch(),
            self.reserved_2(),
            self.cstop()
        )
    }
}
#[doc = "General PWM Timer Start Source Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtssr(pub u32);
impl Gtssr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ssgtrgar(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_ssgtrgar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ssgtrgaf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_ssgtrgaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ssgtrgbr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_ssgtrgbr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ssgtrgbf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_ssgtrgbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscarbl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscarbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscarbh(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscarbh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscafbl(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscafbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscafbh(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscafbh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscbral(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscbral(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscbrah(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscbrah(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscbfal(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscbfal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscbfah(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscbfah(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ELC_GPTA Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselca(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTA Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ELC_GPTB Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselcb(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTB Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselcb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "ELC_GPTC Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselcc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTC Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ELC_GPTD Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselcd(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTD Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ELC_GPTE Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselce(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTE Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ELC_GPTF Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselcf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTF Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ELC_GPTG Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselcg(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTG Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselcg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ELCH Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselch(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "ELCH Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "Software Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cstrt(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_cstrt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Gtssr {
    #[inline(always)]
    fn default() -> Gtssr {
        Gtssr(0)
    }
}
impl core::fmt::Debug for Gtssr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtssr")
            .field("ssgtrgar", &self.ssgtrgar())
            .field("ssgtrgaf", &self.ssgtrgaf())
            .field("ssgtrgbr", &self.ssgtrgbr())
            .field("ssgtrgbf", &self.ssgtrgbf())
            .field("reserved", &self.reserved())
            .field("sscarbl", &self.sscarbl())
            .field("sscarbh", &self.sscarbh())
            .field("sscafbl", &self.sscafbl())
            .field("sscafbh", &self.sscafbh())
            .field("sscbral", &self.sscbral())
            .field("sscbrah", &self.sscbrah())
            .field("sscbfal", &self.sscbfal())
            .field("sscbfah", &self.sscbfah())
            .field("sselca", &self.sselca())
            .field("sselcb", &self.sselcb())
            .field("sselcc", &self.sselcc())
            .field("sselcd", &self.sselcd())
            .field("sselce", &self.sselce())
            .field("sselcf", &self.sselcf())
            .field("sselcg", &self.sselcg())
            .field("sselch", &self.sselch())
            .field("reserved_2", &self.reserved_2())
            .field("cstrt", &self.cstrt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtssr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtssr {{ ssgtrgar: {=bool:?}, ssgtrgaf: {=bool:?}, ssgtrgbr: {=bool:?}, ssgtrgbf: {=bool:?}, reserved: {=u8:?}, sscarbl: {=bool:?}, sscarbh: {=bool:?}, sscafbl: {=bool:?}, sscafbh: {=bool:?}, sscbral: {=bool:?}, sscbrah: {=bool:?}, sscbfal: {=bool:?}, sscbfah: {=bool:?}, sselca: {=bool:?}, sselcb: {=bool:?}, sselcc: {=bool:?}, sselcd: {=bool:?}, sselce: {=bool:?}, sselcf: {=bool:?}, sselcg: {=bool:?}, sselch: {=bool:?}, reserved_2: {=u8:?}, cstrt: {=bool:?} }}",
            self.ssgtrgar(),
            self.ssgtrgaf(),
            self.ssgtrgbr(),
            self.ssgtrgbf(),
            self.reserved(),
            self.sscarbl(),
            self.sscarbh(),
            self.sscafbl(),
            self.sscafbh(),
            self.sscbral(),
            self.sscbrah(),
            self.sscbfal(),
            self.sscbfah(),
            self.sselca(),
            self.sselcb(),
            self.sselcc(),
            self.sselcd(),
            self.sselce(),
            self.sselcf(),
            self.sselcg(),
            self.sselch(),
            self.reserved_2(),
            self.cstrt()
        )
    }
}
#[doc = "General PWM Timer Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtst(pub u32);
impl Gtst {
    #[doc = "Input Capture/Compare Match Flag A"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfa(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Input Capture/Compare Match Flag A"]
    #[inline(always)]
    pub const fn set_tcfa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Input Capture/Compare Match Flag B"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Input Capture/Compare Match Flag B"]
    #[inline(always)]
    pub const fn set_tcfb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Input Compare Match Flag C"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Input Compare Match Flag C"]
    #[inline(always)]
    pub const fn set_tcfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Input Compare Match Flag D"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Input Compare Match Flag D"]
    #[inline(always)]
    pub const fn set_tcfd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Input Compare Match Flag E"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Input Compare Match Flag E"]
    #[inline(always)]
    pub const fn set_tcfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Input Compare Match Flag F"]
    #[must_use]
    #[inline(always)]
    pub const fn tcff(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Input Compare Match Flag F"]
    #[inline(always)]
    pub const fn set_tcff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfpo(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Flag"]
    #[inline(always)]
    pub const fn set_tcfpo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Underflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfpu(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Underflow Flag"]
    #[inline(always)]
    pub const fn set_tcfpu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Count Direction Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tucf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Count Direction Flag"]
    #[inline(always)]
    pub const fn set_tucf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Output Disable Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn odf(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Output Disable Flag"]
    #[inline(always)]
    pub const fn set_odf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
    }
    #[doc = "Same Time Output Level High Disable Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oabhf(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub const fn set_oabhf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Same Time Output Level Low Disable Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oablf(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub const fn set_oablf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Gtst {
    #[inline(always)]
    fn default() -> Gtst {
        Gtst(0)
    }
}
impl core::fmt::Debug for Gtst {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtst")
            .field("tcfa", &self.tcfa())
            .field("tcfb", &self.tcfb())
            .field("tcfc", &self.tcfc())
            .field("tcfd", &self.tcfd())
            .field("tcfe", &self.tcfe())
            .field("tcff", &self.tcff())
            .field("tcfpo", &self.tcfpo())
            .field("tcfpu", &self.tcfpu())
            .field("reserved", &self.reserved())
            .field("tucf", &self.tucf())
            .field("reserved_2", &self.reserved_2())
            .field("odf", &self.odf())
            .field("reserved_3", &self.reserved_3())
            .field("oabhf", &self.oabhf())
            .field("oablf", &self.oablf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtst {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtst {{ tcfa: {=bool:?}, tcfb: {=bool:?}, tcfc: {=bool:?}, tcfd: {=bool:?}, tcfe: {=bool:?}, tcff: {=bool:?}, tcfpo: {=bool:?}, tcfpu: {=bool:?}, reserved: {=u8:?}, tucf: {=bool:?}, reserved_2: {=u8:?}, odf: {=bool:?}, reserved_3: {=u8:?}, oabhf: {=bool:?}, oablf: {=bool:?} }}",
            self.tcfa(),
            self.tcfb(),
            self.tcfc(),
            self.tcfd(),
            self.tcfe(),
            self.tcff(),
            self.tcfpo(),
            self.tcfpu(),
            self.reserved(),
            self.tucf(),
            self.reserved_2(),
            self.odf(),
            self.reserved_3(),
            self.oabhf(),
            self.oablf()
        )
    }
}
#[doc = "General PWM Timer Software Stop Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtstp(pub u32);
impl Gtstp {
    #[doc = "Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[must_use]
    #[inline(always)]
    pub const fn cstop(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub const fn set_cstop(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "These bits are read as 111111111111111111111111. The write value should be 111111111111111111111111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "These bits are read as 111111111111111111111111. The write value should be 111111111111111111111111."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Gtstp {
    #[inline(always)]
    fn default() -> Gtstp {
        Gtstp(0)
    }
}
impl core::fmt::Debug for Gtstp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtstp")
            .field("cstop[0]", &self.cstop(0usize))
            .field("cstop[1]", &self.cstop(1usize))
            .field("cstop[2]", &self.cstop(2usize))
            .field("cstop[3]", &self.cstop(3usize))
            .field("cstop[4]", &self.cstop(4usize))
            .field("cstop[5]", &self.cstop(5usize))
            .field("cstop[6]", &self.cstop(6usize))
            .field("cstop[7]", &self.cstop(7usize))
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtstp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtstp {{ cstop[0]: {=bool:?}, cstop[1]: {=bool:?}, cstop[2]: {=bool:?}, cstop[3]: {=bool:?}, cstop[4]: {=bool:?}, cstop[5]: {=bool:?}, cstop[6]: {=bool:?}, cstop[7]: {=bool:?}, reserved: {=u32:?} }}",
            self.cstop(0usize),
            self.cstop(1usize),
            self.cstop(2usize),
            self.cstop(3usize),
            self.cstop(4usize),
            self.cstop(5usize),
            self.cstop(6usize),
            self.cstop(7usize),
            self.reserved()
        )
    }
}
#[doc = "General PWM Timer Software Start Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtstr(pub u32);
impl Gtstr {
    #[doc = "Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[must_use]
    #[inline(always)]
    pub const fn cstrt(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub const fn set_cstrt(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "These bits are read as 000000000000000000000000. The write value should be 000000000000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "These bits are read as 000000000000000000000000. The write value should be 000000000000000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Gtstr {
    #[inline(always)]
    fn default() -> Gtstr {
        Gtstr(0)
    }
}
impl core::fmt::Debug for Gtstr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtstr")
            .field("cstrt[0]", &self.cstrt(0usize))
            .field("cstrt[1]", &self.cstrt(1usize))
            .field("cstrt[2]", &self.cstrt(2usize))
            .field("cstrt[3]", &self.cstrt(3usize))
            .field("cstrt[4]", &self.cstrt(4usize))
            .field("cstrt[5]", &self.cstrt(5usize))
            .field("cstrt[6]", &self.cstrt(6usize))
            .field("cstrt[7]", &self.cstrt(7usize))
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtstr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtstr {{ cstrt[0]: {=bool:?}, cstrt[1]: {=bool:?}, cstrt[2]: {=bool:?}, cstrt[3]: {=bool:?}, cstrt[4]: {=bool:?}, cstrt[5]: {=bool:?}, cstrt[6]: {=bool:?}, cstrt[7]: {=bool:?}, reserved: {=u32:?} }}",
            self.cstrt(0usize),
            self.cstrt(1usize),
            self.cstrt(2usize),
            self.cstrt(3usize),
            self.cstrt(4usize),
            self.cstrt(5usize),
            self.cstrt(6usize),
            self.cstrt(7usize),
            self.reserved()
        )
    }
}
#[doc = "General PWM Timer Count Direction and Duty Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuddtyc(pub u32);
impl Gtuddtyc {
    #[doc = "Count Direction Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn ud(&self) -> super::vals::Ud {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ud::from_bits(val as u8)
    }
    #[doc = "Count Direction Setting"]
    #[inline(always)]
    pub const fn set_ud(&mut self, val: super::vals::Ud) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Forcible Count Direction Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn udf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Forcible Count Direction Setting"]
    #[inline(always)]
    pub const fn set_udf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GTIOCA Output Duty Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn oadty(&self) -> super::vals::Odty {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Odty::from_bits(val as u8)
    }
    #[doc = "GTIOCA Output Duty Setting"]
    #[inline(always)]
    pub const fn set_oadty(&mut self, val: super::vals::Odty) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Forcible GTIOCA Output Duty Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn oadtyf(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Forcible GTIOCA Output Duty Setting"]
    #[inline(always)]
    pub const fn set_oadtyf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "GTIOCA Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn oadtyr(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
    #[inline(always)]
    pub const fn set_oadtyr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "GTIOCB Output Duty Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn obdty(&self) -> super::vals::Odty {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Odty::from_bits(val as u8)
    }
    #[doc = "GTIOCB Output Duty Setting"]
    #[inline(always)]
    pub const fn set_obdty(&mut self, val: super::vals::Odty) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Forcible GTIOCB Output Duty Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn obdtyf(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Forcible GTIOCB Output Duty Setting"]
    #[inline(always)]
    pub const fn set_obdtyf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "GTIOCB Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn obdtyr(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
    #[inline(always)]
    pub const fn set_obdtyr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Gtuddtyc {
    #[inline(always)]
    fn default() -> Gtuddtyc {
        Gtuddtyc(0)
    }
}
impl core::fmt::Debug for Gtuddtyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtuddtyc")
            .field("ud", &self.ud())
            .field("udf", &self.udf())
            .field("oadty", &self.oadty())
            .field("oadtyf", &self.oadtyf())
            .field("oadtyr", &self.oadtyr())
            .field("reserved", &self.reserved())
            .field("obdty", &self.obdty())
            .field("obdtyf", &self.obdtyf())
            .field("obdtyr", &self.obdtyr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtuddtyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtuddtyc {{ ud: {:?}, udf: {=bool:?}, oadty: {:?}, oadtyf: {=bool:?}, oadtyr: {=bool:?}, reserved: {=u8:?}, obdty: {:?}, obdtyf: {=bool:?}, obdtyr: {=bool:?} }}",
            self.ud(),
            self.udf(),
            self.oadty(),
            self.oadtyf(),
            self.oadtyr(),
            self.reserved(),
            self.obdty(),
            self.obdtyf(),
            self.obdtyr()
        )
    }
}
#[doc = "General PWM Timer Up Count Source Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtupsr(pub u32);
impl Gtupsr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usgtrgar(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_usgtrgar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usgtrgaf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_usgtrgaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usgtrgbr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_usgtrgbr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usgtrgbf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_usgtrgbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscarbl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscarbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscarbh(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscarbh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscafbl(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscafbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscafbh(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscafbh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscbral(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscbral(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscbrah(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscbrah(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscbfal(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscbfal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscbfah(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscbfah(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ELC_GPTA Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselca(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTA Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ELC_GPTB Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselcb(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTB Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselcb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "ELC_GPTC Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselcc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTC Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ELC_GPTD Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselcd(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTD Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ELC_GPTE Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselce(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTE Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ELC_GPTF Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselcf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTF Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ELC_GPTG Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselcg(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ELC_GPTG Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselcg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ELCH Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselch(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "ELCH Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Gtupsr {
    #[inline(always)]
    fn default() -> Gtupsr {
        Gtupsr(0)
    }
}
impl core::fmt::Debug for Gtupsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtupsr")
            .field("usgtrgar", &self.usgtrgar())
            .field("usgtrgaf", &self.usgtrgaf())
            .field("usgtrgbr", &self.usgtrgbr())
            .field("usgtrgbf", &self.usgtrgbf())
            .field("uscarbl", &self.uscarbl())
            .field("uscarbh", &self.uscarbh())
            .field("uscafbl", &self.uscafbl())
            .field("uscafbh", &self.uscafbh())
            .field("uscbral", &self.uscbral())
            .field("uscbrah", &self.uscbrah())
            .field("uscbfal", &self.uscbfal())
            .field("uscbfah", &self.uscbfah())
            .field("uselca", &self.uselca())
            .field("uselcb", &self.uselcb())
            .field("uselcc", &self.uselcc())
            .field("uselcd", &self.uselcd())
            .field("uselce", &self.uselce())
            .field("uselcf", &self.uselcf())
            .field("uselcg", &self.uselcg())
            .field("uselch", &self.uselch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtupsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtupsr {{ usgtrgar: {=bool:?}, usgtrgaf: {=bool:?}, usgtrgbr: {=bool:?}, usgtrgbf: {=bool:?}, uscarbl: {=bool:?}, uscarbh: {=bool:?}, uscafbl: {=bool:?}, uscafbh: {=bool:?}, uscbral: {=bool:?}, uscbrah: {=bool:?}, uscbfal: {=bool:?}, uscbfah: {=bool:?}, uselca: {=bool:?}, uselcb: {=bool:?}, uselcc: {=bool:?}, uselcd: {=bool:?}, uselce: {=bool:?}, uselcf: {=bool:?}, uselcg: {=bool:?}, uselch: {=bool:?} }}",
            self.usgtrgar(),
            self.usgtrgaf(),
            self.usgtrgbr(),
            self.usgtrgbf(),
            self.uscarbl(),
            self.uscarbh(),
            self.uscafbl(),
            self.uscafbh(),
            self.uscbral(),
            self.uscbrah(),
            self.uscbfal(),
            self.uscbfah(),
            self.uselca(),
            self.uselcb(),
            self.uselcc(),
            self.uselcd(),
            self.uselce(),
            self.uselcf(),
            self.uselcg(),
            self.uselch()
        )
    }
}
#[doc = "General PWM Timer Write-Protection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtwp(pub u32);
impl Gtwp {
    #[doc = "Register Write Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn wp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Register Write Disable"]
    #[inline(always)]
    pub const fn set_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "GTWP Key Code"]
    #[must_use]
    #[inline(always)]
    pub const fn prkey(&self) -> super::vals::Prkey {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Prkey::from_bits(val as u8)
    }
    #[doc = "GTWP Key Code"]
    #[inline(always)]
    pub const fn set_prkey(&mut self, val: super::vals::Prkey) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Gtwp {
    #[inline(always)]
    fn default() -> Gtwp {
        Gtwp(0)
    }
}
impl core::fmt::Debug for Gtwp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtwp")
            .field("wp", &self.wp())
            .field("reserved", &self.reserved())
            .field("prkey", &self.prkey())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtwp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtwp {{ wp: {=bool:?}, reserved: {=u8:?}, prkey: {:?}, reserved_2: {=u16:?} }}",
            self.wp(),
            self.reserved(),
            self.prkey(),
            self.reserved_2()
        )
    }
}
