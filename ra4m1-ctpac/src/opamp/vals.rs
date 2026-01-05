#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ampe0 {
    #[doc = "Operation amplifier is stopped."]
    _0 = 0x0,
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for AGT is enabled."]
    _1 = 0x01,
}
impl Ampe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ampe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ampe0 {
    #[inline(always)]
    fn from(val: u8) -> Ampe0 {
        Ampe0::from_bits(val)
    }
}
impl From<Ampe0> for u8 {
    #[inline(always)]
    fn from(val: Ampe0) -> u8 {
        Ampe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ampe1 {
    #[doc = "Operation amplifier is stopped."]
    _0 = 0x0,
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    _1 = 0x01,
}
impl Ampe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ampe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ampe1 {
    #[inline(always)]
    fn from(val: u8) -> Ampe1 {
        Ampe1::from_bits(val)
    }
}
impl From<Ampe1> for u8 {
    #[inline(always)]
    fn from(val: Ampe1) -> u8 {
        Ampe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ampe2 {
    #[doc = "Operation amplifier is stopped."]
    _0 = 0x0,
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    _1 = 0x01,
}
impl Ampe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ampe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ampe2 {
    #[inline(always)]
    fn from(val: u8) -> Ampe2 {
        Ampe2::from_bits(val)
    }
}
impl From<Ampe2> for u8 {
    #[inline(always)]
    fn from(val: Ampe2) -> u8 {
        Ampe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ampe3 {
    #[doc = "Operation amplifier is stopped."]
    _0 = 0x0,
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    _1 = 0x01,
}
impl Ampe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ampe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ampe3 {
    #[inline(always)]
    fn from(val: u8) -> Ampe3 {
        Ampe3::from_bits(val)
    }
}
impl From<Ampe3> for u8 {
    #[inline(always)]
    fn from(val: Ampe3) -> u8 {
        Ampe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ampmon0 {
    #[doc = "Operational amplifier 0 is stopped."]
    _0 = 0x0,
    #[doc = "Operational amplifier 0 is operating."]
    _1 = 0x01,
}
impl Ampmon0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ampmon0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ampmon0 {
    #[inline(always)]
    fn from(val: u8) -> Ampmon0 {
        Ampmon0::from_bits(val)
    }
}
impl From<Ampmon0> for u8 {
    #[inline(always)]
    fn from(val: Ampmon0) -> u8 {
        Ampmon0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ampmon1 {
    #[doc = "Operational amplifier 1 is stopped."]
    _0 = 0x0,
    #[doc = "Operational amplifier 1 is operating."]
    _1 = 0x01,
}
impl Ampmon1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ampmon1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ampmon1 {
    #[inline(always)]
    fn from(val: u8) -> Ampmon1 {
        Ampmon1::from_bits(val)
    }
}
impl From<Ampmon1> for u8 {
    #[inline(always)]
    fn from(val: Ampmon1) -> u8 {
        Ampmon1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ampmon2 {
    #[doc = "Operational amplifier 2 is stopped."]
    _0 = 0x0,
    #[doc = "Operational amplifier 2 is operating."]
    _1 = 0x01,
}
impl Ampmon2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ampmon2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ampmon2 {
    #[inline(always)]
    fn from(val: u8) -> Ampmon2 {
        Ampmon2::from_bits(val)
    }
}
impl From<Ampmon2> for u8 {
    #[inline(always)]
    fn from(val: Ampmon2) -> u8 {
        Ampmon2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ampmon3 {
    #[doc = "Operational amplifier 3 is stopped."]
    _0 = 0x0,
    #[doc = "Operational amplifier 3 is operating."]
    _1 = 0x01,
}
impl Ampmon3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ampmon3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ampmon3 {
    #[inline(always)]
    fn from(val: u8) -> Ampmon3 {
        Ampmon3::from_bits(val)
    }
}
impl From<Ampmon3> for u8 {
    #[inline(always)]
    fn from(val: Ampmon3) -> u8 {
        Ampmon3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amppc0 {
    #[doc = "Precharging is stopped."]
    _0 = 0x0,
    #[doc = "Precharging is enabled."]
    _1 = 0x01,
}
impl Amppc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amppc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amppc0 {
    #[inline(always)]
    fn from(val: u8) -> Amppc0 {
        Amppc0::from_bits(val)
    }
}
impl From<Amppc0> for u8 {
    #[inline(always)]
    fn from(val: Amppc0) -> u8 {
        Amppc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amppc1 {
    #[doc = "Precharging is stopped."]
    _0 = 0x0,
    #[doc = "Precharging is enabled."]
    _1 = 0x01,
}
impl Amppc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amppc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amppc1 {
    #[inline(always)]
    fn from(val: u8) -> Amppc1 {
        Amppc1::from_bits(val)
    }
}
impl From<Amppc1> for u8 {
    #[inline(always)]
    fn from(val: Amppc1) -> u8 {
        Amppc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amppc2 {
    #[doc = "Precharging is stopped."]
    _0 = 0x0,
    #[doc = "Precharging is enabled."]
    _1 = 0x01,
}
impl Amppc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amppc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amppc2 {
    #[inline(always)]
    fn from(val: u8) -> Amppc2 {
        Amppc2::from_bits(val)
    }
}
impl From<Amppc2> for u8 {
    #[inline(always)]
    fn from(val: Amppc2) -> u8 {
        Amppc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amppc3 {
    #[doc = "Precharging is stopped."]
    _0 = 0x0,
    #[doc = "Precharging is enabled."]
    _1 = 0x01,
}
impl Amppc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amppc3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amppc3 {
    #[inline(always)]
    fn from(val: u8) -> Amppc3 {
        Amppc3::from_bits(val)
    }
}
impl From<Amppc3> for u8 {
    #[inline(always)]
    fn from(val: Amppc3) -> u8 {
        Amppc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ampsp {
    #[doc = "Low-power mode (low-speed)."]
    _0 = 0x0,
    #[doc = "High-speed mode."]
    _1 = 0x01,
}
impl Ampsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ampsp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ampsp {
    #[inline(always)]
    fn from(val: u8) -> Ampsp {
        Ampsp::from_bits(val)
    }
}
impl From<Ampsp> for u8 {
    #[inline(always)]
    fn from(val: Ampsp) -> u8 {
        Ampsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amptrm00 {
    #[doc = "Software trigger mode(AMPTRM01=0)/Setting prohibited(AMPTRM01=1)."]
    _0 = 0x0,
    #[doc = "An activation trigger mode(AMPTRM01=0)/An activation and A/D trigger mode(AMPTRM01=1)."]
    _1 = 0x01,
}
impl Amptrm00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amptrm00 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amptrm00 {
    #[inline(always)]
    fn from(val: u8) -> Amptrm00 {
        Amptrm00::from_bits(val)
    }
}
impl From<Amptrm00> for u8 {
    #[inline(always)]
    fn from(val: Amptrm00) -> u8 {
        Amptrm00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amptrm01 {
    #[doc = "Software trigger mode(AMPTRM00=0)/An activation trigger mode(AMPTRM00=1)."]
    _0 = 0x0,
    #[doc = "Setting prohibited(AMPTRM00=0)/An activation and A/D trigger mode(AMPTRM00=1)."]
    _1 = 0x01,
}
impl Amptrm01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amptrm01 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amptrm01 {
    #[inline(always)]
    fn from(val: u8) -> Amptrm01 {
        Amptrm01::from_bits(val)
    }
}
impl From<Amptrm01> for u8 {
    #[inline(always)]
    fn from(val: Amptrm01) -> u8 {
        Amptrm01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amptrm10 {
    #[doc = "Software trigger mode(AMPTRM11=0)/Setting prohibited(AMPTRM11=1)."]
    _0 = 0x0,
    #[doc = "An activation trigger mode(AMPTRM11=0)/An activation and A/D trigger mode(AMPTRM11=1)."]
    _1 = 0x01,
}
impl Amptrm10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amptrm10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amptrm10 {
    #[inline(always)]
    fn from(val: u8) -> Amptrm10 {
        Amptrm10::from_bits(val)
    }
}
impl From<Amptrm10> for u8 {
    #[inline(always)]
    fn from(val: Amptrm10) -> u8 {
        Amptrm10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amptrm11 {
    #[doc = "Software trigger mode(AMPTRM10=0)/An activation trigger mode(AMPTRM10=1)."]
    _0 = 0x0,
    #[doc = "Setting prohibited(AMPTRM10=0)/An activation and A/D trigger mode(AMPTRM10=1)."]
    _1 = 0x01,
}
impl Amptrm11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amptrm11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amptrm11 {
    #[inline(always)]
    fn from(val: u8) -> Amptrm11 {
        Amptrm11::from_bits(val)
    }
}
impl From<Amptrm11> for u8 {
    #[inline(always)]
    fn from(val: Amptrm11) -> u8 {
        Amptrm11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amptrm20 {
    #[doc = "Software trigger mode(AMPTRM21=0)/Setting prohibited(AMPTRM21=1)."]
    _0 = 0x0,
    #[doc = "An activation trigger mode(AMPTRM21=0)/An activation and A/D trigger mode(AMPTRM21=1)."]
    _1 = 0x01,
}
impl Amptrm20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amptrm20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amptrm20 {
    #[inline(always)]
    fn from(val: u8) -> Amptrm20 {
        Amptrm20::from_bits(val)
    }
}
impl From<Amptrm20> for u8 {
    #[inline(always)]
    fn from(val: Amptrm20) -> u8 {
        Amptrm20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amptrm21 {
    #[doc = "Software trigger mode(AMPTRM20=0)/An activation trigger mode(AMPTRM20=1)."]
    _0 = 0x0,
    #[doc = "Setting prohibited(AMPTRM20=0)/An activation and A/D trigger mode(AMPTRM20=1)."]
    _1 = 0x01,
}
impl Amptrm21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amptrm21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amptrm21 {
    #[inline(always)]
    fn from(val: u8) -> Amptrm21 {
        Amptrm21::from_bits(val)
    }
}
impl From<Amptrm21> for u8 {
    #[inline(always)]
    fn from(val: Amptrm21) -> u8 {
        Amptrm21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amptrm30 {
    #[doc = "Software trigger mode(AMPTRM31=0)/Setting prohibited(AMPTRM31=1)."]
    _0 = 0x0,
    #[doc = "An activation trigger mode(AMPTRM31=0)/An activation and A/D trigger mode(AMPTRM31=1)."]
    _1 = 0x01,
}
impl Amptrm30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amptrm30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amptrm30 {
    #[inline(always)]
    fn from(val: u8) -> Amptrm30 {
        Amptrm30::from_bits(val)
    }
}
impl From<Amptrm30> for u8 {
    #[inline(always)]
    fn from(val: Amptrm30) -> u8 {
        Amptrm30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amptrm31 {
    #[doc = "Software trigger mode(AMPTRM30=0)/An activation trigger mode(AMPTRM30=1)."]
    _0 = 0x0,
    #[doc = "Setting prohibited(AMPTRM30=0)/An activation and A/D trigger mode(AMPTRM30=1)."]
    _1 = 0x01,
}
impl Amptrm31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amptrm31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amptrm31 {
    #[inline(always)]
    fn from(val: u8) -> Amptrm31 {
        Amptrm31::from_bits(val)
    }
}
impl From<Amptrm31> for u8 {
    #[inline(always)]
    fn from(val: Amptrm31) -> u8 {
        Amptrm31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amptrs {
    #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 1.Operational amplifier 2: Operational amplifier An activation trigger 2.Operational amplifier 3: Operational amplifier An activation trigger 3"]
    _00 = 0x0,
    #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 1.Operational amplifier 3: Operational amplifier An activation trigger 1"]
    _01 = 0x01,
    #[doc = "Setting prohibited"]
    _10 = 0x02,
    #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 0.Operational amplifier 3: Operational amplifier An activation trigger 0"]
    _11 = 0x03,
}
impl Amptrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amptrs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amptrs {
    #[inline(always)]
    fn from(val: u8) -> Amptrs {
        Amptrs::from_bits(val)
    }
}
impl From<Amptrs> for u8 {
    #[inline(always)]
    fn from(val: Amptrs) -> u8 {
        Amptrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irefe {
    #[doc = "Operational amplifier reference current circuit is stopped."]
    _0 = 0x0,
    #[doc = "Operation of operational amplifier reference current circuit is enabled."]
    _1 = 0x01,
}
impl Irefe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irefe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irefe {
    #[inline(always)]
    fn from(val: u8) -> Irefe {
        Irefe::from_bits(val)
    }
}
impl From<Irefe> for u8 {
    #[inline(always)]
    fn from(val: Irefe) -> u8 {
        Irefe::to_bits(val)
    }
}
