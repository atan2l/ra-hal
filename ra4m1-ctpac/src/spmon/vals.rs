#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MspmpuctlEnable {
    #[doc = "Stack pointer monitor is disabled"]
    _0 = 0x0,
    #[doc = "Stack pointer monitor is enabled."]
    _1 = 0x01,
}
impl MspmpuctlEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MspmpuctlEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MspmpuctlEnable {
    #[inline(always)]
    fn from(val: u8) -> MspmpuctlEnable {
        MspmpuctlEnable::from_bits(val)
    }
}
impl From<MspmpuctlEnable> for u8 {
    #[inline(always)]
    fn from(val: MspmpuctlEnable) -> u8 {
        MspmpuctlEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MspmpuctlError {
    #[doc = "Stack pointer has not overflowed or underflowed"]
    _0 = 0x0,
    #[doc = "Stack pointer has overflowed or underflowed"]
    _1 = 0x01,
}
impl MspmpuctlError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MspmpuctlError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MspmpuctlError {
    #[inline(always)]
    fn from(val: u8) -> MspmpuctlError {
        MspmpuctlError::from_bits(val)
    }
}
impl From<MspmpuctlError> for u8 {
    #[inline(always)]
    fn from(val: MspmpuctlError) -> u8 {
        MspmpuctlError::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MspmpuoadKey(u8);
impl MspmpuoadKey {
    #[doc = "Writing to the OAD bit is valid, when the KEY bits are written 0xA5."]
    pub const _0X_A5: Self = Self(0xa5);
}
impl MspmpuoadKey {
    pub const fn from_bits(val: u8) -> MspmpuoadKey {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for MspmpuoadKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xa5 => f.write_str("_0X_A5"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MspmpuoadKey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xa5 => defmt::write!(f, "_0X_A5"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for MspmpuoadKey {
    #[inline(always)]
    fn from(val: u8) -> MspmpuoadKey {
        MspmpuoadKey::from_bits(val)
    }
}
impl From<MspmpuoadKey> for u8 {
    #[inline(always)]
    fn from(val: MspmpuoadKey) -> u8 {
        MspmpuoadKey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MspmpuoadOad {
    #[doc = "Non-maskable interrupt"]
    _0 = 0x0,
    #[doc = "Reset."]
    _1 = 0x01,
}
impl MspmpuoadOad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MspmpuoadOad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MspmpuoadOad {
    #[inline(always)]
    fn from(val: u8) -> MspmpuoadOad {
        MspmpuoadOad::from_bits(val)
    }
}
impl From<MspmpuoadOad> for u8 {
    #[inline(always)]
    fn from(val: MspmpuoadOad) -> u8 {
        MspmpuoadOad::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MspmpuptKey(u8);
impl MspmpuptKey {
    #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
    pub const _0X_A5: Self = Self(0xa5);
}
impl MspmpuptKey {
    pub const fn from_bits(val: u8) -> MspmpuptKey {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for MspmpuptKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xa5 => f.write_str("_0X_A5"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MspmpuptKey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xa5 => defmt::write!(f, "_0X_A5"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for MspmpuptKey {
    #[inline(always)]
    fn from(val: u8) -> MspmpuptKey {
        MspmpuptKey::from_bits(val)
    }
}
impl From<MspmpuptKey> for u8 {
    #[inline(always)]
    fn from(val: MspmpuptKey) -> u8 {
        MspmpuptKey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MspmpuptProtect {
    #[doc = "Stack Pointer Monitor register writing is possible."]
    _0 = 0x0,
    #[doc = "Stack Pointer Monitor register writing is protected."]
    _1 = 0x01,
}
impl MspmpuptProtect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MspmpuptProtect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MspmpuptProtect {
    #[inline(always)]
    fn from(val: u8) -> MspmpuptProtect {
        MspmpuptProtect::from_bits(val)
    }
}
impl From<MspmpuptProtect> for u8 {
    #[inline(always)]
    fn from(val: MspmpuptProtect) -> u8 {
        MspmpuptProtect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PspmpuctlEnable {
    #[doc = "Stack pointer monitor is disabled"]
    _0 = 0x0,
    #[doc = "Stack pointer monitor is enabled"]
    _1 = 0x01,
}
impl PspmpuctlEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PspmpuctlEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PspmpuctlEnable {
    #[inline(always)]
    fn from(val: u8) -> PspmpuctlEnable {
        PspmpuctlEnable::from_bits(val)
    }
}
impl From<PspmpuctlEnable> for u8 {
    #[inline(always)]
    fn from(val: PspmpuctlEnable) -> u8 {
        PspmpuctlEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PspmpuctlError {
    #[doc = "Stack pointer has not overflowed or underflowed"]
    _0 = 0x0,
    #[doc = "Stack pointer has overflowed or underflowed"]
    _1 = 0x01,
}
impl PspmpuctlError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PspmpuctlError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PspmpuctlError {
    #[inline(always)]
    fn from(val: u8) -> PspmpuctlError {
        PspmpuctlError::from_bits(val)
    }
}
impl From<PspmpuctlError> for u8 {
    #[inline(always)]
    fn from(val: PspmpuctlError) -> u8 {
        PspmpuctlError::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PspmpuoadKey(u8);
impl PspmpuoadKey {
    #[doc = "Writing to the OAD bit is valid, when the KEY bits are written 0xA5."]
    pub const _0X_A5: Self = Self(0xa5);
}
impl PspmpuoadKey {
    pub const fn from_bits(val: u8) -> PspmpuoadKey {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for PspmpuoadKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xa5 => f.write_str("_0X_A5"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PspmpuoadKey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xa5 => defmt::write!(f, "_0X_A5"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for PspmpuoadKey {
    #[inline(always)]
    fn from(val: u8) -> PspmpuoadKey {
        PspmpuoadKey::from_bits(val)
    }
}
impl From<PspmpuoadKey> for u8 {
    #[inline(always)]
    fn from(val: PspmpuoadKey) -> u8 {
        PspmpuoadKey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PspmpuoadOad {
    #[doc = "Non-maskable interrupt"]
    _0 = 0x0,
    #[doc = "Reset."]
    _1 = 0x01,
}
impl PspmpuoadOad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PspmpuoadOad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PspmpuoadOad {
    #[inline(always)]
    fn from(val: u8) -> PspmpuoadOad {
        PspmpuoadOad::from_bits(val)
    }
}
impl From<PspmpuoadOad> for u8 {
    #[inline(always)]
    fn from(val: PspmpuoadOad) -> u8 {
        PspmpuoadOad::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PspmpuptKey(u8);
impl PspmpuptKey {
    #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
    pub const _0X_A5: Self = Self(0xa5);
}
impl PspmpuptKey {
    pub const fn from_bits(val: u8) -> PspmpuptKey {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for PspmpuptKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xa5 => f.write_str("_0X_A5"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PspmpuptKey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xa5 => defmt::write!(f, "_0X_A5"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for PspmpuptKey {
    #[inline(always)]
    fn from(val: u8) -> PspmpuptKey {
        PspmpuptKey::from_bits(val)
    }
}
impl From<PspmpuptKey> for u8 {
    #[inline(always)]
    fn from(val: PspmpuptKey) -> u8 {
        PspmpuptKey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PspmpuptProtect {
    #[doc = "Stack Pointer Monitor register writing is possible."]
    _0 = 0x0,
    #[doc = "Stack Pointer Monitor register writing is protected."]
    _1 = 0x01,
}
impl PspmpuptProtect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PspmpuptProtect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PspmpuptProtect {
    #[inline(always)]
    fn from(val: u8) -> PspmpuptProtect {
        PspmpuptProtect::from_bits(val)
    }
}
impl From<PspmpuptProtect> for u8 {
    #[inline(always)]
    fn from(val: PspmpuptProtect) -> u8 {
        PspmpuptProtect::to_bits(val)
    }
}
