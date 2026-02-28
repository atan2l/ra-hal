#[doc = "D/A-A/D Synchronous Start Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Daadscr(pub u8);
impl Daadscr {
    #[doc = "D/A-A/D Synchronous Conversion"]
    #[must_use]
    #[inline(always)]
    pub const fn daadst(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "D/A-A/D Synchronous Conversion"]
    #[inline(always)]
    pub const fn set_daadst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Daadscr {
    #[inline(always)]
    fn default() -> Daadscr {
        Daadscr(0)
    }
}
impl core::fmt::Debug for Daadscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Daadscr")
            .field("daadst", &self.daadst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Daadscr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Daadscr {{ daadst: {=bool:?} }}", self.daadst())
    }
}
#[doc = "D/A Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacr(pub u8);
impl Dacr {
    #[doc = "These bits are read as 11111. The write value should be 11111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 11111. The write value should be 11111."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "D/A Output Enable 0"]
    #[must_use]
    #[inline(always)]
    pub const fn daoe0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "D/A Output Enable 0"]
    #[inline(always)]
    pub const fn set_daoe0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dacr {
    #[inline(always)]
    fn default() -> Dacr {
        Dacr(0)
    }
}
impl core::fmt::Debug for Dacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dacr")
            .field("reserved", &self.reserved())
            .field("reserved_2", &self.reserved_2())
            .field("daoe0", &self.daoe0())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dacr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dacr {{ reserved: {=u8:?}, reserved_2: {=bool:?}, daoe0: {=bool:?}, reserved_3: {=bool:?} }}",
            self.reserved(),
            self.reserved_2(),
            self.daoe0(),
            self.reserved_3()
        )
    }
}
#[doc = "DADR0 Format Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dadpr(pub u8);
impl Dadpr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
    }
    #[doc = "DADRm Format Select"]
    #[must_use]
    #[inline(always)]
    pub const fn dpsel(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DADRm Format Select"]
    #[inline(always)]
    pub const fn set_dpsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dadpr {
    #[inline(always)]
    fn default() -> Dadpr {
        Dadpr(0)
    }
}
impl core::fmt::Debug for Dadpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dadpr")
            .field("reserved", &self.reserved())
            .field("dpsel", &self.dpsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dadpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dadpr {{ reserved: {=u8:?}, dpsel: {=bool:?} }}",
            self.reserved(),
            self.dpsel()
        )
    }
}
#[doc = "D/A VREF Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Davrefcr(pub u8);
impl Davrefcr {
    #[doc = "D/A Reference Voltage Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> super::vals::Ref {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ref::from_bits(val as u8)
    }
    #[doc = "D/A Reference Voltage Select"]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: super::vals::Ref) {
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
impl Default for Davrefcr {
    #[inline(always)]
    fn default() -> Davrefcr {
        Davrefcr(0)
    }
}
impl core::fmt::Debug for Davrefcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Davrefcr")
            .field("ref_", &self.ref_())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Davrefcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Davrefcr {{ ref_: {:?}, reserved: {=u8:?} }}",
            self.ref_(),
            self.reserved()
        )
    }
}
