#[doc = "Group A Region %s Access Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuaca(pub u16);
impl Mmpuaca {
    #[doc = "Region enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Region enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Read protection"]
    #[must_use]
    #[inline(always)]
    pub const fn rp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub const fn set_rp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn wp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn set_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "These bits are read as 0000000000000. The write value should be 0000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x1fff;
        val as u16
    }
    #[doc = "These bits are read as 0000000000000. The write value should be 0000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u16) & 0x1fff) << 3usize);
    }
}
impl Default for Mmpuaca {
    #[inline(always)]
    fn default() -> Mmpuaca {
        Mmpuaca(0)
    }
}
impl core::fmt::Debug for Mmpuaca {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mmpuaca")
            .field("enable", &self.enable())
            .field("rp", &self.rp())
            .field("wp", &self.wp())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mmpuaca {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mmpuaca {{ enable: {=bool:?}, rp: {=bool:?}, wp: {=bool:?}, reserved: {=u16:?} }}",
            self.enable(),
            self.rp(),
            self.wp(),
            self.reserved()
        )
    }
}
#[doc = "Bus Master MPU Control Register A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuctla(pub u16);
impl Mmpuctla {
    #[doc = "Master Group enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Master Group enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Operation after detection"]
    #[must_use]
    #[inline(always)]
    pub const fn oad(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Operation after detection"]
    #[inline(always)]
    pub const fn set_oad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
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
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u16) & 0x3f) << 2usize);
    }
    #[doc = "Key Code These bits are used to enable or disable writing of the OAD and ENABLE bit."]
    #[must_use]
    #[inline(always)]
    pub const fn key(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Key Code These bits are used to enable or disable writing of the OAD and ENABLE bit."]
    #[inline(always)]
    pub const fn set_key(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Mmpuctla {
    #[inline(always)]
    fn default() -> Mmpuctla {
        Mmpuctla(0)
    }
}
impl core::fmt::Debug for Mmpuctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mmpuctla")
            .field("enable", &self.enable())
            .field("oad", &self.oad())
            .field("reserved", &self.reserved())
            .field("key", &self.key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mmpuctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mmpuctla {{ enable: {=bool:?}, oad: {=bool:?}, reserved: {=u8:?}, key: {=u8:?} }}",
            self.enable(),
            self.oad(),
            self.reserved(),
            self.key()
        )
    }
}
#[doc = "Group A Region %s End Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuea(pub u32);
impl Mmpuea {
    #[doc = "Region end address register Address where the region end, for use in region determination. NOTE: The low-order 2 bits are fixed to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn mmpuea(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Region end address register Address where the region end, for use in region determination. NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub const fn set_mmpuea(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mmpuea {
    #[inline(always)]
    fn default() -> Mmpuea {
        Mmpuea(0)
    }
}
impl core::fmt::Debug for Mmpuea {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mmpuea")
            .field("mmpuea", &self.mmpuea())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mmpuea {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mmpuea {{ mmpuea: {=u32:?} }}", self.mmpuea())
    }
}
#[doc = "Group A Protection of Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpupta(pub u16);
impl Mmpupta {
    #[doc = "Protection of register (MMPUSAn, MMPUEAn and MMPUACAn)"]
    #[must_use]
    #[inline(always)]
    pub const fn protect(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection of register (MMPUSAn, MMPUEAn and MMPUACAn)"]
    #[inline(always)]
    pub const fn set_protect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
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
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u16) & 0x7f) << 1usize);
    }
    #[doc = "Write Keyword The data written to these bits are not stored."]
    #[must_use]
    #[inline(always)]
    pub const fn key(&self) -> super::vals::Key {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Key::from_bits(val as u8)
    }
    #[doc = "Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    pub const fn set_key(&mut self, val: super::vals::Key) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
    }
}
impl Default for Mmpupta {
    #[inline(always)]
    fn default() -> Mmpupta {
        Mmpupta(0)
    }
}
impl core::fmt::Debug for Mmpupta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mmpupta")
            .field("protect", &self.protect())
            .field("reserved", &self.reserved())
            .field("key", &self.key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mmpupta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mmpupta {{ protect: {=bool:?}, reserved: {=u8:?}, key: {:?} }}",
            self.protect(),
            self.reserved(),
            self.key()
        )
    }
}
#[doc = "Group A Region %s Start Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusa(pub u32);
impl Mmpusa {
    #[doc = "Address where the region starts, for use in region determination. NOTE: The low-order 2 bits are fixed to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn mmpusa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address where the region starts, for use in region determination. NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub const fn set_mmpusa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mmpusa {
    #[inline(always)]
    fn default() -> Mmpusa {
        Mmpusa(0)
    }
}
impl core::fmt::Debug for Mmpusa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mmpusa")
            .field("mmpusa", &self.mmpusa())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mmpusa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mmpusa {{ mmpusa: {=u32:?} }}", self.mmpusa())
    }
}
