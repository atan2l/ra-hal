#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpb11 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpb11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpb11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpb11 {
    #[inline(always)]
    fn from(val: u8) -> Mstpb11 {
        Mstpb11::from_bits(val)
    }
}
impl From<Mstpb11> for u8 {
    #[inline(always)]
    fn from(val: Mstpb11) -> u8 {
        Mstpb11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpb18 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpb18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpb18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpb18 {
    #[inline(always)]
    fn from(val: u8) -> Mstpb18 {
        Mstpb18::from_bits(val)
    }
}
impl From<Mstpb18> for u8 {
    #[inline(always)]
    fn from(val: Mstpb18) -> u8 {
        Mstpb18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpb19 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpb19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpb19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpb19 {
    #[inline(always)]
    fn from(val: u8) -> Mstpb19 {
        Mstpb19::from_bits(val)
    }
}
impl From<Mstpb19> for u8 {
    #[inline(always)]
    fn from(val: Mstpb19) -> u8 {
        Mstpb19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpb2 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpb2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpb2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpb2 {
    #[inline(always)]
    fn from(val: u8) -> Mstpb2 {
        Mstpb2::from_bits(val)
    }
}
impl From<Mstpb2> for u8 {
    #[inline(always)]
    fn from(val: Mstpb2) -> u8 {
        Mstpb2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpb22 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpb22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpb22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpb22 {
    #[inline(always)]
    fn from(val: u8) -> Mstpb22 {
        Mstpb22::from_bits(val)
    }
}
impl From<Mstpb22> for u8 {
    #[inline(always)]
    fn from(val: Mstpb22) -> u8 {
        Mstpb22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpb29 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpb29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpb29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpb29 {
    #[inline(always)]
    fn from(val: u8) -> Mstpb29 {
        Mstpb29::from_bits(val)
    }
}
impl From<Mstpb29> for u8 {
    #[inline(always)]
    fn from(val: Mstpb29) -> u8 {
        Mstpb29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpb30 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpb30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpb30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpb30 {
    #[inline(always)]
    fn from(val: u8) -> Mstpb30 {
        Mstpb30::from_bits(val)
    }
}
impl From<Mstpb30> for u8 {
    #[inline(always)]
    fn from(val: Mstpb30) -> u8 {
        Mstpb30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpb31 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpb31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpb31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpb31 {
    #[inline(always)]
    fn from(val: u8) -> Mstpb31 {
        Mstpb31::from_bits(val)
    }
}
impl From<Mstpb31> for u8 {
    #[inline(always)]
    fn from(val: Mstpb31) -> u8 {
        Mstpb31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpb8 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpb8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpb8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpb8 {
    #[inline(always)]
    fn from(val: u8) -> Mstpb8 {
        Mstpb8::from_bits(val)
    }
}
impl From<Mstpb8> for u8 {
    #[inline(always)]
    fn from(val: Mstpb8) -> u8 {
        Mstpb8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpb9 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpb9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpb9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpb9 {
    #[inline(always)]
    fn from(val: u8) -> Mstpb9 {
        Mstpb9::from_bits(val)
    }
}
impl From<Mstpb9> for u8 {
    #[inline(always)]
    fn from(val: Mstpb9) -> u8 {
        Mstpb9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpc0 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpc0 {
    #[inline(always)]
    fn from(val: u8) -> Mstpc0 {
        Mstpc0::from_bits(val)
    }
}
impl From<Mstpc0> for u8 {
    #[inline(always)]
    fn from(val: Mstpc0) -> u8 {
        Mstpc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpc1 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpc1 {
    #[inline(always)]
    fn from(val: u8) -> Mstpc1 {
        Mstpc1::from_bits(val)
    }
}
impl From<Mstpc1> for u8 {
    #[inline(always)]
    fn from(val: Mstpc1) -> u8 {
        Mstpc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpc13 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpc13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpc13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpc13 {
    #[inline(always)]
    fn from(val: u8) -> Mstpc13 {
        Mstpc13::from_bits(val)
    }
}
impl From<Mstpc13> for u8 {
    #[inline(always)]
    fn from(val: Mstpc13) -> u8 {
        Mstpc13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpc14 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpc14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpc14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpc14 {
    #[inline(always)]
    fn from(val: u8) -> Mstpc14 {
        Mstpc14::from_bits(val)
    }
}
impl From<Mstpc14> for u8 {
    #[inline(always)]
    fn from(val: Mstpc14) -> u8 {
        Mstpc14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpc3 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpc3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpc3 {
    #[inline(always)]
    fn from(val: u8) -> Mstpc3 {
        Mstpc3::from_bits(val)
    }
}
impl From<Mstpc3> for u8 {
    #[inline(always)]
    fn from(val: Mstpc3) -> u8 {
        Mstpc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpc31 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpc31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpc31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpc31 {
    #[inline(always)]
    fn from(val: u8) -> Mstpc31 {
        Mstpc31::from_bits(val)
    }
}
impl From<Mstpc31> for u8 {
    #[inline(always)]
    fn from(val: Mstpc31) -> u8 {
        Mstpc31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpc4 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpc4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpc4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpc4 {
    #[inline(always)]
    fn from(val: u8) -> Mstpc4 {
        Mstpc4::from_bits(val)
    }
}
impl From<Mstpc4> for u8 {
    #[inline(always)]
    fn from(val: Mstpc4) -> u8 {
        Mstpc4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpc8 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpc8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpc8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpc8 {
    #[inline(always)]
    fn from(val: u8) -> Mstpc8 {
        Mstpc8::from_bits(val)
    }
}
impl From<Mstpc8> for u8 {
    #[inline(always)]
    fn from(val: Mstpc8) -> u8 {
        Mstpc8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpd14 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpd14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpd14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpd14 {
    #[inline(always)]
    fn from(val: u8) -> Mstpd14 {
        Mstpd14::from_bits(val)
    }
}
impl From<Mstpd14> for u8 {
    #[inline(always)]
    fn from(val: Mstpd14) -> u8 {
        Mstpd14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpd16 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpd16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpd16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpd16 {
    #[inline(always)]
    fn from(val: u8) -> Mstpd16 {
        Mstpd16::from_bits(val)
    }
}
impl From<Mstpd16> for u8 {
    #[inline(always)]
    fn from(val: Mstpd16) -> u8 {
        Mstpd16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpd19 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpd19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpd19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpd19 {
    #[inline(always)]
    fn from(val: u8) -> Mstpd19 {
        Mstpd19::from_bits(val)
    }
}
impl From<Mstpd19> for u8 {
    #[inline(always)]
    fn from(val: Mstpd19) -> u8 {
        Mstpd19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpd2 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpd2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpd2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpd2 {
    #[inline(always)]
    fn from(val: u8) -> Mstpd2 {
        Mstpd2::from_bits(val)
    }
}
impl From<Mstpd2> for u8 {
    #[inline(always)]
    fn from(val: Mstpd2) -> u8 {
        Mstpd2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpd20 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpd20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpd20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpd20 {
    #[inline(always)]
    fn from(val: u8) -> Mstpd20 {
        Mstpd20::from_bits(val)
    }
}
impl From<Mstpd20> for u8 {
    #[inline(always)]
    fn from(val: Mstpd20) -> u8 {
        Mstpd20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpd29 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpd29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpd29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpd29 {
    #[inline(always)]
    fn from(val: u8) -> Mstpd29 {
        Mstpd29::from_bits(val)
    }
}
impl From<Mstpd29> for u8 {
    #[inline(always)]
    fn from(val: Mstpd29) -> u8 {
        Mstpd29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpd3 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpd3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpd3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpd3 {
    #[inline(always)]
    fn from(val: u8) -> Mstpd3 {
        Mstpd3::from_bits(val)
    }
}
impl From<Mstpd3> for u8 {
    #[inline(always)]
    fn from(val: Mstpd3) -> u8 {
        Mstpd3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpd31 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpd31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpd31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpd31 {
    #[inline(always)]
    fn from(val: u8) -> Mstpd31 {
        Mstpd31::from_bits(val)
    }
}
impl From<Mstpd31> for u8 {
    #[inline(always)]
    fn from(val: Mstpd31) -> u8 {
        Mstpd31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpd5 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpd5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpd5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpd5 {
    #[inline(always)]
    fn from(val: u8) -> Mstpd5 {
        Mstpd5::from_bits(val)
    }
}
impl From<Mstpd5> for u8 {
    #[inline(always)]
    fn from(val: Mstpd5) -> u8 {
        Mstpd5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpd6 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpd6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpd6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpd6 {
    #[inline(always)]
    fn from(val: u8) -> Mstpd6 {
        Mstpd6::from_bits(val)
    }
}
impl From<Mstpd6> for u8 {
    #[inline(always)]
    fn from(val: Mstpd6) -> u8 {
        Mstpd6::to_bits(val)
    }
}
