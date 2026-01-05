#[doc = "Event input data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eidr(pub u16);
impl Eidr {
    #[doc = "Pmn Event Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn eidr(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pmn Event Input Data"]
    #[inline(always)]
    pub const fn set_eidr(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
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
        f.debug_struct("Eidr")
            .field("eidr[0]", &self.eidr(0usize))
            .field("eidr[1]", &self.eidr(1usize))
            .field("eidr[2]", &self.eidr(2usize))
            .field("eidr[3]", &self.eidr(3usize))
            .field("eidr[4]", &self.eidr(4usize))
            .field("eidr[5]", &self.eidr(5usize))
            .field("eidr[6]", &self.eidr(6usize))
            .field("eidr[7]", &self.eidr(7usize))
            .field("eidr[8]", &self.eidr(8usize))
            .field("eidr[9]", &self.eidr(9usize))
            .field("eidr[10]", &self.eidr(10usize))
            .field("eidr[11]", &self.eidr(11usize))
            .field("eidr[12]", &self.eidr(12usize))
            .field("eidr[13]", &self.eidr(13usize))
            .field("eidr[14]", &self.eidr(14usize))
            .field("eidr[15]", &self.eidr(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eidr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eidr {{ eidr[0]: {=bool:?}, eidr[1]: {=bool:?}, eidr[2]: {=bool:?}, eidr[3]: {=bool:?}, eidr[4]: {=bool:?}, eidr[5]: {=bool:?}, eidr[6]: {=bool:?}, eidr[7]: {=bool:?}, eidr[8]: {=bool:?}, eidr[9]: {=bool:?}, eidr[10]: {=bool:?}, eidr[11]: {=bool:?}, eidr[12]: {=bool:?}, eidr[13]: {=bool:?}, eidr[14]: {=bool:?}, eidr[15]: {=bool:?} }}",
            self.eidr(0usize),
            self.eidr(1usize),
            self.eidr(2usize),
            self.eidr(3usize),
            self.eidr(4usize),
            self.eidr(5usize),
            self.eidr(6usize),
            self.eidr(7usize),
            self.eidr(8usize),
            self.eidr(9usize),
            self.eidr(10usize),
            self.eidr(11usize),
            self.eidr(12usize),
            self.eidr(13usize),
            self.eidr(14usize),
            self.eidr(15usize)
        )
    }
}
#[doc = "Event output set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eorr(pub u16);
impl Eorr {
    #[doc = "Pmn Event Output Reset Register (level=low)"]
    #[must_use]
    #[inline(always)]
    pub const fn eorr(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pmn Event Output Reset Register (level=low)"]
    #[inline(always)]
    pub const fn set_eorr(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
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
        f.debug_struct("Eorr")
            .field("eorr[0]", &self.eorr(0usize))
            .field("eorr[1]", &self.eorr(1usize))
            .field("eorr[2]", &self.eorr(2usize))
            .field("eorr[3]", &self.eorr(3usize))
            .field("eorr[4]", &self.eorr(4usize))
            .field("eorr[5]", &self.eorr(5usize))
            .field("eorr[6]", &self.eorr(6usize))
            .field("eorr[7]", &self.eorr(7usize))
            .field("eorr[8]", &self.eorr(8usize))
            .field("eorr[9]", &self.eorr(9usize))
            .field("eorr[10]", &self.eorr(10usize))
            .field("eorr[11]", &self.eorr(11usize))
            .field("eorr[12]", &self.eorr(12usize))
            .field("eorr[13]", &self.eorr(13usize))
            .field("eorr[14]", &self.eorr(14usize))
            .field("eorr[15]", &self.eorr(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eorr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eorr {{ eorr[0]: {=bool:?}, eorr[1]: {=bool:?}, eorr[2]: {=bool:?}, eorr[3]: {=bool:?}, eorr[4]: {=bool:?}, eorr[5]: {=bool:?}, eorr[6]: {=bool:?}, eorr[7]: {=bool:?}, eorr[8]: {=bool:?}, eorr[9]: {=bool:?}, eorr[10]: {=bool:?}, eorr[11]: {=bool:?}, eorr[12]: {=bool:?}, eorr[13]: {=bool:?}, eorr[14]: {=bool:?}, eorr[15]: {=bool:?} }}",
            self.eorr(0usize),
            self.eorr(1usize),
            self.eorr(2usize),
            self.eorr(3usize),
            self.eorr(4usize),
            self.eorr(5usize),
            self.eorr(6usize),
            self.eorr(7usize),
            self.eorr(8usize),
            self.eorr(9usize),
            self.eorr(10usize),
            self.eorr(11usize),
            self.eorr(12usize),
            self.eorr(13usize),
            self.eorr(14usize),
            self.eorr(15usize)
        )
    }
}
#[doc = "Event output reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eosr(pub u16);
impl Eosr {
    #[doc = "Pmn Event Output Set Register (level=high)"]
    #[must_use]
    #[inline(always)]
    pub const fn eosr(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pmn Event Output Set Register (level=high)"]
    #[inline(always)]
    pub const fn set_eosr(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
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
        f.debug_struct("Eosr")
            .field("eosr[0]", &self.eosr(0usize))
            .field("eosr[1]", &self.eosr(1usize))
            .field("eosr[2]", &self.eosr(2usize))
            .field("eosr[3]", &self.eosr(3usize))
            .field("eosr[4]", &self.eosr(4usize))
            .field("eosr[5]", &self.eosr(5usize))
            .field("eosr[6]", &self.eosr(6usize))
            .field("eosr[7]", &self.eosr(7usize))
            .field("eosr[8]", &self.eosr(8usize))
            .field("eosr[9]", &self.eosr(9usize))
            .field("eosr[10]", &self.eosr(10usize))
            .field("eosr[11]", &self.eosr(11usize))
            .field("eosr[12]", &self.eosr(12usize))
            .field("eosr[13]", &self.eosr(13usize))
            .field("eosr[14]", &self.eosr(14usize))
            .field("eosr[15]", &self.eosr(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eosr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eosr {{ eosr[0]: {=bool:?}, eosr[1]: {=bool:?}, eosr[2]: {=bool:?}, eosr[3]: {=bool:?}, eosr[4]: {=bool:?}, eosr[5]: {=bool:?}, eosr[6]: {=bool:?}, eosr[7]: {=bool:?}, eosr[8]: {=bool:?}, eosr[9]: {=bool:?}, eosr[10]: {=bool:?}, eosr[11]: {=bool:?}, eosr[12]: {=bool:?}, eosr[13]: {=bool:?}, eosr[14]: {=bool:?}, eosr[15]: {=bool:?} }}",
            self.eosr(0usize),
            self.eosr(1usize),
            self.eosr(2usize),
            self.eosr(3usize),
            self.eosr(4usize),
            self.eosr(5usize),
            self.eosr(6usize),
            self.eosr(7usize),
            self.eosr(8usize),
            self.eosr(9usize),
            self.eosr(10usize),
            self.eosr(11usize),
            self.eosr(12usize),
            self.eosr(13usize),
            self.eosr(14usize),
            self.eosr(15usize)
        )
    }
}
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcntr2 {{ pidr[0]: {=bool:?}, pidr[1]: {=bool:?}, pidr[2]: {=bool:?}, pidr[3]: {=bool:?}, pidr[4]: {=bool:?}, pidr[5]: {=bool:?}, pidr[6]: {=bool:?}, pidr[7]: {=bool:?}, pidr[8]: {=bool:?}, pidr[9]: {=bool:?}, pidr[10]: {=bool:?}, pidr[11]: {=bool:?}, pidr[12]: {=bool:?}, pidr[13]: {=bool:?}, pidr[14]: {=bool:?}, pidr[15]: {=bool:?} }}",
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
            self.pidr(15usize)
        )
    }
}
#[doc = "Port Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcntr3(pub u32);
impl Pcntr3 {
    #[doc = "Pmn Output Set (high)"]
    #[must_use]
    #[inline(always)]
    pub const fn posr(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pmn Output Set (high)"]
    #[inline(always)]
    pub const fn set_posr(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Pmn Output Reset (low)"]
    #[must_use]
    #[inline(always)]
    pub const fn porr(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pmn Output Reset (low)"]
    #[inline(always)]
    pub const fn set_porr(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("posr[0]", &self.posr(0usize))
            .field("posr[1]", &self.posr(1usize))
            .field("posr[2]", &self.posr(2usize))
            .field("posr[3]", &self.posr(3usize))
            .field("posr[4]", &self.posr(4usize))
            .field("posr[5]", &self.posr(5usize))
            .field("posr[6]", &self.posr(6usize))
            .field("posr[7]", &self.posr(7usize))
            .field("posr[8]", &self.posr(8usize))
            .field("posr[9]", &self.posr(9usize))
            .field("posr[10]", &self.posr(10usize))
            .field("posr[11]", &self.posr(11usize))
            .field("posr[12]", &self.posr(12usize))
            .field("posr[13]", &self.posr(13usize))
            .field("posr[14]", &self.posr(14usize))
            .field("posr[15]", &self.posr(15usize))
            .field("porr[0]", &self.porr(0usize))
            .field("porr[1]", &self.porr(1usize))
            .field("porr[2]", &self.porr(2usize))
            .field("porr[3]", &self.porr(3usize))
            .field("porr[4]", &self.porr(4usize))
            .field("porr[5]", &self.porr(5usize))
            .field("porr[6]", &self.porr(6usize))
            .field("porr[7]", &self.porr(7usize))
            .field("porr[8]", &self.porr(8usize))
            .field("porr[9]", &self.porr(9usize))
            .field("porr[10]", &self.porr(10usize))
            .field("porr[11]", &self.porr(11usize))
            .field("porr[12]", &self.porr(12usize))
            .field("porr[13]", &self.porr(13usize))
            .field("porr[14]", &self.porr(14usize))
            .field("porr[15]", &self.porr(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcntr3 {{ posr[0]: {=bool:?}, posr[1]: {=bool:?}, posr[2]: {=bool:?}, posr[3]: {=bool:?}, posr[4]: {=bool:?}, posr[5]: {=bool:?}, posr[6]: {=bool:?}, posr[7]: {=bool:?}, posr[8]: {=bool:?}, posr[9]: {=bool:?}, posr[10]: {=bool:?}, posr[11]: {=bool:?}, posr[12]: {=bool:?}, posr[13]: {=bool:?}, posr[14]: {=bool:?}, posr[15]: {=bool:?}, porr[0]: {=bool:?}, porr[1]: {=bool:?}, porr[2]: {=bool:?}, porr[3]: {=bool:?}, porr[4]: {=bool:?}, porr[5]: {=bool:?}, porr[6]: {=bool:?}, porr[7]: {=bool:?}, porr[8]: {=bool:?}, porr[9]: {=bool:?}, porr[10]: {=bool:?}, porr[11]: {=bool:?}, porr[12]: {=bool:?}, porr[13]: {=bool:?}, porr[14]: {=bool:?}, porr[15]: {=bool:?} }}",
            self.posr(0usize),
            self.posr(1usize),
            self.posr(2usize),
            self.posr(3usize),
            self.posr(4usize),
            self.posr(5usize),
            self.posr(6usize),
            self.posr(7usize),
            self.posr(8usize),
            self.posr(9usize),
            self.posr(10usize),
            self.posr(11usize),
            self.posr(12usize),
            self.posr(13usize),
            self.posr(14usize),
            self.posr(15usize),
            self.porr(0usize),
            self.porr(1usize),
            self.porr(2usize),
            self.porr(3usize),
            self.porr(4usize),
            self.porr(5usize),
            self.porr(6usize),
            self.porr(7usize),
            self.porr(8usize),
            self.porr(9usize),
            self.porr(10usize),
            self.porr(11usize),
            self.porr(12usize),
            self.porr(13usize),
            self.porr(14usize),
            self.porr(15usize)
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
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
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
        f.debug_struct("Pidr")
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pidr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pidr {{ pidr[0]: {=bool:?}, pidr[1]: {=bool:?}, pidr[2]: {=bool:?}, pidr[3]: {=bool:?}, pidr[4]: {=bool:?}, pidr[5]: {=bool:?}, pidr[6]: {=bool:?}, pidr[7]: {=bool:?}, pidr[8]: {=bool:?}, pidr[9]: {=bool:?}, pidr[10]: {=bool:?}, pidr[11]: {=bool:?}, pidr[12]: {=bool:?}, pidr[13]: {=bool:?}, pidr[14]: {=bool:?}, pidr[15]: {=bool:?} }}",
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
            self.pidr(15usize)
        )
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
    pub const fn podr(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
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
        f.debug_struct("Podr")
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
impl defmt::Format for Podr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Podr {{ podr[0]: {=bool:?}, podr[1]: {=bool:?}, podr[2]: {=bool:?}, podr[3]: {=bool:?}, podr[4]: {=bool:?}, podr[5]: {=bool:?}, podr[6]: {=bool:?}, podr[7]: {=bool:?}, podr[8]: {=bool:?}, podr[9]: {=bool:?}, podr[10]: {=bool:?}, podr[11]: {=bool:?}, podr[12]: {=bool:?}, podr[13]: {=bool:?}, podr[14]: {=bool:?}, podr[15]: {=bool:?} }}",
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
#[doc = "Output set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Porr(pub u16);
impl Porr {
    #[doc = "Pmn Output Reset Register (level=low)"]
    #[must_use]
    #[inline(always)]
    pub const fn porr(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pmn Output Reset Register (level=low)"]
    #[inline(always)]
    pub const fn set_porr(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
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
        f.debug_struct("Porr")
            .field("porr[0]", &self.porr(0usize))
            .field("porr[1]", &self.porr(1usize))
            .field("porr[2]", &self.porr(2usize))
            .field("porr[3]", &self.porr(3usize))
            .field("porr[4]", &self.porr(4usize))
            .field("porr[5]", &self.porr(5usize))
            .field("porr[6]", &self.porr(6usize))
            .field("porr[7]", &self.porr(7usize))
            .field("porr[8]", &self.porr(8usize))
            .field("porr[9]", &self.porr(9usize))
            .field("porr[10]", &self.porr(10usize))
            .field("porr[11]", &self.porr(11usize))
            .field("porr[12]", &self.porr(12usize))
            .field("porr[13]", &self.porr(13usize))
            .field("porr[14]", &self.porr(14usize))
            .field("porr[15]", &self.porr(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Porr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Porr {{ porr[0]: {=bool:?}, porr[1]: {=bool:?}, porr[2]: {=bool:?}, porr[3]: {=bool:?}, porr[4]: {=bool:?}, porr[5]: {=bool:?}, porr[6]: {=bool:?}, porr[7]: {=bool:?}, porr[8]: {=bool:?}, porr[9]: {=bool:?}, porr[10]: {=bool:?}, porr[11]: {=bool:?}, porr[12]: {=bool:?}, porr[13]: {=bool:?}, porr[14]: {=bool:?}, porr[15]: {=bool:?} }}",
            self.porr(0usize),
            self.porr(1usize),
            self.porr(2usize),
            self.porr(3usize),
            self.porr(4usize),
            self.porr(5usize),
            self.porr(6usize),
            self.porr(7usize),
            self.porr(8usize),
            self.porr(9usize),
            self.porr(10usize),
            self.porr(11usize),
            self.porr(12usize),
            self.porr(13usize),
            self.porr(14usize),
            self.porr(15usize)
        )
    }
}
#[doc = "Output reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posr(pub u16);
impl Posr {
    #[doc = "Pmn Output Set Register (level=high)"]
    #[must_use]
    #[inline(always)]
    pub const fn posr(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pmn Output Set Register (level=high)"]
    #[inline(always)]
    pub const fn set_posr(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
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
        f.debug_struct("Posr")
            .field("posr[0]", &self.posr(0usize))
            .field("posr[1]", &self.posr(1usize))
            .field("posr[2]", &self.posr(2usize))
            .field("posr[3]", &self.posr(3usize))
            .field("posr[4]", &self.posr(4usize))
            .field("posr[5]", &self.posr(5usize))
            .field("posr[6]", &self.posr(6usize))
            .field("posr[7]", &self.posr(7usize))
            .field("posr[8]", &self.posr(8usize))
            .field("posr[9]", &self.posr(9usize))
            .field("posr[10]", &self.posr(10usize))
            .field("posr[11]", &self.posr(11usize))
            .field("posr[12]", &self.posr(12usize))
            .field("posr[13]", &self.posr(13usize))
            .field("posr[14]", &self.posr(14usize))
            .field("posr[15]", &self.posr(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Posr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Posr {{ posr[0]: {=bool:?}, posr[1]: {=bool:?}, posr[2]: {=bool:?}, posr[3]: {=bool:?}, posr[4]: {=bool:?}, posr[5]: {=bool:?}, posr[6]: {=bool:?}, posr[7]: {=bool:?}, posr[8]: {=bool:?}, posr[9]: {=bool:?}, posr[10]: {=bool:?}, posr[11]: {=bool:?}, posr[12]: {=bool:?}, posr[13]: {=bool:?}, posr[14]: {=bool:?}, posr[15]: {=bool:?} }}",
            self.posr(0usize),
            self.posr(1usize),
            self.posr(2usize),
            self.posr(3usize),
            self.posr(4usize),
            self.posr(5usize),
            self.posr(6usize),
            self.posr(7usize),
            self.posr(8usize),
            self.posr(9usize),
            self.posr(10usize),
            self.posr(11usize),
            self.posr(12usize),
            self.posr(13usize),
            self.posr(14usize),
            self.posr(15usize)
        )
    }
}
