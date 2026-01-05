#[doc = "Flash Cache Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcachee(pub u16);
impl Fcachee {
    #[doc = "FCACHE Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fcacheen(&self) -> super::vals::Fcacheen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Fcacheen::from_bits(val as u8)
    }
    #[doc = "FCACHE Enable"]
    #[inline(always)]
    pub const fn set_fcacheen(&mut self, val: super::vals::Fcacheen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
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
        self.0 = (self.0 & !(0x7fff << 1usize)) | (((val as u16) & 0x7fff) << 1usize);
    }
}
impl Default for Fcachee {
    #[inline(always)]
    fn default() -> Fcachee {
        Fcachee(0)
    }
}
impl core::fmt::Debug for Fcachee {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcachee")
            .field("fcacheen", &self.fcacheen())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcachee {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fcachee {{ fcacheen: {:?}, reserved: {=u16:?} }}",
            self.fcacheen(),
            self.reserved()
        )
    }
}
#[doc = "Flash Cache Invalidate Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcacheiv(pub u16);
impl Fcacheiv {
    #[doc = "FCACHE Invalidation"]
    #[must_use]
    #[inline(always)]
    pub const fn fcacheiv(&self) -> super::vals::Fcacheiv {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Fcacheiv::from_bits(val as u8)
    }
    #[doc = "FCACHE Invalidation"]
    #[inline(always)]
    pub const fn set_fcacheiv(&mut self, val: super::vals::Fcacheiv) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
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
        self.0 = (self.0 & !(0x7fff << 1usize)) | (((val as u16) & 0x7fff) << 1usize);
    }
}
impl Default for Fcacheiv {
    #[inline(always)]
    fn default() -> Fcacheiv {
        Fcacheiv(0)
    }
}
impl core::fmt::Debug for Fcacheiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcacheiv")
            .field("fcacheiv", &self.fcacheiv())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcacheiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fcacheiv {{ fcacheiv: {:?}, reserved: {=u16:?} }}",
            self.fcacheiv(),
            self.reserved()
        )
    }
}
#[doc = "Flash Wait Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flwt(pub u8);
impl Flwt {
    #[doc = "These bits represent the ratio of the CPU clock period to the Flash memory access time."]
    #[must_use]
    #[inline(always)]
    pub const fn flwt(&self) -> super::vals::Flwt {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Flwt::from_bits(val as u8)
    }
    #[doc = "These bits represent the ratio of the CPU clock period to the Flash memory access time."]
    #[inline(always)]
    pub const fn set_flwt(&mut self, val: super::vals::Flwt) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u8) & 0x07) << 0usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for Flwt {
    #[inline(always)]
    fn default() -> Flwt {
        Flwt(0)
    }
}
impl core::fmt::Debug for Flwt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flwt")
            .field("flwt", &self.flwt())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flwt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flwt {{ flwt: {:?}, reserved: {=u8:?} }}",
            self.flwt(),
            self.reserved()
        )
    }
}
