#[doc = "Event input data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eidr(pub u16);
impl Eidr {
    #[doc = "Pmn Event Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn eidr(&self) -> super::vals::EidrEidr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::EidrEidr::from_bits(val as u16)
    }
    #[doc = "Pmn Event Input Data"]
    #[inline(always)]
    pub const fn set_eidr(&mut self, val: super::vals::EidrEidr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u16) & 0xffff) << 0usize);
    }
}
impl Default for Eidr {
    #[inline(always)]
    fn default() -> Eidr {
        Eidr(0)
    }
}
impl core::fmt::Debug for Eidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eidr").field("eidr", &self.eidr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eidr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Eidr {{ eidr: {:?} }}", self.eidr())
    }
}
#[doc = "Event output set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eorr(pub u16);
impl Eorr {
    #[doc = "Pmn Event Output Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn eorr(&self) -> super::vals::EorrEorr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::EorrEorr::from_bits(val as u16)
    }
    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub const fn set_eorr(&mut self, val: super::vals::EorrEorr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u16) & 0xffff) << 0usize);
    }
}
impl Default for Eorr {
    #[inline(always)]
    fn default() -> Eorr {
        Eorr(0)
    }
}
impl core::fmt::Debug for Eorr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eorr").field("eorr", &self.eorr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eorr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Eorr {{ eorr: {:?} }}", self.eorr())
    }
}
#[doc = "Event output reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eosr(pub u16);
impl Eosr {
    #[doc = "Pmn Event Output Set"]
    #[must_use]
    #[inline(always)]
    pub const fn eosr(&self) -> super::vals::EosrEosr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::EosrEosr::from_bits(val as u16)
    }
    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub const fn set_eosr(&mut self, val: super::vals::EosrEosr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u16) & 0xffff) << 0usize);
    }
}
impl Default for Eosr {
    #[inline(always)]
    fn default() -> Eosr {
        Eosr(0)
    }
}
impl core::fmt::Debug for Eosr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eosr").field("eosr", &self.eosr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eosr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Eosr {{ eosr: {:?} }}", self.eosr())
    }
}
#[doc = "Port Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcntr1(pub u32);
impl Pcntr1 {
    #[doc = "Pmn Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::Pcntr1Pdr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pcntr1Pdr::from_bits(val as u16)
    }
    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::Pcntr1Pdr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Pmn Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> super::vals::Pcntr1Podr {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pcntr1Podr::from_bits(val as u16)
    }
    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::Pcntr1Podr) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
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
            .field("pdr", &self.pdr())
            .field("podr", &self.podr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcntr1 {{ pdr: {:?}, podr: {:?} }}",
            self.pdr(),
            self.podr()
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
    pub const fn pidr(&self) -> super::vals::Pcntr2Pidr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pcntr2Pidr::from_bits(val as u16)
    }
    #[doc = "Pmn Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::Pcntr2Pidr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Pmn Event Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn eidr(&self) -> super::vals::Pcntr2Eidr {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pcntr2Eidr::from_bits(val as u16)
    }
    #[doc = "Pmn Event Input Data"]
    #[inline(always)]
    pub const fn set_eidr(&mut self, val: super::vals::Pcntr2Eidr) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
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
            .field("pidr", &self.pidr())
            .field("eidr", &self.eidr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcntr2 {{ pidr: {:?}, eidr: {:?} }}",
            self.pidr(),
            self.eidr()
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
#[doc = "Port Control Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcntr4(pub u32);
impl Pcntr4 {
    #[doc = "Pmn Event Output Set"]
    #[must_use]
    #[inline(always)]
    pub const fn eosr(&self) -> super::vals::Pcntr4Eosr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pcntr4Eosr::from_bits(val as u16)
    }
    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub const fn set_eosr(&mut self, val: super::vals::Pcntr4Eosr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Pmn Event Output Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn eorr(&self) -> super::vals::Pcntr4Eorr {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pcntr4Eorr::from_bits(val as u16)
    }
    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub const fn set_eorr(&mut self, val: super::vals::Pcntr4Eorr) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pcntr4 {
    #[inline(always)]
    fn default() -> Pcntr4 {
        Pcntr4(0)
    }
}
impl core::fmt::Debug for Pcntr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcntr4")
            .field("eosr", &self.eosr())
            .field("eorr", &self.eorr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcntr4 {{ eosr: {:?}, eorr: {:?} }}",
            self.eosr(),
            self.eorr()
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
#[doc = "Output set register"]
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
#[doc = "Output reset register"]
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
