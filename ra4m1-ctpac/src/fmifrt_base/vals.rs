#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ExpectedBase(u32);
impl ExpectedBase {
    pub const RA4M1: Self = Self(0x0100_3c00);
}
impl ExpectedBase {
    pub const fn from_bits(val: u32) -> ExpectedBase {
        Self(val & 0x00ff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for ExpectedBase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0100_3c00 => f.write_str("RA4M1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ExpectedBase {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0100_3c00 => defmt::write!(f, "RA4M1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for ExpectedBase {
    #[inline(always)]
    fn from(val: u32) -> ExpectedBase {
        ExpectedBase::from_bits(val)
    }
}
impl From<ExpectedBase> for u32 {
    #[inline(always)]
    fn from(val: ExpectedBase) -> u32 {
        ExpectedBase::to_bits(val)
    }
}
