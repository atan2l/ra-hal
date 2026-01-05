#[doc = "Event Link Controller Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcr(pub u8);
impl Elcr {
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
    #[doc = "All Event Link Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn elcon(&self) -> super::vals::Elcon {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Elcon::from_bits(val as u8)
    }
    #[doc = "All Event Link Enable"]
    #[inline(always)]
    pub const fn set_elcon(&mut self, val: super::vals::Elcon) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Elcr {
    #[inline(always)]
    fn default() -> Elcr {
        Elcr(0)
    }
}
impl core::fmt::Debug for Elcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Elcr")
            .field("reserved", &self.reserved())
            .field("elcon", &self.elcon())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Elcr {{ reserved: {=u8:?}, elcon: {:?} }}",
            self.reserved(),
            self.elcon()
        )
    }
}
#[doc = "Event Link Software Event Generation Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsegr(pub u8);
impl Elsegr {
    #[doc = "Software Event Generation"]
    #[must_use]
    #[inline(always)]
    pub const fn seg(&self) -> super::vals::Seg {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Seg::from_bits(val as u8)
    }
    #[doc = "Software Event Generation"]
    #[inline(always)]
    pub const fn set_seg(&mut self, val: super::vals::Seg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u8) & 0x1f) << 1usize);
    }
    #[doc = "SEG Bit Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn we(&self) -> super::vals::We {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::We::from_bits(val as u8)
    }
    #[doc = "SEG Bit Write Enable"]
    #[inline(always)]
    pub const fn set_we(&mut self, val: super::vals::We) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "ELSEGR Register Write Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn wi(&self) -> super::vals::Wi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Wi::from_bits(val as u8)
    }
    #[doc = "ELSEGR Register Write Disable"]
    #[inline(always)]
    pub const fn set_wi(&mut self, val: super::vals::Wi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Elsegr {
    #[inline(always)]
    fn default() -> Elsegr {
        Elsegr(0)
    }
}
impl core::fmt::Debug for Elsegr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Elsegr")
            .field("seg", &self.seg())
            .field("reserved", &self.reserved())
            .field("we", &self.we())
            .field("wi", &self.wi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsegr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Elsegr {{ seg: {:?}, reserved: {=u8:?}, we: {:?}, wi: {:?} }}",
            self.seg(),
            self.reserved(),
            self.we(),
            self.wi()
        )
    }
}
#[doc = "Event Link Setting Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsr(pub u16);
impl Elsr {
    #[doc = "Event Link Select"]
    #[must_use]
    #[inline(always)]
    pub const fn els(&self) -> super::vals::ElsrEls {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::ElsrEls::from_bits(val as u8)
    }
    #[doc = "Event Link Select"]
    #[inline(always)]
    pub const fn set_els(&mut self, val: super::vals::ElsrEls) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u16) & 0xff) << 0usize);
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Elsr {
    #[inline(always)]
    fn default() -> Elsr {
        Elsr(0)
    }
}
impl core::fmt::Debug for Elsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Elsr")
            .field("els", &self.els())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Elsr {{ els: {:?}, reserved: {=u8:?} }}",
            self.els(),
            self.reserved()
        )
    }
}
#[doc = "Event Link Setting Register 12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsr12(pub u16);
impl Elsr12 {
    #[doc = "Event Link Select"]
    #[must_use]
    #[inline(always)]
    pub const fn els(&self) -> super::vals::Elsr12Els {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Elsr12Els::from_bits(val as u8)
    }
    #[doc = "Event Link Select"]
    #[inline(always)]
    pub const fn set_els(&mut self, val: super::vals::Elsr12Els) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u16) & 0xff) << 0usize);
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Elsr12 {
    #[inline(always)]
    fn default() -> Elsr12 {
        Elsr12(0)
    }
}
impl core::fmt::Debug for Elsr12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Elsr12")
            .field("els", &self.els())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsr12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Elsr12 {{ els: {:?}, reserved: {=u8:?} }}",
            self.els(),
            self.reserved()
        )
    }
}
#[doc = "Event Link Setting Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsr2(pub u16);
impl Elsr2 {
    #[doc = "Event Link Select"]
    #[must_use]
    #[inline(always)]
    pub const fn els(&self) -> super::vals::Elsr2Els {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Elsr2Els::from_bits(val as u8)
    }
    #[doc = "Event Link Select"]
    #[inline(always)]
    pub const fn set_els(&mut self, val: super::vals::Elsr2Els) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u16) & 0xff) << 0usize);
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Elsr2 {
    #[inline(always)]
    fn default() -> Elsr2 {
        Elsr2(0)
    }
}
impl core::fmt::Debug for Elsr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Elsr2")
            .field("els", &self.els())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Elsr2 {{ els: {:?}, reserved: {=u8:?} }}",
            self.els(),
            self.reserved()
        )
    }
}
