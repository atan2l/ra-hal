#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EidrEidr(u16);
impl EidrEidr {
    #[doc = "Low input"]
    pub const _0: Self = Self(0x0);
    #[doc = "High input."]
    pub const _1: Self = Self(0x01);
}
impl EidrEidr {
    pub const fn from_bits(val: u16) -> EidrEidr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for EidrEidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EidrEidr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for EidrEidr {
    #[inline(always)]
    fn from(val: u16) -> EidrEidr {
        EidrEidr::from_bits(val)
    }
}
impl From<EidrEidr> for u16 {
    #[inline(always)]
    fn from(val: EidrEidr) -> u16 {
        EidrEidr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EorrEorr(u16);
impl EorrEorr {
    #[doc = "No affect to output"]
    pub const _0: Self = Self(0x0);
    #[doc = "Low output"]
    pub const _1: Self = Self(0x01);
}
impl EorrEorr {
    pub const fn from_bits(val: u16) -> EorrEorr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for EorrEorr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EorrEorr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for EorrEorr {
    #[inline(always)]
    fn from(val: u16) -> EorrEorr {
        EorrEorr::from_bits(val)
    }
}
impl From<EorrEorr> for u16 {
    #[inline(always)]
    fn from(val: EorrEorr) -> u16 {
        EorrEorr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EosrEosr(u16);
impl EosrEosr {
    #[doc = "No affect to output"]
    pub const _0: Self = Self(0x0);
    #[doc = "High output."]
    pub const _1: Self = Self(0x01);
}
impl EosrEosr {
    pub const fn from_bits(val: u16) -> EosrEosr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for EosrEosr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EosrEosr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for EosrEosr {
    #[inline(always)]
    fn from(val: u16) -> EosrEosr {
        EosrEosr::from_bits(val)
    }
}
impl From<EosrEosr> for u16 {
    #[inline(always)]
    fn from(val: EosrEosr) -> u16 {
        EosrEosr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pcntr1Pdr(u16);
impl Pcntr1Pdr {
    #[doc = "Input (functions as an input pin)"]
    pub const _0: Self = Self(0x0);
    #[doc = "Output (functions as an output pin)."]
    pub const _1: Self = Self(0x01);
}
impl Pcntr1Pdr {
    pub const fn from_bits(val: u16) -> Pcntr1Pdr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pcntr1Pdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr1Pdr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pcntr1Pdr {
    #[inline(always)]
    fn from(val: u16) -> Pcntr1Pdr {
        Pcntr1Pdr::from_bits(val)
    }
}
impl From<Pcntr1Pdr> for u16 {
    #[inline(always)]
    fn from(val: Pcntr1Pdr) -> u16 {
        Pcntr1Pdr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pcntr1Podr(u16);
impl Pcntr1Podr {
    #[doc = "Low output"]
    pub const _0: Self = Self(0x0);
    #[doc = "High output."]
    pub const _1: Self = Self(0x01);
}
impl Pcntr1Podr {
    pub const fn from_bits(val: u16) -> Pcntr1Podr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pcntr1Podr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr1Podr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pcntr1Podr {
    #[inline(always)]
    fn from(val: u16) -> Pcntr1Podr {
        Pcntr1Podr::from_bits(val)
    }
}
impl From<Pcntr1Podr> for u16 {
    #[inline(always)]
    fn from(val: Pcntr1Podr) -> u16 {
        Pcntr1Podr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pcntr2Eidr(u16);
impl Pcntr2Eidr {
    #[doc = "Low input"]
    pub const _0: Self = Self(0x0);
    #[doc = "High input."]
    pub const _1: Self = Self(0x01);
}
impl Pcntr2Eidr {
    pub const fn from_bits(val: u16) -> Pcntr2Eidr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pcntr2Eidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr2Eidr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pcntr2Eidr {
    #[inline(always)]
    fn from(val: u16) -> Pcntr2Eidr {
        Pcntr2Eidr::from_bits(val)
    }
}
impl From<Pcntr2Eidr> for u16 {
    #[inline(always)]
    fn from(val: Pcntr2Eidr) -> u16 {
        Pcntr2Eidr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pcntr2Pidr(u16);
impl Pcntr2Pidr {
    #[doc = "Low input"]
    pub const _0: Self = Self(0x0);
    #[doc = "High input."]
    pub const _1: Self = Self(0x01);
}
impl Pcntr2Pidr {
    pub const fn from_bits(val: u16) -> Pcntr2Pidr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pcntr2Pidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr2Pidr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pcntr2Pidr {
    #[inline(always)]
    fn from(val: u16) -> Pcntr2Pidr {
        Pcntr2Pidr::from_bits(val)
    }
}
impl From<Pcntr2Pidr> for u16 {
    #[inline(always)]
    fn from(val: Pcntr2Pidr) -> u16 {
        Pcntr2Pidr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pcntr3Porr(u16);
impl Pcntr3Porr {
    #[doc = "No affect to output"]
    pub const _0: Self = Self(0x0);
    #[doc = "Low output."]
    pub const _1: Self = Self(0x01);
}
impl Pcntr3Porr {
    pub const fn from_bits(val: u16) -> Pcntr3Porr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pcntr3Porr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr3Porr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pcntr3Porr {
    #[inline(always)]
    fn from(val: u16) -> Pcntr3Porr {
        Pcntr3Porr::from_bits(val)
    }
}
impl From<Pcntr3Porr> for u16 {
    #[inline(always)]
    fn from(val: Pcntr3Porr) -> u16 {
        Pcntr3Porr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pcntr3Posr(u16);
impl Pcntr3Posr {
    #[doc = "No affect to output"]
    pub const _0: Self = Self(0x0);
    #[doc = "High output."]
    pub const _1: Self = Self(0x01);
}
impl Pcntr3Posr {
    pub const fn from_bits(val: u16) -> Pcntr3Posr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pcntr3Posr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr3Posr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pcntr3Posr {
    #[inline(always)]
    fn from(val: u16) -> Pcntr3Posr {
        Pcntr3Posr::from_bits(val)
    }
}
impl From<Pcntr3Posr> for u16 {
    #[inline(always)]
    fn from(val: Pcntr3Posr) -> u16 {
        Pcntr3Posr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pcntr4Eorr(u16);
impl Pcntr4Eorr {
    #[doc = "No affect to output"]
    pub const _0: Self = Self(0x0);
    #[doc = "Low output"]
    pub const _1: Self = Self(0x01);
}
impl Pcntr4Eorr {
    pub const fn from_bits(val: u16) -> Pcntr4Eorr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pcntr4Eorr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr4Eorr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pcntr4Eorr {
    #[inline(always)]
    fn from(val: u16) -> Pcntr4Eorr {
        Pcntr4Eorr::from_bits(val)
    }
}
impl From<Pcntr4Eorr> for u16 {
    #[inline(always)]
    fn from(val: Pcntr4Eorr) -> u16 {
        Pcntr4Eorr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pcntr4Eosr(u16);
impl Pcntr4Eosr {
    #[doc = "No affect to output"]
    pub const _0: Self = Self(0x0);
    #[doc = "High output."]
    pub const _1: Self = Self(0x01);
}
impl Pcntr4Eosr {
    pub const fn from_bits(val: u16) -> Pcntr4Eosr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pcntr4Eosr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr4Eosr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pcntr4Eosr {
    #[inline(always)]
    fn from(val: u16) -> Pcntr4Eosr {
        Pcntr4Eosr::from_bits(val)
    }
}
impl From<Pcntr4Eosr> for u16 {
    #[inline(always)]
    fn from(val: Pcntr4Eosr) -> u16 {
        Pcntr4Eosr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PdrPdr(u16);
impl PdrPdr {
    #[doc = "Input (functions as an input pin)"]
    pub const _0: Self = Self(0x0);
    #[doc = "Output (functions as an output pin)."]
    pub const _1: Self = Self(0x01);
}
impl PdrPdr {
    pub const fn from_bits(val: u16) -> PdrPdr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for PdrPdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PdrPdr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for PdrPdr {
    #[inline(always)]
    fn from(val: u16) -> PdrPdr {
        PdrPdr::from_bits(val)
    }
}
impl From<PdrPdr> for u16 {
    #[inline(always)]
    fn from(val: PdrPdr) -> u16 {
        PdrPdr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PidrPidr(u16);
impl PidrPidr {
    #[doc = "Low input"]
    pub const _0: Self = Self(0x0);
    #[doc = "High input."]
    pub const _1: Self = Self(0x01);
}
impl PidrPidr {
    pub const fn from_bits(val: u16) -> PidrPidr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for PidrPidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PidrPidr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for PidrPidr {
    #[inline(always)]
    fn from(val: u16) -> PidrPidr {
        PidrPidr::from_bits(val)
    }
}
impl From<PidrPidr> for u16 {
    #[inline(always)]
    fn from(val: PidrPidr) -> u16 {
        PidrPidr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PodrPodr(u16);
impl PodrPodr {
    #[doc = "Low output"]
    pub const _0: Self = Self(0x0);
    #[doc = "High output."]
    pub const _1: Self = Self(0x01);
}
impl PodrPodr {
    pub const fn from_bits(val: u16) -> PodrPodr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for PodrPodr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PodrPodr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for PodrPodr {
    #[inline(always)]
    fn from(val: u16) -> PodrPodr {
        PodrPodr::from_bits(val)
    }
}
impl From<PodrPodr> for u16 {
    #[inline(always)]
    fn from(val: PodrPodr) -> u16 {
        PodrPodr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PorrPorr(u16);
impl PorrPorr {
    #[doc = "No affect to output"]
    pub const _0: Self = Self(0x0);
    #[doc = "Low output."]
    pub const _1: Self = Self(0x01);
}
impl PorrPorr {
    pub const fn from_bits(val: u16) -> PorrPorr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for PorrPorr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PorrPorr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for PorrPorr {
    #[inline(always)]
    fn from(val: u16) -> PorrPorr {
        PorrPorr::from_bits(val)
    }
}
impl From<PorrPorr> for u16 {
    #[inline(always)]
    fn from(val: PorrPorr) -> u16 {
        PorrPorr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PosrPosr(u16);
impl PosrPosr {
    #[doc = "No affect to output"]
    pub const _0: Self = Self(0x0);
    #[doc = "High output."]
    pub const _1: Self = Self(0x01);
}
impl PosrPosr {
    pub const fn from_bits(val: u16) -> PosrPosr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for PosrPosr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PosrPosr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for PosrPosr {
    #[inline(always)]
    fn from(val: u16) -> PosrPosr {
        PosrPosr::from_bits(val)
    }
}
impl From<PosrPosr> for u16 {
    #[inline(always)]
    fn from(val: PosrPosr) -> u16 {
        PosrPosr::to_bits(val)
    }
}
