#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsuatune0 {
    #[doc = "Normal operating mode"]
    _0 = 0x0,
    #[doc = "Low-voltage operating mode"]
    _1 = 0x01,
}
impl Ctsuatune0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsuatune0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsuatune0 {
    #[inline(always)]
    fn from(val: u8) -> Ctsuatune0 {
        Ctsuatune0::from_bits(val)
    }
}
impl From<Ctsuatune0> for u8 {
    #[inline(always)]
    fn from(val: Ctsuatune0) -> u8 {
        Ctsuatune0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsuatune1 {
    #[doc = "Normal output"]
    _0 = 0x0,
    #[doc = "High-current output"]
    _1 = 0x01,
}
impl Ctsuatune1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsuatune1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsuatune1 {
    #[inline(always)]
    fn from(val: u8) -> Ctsuatune1 {
        Ctsuatune1::from_bits(val)
    }
}
impl From<Ctsuatune1> for u8 {
    #[inline(always)]
    fn from(val: Ctsuatune1) -> u8 {
        Ctsuatune1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsucap {
    #[doc = "Software trigger."]
    _0 = 0x0,
    #[doc = "External trigger."]
    _1 = 0x01,
}
impl Ctsucap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsucap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsucap {
    #[inline(always)]
    fn from(val: u8) -> Ctsucap {
        Ctsucap::from_bits(val)
    }
}
impl From<Ctsucap> for u8 {
    #[inline(always)]
    fn from(val: Ctsucap) -> u8 {
        Ctsucap::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctsuchac0(u8);
impl Ctsuchac0 {}
impl Ctsuchac0 {
    pub const fn from_bits(val: u8) -> Ctsuchac0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctsuchac0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchac0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctsuchac0 {
    #[inline(always)]
    fn from(val: u8) -> Ctsuchac0 {
        Ctsuchac0::from_bits(val)
    }
}
impl From<Ctsuchac0> for u8 {
    #[inline(always)]
    fn from(val: Ctsuchac0) -> u8 {
        Ctsuchac0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctsuchac1(u8);
impl Ctsuchac1 {}
impl Ctsuchac1 {
    pub const fn from_bits(val: u8) -> Ctsuchac1 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctsuchac1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchac1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctsuchac1 {
    #[inline(always)]
    fn from(val: u8) -> Ctsuchac1 {
        Ctsuchac1::from_bits(val)
    }
}
impl From<Ctsuchac1> for u8 {
    #[inline(always)]
    fn from(val: Ctsuchac1) -> u8 {
        Ctsuchac1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctsuchac2(u8);
impl Ctsuchac2 {}
impl Ctsuchac2 {
    pub const fn from_bits(val: u8) -> Ctsuchac2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctsuchac2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchac2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctsuchac2 {
    #[inline(always)]
    fn from(val: u8) -> Ctsuchac2 {
        Ctsuchac2::from_bits(val)
    }
}
impl From<Ctsuchac2> for u8 {
    #[inline(always)]
    fn from(val: Ctsuchac2) -> u8 {
        Ctsuchac2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctsuchac3(u8);
impl Ctsuchac3 {}
impl Ctsuchac3 {
    pub const fn from_bits(val: u8) -> Ctsuchac3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctsuchac3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchac3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctsuchac3 {
    #[inline(always)]
    fn from(val: u8) -> Ctsuchac3 {
        Ctsuchac3::from_bits(val)
    }
}
impl From<Ctsuchac3> for u8 {
    #[inline(always)]
    fn from(val: Ctsuchac3) -> u8 {
        Ctsuchac3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsuchac4Ctsuchac4 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ctsuchac4Ctsuchac4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsuchac4Ctsuchac4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsuchac4Ctsuchac4 {
    #[inline(always)]
    fn from(val: u8) -> Ctsuchac4Ctsuchac4 {
        Ctsuchac4Ctsuchac4::from_bits(val)
    }
}
impl From<Ctsuchac4Ctsuchac4> for u8 {
    #[inline(always)]
    fn from(val: Ctsuchac4Ctsuchac4) -> u8 {
        Ctsuchac4Ctsuchac4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctsuchtrc0(u8);
impl Ctsuchtrc0 {
    #[doc = "Reception"]
    pub const _0: Self = Self(0x0);
    #[doc = "Transmission"]
    pub const _1: Self = Self(0x01);
}
impl Ctsuchtrc0 {
    pub const fn from_bits(val: u8) -> Ctsuchtrc0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctsuchtrc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchtrc0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctsuchtrc0 {
    #[inline(always)]
    fn from(val: u8) -> Ctsuchtrc0 {
        Ctsuchtrc0::from_bits(val)
    }
}
impl From<Ctsuchtrc0> for u8 {
    #[inline(always)]
    fn from(val: Ctsuchtrc0) -> u8 {
        Ctsuchtrc0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctsuchtrc1(u8);
impl Ctsuchtrc1 {
    #[doc = "Reception"]
    pub const _0: Self = Self(0x0);
    #[doc = "Transmission"]
    pub const _1: Self = Self(0x01);
}
impl Ctsuchtrc1 {
    pub const fn from_bits(val: u8) -> Ctsuchtrc1 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctsuchtrc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchtrc1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctsuchtrc1 {
    #[inline(always)]
    fn from(val: u8) -> Ctsuchtrc1 {
        Ctsuchtrc1::from_bits(val)
    }
}
impl From<Ctsuchtrc1> for u8 {
    #[inline(always)]
    fn from(val: Ctsuchtrc1) -> u8 {
        Ctsuchtrc1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctsuchtrc2(u8);
impl Ctsuchtrc2 {
    #[doc = "Reception"]
    pub const _0: Self = Self(0x0);
    #[doc = "Transmission"]
    pub const _1: Self = Self(0x01);
}
impl Ctsuchtrc2 {
    pub const fn from_bits(val: u8) -> Ctsuchtrc2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctsuchtrc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchtrc2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctsuchtrc2 {
    #[inline(always)]
    fn from(val: u8) -> Ctsuchtrc2 {
        Ctsuchtrc2::from_bits(val)
    }
}
impl From<Ctsuchtrc2> for u8 {
    #[inline(always)]
    fn from(val: Ctsuchtrc2) -> u8 {
        Ctsuchtrc2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctsuchtrc3(u8);
impl Ctsuchtrc3 {
    #[doc = "Reception"]
    pub const _0: Self = Self(0x0);
    #[doc = "Transmission"]
    pub const _1: Self = Self(0x01);
}
impl Ctsuchtrc3 {
    pub const fn from_bits(val: u8) -> Ctsuchtrc3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctsuchtrc3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchtrc3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctsuchtrc3 {
    #[inline(always)]
    fn from(val: u8) -> Ctsuchtrc3 {
        Ctsuchtrc3::from_bits(val)
    }
}
impl From<Ctsuchtrc3> for u8 {
    #[inline(always)]
    fn from(val: Ctsuchtrc3) -> u8 {
        Ctsuchtrc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsuchtrc4Ctsuchac4 {
    #[doc = "Reception"]
    _0 = 0x0,
    #[doc = "Transmission"]
    _1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ctsuchtrc4Ctsuchac4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsuchtrc4Ctsuchac4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsuchtrc4Ctsuchac4 {
    #[inline(always)]
    fn from(val: u8) -> Ctsuchtrc4Ctsuchac4 {
        Ctsuchtrc4Ctsuchac4::from_bits(val)
    }
}
impl From<Ctsuchtrc4Ctsuchac4> for u8 {
    #[inline(always)]
    fn from(val: Ctsuchtrc4Ctsuchac4) -> u8 {
        Ctsuchtrc4Ctsuchac4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsuclk {
    #[doc = "PCLK"]
    _00 = 0x0,
    #[doc = "PCLK/2 (PCLK divided by 2)"]
    _01 = 0x01,
    #[doc = "PCLK/2 (PCLK divided by 4)"]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Ctsuclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsuclk {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsuclk {
    #[inline(always)]
    fn from(val: u8) -> Ctsuclk {
        Ctsuclk::from_bits(val)
    }
}
impl From<Ctsuclk> for u8 {
    #[inline(always)]
    fn from(val: Ctsuclk) -> u8 {
        Ctsuclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsucsw {
    #[doc = "Turned off capacitance switch"]
    _0 = 0x0,
    #[doc = "Turned on capacitance switch"]
    _1 = 0x01,
}
impl Ctsucsw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsucsw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsucsw {
    #[inline(always)]
    fn from(val: u8) -> Ctsucsw {
        Ctsucsw::from_bits(val)
    }
}
impl From<Ctsucsw> for u8 {
    #[inline(always)]
    fn from(val: Ctsucsw) -> u8 {
        Ctsucsw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsudtsr {
    #[doc = "Measurement result has been read"]
    _0 = 0x0,
    #[doc = "Measurement result has not been read"]
    _1 = 0x01,
}
impl Ctsudtsr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsudtsr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsudtsr {
    #[inline(always)]
    fn from(val: u8) -> Ctsudtsr {
        Ctsudtsr::from_bits(val)
    }
}
impl From<Ctsudtsr> for u8 {
    #[inline(always)]
    fn from(val: Ctsudtsr) -> u8 {
        Ctsudtsr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsuicog {
    #[doc = "100 percent gain"]
    _00 = 0x0,
    #[doc = "66 percent gain"]
    _01 = 0x01,
    #[doc = "50 percent gain"]
    _10 = 0x02,
    #[doc = "40 percent gain"]
    _11 = 0x03,
}
impl Ctsuicog {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsuicog {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsuicog {
    #[inline(always)]
    fn from(val: u8) -> Ctsuicog {
        Ctsuicog::from_bits(val)
    }
}
impl From<Ctsuicog> for u8 {
    #[inline(always)]
    fn from(val: Ctsuicog) -> u8 {
        Ctsuicog::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsuicomp {
    #[doc = "Normal TSCAP voltage"]
    _0 = 0x0,
    #[doc = "Abnormal TSCAP voltage"]
    _1 = 0x01,
}
impl Ctsuicomp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsuicomp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsuicomp {
    #[inline(always)]
    fn from(val: u8) -> Ctsuicomp {
        Ctsuicomp::from_bits(val)
    }
}
impl From<Ctsuicomp> for u8 {
    #[inline(always)]
    fn from(val: Ctsuicomp) -> u8 {
        Ctsuicomp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsuinit {
    #[doc = "Writing a 0 has no effect, this bit is read as 0."]
    _0 = 0x0,
    #[doc = "initializes the CTSU control block and registers."]
    _1 = 0x01,
}
impl Ctsuinit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsuinit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsuinit {
    #[inline(always)]
    fn from(val: u8) -> Ctsuinit {
        Ctsuinit::from_bits(val)
    }
}
impl From<Ctsuinit> for u8 {
    #[inline(always)]
    fn from(val: Ctsuinit) -> u8 {
        Ctsuinit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsumch0 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ctsumch0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsumch0 {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsumch0 {
    #[inline(always)]
    fn from(val: u8) -> Ctsumch0 {
        Ctsumch0::from_bits(val)
    }
}
impl From<Ctsumch0> for u8 {
    #[inline(always)]
    fn from(val: Ctsumch0) -> u8 {
        Ctsumch0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsumch1 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ctsumch1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsumch1 {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsumch1 {
    #[inline(always)]
    fn from(val: u8) -> Ctsumch1 {
        Ctsumch1::from_bits(val)
    }
}
impl From<Ctsumch1> for u8 {
    #[inline(always)]
    fn from(val: Ctsumch1) -> u8 {
        Ctsumch1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsumd {
    #[doc = "Self-capacitance single scan mode"]
    _00 = 0x0,
    #[doc = "Self-capacitance multi-scan mode"]
    _01 = 0x01,
    #[doc = "Mutual capacitance simple scan mode"]
    _10 = 0x02,
    #[doc = "Mutual capacitance full scan mode"]
    _11 = 0x03,
}
impl Ctsumd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsumd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsumd {
    #[inline(always)]
    fn from(val: u8) -> Ctsumd {
        Ctsumd::from_bits(val)
    }
}
impl From<Ctsumd> for u8 {
    #[inline(always)]
    fn from(val: Ctsumd) -> u8 {
        Ctsumd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsupon {
    #[doc = "Powered off the CTSU"]
    _0 = 0x0,
    #[doc = "Powered on the CTSU"]
    _1 = 0x01,
}
impl Ctsupon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsupon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsupon {
    #[inline(always)]
    fn from(val: u8) -> Ctsupon {
        Ctsupon::from_bits(val)
    }
}
impl From<Ctsupon> for u8 {
    #[inline(always)]
    fn from(val: Ctsupon) -> u8 {
        Ctsupon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsuprmode {
    #[doc = "510 pulses"]
    _00 = 0x0,
    #[doc = "126 pulses"]
    _01 = 0x01,
    #[doc = "62 pulses (recommended setting value)"]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Ctsuprmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsuprmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsuprmode {
    #[inline(always)]
    fn from(val: u8) -> Ctsuprmode {
        Ctsuprmode::from_bits(val)
    }
}
impl From<Ctsuprmode> for u8 {
    #[inline(always)]
    fn from(val: Ctsuprmode) -> u8 {
        Ctsuprmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsups {
    #[doc = "First measurement"]
    _0 = 0x0,
    #[doc = "Second measurement"]
    _1 = 0x01,
}
impl Ctsups {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsups {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsups {
    #[inline(always)]
    fn from(val: u8) -> Ctsups {
        Ctsups::from_bits(val)
    }
}
impl From<Ctsups> for u8 {
    #[inline(always)]
    fn from(val: Ctsups) -> u8 {
        Ctsups::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsurovf {
    #[doc = "No overflow"]
    _0 = 0x0,
    #[doc = "An overflow"]
    _1 = 0x01,
}
impl Ctsurovf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsurovf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsurovf {
    #[inline(always)]
    fn from(val: u8) -> Ctsurovf {
        Ctsurovf::from_bits(val)
    }
}
impl From<Ctsurovf> for u8 {
    #[inline(always)]
    fn from(val: Ctsurovf) -> u8 {
        Ctsurovf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsusnz {
    #[doc = "Power-saving function during wait state is disabled."]
    _0 = 0x0,
    #[doc = "Power-saving function during wait state is enabled."]
    _1 = 0x01,
}
impl Ctsusnz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsusnz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsusnz {
    #[inline(always)]
    fn from(val: u8) -> Ctsusnz {
        Ctsusnz::from_bits(val)
    }
}
impl From<Ctsusnz> for u8 {
    #[inline(always)]
    fn from(val: Ctsusnz) -> u8 {
        Ctsusnz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsusoff {
    #[doc = "High-pass noise reduction function turned on"]
    _0 = 0x0,
    #[doc = "High-pass noise reduction function turned off"]
    _1 = 0x01,
}
impl Ctsusoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsusoff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsusoff {
    #[inline(always)]
    fn from(val: u8) -> Ctsusoff {
        Ctsusoff::from_bits(val)
    }
}
impl From<Ctsusoff> for u8 {
    #[inline(always)]
    fn from(val: Ctsusoff) -> u8 {
        Ctsusoff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsusovf {
    #[doc = "No overflow"]
    _0 = 0x0,
    #[doc = "An overflow"]
    _1 = 0x01,
}
impl Ctsusovf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsusovf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsusovf {
    #[inline(always)]
    fn from(val: u8) -> Ctsusovf {
        Ctsusovf::from_bits(val)
    }
}
impl From<Ctsusovf> for u8 {
    #[inline(always)]
    fn from(val: Ctsusovf) -> u8 {
        Ctsusovf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsussdiv {
    #[doc = "4.00 <= fb"]
    _0000 = 0x0,
    #[doc = "2.00 <= fb < 4.00"]
    _0001 = 0x01,
    #[doc = "1.33 <= fb < 2.00"]
    _0010 = 0x02,
    #[doc = "1.00 <= fb < 1.33"]
    _0011 = 0x03,
    #[doc = "0.80 <= fb < 1.00"]
    _0100 = 0x04,
    #[doc = "0.67 <= fb < 0.80"]
    _0101 = 0x05,
    #[doc = "0.57 <= fb < 0.67"]
    _0110 = 0x06,
    #[doc = "0.50 <= fb < 0.57"]
    _0111 = 0x07,
    #[doc = "0.44 <= fb < 0.50"]
    _1000 = 0x08,
    #[doc = "0.40 <= fb < 0.44"]
    _1001 = 0x09,
    #[doc = "0.36 <= fb < 0.40"]
    _1010 = 0x0a,
    #[doc = "0.33 <= fb < 0.36"]
    _1011 = 0x0b,
    #[doc = "0.31 <= fb < 0.33"]
    _1100 = 0x0c,
    #[doc = "0.29 <= fb < 0.31"]
    _1101 = 0x0d,
    #[doc = "0.27 <= fb < 0.29"]
    _1110 = 0x0e,
    #[doc = "fb < 0.27"]
    _1111 = 0x0f,
}
impl Ctsussdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsussdiv {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsussdiv {
    #[inline(always)]
    fn from(val: u8) -> Ctsussdiv {
        Ctsussdiv::from_bits(val)
    }
}
impl From<Ctsussdiv> for u8 {
    #[inline(always)]
    fn from(val: Ctsussdiv) -> u8 {
        Ctsussdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsustc {
    #[doc = "Status 0"]
    _000 = 0x0,
    #[doc = "Status 1"]
    _001 = 0x01,
    #[doc = "Status 2"]
    _010 = 0x02,
    #[doc = "Status 3"]
    _011 = 0x03,
    #[doc = "Status 4"]
    _100 = 0x04,
    #[doc = "Status 5"]
    _101 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctsustc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsustc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsustc {
    #[inline(always)]
    fn from(val: u8) -> Ctsustc {
        Ctsustc::from_bits(val)
    }
}
impl From<Ctsustc> for u8 {
    #[inline(always)]
    fn from(val: Ctsustc) -> u8 {
        Ctsustc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsustrt {
    #[doc = "Measurement operation stops."]
    _0 = 0x0,
    #[doc = "Measurement operation starts."]
    _1 = 0x01,
}
impl Ctsustrt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsustrt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsustrt {
    #[inline(always)]
    fn from(val: u8) -> Ctsustrt {
        Ctsustrt::from_bits(val)
    }
}
impl From<Ctsustrt> for u8 {
    #[inline(always)]
    fn from(val: Ctsustrt) -> u8 {
        Ctsustrt::to_bits(val)
    }
}
