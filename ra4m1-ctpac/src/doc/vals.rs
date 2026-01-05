#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcsel {
    #[doc = "DOPCF is set when data mismatch is detected."]
    _0 = 0x0,
    #[doc = "DOPCF is set when data match is detected."]
    _1 = 0x01,
}
impl Dcsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcsel {
    #[inline(always)]
    fn from(val: u8) -> Dcsel {
        Dcsel::from_bits(val)
    }
}
impl From<Dcsel> for u8 {
    #[inline(always)]
    fn from(val: Dcsel) -> u8 {
        Dcsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dopcfcl {
    #[doc = "Maintains the DOPCF flag state."]
    _0 = 0x0,
    #[doc = "Clears the DOPCF flag."]
    _1 = 0x01,
}
impl Dopcfcl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dopcfcl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dopcfcl {
    #[inline(always)]
    fn from(val: u8) -> Dopcfcl {
        Dopcfcl::from_bits(val)
    }
}
impl From<Dopcfcl> for u8 {
    #[inline(always)]
    fn from(val: Dopcfcl) -> u8 {
        Dopcfcl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oms {
    #[doc = "Data comparison mode"]
    _00 = 0x0,
    #[doc = "Data addition mode"]
    _01 = 0x01,
    #[doc = "Data subtraction mode"]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Oms {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oms {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oms {
    #[inline(always)]
    fn from(val: u8) -> Oms {
        Oms::from_bits(val)
    }
}
impl From<Oms> for u8 {
    #[inline(always)]
    fn from(val: Oms) -> u8 {
        Oms::to_bits(val)
    }
}
