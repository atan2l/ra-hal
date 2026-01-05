#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key(u8);
impl Key {
    #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
    pub const _0X_A5: Self = Self(0xa5);
}
impl Key {
    pub const fn from_bits(val: u8) -> Key {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Key {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xa5 => f.write_str("_0X_A5"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Key {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xa5 => defmt::write!(f, "_0X_A5"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Key {
    #[inline(always)]
    fn from(val: u8) -> Key {
        Key::from_bits(val)
    }
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(val: Key) -> u8 {
        Key::to_bits(val)
    }
}
