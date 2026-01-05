#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fclksel {
    #[doc = "PCLKB"]
    PCLKB_1 = 0x0,
    #[doc = "PCLKB/8"]
    PCLKB_8 = 0x01,
    #[doc = "PCLKB/32"]
    PCLKB_32 = 0x02,
    #[doc = "PCLKB/64"]
    PCLKB_64 = 0x03,
}
impl Fclksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fclksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fclksel {
    #[inline(always)]
    fn from(val: u8) -> Fclksel {
        Fclksel::from_bits(val)
    }
}
impl From<Fclksel> for u8 {
    #[inline(always)]
    fn from(val: Fclksel) -> u8 {
        Fclksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqmd {
    #[doc = "Falling edge"]
    FallingEdge = 0x0,
    #[doc = "Rising edge"]
    RisingEdge = 0x01,
    #[doc = "Rising and falling edges"]
    AnyEdge = 0x02,
    #[doc = "Low level"]
    LowLevel = 0x03,
}
impl Irqmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqmd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqmd {
    #[inline(always)]
    fn from(val: u8) -> Irqmd {
        Irqmd::from_bits(val)
    }
}
impl From<Irqmd> for u8 {
    #[inline(always)]
    fn from(val: Irqmd) -> u8 {
        Irqmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nfclksel {
    #[doc = "PCLKB"]
    PCLKB_1 = 0x0,
    #[doc = "PCLKB/8"]
    PCLKB_8 = 0x01,
    #[doc = "PCLKB/32"]
    PCLKB_32 = 0x02,
    #[doc = "PCLKB/64"]
    PCLKB_64 = 0x03,
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
