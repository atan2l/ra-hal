#[doc = "Port Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcntr1(pub u32);
impl Pcntr1 {
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pdr(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_podr(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pcntr1 {
    #[inline(always)]
    fn default() -> Pcntr1 {
        Pcntr1(0)
    }
}
impl core::fmt::Debug for Pcntr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcntr1")
            .field("pdr[0]", &self.pdr(0usize))
            .field("pdr[1]", &self.pdr(1usize))
            .field("pdr[2]", &self.pdr(2usize))
            .field("pdr[3]", &self.pdr(3usize))
            .field("pdr[4]", &self.pdr(4usize))
            .field("pdr[5]", &self.pdr(5usize))
            .field("pdr[6]", &self.pdr(6usize))
            .field("pdr[7]", &self.pdr(7usize))
            .field("pdr[8]", &self.pdr(8usize))
            .field("pdr[9]", &self.pdr(9usize))
            .field("pdr[10]", &self.pdr(10usize))
            .field("pdr[11]", &self.pdr(11usize))
            .field("pdr[12]", &self.pdr(12usize))
            .field("pdr[13]", &self.pdr(13usize))
            .field("pdr[14]", &self.pdr(14usize))
            .field("pdr[15]", &self.pdr(15usize))
            .field("podr[0]", &self.podr(0usize))
            .field("podr[1]", &self.podr(1usize))
            .field("podr[2]", &self.podr(2usize))
            .field("podr[3]", &self.podr(3usize))
            .field("podr[4]", &self.podr(4usize))
            .field("podr[5]", &self.podr(5usize))
            .field("podr[6]", &self.podr(6usize))
            .field("podr[7]", &self.podr(7usize))
            .field("podr[8]", &self.podr(8usize))
            .field("podr[9]", &self.podr(9usize))
            .field("podr[10]", &self.podr(10usize))
            .field("podr[11]", &self.podr(11usize))
            .field("podr[12]", &self.podr(12usize))
            .field("podr[13]", &self.podr(13usize))
            .field("podr[14]", &self.podr(14usize))
            .field("podr[15]", &self.podr(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcntr1 {{ pdr[0]: {=bool:?}, pdr[1]: {=bool:?}, pdr[2]: {=bool:?}, pdr[3]: {=bool:?}, pdr[4]: {=bool:?}, pdr[5]: {=bool:?}, pdr[6]: {=bool:?}, pdr[7]: {=bool:?}, pdr[8]: {=bool:?}, pdr[9]: {=bool:?}, pdr[10]: {=bool:?}, pdr[11]: {=bool:?}, pdr[12]: {=bool:?}, pdr[13]: {=bool:?}, pdr[14]: {=bool:?}, pdr[15]: {=bool:?}, podr[0]: {=bool:?}, podr[1]: {=bool:?}, podr[2]: {=bool:?}, podr[3]: {=bool:?}, podr[4]: {=bool:?}, podr[5]: {=bool:?}, podr[6]: {=bool:?}, podr[7]: {=bool:?}, podr[8]: {=bool:?}, podr[9]: {=bool:?}, podr[10]: {=bool:?}, podr[11]: {=bool:?}, podr[12]: {=bool:?}, podr[13]: {=bool:?}, podr[14]: {=bool:?}, podr[15]: {=bool:?} }}",
            self.pdr(0usize),
            self.pdr(1usize),
            self.pdr(2usize),
            self.pdr(3usize),
            self.pdr(4usize),
            self.pdr(5usize),
            self.pdr(6usize),
            self.pdr(7usize),
            self.pdr(8usize),
            self.pdr(9usize),
            self.pdr(10usize),
            self.pdr(11usize),
            self.pdr(12usize),
            self.pdr(13usize),
            self.pdr(14usize),
            self.pdr(15usize),
            self.podr(0usize),
            self.podr(1usize),
            self.podr(2usize),
            self.podr(3usize),
            self.podr(4usize),
            self.podr(5usize),
            self.podr(6usize),
            self.podr(7usize),
            self.podr(8usize),
            self.podr(9usize),
            self.podr(10usize),
            self.podr(11usize),
            self.podr(12usize),
            self.podr(13usize),
            self.podr(14usize),
            self.podr(15usize)
        )
    }
}
#[doc = "Port Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcntr2(pub u32);
impl Pcntr2 {
    #[doc = "Pmn Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pmn Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "These bits are read as 0000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "These bits are read as 0000000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pcntr2 {
    #[inline(always)]
    fn default() -> Pcntr2 {
        Pcntr2(0)
    }
}
impl core::fmt::Debug for Pcntr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcntr2")
            .field("pidr[0]", &self.pidr(0usize))
            .field("pidr[1]", &self.pidr(1usize))
            .field("pidr[2]", &self.pidr(2usize))
            .field("pidr[3]", &self.pidr(3usize))
            .field("pidr[4]", &self.pidr(4usize))
            .field("pidr[5]", &self.pidr(5usize))
            .field("pidr[6]", &self.pidr(6usize))
            .field("pidr[7]", &self.pidr(7usize))
            .field("pidr[8]", &self.pidr(8usize))
            .field("pidr[9]", &self.pidr(9usize))
            .field("pidr[10]", &self.pidr(10usize))
            .field("pidr[11]", &self.pidr(11usize))
            .field("pidr[12]", &self.pidr(12usize))
            .field("pidr[13]", &self.pidr(13usize))
            .field("pidr[14]", &self.pidr(14usize))
            .field("pidr[15]", &self.pidr(15usize))
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcntr2 {{ pidr[0]: {=bool:?}, pidr[1]: {=bool:?}, pidr[2]: {=bool:?}, pidr[3]: {=bool:?}, pidr[4]: {=bool:?}, pidr[5]: {=bool:?}, pidr[6]: {=bool:?}, pidr[7]: {=bool:?}, pidr[8]: {=bool:?}, pidr[9]: {=bool:?}, pidr[10]: {=bool:?}, pidr[11]: {=bool:?}, pidr[12]: {=bool:?}, pidr[13]: {=bool:?}, pidr[14]: {=bool:?}, pidr[15]: {=bool:?}, reserved: {=u16:?} }}",
            self.pidr(0usize),
            self.pidr(1usize),
            self.pidr(2usize),
            self.pidr(3usize),
            self.pidr(4usize),
            self.pidr(5usize),
            self.pidr(6usize),
            self.pidr(7usize),
            self.pidr(8usize),
            self.pidr(9usize),
            self.pidr(10usize),
            self.pidr(11usize),
            self.pidr(12usize),
            self.pidr(13usize),
            self.pidr(14usize),
            self.pidr(15usize),
            self.reserved()
        )
    }
}
#[doc = "Port Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcntr3(pub u32);
impl Pcntr3 {
    #[doc = "Pmn Output Set"]
    #[must_use]
    #[inline(always)]
    pub const fn posr(&self) -> super::vals::Pcntr3Posr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pcntr3Posr::from_bits(val as u16)
    }
    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub const fn set_posr(&mut self, val: super::vals::Pcntr3Posr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Pmn Output Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn porr(&self) -> super::vals::Pcntr3Porr {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pcntr3Porr::from_bits(val as u16)
    }
    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub const fn set_porr(&mut self, val: super::vals::Pcntr3Porr) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pcntr3 {
    #[inline(always)]
    fn default() -> Pcntr3 {
        Pcntr3(0)
    }
}
impl core::fmt::Debug for Pcntr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcntr3")
            .field("posr", &self.posr())
            .field("porr", &self.porr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcntr3 {{ posr: {:?}, porr: {:?} }}",
            self.posr(),
            self.porr()
        )
    }
}
#[doc = "Data direction register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr(pub u16);
impl Pdr {
    #[doc = "Pmn Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::PdrPdr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::PdrPdr::from_bits(val as u16)
    }
    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::PdrPdr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u16) & 0xffff) << 0usize);
    }
}
impl Default for Pdr {
    #[inline(always)]
    fn default() -> Pdr {
        Pdr(0)
    }
}
impl core::fmt::Debug for Pdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdr").field("pdr", &self.pdr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pdr {{ pdr: {:?} }}", self.pdr())
    }
}
#[doc = "Input data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr(pub u16);
impl Pidr {
    #[doc = "Pmn Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::PidrPidr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::PidrPidr::from_bits(val as u16)
    }
    #[doc = "Pmn Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::PidrPidr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u16) & 0xffff) << 0usize);
    }
}
impl Default for Pidr {
    #[inline(always)]
    fn default() -> Pidr {
        Pidr(0)
    }
}
impl core::fmt::Debug for Pidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pidr").field("pidr", &self.pidr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pidr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pidr {{ pidr: {:?} }}", self.pidr())
    }
}
#[doc = "Output data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Podr(pub u16);
impl Podr {
    #[doc = "Pmn Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> super::vals::PodrPodr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::PodrPodr::from_bits(val as u16)
    }
    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::PodrPodr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u16) & 0xffff) << 0usize);
    }
}
impl Default for Podr {
    #[inline(always)]
    fn default() -> Podr {
        Podr(0)
    }
}
impl core::fmt::Debug for Podr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Podr").field("podr", &self.podr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Podr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Podr {{ podr: {:?} }}", self.podr())
    }
}
#[doc = "Output reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Porr(pub u16);
impl Porr {
    #[doc = "Pmn Output Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn porr(&self) -> super::vals::PorrPorr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::PorrPorr::from_bits(val as u16)
    }
    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub const fn set_porr(&mut self, val: super::vals::PorrPorr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u16) & 0xffff) << 0usize);
    }
}
impl Default for Porr {
    #[inline(always)]
    fn default() -> Porr {
        Porr(0)
    }
}
impl core::fmt::Debug for Porr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Porr").field("porr", &self.porr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Porr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Porr {{ porr: {:?} }}", self.porr())
    }
}
#[doc = "Output set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posr(pub u16);
impl Posr {
    #[doc = "Pmn Output Set"]
    #[must_use]
    #[inline(always)]
    pub const fn posr(&self) -> super::vals::PosrPosr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::PosrPosr::from_bits(val as u16)
    }
    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub const fn set_posr(&mut self, val: super::vals::PosrPosr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u16) & 0xffff) << 0usize);
    }
}
impl Default for Posr {
    #[inline(always)]
    fn default() -> Posr {
        Posr(0)
    }
}
impl core::fmt::Debug for Posr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Posr").field("posr", &self.posr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Posr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Posr {{ posr: {:?} }}", self.posr())
    }
}
