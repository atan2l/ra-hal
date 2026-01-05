#[doc = "Stack Pointer Monitor Access Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspmpuctl(pub u16);
impl Mspmpuctl {
    #[doc = "Stack Pointer Monitor Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::MspmpuctlEnable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MspmpuctlEnable::from_bits(val as u8)
    }
    #[doc = "Stack Pointer Monitor Enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: super::vals::MspmpuctlEnable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
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
    #[doc = "Stack Pointer Monitor Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> super::vals::MspmpuctlError {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::MspmpuctlError::from_bits(val as u8)
    }
    #[doc = "Stack Pointer Monitor Error Flag"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: super::vals::MspmpuctlError) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u16) & 0x7f) << 9usize);
    }
}
impl Default for Mspmpuctl {
    #[inline(always)]
    fn default() -> Mspmpuctl {
        Mspmpuctl(0)
    }
}
impl core::fmt::Debug for Mspmpuctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mspmpuctl")
            .field("enable", &self.enable())
            .field("reserved", &self.reserved())
            .field("error", &self.error())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mspmpuctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mspmpuctl {{ enable: {:?}, reserved: {=u8:?}, error: {:?}, reserved_2: {=u8:?} }}",
            self.enable(),
            self.reserved(),
            self.error(),
            self.reserved_2()
        )
    }
}
#[doc = "Main Stack Pointer (MSP) Monitor End Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspmpuea(pub u32);
impl Mspmpuea {
    #[doc = "Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn mspmpuea(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub const fn set_mspmpuea(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mspmpuea {
    #[inline(always)]
    fn default() -> Mspmpuea {
        Mspmpuea(0)
    }
}
impl core::fmt::Debug for Mspmpuea {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mspmpuea")
            .field("mspmpuea", &self.mspmpuea())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mspmpuea {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mspmpuea {{ mspmpuea: {=u32:?} }}", self.mspmpuea())
    }
}
#[doc = "Stack Pointer Monitor Operation After Detection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspmpuoad(pub u16);
impl Mspmpuoad {
    #[doc = "Operation after detection"]
    #[must_use]
    #[inline(always)]
    pub const fn oad(&self) -> super::vals::MspmpuoadOad {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MspmpuoadOad::from_bits(val as u8)
    }
    #[doc = "Operation after detection"]
    #[inline(always)]
    pub const fn set_oad(&mut self, val: super::vals::MspmpuoadOad) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
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
    pub const fn key(&self) -> super::vals::MspmpuoadKey {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::MspmpuoadKey::from_bits(val as u8)
    }
    #[doc = "Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    pub const fn set_key(&mut self, val: super::vals::MspmpuoadKey) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
    }
}
impl Default for Mspmpuoad {
    #[inline(always)]
    fn default() -> Mspmpuoad {
        Mspmpuoad(0)
    }
}
impl core::fmt::Debug for Mspmpuoad {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mspmpuoad")
            .field("oad", &self.oad())
            .field("reserved", &self.reserved())
            .field("key", &self.key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mspmpuoad {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mspmpuoad {{ oad: {:?}, reserved: {=u8:?}, key: {:?} }}",
            self.oad(),
            self.reserved(),
            self.key()
        )
    }
}
#[doc = "Stack Pointer Monitor Protection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspmpupt(pub u16);
impl Mspmpupt {
    #[doc = "Protection of register (MSPMPUAC, MSPMPUSA and MSPMPUSE)"]
    #[must_use]
    #[inline(always)]
    pub const fn protect(&self) -> super::vals::MspmpuptProtect {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MspmpuptProtect::from_bits(val as u8)
    }
    #[doc = "Protection of register (MSPMPUAC, MSPMPUSA and MSPMPUSE)"]
    #[inline(always)]
    pub const fn set_protect(&mut self, val: super::vals::MspmpuptProtect) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
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
    pub const fn key(&self) -> super::vals::MspmpuptKey {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::MspmpuptKey::from_bits(val as u8)
    }
    #[doc = "Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    pub const fn set_key(&mut self, val: super::vals::MspmpuptKey) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
    }
}
impl Default for Mspmpupt {
    #[inline(always)]
    fn default() -> Mspmpupt {
        Mspmpupt(0)
    }
}
impl core::fmt::Debug for Mspmpupt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mspmpupt")
            .field("protect", &self.protect())
            .field("reserved", &self.reserved())
            .field("key", &self.key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mspmpupt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mspmpupt {{ protect: {:?}, reserved: {=u8:?}, key: {:?} }}",
            self.protect(),
            self.reserved(),
            self.key()
        )
    }
}
#[doc = "Main Stack Pointer (MSP) Monitor Start Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspmpusa(pub u32);
impl Mspmpusa {
    #[doc = "Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn mspmpusa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub const fn set_mspmpusa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mspmpusa {
    #[inline(always)]
    fn default() -> Mspmpusa {
        Mspmpusa(0)
    }
}
impl core::fmt::Debug for Mspmpusa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mspmpusa")
            .field("mspmpusa", &self.mspmpusa())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mspmpusa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mspmpusa {{ mspmpusa: {=u32:?} }}", self.mspmpusa())
    }
}
#[doc = "Stack Pointer Monitor Access Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pspmpuctl(pub u16);
impl Pspmpuctl {
    #[doc = "Stack Pointer Monitor Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::PspmpuctlEnable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PspmpuctlEnable::from_bits(val as u8)
    }
    #[doc = "Stack Pointer Monitor Enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: super::vals::PspmpuctlEnable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
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
    #[doc = "Stack Pointer Monitor Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> super::vals::PspmpuctlError {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PspmpuctlError::from_bits(val as u8)
    }
    #[doc = "Stack Pointer Monitor Error Flag"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: super::vals::PspmpuctlError) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u16) & 0x7f) << 9usize);
    }
}
impl Default for Pspmpuctl {
    #[inline(always)]
    fn default() -> Pspmpuctl {
        Pspmpuctl(0)
    }
}
impl core::fmt::Debug for Pspmpuctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pspmpuctl")
            .field("enable", &self.enable())
            .field("reserved", &self.reserved())
            .field("error", &self.error())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pspmpuctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pspmpuctl {{ enable: {:?}, reserved: {=u8:?}, error: {:?}, reserved_2: {=u8:?} }}",
            self.enable(),
            self.reserved(),
            self.error(),
            self.reserved_2()
        )
    }
}
#[doc = "Process Stack Pointer (PSP) Monitor End Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pspmpuea(pub u32);
impl Pspmpuea {
    #[doc = "Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn pspmpuea(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub const fn set_pspmpuea(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pspmpuea {
    #[inline(always)]
    fn default() -> Pspmpuea {
        Pspmpuea(0)
    }
}
impl core::fmt::Debug for Pspmpuea {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pspmpuea")
            .field("pspmpuea", &self.pspmpuea())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pspmpuea {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pspmpuea {{ pspmpuea: {=u32:?} }}", self.pspmpuea())
    }
}
#[doc = "Stack Pointer Monitor Operation After Detection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pspmpuoad(pub u16);
impl Pspmpuoad {
    #[doc = "Operation after detection"]
    #[must_use]
    #[inline(always)]
    pub const fn oad(&self) -> super::vals::PspmpuoadOad {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PspmpuoadOad::from_bits(val as u8)
    }
    #[doc = "Operation after detection"]
    #[inline(always)]
    pub const fn set_oad(&mut self, val: super::vals::PspmpuoadOad) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
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
    #[doc = "Key Code The data written to these bits are not stored."]
    #[must_use]
    #[inline(always)]
    pub const fn key(&self) -> super::vals::PspmpuoadKey {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::PspmpuoadKey::from_bits(val as u8)
    }
    #[doc = "Key Code The data written to these bits are not stored."]
    #[inline(always)]
    pub const fn set_key(&mut self, val: super::vals::PspmpuoadKey) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
    }
}
impl Default for Pspmpuoad {
    #[inline(always)]
    fn default() -> Pspmpuoad {
        Pspmpuoad(0)
    }
}
impl core::fmt::Debug for Pspmpuoad {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pspmpuoad")
            .field("oad", &self.oad())
            .field("reserved", &self.reserved())
            .field("key", &self.key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pspmpuoad {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pspmpuoad {{ oad: {:?}, reserved: {=u8:?}, key: {:?} }}",
            self.oad(),
            self.reserved(),
            self.key()
        )
    }
}
#[doc = "Stack Pointer Monitor Protection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pspmpupt(pub u16);
impl Pspmpupt {
    #[doc = "Protection register (PSPMPUAC, PSPMPUSA and PSPMPUSE)"]
    #[must_use]
    #[inline(always)]
    pub const fn protect(&self) -> super::vals::PspmpuptProtect {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PspmpuptProtect::from_bits(val as u8)
    }
    #[doc = "Protection register (PSPMPUAC, PSPMPUSA and PSPMPUSE)"]
    #[inline(always)]
    pub const fn set_protect(&mut self, val: super::vals::PspmpuptProtect) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
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
    #[doc = "Key Code The data written to these bits are not stored."]
    #[must_use]
    #[inline(always)]
    pub const fn key(&self) -> super::vals::PspmpuptKey {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::PspmpuptKey::from_bits(val as u8)
    }
    #[doc = "Key Code The data written to these bits are not stored."]
    #[inline(always)]
    pub const fn set_key(&mut self, val: super::vals::PspmpuptKey) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
    }
}
impl Default for Pspmpupt {
    #[inline(always)]
    fn default() -> Pspmpupt {
        Pspmpupt(0)
    }
}
impl core::fmt::Debug for Pspmpupt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pspmpupt")
            .field("protect", &self.protect())
            .field("reserved", &self.reserved())
            .field("key", &self.key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pspmpupt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pspmpupt {{ protect: {:?}, reserved: {=u8:?}, key: {:?} }}",
            self.protect(),
            self.reserved(),
            self.key()
        )
    }
}
#[doc = "Process Stack Pointer (PSP) Monitor Start Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pspmpusa(pub u32);
impl Pspmpusa {
    #[doc = "Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn pspmpusa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub const fn set_pspmpusa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pspmpusa {
    #[inline(always)]
    fn default() -> Pspmpusa {
        Pspmpusa(0)
    }
}
impl core::fmt::Debug for Pspmpusa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pspmpusa")
            .field("pspmpusa", &self.pspmpusa())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pspmpusa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pspmpusa {{ pspmpusa: {=u32:?} }}", self.pspmpusa())
    }
}
