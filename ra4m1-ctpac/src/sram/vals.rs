#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum E1stsen {
    #[doc = "Disables updating of the 1-bit ECC error information."]
    _0 = 0x0,
    #[doc = "Enables updating of the 1-bit ECC error information."]
    _1 = 0x01,
}
impl E1stsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> E1stsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for E1stsen {
    #[inline(always)]
    fn from(val: u8) -> E1stsen {
        E1stsen::from_bits(val)
    }
}
impl From<E1stsen> for u8 {
    #[inline(always)]
    fn from(val: E1stsen) -> u8 {
        E1stsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ecc1err {
    #[doc = "No 1-bit ECC error occurred"]
    _0 = 0x0,
    #[doc = "1-bit ECC error occurred"]
    _1 = 0x01,
}
impl Ecc1err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ecc1err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ecc1err {
    #[inline(always)]
    fn from(val: u8) -> Ecc1err {
        Ecc1err::from_bits(val)
    }
}
impl From<Ecc1err> for u8 {
    #[inline(always)]
    fn from(val: Ecc1err) -> u8 {
        Ecc1err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ecc2err {
    #[doc = "No 2-bit ECC error occurred"]
    _0 = 0x0,
    #[doc = "2-bit ECC error occurred."]
    _1 = 0x01,
}
impl Ecc2err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ecc2err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ecc2err {
    #[inline(always)]
    fn from(val: u8) -> Ecc2err {
        Ecc2err::from_bits(val)
    }
}
impl From<Ecc2err> for u8 {
    #[inline(always)]
    fn from(val: Ecc2err) -> u8 {
        Ecc2err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eccmod {
    #[doc = "Disable ECC function"]
    _00 = 0x0,
    #[doc = "Setting prohibited"]
    _01 = 0x01,
    #[doc = "Enable ECC function without error checking"]
    _10 = 0x02,
    #[doc = "Enable ECC function with error checking"]
    _11 = 0x03,
}
impl Eccmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eccmod {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eccmod {
    #[inline(always)]
    fn from(val: u8) -> Eccmod {
        Eccmod::from_bits(val)
    }
}
impl From<Eccmod> for u8 {
    #[inline(always)]
    fn from(val: Eccmod) -> u8 {
        Eccmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EccoadOad {
    #[doc = "Non-maskable interrupt"]
    _0 = 0x0,
    #[doc = "Reset"]
    _1 = 0x01,
}
impl EccoadOad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EccoadOad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EccoadOad {
    #[inline(always)]
    fn from(val: u8) -> EccoadOad {
        EccoadOad::from_bits(val)
    }
}
impl From<EccoadOad> for u8 {
    #[inline(always)]
    fn from(val: EccoadOad) -> u8 {
        EccoadOad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eccprcr {
    #[doc = "Disable writes to the protected registers"]
    _0 = 0x0,
    #[doc = "Enable writes to the protected registers"]
    _1 = 0x01,
}
impl Eccprcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eccprcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eccprcr {
    #[inline(always)]
    fn from(val: u8) -> Eccprcr {
        Eccprcr::from_bits(val)
    }
}
impl From<Eccprcr> for u8 {
    #[inline(always)]
    fn from(val: Eccprcr) -> u8 {
        Eccprcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eccprcr2 {
    #[doc = "Disable writes to the protected registers"]
    _0 = 0x0,
    #[doc = "Enable writes to the protected registers."]
    _1 = 0x01,
}
impl Eccprcr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eccprcr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eccprcr2 {
    #[inline(always)]
    fn from(val: u8) -> Eccprcr2 {
        Eccprcr2::from_bits(val)
    }
}
impl From<Eccprcr2> for u8 {
    #[inline(always)]
    fn from(val: Eccprcr2) -> u8 {
        Eccprcr2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EccprcrKw(u8);
impl EccprcrKw {
    #[doc = "Writing to the ECCRAMPRCR bit is valid, when the KEY bits are written 1111000b."]
    pub const _1111000: Self = Self(0x78);
}
impl EccprcrKw {
    pub const fn from_bits(val: u8) -> EccprcrKw {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for EccprcrKw {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x78 => f.write_str("_1111000"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EccprcrKw {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x78 => defmt::write!(f, "_1111000"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for EccprcrKw {
    #[inline(always)]
    fn from(val: u8) -> EccprcrKw {
        EccprcrKw::from_bits(val)
    }
}
impl From<EccprcrKw> for u8 {
    #[inline(always)]
    fn from(val: EccprcrKw) -> u8 {
        EccprcrKw::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Kw2(u8);
impl Kw2 {
    #[doc = "These bits enable or disable writes to the ECCPRCR2 bit.."]
    pub const _1111000: Self = Self(0x78);
}
impl Kw2 {
    pub const fn from_bits(val: u8) -> Kw2 {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Kw2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x78 => f.write_str("_1111000"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kw2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x78 => defmt::write!(f, "_1111000"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Kw2 {
    #[inline(always)]
    fn from(val: u8) -> Kw2 {
        Kw2::from_bits(val)
    }
}
impl From<Kw2> for u8 {
    #[inline(always)]
    fn from(val: Kw2) -> u8 {
        Kw2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ParioadOad {
    #[doc = "Non maskable interrupt."]
    _0 = 0x0,
    #[doc = "Reset"]
    _1 = 0x01,
}
impl ParioadOad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ParioadOad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ParioadOad {
    #[inline(always)]
    fn from(val: u8) -> ParioadOad {
        ParioadOad::from_bits(val)
    }
}
impl From<ParioadOad> for u8 {
    #[inline(always)]
    fn from(val: ParioadOad) -> u8 {
        ParioadOad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramprcr {
    #[doc = "Disable writes to protected registers"]
    _0 = 0x0,
    #[doc = "Enable writes to protected registers."]
    _1 = 0x01,
}
impl Sramprcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramprcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramprcr {
    #[inline(always)]
    fn from(val: u8) -> Sramprcr {
        Sramprcr::from_bits(val)
    }
}
impl From<Sramprcr> for u8 {
    #[inline(always)]
    fn from(val: Sramprcr) -> u8 {
        Sramprcr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SramprcrKw(u8);
impl SramprcrKw {
    #[doc = "Writing to the RAMPRCR bit is valid, when the KEY bits are written 1111000b."]
    pub const _1111000: Self = Self(0x78);
}
impl SramprcrKw {
    pub const fn from_bits(val: u8) -> SramprcrKw {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for SramprcrKw {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x78 => f.write_str("_1111000"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SramprcrKw {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x78 => defmt::write!(f, "_1111000"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for SramprcrKw {
    #[inline(always)]
    fn from(val: u8) -> SramprcrKw {
        SramprcrKw::from_bits(val)
    }
}
impl From<SramprcrKw> for u8 {
    #[inline(always)]
    fn from(val: SramprcrKw) -> u8 {
        SramprcrKw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstbyp {
    #[doc = "ECC bypass disabled."]
    _0 = 0x0,
    #[doc = "ECC bypass enabled."]
    _1 = 0x01,
}
impl Tstbyp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstbyp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstbyp {
    #[inline(always)]
    fn from(val: u8) -> Tstbyp {
        Tstbyp::from_bits(val)
    }
}
impl From<Tstbyp> for u8 {
    #[inline(always)]
    fn from(val: Tstbyp) -> u8 {
        Tstbyp::to_bits(val)
    }
}
