#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Kreg {
    #[doc = "Falling edge"]
    _0 = 0x0,
    #[doc = "Rising edge"]
    _1 = 0x01,
}
impl Kreg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Kreg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Kreg {
    #[inline(always)]
    fn from(val: u8) -> Kreg {
        Kreg::from_bits(val)
    }
}
impl From<Kreg> for u8 {
    #[inline(always)]
    fn from(val: Kreg) -> u8 {
        Kreg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krf0 {
    #[doc = "No interrupt detected"]
    _0 = 0x0,
    #[doc = "Interrupt detected."]
    _1 = 0x01,
}
impl Krf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krf0 {
    #[inline(always)]
    fn from(val: u8) -> Krf0 {
        Krf0::from_bits(val)
    }
}
impl From<Krf0> for u8 {
    #[inline(always)]
    fn from(val: Krf0) -> u8 {
        Krf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krf1 {
    #[doc = "No interrupt detected"]
    _0 = 0x0,
    #[doc = "Interrupt detected."]
    _1 = 0x01,
}
impl Krf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krf1 {
    #[inline(always)]
    fn from(val: u8) -> Krf1 {
        Krf1::from_bits(val)
    }
}
impl From<Krf1> for u8 {
    #[inline(always)]
    fn from(val: Krf1) -> u8 {
        Krf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krf2 {
    #[doc = "No interrupt detected"]
    _0 = 0x0,
    #[doc = "Interrupt detected."]
    _1 = 0x01,
}
impl Krf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krf2 {
    #[inline(always)]
    fn from(val: u8) -> Krf2 {
        Krf2::from_bits(val)
    }
}
impl From<Krf2> for u8 {
    #[inline(always)]
    fn from(val: Krf2) -> u8 {
        Krf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krf3 {
    #[doc = "No interrupt detected"]
    _0 = 0x0,
    #[doc = "Interrupt detected."]
    _1 = 0x01,
}
impl Krf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krf3 {
    #[inline(always)]
    fn from(val: u8) -> Krf3 {
        Krf3::from_bits(val)
    }
}
impl From<Krf3> for u8 {
    #[inline(always)]
    fn from(val: Krf3) -> u8 {
        Krf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krf4 {
    #[doc = "No interrupt detected"]
    _0 = 0x0,
    #[doc = "Interrupt detected."]
    _1 = 0x01,
}
impl Krf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krf4 {
    #[inline(always)]
    fn from(val: u8) -> Krf4 {
        Krf4::from_bits(val)
    }
}
impl From<Krf4> for u8 {
    #[inline(always)]
    fn from(val: Krf4) -> u8 {
        Krf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krf5 {
    #[doc = "No interrupt detected"]
    _0 = 0x0,
    #[doc = "Interrupt detected."]
    _1 = 0x01,
}
impl Krf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krf5 {
    #[inline(always)]
    fn from(val: u8) -> Krf5 {
        Krf5::from_bits(val)
    }
}
impl From<Krf5> for u8 {
    #[inline(always)]
    fn from(val: Krf5) -> u8 {
        Krf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krf6 {
    #[doc = "No interrupt detected"]
    _0 = 0x0,
    #[doc = "Interrupt detected."]
    _1 = 0x01,
}
impl Krf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krf6 {
    #[inline(always)]
    fn from(val: u8) -> Krf6 {
        Krf6::from_bits(val)
    }
}
impl From<Krf6> for u8 {
    #[inline(always)]
    fn from(val: Krf6) -> u8 {
        Krf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krf7 {
    #[doc = "No interrupt detected"]
    _0 = 0x0,
    #[doc = "Interrupt detected."]
    _1 = 0x01,
}
impl Krf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krf7 {
    #[inline(always)]
    fn from(val: u8) -> Krf7 {
        Krf7::from_bits(val)
    }
}
impl From<Krf7> for u8 {
    #[inline(always)]
    fn from(val: Krf7) -> u8 {
        Krf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krm0 {
    #[doc = "Does not detect key interrupt signal"]
    _0 = 0x0,
    #[doc = "Detect key interrupt signal."]
    _1 = 0x01,
}
impl Krm0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krm0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krm0 {
    #[inline(always)]
    fn from(val: u8) -> Krm0 {
        Krm0::from_bits(val)
    }
}
impl From<Krm0> for u8 {
    #[inline(always)]
    fn from(val: Krm0) -> u8 {
        Krm0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krm1 {
    #[doc = "Does not detect key interrupt signal"]
    _0 = 0x0,
    #[doc = "Detect key interrupt signal."]
    _1 = 0x01,
}
impl Krm1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krm1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krm1 {
    #[inline(always)]
    fn from(val: u8) -> Krm1 {
        Krm1::from_bits(val)
    }
}
impl From<Krm1> for u8 {
    #[inline(always)]
    fn from(val: Krm1) -> u8 {
        Krm1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krm2 {
    #[doc = "Does not detect key interrupt signal"]
    _0 = 0x0,
    #[doc = "Detect key interrupt signal."]
    _1 = 0x01,
}
impl Krm2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krm2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krm2 {
    #[inline(always)]
    fn from(val: u8) -> Krm2 {
        Krm2::from_bits(val)
    }
}
impl From<Krm2> for u8 {
    #[inline(always)]
    fn from(val: Krm2) -> u8 {
        Krm2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krm3 {
    #[doc = "Does not detect key interrupt signal"]
    _0 = 0x0,
    #[doc = "Detect key interrupt signal."]
    _1 = 0x01,
}
impl Krm3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krm3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krm3 {
    #[inline(always)]
    fn from(val: u8) -> Krm3 {
        Krm3::from_bits(val)
    }
}
impl From<Krm3> for u8 {
    #[inline(always)]
    fn from(val: Krm3) -> u8 {
        Krm3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krm4 {
    #[doc = "Does not detect key interrupt signal"]
    _0 = 0x0,
    #[doc = "Detect key interrupt signal."]
    _1 = 0x01,
}
impl Krm4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krm4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krm4 {
    #[inline(always)]
    fn from(val: u8) -> Krm4 {
        Krm4::from_bits(val)
    }
}
impl From<Krm4> for u8 {
    #[inline(always)]
    fn from(val: Krm4) -> u8 {
        Krm4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krm5 {
    #[doc = "Does not detect key interrupt signal"]
    _0 = 0x0,
    #[doc = "Detect key interrupt signal."]
    _1 = 0x01,
}
impl Krm5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krm5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krm5 {
    #[inline(always)]
    fn from(val: u8) -> Krm5 {
        Krm5::from_bits(val)
    }
}
impl From<Krm5> for u8 {
    #[inline(always)]
    fn from(val: Krm5) -> u8 {
        Krm5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krm6 {
    #[doc = "Does not detect key interrupt signal"]
    _0 = 0x0,
    #[doc = "Detect key interrupt signal."]
    _1 = 0x01,
}
impl Krm6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krm6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krm6 {
    #[inline(always)]
    fn from(val: u8) -> Krm6 {
        Krm6::from_bits(val)
    }
}
impl From<Krm6> for u8 {
    #[inline(always)]
    fn from(val: Krm6) -> u8 {
        Krm6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krm7 {
    #[doc = "Does not detect key interrupt signal"]
    _0 = 0x0,
    #[doc = "Detect key interrupt signal."]
    _1 = 0x01,
}
impl Krm7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krm7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krm7 {
    #[inline(always)]
    fn from(val: u8) -> Krm7 {
        Krm7::from_bits(val)
    }
}
impl From<Krm7> for u8 {
    #[inline(always)]
    fn from(val: Krm7) -> u8 {
        Krm7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krmd {
    #[doc = "Do not use key interrupt flags"]
    _0 = 0x0,
    #[doc = "Use key interrupt flags."]
    _1 = 0x01,
}
impl Krmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krmd {
    #[inline(always)]
    fn from(val: u8) -> Krmd {
        Krmd::from_bits(val)
    }
}
impl From<Krmd> for u8 {
    #[inline(always)]
    fn from(val: Krmd) -> u8 {
        Krmd::to_bits(val)
    }
}
