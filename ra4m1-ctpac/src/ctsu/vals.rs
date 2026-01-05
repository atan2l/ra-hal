#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtsuPrMode {
    #[doc = "510 pulses"]
    _510Pulses = 0x0,
    #[doc = "126 pulses"]
    _126Pulses = 0x01,
    #[doc = "62 pulses (recommended setting value)"]
    _62Pulses = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl CtsuPrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtsuPrMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtsuPrMode {
    #[inline(always)]
    fn from(val: u8) -> CtsuPrMode {
        CtsuPrMode::from_bits(val)
    }
}
impl From<CtsuPrMode> for u8 {
    #[inline(always)]
    fn from(val: CtsuPrMode) -> u8 {
        CtsuPrMode::to_bits(val)
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
pub enum Ctsuicog {
    #[doc = "100 percent gain"]
    _100_PCT = 0x0,
    #[doc = "66 percent gain"]
    _66_PCT = 0x01,
    #[doc = "50 percent gain"]
    _50_PCT = 0x02,
    #[doc = "40 percent gain"]
    _40_PCT = 0x03,
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
