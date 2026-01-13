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
#[doc = "CRC Data Input Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcdir(pub u32);
impl Crcdir {
    #[doc = "Calculation input Data (Case of CRC-32, CRC-32C )"]
    #[must_use]
    #[inline(always)]
    pub const fn crcdir(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Calculation input Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    pub const fn set_crcdir(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Crcdir {
    #[inline(always)]
    fn default() -> Crcdir {
        Crcdir(0)
    }
}
impl core::fmt::Debug for Crcdir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crcdir")
            .field("crcdir", &self.crcdir())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crcdir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Crcdir {{ crcdir: {=u32:?} }}", self.crcdir())
    }
}
#[doc = "CRC Data Input Register (byte access)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrcdirBy(pub u8);
impl CrcdirBy {
    #[doc = "Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
    #[must_use]
    #[inline(always)]
    pub const fn crcdir_by(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    pub const fn set_crcdir_by(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for CrcdirBy {
    #[inline(always)]
    fn default() -> CrcdirBy {
        CrcdirBy(0)
    }
}
impl core::fmt::Debug for CrcdirBy {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CrcdirBy")
            .field("crcdir_by", &self.crcdir_by())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CrcdirBy {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CrcdirBy {{ crcdir_by: {=u8:?} }}", self.crcdir_by())
    }
}
#[doc = "CRC Data Output Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcdor(pub u32);
impl Crcdor {
    #[doc = "Calculation output Data (Case of CRC-32, CRC-32C )"]
    #[must_use]
    #[inline(always)]
    pub const fn crcdor(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Calculation output Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    pub const fn set_crcdor(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Crcdor {
    #[inline(always)]
    fn default() -> Crcdor {
        Crcdor(0)
    }
}
impl core::fmt::Debug for Crcdor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crcdor")
            .field("crcdor", &self.crcdor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crcdor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Crcdor {{ crcdor: {=u32:?} }}", self.crcdor())
    }
}
#[doc = "CRC Data Output Register(byte access)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrcdorBy(pub u8);
impl CrcdorBy {
    #[doc = "Calculation output Data (Case of CRC-8 )"]
    #[must_use]
    #[inline(always)]
    pub const fn crcdor_by(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Calculation output Data (Case of CRC-8 )"]
    #[inline(always)]
    pub const fn set_crcdor_by(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for CrcdorBy {
    #[inline(always)]
    fn default() -> CrcdorBy {
        CrcdorBy(0)
    }
}
impl core::fmt::Debug for CrcdorBy {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CrcdorBy")
            .field("crcdor_by", &self.crcdor_by())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CrcdorBy {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CrcdorBy {{ crcdor_by: {=u8:?} }}", self.crcdor_by())
    }
}
#[doc = "CRC Data Output Register (halfword access)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrcdorHa(pub u16);
impl CrcdorHa {
    #[doc = "Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
    #[must_use]
    #[inline(always)]
    pub const fn crcdor_ha(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    pub const fn set_crcdor_ha(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for CrcdorHa {
    #[inline(always)]
    fn default() -> CrcdorHa {
        CrcdorHa(0)
    }
}
impl core::fmt::Debug for CrcdorHa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CrcdorHa")
            .field("crcdor_ha", &self.crcdor_ha())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CrcdorHa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CrcdorHa {{ crcdor_ha: {=u16:?} }}", self.crcdor_ha())
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
