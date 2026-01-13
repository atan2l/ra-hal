#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dels(u8);
impl Dels {
    #[doc = "Nothing is selected."]
    pub const _0X000: Self = Self(0x0);
}
impl Dels {
    pub const fn from_bits(val: u8) -> Dels {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Dels {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0X000"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dels {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0X000"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Dels {
    #[inline(always)]
    fn from(val: u8) -> Dels {
        Dels::from_bits(val)
    }
}
impl From<Dels> for u8 {
    #[inline(always)]
    fn from(val: Dels) -> u8 {
        Dels::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Iels(u8);
impl Iels {
    #[doc = "Nothing is selected"]
    pub const _0X000: Self = Self(0x0);
}
impl Iels {
    pub const fn from_bits(val: u8) -> Iels {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Iels {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0X000"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iels {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0X000"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Iels {
    #[inline(always)]
    fn from(val: u8) -> Iels {
        Iels::from_bits(val)
    }
}
impl From<Iels> for u8 {
    #[inline(always)]
    fn from(val: Iels) -> u8 {
        Iels::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqcr2Fclksel {
    #[doc = "PCLKB"]
    _00 = 0x0,
    #[doc = "PCLKB/8"]
    _01 = 0x01,
    #[doc = "PCLKB/32"]
    _10 = 0x02,
    #[doc = "PCLKB/64"]
    _11 = 0x03,
}
impl Irqcr2Fclksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqcr2Fclksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqcr2Fclksel {
    #[inline(always)]
    fn from(val: u8) -> Irqcr2Fclksel {
        Irqcr2Fclksel::from_bits(val)
    }
}
impl From<Irqcr2Fclksel> for u8 {
    #[inline(always)]
    fn from(val: Irqcr2Fclksel) -> u8 {
        Irqcr2Fclksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqcr2Irqmd {
    #[doc = "Falling edge"]
    _00 = 0x0,
    #[doc = "Rising edge"]
    _01 = 0x01,
    #[doc = "Rising and falling edges"]
    _10 = 0x02,
    #[doc = "Low level"]
    _11 = 0x03,
}
impl Irqcr2Irqmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqcr2Irqmd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqcr2Irqmd {
    #[inline(always)]
    fn from(val: u8) -> Irqcr2Irqmd {
        Irqcr2Irqmd::from_bits(val)
    }
}
impl From<Irqcr2Irqmd> for u8 {
    #[inline(always)]
    fn from(val: Irqcr2Irqmd) -> u8 {
        Irqcr2Irqmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IrqcrFclksel {
    #[doc = "PCLKB"]
    _00 = 0x0,
    #[doc = "PCLKB/8"]
    _01 = 0x01,
    #[doc = "PCLKB/32"]
    _10 = 0x02,
    #[doc = "PCLKB/64"]
    _11 = 0x03,
}
impl IrqcrFclksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IrqcrFclksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IrqcrFclksel {
    #[inline(always)]
    fn from(val: u8) -> IrqcrFclksel {
        IrqcrFclksel::from_bits(val)
    }
}
impl From<IrqcrFclksel> for u8 {
    #[inline(always)]
    fn from(val: IrqcrFclksel) -> u8 {
        IrqcrFclksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IrqcrIrqmd {
    #[doc = "Falling edge"]
    _00 = 0x0,
    #[doc = "Rising edge"]
    _01 = 0x01,
    #[doc = "Rising and falling edges"]
    _10 = 0x02,
    #[doc = "Low level"]
    _11 = 0x03,
}
impl IrqcrIrqmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IrqcrIrqmd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IrqcrIrqmd {
    #[inline(always)]
    fn from(val: u8) -> IrqcrIrqmd {
        IrqcrIrqmd::from_bits(val)
    }
}
impl From<IrqcrIrqmd> for u8 {
    #[inline(always)]
    fn from(val: IrqcrIrqmd) -> u8 {
        IrqcrIrqmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nfclksel {
    #[doc = "PCLKB"]
    _00 = 0x0,
    #[doc = "PCLKB/8"]
    _01 = 0x01,
    #[doc = "PCLKB/32"]
    _10 = 0x02,
    #[doc = "PCLKB/64"]
    _11 = 0x03,
}
impl Nfclksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfclksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfclksel {
    #[inline(always)]
    fn from(val: u8) -> Nfclksel {
        Nfclksel::from_bits(val)
    }
}
impl From<Nfclksel> for u8 {
    #[inline(always)]
    fn from(val: Nfclksel) -> u8 {
        Nfclksel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sels(u8);
impl Sels {
    #[doc = "Disable event output to the associated low-power mode module"]
    pub const _0X00: Self = Self(0x0);
}
impl Sels {
    pub const fn from_bits(val: u8) -> Sels {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Sels {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0X00"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sels {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0X00"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Sels {
    #[inline(always)]
    fn from(val: u8) -> Sels {
        Sels::from_bits(val)
    }
}
impl From<Sels> for u8 {
    #[inline(always)]
    fn from(val: Sels) -> u8 {
        Sels::to_bits(val)
    }
}
