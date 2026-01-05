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
