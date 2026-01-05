#[doc = "ECC 1-Bit Error Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecc1sts(pub u8);
impl Ecc1sts {
    #[doc = "ECC 1-Bit Error Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ecc1err(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ECC 1-Bit Error Status"]
    #[inline(always)]
    pub const fn set_ecc1err(&mut self, val: bool) {
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
impl Default for Ecc1sts {
    #[inline(always)]
    fn default() -> Ecc1sts {
        Ecc1sts(0)
    }
}
impl core::fmt::Debug for Ecc1sts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ecc1sts")
            .field("ecc1err", &self.ecc1err())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecc1sts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ecc1sts {{ ecc1err: {=bool:?}, reserved: {=u8:?} }}",
            self.ecc1err(),
            self.reserved()
        )
    }
}
#[doc = "ECC 1-Bit Error Information Update Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecc1stsen(pub u8);
impl Ecc1stsen {
    #[doc = "ECC 1-Bit Error Information Update Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn e1stsen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ECC 1-Bit Error Information Update Enable"]
    #[inline(always)]
    pub const fn set_e1stsen(&mut self, val: bool) {
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
impl Default for Ecc1stsen {
    #[inline(always)]
    fn default() -> Ecc1stsen {
        Ecc1stsen(0)
    }
}
impl core::fmt::Debug for Ecc1stsen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ecc1stsen")
            .field("e1stsen", &self.e1stsen())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecc1stsen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ecc1stsen {{ e1stsen: {=bool:?}, reserved: {=u8:?} }}",
            self.e1stsen(),
            self.reserved()
        )
    }
}
#[doc = "ECC 2-Bit Error Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecc2sts(pub u8);
impl Ecc2sts {
    #[doc = "ECC 2-Bit Error Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ecc2err(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ECC 2-Bit Error Status"]
    #[inline(always)]
    pub const fn set_ecc2err(&mut self, val: bool) {
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
impl Default for Ecc2sts {
    #[inline(always)]
    fn default() -> Ecc2sts {
        Ecc2sts(0)
    }
}
impl core::fmt::Debug for Ecc2sts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ecc2sts")
            .field("ecc2err", &self.ecc2err())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecc2sts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ecc2sts {{ ecc2err: {=bool:?}, reserved: {=u8:?} }}",
            self.ecc2err(),
            self.reserved()
        )
    }
}
#[doc = "ECC Test Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccetst(pub u8);
impl Eccetst {
    #[doc = "ECC Bypass Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tstbyp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ECC Bypass Select"]
    #[inline(always)]
    pub const fn set_tstbyp(&mut self, val: bool) {
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
impl Default for Eccetst {
    #[inline(always)]
    fn default() -> Eccetst {
        Eccetst(0)
    }
}
impl core::fmt::Debug for Eccetst {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eccetst")
            .field("tstbyp", &self.tstbyp())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eccetst {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eccetst {{ tstbyp: {=bool:?}, reserved: {=u8:?} }}",
            self.tstbyp(),
            self.reserved()
        )
    }
}
#[doc = "ECC Operating Mode Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccmode(pub u8);
impl Eccmode {
    #[doc = "ECC Operating Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn eccmod(&self) -> super::vals::Eccmod {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Eccmod::from_bits(val as u8)
    }
    #[doc = "ECC Operating Mode Select"]
    #[inline(always)]
    pub const fn set_eccmod(&mut self, val: super::vals::Eccmod) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
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
impl Default for Eccmode {
    #[inline(always)]
    fn default() -> Eccmode {
        Eccmode(0)
    }
}
impl core::fmt::Debug for Eccmode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eccmode")
            .field("eccmod", &self.eccmod())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eccmode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eccmode {{ eccmod: {:?}, reserved: {=u8:?} }}",
            self.eccmod(),
            self.reserved()
        )
    }
}
#[doc = "SRAM ECC Error Operation After Detection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccoad(pub u8);
impl Eccoad {
    #[doc = "Operation after Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn oad(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Operation after Detection"]
    #[inline(always)]
    pub const fn set_oad(&mut self, val: bool) {
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
impl Default for Eccoad {
    #[inline(always)]
    fn default() -> Eccoad {
        Eccoad(0)
    }
}
impl core::fmt::Debug for Eccoad {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eccoad")
            .field("oad", &self.oad())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eccoad {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eccoad {{ oad: {=bool:?}, reserved: {=u8:?} }}",
            self.oad(),
            self.reserved()
        )
    }
}
#[doc = "ECC Protection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccprcr(pub u8);
impl Eccprcr {
    #[doc = "Register Write Control"]
    #[must_use]
    #[inline(always)]
    pub const fn eccprcr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Register Write Control"]
    #[inline(always)]
    pub const fn set_eccprcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Write Key Code"]
    #[must_use]
    #[inline(always)]
    pub const fn kw(&self) -> super::vals::EccprcrKw {
        let val = (self.0 >> 1usize) & 0x7f;
        super::vals::EccprcrKw::from_bits(val as u8)
    }
    #[doc = "Write Key Code"]
    #[inline(always)]
    pub const fn set_kw(&mut self, val: super::vals::EccprcrKw) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val.to_bits() as u8) & 0x7f) << 1usize);
    }
}
impl Default for Eccprcr {
    #[inline(always)]
    fn default() -> Eccprcr {
        Eccprcr(0)
    }
}
impl core::fmt::Debug for Eccprcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eccprcr")
            .field("eccprcr", &self.eccprcr())
            .field("kw", &self.kw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eccprcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eccprcr {{ eccprcr: {=bool:?}, kw: {:?} }}",
            self.eccprcr(),
            self.kw()
        )
    }
}
#[doc = "ECC Protection Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccprcr2(pub u8);
impl Eccprcr2 {
    #[doc = "Register Write Control"]
    #[must_use]
    #[inline(always)]
    pub const fn eccprcr2(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Register Write Control"]
    #[inline(always)]
    pub const fn set_eccprcr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Write Key Code"]
    #[must_use]
    #[inline(always)]
    pub const fn kw2(&self) -> super::vals::Kw2 {
        let val = (self.0 >> 1usize) & 0x7f;
        super::vals::Kw2::from_bits(val as u8)
    }
    #[doc = "Write Key Code"]
    #[inline(always)]
    pub const fn set_kw2(&mut self, val: super::vals::Kw2) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val.to_bits() as u8) & 0x7f) << 1usize);
    }
}
impl Default for Eccprcr2 {
    #[inline(always)]
    fn default() -> Eccprcr2 {
        Eccprcr2(0)
    }
}
impl core::fmt::Debug for Eccprcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eccprcr2")
            .field("eccprcr2", &self.eccprcr2())
            .field("kw2", &self.kw2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eccprcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eccprcr2 {{ eccprcr2: {=bool:?}, kw2: {:?} }}",
            self.eccprcr2(),
            self.kw2()
        )
    }
}
#[doc = "SRAM Parity Error Operation After Detection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Parioad(pub u8);
impl Parioad {
    #[doc = "Operation after Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn oad(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Operation after Detection"]
    #[inline(always)]
    pub const fn set_oad(&mut self, val: bool) {
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
impl Default for Parioad {
    #[inline(always)]
    fn default() -> Parioad {
        Parioad(0)
    }
}
impl core::fmt::Debug for Parioad {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Parioad")
            .field("oad", &self.oad())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Parioad {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Parioad {{ oad: {=bool:?}, reserved: {=u8:?} }}",
            self.oad(),
            self.reserved()
        )
    }
}
#[doc = "SRAM Protection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramprcr(pub u8);
impl Sramprcr {
    #[doc = "Register Write Control"]
    #[must_use]
    #[inline(always)]
    pub const fn sramprcr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Register Write Control"]
    #[inline(always)]
    pub const fn set_sramprcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Write Key Code"]
    #[must_use]
    #[inline(always)]
    pub const fn kw(&self) -> super::vals::SramprcrKw {
        let val = (self.0 >> 1usize) & 0x7f;
        super::vals::SramprcrKw::from_bits(val as u8)
    }
    #[doc = "Write Key Code"]
    #[inline(always)]
    pub const fn set_kw(&mut self, val: super::vals::SramprcrKw) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val.to_bits() as u8) & 0x7f) << 1usize);
    }
}
impl Default for Sramprcr {
    #[inline(always)]
    fn default() -> Sramprcr {
        Sramprcr(0)
    }
}
impl core::fmt::Debug for Sramprcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramprcr")
            .field("sramprcr", &self.sramprcr())
            .field("kw", &self.kw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramprcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sramprcr {{ sramprcr: {=bool:?}, kw: {:?} }}",
            self.sramprcr(),
            self.kw()
        )
    }
}
