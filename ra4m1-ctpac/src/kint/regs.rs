#[doc = "KEY Return Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krctl(pub u8);
impl Krctl {
    #[doc = "Detection Edge Selection (KRF0 to KRF7)"]
    #[must_use]
    #[inline(always)]
    pub const fn kreg(&self) -> super::vals::Kreg {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Kreg::from_bits(val as u8)
    }
    #[doc = "Detection Edge Selection (KRF0 to KRF7)"]
    #[inline(always)]
    pub const fn set_kreg(&mut self, val: super::vals::Kreg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u8) & 0x3f) << 1usize);
    }
    #[doc = "Usage of Key Interrupt Flags(KR0 to KR7)"]
    #[must_use]
    #[inline(always)]
    pub const fn krmd(&self) -> super::vals::Krmd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Krmd::from_bits(val as u8)
    }
    #[doc = "Usage of Key Interrupt Flags(KR0 to KR7)"]
    #[inline(always)]
    pub const fn set_krmd(&mut self, val: super::vals::Krmd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Krctl {
    #[inline(always)]
    fn default() -> Krctl {
        Krctl(0)
    }
}
impl core::fmt::Debug for Krctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Krctl")
            .field("kreg", &self.kreg())
            .field("reserved", &self.reserved())
            .field("krmd", &self.krmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Krctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Krctl {{ kreg: {:?}, reserved: {=u8:?}, krmd: {:?} }}",
            self.kreg(),
            self.reserved(),
            self.krmd()
        )
    }
}
#[doc = "KEY Return Flag Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krf(pub u8);
impl Krf {
    #[doc = "Key interrupt flag 0"]
    #[must_use]
    #[inline(always)]
    pub const fn krf0(&self) -> super::vals::Krf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Krf0::from_bits(val as u8)
    }
    #[doc = "Key interrupt flag 0"]
    #[inline(always)]
    pub const fn set_krf0(&mut self, val: super::vals::Krf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Key interrupt flag 1"]
    #[must_use]
    #[inline(always)]
    pub const fn krf1(&self) -> super::vals::Krf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Krf1::from_bits(val as u8)
    }
    #[doc = "Key interrupt flag 1"]
    #[inline(always)]
    pub const fn set_krf1(&mut self, val: super::vals::Krf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Key interrupt flag 2"]
    #[must_use]
    #[inline(always)]
    pub const fn krf2(&self) -> super::vals::Krf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Krf2::from_bits(val as u8)
    }
    #[doc = "Key interrupt flag 2"]
    #[inline(always)]
    pub const fn set_krf2(&mut self, val: super::vals::Krf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Key interrupt flag 3"]
    #[must_use]
    #[inline(always)]
    pub const fn krf3(&self) -> super::vals::Krf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Krf3::from_bits(val as u8)
    }
    #[doc = "Key interrupt flag 3"]
    #[inline(always)]
    pub const fn set_krf3(&mut self, val: super::vals::Krf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Key interrupt flag 4"]
    #[must_use]
    #[inline(always)]
    pub const fn krf4(&self) -> super::vals::Krf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Krf4::from_bits(val as u8)
    }
    #[doc = "Key interrupt flag 4"]
    #[inline(always)]
    pub const fn set_krf4(&mut self, val: super::vals::Krf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Key interrupt flag 5"]
    #[must_use]
    #[inline(always)]
    pub const fn krf5(&self) -> super::vals::Krf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Krf5::from_bits(val as u8)
    }
    #[doc = "Key interrupt flag 5"]
    #[inline(always)]
    pub const fn set_krf5(&mut self, val: super::vals::Krf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Key interrupt flag 6"]
    #[must_use]
    #[inline(always)]
    pub const fn krf6(&self) -> super::vals::Krf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Krf6::from_bits(val as u8)
    }
    #[doc = "Key interrupt flag 6"]
    #[inline(always)]
    pub const fn set_krf6(&mut self, val: super::vals::Krf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Key interrupt flag 7"]
    #[must_use]
    #[inline(always)]
    pub const fn krf7(&self) -> super::vals::Krf7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Krf7::from_bits(val as u8)
    }
    #[doc = "Key interrupt flag 7"]
    #[inline(always)]
    pub const fn set_krf7(&mut self, val: super::vals::Krf7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Krf {
    #[inline(always)]
    fn default() -> Krf {
        Krf(0)
    }
}
impl core::fmt::Debug for Krf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Krf")
            .field("krf0", &self.krf0())
            .field("krf1", &self.krf1())
            .field("krf2", &self.krf2())
            .field("krf3", &self.krf3())
            .field("krf4", &self.krf4())
            .field("krf5", &self.krf5())
            .field("krf6", &self.krf6())
            .field("krf7", &self.krf7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Krf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Krf {{ krf0: {:?}, krf1: {:?}, krf2: {:?}, krf3: {:?}, krf4: {:?}, krf5: {:?}, krf6: {:?}, krf7: {:?} }}",
            self.krf0(),
            self.krf1(),
            self.krf2(),
            self.krf3(),
            self.krf4(),
            self.krf5(),
            self.krf6(),
            self.krf7()
        )
    }
}
#[doc = "KEY Return Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krm(pub u8);
impl Krm {
    #[doc = "Key interrupt mode control 0"]
    #[must_use]
    #[inline(always)]
    pub const fn krm0(&self) -> super::vals::Krm0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Krm0::from_bits(val as u8)
    }
    #[doc = "Key interrupt mode control 0"]
    #[inline(always)]
    pub const fn set_krm0(&mut self, val: super::vals::Krm0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Key interrupt mode control 1"]
    #[must_use]
    #[inline(always)]
    pub const fn krm1(&self) -> super::vals::Krm1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Krm1::from_bits(val as u8)
    }
    #[doc = "Key interrupt mode control 1"]
    #[inline(always)]
    pub const fn set_krm1(&mut self, val: super::vals::Krm1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Key interrupt mode control 2"]
    #[must_use]
    #[inline(always)]
    pub const fn krm2(&self) -> super::vals::Krm2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Krm2::from_bits(val as u8)
    }
    #[doc = "Key interrupt mode control 2"]
    #[inline(always)]
    pub const fn set_krm2(&mut self, val: super::vals::Krm2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Key interrupt mode control 3"]
    #[must_use]
    #[inline(always)]
    pub const fn krm3(&self) -> super::vals::Krm3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Krm3::from_bits(val as u8)
    }
    #[doc = "Key interrupt mode control 3"]
    #[inline(always)]
    pub const fn set_krm3(&mut self, val: super::vals::Krm3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Key interrupt mode control 4"]
    #[must_use]
    #[inline(always)]
    pub const fn krm4(&self) -> super::vals::Krm4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Krm4::from_bits(val as u8)
    }
    #[doc = "Key interrupt mode control 4"]
    #[inline(always)]
    pub const fn set_krm4(&mut self, val: super::vals::Krm4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Key interrupt mode control 5"]
    #[must_use]
    #[inline(always)]
    pub const fn krm5(&self) -> super::vals::Krm5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Krm5::from_bits(val as u8)
    }
    #[doc = "Key interrupt mode control 5"]
    #[inline(always)]
    pub const fn set_krm5(&mut self, val: super::vals::Krm5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Key interrupt mode control 6"]
    #[must_use]
    #[inline(always)]
    pub const fn krm6(&self) -> super::vals::Krm6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Krm6::from_bits(val as u8)
    }
    #[doc = "Key interrupt mode control 6"]
    #[inline(always)]
    pub const fn set_krm6(&mut self, val: super::vals::Krm6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Key interrupt mode control 7"]
    #[must_use]
    #[inline(always)]
    pub const fn krm7(&self) -> super::vals::Krm7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Krm7::from_bits(val as u8)
    }
    #[doc = "Key interrupt mode control 7"]
    #[inline(always)]
    pub const fn set_krm7(&mut self, val: super::vals::Krm7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Krm {
    #[inline(always)]
    fn default() -> Krm {
        Krm(0)
    }
}
impl core::fmt::Debug for Krm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Krm")
            .field("krm0", &self.krm0())
            .field("krm1", &self.krm1())
            .field("krm2", &self.krm2())
            .field("krm3", &self.krm3())
            .field("krm4", &self.krm4())
            .field("krm5", &self.krm5())
            .field("krm6", &self.krm6())
            .field("krm7", &self.krm7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Krm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Krm {{ krm0: {:?}, krm1: {:?}, krm2: {:?}, krm3: {:?}, krm4: {:?}, krm5: {:?}, krm6: {:?}, krm7: {:?} }}",
            self.krm0(),
            self.krm1(),
            self.krm2(),
            self.krm3(),
            self.krm4(),
            self.krm5(),
            self.krm6(),
            self.krm7()
        )
    }
}
