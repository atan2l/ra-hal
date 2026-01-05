#[doc = "CRC Control Register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crccr0(pub u8);
impl Crccr0 {
    #[doc = "CRC Generating Polynomial Switching"]
    #[must_use]
    #[inline(always)]
    pub const fn gps(&self) -> super::vals::Gps {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Gps::from_bits(val as u8)
    }
    #[doc = "CRC Generating Polynomial Switching"]
    #[inline(always)]
    pub const fn set_gps(&mut self, val: super::vals::Gps) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u8) & 0x07) << 0usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u8) & 0x07) << 3usize);
    }
    #[doc = "CRC Calculation Switching"]
    #[must_use]
    #[inline(always)]
    pub const fn lms(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Calculation Switching"]
    #[inline(always)]
    pub const fn set_lms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "CRCDOR Register Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn dorclr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CRCDOR Register Clear"]
    #[inline(always)]
    pub const fn set_dorclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Crccr0 {
    #[inline(always)]
    fn default() -> Crccr0 {
        Crccr0(0)
    }
}
impl core::fmt::Debug for Crccr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crccr0")
            .field("gps", &self.gps())
            .field("reserved", &self.reserved())
            .field("lms", &self.lms())
            .field("dorclr", &self.dorclr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crccr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Crccr0 {{ gps: {:?}, reserved: {=u8:?}, lms: {=bool:?}, dorclr: {=bool:?} }}",
            self.gps(),
            self.reserved(),
            self.lms(),
            self.dorclr()
        )
    }
}
#[doc = "CRC Control Register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crccr1(pub u8);
impl Crccr1 {
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
    #[doc = "Snoop-on-write/read switch bit"]
    #[must_use]
    #[inline(always)]
    pub const fn crcswr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Snoop-on-write/read switch bit"]
    #[inline(always)]
    pub const fn set_crcswr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Snoop enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn crcsen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Snoop enable bit"]
    #[inline(always)]
    pub const fn set_crcsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Crccr1 {
    #[inline(always)]
    fn default() -> Crccr1 {
        Crccr1(0)
    }
}
impl core::fmt::Debug for Crccr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crccr1")
            .field("reserved", &self.reserved())
            .field("crcswr", &self.crcswr())
            .field("crcsen", &self.crcsen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crccr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Crccr1 {{ reserved: {=u8:?}, crcswr: {=bool:?}, crcsen: {=bool:?} }}",
            self.reserved(),
            self.crcswr(),
            self.crcsen()
        )
    }
}
#[doc = "Snoop Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcsar(pub u16);
impl Crcsar {
    #[doc = "snoop address bit Set the I/O register address to snoop"]
    #[must_use]
    #[inline(always)]
    pub const fn crcsa(&self) -> super::vals::Crcsa {
        let val = (self.0 >> 0usize) & 0x3fff;
        super::vals::Crcsa::from_bits(val as u16)
    }
    #[doc = "snoop address bit Set the I/O register address to snoop"]
    #[inline(always)]
    pub const fn set_crcsa(&mut self, val: super::vals::Crcsa) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val.to_bits() as u16) & 0x3fff) << 0usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
    }
}
impl Default for Crcsar {
    #[inline(always)]
    fn default() -> Crcsar {
        Crcsar(0)
    }
}
impl core::fmt::Debug for Crcsar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crcsar")
            .field("crcsa", &self.crcsa())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crcsar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Crcsar {{ crcsa: {:?}, reserved: {=u8:?} }}",
            self.crcsa(),
            self.reserved()
        )
    }
}
