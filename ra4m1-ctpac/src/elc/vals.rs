#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Elsr12Els(u8);
impl Elsr12Els {
    #[doc = "Event output to the corresponding peripheral module is disabled."]
    pub const _0X00: Self = Self(0x0);
}
impl Elsr12Els {
    pub const fn from_bits(val: u8) -> Elsr12Els {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Elsr12Els {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0X00"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsr12Els {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0X00"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Elsr12Els {
    #[inline(always)]
    fn from(val: u8) -> Elsr12Els {
        Elsr12Els::from_bits(val)
    }
}
impl From<Elsr12Els> for u8 {
    #[inline(always)]
    fn from(val: Elsr12Els) -> u8 {
        Elsr12Els::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Elsr2Els(u8);
impl Elsr2Els {
    #[doc = "Event output to the corresponding peripheral module is disabled."]
    pub const _0X00: Self = Self(0x0);
}
impl Elsr2Els {
    pub const fn from_bits(val: u8) -> Elsr2Els {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Elsr2Els {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0X00"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsr2Els {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0X00"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Elsr2Els {
    #[inline(always)]
    fn from(val: u8) -> Elsr2Els {
        Elsr2Els::from_bits(val)
    }
}
impl From<Elsr2Els> for u8 {
    #[inline(always)]
    fn from(val: Elsr2Els) -> u8 {
        Elsr2Els::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ElsrEls(u8);
impl ElsrEls {
    #[doc = "Event output to the corresponding peripheral module is disabled."]
    pub const _0X00: Self = Self(0x0);
}
impl ElsrEls {
    pub const fn from_bits(val: u8) -> ElsrEls {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ElsrEls {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0X00"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsrEls {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0X00"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ElsrEls {
    #[inline(always)]
    fn from(val: u8) -> ElsrEls {
        ElsrEls::from_bits(val)
    }
}
impl From<ElsrEls> for u8 {
    #[inline(always)]
    fn from(val: ElsrEls) -> u8 {
        ElsrEls::to_bits(val)
    }
}
