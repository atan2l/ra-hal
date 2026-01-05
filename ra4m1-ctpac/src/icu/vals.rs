#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmplp0wupen {
    #[doc = "S/W standby returns by ACMPLP0 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by ACMPLP0 interrupt is enabled"]
    _1 = 0x01,
}
impl Acmplp0wupen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmplp0wupen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmplp0wupen {
    #[inline(always)]
    fn from(val: u8) -> Acmplp0wupen {
        Acmplp0wupen::from_bits(val)
    }
}
impl From<Acmplp0wupen> for u8 {
    #[inline(always)]
    fn from(val: Acmplp0wupen) -> u8 {
        Acmplp0wupen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Agt1cawupen {
    #[doc = "S/W standby returns by AGT1 compare match A interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by AGT1 compare match A interrupt is enabled"]
    _1 = 0x01,
}
impl Agt1cawupen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Agt1cawupen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Agt1cawupen {
    #[inline(always)]
    fn from(val: u8) -> Agt1cawupen {
        Agt1cawupen::from_bits(val)
    }
}
impl From<Agt1cawupen> for u8 {
    #[inline(always)]
    fn from(val: Agt1cawupen) -> u8 {
        Agt1cawupen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Agt1cbwupen {
    #[doc = "S/W standby returns by AGT1 compare match B interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by AGT1 compare match B interrupt is enabled"]
    _1 = 0x01,
}
impl Agt1cbwupen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Agt1cbwupen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Agt1cbwupen {
    #[inline(always)]
    fn from(val: u8) -> Agt1cbwupen {
        Agt1cbwupen::from_bits(val)
    }
}
impl From<Agt1cbwupen> for u8 {
    #[inline(always)]
    fn from(val: Agt1cbwupen) -> u8 {
        Agt1cbwupen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Agt1udwupen {
    #[doc = "S/W standby returns by AGT1 underflow interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by AGT1 underflow interrupt is enabled"]
    _1 = 0x01,
}
impl Agt1udwupen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Agt1udwupen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Agt1udwupen {
    #[inline(always)]
    fn from(val: u8) -> Agt1udwupen {
        Agt1udwupen::from_bits(val)
    }
}
impl From<Agt1udwupen> for u8 {
    #[inline(always)]
    fn from(val: Agt1udwupen) -> u8 {
        Agt1udwupen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busmclr {
    #[doc = "No effect."]
    _0 = 0x0,
    #[doc = "Clear the NMISR.BUSMST flag."]
    _1 = 0x01,
}
impl Busmclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busmclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busmclr {
    #[inline(always)]
    fn from(val: u8) -> Busmclr {
        Busmclr::from_bits(val)
    }
}
impl From<Busmclr> for u8 {
    #[inline(always)]
    fn from(val: Busmclr) -> u8 {
        Busmclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busmen {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled."]
    _1 = 0x01,
}
impl Busmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busmen {
    #[inline(always)]
    fn from(val: u8) -> Busmen {
        Busmen::from_bits(val)
    }
}
impl From<Busmen> for u8 {
    #[inline(always)]
    fn from(val: Busmen) -> u8 {
        Busmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busmst {
    #[doc = "Interrupt not requested"]
    _0 = 0x0,
    #[doc = "Interrupt requested."]
    _1 = 0x01,
}
impl Busmst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busmst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busmst {
    #[inline(always)]
    fn from(val: u8) -> Busmst {
        Busmst::from_bits(val)
    }
}
impl From<Busmst> for u8 {
    #[inline(always)]
    fn from(val: Busmst) -> u8 {
        Busmst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bussclr {
    #[doc = "No effect."]
    _0 = 0x0,
    #[doc = "Clear the NMISR.BUSSST flag."]
    _1 = 0x01,
}
impl Bussclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bussclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bussclr {
    #[inline(always)]
    fn from(val: u8) -> Bussclr {
        Bussclr::from_bits(val)
    }
}
impl From<Bussclr> for u8 {
    #[inline(always)]
    fn from(val: Bussclr) -> u8 {
        Bussclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bussen {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled."]
    _1 = 0x01,
}
impl Bussen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bussen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bussen {
    #[inline(always)]
    fn from(val: u8) -> Bussen {
        Bussen::from_bits(val)
    }
}
impl From<Bussen> for u8 {
    #[inline(always)]
    fn from(val: Bussen) -> u8 {
        Bussen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bussst {
    #[doc = "Interrupt not requested"]
    _0 = 0x0,
    #[doc = "Interrupt requested."]
    _1 = 0x01,
}
impl Bussst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bussst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bussst {
    #[inline(always)]
    fn from(val: u8) -> Bussst {
        Bussst::from_bits(val)
    }
}
impl From<Bussst> for u8 {
    #[inline(always)]
    fn from(val: Bussst) -> u8 {
        Bussst::to_bits(val)
    }
}
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtce {
    #[doc = "DTC activation is disabled"]
    _0 = 0x0,
    #[doc = "DTC activation is enabled"]
    _1 = 0x01,
}
impl Dtce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtce {
    #[inline(always)]
    fn from(val: u8) -> Dtce {
        Dtce::from_bits(val)
    }
}
impl From<Dtce> for u8 {
    #[inline(always)]
    fn from(val: Dtce) -> u8 {
        Dtce::to_bits(val)
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
pub enum Iic0wupen {
    #[doc = "S/W standby returns by IIC0 address match interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IIC0 address match interrupt is enabled"]
    _1 = 0x01,
}
impl Iic0wupen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iic0wupen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iic0wupen {
    #[inline(always)]
    fn from(val: u8) -> Iic0wupen {
        Iic0wupen::from_bits(val)
    }
}
impl From<Iic0wupen> for u8 {
    #[inline(always)]
    fn from(val: Iic0wupen) -> u8 {
        Iic0wupen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ir {
    #[doc = "No interrupt request is generated"]
    _0 = 0x0,
    #[doc = "An interrupt request is generated ( 1 write to the IR bit is prohibited. )"]
    _1 = 0x01,
}
impl Ir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ir {
    #[inline(always)]
    fn from(val: u8) -> Ir {
        Ir::from_bits(val)
    }
}
impl From<Ir> for u8 {
    #[inline(always)]
    fn from(val: Ir) -> u8 {
        Ir::to_bits(val)
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
pub enum Irqcr2Flten {
    #[doc = "Digital filter disabled."]
    _0 = 0x0,
    #[doc = "Digital filter enabled."]
    _1 = 0x01,
}
impl Irqcr2Flten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqcr2Flten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqcr2Flten {
    #[inline(always)]
    fn from(val: u8) -> Irqcr2Flten {
        Irqcr2Flten::from_bits(val)
    }
}
impl From<Irqcr2Flten> for u8 {
    #[inline(always)]
    fn from(val: Irqcr2Flten) -> u8 {
        Irqcr2Flten::to_bits(val)
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
pub enum IrqcrFlten {
    #[doc = "Digital filter disabled."]
    _0 = 0x0,
    #[doc = "Digital filter enabled."]
    _1 = 0x01,
}
impl IrqcrFlten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IrqcrFlten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IrqcrFlten {
    #[inline(always)]
    fn from(val: u8) -> IrqcrFlten {
        IrqcrFlten::from_bits(val)
    }
}
impl From<IrqcrFlten> for u8 {
    #[inline(always)]
    fn from(val: IrqcrFlten) -> u8 {
        IrqcrFlten::to_bits(val)
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
pub enum Irqwupen0 {
    #[doc = "S/W standby returns by IRQ0 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ0 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen0 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen0 {
        Irqwupen0::from_bits(val)
    }
}
impl From<Irqwupen0> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen0) -> u8 {
        Irqwupen0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqwupen1 {
    #[doc = "S/W standby returns by IRQ1 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ1 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen1 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen1 {
        Irqwupen1::from_bits(val)
    }
}
impl From<Irqwupen1> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen1) -> u8 {
        Irqwupen1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqwupen10 {
    #[doc = "S/W standby returns by IRQ10 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ10 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen10 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen10 {
        Irqwupen10::from_bits(val)
    }
}
impl From<Irqwupen10> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen10) -> u8 {
        Irqwupen10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqwupen11 {
    #[doc = "S/W standby returns by IRQ11 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ11 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen11 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen11 {
        Irqwupen11::from_bits(val)
    }
}
impl From<Irqwupen11> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen11) -> u8 {
        Irqwupen11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqwupen12 {
    #[doc = "S/W standby returns by IRQ12 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ12 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen12 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen12 {
        Irqwupen12::from_bits(val)
    }
}
impl From<Irqwupen12> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen12) -> u8 {
        Irqwupen12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqwupen14 {
    #[doc = "S/W standby returns by IRQ14 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ14 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen14 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen14 {
        Irqwupen14::from_bits(val)
    }
}
impl From<Irqwupen14> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen14) -> u8 {
        Irqwupen14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqwupen15 {
    #[doc = "S/W standby returns by IRQ15 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ15 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen15 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen15 {
        Irqwupen15::from_bits(val)
    }
}
impl From<Irqwupen15> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen15) -> u8 {
        Irqwupen15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqwupen2 {
    #[doc = "S/W standby returns by IRQ2 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ2 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen2 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen2 {
        Irqwupen2::from_bits(val)
    }
}
impl From<Irqwupen2> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen2) -> u8 {
        Irqwupen2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqwupen3 {
    #[doc = "S/W standby returns by IRQ3 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ3 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen3 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen3 {
        Irqwupen3::from_bits(val)
    }
}
impl From<Irqwupen3> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen3) -> u8 {
        Irqwupen3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqwupen4 {
    #[doc = "S/W standby returns by IRQ4 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ4 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen4 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen4 {
        Irqwupen4::from_bits(val)
    }
}
impl From<Irqwupen4> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen4) -> u8 {
        Irqwupen4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqwupen5 {
    #[doc = "S/W standby returns by IRQ5 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ5 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen5 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen5 {
        Irqwupen5::from_bits(val)
    }
}
impl From<Irqwupen5> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen5) -> u8 {
        Irqwupen5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqwupen6 {
    #[doc = "S/W standby returns by IRQ6 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ6 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen6 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen6 {
        Irqwupen6::from_bits(val)
    }
}
impl From<Irqwupen6> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen6) -> u8 {
        Irqwupen6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqwupen7 {
    #[doc = "S/W standby returns by IRQ7 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ7 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen7 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen7 {
        Irqwupen7::from_bits(val)
    }
}
impl From<Irqwupen7> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen7) -> u8 {
        Irqwupen7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqwupen8 {
    #[doc = "S/W standby returns by IRQ8 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ8 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen8 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen8 {
        Irqwupen8::from_bits(val)
    }
}
impl From<Irqwupen8> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen8) -> u8 {
        Irqwupen8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqwupen9 {
    #[doc = "S/W standby returns by IRQ9 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IRQ9 interrupt is enabled"]
    _1 = 0x01,
}
impl Irqwupen9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqwupen9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqwupen9 {
    #[inline(always)]
    fn from(val: u8) -> Irqwupen9 {
        Irqwupen9::from_bits(val)
    }
}
impl From<Irqwupen9> for u8 {
    #[inline(always)]
    fn from(val: Irqwupen9) -> u8 {
        Irqwupen9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iwdtclr {
    #[doc = "No effect."]
    _0 = 0x0,
    #[doc = "Clear the NMISR.IWDTST flag."]
    _1 = 0x01,
}
impl Iwdtclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iwdtclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iwdtclr {
    #[inline(always)]
    fn from(val: u8) -> Iwdtclr {
        Iwdtclr::from_bits(val)
    }
}
impl From<Iwdtclr> for u8 {
    #[inline(always)]
    fn from(val: Iwdtclr) -> u8 {
        Iwdtclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iwdten {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled."]
    _1 = 0x01,
}
impl Iwdten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iwdten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iwdten {
    #[inline(always)]
    fn from(val: u8) -> Iwdten {
        Iwdten::from_bits(val)
    }
}
impl From<Iwdten> for u8 {
    #[inline(always)]
    fn from(val: Iwdten) -> u8 {
        Iwdten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iwdtst {
    #[doc = "Interrupt not requested"]
    _0 = 0x0,
    #[doc = "Interrupt requested."]
    _1 = 0x01,
}
impl Iwdtst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iwdtst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iwdtst {
    #[inline(always)]
    fn from(val: u8) -> Iwdtst {
        Iwdtst::from_bits(val)
    }
}
impl From<Iwdtst> for u8 {
    #[inline(always)]
    fn from(val: Iwdtst) -> u8 {
        Iwdtst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iwdtwupen {
    #[doc = "S/W standby returns by IWDT interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by IWDT interrupt is enabled"]
    _1 = 0x01,
}
impl Iwdtwupen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iwdtwupen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iwdtwupen {
    #[inline(always)]
    fn from(val: u8) -> Iwdtwupen {
        Iwdtwupen::from_bits(val)
    }
}
impl From<Iwdtwupen> for u8 {
    #[inline(always)]
    fn from(val: Iwdtwupen) -> u8 {
        Iwdtwupen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Keywupen {
    #[doc = "S/W standby returns by KEY interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by KEY interrupt is enabled"]
    _1 = 0x01,
}
impl Keywupen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Keywupen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Keywupen {
    #[inline(always)]
    fn from(val: u8) -> Keywupen {
        Keywupen::from_bits(val)
    }
}
impl From<Keywupen> for u8 {
    #[inline(always)]
    fn from(val: Keywupen) -> u8 {
        Keywupen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvd1clr {
    #[doc = "No effect."]
    _0 = 0x0,
    #[doc = "Clear the NMISR.LVD1ST flag."]
    _1 = 0x01,
}
impl Lvd1clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd1clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd1clr {
    #[inline(always)]
    fn from(val: u8) -> Lvd1clr {
        Lvd1clr::from_bits(val)
    }
}
impl From<Lvd1clr> for u8 {
    #[inline(always)]
    fn from(val: Lvd1clr) -> u8 {
        Lvd1clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvd1en {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled."]
    _1 = 0x01,
}
impl Lvd1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd1en {
    #[inline(always)]
    fn from(val: u8) -> Lvd1en {
        Lvd1en::from_bits(val)
    }
}
impl From<Lvd1en> for u8 {
    #[inline(always)]
    fn from(val: Lvd1en) -> u8 {
        Lvd1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvd1st {
    #[doc = "Interrupt not requested"]
    _0 = 0x0,
    #[doc = "Interrupt requested."]
    _1 = 0x01,
}
impl Lvd1st {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd1st {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd1st {
    #[inline(always)]
    fn from(val: u8) -> Lvd1st {
        Lvd1st::from_bits(val)
    }
}
impl From<Lvd1st> for u8 {
    #[inline(always)]
    fn from(val: Lvd1st) -> u8 {
        Lvd1st::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvd1wupen {
    #[doc = "S/W standby returns by LVD1 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by LVD1 interrupt is enabled"]
    _1 = 0x01,
}
impl Lvd1wupen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd1wupen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd1wupen {
    #[inline(always)]
    fn from(val: u8) -> Lvd1wupen {
        Lvd1wupen::from_bits(val)
    }
}
impl From<Lvd1wupen> for u8 {
    #[inline(always)]
    fn from(val: Lvd1wupen) -> u8 {
        Lvd1wupen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvd2clr {
    #[doc = "No effect."]
    _0 = 0x0,
    #[doc = "Clear the NMISR.LVD2ST flag."]
    _1 = 0x01,
}
impl Lvd2clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd2clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd2clr {
    #[inline(always)]
    fn from(val: u8) -> Lvd2clr {
        Lvd2clr::from_bits(val)
    }
}
impl From<Lvd2clr> for u8 {
    #[inline(always)]
    fn from(val: Lvd2clr) -> u8 {
        Lvd2clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvd2en {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled."]
    _1 = 0x01,
}
impl Lvd2en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd2en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd2en {
    #[inline(always)]
    fn from(val: u8) -> Lvd2en {
        Lvd2en::from_bits(val)
    }
}
impl From<Lvd2en> for u8 {
    #[inline(always)]
    fn from(val: Lvd2en) -> u8 {
        Lvd2en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvd2st {
    #[doc = "Interrupt not requested"]
    _0 = 0x0,
    #[doc = "Interrupt requested."]
    _1 = 0x01,
}
impl Lvd2st {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd2st {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd2st {
    #[inline(always)]
    fn from(val: u8) -> Lvd2st {
        Lvd2st::from_bits(val)
    }
}
impl From<Lvd2st> for u8 {
    #[inline(always)]
    fn from(val: Lvd2st) -> u8 {
        Lvd2st::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvd2wupen {
    #[doc = "S/W standby returns by LVD2 interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by LVD2 interrupt is enabled"]
    _1 = 0x01,
}
impl Lvd2wupen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd2wupen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd2wupen {
    #[inline(always)]
    fn from(val: u8) -> Lvd2wupen {
        Lvd2wupen::from_bits(val)
    }
}
impl From<Lvd2wupen> for u8 {
    #[inline(always)]
    fn from(val: Lvd2wupen) -> u8 {
        Lvd2wupen::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nflten {
    #[doc = "Digital filter is disabled."]
    _0 = 0x0,
    #[doc = "Digital filter is enabled."]
    _1 = 0x01,
}
impl Nflten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nflten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nflten {
    #[inline(always)]
    fn from(val: u8) -> Nflten {
        Nflten::from_bits(val)
    }
}
impl From<Nflten> for u8 {
    #[inline(always)]
    fn from(val: Nflten) -> u8 {
        Nflten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nmiclr {
    #[doc = "No effect."]
    _0 = 0x0,
    #[doc = "Clear the NMISR.NMIST flag."]
    _1 = 0x01,
}
impl Nmiclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nmiclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nmiclr {
    #[inline(always)]
    fn from(val: u8) -> Nmiclr {
        Nmiclr::from_bits(val)
    }
}
impl From<Nmiclr> for u8 {
    #[inline(always)]
    fn from(val: Nmiclr) -> u8 {
        Nmiclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nmien {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled."]
    _1 = 0x01,
}
impl Nmien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nmien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nmien {
    #[inline(always)]
    fn from(val: u8) -> Nmien {
        Nmien::from_bits(val)
    }
}
impl From<Nmien> for u8 {
    #[inline(always)]
    fn from(val: Nmien) -> u8 {
        Nmien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nmimd {
    #[doc = "Falling edge"]
    _0 = 0x0,
    #[doc = "Rising edge"]
    _1 = 0x01,
}
impl Nmimd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nmimd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nmimd {
    #[inline(always)]
    fn from(val: u8) -> Nmimd {
        Nmimd::from_bits(val)
    }
}
impl From<Nmimd> for u8 {
    #[inline(always)]
    fn from(val: Nmimd) -> u8 {
        Nmimd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nmist {
    #[doc = "Interrupt not requested"]
    _0 = 0x0,
    #[doc = "Interrupt requested."]
    _1 = 0x01,
}
impl Nmist {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nmist {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nmist {
    #[inline(always)]
    fn from(val: u8) -> Nmist {
        Nmist::from_bits(val)
    }
}
impl From<Nmist> for u8 {
    #[inline(always)]
    fn from(val: Nmist) -> u8 {
        Nmist::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ostclr {
    #[doc = "No effect."]
    _0 = 0x0,
    #[doc = "Clear the NMISR.OSTST flag."]
    _1 = 0x01,
}
impl Ostclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ostclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ostclr {
    #[inline(always)]
    fn from(val: u8) -> Ostclr {
        Ostclr::from_bits(val)
    }
}
impl From<Ostclr> for u8 {
    #[inline(always)]
    fn from(val: Ostclr) -> u8 {
        Ostclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osten {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled."]
    _1 = 0x01,
}
impl Osten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Osten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Osten {
    #[inline(always)]
    fn from(val: u8) -> Osten {
        Osten::from_bits(val)
    }
}
impl From<Osten> for u8 {
    #[inline(always)]
    fn from(val: Osten) -> u8 {
        Osten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ostst {
    #[doc = "Interrupt not requested for main oscillation stop"]
    _0 = 0x0,
    #[doc = "Interrupt requested for main oscillation stop."]
    _1 = 0x01,
}
impl Ostst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ostst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ostst {
    #[inline(always)]
    fn from(val: u8) -> Ostst {
        Ostst::from_bits(val)
    }
}
impl From<Ostst> for u8 {
    #[inline(always)]
    fn from(val: Ostst) -> u8 {
        Ostst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reccclr {
    #[doc = "No effect."]
    _0 = 0x0,
    #[doc = "Clear the NMISR.RECCST flag."]
    _1 = 0x01,
}
impl Reccclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reccclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reccclr {
    #[inline(always)]
    fn from(val: u8) -> Reccclr {
        Reccclr::from_bits(val)
    }
}
impl From<Reccclr> for u8 {
    #[inline(always)]
    fn from(val: Reccclr) -> u8 {
        Reccclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reccen {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled."]
    _1 = 0x01,
}
impl Reccen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reccen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reccen {
    #[inline(always)]
    fn from(val: u8) -> Reccen {
        Reccen::from_bits(val)
    }
}
impl From<Reccen> for u8 {
    #[inline(always)]
    fn from(val: Reccen) -> u8 {
        Reccen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reccst {
    #[doc = "Interrupt not requested"]
    _0 = 0x0,
    #[doc = "Interrupt requested."]
    _1 = 0x01,
}
impl Reccst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reccst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reccst {
    #[inline(always)]
    fn from(val: u8) -> Reccst {
        Reccst::from_bits(val)
    }
}
impl From<Reccst> for u8 {
    #[inline(always)]
    fn from(val: Reccst) -> u8 {
        Reccst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rpeclr {
    #[doc = "No effect."]
    _0 = 0x0,
    #[doc = "Clear the NMISR.RPEST flag."]
    _1 = 0x01,
}
impl Rpeclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rpeclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rpeclr {
    #[inline(always)]
    fn from(val: u8) -> Rpeclr {
        Rpeclr::from_bits(val)
    }
}
impl From<Rpeclr> for u8 {
    #[inline(always)]
    fn from(val: Rpeclr) -> u8 {
        Rpeclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rpeen {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled."]
    _1 = 0x01,
}
impl Rpeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rpeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rpeen {
    #[inline(always)]
    fn from(val: u8) -> Rpeen {
        Rpeen::from_bits(val)
    }
}
impl From<Rpeen> for u8 {
    #[inline(always)]
    fn from(val: Rpeen) -> u8 {
        Rpeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rpest {
    #[doc = "Interrupt not requested"]
    _0 = 0x0,
    #[doc = "Interrupt requested."]
    _1 = 0x01,
}
impl Rpest {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rpest {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rpest {
    #[inline(always)]
    fn from(val: u8) -> Rpest {
        Rpest::from_bits(val)
    }
}
impl From<Rpest> for u8 {
    #[inline(always)]
    fn from(val: Rpest) -> u8 {
        Rpest::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtcalmwupen {
    #[doc = "S/W standby returns by RTC alarm interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by RTC alarm interrupt is enabled"]
    _1 = 0x01,
}
impl Rtcalmwupen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtcalmwupen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtcalmwupen {
    #[inline(always)]
    fn from(val: u8) -> Rtcalmwupen {
        Rtcalmwupen::from_bits(val)
    }
}
impl From<Rtcalmwupen> for u8 {
    #[inline(always)]
    fn from(val: Rtcalmwupen) -> u8 {
        Rtcalmwupen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtcprdwupen {
    #[doc = "S/W standby returns by RTC period interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by RTC period interrupt is enabled"]
    _1 = 0x01,
}
impl Rtcprdwupen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtcprdwupen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtcprdwupen {
    #[inline(always)]
    fn from(val: u8) -> Rtcprdwupen {
        Rtcprdwupen::from_bits(val)
    }
}
impl From<Rtcprdwupen> for u8 {
    #[inline(always)]
    fn from(val: Rtcprdwupen) -> u8 {
        Rtcprdwupen::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Speclr {
    #[doc = "No effect."]
    _0 = 0x0,
    #[doc = "Clear the NMISR.SPEST flag."]
    _1 = 0x01,
}
impl Speclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Speclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Speclr {
    #[inline(always)]
    fn from(val: u8) -> Speclr {
        Speclr::from_bits(val)
    }
}
impl From<Speclr> for u8 {
    #[inline(always)]
    fn from(val: Speclr) -> u8 {
        Speclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Speen {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled."]
    _1 = 0x01,
}
impl Speen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Speen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Speen {
    #[inline(always)]
    fn from(val: u8) -> Speen {
        Speen::from_bits(val)
    }
}
impl From<Speen> for u8 {
    #[inline(always)]
    fn from(val: Speen) -> u8 {
        Speen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spest {
    #[doc = "Interrupt not requested"]
    _0 = 0x0,
    #[doc = "Interrupt requested."]
    _1 = 0x01,
}
impl Spest {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spest {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spest {
    #[inline(always)]
    fn from(val: u8) -> Spest {
        Spest::from_bits(val)
    }
}
impl From<Spest> for u8 {
    #[inline(always)]
    fn from(val: Spest) -> u8 {
        Spest::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbfswupen {
    #[doc = "S/W standby returns by USBFS interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by USBFS interrupt is enabled"]
    _1 = 0x01,
}
impl Usbfswupen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbfswupen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbfswupen {
    #[inline(always)]
    fn from(val: u8) -> Usbfswupen {
        Usbfswupen::from_bits(val)
    }
}
impl From<Usbfswupen> for u8 {
    #[inline(always)]
    fn from(val: Usbfswupen) -> u8 {
        Usbfswupen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbattclr {
    #[doc = "No effect."]
    _0 = 0x0,
    #[doc = "Clear the NMISR.VBATTST flag."]
    _1 = 0x01,
}
impl Vbattclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbattclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbattclr {
    #[inline(always)]
    fn from(val: u8) -> Vbattclr {
        Vbattclr::from_bits(val)
    }
}
impl From<Vbattclr> for u8 {
    #[inline(always)]
    fn from(val: Vbattclr) -> u8 {
        Vbattclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbatten {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled."]
    _1 = 0x01,
}
impl Vbatten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbatten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbatten {
    #[inline(always)]
    fn from(val: u8) -> Vbatten {
        Vbatten::from_bits(val)
    }
}
impl From<Vbatten> for u8 {
    #[inline(always)]
    fn from(val: Vbatten) -> u8 {
        Vbatten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbattst {
    #[doc = "Interrupt not requested"]
    _0 = 0x0,
    #[doc = "Interrupt requested."]
    _1 = 0x01,
}
impl Vbattst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbattst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbattst {
    #[inline(always)]
    fn from(val: u8) -> Vbattst {
        Vbattst::from_bits(val)
    }
}
impl From<Vbattst> for u8 {
    #[inline(always)]
    fn from(val: Vbattst) -> u8 {
        Vbattst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbattwupen {
    #[doc = "S/W standby returns by VBATT monitor interrupt is disabled"]
    _0 = 0x0,
    #[doc = "S/W standby returns by VBATT monitor interrupt is enabled"]
    _1 = 0x01,
}
impl Vbattwupen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbattwupen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbattwupen {
    #[inline(always)]
    fn from(val: u8) -> Vbattwupen {
        Vbattwupen::from_bits(val)
    }
}
impl From<Vbattwupen> for u8 {
    #[inline(always)]
    fn from(val: Vbattwupen) -> u8 {
        Vbattwupen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdtclr {
    #[doc = "No effect."]
    _0 = 0x0,
    #[doc = "Clear the NMISR.WDTST flag."]
    _1 = 0x01,
}
impl Wdtclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdtclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdtclr {
    #[inline(always)]
    fn from(val: u8) -> Wdtclr {
        Wdtclr::from_bits(val)
    }
}
impl From<Wdtclr> for u8 {
    #[inline(always)]
    fn from(val: Wdtclr) -> u8 {
        Wdtclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdten {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled."]
    _1 = 0x01,
}
impl Wdten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdten {
    #[inline(always)]
    fn from(val: u8) -> Wdten {
        Wdten::from_bits(val)
    }
}
impl From<Wdten> for u8 {
    #[inline(always)]
    fn from(val: Wdten) -> u8 {
        Wdten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdtst {
    #[doc = "Interrupt not requested"]
    _0 = 0x0,
    #[doc = "Interrupt requested."]
    _1 = 0x01,
}
impl Wdtst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdtst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdtst {
    #[inline(always)]
    fn from(val: u8) -> Wdtst {
        Wdtst::from_bits(val)
    }
}
impl From<Wdtst> for u8 {
    #[inline(always)]
    fn from(val: Wdtst) -> u8 {
        Wdtst::to_bits(val)
    }
}
