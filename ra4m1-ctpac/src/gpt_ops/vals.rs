#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Align {
    #[doc = "Input phase is aligned to PCLK."]
    _0 = 0x0,
    #[doc = "Input phase is aligned PWM."]
    _1 = 0x01,
}
impl Align {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Align {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Align {
    #[inline(always)]
    fn from(val: u8) -> Align {
        Align::from_bits(val)
    }
}
impl From<Align> for u8 {
    #[inline(always)]
    fn from(val: Align) -> u8 {
        Align::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En {
    #[doc = "Not Output(Hi-Z external terminals)."]
    _0 = 0x0,
    #[doc = "Output"]
    _1 = 0x01,
}
impl En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En {
    #[inline(always)]
    fn from(val: u8) -> En {
        En::from_bits(val)
    }
}
impl From<En> for u8 {
    #[inline(always)]
    fn from(val: En) -> u8 {
        En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fb {
    #[doc = "Select the external input."]
    _0 = 0x0,
    #[doc = "Select the soft setting(OPSCR.UF, VF, WF)."]
    _1 = 0x01,
}
impl Fb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fb {
    #[inline(always)]
    fn from(val: u8) -> Fb {
        Fb::from_bits(val)
    }
}
impl From<Fb> for u8 {
    #[inline(always)]
    fn from(val: Fb) -> u8 {
        Fb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Godf {
    #[doc = "This bit function is ignored."]
    _0 = 0x0,
    #[doc = "Group disable will clear OPSCR.EN Bit."]
    _1 = 0x01,
}
impl Godf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Godf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Godf {
    #[inline(always)]
    fn from(val: u8) -> Godf {
        Godf::from_bits(val)
    }
}
impl From<Godf> for u8 {
    #[inline(always)]
    fn from(val: Godf) -> u8 {
        Godf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Grp {
    #[doc = "Select Group A output disable source"]
    _00 = 0x0,
    #[doc = "Select Group B output disable source"]
    _01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Grp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Grp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Grp {
    #[inline(always)]
    fn from(val: u8) -> Grp {
        Grp::from_bits(val)
    }
}
impl From<Grp> for u8 {
    #[inline(always)]
    fn from(val: Grp) -> u8 {
        Grp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inv {
    #[doc = "Positive Logic (Active High)output"]
    _0 = 0x0,
    #[doc = "Negative Logic (Active Low)output"]
    _1 = 0x01,
}
impl Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inv {
    #[inline(always)]
    fn from(val: u8) -> Inv {
        Inv::from_bits(val)
    }
}
impl From<Inv> for u8 {
    #[inline(always)]
    fn from(val: Inv) -> u8 {
        Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum N {
    #[doc = "Level signal output"]
    _0 = 0x0,
    #[doc = "PWM signal output (PWM of GPT0)"]
    _1 = 0x01,
}
impl N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> N {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for N {
    #[inline(always)]
    fn from(val: u8) -> N {
        N::from_bits(val)
    }
}
impl From<N> for u8 {
    #[inline(always)]
    fn from(val: N) -> u8 {
        N::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nfcs {
    #[doc = "PCLK/1"]
    _00 = 0x0,
    #[doc = "PCLK/4"]
    _01 = 0x01,
    #[doc = "PCLK/16"]
    _10 = 0x02,
    #[doc = "PCLK/64"]
    _11 = 0x03,
}
impl Nfcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfcs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfcs {
    #[inline(always)]
    fn from(val: u8) -> Nfcs {
        Nfcs::from_bits(val)
    }
}
impl From<Nfcs> for u8 {
    #[inline(always)]
    fn from(val: Nfcs) -> u8 {
        Nfcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nfen {
    #[doc = "Do not use a noise filter to the external input."]
    _0 = 0x0,
    #[doc = "Use a noise filter to the external input."]
    _1 = 0x01,
}
impl Nfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfen {
    #[inline(always)]
    fn from(val: u8) -> Nfen {
        Nfen::from_bits(val)
    }
}
impl From<Nfen> for u8 {
    #[inline(always)]
    fn from(val: Nfen) -> u8 {
        Nfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P {
    #[doc = "Level signal output"]
    _0 = 0x0,
    #[doc = "PWM signal output (PWM of GPT0)"]
    _1 = 0x01,
}
impl P {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P {
    #[inline(always)]
    fn from(val: u8) -> P {
        P::from_bits(val)
    }
}
impl From<P> for u8 {
    #[inline(always)]
    fn from(val: P) -> u8 {
        P::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rv {
    #[doc = "U/V/W-Phase output"]
    _0 = 0x0,
    #[doc = "Output to reverse the V / W-phase"]
    _1 = 0x01,
}
impl Rv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rv {
    #[inline(always)]
    fn from(val: u8) -> Rv {
        Rv::from_bits(val)
    }
}
impl From<Rv> for u8 {
    #[inline(always)]
    fn from(val: Rv) -> u8 {
        Rv::to_bits(val)
    }
}
