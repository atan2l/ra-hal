#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdbgpwrupack {
    #[doc = "Debug power-up request is not acknowledged"]
    _0 = 0x0,
    #[doc = "Debug power-up request is acknowledged"]
    _1 = 0x01,
}
impl Cdbgpwrupack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdbgpwrupack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdbgpwrupack {
    #[inline(always)]
    fn from(val: u8) -> Cdbgpwrupack {
        Cdbgpwrupack::from_bits(val)
    }
}
impl From<Cdbgpwrupack> for u8 {
    #[inline(always)]
    fn from(val: Cdbgpwrupack) -> u8 {
        Cdbgpwrupack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdbgpwrupreq {
    #[doc = "OCD is not requesting debug power-up"]
    _0 = 0x0,
    #[doc = "OCD is requesting debug power-up"]
    _1 = 0x01,
}
impl Cdbgpwrupreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdbgpwrupreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdbgpwrupreq {
    #[inline(always)]
    fn from(val: u8) -> Cdbgpwrupreq {
        Cdbgpwrupreq::from_bits(val)
    }
}
impl From<Cdbgpwrupreq> for u8 {
    #[inline(always)]
    fn from(val: Cdbgpwrupreq) -> u8 {
        Cdbgpwrupreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgstopIwdt {
    #[doc = "Mask IWDT reset/interrupt"]
    _0 = 0x0,
    #[doc = "Enable IWDT reset"]
    _1 = 0x01,
}
impl DbgstopIwdt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgstopIwdt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgstopIwdt {
    #[inline(always)]
    fn from(val: u8) -> DbgstopIwdt {
        DbgstopIwdt::from_bits(val)
    }
}
impl From<DbgstopIwdt> for u8 {
    #[inline(always)]
    fn from(val: DbgstopIwdt) -> u8 {
        DbgstopIwdt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgstopReccr {
    #[doc = "Enable RAM ECC error reset/interrupt"]
    _0 = 0x0,
    #[doc = "Mask RAM ECC error reset/interrupt"]
    _1 = 0x01,
}
impl DbgstopReccr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgstopReccr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgstopReccr {
    #[inline(always)]
    fn from(val: u8) -> DbgstopReccr {
        DbgstopReccr::from_bits(val)
    }
}
impl From<DbgstopReccr> for u8 {
    #[inline(always)]
    fn from(val: DbgstopReccr) -> u8 {
        DbgstopReccr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgstopRper {
    #[doc = "Enable RAM parity error reset/interrupt"]
    _0 = 0x0,
    #[doc = "Mask RAM parity error reset/interrupt"]
    _1 = 0x01,
}
impl DbgstopRper {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgstopRper {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgstopRper {
    #[inline(always)]
    fn from(val: u8) -> DbgstopRper {
        DbgstopRper::from_bits(val)
    }
}
impl From<DbgstopRper> for u8 {
    #[inline(always)]
    fn from(val: DbgstopRper) -> u8 {
        DbgstopRper::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgstopWdt {
    #[doc = "Mask WDT reset/interrupt"]
    _0 = 0x0,
    #[doc = "Enable WDT reset"]
    _1 = 0x01,
}
impl DbgstopWdt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgstopWdt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgstopWdt {
    #[inline(always)]
    fn from(val: u8) -> DbgstopWdt {
        DbgstopWdt::from_bits(val)
    }
}
impl From<DbgstopWdt> for u8 {
    #[inline(always)]
    fn from(val: DbgstopWdt) -> u8 {
        DbgstopWdt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enetbfull {
    #[doc = "ETB full does not cause CPU halt"]
    _0 = 0x0,
    #[doc = "ETB full cause CPU halt"]
    _1 = 0x01,
}
impl Enetbfull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enetbfull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enetbfull {
    #[inline(always)]
    fn from(val: u8) -> Enetbfull {
        Enetbfull::from_bits(val)
    }
}
impl From<Enetbfull> for u8 {
    #[inline(always)]
    fn from(val: Enetbfull) -> u8 {
        Enetbfull::to_bits(val)
    }
}
