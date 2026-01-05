#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C0edg {
    #[doc = "Interrupt and ELC event request by one-edge detection"]
    _0 = 0x0,
    #[doc = "Interrupt and ELC event request by both-edge detection"]
    _1 = 0x01,
}
impl C0edg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C0edg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C0edg {
    #[inline(always)]
    fn from(val: u8) -> C0edg {
        C0edg::from_bits(val)
    }
}
impl From<C0edg> for u8 {
    #[inline(always)]
    fn from(val: C0edg) -> u8 {
        C0edg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C0enb {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled"]
    _1 = 0x01,
}
impl C0enb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C0enb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C0enb {
    #[inline(always)]
    fn from(val: u8) -> C0enb {
        C0enb::from_bits(val)
    }
}
impl From<C0enb> for u8 {
    #[inline(always)]
    fn from(val: C0enb) -> u8 {
        C0enb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C0epo {
    #[doc = "Interrupt and ELC event request at rising edge"]
    _0 = 0x0,
    #[doc = "Interrupt and ELC event request at falling edge"]
    _1 = 0x01,
}
impl C0epo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C0epo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C0epo {
    #[inline(always)]
    fn from(val: u8) -> C0epo {
        C0epo::from_bits(val)
    }
}
impl From<C0epo> for u8 {
    #[inline(always)]
    fn from(val: C0epo) -> u8 {
        C0epo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C0fck {
    #[doc = "No Sampling (bypass)"]
    _00 = 0x0,
    #[doc = "Sampling at PCLK"]
    _01 = 0x01,
    #[doc = "Sampling at PCLK/8"]
    _10 = 0x02,
    #[doc = "Sampling at PCLK/32"]
    _11 = 0x03,
}
impl C0fck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C0fck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C0fck {
    #[inline(always)]
    fn from(val: u8) -> C0fck {
        C0fck::from_bits(val)
    }
}
impl From<C0fck> for u8 {
    #[inline(always)]
    fn from(val: C0fck) -> u8 {
        C0fck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C0mon {
    #[doc = "CMPIN0 < CMPREF0, CMPIN0 < internal reference voltage, or ACMPLP0 operation disabled.(When the window function is disabled)/CMPIN0 < VRFL, CMPIN0 > VRFH, or ACMPLP0 operation disabled.(When the window function is enabled)"]
    _0 = 0x0,
    #[doc = "CMPIN0 > CMPREF0, or CMPIN0 > internal reference voltage.(When the window function is disabled)/VRFL < CMPIN0 < VRFH.(When the window function is enabled)"]
    _1 = 0x01,
}
impl C0mon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C0mon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C0mon {
    #[inline(always)]
    fn from(val: u8) -> C0mon {
        C0mon::from_bits(val)
    }
}
impl From<C0mon> for u8 {
    #[inline(always)]
    fn from(val: C0mon) -> u8 {
        C0mon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C0oe {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled"]
    _1 = 0x01,
}
impl C0oe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C0oe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C0oe {
    #[inline(always)]
    fn from(val: u8) -> C0oe {
        C0oe::from_bits(val)
    }
}
impl From<C0oe> for u8 {
    #[inline(always)]
    fn from(val: C0oe) -> u8 {
        C0oe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C0op {
    #[doc = "Non inverted"]
    _0 = 0x0,
    #[doc = "Inverted"]
    _1 = 0x01,
}
impl C0op {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C0op {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C0op {
    #[inline(always)]
    fn from(val: u8) -> C0op {
        C0op::from_bits(val)
    }
}
impl From<C0op> for u8 {
    #[inline(always)]
    fn from(val: C0op) -> u8 {
        C0op::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C0vrf {
    #[doc = "IVREF0"]
    _0 = 0x0,
    #[doc = "internal reference voltage (Vref)"]
    _1 = 0x01,
}
impl C0vrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C0vrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C0vrf {
    #[inline(always)]
    fn from(val: u8) -> C0vrf {
        C0vrf::from_bits(val)
    }
}
impl From<C0vrf> for u8 {
    #[inline(always)]
    fn from(val: C0vrf) -> u8 {
        C0vrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C0wde {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled"]
    _1 = 0x01,
}
impl C0wde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C0wde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C0wde {
    #[inline(always)]
    fn from(val: u8) -> C0wde {
        C0wde::from_bits(val)
    }
}
impl From<C0wde> for u8 {
    #[inline(always)]
    fn from(val: C0wde) -> u8 {
        C0wde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C1edg {
    #[doc = "Interrupt and ELC event request by one-edge detection"]
    _0 = 0x0,
    #[doc = "Interrupt and ELC event request by both-edge detection"]
    _1 = 0x01,
}
impl C1edg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C1edg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C1edg {
    #[inline(always)]
    fn from(val: u8) -> C1edg {
        C1edg::from_bits(val)
    }
}
impl From<C1edg> for u8 {
    #[inline(always)]
    fn from(val: C1edg) -> u8 {
        C1edg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C1enb {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled"]
    _1 = 0x01,
}
impl C1enb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C1enb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C1enb {
    #[inline(always)]
    fn from(val: u8) -> C1enb {
        C1enb::from_bits(val)
    }
}
impl From<C1enb> for u8 {
    #[inline(always)]
    fn from(val: C1enb) -> u8 {
        C1enb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C1epo {
    #[doc = "Interrupt and ELC event request at rising edge"]
    _0 = 0x0,
    #[doc = "Interrupt and ELC event request at falling edge"]
    _1 = 0x01,
}
impl C1epo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C1epo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C1epo {
    #[inline(always)]
    fn from(val: u8) -> C1epo {
        C1epo::from_bits(val)
    }
}
impl From<C1epo> for u8 {
    #[inline(always)]
    fn from(val: C1epo) -> u8 {
        C1epo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C1fck {
    #[doc = "No Sampling (bypass)"]
    _00 = 0x0,
    #[doc = "Sampling at PCLK"]
    _01 = 0x01,
    #[doc = "Sampling at PCLK/8"]
    _10 = 0x02,
    #[doc = "Sampling at PCLK/32"]
    _11 = 0x03,
}
impl C1fck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C1fck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C1fck {
    #[inline(always)]
    fn from(val: u8) -> C1fck {
        C1fck::from_bits(val)
    }
}
impl From<C1fck> for u8 {
    #[inline(always)]
    fn from(val: C1fck) -> u8 {
        C1fck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C1mon {
    #[doc = "CMPIN1 < CMPREF1, CMPIN1 < internal reference voltage, or ACMPLP1 operation disabled.(When the window function is disabled)/CMPIN1 < VRFL, CMPIN1 > VRFH, or ACMPLP1 operation disabled.(When the window function is enabled)"]
    _0 = 0x0,
    #[doc = "CMPIN1 > CMPREF1, or CMPIN1 > internal reference voltage.(When the window function is disabled)/VRFL < CMPIN1 < VRFH.(When the window function is enabled)"]
    _1 = 0x01,
}
impl C1mon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C1mon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C1mon {
    #[inline(always)]
    fn from(val: u8) -> C1mon {
        C1mon::from_bits(val)
    }
}
impl From<C1mon> for u8 {
    #[inline(always)]
    fn from(val: C1mon) -> u8 {
        C1mon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C1oe {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled"]
    _1 = 0x01,
}
impl C1oe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C1oe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C1oe {
    #[inline(always)]
    fn from(val: u8) -> C1oe {
        C1oe::from_bits(val)
    }
}
impl From<C1oe> for u8 {
    #[inline(always)]
    fn from(val: C1oe) -> u8 {
        C1oe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C1op {
    #[doc = "Non inverted"]
    _0 = 0x0,
    #[doc = "Inverted"]
    _1 = 0x01,
}
impl C1op {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C1op {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C1op {
    #[inline(always)]
    fn from(val: u8) -> C1op {
        C1op::from_bits(val)
    }
}
impl From<C1op> for u8 {
    #[inline(always)]
    fn from(val: C1op) -> u8 {
        C1op::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C1vrf {
    #[doc = "IVREF0 or IVREF1"]
    _0 = 0x0,
    #[doc = "internal reference voltage (Vref)"]
    _1 = 0x01,
}
impl C1vrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C1vrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C1vrf {
    #[inline(always)]
    fn from(val: u8) -> C1vrf {
        C1vrf::from_bits(val)
    }
}
impl From<C1vrf> for u8 {
    #[inline(always)]
    fn from(val: C1vrf) -> u8 {
        C1vrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C1vrf2 {
    #[doc = "IVREF0 selected"]
    _0 = 0x0,
    #[doc = "IVREF1 selected."]
    _1 = 0x01,
}
impl C1vrf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C1vrf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C1vrf2 {
    #[inline(always)]
    fn from(val: u8) -> C1vrf2 {
        C1vrf2::from_bits(val)
    }
}
impl From<C1vrf2> for u8 {
    #[inline(always)]
    fn from(val: C1vrf2) -> u8 {
        C1vrf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C1wde {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled"]
    _1 = 0x01,
}
impl C1wde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C1wde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C1wde {
    #[inline(always)]
    fn from(val: u8) -> C1wde {
        C1wde::from_bits(val)
    }
}
impl From<C1wde> for u8 {
    #[inline(always)]
    fn from(val: C1wde) -> u8 {
        C1wde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpsel20 {
    #[doc = "No input"]
    _000 = 0x0,
    #[doc = "CMPIN0 (P100)"]
    _001 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "CMPIN0 (P503)"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cmpsel20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpsel20 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpsel20 {
    #[inline(always)]
    fn from(val: u8) -> Cmpsel20 {
        Cmpsel20::from_bits(val)
    }
}
impl From<Cmpsel20> for u8 {
    #[inline(always)]
    fn from(val: Cmpsel20) -> u8 {
        Cmpsel20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpsel64 {
    #[doc = "No input"]
    _000 = 0x0,
    #[doc = "CMPIN1 (P102)"]
    _001 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "CMPIN1 (P501)"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cmpsel64 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpsel64 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpsel64 {
    #[inline(always)]
    fn from(val: u8) -> Cmpsel64 {
        Cmpsel64::from_bits(val)
    }
}
impl From<Cmpsel64> for u8 {
    #[inline(always)]
    fn from(val: Cmpsel64) -> u8 {
        Cmpsel64::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crvs20 {
    #[doc = "No input"]
    _000 = 0x0,
    #[doc = "CMPREF0 (P101)"]
    _001 = 0x01,
    #[doc = "DAC8 (ch0) output"]
    _010 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "CMPREF0 (P502)"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Crvs20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crvs20 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crvs20 {
    #[inline(always)]
    fn from(val: u8) -> Crvs20 {
        Crvs20::from_bits(val)
    }
}
impl From<Crvs20> for u8 {
    #[inline(always)]
    fn from(val: Crvs20) -> u8 {
        Crvs20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crvs64 {
    #[doc = "No input"]
    _000 = 0x0,
    #[doc = "CMPREF1 (P103)"]
    _001 = 0x01,
    #[doc = "DAC8 (ch1) output"]
    _010 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "CMPREF1 (P500)"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Crvs64 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crvs64 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crvs64 {
    #[inline(always)]
    fn from(val: u8) -> Crvs64 {
        Crvs64::from_bits(val)
    }
}
impl From<Crvs64> for u8 {
    #[inline(always)]
    fn from(val: Crvs64) -> u8 {
        Crvs64::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdmd {
    #[doc = "Comparator low-speed mode"]
    _0 = 0x0,
    #[doc = "Comparator high-speed mode"]
    _1 = 0x01,
}
impl Spdmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdmd {
    #[inline(always)]
    fn from(val: u8) -> Spdmd {
        Spdmd::from_bits(val)
    }
}
impl From<Spdmd> for u8 {
    #[inline(always)]
    fn from(val: Spdmd) -> u8 {
        Spdmd::to_bits(val)
    }
}
