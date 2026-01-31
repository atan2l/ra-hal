#[doc = "A/D-Converted Value Addition/Average Count Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adadc(pub u8);
impl Adadc {
    #[doc = "Addition frequency selection bit. NOTE: AVEE bit is valid at the only setting of ADC\\[2:0\\] bits = 001b or 011b. When average mode is selected by setting the ADADC.AVEE bit to 1, do not set the addition count to three times (ADADC.ADC\\[2:0\\] = 010b)"]
    #[must_use]
    #[inline(always)]
    pub const fn adc(&self) -> super::vals::Adc {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Adc::from_bits(val as u8)
    }
    #[doc = "Addition frequency selection bit. NOTE: AVEE bit is valid at the only setting of ADC\\[2:0\\] bits = 001b or 011b. When average mode is selected by setting the ADADC.AVEE bit to 1, do not set the addition count to three times (ADADC.ADC\\[2:0\\] = 010b)"]
    #[inline(always)]
    pub const fn set_adc(&mut self, val: super::vals::Adc) {
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
    #[doc = "Average mode enable bit. Note: The AVEE bit converts twice, and only when converting it four times, is effective. Please do not set (ADADC.AVEE=1) to conversion (ADADC.ADC 2:0=010b) three times when you select the average mode."]
    #[must_use]
    #[inline(always)]
    pub const fn avee(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Average mode enable bit. Note: The AVEE bit converts twice, and only when converting it four times, is effective. Please do not set (ADADC.AVEE=1) to conversion (ADADC.ADC 2:0=010b) three times when you select the average mode."]
    #[inline(always)]
    pub const fn set_avee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Adadc {
    #[inline(always)]
    fn default() -> Adadc {
        Adadc(0)
    }
}
impl core::fmt::Debug for Adadc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adadc")
            .field("adc", &self.adc())
            .field("reserved", &self.reserved())
            .field("avee", &self.avee())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adadc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adadc {{ adc: {:?}, reserved: {=u8:?}, avee: {=bool:?} }}",
            self.adc(),
            self.reserved(),
            self.avee()
        )
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel Select Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adads0(pub u16);
impl Adads0 {
    #[doc = "A/D-Converted Value Addition/Average Channel AN000 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads00(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN000 Select"]
    #[inline(always)]
    pub const fn set_ads00(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN001 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads01(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN001 Select"]
    #[inline(always)]
    pub const fn set_ads01(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN002 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads02(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN002 Select"]
    #[inline(always)]
    pub const fn set_ads02(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN003 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads03(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN003 Select"]
    #[inline(always)]
    pub const fn set_ads03(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN004 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads04(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN004 Select"]
    #[inline(always)]
    pub const fn set_ads04(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN005 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads05(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN005 Select"]
    #[inline(always)]
    pub const fn set_ads05(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN006 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads06(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN006 Select"]
    #[inline(always)]
    pub const fn set_ads06(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN007 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads07(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN007 Select"]
    #[inline(always)]
    pub const fn set_ads07(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN008 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads08(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN008 Select"]
    #[inline(always)]
    pub const fn set_ads08(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN009 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads09(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN009 Select"]
    #[inline(always)]
    pub const fn set_ads09(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN010 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN010 Select"]
    #[inline(always)]
    pub const fn set_ads10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN011 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN011 Select"]
    #[inline(always)]
    pub const fn set_ads11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN012 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN012 Select"]
    #[inline(always)]
    pub const fn set_ads12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN013 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN013 Select"]
    #[inline(always)]
    pub const fn set_ads13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN014 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN014 Select"]
    #[inline(always)]
    pub const fn set_ads14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Adads0 {
    #[inline(always)]
    fn default() -> Adads0 {
        Adads0(0)
    }
}
impl core::fmt::Debug for Adads0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adads0")
            .field("ads00", &self.ads00())
            .field("ads01", &self.ads01())
            .field("ads02", &self.ads02())
            .field("ads03", &self.ads03())
            .field("ads04", &self.ads04())
            .field("ads05", &self.ads05())
            .field("ads06", &self.ads06())
            .field("ads07", &self.ads07())
            .field("ads08", &self.ads08())
            .field("ads09", &self.ads09())
            .field("ads10", &self.ads10())
            .field("ads11", &self.ads11())
            .field("ads12", &self.ads12())
            .field("ads13", &self.ads13())
            .field("ads14", &self.ads14())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adads0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adads0 {{ ads00: {=bool:?}, ads01: {=bool:?}, ads02: {=bool:?}, ads03: {=bool:?}, ads04: {=bool:?}, ads05: {=bool:?}, ads06: {=bool:?}, ads07: {=bool:?}, ads08: {=bool:?}, ads09: {=bool:?}, ads10: {=bool:?}, ads11: {=bool:?}, ads12: {=bool:?}, ads13: {=bool:?}, ads14: {=bool:?}, reserved: {=bool:?} }}",
            self.ads00(),
            self.ads01(),
            self.ads02(),
            self.ads03(),
            self.ads04(),
            self.ads05(),
            self.ads06(),
            self.ads07(),
            self.ads08(),
            self.ads09(),
            self.ads10(),
            self.ads11(),
            self.ads12(),
            self.ads13(),
            self.ads14(),
            self.reserved()
        )
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel Select Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adads1(pub u16);
impl Adads1 {
    #[doc = "A/D-Converted Value Addition/Average Channel AN016 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads16(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN016 Select"]
    #[inline(always)]
    pub const fn set_ads16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN017 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads17(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN017 Select"]
    #[inline(always)]
    pub const fn set_ads17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN018 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads18(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN018 Select"]
    #[inline(always)]
    pub const fn set_ads18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN019 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads19(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN019 Select"]
    #[inline(always)]
    pub const fn set_ads19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN020 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads20(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN020 Select"]
    #[inline(always)]
    pub const fn set_ads20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN021 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads21(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN021 Select"]
    #[inline(always)]
    pub const fn set_ads21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN022 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads22(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN022 Select"]
    #[inline(always)]
    pub const fn set_ads22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN023 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads23(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN023 Select"]
    #[inline(always)]
    pub const fn set_ads23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN024 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads24(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN024 Select"]
    #[inline(always)]
    pub const fn set_ads24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN025 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ads25(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "A/D-Converted Value Addition/Average Channel AN025 Select"]
    #[inline(always)]
    pub const fn set_ads25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Adads1 {
    #[inline(always)]
    fn default() -> Adads1 {
        Adads1(0)
    }
}
impl core::fmt::Debug for Adads1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adads1")
            .field("ads16", &self.ads16())
            .field("ads17", &self.ads17())
            .field("ads18", &self.ads18())
            .field("ads19", &self.ads19())
            .field("ads20", &self.ads20())
            .field("ads21", &self.ads21())
            .field("ads22", &self.ads22())
            .field("ads23", &self.ads23())
            .field("ads24", &self.ads24())
            .field("ads25", &self.ads25())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adads1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adads1 {{ ads16: {=bool:?}, ads17: {=bool:?}, ads18: {=bool:?}, ads19: {=bool:?}, ads20: {=bool:?}, ads21: {=bool:?}, ads22: {=bool:?}, ads23: {=bool:?}, ads24: {=bool:?}, ads25: {=bool:?}, reserved: {=u8:?} }}",
            self.ads16(),
            self.ads17(),
            self.ads18(),
            self.ads19(),
            self.ads20(),
            self.ads21(),
            self.ads22(),
            self.ads23(),
            self.ads24(),
            self.ads25(),
            self.reserved()
        )
    }
}
#[doc = "A/D Channel Select Register A0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adansa0(pub u16);
impl Adansa0 {
    #[doc = "AN000 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansa(&self, n: usize) -> bool {
        assert!(n < 15usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "AN000 Select"]
    #[inline(always)]
    pub const fn set_ansa(&mut self, n: usize, val: bool) {
        assert!(n < 15usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Adansa0 {
    #[inline(always)]
    fn default() -> Adansa0 {
        Adansa0(0)
    }
}
impl core::fmt::Debug for Adansa0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adansa0")
            .field("ansa[0]", &self.ansa(0usize))
            .field("ansa[1]", &self.ansa(1usize))
            .field("ansa[2]", &self.ansa(2usize))
            .field("ansa[3]", &self.ansa(3usize))
            .field("ansa[4]", &self.ansa(4usize))
            .field("ansa[5]", &self.ansa(5usize))
            .field("ansa[6]", &self.ansa(6usize))
            .field("ansa[7]", &self.ansa(7usize))
            .field("ansa[8]", &self.ansa(8usize))
            .field("ansa[9]", &self.ansa(9usize))
            .field("ansa[10]", &self.ansa(10usize))
            .field("ansa[11]", &self.ansa(11usize))
            .field("ansa[12]", &self.ansa(12usize))
            .field("ansa[13]", &self.ansa(13usize))
            .field("ansa[14]", &self.ansa(14usize))
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adansa0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adansa0 {{ ansa[0]: {=bool:?}, ansa[1]: {=bool:?}, ansa[2]: {=bool:?}, ansa[3]: {=bool:?}, ansa[4]: {=bool:?}, ansa[5]: {=bool:?}, ansa[6]: {=bool:?}, ansa[7]: {=bool:?}, ansa[8]: {=bool:?}, ansa[9]: {=bool:?}, ansa[10]: {=bool:?}, ansa[11]: {=bool:?}, ansa[12]: {=bool:?}, ansa[13]: {=bool:?}, ansa[14]: {=bool:?}, reserved: {=bool:?} }}",
            self.ansa(0usize),
            self.ansa(1usize),
            self.ansa(2usize),
            self.ansa(3usize),
            self.ansa(4usize),
            self.ansa(5usize),
            self.ansa(6usize),
            self.ansa(7usize),
            self.ansa(8usize),
            self.ansa(9usize),
            self.ansa(10usize),
            self.ansa(11usize),
            self.ansa(12usize),
            self.ansa(13usize),
            self.ansa(14usize),
            self.reserved()
        )
    }
}
#[doc = "A/D Channel Select Register A1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adansa1(pub u16);
impl Adansa1 {
    #[doc = "AN016 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansa(&self, n: usize) -> bool {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "AN016 Select"]
    #[inline(always)]
    pub const fn set_ansa(&mut self, n: usize, val: bool) {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Adansa1 {
    #[inline(always)]
    fn default() -> Adansa1 {
        Adansa1(0)
    }
}
impl core::fmt::Debug for Adansa1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adansa1")
            .field("ansa[0]", &self.ansa(0usize))
            .field("ansa[1]", &self.ansa(1usize))
            .field("ansa[2]", &self.ansa(2usize))
            .field("ansa[3]", &self.ansa(3usize))
            .field("ansa[4]", &self.ansa(4usize))
            .field("ansa[5]", &self.ansa(5usize))
            .field("ansa[6]", &self.ansa(6usize))
            .field("ansa[7]", &self.ansa(7usize))
            .field("ansa[8]", &self.ansa(8usize))
            .field("ansa[9]", &self.ansa(9usize))
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adansa1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adansa1 {{ ansa[0]: {=bool:?}, ansa[1]: {=bool:?}, ansa[2]: {=bool:?}, ansa[3]: {=bool:?}, ansa[4]: {=bool:?}, ansa[5]: {=bool:?}, ansa[6]: {=bool:?}, ansa[7]: {=bool:?}, ansa[8]: {=bool:?}, ansa[9]: {=bool:?}, reserved: {=u8:?} }}",
            self.ansa(0usize),
            self.ansa(1usize),
            self.ansa(2usize),
            self.ansa(3usize),
            self.ansa(4usize),
            self.ansa(5usize),
            self.ansa(6usize),
            self.ansa(7usize),
            self.ansa(8usize),
            self.ansa(9usize),
            self.reserved()
        )
    }
}
#[doc = "A/D Channel Select Register B0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adansb0(pub u16);
impl Adansb0 {
    #[doc = "AN000 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb00(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AN000 Select"]
    #[inline(always)]
    pub const fn set_ansb00(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "AN001 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb01(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "AN001 Select"]
    #[inline(always)]
    pub const fn set_ansb01(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "AN002 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb02(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AN002 Select"]
    #[inline(always)]
    pub const fn set_ansb02(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "AN003 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb03(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "AN003 Select"]
    #[inline(always)]
    pub const fn set_ansb03(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "AN004 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb04(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AN004 Select"]
    #[inline(always)]
    pub const fn set_ansb04(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "AN005 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb05(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "AN005 Select"]
    #[inline(always)]
    pub const fn set_ansb05(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "AN006 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb06(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "AN006 Select"]
    #[inline(always)]
    pub const fn set_ansb06(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "AN007 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb07(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "AN007 Select"]
    #[inline(always)]
    pub const fn set_ansb07(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "AN008 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb08(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "AN008 Select"]
    #[inline(always)]
    pub const fn set_ansb08(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "AN009 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb09(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "AN009 Select"]
    #[inline(always)]
    pub const fn set_ansb09(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "AN010 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "AN010 Select"]
    #[inline(always)]
    pub const fn set_ansb10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "AN011 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "AN011 Select"]
    #[inline(always)]
    pub const fn set_ansb11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "AN012 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "AN012 Select"]
    #[inline(always)]
    pub const fn set_ansb12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "AN013 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "AN013 Select"]
    #[inline(always)]
    pub const fn set_ansb13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "AN014 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "AN014 Select"]
    #[inline(always)]
    pub const fn set_ansb14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Adansb0 {
    #[inline(always)]
    fn default() -> Adansb0 {
        Adansb0(0)
    }
}
impl core::fmt::Debug for Adansb0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adansb0")
            .field("ansb00", &self.ansb00())
            .field("ansb01", &self.ansb01())
            .field("ansb02", &self.ansb02())
            .field("ansb03", &self.ansb03())
            .field("ansb04", &self.ansb04())
            .field("ansb05", &self.ansb05())
            .field("ansb06", &self.ansb06())
            .field("ansb07", &self.ansb07())
            .field("ansb08", &self.ansb08())
            .field("ansb09", &self.ansb09())
            .field("ansb10", &self.ansb10())
            .field("ansb11", &self.ansb11())
            .field("ansb12", &self.ansb12())
            .field("ansb13", &self.ansb13())
            .field("ansb14", &self.ansb14())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adansb0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adansb0 {{ ansb00: {=bool:?}, ansb01: {=bool:?}, ansb02: {=bool:?}, ansb03: {=bool:?}, ansb04: {=bool:?}, ansb05: {=bool:?}, ansb06: {=bool:?}, ansb07: {=bool:?}, ansb08: {=bool:?}, ansb09: {=bool:?}, ansb10: {=bool:?}, ansb11: {=bool:?}, ansb12: {=bool:?}, ansb13: {=bool:?}, ansb14: {=bool:?}, reserved: {=bool:?} }}",
            self.ansb00(),
            self.ansb01(),
            self.ansb02(),
            self.ansb03(),
            self.ansb04(),
            self.ansb05(),
            self.ansb06(),
            self.ansb07(),
            self.ansb08(),
            self.ansb09(),
            self.ansb10(),
            self.ansb11(),
            self.ansb12(),
            self.ansb13(),
            self.ansb14(),
            self.reserved()
        )
    }
}
#[doc = "A/D Channel Select Register B1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adansb1(pub u16);
impl Adansb1 {
    #[doc = "AN016 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb16(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AN016 Select"]
    #[inline(always)]
    pub const fn set_ansb16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "AN017 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb17(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "AN017 Select"]
    #[inline(always)]
    pub const fn set_ansb17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "AN018 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb18(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AN018 Select"]
    #[inline(always)]
    pub const fn set_ansb18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "AN019 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb19(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "AN019 Select"]
    #[inline(always)]
    pub const fn set_ansb19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "AN020 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb20(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AN020 Select"]
    #[inline(always)]
    pub const fn set_ansb20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "AN021 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb21(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "AN021 Select"]
    #[inline(always)]
    pub const fn set_ansb21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "AN022 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb22(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "AN022 Select"]
    #[inline(always)]
    pub const fn set_ansb22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "AN023 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb23(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "AN023 Select"]
    #[inline(always)]
    pub const fn set_ansb23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "AN024 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb24(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "AN024 Select"]
    #[inline(always)]
    pub const fn set_ansb24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "AN025 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ansb25(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "AN025 Select"]
    #[inline(always)]
    pub const fn set_ansb25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Adansb1 {
    #[inline(always)]
    fn default() -> Adansb1 {
        Adansb1(0)
    }
}
impl core::fmt::Debug for Adansb1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adansb1")
            .field("ansb16", &self.ansb16())
            .field("ansb17", &self.ansb17())
            .field("ansb18", &self.ansb18())
            .field("ansb19", &self.ansb19())
            .field("ansb20", &self.ansb20())
            .field("ansb21", &self.ansb21())
            .field("ansb22", &self.ansb22())
            .field("ansb23", &self.ansb23())
            .field("ansb24", &self.ansb24())
            .field("ansb25", &self.ansb25())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adansb1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adansb1 {{ ansb16: {=bool:?}, ansb17: {=bool:?}, ansb18: {=bool:?}, ansb19: {=bool:?}, ansb20: {=bool:?}, ansb21: {=bool:?}, ansb22: {=bool:?}, ansb23: {=bool:?}, ansb24: {=bool:?}, ansb25: {=bool:?}, reserved: {=u8:?} }}",
            self.ansb16(),
            self.ansb17(),
            self.ansb18(),
            self.ansb19(),
            self.ansb20(),
            self.ansb21(),
            self.ansb22(),
            self.ansb23(),
            self.ansb24(),
            self.ansb25(),
            self.reserved()
        )
    }
}
#[doc = "A/D Control Extended Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcer(pub u16);
impl Adcer {
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "A/D Conversion Accuracy Specify"]
    #[must_use]
    #[inline(always)]
    pub const fn adprc(&self) -> super::vals::Adprc {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Adprc::from_bits(val as u8)
    }
    #[doc = "A/D Conversion Accuracy Specify"]
    #[inline(always)]
    pub const fn set_adprc(&mut self, val: super::vals::Adprc) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u16) & 0x03) << 1usize);
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
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "A/D Data Register Automatic Clearing Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ace(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "A/D Data Register Automatic Clearing Enable"]
    #[inline(always)]
    pub const fn set_ace(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
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
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
    }
    #[doc = "Self-Diagnosis Conversion Voltage Select"]
    #[must_use]
    #[inline(always)]
    pub const fn diagval(&self) -> super::vals::Diagval {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Diagval::from_bits(val as u8)
    }
    #[doc = "Self-Diagnosis Conversion Voltage Select"]
    #[inline(always)]
    pub const fn set_diagval(&mut self, val: super::vals::Diagval) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Self-Diagnosis Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn diagld(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Self-Diagnosis Mode Select"]
    #[inline(always)]
    pub const fn set_diagld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Self-Diagnosis Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn diagm(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Self-Diagnosis Enable"]
    #[inline(always)]
    pub const fn set_diagm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_5(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u16) & 0x07) << 12usize);
    }
    #[doc = "A/D Data Register Format Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adrfmt(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "A/D Data Register Format Select"]
    #[inline(always)]
    pub const fn set_adrfmt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Adcer {
    #[inline(always)]
    fn default() -> Adcer {
        Adcer(0)
    }
}
impl core::fmt::Debug for Adcer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcer")
            .field("reserved", &self.reserved())
            .field("adprc", &self.adprc())
            .field("reserved_2", &self.reserved_2())
            .field("reserved_3", &self.reserved_3())
            .field("ace", &self.ace())
            .field("reserved_4", &self.reserved_4())
            .field("diagval", &self.diagval())
            .field("diagld", &self.diagld())
            .field("diagm", &self.diagm())
            .field("reserved_5", &self.reserved_5())
            .field("adrfmt", &self.adrfmt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcer {{ reserved: {=bool:?}, adprc: {:?}, reserved_2: {=bool:?}, reserved_3: {=bool:?}, ace: {=bool:?}, reserved_4: {=u8:?}, diagval: {:?}, diagld: {=bool:?}, diagm: {=bool:?}, reserved_5: {=u8:?}, adrfmt: {=bool:?} }}",
            self.reserved(),
            self.adprc(),
            self.reserved_2(),
            self.reserved_3(),
            self.ace(),
            self.reserved_4(),
            self.diagval(),
            self.diagld(),
            self.diagm(),
            self.reserved_5(),
            self.adrfmt()
        )
    }
}
#[doc = "A/D Compare Function Window A Extended Input Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpanser(pub u8);
impl Adcmpanser {
    #[doc = "Temperature sensor output Compare selection bit."]
    #[must_use]
    #[inline(always)]
    pub const fn cmptsa(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature sensor output Compare selection bit."]
    #[inline(always)]
    pub const fn set_cmptsa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Internal reference voltage Compare selection bit."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpoca(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Internal reference voltage Compare selection bit."]
    #[inline(always)]
    pub const fn set_cmpoca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for Adcmpanser {
    #[inline(always)]
    fn default() -> Adcmpanser {
        Adcmpanser(0)
    }
}
impl core::fmt::Debug for Adcmpanser {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcmpanser")
            .field("cmptsa", &self.cmptsa())
            .field("cmpoca", &self.cmpoca())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcmpanser {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcmpanser {{ cmptsa: {=bool:?}, cmpoca: {=bool:?}, reserved: {=u8:?} }}",
            self.cmptsa(),
            self.cmpoca(),
            self.reserved()
        )
    }
}
#[doc = "A/D Compare Function Window A Channel Select Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpansr0(pub u16);
impl Adcmpansr0 {
    #[doc = "AN000 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha00(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AN000 Select"]
    #[inline(always)]
    pub const fn set_cmpcha00(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "AN001 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha01(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "AN001 Select"]
    #[inline(always)]
    pub const fn set_cmpcha01(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "AN002 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha02(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AN002 Select"]
    #[inline(always)]
    pub const fn set_cmpcha02(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "AN003 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha03(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "AN003 Select"]
    #[inline(always)]
    pub const fn set_cmpcha03(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "AN004 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha04(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AN004 Select"]
    #[inline(always)]
    pub const fn set_cmpcha04(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "AN005 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha05(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "AN005 Select"]
    #[inline(always)]
    pub const fn set_cmpcha05(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "AN006 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha06(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "AN006 Select"]
    #[inline(always)]
    pub const fn set_cmpcha06(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "AN007 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha07(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "AN007 Select"]
    #[inline(always)]
    pub const fn set_cmpcha07(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "AN008 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha08(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "AN008 Select"]
    #[inline(always)]
    pub const fn set_cmpcha08(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "AN009 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha09(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "AN009 Select"]
    #[inline(always)]
    pub const fn set_cmpcha09(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "AN010 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "AN010 Select"]
    #[inline(always)]
    pub const fn set_cmpcha10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "AN011 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "AN011 Select"]
    #[inline(always)]
    pub const fn set_cmpcha11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "AN012 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "AN012 Select"]
    #[inline(always)]
    pub const fn set_cmpcha12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "AN013 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "AN013 Select"]
    #[inline(always)]
    pub const fn set_cmpcha13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "AN014 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "AN014 Select"]
    #[inline(always)]
    pub const fn set_cmpcha14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Adcmpansr0 {
    #[inline(always)]
    fn default() -> Adcmpansr0 {
        Adcmpansr0(0)
    }
}
impl core::fmt::Debug for Adcmpansr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcmpansr0")
            .field("cmpcha00", &self.cmpcha00())
            .field("cmpcha01", &self.cmpcha01())
            .field("cmpcha02", &self.cmpcha02())
            .field("cmpcha03", &self.cmpcha03())
            .field("cmpcha04", &self.cmpcha04())
            .field("cmpcha05", &self.cmpcha05())
            .field("cmpcha06", &self.cmpcha06())
            .field("cmpcha07", &self.cmpcha07())
            .field("cmpcha08", &self.cmpcha08())
            .field("cmpcha09", &self.cmpcha09())
            .field("cmpcha10", &self.cmpcha10())
            .field("cmpcha11", &self.cmpcha11())
            .field("cmpcha12", &self.cmpcha12())
            .field("cmpcha13", &self.cmpcha13())
            .field("cmpcha14", &self.cmpcha14())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcmpansr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcmpansr0 {{ cmpcha00: {=bool:?}, cmpcha01: {=bool:?}, cmpcha02: {=bool:?}, cmpcha03: {=bool:?}, cmpcha04: {=bool:?}, cmpcha05: {=bool:?}, cmpcha06: {=bool:?}, cmpcha07: {=bool:?}, cmpcha08: {=bool:?}, cmpcha09: {=bool:?}, cmpcha10: {=bool:?}, cmpcha11: {=bool:?}, cmpcha12: {=bool:?}, cmpcha13: {=bool:?}, cmpcha14: {=bool:?}, reserved: {=bool:?} }}",
            self.cmpcha00(),
            self.cmpcha01(),
            self.cmpcha02(),
            self.cmpcha03(),
            self.cmpcha04(),
            self.cmpcha05(),
            self.cmpcha06(),
            self.cmpcha07(),
            self.cmpcha08(),
            self.cmpcha09(),
            self.cmpcha10(),
            self.cmpcha11(),
            self.cmpcha12(),
            self.cmpcha13(),
            self.cmpcha14(),
            self.reserved()
        )
    }
}
#[doc = "A/D Compare Function Window A Channel Select Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpansr1(pub u16);
impl Adcmpansr1 {
    #[doc = "AN016 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha16(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AN016 Select"]
    #[inline(always)]
    pub const fn set_cmpcha16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "AN017 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha17(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "AN017 Select"]
    #[inline(always)]
    pub const fn set_cmpcha17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "AN018 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha18(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AN018 Select"]
    #[inline(always)]
    pub const fn set_cmpcha18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "AN019 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha19(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "AN019 Select"]
    #[inline(always)]
    pub const fn set_cmpcha19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "AN020 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha20(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AN020 Select"]
    #[inline(always)]
    pub const fn set_cmpcha20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "AN021 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha21(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "AN021 Select"]
    #[inline(always)]
    pub const fn set_cmpcha21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "AN022 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha22(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "AN022 Select"]
    #[inline(always)]
    pub const fn set_cmpcha22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "AN023 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha23(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "AN023 Select"]
    #[inline(always)]
    pub const fn set_cmpcha23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "AN024 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha24(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "AN024 Select"]
    #[inline(always)]
    pub const fn set_cmpcha24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "AN025 Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcha25(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "AN025 Select"]
    #[inline(always)]
    pub const fn set_cmpcha25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Adcmpansr1 {
    #[inline(always)]
    fn default() -> Adcmpansr1 {
        Adcmpansr1(0)
    }
}
impl core::fmt::Debug for Adcmpansr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcmpansr1")
            .field("cmpcha16", &self.cmpcha16())
            .field("cmpcha17", &self.cmpcha17())
            .field("cmpcha18", &self.cmpcha18())
            .field("cmpcha19", &self.cmpcha19())
            .field("cmpcha20", &self.cmpcha20())
            .field("cmpcha21", &self.cmpcha21())
            .field("cmpcha22", &self.cmpcha22())
            .field("cmpcha23", &self.cmpcha23())
            .field("cmpcha24", &self.cmpcha24())
            .field("cmpcha25", &self.cmpcha25())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcmpansr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcmpansr1 {{ cmpcha16: {=bool:?}, cmpcha17: {=bool:?}, cmpcha18: {=bool:?}, cmpcha19: {=bool:?}, cmpcha20: {=bool:?}, cmpcha21: {=bool:?}, cmpcha22: {=bool:?}, cmpcha23: {=bool:?}, cmpcha24: {=bool:?}, cmpcha25: {=bool:?}, reserved: {=u8:?} }}",
            self.cmpcha16(),
            self.cmpcha17(),
            self.cmpcha18(),
            self.cmpcha19(),
            self.cmpcha20(),
            self.cmpcha21(),
            self.cmpcha22(),
            self.cmpcha23(),
            self.cmpcha24(),
            self.cmpcha25(),
            self.reserved()
        )
    }
}
#[doc = "A/D Compare Function Window B Channel Selection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpbnsr(pub u8);
impl Adcmpbnsr {
    #[doc = "Compare window B channel selection bit. The channel that compares it on the condition of compare window B is selected."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpchb(&self) -> super::vals::Cmpchb {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Cmpchb::from_bits(val as u8)
    }
    #[doc = "Compare window B channel selection bit. The channel that compares it on the condition of compare window B is selected."]
    #[inline(always)]
    pub const fn set_cmpchb(&mut self, val: super::vals::Cmpchb) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u8) & 0x3f) << 0usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Compare window B Compare condition setting bit."]
    #[must_use]
    #[inline(always)]
    pub const fn cmplb(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window B Compare condition setting bit."]
    #[inline(always)]
    pub const fn set_cmplb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Adcmpbnsr {
    #[inline(always)]
    fn default() -> Adcmpbnsr {
        Adcmpbnsr(0)
    }
}
impl core::fmt::Debug for Adcmpbnsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcmpbnsr")
            .field("cmpchb", &self.cmpchb())
            .field("reserved", &self.reserved())
            .field("cmplb", &self.cmplb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcmpbnsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcmpbnsr {{ cmpchb: {:?}, reserved: {=bool:?}, cmplb: {=bool:?} }}",
            self.cmpchb(),
            self.reserved(),
            self.cmplb()
        )
    }
}
#[doc = "A/D Compare Function Window B Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpbsr(pub u8);
impl Adcmpbsr {
    #[doc = "Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN027, temperature sensor, and internal reference voltage) made the object of window B relation condition."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN027, temperature sensor, and internal reference voltage) made the object of window B relation condition."]
    #[inline(always)]
    pub const fn set_cmpstb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
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
impl Default for Adcmpbsr {
    #[inline(always)]
    fn default() -> Adcmpbsr {
        Adcmpbsr(0)
    }
}
impl core::fmt::Debug for Adcmpbsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcmpbsr")
            .field("cmpstb", &self.cmpstb())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcmpbsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcmpbsr {{ cmpstb: {=bool:?}, reserved: {=u8:?} }}",
            self.cmpstb(),
            self.reserved()
        )
    }
}
#[doc = "A/D Compare Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpcr(pub u16);
impl Adcmpcr {
    #[doc = "Window A/B Composite Conditions Setting NOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1)."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpab(&self) -> super::vals::Cmpab {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmpab::from_bits(val as u8)
    }
    #[doc = "Window A/B Composite Conditions Setting NOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1)."]
    #[inline(always)]
    pub const fn set_cmpab(&mut self, val: super::vals::Cmpab) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u16) & 0x7f) << 2usize);
    }
    #[doc = "Compare Window B Operation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpbe(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Window B Operation Enable"]
    #[inline(always)]
    pub const fn set_cmpbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Compare Window A Operation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpae(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Window A Operation Enable"]
    #[inline(always)]
    pub const fn set_cmpae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Compare B Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpbie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Compare B Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cmpbie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Window Function Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn wcmpe(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Window Function Setting"]
    #[inline(always)]
    pub const fn set_wcmpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Compare A Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpaie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Compare A Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cmpaie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Adcmpcr {
    #[inline(always)]
    fn default() -> Adcmpcr {
        Adcmpcr(0)
    }
}
impl core::fmt::Debug for Adcmpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcmpcr")
            .field("cmpab", &self.cmpab())
            .field("reserved", &self.reserved())
            .field("cmpbe", &self.cmpbe())
            .field("reserved_2", &self.reserved_2())
            .field("cmpae", &self.cmpae())
            .field("reserved_3", &self.reserved_3())
            .field("cmpbie", &self.cmpbie())
            .field("wcmpe", &self.wcmpe())
            .field("cmpaie", &self.cmpaie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcmpcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcmpcr {{ cmpab: {:?}, reserved: {=u8:?}, cmpbe: {=bool:?}, reserved_2: {=bool:?}, cmpae: {=bool:?}, reserved_3: {=bool:?}, cmpbie: {=bool:?}, wcmpe: {=bool:?}, cmpaie: {=bool:?} }}",
            self.cmpab(),
            self.reserved(),
            self.cmpbe(),
            self.reserved_2(),
            self.cmpae(),
            self.reserved_3(),
            self.cmpbie(),
            self.wcmpe(),
            self.cmpaie()
        )
    }
}
#[doc = "A/D Compare Function Window A Extended Input Comparison Condition Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpler(pub u8);
impl Adcmpler {
    #[doc = "Compare Window A Temperature Sensor Output Comparison Condition Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpltsa(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Window A Temperature Sensor Output Comparison Condition Select"]
    #[inline(always)]
    pub const fn set_cmpltsa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Compare Window A Internal Reference Voltage Comparison Condition Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmploca(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Window A Internal Reference Voltage Comparison Condition Select"]
    #[inline(always)]
    pub const fn set_cmploca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for Adcmpler {
    #[inline(always)]
    fn default() -> Adcmpler {
        Adcmpler(0)
    }
}
impl core::fmt::Debug for Adcmpler {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcmpler")
            .field("cmpltsa", &self.cmpltsa())
            .field("cmploca", &self.cmploca())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcmpler {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcmpler {{ cmpltsa: {=bool:?}, cmploca: {=bool:?}, reserved: {=u8:?} }}",
            self.cmpltsa(),
            self.cmploca(),
            self.reserved()
        )
    }
}
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmplr0(pub u16);
impl Adcmplr0 {
    #[doc = "Comparison condition of AN000"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha00(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN000"]
    #[inline(always)]
    pub const fn set_cmplcha00(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Comparison condition of AN001"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha01(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN001"]
    #[inline(always)]
    pub const fn set_cmplcha01(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Comparison condition of AN002"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha02(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN002"]
    #[inline(always)]
    pub const fn set_cmplcha02(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Comparison condition of AN003"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha03(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN003"]
    #[inline(always)]
    pub const fn set_cmplcha03(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Comparison condition of AN004"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha04(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN004"]
    #[inline(always)]
    pub const fn set_cmplcha04(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Comparison condition of AN005"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha05(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN005"]
    #[inline(always)]
    pub const fn set_cmplcha05(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Comparison condition of AN006"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha06(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN006"]
    #[inline(always)]
    pub const fn set_cmplcha06(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Comparison condition of AN007"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha07(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN007"]
    #[inline(always)]
    pub const fn set_cmplcha07(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Comparison condition of AN008"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha08(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN008"]
    #[inline(always)]
    pub const fn set_cmplcha08(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Comparison condition of AN009"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha09(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN009"]
    #[inline(always)]
    pub const fn set_cmplcha09(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Comparison condition of AN010"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN010"]
    #[inline(always)]
    pub const fn set_cmplcha10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Comparison condition of AN011"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN011"]
    #[inline(always)]
    pub const fn set_cmplcha11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Comparison condition of AN012"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN012"]
    #[inline(always)]
    pub const fn set_cmplcha12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Comparison condition of AN013"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN013"]
    #[inline(always)]
    pub const fn set_cmplcha13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Comparison condition of AN014"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN014"]
    #[inline(always)]
    pub const fn set_cmplcha14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Adcmplr0 {
    #[inline(always)]
    fn default() -> Adcmplr0 {
        Adcmplr0(0)
    }
}
impl core::fmt::Debug for Adcmplr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcmplr0")
            .field("cmplcha00", &self.cmplcha00())
            .field("cmplcha01", &self.cmplcha01())
            .field("cmplcha02", &self.cmplcha02())
            .field("cmplcha03", &self.cmplcha03())
            .field("cmplcha04", &self.cmplcha04())
            .field("cmplcha05", &self.cmplcha05())
            .field("cmplcha06", &self.cmplcha06())
            .field("cmplcha07", &self.cmplcha07())
            .field("cmplcha08", &self.cmplcha08())
            .field("cmplcha09", &self.cmplcha09())
            .field("cmplcha10", &self.cmplcha10())
            .field("cmplcha11", &self.cmplcha11())
            .field("cmplcha12", &self.cmplcha12())
            .field("cmplcha13", &self.cmplcha13())
            .field("cmplcha14", &self.cmplcha14())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcmplr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcmplr0 {{ cmplcha00: {=bool:?}, cmplcha01: {=bool:?}, cmplcha02: {=bool:?}, cmplcha03: {=bool:?}, cmplcha04: {=bool:?}, cmplcha05: {=bool:?}, cmplcha06: {=bool:?}, cmplcha07: {=bool:?}, cmplcha08: {=bool:?}, cmplcha09: {=bool:?}, cmplcha10: {=bool:?}, cmplcha11: {=bool:?}, cmplcha12: {=bool:?}, cmplcha13: {=bool:?}, cmplcha14: {=bool:?}, reserved: {=bool:?} }}",
            self.cmplcha00(),
            self.cmplcha01(),
            self.cmplcha02(),
            self.cmplcha03(),
            self.cmplcha04(),
            self.cmplcha05(),
            self.cmplcha06(),
            self.cmplcha07(),
            self.cmplcha08(),
            self.cmplcha09(),
            self.cmplcha10(),
            self.cmplcha11(),
            self.cmplcha12(),
            self.cmplcha13(),
            self.cmplcha14(),
            self.reserved()
        )
    }
}
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmplr1(pub u16);
impl Adcmplr1 {
    #[doc = "Comparison condition of AN016"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha16(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN016"]
    #[inline(always)]
    pub const fn set_cmplcha16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Comparison condition of AN017"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha17(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN017"]
    #[inline(always)]
    pub const fn set_cmplcha17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Comparison condition of AN018"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha18(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN018"]
    #[inline(always)]
    pub const fn set_cmplcha18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Comparison condition of AN019"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha19(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN019"]
    #[inline(always)]
    pub const fn set_cmplcha19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Comparison condition of AN020"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha20(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN020"]
    #[inline(always)]
    pub const fn set_cmplcha20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Comparison condition of AN021"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha21(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN021"]
    #[inline(always)]
    pub const fn set_cmplcha21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Comparison condition of AN022"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha22(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN022"]
    #[inline(always)]
    pub const fn set_cmplcha22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Comparison condition of AN023"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha23(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN023"]
    #[inline(always)]
    pub const fn set_cmplcha23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Comparison condition of AN024"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha24(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN024"]
    #[inline(always)]
    pub const fn set_cmplcha24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Comparison condition of AN025"]
    #[must_use]
    #[inline(always)]
    pub const fn cmplcha25(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison condition of AN025"]
    #[inline(always)]
    pub const fn set_cmplcha25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Adcmplr1 {
    #[inline(always)]
    fn default() -> Adcmplr1 {
        Adcmplr1(0)
    }
}
impl core::fmt::Debug for Adcmplr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcmplr1")
            .field("cmplcha16", &self.cmplcha16())
            .field("cmplcha17", &self.cmplcha17())
            .field("cmplcha18", &self.cmplcha18())
            .field("cmplcha19", &self.cmplcha19())
            .field("cmplcha20", &self.cmplcha20())
            .field("cmplcha21", &self.cmplcha21())
            .field("cmplcha22", &self.cmplcha22())
            .field("cmplcha23", &self.cmplcha23())
            .field("cmplcha24", &self.cmplcha24())
            .field("cmplcha25", &self.cmplcha25())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcmplr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcmplr1 {{ cmplcha16: {=bool:?}, cmplcha17: {=bool:?}, cmplcha18: {=bool:?}, cmplcha19: {=bool:?}, cmplcha20: {=bool:?}, cmplcha21: {=bool:?}, cmplcha22: {=bool:?}, cmplcha23: {=bool:?}, cmplcha24: {=bool:?}, cmplcha25: {=bool:?}, reserved: {=u8:?} }}",
            self.cmplcha16(),
            self.cmplcha17(),
            self.cmplcha18(),
            self.cmplcha19(),
            self.cmplcha20(),
            self.cmplcha21(),
            self.cmplcha22(),
            self.cmplcha23(),
            self.cmplcha24(),
            self.cmplcha25(),
            self.reserved()
        )
    }
}
#[doc = "A/D Compare Function Window A Extended Input Channel Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpser(pub u8);
impl Adcmpser {
    #[doc = "Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpsttsa(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
    #[inline(always)]
    pub const fn set_cmpsttsa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstoca(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
    #[inline(always)]
    pub const fn set_cmpstoca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for Adcmpser {
    #[inline(always)]
    fn default() -> Adcmpser {
        Adcmpser(0)
    }
}
impl core::fmt::Debug for Adcmpser {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcmpser")
            .field("cmpsttsa", &self.cmpsttsa())
            .field("cmpstoca", &self.cmpstoca())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcmpser {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcmpser {{ cmpsttsa: {=bool:?}, cmpstoca: {=bool:?}, reserved: {=u8:?} }}",
            self.cmpsttsa(),
            self.cmpstoca(),
            self.reserved()
        )
    }
}
#[doc = "A/D Compare Function Window A Channel Status Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpsr0(pub u16);
impl Adcmpsr0 {
    #[doc = "Compare window A flag of AN000"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha00(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN000"]
    #[inline(always)]
    pub const fn set_cmpstcha00(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Compare window A flag of AN001"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha01(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN001"]
    #[inline(always)]
    pub const fn set_cmpstcha01(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Compare window A flag of AN002"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha02(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN002"]
    #[inline(always)]
    pub const fn set_cmpstcha02(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Compare window A flag of AN003"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha03(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN003"]
    #[inline(always)]
    pub const fn set_cmpstcha03(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Compare window A flag of AN004"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha04(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN004"]
    #[inline(always)]
    pub const fn set_cmpstcha04(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Compare window A flag of AN005"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha05(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN005"]
    #[inline(always)]
    pub const fn set_cmpstcha05(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Compare window A flag of AN006"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha06(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN006"]
    #[inline(always)]
    pub const fn set_cmpstcha06(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Compare window A flag of AN007"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha07(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN007"]
    #[inline(always)]
    pub const fn set_cmpstcha07(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Compare window A flag of AN008"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha08(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN008"]
    #[inline(always)]
    pub const fn set_cmpstcha08(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Compare window A flag of AN009"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha09(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN009"]
    #[inline(always)]
    pub const fn set_cmpstcha09(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Compare window A flag of AN010"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN010"]
    #[inline(always)]
    pub const fn set_cmpstcha10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Compare window A flag of AN011"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN011"]
    #[inline(always)]
    pub const fn set_cmpstcha11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Compare window A flag of AN012"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN012"]
    #[inline(always)]
    pub const fn set_cmpstcha12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Compare window A flag of AN013"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN013"]
    #[inline(always)]
    pub const fn set_cmpstcha13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Compare window A flag of AN014"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN014"]
    #[inline(always)]
    pub const fn set_cmpstcha14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Adcmpsr0 {
    #[inline(always)]
    fn default() -> Adcmpsr0 {
        Adcmpsr0(0)
    }
}
impl core::fmt::Debug for Adcmpsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcmpsr0")
            .field("cmpstcha00", &self.cmpstcha00())
            .field("cmpstcha01", &self.cmpstcha01())
            .field("cmpstcha02", &self.cmpstcha02())
            .field("cmpstcha03", &self.cmpstcha03())
            .field("cmpstcha04", &self.cmpstcha04())
            .field("cmpstcha05", &self.cmpstcha05())
            .field("cmpstcha06", &self.cmpstcha06())
            .field("cmpstcha07", &self.cmpstcha07())
            .field("cmpstcha08", &self.cmpstcha08())
            .field("cmpstcha09", &self.cmpstcha09())
            .field("cmpstcha10", &self.cmpstcha10())
            .field("cmpstcha11", &self.cmpstcha11())
            .field("cmpstcha12", &self.cmpstcha12())
            .field("cmpstcha13", &self.cmpstcha13())
            .field("cmpstcha14", &self.cmpstcha14())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcmpsr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcmpsr0 {{ cmpstcha00: {=bool:?}, cmpstcha01: {=bool:?}, cmpstcha02: {=bool:?}, cmpstcha03: {=bool:?}, cmpstcha04: {=bool:?}, cmpstcha05: {=bool:?}, cmpstcha06: {=bool:?}, cmpstcha07: {=bool:?}, cmpstcha08: {=bool:?}, cmpstcha09: {=bool:?}, cmpstcha10: {=bool:?}, cmpstcha11: {=bool:?}, cmpstcha12: {=bool:?}, cmpstcha13: {=bool:?}, cmpstcha14: {=bool:?}, reserved: {=bool:?} }}",
            self.cmpstcha00(),
            self.cmpstcha01(),
            self.cmpstcha02(),
            self.cmpstcha03(),
            self.cmpstcha04(),
            self.cmpstcha05(),
            self.cmpstcha06(),
            self.cmpstcha07(),
            self.cmpstcha08(),
            self.cmpstcha09(),
            self.cmpstcha10(),
            self.cmpstcha11(),
            self.cmpstcha12(),
            self.cmpstcha13(),
            self.cmpstcha14(),
            self.reserved()
        )
    }
}
#[doc = "A/D Compare Function Window A Channel Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpsr1(pub u16);
impl Adcmpsr1 {
    #[doc = "Compare window A flag of AN016"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha16(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN016"]
    #[inline(always)]
    pub const fn set_cmpstcha16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Compare window A flag of AN017"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha17(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN017"]
    #[inline(always)]
    pub const fn set_cmpstcha17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Compare window A flag of AN018"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha18(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN018"]
    #[inline(always)]
    pub const fn set_cmpstcha18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Compare window A flag of AN019"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha19(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN019"]
    #[inline(always)]
    pub const fn set_cmpstcha19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Compare window A flag of AN020"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha20(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN020"]
    #[inline(always)]
    pub const fn set_cmpstcha20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Compare window A flag of AN021"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha21(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN021"]
    #[inline(always)]
    pub const fn set_cmpstcha21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Compare window A flag of AN022"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha22(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN022"]
    #[inline(always)]
    pub const fn set_cmpstcha22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Compare window A flag of AN023"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha23(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN023"]
    #[inline(always)]
    pub const fn set_cmpstcha23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Compare window A flag of AN024"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha24(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN024"]
    #[inline(always)]
    pub const fn set_cmpstcha24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Compare window A flag of AN025"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpstcha25(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Compare window A flag of AN025"]
    #[inline(always)]
    pub const fn set_cmpstcha25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Adcmpsr1 {
    #[inline(always)]
    fn default() -> Adcmpsr1 {
        Adcmpsr1(0)
    }
}
impl core::fmt::Debug for Adcmpsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcmpsr1")
            .field("cmpstcha16", &self.cmpstcha16())
            .field("cmpstcha17", &self.cmpstcha17())
            .field("cmpstcha18", &self.cmpstcha18())
            .field("cmpstcha19", &self.cmpstcha19())
            .field("cmpstcha20", &self.cmpstcha20())
            .field("cmpstcha21", &self.cmpstcha21())
            .field("cmpstcha22", &self.cmpstcha22())
            .field("cmpstcha23", &self.cmpstcha23())
            .field("cmpstcha24", &self.cmpstcha24())
            .field("cmpstcha25", &self.cmpstcha25())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcmpsr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcmpsr1 {{ cmpstcha16: {=bool:?}, cmpstcha17: {=bool:?}, cmpstcha18: {=bool:?}, cmpstcha19: {=bool:?}, cmpstcha20: {=bool:?}, cmpstcha21: {=bool:?}, cmpstcha22: {=bool:?}, cmpstcha23: {=bool:?}, cmpstcha24: {=bool:?}, cmpstcha25: {=bool:?}, reserved: {=u8:?} }}",
            self.cmpstcha16(),
            self.cmpstcha17(),
            self.cmpstcha18(),
            self.cmpstcha19(),
            self.cmpstcha20(),
            self.cmpstcha21(),
            self.cmpstcha22(),
            self.cmpstcha23(),
            self.cmpstcha24(),
            self.cmpstcha25(),
            self.reserved()
        )
    }
}
#[doc = "A/D Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcsr(pub u16);
impl Adcsr {
    #[doc = "Double Trigger Channel Select These bits select one analog input channel for double triggered operation. The setting is only effective while double trigger mode is selected."]
    #[must_use]
    #[inline(always)]
    pub const fn dblans(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Double Trigger Channel Select These bits select one analog input channel for double triggered operation. The setting is only effective while double trigger mode is selected."]
    #[inline(always)]
    pub const fn set_dblans(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
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
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Group B Scan End Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gbadie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Group B Scan End Interrupt Enable"]
    #[inline(always)]
    pub const fn set_gbadie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Double Trigger Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn dble(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Double Trigger Mode Select"]
    #[inline(always)]
    pub const fn set_dble(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Trigger Select"]
    #[must_use]
    #[inline(always)]
    pub const fn extrg(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Select"]
    #[inline(always)]
    pub const fn set_extrg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Trigger Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn trge(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Start Enable"]
    #[inline(always)]
    pub const fn set_trge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "A/D Conversion Operation Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adhsc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "A/D Conversion Operation Mode Select"]
    #[inline(always)]
    pub const fn set_adhsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
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
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u16) & 0x03) << 11usize);
    }
    #[doc = "Scan Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adcs(&self) -> super::vals::Adcs {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Adcs::from_bits(val as u8)
    }
    #[doc = "Scan Mode Select"]
    #[inline(always)]
    pub const fn set_adcs(&mut self, val: super::vals::Adcs) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u16) & 0x03) << 13usize);
    }
    #[doc = "A/D Conversion Start"]
    #[must_use]
    #[inline(always)]
    pub const fn adst(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "A/D Conversion Start"]
    #[inline(always)]
    pub const fn set_adst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Adcsr {
    #[inline(always)]
    fn default() -> Adcsr {
        Adcsr(0)
    }
}
impl core::fmt::Debug for Adcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcsr")
            .field("dblans", &self.dblans())
            .field("reserved", &self.reserved())
            .field("gbadie", &self.gbadie())
            .field("dble", &self.dble())
            .field("extrg", &self.extrg())
            .field("trge", &self.trge())
            .field("adhsc", &self.adhsc())
            .field("reserved_2", &self.reserved_2())
            .field("adcs", &self.adcs())
            .field("adst", &self.adst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcsr {{ dblans: {=u8:?}, reserved: {=bool:?}, gbadie: {=bool:?}, dble: {=bool:?}, extrg: {=bool:?}, trge: {=bool:?}, adhsc: {=bool:?}, reserved_2: {=u8:?}, adcs: {:?}, adst: {=bool:?} }}",
            self.dblans(),
            self.reserved(),
            self.gbadie(),
            self.dble(),
            self.extrg(),
            self.trge(),
            self.adhsc(),
            self.reserved_2(),
            self.adcs(),
            self.adst()
        )
    }
}
#[doc = "A/D Disconnection Detection Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addiscr(pub u8);
impl Addiscr {
    #[doc = "The charging time"]
    #[must_use]
    #[inline(always)]
    pub const fn adndis(&self) -> super::vals::Adndis {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Adndis::from_bits(val as u8)
    }
    #[doc = "The charging time"]
    #[inline(always)]
    pub const fn set_adndis(&mut self, val: super::vals::Adndis) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
    #[doc = "Selection of Precharge or Discharge"]
    #[must_use]
    #[inline(always)]
    pub const fn pchg(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of Precharge or Discharge"]
    #[inline(always)]
    pub const fn set_pchg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Addiscr {
    #[inline(always)]
    fn default() -> Addiscr {
        Addiscr(0)
    }
}
impl core::fmt::Debug for Addiscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Addiscr")
            .field("adndis", &self.adndis())
            .field("pchg", &self.pchg())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Addiscr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Addiscr {{ adndis: {:?}, pchg: {=bool:?}, reserved: {=u8:?} }}",
            self.adndis(),
            self.pchg(),
            self.reserved()
        )
    }
}
#[doc = "A/D Conversion Extended Input Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adexicr(pub u16);
impl Adexicr {
    #[doc = "Temperature Sensor Output A/D converted Value Addition/Average Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tssad(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Sensor Output A/D converted Value Addition/Average Mode Select"]
    #[inline(always)]
    pub const fn set_tssad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Internal Reference Voltage A/D converted Value Addition/Average Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ocsad(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Reference Voltage A/D converted Value Addition/Average Mode Select"]
    #[inline(always)]
    pub const fn set_ocsad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u16) & 0x3f) << 2usize);
    }
    #[doc = "Temperature Sensor Output A/D Conversion Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tssa(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Sensor Output A/D Conversion Select"]
    #[inline(always)]
    pub const fn set_tssa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Internal Reference Voltage A/D Conversion Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ocsa(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Reference Voltage A/D Conversion Select"]
    #[inline(always)]
    pub const fn set_ocsa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_5(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_6(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Adexicr {
    #[inline(always)]
    fn default() -> Adexicr {
        Adexicr(0)
    }
}
impl core::fmt::Debug for Adexicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adexicr")
            .field("tssad", &self.tssad())
            .field("ocsad", &self.ocsad())
            .field("reserved", &self.reserved())
            .field("tssa", &self.tssa())
            .field("ocsa", &self.ocsa())
            .field("reserved_2", &self.reserved_2())
            .field("reserved_3", &self.reserved_3())
            .field("reserved_4", &self.reserved_4())
            .field("reserved_5", &self.reserved_5())
            .field("reserved_6", &self.reserved_6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adexicr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adexicr {{ tssad: {=bool:?}, ocsad: {=bool:?}, reserved: {=u8:?}, tssa: {=bool:?}, ocsa: {=bool:?}, reserved_2: {=bool:?}, reserved_3: {=bool:?}, reserved_4: {=bool:?}, reserved_5: {=bool:?}, reserved_6: {=bool:?} }}",
            self.tssad(),
            self.ocsad(),
            self.reserved(),
            self.tssa(),
            self.ocsa(),
            self.reserved_2(),
            self.reserved_3(),
            self.reserved_4(),
            self.reserved_5(),
            self.reserved_6()
        )
    }
}
#[doc = "A/D Group Scan Priority Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adgspcr(pub u16);
impl Adgspcr {
    #[doc = "Group A priority control setting bit. Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\\[1:0\\] bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed."]
    #[must_use]
    #[inline(always)]
    pub const fn pgs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Group A priority control setting bit. Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\\[1:0\\] bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed."]
    #[inline(always)]
    pub const fn set_pgs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Group B Restart Setting (Enabled only when PGS = 1. Reserved when PGS = 0.)"]
    #[must_use]
    #[inline(always)]
    pub const fn gbrscn(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Group B Restart Setting (Enabled only when PGS = 1. Reserved when PGS = 0.)"]
    #[inline(always)]
    pub const fn set_gbrscn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u16) & 0x3f) << 2usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u16) & 0x3f) << 9usize);
    }
    #[doc = "Group B Single Scan Continuous Start (Enabled only when PGS = 1. Reserved when PGS = 0.) Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit."]
    #[must_use]
    #[inline(always)]
    pub const fn gbrp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Group B Single Scan Continuous Start (Enabled only when PGS = 1. Reserved when PGS = 0.) Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit."]
    #[inline(always)]
    pub const fn set_gbrp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Adgspcr {
    #[inline(always)]
    fn default() -> Adgspcr {
        Adgspcr(0)
    }
}
impl core::fmt::Debug for Adgspcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adgspcr")
            .field("pgs", &self.pgs())
            .field("gbrscn", &self.gbrscn())
            .field("reserved", &self.reserved())
            .field("reserved_2", &self.reserved_2())
            .field("reserved_3", &self.reserved_3())
            .field("gbrp", &self.gbrp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adgspcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adgspcr {{ pgs: {=bool:?}, gbrscn: {=bool:?}, reserved: {=u8:?}, reserved_2: {=bool:?}, reserved_3: {=u8:?}, gbrp: {=bool:?} }}",
            self.pgs(),
            self.gbrscn(),
            self.reserved(),
            self.reserved_2(),
            self.reserved_3(),
            self.gbrp()
        )
    }
}
#[doc = "A/D High-Potential/Low-Potential Reference Voltage Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adhvrefcnt(pub u8);
impl Adhvrefcnt {
    #[doc = "High-Potential Reference Voltage Select"]
    #[must_use]
    #[inline(always)]
    pub const fn hvsel(&self) -> super::vals::Hvsel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Hvsel::from_bits(val as u8)
    }
    #[doc = "High-Potential Reference Voltage Select"]
    #[inline(always)]
    pub const fn set_hvsel(&mut self, val: super::vals::Hvsel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "Low-Potential Reference Voltage Select"]
    #[must_use]
    #[inline(always)]
    pub const fn lvsel(&self) -> super::vals::Lvsel {
        let val = (self.0 >> 2usize) & 0x07;
        super::vals::Lvsel::from_bits(val as u8)
    }
    #[doc = "Low-Potential Reference Voltage Select"]
    #[inline(always)]
    pub const fn set_lvsel(&mut self, val: super::vals::Lvsel) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u8) & 0x07) << 2usize);
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
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u8) & 0x03) << 5usize);
    }
    #[doc = "Sleep"]
    #[must_use]
    #[inline(always)]
    pub const fn adslp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep"]
    #[inline(always)]
    pub const fn set_adslp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Adhvrefcnt {
    #[inline(always)]
    fn default() -> Adhvrefcnt {
        Adhvrefcnt(0)
    }
}
impl core::fmt::Debug for Adhvrefcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adhvrefcnt")
            .field("hvsel", &self.hvsel())
            .field("lvsel", &self.lvsel())
            .field("reserved", &self.reserved())
            .field("reserved_2", &self.reserved_2())
            .field("adslp", &self.adslp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adhvrefcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adhvrefcnt {{ hvsel: {:?}, lvsel: {:?}, reserved: {=u8:?}, reserved_2: {=u8:?}, adslp: {=bool:?} }}",
            self.hvsel(),
            self.lvsel(),
            self.reserved(),
            self.reserved_2(),
            self.adslp()
        )
    }
}
#[doc = "A/D Self-Diagnosis Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adrd(pub u16);
impl Adrd {
    #[doc = "A/D-converted value (right-justified) The format for data determine ADCER.ADRFMT and ADCER.ADPRC."]
    #[must_use]
    #[inline(always)]
    pub const fn ad(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "A/D-converted value (right-justified) The format for data determine ADCER.ADRFMT and ADCER.ADPRC."]
    #[inline(always)]
    pub const fn set_ad(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u16) & 0x3fff) << 0usize);
    }
    #[doc = "Self-Diagnosis Status"]
    #[must_use]
    #[inline(always)]
    pub const fn diagst(&self) -> super::vals::Diagst {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Diagst::from_bits(val as u8)
    }
    #[doc = "Self-Diagnosis Status"]
    #[inline(always)]
    pub const fn set_diagst(&mut self, val: super::vals::Diagst) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Adrd {
    #[inline(always)]
    fn default() -> Adrd {
        Adrd(0)
    }
}
impl core::fmt::Debug for Adrd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adrd")
            .field("ad", &self.ad())
            .field("diagst", &self.diagst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adrd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adrd {{ ad: {=u16:?}, diagst: {:?} }}",
            self.ad(),
            self.diagst()
        )
    }
}
#[doc = "A/D Conversion Start Trigger Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adstrgr(pub u16);
impl Adstrgr {
    #[doc = "A/D Conversion Start Trigger Select for Group B Select the A/D conversion start trigger for group B in group scan mode."]
    #[must_use]
    #[inline(always)]
    pub const fn trsb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "A/D Conversion Start Trigger Select for Group B Select the A/D conversion start trigger for group B in group scan mode."]
    #[inline(always)]
    pub const fn set_trsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
    }
    #[doc = "A/D Conversion Start Trigger Select Select the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
    #[must_use]
    #[inline(always)]
    pub const fn trsa(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "A/D Conversion Start Trigger Select Select the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
    #[inline(always)]
    pub const fn set_trsa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u16) & 0x3f) << 8usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
    }
}
impl Default for Adstrgr {
    #[inline(always)]
    fn default() -> Adstrgr {
        Adstrgr(0)
    }
}
impl core::fmt::Debug for Adstrgr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adstrgr")
            .field("trsb", &self.trsb())
            .field("reserved", &self.reserved())
            .field("trsa", &self.trsa())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adstrgr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adstrgr {{ trsb: {=u8:?}, reserved: {=u8:?}, trsa: {=u8:?}, reserved_2: {=u8:?} }}",
            self.trsb(),
            self.reserved(),
            self.trsa(),
            self.reserved_2()
        )
    }
}
#[doc = "A/D Compare Function Window A/B Status Monitor Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adwinmon(pub u8);
impl Adwinmon {
    #[doc = "Combination result monitor This bit indicates the combination result. This bit is valid when both window A operation and window B operation are enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn moncomb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Combination result monitor This bit indicates the combination result. This bit is valid when both window A operation and window B operation are enabled."]
    #[inline(always)]
    pub const fn set_moncomb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u8) & 0x07) << 1usize);
    }
    #[doc = "Comparison Result Monitor A"]
    #[must_use]
    #[inline(always)]
    pub const fn moncmpa(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result Monitor A"]
    #[inline(always)]
    pub const fn set_moncmpa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Comparison Result Monitor B"]
    #[must_use]
    #[inline(always)]
    pub const fn moncmpb(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result Monitor B"]
    #[inline(always)]
    pub const fn set_moncmpb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "These bits are read as 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
    }
}
impl Default for Adwinmon {
    #[inline(always)]
    fn default() -> Adwinmon {
        Adwinmon(0)
    }
}
impl core::fmt::Debug for Adwinmon {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adwinmon")
            .field("moncomb", &self.moncomb())
            .field("reserved", &self.reserved())
            .field("moncmpa", &self.moncmpa())
            .field("moncmpb", &self.moncmpb())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adwinmon {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adwinmon {{ moncomb: {=bool:?}, reserved: {=u8:?}, moncmpa: {=bool:?}, moncmpb: {=bool:?}, reserved_2: {=u8:?} }}",
            self.moncomb(),
            self.reserved(),
            self.moncmpa(),
            self.moncmpb(),
            self.reserved_2()
        )
    }
}
