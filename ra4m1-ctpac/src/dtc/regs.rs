#[doc = "DTC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtccr(pub u8);
impl Dtccr {
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "DTC Transfer Information Read Skip Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rrs(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DTC Transfer Information Read Skip Enable."]
    #[inline(always)]
    pub const fn set_rrs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Dtccr {
    #[inline(always)]
    fn default() -> Dtccr {
        Dtccr(0)
    }
}
impl core::fmt::Debug for Dtccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtccr")
            .field("reserved", &self.reserved())
            .field("reserved_2", &self.reserved_2())
            .field("rrs", &self.rrs())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dtccr {{ reserved: {=u8:?}, reserved_2: {=bool:?}, rrs: {=bool:?}, reserved_3: {=u8:?} }}",
            self.reserved(),
            self.reserved_2(),
            self.rrs(),
            self.reserved_3()
        )
    }
}
#[doc = "DTC Module Start Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcst(pub u8);
impl Dtcst {
    #[doc = "DTC Module Start"]
    #[must_use]
    #[inline(always)]
    pub const fn dtcst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DTC Module Start"]
    #[inline(always)]
    pub const fn set_dtcst(&mut self, val: bool) {
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
impl Default for Dtcst {
    #[inline(always)]
    fn default() -> Dtcst {
        Dtcst(0)
    }
}
impl core::fmt::Debug for Dtcst {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtcst")
            .field("dtcst", &self.dtcst())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtcst {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dtcst {{ dtcst: {=bool:?}, reserved: {=u8:?} }}",
            self.dtcst(),
            self.reserved()
        )
    }
}
#[doc = "DTC Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcsts(pub u16);
impl Dtcsts {
    #[doc = "DTC-Activating Vector Number Monitoring These bits indicate the vector number for the activating source when DTC transfer is in progress. The value is only valid if DTC transfer is in progress (the value of the ACT flag is 1)"]
    #[must_use]
    #[inline(always)]
    pub const fn vecn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DTC-Activating Vector Number Monitoring These bits indicate the vector number for the activating source when DTC transfer is in progress. The value is only valid if DTC transfer is in progress (the value of the ACT flag is 1)"]
    #[inline(always)]
    pub const fn set_vecn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "These bits are read as 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
    #[doc = "DTC Active Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn act(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DTC Active Flag"]
    #[inline(always)]
    pub const fn set_act(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Dtcsts {
    #[inline(always)]
    fn default() -> Dtcsts {
        Dtcsts(0)
    }
}
impl core::fmt::Debug for Dtcsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtcsts")
            .field("vecn", &self.vecn())
            .field("reserved", &self.reserved())
            .field("act", &self.act())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtcsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dtcsts {{ vecn: {=u8:?}, reserved: {=u8:?}, act: {=bool:?} }}",
            self.vecn(),
            self.reserved(),
            self.act()
        )
    }
}
#[doc = "DTC Vector Base Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcvbr(pub u32);
impl Dtcvbr {
    #[doc = "DTC Vector Base Address. Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcvbr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "DTC Vector Base Address. Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
    #[inline(always)]
    pub const fn set_dtcvbr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dtcvbr {
    #[inline(always)]
    fn default() -> Dtcvbr {
        Dtcvbr(0)
    }
}
impl core::fmt::Debug for Dtcvbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtcvbr")
            .field("dtcvbr", &self.dtcvbr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtcvbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dtcvbr {{ dtcvbr: {=u32:?} }}", self.dtcvbr())
    }
}
