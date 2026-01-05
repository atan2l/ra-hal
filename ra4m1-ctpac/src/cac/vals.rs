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
    Rising = 0x0,
    #[doc = "Falling edge"]
    Falling = 0x01,
    #[doc = "Both rising and falling edges"]
    Both = 0x02,
    #[doc = "Setting prohibited"]
    RESERVED = 0x03,
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
pub enum Fmcs {
    #[doc = "Main clock"]
    MainClockOscillator = 0x0,
    #[doc = "Sub-clock"]
    SubClockOscillator = 0x01,
    #[doc = "HOCO clock"]
    Hoco = 0x02,
    #[doc = "MOCO clock"]
    Moco = 0x03,
    #[doc = "LOCO clock"]
    Loco = 0x04,
    #[doc = "Peripheral module clock(PCLKB)"]
    PclkB = 0x05,
    #[doc = "IWDTCLK clock"]
    IwdtClk = 0x06,
    #[doc = "Setting prohibited"]
    RESERVED = 0x07,
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
pub enum Rcds {
    #[doc = "1/32 clock"]
    DIV_32 = 0x0,
    #[doc = "1/128 clock"]
    DIV_128 = 0x01,
    #[doc = "1/1024 clock"]
    DIV_1024 = 0x02,
    #[doc = "1/8192 clock"]
    DIV_8192 = 0x03,
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
pub enum Rscs {
    #[doc = "Main clock"]
    MainClockOscillator = 0x0,
    #[doc = "Sub-clock"]
    SubClockOscillator = 0x01,
    #[doc = "HOCO clock"]
    Hoco = 0x02,
    #[doc = "MOCO clock"]
    Moco = 0x03,
    #[doc = "LOCO clock"]
    Loco = 0x04,
    #[doc = "Peripheral module clock(PCLKB)"]
    PclkB = 0x05,
    #[doc = "IWDTCLK clock"]
    IwdtClk = 0x06,
    #[doc = "Setting prohibited"]
    RESERVED = 0x07,
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
    DIV_1 = 0x0,
    #[doc = "x 1/4 clock"]
    DIV_4 = 0x01,
    #[doc = "x 1/8 clock"]
    DIV_8 = 0x02,
    #[doc = "x 1/32 clock"]
    DIV_32 = 0x03,
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
