#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cacrefe {
    #[doc = "Disable"]
    _0 = 0x0,
    #[doc = "Enable"]
    _1 = 0x01,
}
impl Cacrefe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cacrefe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cacrefe {
    #[inline(always)]
    fn from(val: u8) -> Cacrefe {
        Cacrefe::from_bits(val)
    }
}
impl From<Cacrefe> for u8 {
    #[inline(always)]
    fn from(val: Cacrefe) -> u8 {
        Cacrefe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfme {
    #[doc = "Disable"]
    _0 = 0x0,
    #[doc = "Enable"]
    _1 = 0x01,
}
impl Cfme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfme {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfme {
    #[inline(always)]
    fn from(val: u8) -> Cfme {
        Cfme::from_bits(val)
    }
}
impl From<Cfme> for u8 {
    #[inline(always)]
    fn from(val: Cfme) -> u8 {
        Cfme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dfs {
    #[doc = "Digital filtering is disabled."]
    _00 = 0x0,
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock."]
    _01 = 0x01,
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock divided by 4."]
    _10 = 0x02,
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock divided by 16."]
    _11 = 0x03,
}
impl Dfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfs {
    #[inline(always)]
    fn from(val: u8) -> Dfs {
        Dfs::from_bits(val)
    }
}
impl From<Dfs> for u8 {
    #[inline(always)]
    fn from(val: Dfs) -> u8 {
        Dfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edges {
    #[doc = "Rising edge"]
    _00 = 0x0,
    #[doc = "Falling edge"]
    _01 = 0x01,
    #[doc = "Both rising and falling edges"]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Edges {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edges {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edges {
    #[inline(always)]
    fn from(val: u8) -> Edges {
        Edges::from_bits(val)
    }
}
impl From<Edges> for u8 {
    #[inline(always)]
    fn from(val: Edges) -> u8 {
        Edges::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ferrf {
    #[doc = "The clock frequency is within the range corresponding to the settings."]
    _0 = 0x0,
    #[doc = "The clock frequency has deviated beyond the range corresponding to the settings (frequency error)."]
    _1 = 0x01,
}
impl Ferrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ferrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ferrf {
    #[inline(always)]
    fn from(val: u8) -> Ferrf {
        Ferrf::from_bits(val)
    }
}
impl From<Ferrf> for u8 {
    #[inline(always)]
    fn from(val: Ferrf) -> u8 {
        Ferrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ferrfcl {
    #[doc = "No effect on operations"]
    _0 = 0x0,
    #[doc = "Clears the FERRF flag"]
    _1 = 0x01,
}
impl Ferrfcl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ferrfcl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ferrfcl {
    #[inline(always)]
    fn from(val: u8) -> Ferrfcl {
        Ferrfcl::from_bits(val)
    }
}
impl From<Ferrfcl> for u8 {
    #[inline(always)]
    fn from(val: Ferrfcl) -> u8 {
        Ferrfcl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ferrie {
    #[doc = "Disable"]
    _0 = 0x0,
    #[doc = "Enable"]
    _1 = 0x01,
}
impl Ferrie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ferrie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ferrie {
    #[inline(always)]
    fn from(val: u8) -> Ferrie {
        Ferrie::from_bits(val)
    }
}
impl From<Ferrie> for u8 {
    #[inline(always)]
    fn from(val: Ferrie) -> u8 {
        Ferrie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fmcs {
    #[doc = "Main clock"]
    _000 = 0x0,
    #[doc = "Sub-clock"]
    _001 = 0x01,
    #[doc = "HOCO clock"]
    _010 = 0x02,
    #[doc = "MOCO clock"]
    _011 = 0x03,
    #[doc = "LOCO clock"]
    _100 = 0x04,
    #[doc = "Peripheral module clock(PCLKB)"]
    _101 = 0x05,
    #[doc = "IWDTCLK clock"]
    _110 = 0x06,
    #[doc = "Setting prohibited"]
    _111 = 0x07,
}
impl Fmcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmcs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmcs {
    #[inline(always)]
    fn from(val: u8) -> Fmcs {
        Fmcs::from_bits(val)
    }
}
impl From<Fmcs> for u8 {
    #[inline(always)]
    fn from(val: Fmcs) -> u8 {
        Fmcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mendf {
    #[doc = "Measurement is in progress."]
    _0 = 0x0,
    #[doc = "Measurement has ended."]
    _1 = 0x01,
}
impl Mendf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mendf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mendf {
    #[inline(always)]
    fn from(val: u8) -> Mendf {
        Mendf::from_bits(val)
    }
}
impl From<Mendf> for u8 {
    #[inline(always)]
    fn from(val: Mendf) -> u8 {
        Mendf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mendfcl {
    #[doc = "No effect on operations"]
    _0 = 0x0,
    #[doc = "Clears the MENDF flag"]
    _1 = 0x01,
}
impl Mendfcl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mendfcl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mendfcl {
    #[inline(always)]
    fn from(val: u8) -> Mendfcl {
        Mendfcl::from_bits(val)
    }
}
impl From<Mendfcl> for u8 {
    #[inline(always)]
    fn from(val: Mendfcl) -> u8 {
        Mendfcl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mendie {
    #[doc = "Disable"]
    _0 = 0x0,
    #[doc = "Enable"]
    _1 = 0x01,
}
impl Mendie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mendie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mendie {
    #[inline(always)]
    fn from(val: u8) -> Mendie {
        Mendie::from_bits(val)
    }
}
impl From<Mendie> for u8 {
    #[inline(always)]
    fn from(val: Mendie) -> u8 {
        Mendie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ovff {
    #[doc = "The counter has not overflowed."]
    _0 = 0x0,
    #[doc = "The counter has overflowed."]
    _1 = 0x01,
}
impl Ovff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ovff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ovff {
    #[inline(always)]
    fn from(val: u8) -> Ovff {
        Ovff::from_bits(val)
    }
}
impl From<Ovff> for u8 {
    #[inline(always)]
    fn from(val: Ovff) -> u8 {
        Ovff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ovffcl {
    #[doc = "No effect on operations"]
    _0 = 0x0,
    #[doc = "Clears the OVFF flag"]
    _1 = 0x01,
}
impl Ovffcl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ovffcl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ovffcl {
    #[inline(always)]
    fn from(val: u8) -> Ovffcl {
        Ovffcl::from_bits(val)
    }
}
impl From<Ovffcl> for u8 {
    #[inline(always)]
    fn from(val: Ovffcl) -> u8 {
        Ovffcl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ovfie {
    #[doc = "Disable"]
    _0 = 0x0,
    #[doc = "Enable"]
    _1 = 0x01,
}
impl Ovfie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ovfie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ovfie {
    #[inline(always)]
    fn from(val: u8) -> Ovfie {
        Ovfie::from_bits(val)
    }
}
impl From<Ovfie> for u8 {
    #[inline(always)]
    fn from(val: Ovfie) -> u8 {
        Ovfie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcds {
    #[doc = "1/32 clock"]
    _00 = 0x0,
    #[doc = "1/128 clock"]
    _01 = 0x01,
    #[doc = "1/1024 clock"]
    _10 = 0x02,
    #[doc = "1/8192 clock"]
    _11 = 0x03,
}
impl Rcds {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcds {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcds {
    #[inline(always)]
    fn from(val: u8) -> Rcds {
        Rcds::from_bits(val)
    }
}
impl From<Rcds> for u8 {
    #[inline(always)]
    fn from(val: Rcds) -> u8 {
        Rcds::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rps {
    #[doc = "CACREF pin input"]
    _0 = 0x0,
    #[doc = "Internal clock (internally generated signal)"]
    _1 = 0x01,
}
impl Rps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rps {
    #[inline(always)]
    fn from(val: u8) -> Rps {
        Rps::from_bits(val)
    }
}
impl From<Rps> for u8 {
    #[inline(always)]
    fn from(val: Rps) -> u8 {
        Rps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rscs {
    #[doc = "Main clock"]
    _000 = 0x0,
    #[doc = "Sub-clock"]
    _001 = 0x01,
    #[doc = "HOCO clock"]
    _010 = 0x02,
    #[doc = "MOCO clock"]
    _011 = 0x03,
    #[doc = "LOCO clock"]
    _100 = 0x04,
    #[doc = "Peripheral module clock(PCLKB)"]
    _101 = 0x05,
    #[doc = "IWDTCLK clock"]
    _110 = 0x06,
    #[doc = "Setting prohibited"]
    _111 = 0x07,
}
impl Rscs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rscs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rscs {
    #[inline(always)]
    fn from(val: u8) -> Rscs {
        Rscs::from_bits(val)
    }
}
impl From<Rscs> for u8 {
    #[inline(always)]
    fn from(val: Rscs) -> u8 {
        Rscs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcss {
    #[doc = "No division"]
    _00 = 0x0,
    #[doc = "x 1/4 clock"]
    _01 = 0x01,
    #[doc = "x 1/8 clock"]
    _10 = 0x02,
    #[doc = "x 1/32 clock"]
    _11 = 0x03,
}
impl Tcss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcss {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcss {
    #[inline(always)]
    fn from(val: u8) -> Tcss {
        Tcss::from_bits(val)
    }
}
impl From<Tcss> for u8 {
    #[inline(always)]
    fn from(val: Tcss) -> u8 {
        Tcss::to_bits(val)
    }
}
