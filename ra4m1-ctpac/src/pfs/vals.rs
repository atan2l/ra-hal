#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutputType {
    #[doc = "CMOS output"]
    Cmos = 0x0,
    #[doc = "NMOS open-drain output"]
    Nmos = 0x01,
}
impl OutputType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutputType {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutputType {
    #[inline(always)]
    fn from(val: u8) -> OutputType {
        OutputType::from_bits(val)
    }
}
impl From<OutputType> for u8 {
    #[inline(always)]
    fn from(val: OutputType) -> u8 {
        OutputType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PortDirection {
    Input = 0x0,
    Output = 0x01,
}
impl PortDirection {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PortDirection {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PortDirection {
    #[inline(always)]
    fn from(val: u8) -> PortDirection {
        PortDirection::from_bits(val)
    }
}
impl From<PortDirection> for u8 {
    #[inline(always)]
    fn from(val: PortDirection) -> u8 {
        PortDirection::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PortDrive {
    Low = 0x0,
    Middle = 0x01,
}
impl PortDrive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PortDrive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PortDrive {
    #[inline(always)]
    fn from(val: u8) -> PortDrive {
        PortDrive::from_bits(val)
    }
}
impl From<PortDrive> for u8 {
    #[inline(always)]
    fn from(val: PortDrive) -> u8 {
        PortDrive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PortFunction {
    HiZ = 0x0,
    Pf1 = 0x01,
    Pf2 = 0x02,
    Pf3 = 0x03,
    Pf4 = 0x04,
    Pf5 = 0x05,
    Pf6 = 0x06,
    Pf7 = 0x07,
    _RESERVED_8 = 0x08,
    Pf9 = 0x09,
    Pf10 = 0x0a,
    _RESERVED_b = 0x0b,
    Pf12 = 0x0c,
    Pf13 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    Pf16 = 0x10,
    _RESERVED_11 = 0x11,
    Pf18 = 0x12,
    Pf19 = 0x13,
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
}
impl PortFunction {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PortFunction {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PortFunction {
    #[inline(always)]
    fn from(val: u8) -> PortFunction {
        PortFunction::from_bits(val)
    }
}
impl From<PortFunction> for u8 {
    #[inline(always)]
    fn from(val: PortFunction) -> u8 {
        PortFunction::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PortLevel {
    Low = 0x0,
    High = 0x01,
}
impl PortLevel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PortLevel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PortLevel {
    #[inline(always)]
    fn from(val: u8) -> PortLevel {
        PortLevel::from_bits(val)
    }
}
impl From<PortLevel> for u8 {
    #[inline(always)]
    fn from(val: PortLevel) -> u8 {
        PortLevel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PortMode {
    Gpio = 0x0,
    Peripheral = 0x01,
}
impl PortMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PortMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PortMode {
    #[inline(always)]
    fn from(val: u8) -> PortMode {
        PortMode::from_bits(val)
    }
}
impl From<PortMode> for u8 {
    #[inline(always)]
    fn from(val: PortMode) -> u8 {
        PortMode::to_bits(val)
    }
}
