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
    #[doc = "Hi-Z / JTAG / SWD (0b00000)"]
    HiZ = 0x0,
    #[doc = "Low-Power Asynchronous General Purpose Timer (0b00001)"]
    Agt = 0x01,
    #[doc = "General PWM Timer (0b00010)"]
    Gpt1 = 0x02,
    #[doc = "General PWM Timer (0b00011)"]
    Gpt2 = 0x03,
    #[doc = "Serial Communications Interface (0b00100)"]
    Sci1 = 0x04,
    #[doc = "Serial Communications Interface (0b00101)"]
    Sci2 = 0x05,
    #[doc = "Serial Peripheral Interface (0b00110)"]
    Spi = 0x06,
    #[doc = "Inter-Integrated Circuit Bus Interface (0b00111)"]
    I2c = 0x07,
    #[doc = "Key Interrupt Function (0b01000)"]
    Kint = 0x08,
    #[doc = "CLKOUT / Analog Comparator / Real-Time Clock (0b01001)"]
    ClkCmpRtc = 0x09,
    #[doc = "Clock Frequency Accuracy / ADC14 (0b01010)"]
    CacAdc = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Capacitive Touch (0b01100)"]
    Ctsu = 0x0c,
    #[doc = "Segment LCD (0b01101)"]
    Slcdc = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Controller Area Network Bus (0b10000)"]
    Can = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "Serial Sound Interface Enhanced (0b10010)"]
    Ssie = 0x12,
    #[doc = "USB Full-Speed (0b10011)"]
    UsbFs = 0x13,
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
