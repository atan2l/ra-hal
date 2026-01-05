#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ascafbh {
    #[doc = "GTCCRA input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 0x01,
}
impl Ascafbh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ascafbh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ascafbh {
    #[inline(always)]
    fn from(val: u8) -> Ascafbh {
        Ascafbh::from_bits(val)
    }
}
impl From<Ascafbh> for u8 {
    #[inline(always)]
    fn from(val: Ascafbh) -> u8 {
        Ascafbh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ascafbl {
    #[doc = "GTCCRA input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 0x01,
}
impl Ascafbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ascafbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ascafbl {
    #[inline(always)]
    fn from(val: u8) -> Ascafbl {
        Ascafbl::from_bits(val)
    }
}
impl From<Ascafbl> for u8 {
    #[inline(always)]
    fn from(val: Ascafbl) -> u8 {
        Ascafbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ascarbh {
    #[doc = "GTCCRA input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 0x01,
}
impl Ascarbh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ascarbh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ascarbh {
    #[inline(always)]
    fn from(val: u8) -> Ascarbh {
        Ascarbh::from_bits(val)
    }
}
impl From<Ascarbh> for u8 {
    #[inline(always)]
    fn from(val: Ascarbh) -> u8 {
        Ascarbh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ascarbl {
    #[doc = "GTCCRA input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 0x01,
}
impl Ascarbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ascarbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ascarbl {
    #[inline(always)]
    fn from(val: u8) -> Ascarbl {
        Ascarbl::from_bits(val)
    }
}
impl From<Ascarbl> for u8 {
    #[inline(always)]
    fn from(val: Ascarbl) -> u8 {
        Ascarbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ascbfah {
    #[doc = "GTCCRA input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 0x01,
}
impl Ascbfah {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ascbfah {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ascbfah {
    #[inline(always)]
    fn from(val: u8) -> Ascbfah {
        Ascbfah::from_bits(val)
    }
}
impl From<Ascbfah> for u8 {
    #[inline(always)]
    fn from(val: Ascbfah) -> u8 {
        Ascbfah::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ascbfal {
    #[doc = "GTCCRA input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 0x01,
}
impl Ascbfal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ascbfal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ascbfal {
    #[inline(always)]
    fn from(val: u8) -> Ascbfal {
        Ascbfal::from_bits(val)
    }
}
impl From<Ascbfal> for u8 {
    #[inline(always)]
    fn from(val: Ascbfal) -> u8 {
        Ascbfal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ascbrah {
    #[doc = "GTCCRA input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 0x01,
}
impl Ascbrah {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ascbrah {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ascbrah {
    #[inline(always)]
    fn from(val: u8) -> Ascbrah {
        Ascbrah::from_bits(val)
    }
}
impl From<Ascbrah> for u8 {
    #[inline(always)]
    fn from(val: Ascbrah) -> u8 {
        Ascbrah::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ascbral {
    #[doc = "GTCCRA input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 0x01,
}
impl Ascbral {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ascbral {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ascbral {
    #[inline(always)]
    fn from(val: u8) -> Ascbral {
        Ascbral::from_bits(val)
    }
}
impl From<Ascbral> for u8 {
    #[inline(always)]
    fn from(val: Ascbral) -> u8 {
        Ascbral::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aselca {
    #[doc = "GTCCRA input capture is disable at the ELC_GPTA input"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the ELC_GPTA input"]
    _1 = 0x01,
}
impl Aselca {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aselca {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aselca {
    #[inline(always)]
    fn from(val: u8) -> Aselca {
        Aselca::from_bits(val)
    }
}
impl From<Aselca> for u8 {
    #[inline(always)]
    fn from(val: Aselca) -> u8 {
        Aselca::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aselcb {
    #[doc = "GTCCRA input capture is disable at the ELC_GPTB input"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the ELC_GPTB input"]
    _1 = 0x01,
}
impl Aselcb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aselcb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aselcb {
    #[inline(always)]
    fn from(val: u8) -> Aselcb {
        Aselcb::from_bits(val)
    }
}
impl From<Aselcb> for u8 {
    #[inline(always)]
    fn from(val: Aselcb) -> u8 {
        Aselcb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aselcc {
    #[doc = "GTCCRA input capture is disable at the ELC_GPTC input"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the ELC_GPTC input"]
    _1 = 0x01,
}
impl Aselcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aselcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aselcc {
    #[inline(always)]
    fn from(val: u8) -> Aselcc {
        Aselcc::from_bits(val)
    }
}
impl From<Aselcc> for u8 {
    #[inline(always)]
    fn from(val: Aselcc) -> u8 {
        Aselcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aselcd {
    #[doc = "GTCCRA input capture is disable at the ELC_GPTD input"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the ELC_GPTD input"]
    _1 = 0x01,
}
impl Aselcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aselcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aselcd {
    #[inline(always)]
    fn from(val: u8) -> Aselcd {
        Aselcd::from_bits(val)
    }
}
impl From<Aselcd> for u8 {
    #[inline(always)]
    fn from(val: Aselcd) -> u8 {
        Aselcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aselce {
    #[doc = "GTCCRA input capture is disable at the ELC_GPTE input"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the ELC_GPTE input"]
    _1 = 0x01,
}
impl Aselce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aselce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aselce {
    #[inline(always)]
    fn from(val: u8) -> Aselce {
        Aselce::from_bits(val)
    }
}
impl From<Aselce> for u8 {
    #[inline(always)]
    fn from(val: Aselce) -> u8 {
        Aselce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aselcf {
    #[doc = "GTCCRA input capture is disable at the ELC_GPTF input"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the ELC_GPTF input"]
    _1 = 0x01,
}
impl Aselcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aselcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aselcf {
    #[inline(always)]
    fn from(val: u8) -> Aselcf {
        Aselcf::from_bits(val)
    }
}
impl From<Aselcf> for u8 {
    #[inline(always)]
    fn from(val: Aselcf) -> u8 {
        Aselcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aselcg {
    #[doc = "GTCCRA input capture is disable at the ELC_GPTG input"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the ELC_GPTG input"]
    _1 = 0x01,
}
impl Aselcg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aselcg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aselcg {
    #[inline(always)]
    fn from(val: u8) -> Aselcg {
        Aselcg::from_bits(val)
    }
}
impl From<Aselcg> for u8 {
    #[inline(always)]
    fn from(val: Aselcg) -> u8 {
        Aselcg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aselch {
    #[doc = "GTCCRA input capture is disable at the ELC_GPTH input"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the ELC_GPTH input"]
    _1 = 0x01,
}
impl Aselch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aselch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aselch {
    #[inline(always)]
    fn from(val: u8) -> Aselch {
        Aselch::from_bits(val)
    }
}
impl From<Aselch> for u8 {
    #[inline(always)]
    fn from(val: Aselch) -> u8 {
        Aselch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Asgtrgaf {
    #[doc = "GTCCRA input capture is disable at the falling edge of GTETRGA input"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the falling edge of GTETRGA input"]
    _1 = 0x01,
}
impl Asgtrgaf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Asgtrgaf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Asgtrgaf {
    #[inline(always)]
    fn from(val: u8) -> Asgtrgaf {
        Asgtrgaf::from_bits(val)
    }
}
impl From<Asgtrgaf> for u8 {
    #[inline(always)]
    fn from(val: Asgtrgaf) -> u8 {
        Asgtrgaf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Asgtrgar {
    #[doc = "GTCCRA input capture is disable at the rising edge of GTETRGA input"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the rising edge of GTETRGA input"]
    _1 = 0x01,
}
impl Asgtrgar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Asgtrgar {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Asgtrgar {
    #[inline(always)]
    fn from(val: u8) -> Asgtrgar {
        Asgtrgar::from_bits(val)
    }
}
impl From<Asgtrgar> for u8 {
    #[inline(always)]
    fn from(val: Asgtrgar) -> u8 {
        Asgtrgar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Asgtrgbf {
    #[doc = "GTCCRA input capture is disable at the falling edge of GTETRGB input"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the falling edge of GTETRGB input"]
    _1 = 0x01,
}
impl Asgtrgbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Asgtrgbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Asgtrgbf {
    #[inline(always)]
    fn from(val: u8) -> Asgtrgbf {
        Asgtrgbf::from_bits(val)
    }
}
impl From<Asgtrgbf> for u8 {
    #[inline(always)]
    fn from(val: Asgtrgbf) -> u8 {
        Asgtrgbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Asgtrgbr {
    #[doc = "GTCCRA input capture is disable at the rising edge of GTETRGB input"]
    _0 = 0x0,
    #[doc = "GTCCRA input capture is enable at the rising edge of GTETRGB input"]
    _1 = 0x01,
}
impl Asgtrgbr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Asgtrgbr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Asgtrgbr {
    #[inline(always)]
    fn from(val: u8) -> Asgtrgbr {
        Asgtrgbr::from_bits(val)
    }
}
impl From<Asgtrgbr> for u8 {
    #[inline(always)]
    fn from(val: Asgtrgbr) -> u8 {
        Asgtrgbr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bd {
    #[doc = "Buffer operation is enabled"]
    _0 = 0x0,
    #[doc = "Buffer operation is disabled"]
    _1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Bd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bd {
    #[inline(always)]
    fn from(val: u8) -> Bd {
        Bd::from_bits(val)
    }
}
impl From<Bd> for u8 {
    #[inline(always)]
    fn from(val: Bd) -> u8 {
        Bd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bscafbh {
    #[doc = "GTCCRB input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 0x01,
}
impl Bscafbh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bscafbh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bscafbh {
    #[inline(always)]
    fn from(val: u8) -> Bscafbh {
        Bscafbh::from_bits(val)
    }
}
impl From<Bscafbh> for u8 {
    #[inline(always)]
    fn from(val: Bscafbh) -> u8 {
        Bscafbh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bscafbl {
    #[doc = "GTCCRB input capture is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 0x01,
}
impl Bscafbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bscafbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bscafbl {
    #[inline(always)]
    fn from(val: u8) -> Bscafbl {
        Bscafbl::from_bits(val)
    }
}
impl From<Bscafbl> for u8 {
    #[inline(always)]
    fn from(val: Bscafbl) -> u8 {
        Bscafbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bscarbh {
    #[doc = "GTCCRB input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 0x01,
}
impl Bscarbh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bscarbh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bscarbh {
    #[inline(always)]
    fn from(val: u8) -> Bscarbh {
        Bscarbh::from_bits(val)
    }
}
impl From<Bscarbh> for u8 {
    #[inline(always)]
    fn from(val: Bscarbh) -> u8 {
        Bscarbh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bscarbl {
    #[doc = "GTCCRB input capture is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 0x01,
}
impl Bscarbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bscarbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bscarbl {
    #[inline(always)]
    fn from(val: u8) -> Bscarbl {
        Bscarbl::from_bits(val)
    }
}
impl From<Bscarbl> for u8 {
    #[inline(always)]
    fn from(val: Bscarbl) -> u8 {
        Bscarbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bscbfah {
    #[doc = "GTCCRB input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 0x01,
}
impl Bscbfah {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bscbfah {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bscbfah {
    #[inline(always)]
    fn from(val: u8) -> Bscbfah {
        Bscbfah::from_bits(val)
    }
}
impl From<Bscbfah> for u8 {
    #[inline(always)]
    fn from(val: Bscbfah) -> u8 {
        Bscbfah::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bscbfal {
    #[doc = "GTCCRB input capture is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 0x01,
}
impl Bscbfal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bscbfal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bscbfal {
    #[inline(always)]
    fn from(val: u8) -> Bscbfal {
        Bscbfal::from_bits(val)
    }
}
impl From<Bscbfal> for u8 {
    #[inline(always)]
    fn from(val: Bscbfal) -> u8 {
        Bscbfal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bscbrah {
    #[doc = "GTCCRB input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 0x01,
}
impl Bscbrah {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bscbrah {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bscbrah {
    #[inline(always)]
    fn from(val: u8) -> Bscbrah {
        Bscbrah::from_bits(val)
    }
}
impl From<Bscbrah> for u8 {
    #[inline(always)]
    fn from(val: Bscbrah) -> u8 {
        Bscbrah::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bscbral {
    #[doc = "GTCCRB input capture is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 0x01,
}
impl Bscbral {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bscbral {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bscbral {
    #[inline(always)]
    fn from(val: u8) -> Bscbral {
        Bscbral::from_bits(val)
    }
}
impl From<Bscbral> for u8 {
    #[inline(always)]
    fn from(val: Bscbral) -> u8 {
        Bscbral::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bselca {
    #[doc = "GTCCRB input capture is disable at the ELC_GPTA input"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the ELC_GPTA input"]
    _1 = 0x01,
}
impl Bselca {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bselca {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bselca {
    #[inline(always)]
    fn from(val: u8) -> Bselca {
        Bselca::from_bits(val)
    }
}
impl From<Bselca> for u8 {
    #[inline(always)]
    fn from(val: Bselca) -> u8 {
        Bselca::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bselcb {
    #[doc = "GTCCRB input capture is disable at the ELC_GPTB input"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the ELC_GPTB input"]
    _1 = 0x01,
}
impl Bselcb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bselcb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bselcb {
    #[inline(always)]
    fn from(val: u8) -> Bselcb {
        Bselcb::from_bits(val)
    }
}
impl From<Bselcb> for u8 {
    #[inline(always)]
    fn from(val: Bselcb) -> u8 {
        Bselcb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bselcc {
    #[doc = "GTCCRB input capture is disable at the ELC_GPTC input"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the ELC_GPTC input"]
    _1 = 0x01,
}
impl Bselcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bselcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bselcc {
    #[inline(always)]
    fn from(val: u8) -> Bselcc {
        Bselcc::from_bits(val)
    }
}
impl From<Bselcc> for u8 {
    #[inline(always)]
    fn from(val: Bselcc) -> u8 {
        Bselcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bselcd {
    #[doc = "GTCCRB input capture is disable at the ELC_GPTD input"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the ELC_GPTD input"]
    _1 = 0x01,
}
impl Bselcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bselcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bselcd {
    #[inline(always)]
    fn from(val: u8) -> Bselcd {
        Bselcd::from_bits(val)
    }
}
impl From<Bselcd> for u8 {
    #[inline(always)]
    fn from(val: Bselcd) -> u8 {
        Bselcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bselce {
    #[doc = "GTCCRB input capture is disable at the ELC_GPTE input"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the ELC_GPTE input"]
    _1 = 0x01,
}
impl Bselce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bselce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bselce {
    #[inline(always)]
    fn from(val: u8) -> Bselce {
        Bselce::from_bits(val)
    }
}
impl From<Bselce> for u8 {
    #[inline(always)]
    fn from(val: Bselce) -> u8 {
        Bselce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bselcf {
    #[doc = "GTCCRB input capture is disable at the ELC_GPTF input"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the ELC_GPTF input"]
    _1 = 0x01,
}
impl Bselcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bselcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bselcf {
    #[inline(always)]
    fn from(val: u8) -> Bselcf {
        Bselcf::from_bits(val)
    }
}
impl From<Bselcf> for u8 {
    #[inline(always)]
    fn from(val: Bselcf) -> u8 {
        Bselcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bselcg {
    #[doc = "GTCCRB input capture is disable at the ELC_GPTG input"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the ELC_GPTG input"]
    _1 = 0x01,
}
impl Bselcg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bselcg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bselcg {
    #[inline(always)]
    fn from(val: u8) -> Bselcg {
        Bselcg::from_bits(val)
    }
}
impl From<Bselcg> for u8 {
    #[inline(always)]
    fn from(val: Bselcg) -> u8 {
        Bselcg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bselch {
    #[doc = "GTCCRB input capture is disable at the ELC_GPTH input"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the ELC_GPTH input"]
    _1 = 0x01,
}
impl Bselch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bselch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bselch {
    #[inline(always)]
    fn from(val: u8) -> Bselch {
        Bselch::from_bits(val)
    }
}
impl From<Bselch> for u8 {
    #[inline(always)]
    fn from(val: Bselch) -> u8 {
        Bselch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bsgtrgaf {
    #[doc = "GTCCRB input capture is disable at the falling edge of GTETRGA input"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the falling edge of GTETRGA input"]
    _1 = 0x01,
}
impl Bsgtrgaf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bsgtrgaf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bsgtrgaf {
    #[inline(always)]
    fn from(val: u8) -> Bsgtrgaf {
        Bsgtrgaf::from_bits(val)
    }
}
impl From<Bsgtrgaf> for u8 {
    #[inline(always)]
    fn from(val: Bsgtrgaf) -> u8 {
        Bsgtrgaf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bsgtrgar {
    #[doc = "GTCCRB input capture is disable at the rising edge of GTETRGA input"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the rising edge of GTETRGA input"]
    _1 = 0x01,
}
impl Bsgtrgar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bsgtrgar {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bsgtrgar {
    #[inline(always)]
    fn from(val: u8) -> Bsgtrgar {
        Bsgtrgar::from_bits(val)
    }
}
impl From<Bsgtrgar> for u8 {
    #[inline(always)]
    fn from(val: Bsgtrgar) -> u8 {
        Bsgtrgar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bsgtrgbf {
    #[doc = "GTCCRB input capture is disable at the falling edge of GTETRGB input"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the falling edge of GTETRGB input"]
    _1 = 0x01,
}
impl Bsgtrgbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bsgtrgbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bsgtrgbf {
    #[inline(always)]
    fn from(val: u8) -> Bsgtrgbf {
        Bsgtrgbf::from_bits(val)
    }
}
impl From<Bsgtrgbf> for u8 {
    #[inline(always)]
    fn from(val: Bsgtrgbf) -> u8 {
        Bsgtrgbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bsgtrgbr {
    #[doc = "GTCCRB input capture is disable at the rising edge of GTETRGB input"]
    _0 = 0x0,
    #[doc = "GTCCRB input capture is enable at the rising edge of GTETRGB input"]
    _1 = 0x01,
}
impl Bsgtrgbr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bsgtrgbr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bsgtrgbr {
    #[inline(always)]
    fn from(val: u8) -> Bsgtrgbr {
        Bsgtrgbr::from_bits(val)
    }
}
impl From<Bsgtrgbr> for u8 {
    #[inline(always)]
    fn from(val: Bsgtrgbr) -> u8 {
        Bsgtrgbr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cclr {
    #[doc = "Counter clear is disable by the GTCLR register"]
    _0 = 0x0,
    #[doc = "Counter clear is enable by the GTCLR register"]
    _1 = 0x01,
}
impl Cclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cclr {
    #[inline(always)]
    fn from(val: u8) -> Cclr {
        Cclr::from_bits(val)
    }
}
impl From<Cclr> for u8 {
    #[inline(always)]
    fn from(val: Cclr) -> u8 {
        Cclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cclr0 {
    #[doc = "No effect"]
    _0 = 0x0,
    #[doc = "GPT320.GTCNT counter clears"]
    _1 = 0x01,
}
impl Cclr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cclr0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cclr0 {
    #[inline(always)]
    fn from(val: u8) -> Cclr0 {
        Cclr0::from_bits(val)
    }
}
impl From<Cclr0> for u8 {
    #[inline(always)]
    fn from(val: Cclr0) -> u8 {
        Cclr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cclr1 {
    #[doc = "No effect"]
    _0 = 0x0,
    #[doc = "GPT321.GTCNT counter clears"]
    _1 = 0x01,
}
impl Cclr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cclr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cclr1 {
    #[inline(always)]
    fn from(val: u8) -> Cclr1 {
        Cclr1::from_bits(val)
    }
}
impl From<Cclr1> for u8 {
    #[inline(always)]
    fn from(val: Cclr1) -> u8 {
        Cclr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cclr2 {
    #[doc = "No effect"]
    _0 = 0x0,
    #[doc = "GPT322.GTCNT counter clears"]
    _1 = 0x01,
}
impl Cclr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cclr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cclr2 {
    #[inline(always)]
    fn from(val: u8) -> Cclr2 {
        Cclr2::from_bits(val)
    }
}
impl From<Cclr2> for u8 {
    #[inline(always)]
    fn from(val: Cclr2) -> u8 {
        Cclr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cclr3 {
    #[doc = "No effect"]
    _0 = 0x0,
    #[doc = "GPT323.GTCNT counter clears"]
    _1 = 0x01,
}
impl Cclr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cclr3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cclr3 {
    #[inline(always)]
    fn from(val: u8) -> Cclr3 {
        Cclr3::from_bits(val)
    }
}
impl From<Cclr3> for u8 {
    #[inline(always)]
    fn from(val: Cclr3) -> u8 {
        Cclr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cclr4 {
    #[doc = "No effect"]
    _0 = 0x0,
    #[doc = "GPT164.GTCNT counter clears"]
    _1 = 0x01,
}
impl Cclr4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cclr4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cclr4 {
    #[inline(always)]
    fn from(val: u8) -> Cclr4 {
        Cclr4::from_bits(val)
    }
}
impl From<Cclr4> for u8 {
    #[inline(always)]
    fn from(val: Cclr4) -> u8 {
        Cclr4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cclr5 {
    #[doc = "No effect"]
    _0 = 0x0,
    #[doc = "GPT165.GTCNT counter clears"]
    _1 = 0x01,
}
impl Cclr5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cclr5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cclr5 {
    #[inline(always)]
    fn from(val: u8) -> Cclr5 {
        Cclr5::from_bits(val)
    }
}
impl From<Cclr5> for u8 {
    #[inline(always)]
    fn from(val: Cclr5) -> u8 {
        Cclr5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cclr6 {
    #[doc = "No effect"]
    _0 = 0x0,
    #[doc = "GPT166.GTCNT counter clears"]
    _1 = 0x01,
}
impl Cclr6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cclr6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cclr6 {
    #[inline(always)]
    fn from(val: u8) -> Cclr6 {
        Cclr6::from_bits(val)
    }
}
impl From<Cclr6> for u8 {
    #[inline(always)]
    fn from(val: Cclr6) -> u8 {
        Cclr6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cclr7 {
    #[doc = "No effect"]
    _0 = 0x0,
    #[doc = "GPT167.GTCNT counter clears"]
    _1 = 0x01,
}
impl Cclr7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cclr7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cclr7 {
    #[inline(always)]
    fn from(val: u8) -> Cclr7 {
        Cclr7::from_bits(val)
    }
}
impl From<Cclr7> for u8 {
    #[inline(always)]
    fn from(val: Cclr7) -> u8 {
        Cclr7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccra {
    #[doc = "Buffer operation is not performed"]
    _00 = 0x0,
    #[doc = "Single buffer operation (GTCCRA <--> GTCCRC)"]
    _01 = 0x01,
    #[doc = "Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
    _10 = 0x02,
    #[doc = "Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
    _11 = 0x03,
}
impl Ccra {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccra {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccra {
    #[inline(always)]
    fn from(val: u8) -> Ccra {
        Ccra::from_bits(val)
    }
}
impl From<Ccra> for u8 {
    #[inline(always)]
    fn from(val: Ccra) -> u8 {
        Ccra::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccrb {
    #[doc = "Buffer operation is not performed"]
    _00 = 0x0,
    #[doc = "Single buffer operation (GTCCRB <--> GTCCRE)"]
    _01 = 0x01,
    #[doc = "Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
    _10 = 0x02,
    #[doc = "Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
    _11 = 0x03,
}
impl Ccrb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccrb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccrb {
    #[inline(always)]
    fn from(val: u8) -> Ccrb {
        Ccrb::from_bits(val)
    }
}
impl From<Ccrb> for u8 {
    #[inline(always)]
    fn from(val: Ccrb) -> u8 {
        Ccrb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccrswt {
    #[doc = "no effect"]
    _0 = 0x0,
    #[doc = "Forcibly performs buffer transfer of GTCCRA and GTCCRB. This bit automatically returns to 0 after the writing of 1."]
    _1 = 0x01,
}
impl Ccrswt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccrswt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccrswt {
    #[inline(always)]
    fn from(val: u8) -> Ccrswt {
        Ccrswt::from_bits(val)
    }
}
impl From<Ccrswt> for u8 {
    #[inline(always)]
    fn from(val: Ccrswt) -> u8 {
        Ccrswt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cscafbh {
    #[doc = "Counter clear is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 0x01,
}
impl Cscafbh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cscafbh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cscafbh {
    #[inline(always)]
    fn from(val: u8) -> Cscafbh {
        Cscafbh::from_bits(val)
    }
}
impl From<Cscafbh> for u8 {
    #[inline(always)]
    fn from(val: Cscafbh) -> u8 {
        Cscafbh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cscafbl {
    #[doc = "Counter clear is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 0x01,
}
impl Cscafbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cscafbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cscafbl {
    #[inline(always)]
    fn from(val: u8) -> Cscafbl {
        Cscafbl::from_bits(val)
    }
}
impl From<Cscafbl> for u8 {
    #[inline(always)]
    fn from(val: Cscafbl) -> u8 {
        Cscafbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cscarbh {
    #[doc = "Counter clear is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 0x01,
}
impl Cscarbh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cscarbh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cscarbh {
    #[inline(always)]
    fn from(val: u8) -> Cscarbh {
        Cscarbh::from_bits(val)
    }
}
impl From<Cscarbh> for u8 {
    #[inline(always)]
    fn from(val: Cscarbh) -> u8 {
        Cscarbh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cscarbl {
    #[doc = "Counter clear is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 0x01,
}
impl Cscarbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cscarbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cscarbl {
    #[inline(always)]
    fn from(val: u8) -> Cscarbl {
        Cscarbl::from_bits(val)
    }
}
impl From<Cscarbl> for u8 {
    #[inline(always)]
    fn from(val: Cscarbl) -> u8 {
        Cscarbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cscbfah {
    #[doc = "Counter clear is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 0x01,
}
impl Cscbfah {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cscbfah {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cscbfah {
    #[inline(always)]
    fn from(val: u8) -> Cscbfah {
        Cscbfah::from_bits(val)
    }
}
impl From<Cscbfah> for u8 {
    #[inline(always)]
    fn from(val: Cscbfah) -> u8 {
        Cscbfah::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cscbfal {
    #[doc = "Counter clear is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 0x01,
}
impl Cscbfal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cscbfal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cscbfal {
    #[inline(always)]
    fn from(val: u8) -> Cscbfal {
        Cscbfal::from_bits(val)
    }
}
impl From<Cscbfal> for u8 {
    #[inline(always)]
    fn from(val: Cscbfal) -> u8 {
        Cscbfal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cscbrah {
    #[doc = "Counter clear is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 0x01,
}
impl Cscbrah {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cscbrah {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cscbrah {
    #[inline(always)]
    fn from(val: u8) -> Cscbrah {
        Cscbrah::from_bits(val)
    }
}
impl From<Cscbrah> for u8 {
    #[inline(always)]
    fn from(val: Cscbrah) -> u8 {
        Cscbrah::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cscbral {
    #[doc = "Counter clear is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 0x01,
}
impl Cscbral {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cscbral {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cscbral {
    #[inline(always)]
    fn from(val: u8) -> Cscbral {
        Cscbral::from_bits(val)
    }
}
impl From<Cscbral> for u8 {
    #[inline(always)]
    fn from(val: Cscbral) -> u8 {
        Cscbral::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cselca {
    #[doc = "Counter clear is disable at the ELC_GPTA input"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the ELC_GPTA input"]
    _1 = 0x01,
}
impl Cselca {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cselca {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cselca {
    #[inline(always)]
    fn from(val: u8) -> Cselca {
        Cselca::from_bits(val)
    }
}
impl From<Cselca> for u8 {
    #[inline(always)]
    fn from(val: Cselca) -> u8 {
        Cselca::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cselcb {
    #[doc = "Counter clear is disable at the ELC_GPTB input"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the ELC_GPTB input"]
    _1 = 0x01,
}
impl Cselcb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cselcb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cselcb {
    #[inline(always)]
    fn from(val: u8) -> Cselcb {
        Cselcb::from_bits(val)
    }
}
impl From<Cselcb> for u8 {
    #[inline(always)]
    fn from(val: Cselcb) -> u8 {
        Cselcb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cselcc {
    #[doc = "Counter clear is disable at the ELC_GPTC input"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the ELC_GPTC input"]
    _1 = 0x01,
}
impl Cselcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cselcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cselcc {
    #[inline(always)]
    fn from(val: u8) -> Cselcc {
        Cselcc::from_bits(val)
    }
}
impl From<Cselcc> for u8 {
    #[inline(always)]
    fn from(val: Cselcc) -> u8 {
        Cselcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cselcd {
    #[doc = "Counter clear is disable at the ELC_GPTD input"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the ELC_GPTD input"]
    _1 = 0x01,
}
impl Cselcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cselcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cselcd {
    #[inline(always)]
    fn from(val: u8) -> Cselcd {
        Cselcd::from_bits(val)
    }
}
impl From<Cselcd> for u8 {
    #[inline(always)]
    fn from(val: Cselcd) -> u8 {
        Cselcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cselce {
    #[doc = "Counter clear is disable at the ELC_GPTE input"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the ELC_GPTE input"]
    _1 = 0x01,
}
impl Cselce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cselce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cselce {
    #[inline(always)]
    fn from(val: u8) -> Cselce {
        Cselce::from_bits(val)
    }
}
impl From<Cselce> for u8 {
    #[inline(always)]
    fn from(val: Cselce) -> u8 {
        Cselce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cselcf {
    #[doc = "Counter clear is disable at the ELC_GPTF input"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the ELC_GPTF input"]
    _1 = 0x01,
}
impl Cselcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cselcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cselcf {
    #[inline(always)]
    fn from(val: u8) -> Cselcf {
        Cselcf::from_bits(val)
    }
}
impl From<Cselcf> for u8 {
    #[inline(always)]
    fn from(val: Cselcf) -> u8 {
        Cselcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cselcg {
    #[doc = "Counter clear is disable at the ELC_GPTG input"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the ELC_GPTG input"]
    _1 = 0x01,
}
impl Cselcg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cselcg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cselcg {
    #[inline(always)]
    fn from(val: u8) -> Cselcg {
        Cselcg::from_bits(val)
    }
}
impl From<Cselcg> for u8 {
    #[inline(always)]
    fn from(val: Cselcg) -> u8 {
        Cselcg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cselch {
    #[doc = "Counter clear is disable at the ELC_GPTH input"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the ELC_GPTH input"]
    _1 = 0x01,
}
impl Cselch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cselch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cselch {
    #[inline(always)]
    fn from(val: u8) -> Cselch {
        Cselch::from_bits(val)
    }
}
impl From<Cselch> for u8 {
    #[inline(always)]
    fn from(val: Cselch) -> u8 {
        Cselch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csgtrgaf {
    #[doc = "Counter clear is disable at the falling edge of GTETRGA input"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the falling edge of GTETRGA input"]
    _1 = 0x01,
}
impl Csgtrgaf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csgtrgaf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csgtrgaf {
    #[inline(always)]
    fn from(val: u8) -> Csgtrgaf {
        Csgtrgaf::from_bits(val)
    }
}
impl From<Csgtrgaf> for u8 {
    #[inline(always)]
    fn from(val: Csgtrgaf) -> u8 {
        Csgtrgaf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csgtrgar {
    #[doc = "Counter clear is disable at the rising edge of GTETRGA input"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the rising edge of GTETRGA input"]
    _1 = 0x01,
}
impl Csgtrgar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csgtrgar {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csgtrgar {
    #[inline(always)]
    fn from(val: u8) -> Csgtrgar {
        Csgtrgar::from_bits(val)
    }
}
impl From<Csgtrgar> for u8 {
    #[inline(always)]
    fn from(val: Csgtrgar) -> u8 {
        Csgtrgar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csgtrgbf {
    #[doc = "Counter clear is disable at the falling edge of GTETRGB input"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the falling edge of GTETRGB input"]
    _1 = 0x01,
}
impl Csgtrgbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csgtrgbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csgtrgbf {
    #[inline(always)]
    fn from(val: u8) -> Csgtrgbf {
        Csgtrgbf::from_bits(val)
    }
}
impl From<Csgtrgbf> for u8 {
    #[inline(always)]
    fn from(val: Csgtrgbf) -> u8 {
        Csgtrgbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csgtrgbr {
    #[doc = "Counter clear is disable at the rising edge of GTETRGB input"]
    _0 = 0x0,
    #[doc = "Counter clear is enable at the rising edge of GTETRGB input"]
    _1 = 0x01,
}
impl Csgtrgbr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csgtrgbr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csgtrgbr {
    #[inline(always)]
    fn from(val: u8) -> Csgtrgbr {
        Csgtrgbr::from_bits(val)
    }
}
impl From<Csgtrgbr> for u8 {
    #[inline(always)]
    fn from(val: Csgtrgbr) -> u8 {
        Csgtrgbr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cst {
    #[doc = "Count operation is stopped"]
    _0 = 0x0,
    #[doc = "Count operation is performed"]
    _1 = 0x01,
}
impl Cst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cst {
    #[inline(always)]
    fn from(val: u8) -> Cst {
        Cst::from_bits(val)
    }
}
impl From<Cst> for u8 {
    #[inline(always)]
    fn from(val: Cst) -> u8 {
        Cst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstop {
    #[doc = "Counter stop is disable by the GTSTP register"]
    _0 = 0x0,
    #[doc = "Counter stop is enable by the GTSTP register"]
    _1 = 0x01,
}
impl Cstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstop {
    #[inline(always)]
    fn from(val: u8) -> Cstop {
        Cstop::from_bits(val)
    }
}
impl From<Cstop> for u8 {
    #[inline(always)]
    fn from(val: Cstop) -> u8 {
        Cstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstop0 {
    #[doc = "No effect (write) / counter running (read)"]
    _0 = 0x0,
    #[doc = "GPT320.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 0x01,
}
impl Cstop0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstop0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstop0 {
    #[inline(always)]
    fn from(val: u8) -> Cstop0 {
        Cstop0::from_bits(val)
    }
}
impl From<Cstop0> for u8 {
    #[inline(always)]
    fn from(val: Cstop0) -> u8 {
        Cstop0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstop1 {
    #[doc = "No effect (write) / counter running (read)"]
    _0 = 0x0,
    #[doc = "GPT321.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 0x01,
}
impl Cstop1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstop1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstop1 {
    #[inline(always)]
    fn from(val: u8) -> Cstop1 {
        Cstop1::from_bits(val)
    }
}
impl From<Cstop1> for u8 {
    #[inline(always)]
    fn from(val: Cstop1) -> u8 {
        Cstop1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstop2 {
    #[doc = "No effect (write) / counter running (read)"]
    _0 = 0x0,
    #[doc = "GPT322.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 0x01,
}
impl Cstop2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstop2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstop2 {
    #[inline(always)]
    fn from(val: u8) -> Cstop2 {
        Cstop2::from_bits(val)
    }
}
impl From<Cstop2> for u8 {
    #[inline(always)]
    fn from(val: Cstop2) -> u8 {
        Cstop2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstop3 {
    #[doc = "No effect (write) / counter running (read)"]
    _0 = 0x0,
    #[doc = "GPT323.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 0x01,
}
impl Cstop3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstop3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstop3 {
    #[inline(always)]
    fn from(val: u8) -> Cstop3 {
        Cstop3::from_bits(val)
    }
}
impl From<Cstop3> for u8 {
    #[inline(always)]
    fn from(val: Cstop3) -> u8 {
        Cstop3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstop4 {
    #[doc = "No effect (write) / counter running (read)"]
    _0 = 0x0,
    #[doc = "GPT164.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 0x01,
}
impl Cstop4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstop4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstop4 {
    #[inline(always)]
    fn from(val: u8) -> Cstop4 {
        Cstop4::from_bits(val)
    }
}
impl From<Cstop4> for u8 {
    #[inline(always)]
    fn from(val: Cstop4) -> u8 {
        Cstop4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstop5 {
    #[doc = "No effect (write) / counter running (read)"]
    _0 = 0x0,
    #[doc = "GPT165.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 0x01,
}
impl Cstop5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstop5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstop5 {
    #[inline(always)]
    fn from(val: u8) -> Cstop5 {
        Cstop5::from_bits(val)
    }
}
impl From<Cstop5> for u8 {
    #[inline(always)]
    fn from(val: Cstop5) -> u8 {
        Cstop5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstop6 {
    #[doc = "No effect (write) / counter running (read)"]
    _0 = 0x0,
    #[doc = "GPT166.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 0x01,
}
impl Cstop6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstop6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstop6 {
    #[inline(always)]
    fn from(val: u8) -> Cstop6 {
        Cstop6::from_bits(val)
    }
}
impl From<Cstop6> for u8 {
    #[inline(always)]
    fn from(val: Cstop6) -> u8 {
        Cstop6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstop7 {
    #[doc = "No effect (write) / counter running (read)"]
    _0 = 0x0,
    #[doc = "GPT167.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 0x01,
}
impl Cstop7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstop7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstop7 {
    #[inline(always)]
    fn from(val: u8) -> Cstop7 {
        Cstop7::from_bits(val)
    }
}
impl From<Cstop7> for u8 {
    #[inline(always)]
    fn from(val: Cstop7) -> u8 {
        Cstop7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstrt {
    #[doc = "Counter start is disable by the GTSTR register"]
    _0 = 0x0,
    #[doc = "Counter start is enable by the GTSTR register"]
    _1 = 0x01,
}
impl Cstrt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstrt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstrt {
    #[inline(always)]
    fn from(val: u8) -> Cstrt {
        Cstrt::from_bits(val)
    }
}
impl From<Cstrt> for u8 {
    #[inline(always)]
    fn from(val: Cstrt) -> u8 {
        Cstrt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstrt0 {
    #[doc = "No effect (write) / counter stop (read)"]
    _0 = 0x0,
    #[doc = "GPT320.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 0x01,
}
impl Cstrt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstrt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstrt0 {
    #[inline(always)]
    fn from(val: u8) -> Cstrt0 {
        Cstrt0::from_bits(val)
    }
}
impl From<Cstrt0> for u8 {
    #[inline(always)]
    fn from(val: Cstrt0) -> u8 {
        Cstrt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstrt1 {
    #[doc = "No effect (write) / counter stop (read)"]
    _0 = 0x0,
    #[doc = "GPT321.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 0x01,
}
impl Cstrt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstrt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstrt1 {
    #[inline(always)]
    fn from(val: u8) -> Cstrt1 {
        Cstrt1::from_bits(val)
    }
}
impl From<Cstrt1> for u8 {
    #[inline(always)]
    fn from(val: Cstrt1) -> u8 {
        Cstrt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstrt2 {
    #[doc = "No effect (write) / counter stop (read)"]
    _0 = 0x0,
    #[doc = "GPT322.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 0x01,
}
impl Cstrt2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstrt2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstrt2 {
    #[inline(always)]
    fn from(val: u8) -> Cstrt2 {
        Cstrt2::from_bits(val)
    }
}
impl From<Cstrt2> for u8 {
    #[inline(always)]
    fn from(val: Cstrt2) -> u8 {
        Cstrt2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstrt3 {
    #[doc = "No effect (write) / counter stop (read)"]
    _0 = 0x0,
    #[doc = "GPT323.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 0x01,
}
impl Cstrt3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstrt3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstrt3 {
    #[inline(always)]
    fn from(val: u8) -> Cstrt3 {
        Cstrt3::from_bits(val)
    }
}
impl From<Cstrt3> for u8 {
    #[inline(always)]
    fn from(val: Cstrt3) -> u8 {
        Cstrt3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstrt4 {
    #[doc = "No effect (write) / counter stop (read)"]
    _0 = 0x0,
    #[doc = "GPT164.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 0x01,
}
impl Cstrt4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstrt4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstrt4 {
    #[inline(always)]
    fn from(val: u8) -> Cstrt4 {
        Cstrt4::from_bits(val)
    }
}
impl From<Cstrt4> for u8 {
    #[inline(always)]
    fn from(val: Cstrt4) -> u8 {
        Cstrt4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstrt5 {
    #[doc = "No effect (write) / counter stop (read)"]
    _0 = 0x0,
    #[doc = "GPT165.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 0x01,
}
impl Cstrt5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstrt5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstrt5 {
    #[inline(always)]
    fn from(val: u8) -> Cstrt5 {
        Cstrt5::from_bits(val)
    }
}
impl From<Cstrt5> for u8 {
    #[inline(always)]
    fn from(val: Cstrt5) -> u8 {
        Cstrt5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstrt6 {
    #[doc = "No effect (write) / counter stop (read)"]
    _0 = 0x0,
    #[doc = "GPT166.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 0x01,
}
impl Cstrt6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstrt6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstrt6 {
    #[inline(always)]
    fn from(val: u8) -> Cstrt6 {
        Cstrt6::from_bits(val)
    }
}
impl From<Cstrt6> for u8 {
    #[inline(always)]
    fn from(val: Cstrt6) -> u8 {
        Cstrt6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cstrt7 {
    #[doc = "No effect (write) / counter stop (read)"]
    _0 = 0x0,
    #[doc = "GPT167.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 0x01,
}
impl Cstrt7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cstrt7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cstrt7 {
    #[inline(always)]
    fn from(val: u8) -> Cstrt7 {
        Cstrt7::from_bits(val)
    }
}
impl From<Cstrt7> for u8 {
    #[inline(always)]
    fn from(val: Cstrt7) -> u8 {
        Cstrt7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dscafbh {
    #[doc = "Counter count down is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 0x01,
}
impl Dscafbh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dscafbh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dscafbh {
    #[inline(always)]
    fn from(val: u8) -> Dscafbh {
        Dscafbh::from_bits(val)
    }
}
impl From<Dscafbh> for u8 {
    #[inline(always)]
    fn from(val: Dscafbh) -> u8 {
        Dscafbh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dscafbl {
    #[doc = "Counter count down is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 0x01,
}
impl Dscafbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dscafbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dscafbl {
    #[inline(always)]
    fn from(val: u8) -> Dscafbl {
        Dscafbl::from_bits(val)
    }
}
impl From<Dscafbl> for u8 {
    #[inline(always)]
    fn from(val: Dscafbl) -> u8 {
        Dscafbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dscarbh {
    #[doc = "Counter count down is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 0x01,
}
impl Dscarbh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dscarbh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dscarbh {
    #[inline(always)]
    fn from(val: u8) -> Dscarbh {
        Dscarbh::from_bits(val)
    }
}
impl From<Dscarbh> for u8 {
    #[inline(always)]
    fn from(val: Dscarbh) -> u8 {
        Dscarbh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dscarbl {
    #[doc = "Counter count down is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 0x01,
}
impl Dscarbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dscarbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dscarbl {
    #[inline(always)]
    fn from(val: u8) -> Dscarbl {
        Dscarbl::from_bits(val)
    }
}
impl From<Dscarbl> for u8 {
    #[inline(always)]
    fn from(val: Dscarbl) -> u8 {
        Dscarbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dscbfah {
    #[doc = "Counter count down is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 0x01,
}
impl Dscbfah {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dscbfah {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dscbfah {
    #[inline(always)]
    fn from(val: u8) -> Dscbfah {
        Dscbfah::from_bits(val)
    }
}
impl From<Dscbfah> for u8 {
    #[inline(always)]
    fn from(val: Dscbfah) -> u8 {
        Dscbfah::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dscbfal {
    #[doc = "Counter count down is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 0x01,
}
impl Dscbfal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dscbfal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dscbfal {
    #[inline(always)]
    fn from(val: u8) -> Dscbfal {
        Dscbfal::from_bits(val)
    }
}
impl From<Dscbfal> for u8 {
    #[inline(always)]
    fn from(val: Dscbfal) -> u8 {
        Dscbfal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dscbrah {
    #[doc = "Counter count down is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 0x01,
}
impl Dscbrah {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dscbrah {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dscbrah {
    #[inline(always)]
    fn from(val: u8) -> Dscbrah {
        Dscbrah::from_bits(val)
    }
}
impl From<Dscbrah> for u8 {
    #[inline(always)]
    fn from(val: Dscbrah) -> u8 {
        Dscbrah::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dscbral {
    #[doc = "Counter count down is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 0x01,
}
impl Dscbral {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dscbral {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dscbral {
    #[inline(always)]
    fn from(val: u8) -> Dscbral {
        Dscbral::from_bits(val)
    }
}
impl From<Dscbral> for u8 {
    #[inline(always)]
    fn from(val: Dscbral) -> u8 {
        Dscbral::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dselca {
    #[doc = "Counter count down is disable at the ELC_GPTA input"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the ELC_GPTA input"]
    _1 = 0x01,
}
impl Dselca {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dselca {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dselca {
    #[inline(always)]
    fn from(val: u8) -> Dselca {
        Dselca::from_bits(val)
    }
}
impl From<Dselca> for u8 {
    #[inline(always)]
    fn from(val: Dselca) -> u8 {
        Dselca::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dselcb {
    #[doc = "Counter count down is disable at the ELC_GPTB input"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the ELC_GPTB input"]
    _1 = 0x01,
}
impl Dselcb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dselcb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dselcb {
    #[inline(always)]
    fn from(val: u8) -> Dselcb {
        Dselcb::from_bits(val)
    }
}
impl From<Dselcb> for u8 {
    #[inline(always)]
    fn from(val: Dselcb) -> u8 {
        Dselcb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dselcc {
    #[doc = "Counter count down is disable at the ELC_GPTC input"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the ELC_GPTC input"]
    _1 = 0x01,
}
impl Dselcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dselcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dselcc {
    #[inline(always)]
    fn from(val: u8) -> Dselcc {
        Dselcc::from_bits(val)
    }
}
impl From<Dselcc> for u8 {
    #[inline(always)]
    fn from(val: Dselcc) -> u8 {
        Dselcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dselcd {
    #[doc = "Counter count down is disable at the ELC_GPTD input"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the ELC_GPTD input"]
    _1 = 0x01,
}
impl Dselcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dselcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dselcd {
    #[inline(always)]
    fn from(val: u8) -> Dselcd {
        Dselcd::from_bits(val)
    }
}
impl From<Dselcd> for u8 {
    #[inline(always)]
    fn from(val: Dselcd) -> u8 {
        Dselcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dselce {
    #[doc = "Counter count down is disable at the ELC_GPTE input"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the ELC_GPTE input"]
    _1 = 0x01,
}
impl Dselce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dselce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dselce {
    #[inline(always)]
    fn from(val: u8) -> Dselce {
        Dselce::from_bits(val)
    }
}
impl From<Dselce> for u8 {
    #[inline(always)]
    fn from(val: Dselce) -> u8 {
        Dselce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dselcf {
    #[doc = "Counter count down is disable at the ELC_GPTF input"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the ELC_GPTF input"]
    _1 = 0x01,
}
impl Dselcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dselcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dselcf {
    #[inline(always)]
    fn from(val: u8) -> Dselcf {
        Dselcf::from_bits(val)
    }
}
impl From<Dselcf> for u8 {
    #[inline(always)]
    fn from(val: Dselcf) -> u8 {
        Dselcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dselcg {
    #[doc = "Counter count down is disable at the ELC_GPTG input"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the ELC_GPTG input"]
    _1 = 0x01,
}
impl Dselcg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dselcg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dselcg {
    #[inline(always)]
    fn from(val: u8) -> Dselcg {
        Dselcg::from_bits(val)
    }
}
impl From<Dselcg> for u8 {
    #[inline(always)]
    fn from(val: Dselcg) -> u8 {
        Dselcg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dselch {
    #[doc = "Counter count down is disable at the ELC_GPTH input"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the ELC_GPTH input"]
    _1 = 0x01,
}
impl Dselch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dselch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dselch {
    #[inline(always)]
    fn from(val: u8) -> Dselch {
        Dselch::from_bits(val)
    }
}
impl From<Dselch> for u8 {
    #[inline(always)]
    fn from(val: Dselch) -> u8 {
        Dselch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dsgtrgaf {
    #[doc = "Counter count down is disable at the falling edge of GTETRGA input"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the falling edge of GTETRGA input"]
    _1 = 0x01,
}
impl Dsgtrgaf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dsgtrgaf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dsgtrgaf {
    #[inline(always)]
    fn from(val: u8) -> Dsgtrgaf {
        Dsgtrgaf::from_bits(val)
    }
}
impl From<Dsgtrgaf> for u8 {
    #[inline(always)]
    fn from(val: Dsgtrgaf) -> u8 {
        Dsgtrgaf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dsgtrgar {
    #[doc = "Counter count down is disable at the rising edge of GTETRGA input"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the rising edge of GTETRGA input"]
    _1 = 0x01,
}
impl Dsgtrgar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dsgtrgar {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dsgtrgar {
    #[inline(always)]
    fn from(val: u8) -> Dsgtrgar {
        Dsgtrgar::from_bits(val)
    }
}
impl From<Dsgtrgar> for u8 {
    #[inline(always)]
    fn from(val: Dsgtrgar) -> u8 {
        Dsgtrgar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dsgtrgbf {
    #[doc = "Counter count down is disable at the falling edge of GTETRGB input"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the falling edge of GTETRGB input"]
    _1 = 0x01,
}
impl Dsgtrgbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dsgtrgbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dsgtrgbf {
    #[inline(always)]
    fn from(val: u8) -> Dsgtrgbf {
        Dsgtrgbf::from_bits(val)
    }
}
impl From<Dsgtrgbf> for u8 {
    #[inline(always)]
    fn from(val: Dsgtrgbf) -> u8 {
        Dsgtrgbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dsgtrgbr {
    #[doc = "Counter count down is disable at the rising edge of GTETRGB input"]
    _0 = 0x0,
    #[doc = "Counter count down is enable at the rising edge of GTETRGB input"]
    _1 = 0x01,
}
impl Dsgtrgbr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dsgtrgbr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dsgtrgbr {
    #[inline(always)]
    fn from(val: u8) -> Dsgtrgbr {
        Dsgtrgbr::from_bits(val)
    }
}
impl From<Dsgtrgbr> for u8 {
    #[inline(always)]
    fn from(val: Dsgtrgbr) -> u8 {
        Dsgtrgbr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Grp {
    #[doc = "Group A output disable request"]
    _00 = 0x0,
    #[doc = "Group B output disable request"]
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
pub enum Grpabh {
    #[doc = "Same time output level high disable request is disabled."]
    _0 = 0x0,
    #[doc = "Same time output level high disable request is enabled."]
    _1 = 0x01,
}
impl Grpabh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Grpabh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Grpabh {
    #[inline(always)]
    fn from(val: u8) -> Grpabh {
        Grpabh::from_bits(val)
    }
}
impl From<Grpabh> for u8 {
    #[inline(always)]
    fn from(val: Grpabh) -> u8 {
        Grpabh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Grpabl {
    #[doc = "Same time output level low disable request is disabled."]
    _0 = 0x0,
    #[doc = "Same time output level low disable request is enabled."]
    _1 = 0x01,
}
impl Grpabl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Grpabl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Grpabl {
    #[inline(always)]
    fn from(val: u8) -> Grpabl {
        Grpabl::from_bits(val)
    }
}
impl From<Grpabl> for u8 {
    #[inline(always)]
    fn from(val: Grpabl) -> u8 {
        Grpabl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gtioa {
    #[doc = "Initial output is Low. Output retained at cycle end. Output retained at GTCCRA compare match."]
    _00000 = 0x0,
    #[doc = "Initial output is Low. Output retained at cycle end. Low output at GTCCRA compare match."]
    _00001 = 0x01,
    #[doc = "Initial output is Low. Output retained at cycle end. High output at GTCCRA compare match."]
    _00010 = 0x02,
    #[doc = "Initial output is Low. Output retained at cycle end. Output toggled at GTCCRA compare match."]
    _00011 = 0x03,
    #[doc = "Initial output is Low. Low output at cycle end. Output retained at GTCCRA compare match."]
    _00100 = 0x04,
    #[doc = "Initial output is Low. Low output at cycle end. Low output at GTCCRA compare match."]
    _00101 = 0x05,
    #[doc = "Initial output is Low. Low output at cycle end. High output at GTCCRA compare match."]
    _00110 = 0x06,
    #[doc = "Initial output is Low. Low output at cycle end. Output toggled at GTCCRA compare match."]
    _00111 = 0x07,
    #[doc = "Initial output is Low. High output at cycle end. Output retained at GTCCRA compare match."]
    _01000 = 0x08,
    #[doc = "Initial output is Low. High output at cycle end. Low output at GTCCRA compare match."]
    _01001 = 0x09,
    #[doc = "Initial output is Low. High output at cycle end. High output at GTCCRA compare match."]
    _01010 = 0x0a,
    #[doc = "Initial output is Low. High output at cycle end. Output toggled at GTCCRA compare match."]
    _01011 = 0x0b,
    #[doc = "Initial output is Low. Output toggled at cycle end. Output retained at GTCCRA compare match."]
    _01100 = 0x0c,
    #[doc = "Initial output is Low. Output toggled at cycle end. Low output at GTCCRA compare match."]
    _01101 = 0x0d,
    #[doc = "Initial output is Low. Output toggled at cycle end. High output at GTCCRA compare match."]
    _01110 = 0x0e,
    #[doc = "Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRA compare match."]
    _01111 = 0x0f,
    #[doc = "Initial output is High. Output retained at cycle end. Output retained at GTCCRA compare match."]
    _10000 = 0x10,
    #[doc = "Initial output is High. Output retained at cycle end. Low output at GTCCRA compare match."]
    _10001 = 0x11,
    #[doc = "Initial output is High. Output retained at cycle end. High output at GTCCRA compare match."]
    _10010 = 0x12,
    #[doc = "Initial output is High. Output retained at cycle end. Output toggled at GTCCRA compare match."]
    _10011 = 0x13,
    #[doc = "Initial output is High. Low output at cycle end. Output retained at GTCCRA compare match."]
    _10100 = 0x14,
    #[doc = "Initial output is High. Low output at cycle end. Low output at GTCCRA compare match."]
    _10101 = 0x15,
    #[doc = "Initial output is High. Low output at cycle end. High output at GTCCRA compare match."]
    _10110 = 0x16,
    #[doc = "Initial output is High. Low output at cycle end. Output toggled at GTCCRA compare match."]
    _10111 = 0x17,
    #[doc = "Initial output is High. High output at cycle end. Output retained at GTCCRA compare match."]
    _11000 = 0x18,
    #[doc = "Initial output is High. High output at cycle end. Low output at GTCCRA compare match."]
    _11001 = 0x19,
    #[doc = "Initial output is High. High output at cycle end. High output at GTCCRA compare match."]
    _11010 = 0x1a,
    #[doc = "Initial output is High. High output at cycle end. Output toggled at GTCCRA compare match."]
    _11011 = 0x1b,
    #[doc = "Initial output is High. Output toggled at cycle end. Output retained at GTCCRA compare match."]
    _11100 = 0x1c,
    #[doc = "Initial output is High. Output toggled at cycle end. Low output at GTCCRA compare match."]
    _11101 = 0x1d,
    #[doc = "Initial output is High. Output toggled at cycle end. High output at GTCCRA compare match."]
    _11110 = 0x1e,
    #[doc = "Initial output is High. Output toggled at cycle end. Output toggled at GTCCRA compare match."]
    _11111 = 0x1f,
}
impl Gtioa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gtioa {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gtioa {
    #[inline(always)]
    fn from(val: u8) -> Gtioa {
        Gtioa::from_bits(val)
    }
}
impl From<Gtioa> for u8 {
    #[inline(always)]
    fn from(val: Gtioa) -> u8 {
        Gtioa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gtiob {
    #[doc = "Initial output is Low. Output retained at cycle end. Output retained at GTCCRB compare match."]
    _00000 = 0x0,
    #[doc = "Initial output is Low. Output retained at cycle end. Low output at GTCCRB compare match."]
    _00001 = 0x01,
    #[doc = "Initial output is Low. Output retained at cycle end. High output at GTCCRB compare match."]
    _00010 = 0x02,
    #[doc = "Initial output is Low. Output retained at cycle end. Output toggled at GTCCRB compare match."]
    _00011 = 0x03,
    #[doc = "Initial output is Low. Low output at cycle end. Output retained at GTCCRB compare match."]
    _00100 = 0x04,
    #[doc = "Initial output is Low. Low output at cycle end. Low output at GTCCRB compare match."]
    _00101 = 0x05,
    #[doc = "Initial output is Low. Low output at cycle end. High output at GTCCRB compare match."]
    _00110 = 0x06,
    #[doc = "Initial output is Low. Low output at cycle end. Output toggled at GTCCRB compare match."]
    _00111 = 0x07,
    #[doc = "Initial output is Low. High output at cycle end. Output retained at GTCCRB compare match."]
    _01000 = 0x08,
    #[doc = "Initial output is Low. High output at cycle end. Low output at GTCCRB compare match."]
    _01001 = 0x09,
    #[doc = "Initial output is Low. High output at cycle end. High output at GTCCRB compare match."]
    _01010 = 0x0a,
    #[doc = "Initial output is Low. High output at cycle end. Output toggled at GTCCRB compare match."]
    _01011 = 0x0b,
    #[doc = "Initial output is Low. Output toggled at cycle end. Output retained at GTCCRB compare match."]
    _01100 = 0x0c,
    #[doc = "Initial output is Low. Output toggled at cycle end. Low output at GTCCRB compare match."]
    _01101 = 0x0d,
    #[doc = "Initial output is Low. Output toggled at cycle end. High output at GTCCRB compare match."]
    _01110 = 0x0e,
    #[doc = "Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRB compare match."]
    _01111 = 0x0f,
    #[doc = "Initial output is High. Output retained at cycle end. Output retained at GTCCRB compare match."]
    _10000 = 0x10,
    #[doc = "Initial output is High. Output retained at cycle end. Low output at GTCCRB compare match."]
    _10001 = 0x11,
    #[doc = "Initial output is High. Output retained at cycle end. High output at GTCCRB compare match."]
    _10010 = 0x12,
    #[doc = "Initial output is High. Output retained at cycle end. Output toggled at GTCCRB compare match."]
    _10011 = 0x13,
    #[doc = "Initial output is High. Low output at cycle end. Output retained at GTCCRB compare match."]
    _10100 = 0x14,
    #[doc = "Initial output is High. Low output at cycle end. Low output at GTCCRB compare match."]
    _10101 = 0x15,
    #[doc = "Initial output is High. Low output at cycle end. High output at GTCCRB compare match."]
    _10110 = 0x16,
    #[doc = "Initial output is High. Low output at cycle end. Output toggled at GTCCRB compare match."]
    _10111 = 0x17,
    #[doc = "Initial output is High. High output at cycle end. Output retained at GTCCRB compare match."]
    _11000 = 0x18,
    #[doc = "Initial output is High. High output at cycle end. Low output at GTCCRB compare match."]
    _11001 = 0x19,
    #[doc = "Initial output is High. High output at cycle end. High output at GTCCRB compare match."]
    _11010 = 0x1a,
    #[doc = "Initial output is High. High output at cycle end. Output toggled at GTCCRB compare match."]
    _11011 = 0x1b,
    #[doc = "Initial output is High. Output toggled at cycle end. Output retained at GTCCRB compare match."]
    _11100 = 0x1c,
    #[doc = "Initial output is High. Output toggled at cycle end. Low output at GTCCRB compare match."]
    _11101 = 0x1d,
    #[doc = "Initial output is High. Output toggled at cycle end. High output at GTCCRB compare match."]
    _11110 = 0x1e,
    #[doc = "Initial output is High. Output toggled at cycle end. Output toggled at GTCCRB compare match."]
    _11111 = 0x1f,
}
impl Gtiob {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gtiob {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gtiob {
    #[inline(always)]
    fn from(val: u8) -> Gtiob {
        Gtiob::from_bits(val)
    }
}
impl From<Gtiob> for u8 {
    #[inline(always)]
    fn from(val: Gtiob) -> u8 {
        Gtiob::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Md {
    #[doc = "Saw-wave PWM mode (single buffer or double buffer possible)"]
    _000 = 0x0,
    #[doc = "Saw-wave one-shot pulse mode (fixed buffer operation)"]
    _001 = 0x01,
    #[doc = "Setting prohibited"]
    _010 = 0x02,
    #[doc = "Setting prohibited"]
    _011 = 0x03,
    #[doc = "Triangle-wave PWM mode 1 (16-bit transfer at crest) (single buffer or double buffer possible)"]
    _100 = 0x04,
    #[doc = "Triangle-wave PWM mode 2 (16-bit transfer at crest and trough) (single buffer or double buffer possible)"]
    _101 = 0x05,
    #[doc = "Triangle-wave PWM mode 3 (32-bit transfer at trough) fixed buffer operation)"]
    _110 = 0x06,
    #[doc = "Setting prohibited"]
    _111 = 0x07,
}
impl Md {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Md {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Md {
    #[inline(always)]
    fn from(val: u8) -> Md {
        Md::from_bits(val)
    }
}
impl From<Md> for u8 {
    #[inline(always)]
    fn from(val: Md) -> u8 {
        Md::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nfaen {
    #[doc = "The noise filter for the GTIOCA pin is disabled."]
    _0 = 0x0,
    #[doc = "The noise filter for the GTIOCA pin is enabled."]
    _1 = 0x01,
}
impl Nfaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfaen {
    #[inline(always)]
    fn from(val: u8) -> Nfaen {
        Nfaen::from_bits(val)
    }
}
impl From<Nfaen> for u8 {
    #[inline(always)]
    fn from(val: Nfaen) -> u8 {
        Nfaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nfben {
    #[doc = "The noise filter for the GTIOCB pin is disabled."]
    _0 = 0x0,
    #[doc = "The noise filter for the GTIOCB pin is enabled."]
    _1 = 0x01,
}
impl Nfben {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfben {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfben {
    #[inline(always)]
    fn from(val: u8) -> Nfben {
        Nfben::from_bits(val)
    }
}
impl From<Nfben> for u8 {
    #[inline(always)]
    fn from(val: Nfben) -> u8 {
        Nfben::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nfcsa {
    #[doc = "PCLK/1"]
    _00 = 0x0,
    #[doc = "PCLK/4"]
    _01 = 0x01,
    #[doc = "PCLK/16"]
    _10 = 0x02,
    #[doc = "PCLK/64"]
    _11 = 0x03,
}
impl Nfcsa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfcsa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfcsa {
    #[inline(always)]
    fn from(val: u8) -> Nfcsa {
        Nfcsa::from_bits(val)
    }
}
impl From<Nfcsa> for u8 {
    #[inline(always)]
    fn from(val: Nfcsa) -> u8 {
        Nfcsa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nfcsb {
    #[doc = "PCLK/1"]
    _00 = 0x0,
    #[doc = "PCLK/4"]
    _01 = 0x01,
    #[doc = "PCLK/16"]
    _10 = 0x02,
    #[doc = "PCLK/64"]
    _11 = 0x03,
}
impl Nfcsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfcsb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfcsb {
    #[inline(always)]
    fn from(val: u8) -> Nfcsb {
        Nfcsb::from_bits(val)
    }
}
impl From<Nfcsb> for u8 {
    #[inline(always)]
    fn from(val: Nfcsb) -> u8 {
        Nfcsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oabhf {
    #[doc = "GTIOCA pin and GTIOCB pin don't output 1 at the same time."]
    _0 = 0x0,
    #[doc = "GTIOCA pin and GTIOCB pin output 1 at the same time."]
    _1 = 0x01,
}
impl Oabhf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oabhf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oabhf {
    #[inline(always)]
    fn from(val: u8) -> Oabhf {
        Oabhf::from_bits(val)
    }
}
impl From<Oabhf> for u8 {
    #[inline(always)]
    fn from(val: Oabhf) -> u8 {
        Oabhf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oablf {
    #[doc = "GTIOCA pin and GTIOCB pin don't output 0 at the same time."]
    _0 = 0x0,
    #[doc = "GTIOCA pin and GTIOCB pin output 0 at the same time."]
    _1 = 0x01,
}
impl Oablf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oablf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oablf {
    #[inline(always)]
    fn from(val: u8) -> Oablf {
        Oablf::from_bits(val)
    }
}
impl From<Oablf> for u8 {
    #[inline(always)]
    fn from(val: Oablf) -> u8 {
        Oablf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oadf {
    #[doc = "Output disable is prohibited."]
    _00 = 0x0,
    #[doc = "GTIOCA pin is set to Hi-Z when output disable is performed."]
    _01 = 0x01,
    #[doc = "GTIOCA pin is set to 0 when output disable is performed."]
    _10 = 0x02,
    #[doc = "GTIOCA pin is set to 1 when output disable is performed."]
    _11 = 0x03,
}
impl Oadf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oadf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oadf {
    #[inline(always)]
    fn from(val: u8) -> Oadf {
        Oadf::from_bits(val)
    }
}
impl From<Oadf> for u8 {
    #[inline(always)]
    fn from(val: Oadf) -> u8 {
        Oadf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oadflt {
    #[doc = "The GTIOCA pin outputs low when counting is stopped."]
    _0 = 0x0,
    #[doc = "The GTIOCA pin outputs high when counting is stopped."]
    _1 = 0x01,
}
impl Oadflt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oadflt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oadflt {
    #[inline(always)]
    fn from(val: u8) -> Oadflt {
        Oadflt::from_bits(val)
    }
}
impl From<Oadflt> for u8 {
    #[inline(always)]
    fn from(val: Oadflt) -> u8 {
        Oadflt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oadty {
    #[doc = "GTIOCA pin duty is depend on compare match"]
    _00 = 0x0,
    #[doc = "GTIOCA pin duty is depend on compare match"]
    _01 = 0x01,
    #[doc = "GTIOCA pin duty 0 percent"]
    _10 = 0x02,
    #[doc = "GTIOCA pin duty 100 percent"]
    _11 = 0x03,
}
impl Oadty {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oadty {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oadty {
    #[inline(always)]
    fn from(val: u8) -> Oadty {
        Oadty::from_bits(val)
    }
}
impl From<Oadty> for u8 {
    #[inline(always)]
    fn from(val: Oadty) -> u8 {
        Oadty::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oadtyf {
    #[doc = "Not forcibly set"]
    _0 = 0x0,
    #[doc = "Forcibly set"]
    _1 = 0x01,
}
impl Oadtyf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oadtyf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oadtyf {
    #[inline(always)]
    fn from(val: u8) -> Oadtyf {
        Oadtyf::from_bits(val)
    }
}
impl From<Oadtyf> for u8 {
    #[inline(always)]
    fn from(val: Oadtyf) -> u8 {
        Oadtyf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oadtyr {
    #[doc = "Apply output value set in 0 percent/100 percent duty to GTIOA\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    _0 = 0x0,
    #[doc = "Apply masked compare match output value to GTIOA\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    _1 = 0x01,
}
impl Oadtyr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oadtyr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oadtyr {
    #[inline(always)]
    fn from(val: u8) -> Oadtyr {
        Oadtyr::from_bits(val)
    }
}
impl From<Oadtyr> for u8 {
    #[inline(always)]
    fn from(val: Oadtyr) -> u8 {
        Oadtyr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oae {
    #[doc = "Output is disabled"]
    _0 = 0x0,
    #[doc = "Output is enabled"]
    _1 = 0x01,
}
impl Oae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oae {
    #[inline(always)]
    fn from(val: u8) -> Oae {
        Oae::from_bits(val)
    }
}
impl From<Oae> for u8 {
    #[inline(always)]
    fn from(val: Oae) -> u8 {
        Oae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oahld {
    #[doc = "The GTIOCA pin output level at start/stop of counting depends on the register setting."]
    _0 = 0x0,
    #[doc = "The GTIOCA pin output level is retained at start/stop of counting."]
    _1 = 0x01,
}
impl Oahld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oahld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oahld {
    #[inline(always)]
    fn from(val: u8) -> Oahld {
        Oahld::from_bits(val)
    }
}
impl From<Oahld> for u8 {
    #[inline(always)]
    fn from(val: Oahld) -> u8 {
        Oahld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Obdf {
    #[doc = "Output disable is prohibited."]
    _00 = 0x0,
    #[doc = "GTIOCB pin is set to Hi-Z when output disable is performed."]
    _01 = 0x01,
    #[doc = "GTIOCB pin is set to 0 when output disable is performed."]
    _10 = 0x02,
    #[doc = "GTIOCB pin is set to 1 when output disable is performed."]
    _11 = 0x03,
}
impl Obdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Obdf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Obdf {
    #[inline(always)]
    fn from(val: u8) -> Obdf {
        Obdf::from_bits(val)
    }
}
impl From<Obdf> for u8 {
    #[inline(always)]
    fn from(val: Obdf) -> u8 {
        Obdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Obdflt {
    #[doc = "The GTIOCB pin outputs low when counting is stopped."]
    _0 = 0x0,
    #[doc = "The GTIOCB pin outputs high when counting is stopped."]
    _1 = 0x01,
}
impl Obdflt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Obdflt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Obdflt {
    #[inline(always)]
    fn from(val: u8) -> Obdflt {
        Obdflt::from_bits(val)
    }
}
impl From<Obdflt> for u8 {
    #[inline(always)]
    fn from(val: Obdflt) -> u8 {
        Obdflt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Obdty {
    #[doc = "GTIOCB pin duty is depend on compare match"]
    _00 = 0x0,
    #[doc = "GTIOCB pin duty is depend on compare match"]
    _01 = 0x01,
    #[doc = "GTIOCB pin duty 0 percent"]
    _10 = 0x02,
    #[doc = "GTIOCB pin duty 100 percent"]
    _11 = 0x03,
}
impl Obdty {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Obdty {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Obdty {
    #[inline(always)]
    fn from(val: u8) -> Obdty {
        Obdty::from_bits(val)
    }
}
impl From<Obdty> for u8 {
    #[inline(always)]
    fn from(val: Obdty) -> u8 {
        Obdty::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Obdtyf {
    #[doc = "Not forcibly set"]
    _0 = 0x0,
    #[doc = "Forcibly set"]
    _1 = 0x01,
}
impl Obdtyf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Obdtyf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Obdtyf {
    #[inline(always)]
    fn from(val: u8) -> Obdtyf {
        Obdtyf::from_bits(val)
    }
}
impl From<Obdtyf> for u8 {
    #[inline(always)]
    fn from(val: Obdtyf) -> u8 {
        Obdtyf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Obdtyr {
    #[doc = "Apply output value set in 0 percent/100 percent duty to GTIOB\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    _0 = 0x0,
    #[doc = "Apply masked compare match output value to GTIOB\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    _1 = 0x01,
}
impl Obdtyr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Obdtyr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Obdtyr {
    #[inline(always)]
    fn from(val: u8) -> Obdtyr {
        Obdtyr::from_bits(val)
    }
}
impl From<Obdtyr> for u8 {
    #[inline(always)]
    fn from(val: Obdtyr) -> u8 {
        Obdtyr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Obe {
    #[doc = "Output is disabled"]
    _0 = 0x0,
    #[doc = "Output is enabled"]
    _1 = 0x01,
}
impl Obe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Obe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Obe {
    #[inline(always)]
    fn from(val: u8) -> Obe {
        Obe::from_bits(val)
    }
}
impl From<Obe> for u8 {
    #[inline(always)]
    fn from(val: Obe) -> u8 {
        Obe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Obhld {
    #[doc = "The GTIOCB pin output level at start/stop of counting depends on the register setting."]
    _0 = 0x0,
    #[doc = "The GTIOCB pin output level is retained at start/stop of counting."]
    _1 = 0x01,
}
impl Obhld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Obhld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Obhld {
    #[inline(always)]
    fn from(val: u8) -> Obhld {
        Obhld::from_bits(val)
    }
}
impl From<Obhld> for u8 {
    #[inline(always)]
    fn from(val: Obhld) -> u8 {
        Obhld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Odf {
    #[doc = "No output disable request is generated."]
    _0 = 0x0,
    #[doc = "An output disable request is generated."]
    _1 = 0x01,
}
impl Odf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Odf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Odf {
    #[inline(always)]
    fn from(val: u8) -> Odf {
        Odf::from_bits(val)
    }
}
impl From<Odf> for u8 {
    #[inline(always)]
    fn from(val: Odf) -> u8 {
        Odf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pr {
    #[doc = "Buffer operation is not performed"]
    _00 = 0x0,
    #[doc = "Single buffer operation (GTPBR --> GTPR)"]
    _01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pr {
    #[inline(always)]
    fn from(val: u8) -> Pr {
        Pr::from_bits(val)
    }
}
impl From<Pr> for u8 {
    #[inline(always)]
    fn from(val: Pr) -> u8 {
        Pr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Prkey(u8);
impl Prkey {
    #[doc = "Written to these bits, the WP bits write is permitted."]
    pub const _0X_A5: Self = Self(0xa5);
}
impl Prkey {
    pub const fn from_bits(val: u8) -> Prkey {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Prkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xa5 => f.write_str("_0X_A5"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prkey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xa5 => defmt::write!(f, "_0X_A5"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Prkey {
    #[inline(always)]
    fn from(val: u8) -> Prkey {
        Prkey::from_bits(val)
    }
}
impl From<Prkey> for u8 {
    #[inline(always)]
    fn from(val: Prkey) -> u8 {
        Prkey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscafbh {
    #[doc = "Counter stop is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 0x01,
}
impl Pscafbh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscafbh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscafbh {
    #[inline(always)]
    fn from(val: u8) -> Pscafbh {
        Pscafbh::from_bits(val)
    }
}
impl From<Pscafbh> for u8 {
    #[inline(always)]
    fn from(val: Pscafbh) -> u8 {
        Pscafbh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscafbl {
    #[doc = "Counter stop is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 0x01,
}
impl Pscafbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscafbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscafbl {
    #[inline(always)]
    fn from(val: u8) -> Pscafbl {
        Pscafbl::from_bits(val)
    }
}
impl From<Pscafbl> for u8 {
    #[inline(always)]
    fn from(val: Pscafbl) -> u8 {
        Pscafbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscarbh {
    #[doc = "Counter stop is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 0x01,
}
impl Pscarbh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscarbh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscarbh {
    #[inline(always)]
    fn from(val: u8) -> Pscarbh {
        Pscarbh::from_bits(val)
    }
}
impl From<Pscarbh> for u8 {
    #[inline(always)]
    fn from(val: Pscarbh) -> u8 {
        Pscarbh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscarbl {
    #[doc = "Counter stop is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 0x01,
}
impl Pscarbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscarbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscarbl {
    #[inline(always)]
    fn from(val: u8) -> Pscarbl {
        Pscarbl::from_bits(val)
    }
}
impl From<Pscarbl> for u8 {
    #[inline(always)]
    fn from(val: Pscarbl) -> u8 {
        Pscarbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscbfah {
    #[doc = "Counter stop is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 0x01,
}
impl Pscbfah {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscbfah {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscbfah {
    #[inline(always)]
    fn from(val: u8) -> Pscbfah {
        Pscbfah::from_bits(val)
    }
}
impl From<Pscbfah> for u8 {
    #[inline(always)]
    fn from(val: Pscbfah) -> u8 {
        Pscbfah::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscbfal {
    #[doc = "Counter stop is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 0x01,
}
impl Pscbfal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscbfal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscbfal {
    #[inline(always)]
    fn from(val: u8) -> Pscbfal {
        Pscbfal::from_bits(val)
    }
}
impl From<Pscbfal> for u8 {
    #[inline(always)]
    fn from(val: Pscbfal) -> u8 {
        Pscbfal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscbrah {
    #[doc = "Counter stop is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 0x01,
}
impl Pscbrah {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscbrah {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscbrah {
    #[inline(always)]
    fn from(val: u8) -> Pscbrah {
        Pscbrah::from_bits(val)
    }
}
impl From<Pscbrah> for u8 {
    #[inline(always)]
    fn from(val: Pscbrah) -> u8 {
        Pscbrah::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscbral {
    #[doc = "Counter stop is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 0x01,
}
impl Pscbral {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscbral {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscbral {
    #[inline(always)]
    fn from(val: u8) -> Pscbral {
        Pscbral::from_bits(val)
    }
}
impl From<Pscbral> for u8 {
    #[inline(always)]
    fn from(val: Pscbral) -> u8 {
        Pscbral::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pselca {
    #[doc = "Counter stop is disable at the ELC_GPTA input"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the ELC_GPTA input"]
    _1 = 0x01,
}
impl Pselca {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pselca {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pselca {
    #[inline(always)]
    fn from(val: u8) -> Pselca {
        Pselca::from_bits(val)
    }
}
impl From<Pselca> for u8 {
    #[inline(always)]
    fn from(val: Pselca) -> u8 {
        Pselca::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pselcb {
    #[doc = "Counter stop is disable at the ELC_GPTB input"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the ELC_GPTB input"]
    _1 = 0x01,
}
impl Pselcb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pselcb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pselcb {
    #[inline(always)]
    fn from(val: u8) -> Pselcb {
        Pselcb::from_bits(val)
    }
}
impl From<Pselcb> for u8 {
    #[inline(always)]
    fn from(val: Pselcb) -> u8 {
        Pselcb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pselcc {
    #[doc = "Counter stop is disable at the ELC_GPTC input"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the ELC_GPTC input"]
    _1 = 0x01,
}
impl Pselcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pselcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pselcc {
    #[inline(always)]
    fn from(val: u8) -> Pselcc {
        Pselcc::from_bits(val)
    }
}
impl From<Pselcc> for u8 {
    #[inline(always)]
    fn from(val: Pselcc) -> u8 {
        Pselcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pselcd {
    #[doc = "Counter stop is disable at the ELC_GPTD input"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the ELC_GPTD input"]
    _1 = 0x01,
}
impl Pselcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pselcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pselcd {
    #[inline(always)]
    fn from(val: u8) -> Pselcd {
        Pselcd::from_bits(val)
    }
}
impl From<Pselcd> for u8 {
    #[inline(always)]
    fn from(val: Pselcd) -> u8 {
        Pselcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pselce {
    #[doc = "Counter stop is disable at the ELC_GPTE input"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the ELC_GPTE input"]
    _1 = 0x01,
}
impl Pselce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pselce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pselce {
    #[inline(always)]
    fn from(val: u8) -> Pselce {
        Pselce::from_bits(val)
    }
}
impl From<Pselce> for u8 {
    #[inline(always)]
    fn from(val: Pselce) -> u8 {
        Pselce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pselcf {
    #[doc = "Counter stop is disable at the ELC_GPTF input"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the ELC_GPTF input"]
    _1 = 0x01,
}
impl Pselcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pselcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pselcf {
    #[inline(always)]
    fn from(val: u8) -> Pselcf {
        Pselcf::from_bits(val)
    }
}
impl From<Pselcf> for u8 {
    #[inline(always)]
    fn from(val: Pselcf) -> u8 {
        Pselcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pselcg {
    #[doc = "Counter stop is disable at the ELC_GPTG input"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the ELC_GPTG input"]
    _1 = 0x01,
}
impl Pselcg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pselcg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pselcg {
    #[inline(always)]
    fn from(val: u8) -> Pselcg {
        Pselcg::from_bits(val)
    }
}
impl From<Pselcg> for u8 {
    #[inline(always)]
    fn from(val: Pselcg) -> u8 {
        Pselcg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pselch {
    #[doc = "Counter stop is disable at the ELC_GPTH input"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the ELC_GPTH input"]
    _1 = 0x01,
}
impl Pselch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pselch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pselch {
    #[inline(always)]
    fn from(val: u8) -> Pselch {
        Pselch::from_bits(val)
    }
}
impl From<Pselch> for u8 {
    #[inline(always)]
    fn from(val: Pselch) -> u8 {
        Pselch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psgtrgaf {
    #[doc = "Counter stop is disable at the falling edge of GTETRGA input"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the falling edge of GTETRGA input"]
    _1 = 0x01,
}
impl Psgtrgaf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psgtrgaf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psgtrgaf {
    #[inline(always)]
    fn from(val: u8) -> Psgtrgaf {
        Psgtrgaf::from_bits(val)
    }
}
impl From<Psgtrgaf> for u8 {
    #[inline(always)]
    fn from(val: Psgtrgaf) -> u8 {
        Psgtrgaf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psgtrgar {
    #[doc = "Counter stop is disable at the rising edge of GTETRGA input"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the rising edge of GTETRGA input"]
    _1 = 0x01,
}
impl Psgtrgar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psgtrgar {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psgtrgar {
    #[inline(always)]
    fn from(val: u8) -> Psgtrgar {
        Psgtrgar::from_bits(val)
    }
}
impl From<Psgtrgar> for u8 {
    #[inline(always)]
    fn from(val: Psgtrgar) -> u8 {
        Psgtrgar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psgtrgbf {
    #[doc = "Counter stop is disable at the falling edge of GTETRGB input"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the falling edge of GTETRGB input"]
    _1 = 0x01,
}
impl Psgtrgbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psgtrgbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psgtrgbf {
    #[inline(always)]
    fn from(val: u8) -> Psgtrgbf {
        Psgtrgbf::from_bits(val)
    }
}
impl From<Psgtrgbf> for u8 {
    #[inline(always)]
    fn from(val: Psgtrgbf) -> u8 {
        Psgtrgbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psgtrgbr {
    #[doc = "Counter stop is disable at the rising edge of GTETRGB input"]
    _0 = 0x0,
    #[doc = "Counter stop is enable at the rising edge of GTETRGB input"]
    _1 = 0x01,
}
impl Psgtrgbr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psgtrgbr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psgtrgbr {
    #[inline(always)]
    fn from(val: u8) -> Psgtrgbr {
        Psgtrgbr::from_bits(val)
    }
}
impl From<Psgtrgbr> for u8 {
    #[inline(always)]
    fn from(val: Psgtrgbr) -> u8 {
        Psgtrgbr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sscafbh {
    #[doc = "Counter start is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 0x01,
}
impl Sscafbh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sscafbh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sscafbh {
    #[inline(always)]
    fn from(val: u8) -> Sscafbh {
        Sscafbh::from_bits(val)
    }
}
impl From<Sscafbh> for u8 {
    #[inline(always)]
    fn from(val: Sscafbh) -> u8 {
        Sscafbh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sscafbl {
    #[doc = "Counter start is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 0x01,
}
impl Sscafbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sscafbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sscafbl {
    #[inline(always)]
    fn from(val: u8) -> Sscafbl {
        Sscafbl::from_bits(val)
    }
}
impl From<Sscafbl> for u8 {
    #[inline(always)]
    fn from(val: Sscafbl) -> u8 {
        Sscafbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sscarbh {
    #[doc = "Counter start is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 0x01,
}
impl Sscarbh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sscarbh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sscarbh {
    #[inline(always)]
    fn from(val: u8) -> Sscarbh {
        Sscarbh::from_bits(val)
    }
}
impl From<Sscarbh> for u8 {
    #[inline(always)]
    fn from(val: Sscarbh) -> u8 {
        Sscarbh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sscarbl {
    #[doc = "Counter start is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 0x01,
}
impl Sscarbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sscarbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sscarbl {
    #[inline(always)]
    fn from(val: u8) -> Sscarbl {
        Sscarbl::from_bits(val)
    }
}
impl From<Sscarbl> for u8 {
    #[inline(always)]
    fn from(val: Sscarbl) -> u8 {
        Sscarbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sscbfah {
    #[doc = "Counter start is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 0x01,
}
impl Sscbfah {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sscbfah {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sscbfah {
    #[inline(always)]
    fn from(val: u8) -> Sscbfah {
        Sscbfah::from_bits(val)
    }
}
impl From<Sscbfah> for u8 {
    #[inline(always)]
    fn from(val: Sscbfah) -> u8 {
        Sscbfah::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sscbfal {
    #[doc = "Counter start is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 0x01,
}
impl Sscbfal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sscbfal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sscbfal {
    #[inline(always)]
    fn from(val: u8) -> Sscbfal {
        Sscbfal::from_bits(val)
    }
}
impl From<Sscbfal> for u8 {
    #[inline(always)]
    fn from(val: Sscbfal) -> u8 {
        Sscbfal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sscbrah {
    #[doc = "Counter start is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 0x01,
}
impl Sscbrah {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sscbrah {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sscbrah {
    #[inline(always)]
    fn from(val: u8) -> Sscbrah {
        Sscbrah::from_bits(val)
    }
}
impl From<Sscbrah> for u8 {
    #[inline(always)]
    fn from(val: Sscbrah) -> u8 {
        Sscbrah::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sscbral {
    #[doc = "Counter start is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 0x01,
}
impl Sscbral {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sscbral {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sscbral {
    #[inline(always)]
    fn from(val: u8) -> Sscbral {
        Sscbral::from_bits(val)
    }
}
impl From<Sscbral> for u8 {
    #[inline(always)]
    fn from(val: Sscbral) -> u8 {
        Sscbral::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sselca {
    #[doc = "Counter start is disable at the ELC_GPTA input"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the ELC_GPTA input"]
    _1 = 0x01,
}
impl Sselca {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sselca {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sselca {
    #[inline(always)]
    fn from(val: u8) -> Sselca {
        Sselca::from_bits(val)
    }
}
impl From<Sselca> for u8 {
    #[inline(always)]
    fn from(val: Sselca) -> u8 {
        Sselca::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sselcb {
    #[doc = "Counter start is disable at the ELC_GPTB input"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the ELC_GPTB input"]
    _1 = 0x01,
}
impl Sselcb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sselcb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sselcb {
    #[inline(always)]
    fn from(val: u8) -> Sselcb {
        Sselcb::from_bits(val)
    }
}
impl From<Sselcb> for u8 {
    #[inline(always)]
    fn from(val: Sselcb) -> u8 {
        Sselcb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sselcc {
    #[doc = "Counter start is disable at the ELC_GPTC input"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the ELC_GPTC input"]
    _1 = 0x01,
}
impl Sselcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sselcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sselcc {
    #[inline(always)]
    fn from(val: u8) -> Sselcc {
        Sselcc::from_bits(val)
    }
}
impl From<Sselcc> for u8 {
    #[inline(always)]
    fn from(val: Sselcc) -> u8 {
        Sselcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sselcd {
    #[doc = "Counter start is disable at the ELC_GPTD input"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the ELC_GPTD input"]
    _1 = 0x01,
}
impl Sselcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sselcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sselcd {
    #[inline(always)]
    fn from(val: u8) -> Sselcd {
        Sselcd::from_bits(val)
    }
}
impl From<Sselcd> for u8 {
    #[inline(always)]
    fn from(val: Sselcd) -> u8 {
        Sselcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sselce {
    #[doc = "Counter start is disable at the ELC_GPTE input"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the ELC_GPTE input"]
    _1 = 0x01,
}
impl Sselce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sselce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sselce {
    #[inline(always)]
    fn from(val: u8) -> Sselce {
        Sselce::from_bits(val)
    }
}
impl From<Sselce> for u8 {
    #[inline(always)]
    fn from(val: Sselce) -> u8 {
        Sselce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sselcf {
    #[doc = "Counter start is disable at the ELC_GPTF input"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the ELC_GPTF input"]
    _1 = 0x01,
}
impl Sselcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sselcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sselcf {
    #[inline(always)]
    fn from(val: u8) -> Sselcf {
        Sselcf::from_bits(val)
    }
}
impl From<Sselcf> for u8 {
    #[inline(always)]
    fn from(val: Sselcf) -> u8 {
        Sselcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sselcg {
    #[doc = "Counter start is disable at the ELC_GPTG input"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the ELC_GPTG input"]
    _1 = 0x01,
}
impl Sselcg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sselcg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sselcg {
    #[inline(always)]
    fn from(val: u8) -> Sselcg {
        Sselcg::from_bits(val)
    }
}
impl From<Sselcg> for u8 {
    #[inline(always)]
    fn from(val: Sselcg) -> u8 {
        Sselcg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sselch {
    #[doc = "Counter start is disable at the ELC_GPTH input"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the ELC_GPTH input"]
    _1 = 0x01,
}
impl Sselch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sselch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sselch {
    #[inline(always)]
    fn from(val: u8) -> Sselch {
        Sselch::from_bits(val)
    }
}
impl From<Sselch> for u8 {
    #[inline(always)]
    fn from(val: Sselch) -> u8 {
        Sselch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssgtrgaf {
    #[doc = "Counter start is disable at the falling edge of GTETRGA input"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the falling edge of GTETRGA input"]
    _1 = 0x01,
}
impl Ssgtrgaf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssgtrgaf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssgtrgaf {
    #[inline(always)]
    fn from(val: u8) -> Ssgtrgaf {
        Ssgtrgaf::from_bits(val)
    }
}
impl From<Ssgtrgaf> for u8 {
    #[inline(always)]
    fn from(val: Ssgtrgaf) -> u8 {
        Ssgtrgaf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssgtrgar {
    #[doc = "Counter start is disable at the rising edge of GTETRGA input"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the rising edge of GTETRGA input"]
    _1 = 0x01,
}
impl Ssgtrgar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssgtrgar {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssgtrgar {
    #[inline(always)]
    fn from(val: u8) -> Ssgtrgar {
        Ssgtrgar::from_bits(val)
    }
}
impl From<Ssgtrgar> for u8 {
    #[inline(always)]
    fn from(val: Ssgtrgar) -> u8 {
        Ssgtrgar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssgtrgbf {
    #[doc = "Counter start is disable at the falling edge of GTETRGB input"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the falling edge of GTETRGB input"]
    _1 = 0x01,
}
impl Ssgtrgbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssgtrgbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssgtrgbf {
    #[inline(always)]
    fn from(val: u8) -> Ssgtrgbf {
        Ssgtrgbf::from_bits(val)
    }
}
impl From<Ssgtrgbf> for u8 {
    #[inline(always)]
    fn from(val: Ssgtrgbf) -> u8 {
        Ssgtrgbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssgtrgbr {
    #[doc = "Counter start is disable at the rising edge of GTETRGB input"]
    _0 = 0x0,
    #[doc = "Counter start is enable at the rising edge of GTETRGB input"]
    _1 = 0x01,
}
impl Ssgtrgbr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssgtrgbr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssgtrgbr {
    #[inline(always)]
    fn from(val: u8) -> Ssgtrgbr {
        Ssgtrgbr::from_bits(val)
    }
}
impl From<Ssgtrgbr> for u8 {
    #[inline(always)]
    fn from(val: Ssgtrgbr) -> u8 {
        Ssgtrgbr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcfa {
    #[doc = "No input capture/compare match of GTCCRA is generated."]
    _0 = 0x0,
    #[doc = "An input capture/compare match of GTCCRA is generated."]
    _1 = 0x01,
}
impl Tcfa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcfa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcfa {
    #[inline(always)]
    fn from(val: u8) -> Tcfa {
        Tcfa::from_bits(val)
    }
}
impl From<Tcfa> for u8 {
    #[inline(always)]
    fn from(val: Tcfa) -> u8 {
        Tcfa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcfb {
    #[doc = "No input capture/compare match of GTCCRB is generated."]
    _0 = 0x0,
    #[doc = "An input capture/compare match of GTCCRB is generated."]
    _1 = 0x01,
}
impl Tcfb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcfb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcfb {
    #[inline(always)]
    fn from(val: u8) -> Tcfb {
        Tcfb::from_bits(val)
    }
}
impl From<Tcfb> for u8 {
    #[inline(always)]
    fn from(val: Tcfb) -> u8 {
        Tcfb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcfc {
    #[doc = "No compare match of GTCCRC is generated."]
    _0 = 0x0,
    #[doc = "A compare match of GTCCRC is generated."]
    _1 = 0x01,
}
impl Tcfc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcfc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcfc {
    #[inline(always)]
    fn from(val: u8) -> Tcfc {
        Tcfc::from_bits(val)
    }
}
impl From<Tcfc> for u8 {
    #[inline(always)]
    fn from(val: Tcfc) -> u8 {
        Tcfc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcfd {
    #[doc = "No compare match of GTCCRD is generated."]
    _0 = 0x0,
    #[doc = "A compare match of GTCCRD is generated."]
    _1 = 0x01,
}
impl Tcfd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcfd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcfd {
    #[inline(always)]
    fn from(val: u8) -> Tcfd {
        Tcfd::from_bits(val)
    }
}
impl From<Tcfd> for u8 {
    #[inline(always)]
    fn from(val: Tcfd) -> u8 {
        Tcfd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcfe {
    #[doc = "No compare match of GTCCRE is generated."]
    _0 = 0x0,
    #[doc = "A compare match of GTCCRE is generated."]
    _1 = 0x01,
}
impl Tcfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcfe {
    #[inline(always)]
    fn from(val: u8) -> Tcfe {
        Tcfe::from_bits(val)
    }
}
impl From<Tcfe> for u8 {
    #[inline(always)]
    fn from(val: Tcfe) -> u8 {
        Tcfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcff {
    #[doc = "No compare match of GTCCRF is generated."]
    _0 = 0x0,
    #[doc = "A compare match of GTCCRF is generated."]
    _1 = 0x01,
}
impl Tcff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcff {
    #[inline(always)]
    fn from(val: u8) -> Tcff {
        Tcff::from_bits(val)
    }
}
impl From<Tcff> for u8 {
    #[inline(always)]
    fn from(val: Tcff) -> u8 {
        Tcff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcfpo {
    #[doc = "No overflow (crest) has occurred."]
    _0 = 0x0,
    #[doc = "An overflow (crest) has occurred."]
    _1 = 0x01,
}
impl Tcfpo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcfpo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcfpo {
    #[inline(always)]
    fn from(val: u8) -> Tcfpo {
        Tcfpo::from_bits(val)
    }
}
impl From<Tcfpo> for u8 {
    #[inline(always)]
    fn from(val: Tcfpo) -> u8 {
        Tcfpo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcfpu {
    #[doc = "No underflow (trough) has occurred."]
    _0 = 0x0,
    #[doc = "An underflow (trough) has occurred."]
    _1 = 0x01,
}
impl Tcfpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcfpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcfpu {
    #[inline(always)]
    fn from(val: u8) -> Tcfpu {
        Tcfpu::from_bits(val)
    }
}
impl From<Tcfpu> for u8 {
    #[inline(always)]
    fn from(val: Tcfpu) -> u8 {
        Tcfpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tde {
    #[doc = "GTCCRB is set without using GTDVU and GTDVD."]
    _0 = 0x0,
    #[doc = "GTDVU and GTDVD are used to set the compare match value for negative-phase waveform with dead time automatically in GTCCRB."]
    _1 = 0x01,
}
impl Tde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tde {
    #[inline(always)]
    fn from(val: u8) -> Tde {
        Tde::from_bits(val)
    }
}
impl From<Tde> for u8 {
    #[inline(always)]
    fn from(val: Tde) -> u8 {
        Tde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpcs {
    #[doc = "PCLK/1"]
    _000 = 0x0,
    #[doc = "PCLK/4"]
    _001 = 0x01,
    #[doc = "PCLK/16"]
    _010 = 0x02,
    #[doc = "PCLK/64"]
    _011 = 0x03,
    #[doc = "PCLK/256"]
    _100 = 0x04,
    #[doc = "PCLK/1024"]
    _101 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tpcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpcs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpcs {
    #[inline(always)]
    fn from(val: u8) -> Tpcs {
        Tpcs::from_bits(val)
    }
}
impl From<Tpcs> for u8 {
    #[inline(always)]
    fn from(val: Tpcs) -> u8 {
        Tpcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tucf {
    #[doc = "The GTCNT counter counts downward."]
    _0 = 0x0,
    #[doc = "The GTCNT counter counts upward."]
    _1 = 0x01,
}
impl Tucf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tucf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tucf {
    #[inline(always)]
    fn from(val: u8) -> Tucf {
        Tucf::from_bits(val)
    }
}
impl From<Tucf> for u8 {
    #[inline(always)]
    fn from(val: Tucf) -> u8 {
        Tucf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ud {
    #[doc = "GTCNT counts down."]
    _0 = 0x0,
    #[doc = "GTCNT counts up."]
    _1 = 0x01,
}
impl Ud {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ud {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ud {
    #[inline(always)]
    fn from(val: u8) -> Ud {
        Ud::from_bits(val)
    }
}
impl From<Ud> for u8 {
    #[inline(always)]
    fn from(val: Ud) -> u8 {
        Ud::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Udf {
    #[doc = "Not forcibly set"]
    _0 = 0x0,
    #[doc = "Forcibly set"]
    _1 = 0x01,
}
impl Udf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Udf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Udf {
    #[inline(always)]
    fn from(val: u8) -> Udf {
        Udf::from_bits(val)
    }
}
impl From<Udf> for u8 {
    #[inline(always)]
    fn from(val: Udf) -> u8 {
        Udf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uscafbh {
    #[doc = "Counter count up is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 0x01,
}
impl Uscafbh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uscafbh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uscafbh {
    #[inline(always)]
    fn from(val: u8) -> Uscafbh {
        Uscafbh::from_bits(val)
    }
}
impl From<Uscafbh> for u8 {
    #[inline(always)]
    fn from(val: Uscafbh) -> u8 {
        Uscafbh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uscafbl {
    #[doc = "Counter count up is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 0x01,
}
impl Uscafbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uscafbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uscafbl {
    #[inline(always)]
    fn from(val: u8) -> Uscafbl {
        Uscafbl::from_bits(val)
    }
}
impl From<Uscafbl> for u8 {
    #[inline(always)]
    fn from(val: Uscafbl) -> u8 {
        Uscafbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uscarbh {
    #[doc = "Counter count up is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 0x01,
}
impl Uscarbh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uscarbh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uscarbh {
    #[inline(always)]
    fn from(val: u8) -> Uscarbh {
        Uscarbh::from_bits(val)
    }
}
impl From<Uscarbh> for u8 {
    #[inline(always)]
    fn from(val: Uscarbh) -> u8 {
        Uscarbh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uscarbl {
    #[doc = "Counter count up is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 0x01,
}
impl Uscarbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uscarbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uscarbl {
    #[inline(always)]
    fn from(val: u8) -> Uscarbl {
        Uscarbl::from_bits(val)
    }
}
impl From<Uscarbl> for u8 {
    #[inline(always)]
    fn from(val: Uscarbl) -> u8 {
        Uscarbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uscbfah {
    #[doc = "Counter count up is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 0x01,
}
impl Uscbfah {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uscbfah {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uscbfah {
    #[inline(always)]
    fn from(val: u8) -> Uscbfah {
        Uscbfah::from_bits(val)
    }
}
impl From<Uscbfah> for u8 {
    #[inline(always)]
    fn from(val: Uscbfah) -> u8 {
        Uscbfah::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uscbfal {
    #[doc = "Counter count up is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 0x01,
}
impl Uscbfal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uscbfal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uscbfal {
    #[inline(always)]
    fn from(val: u8) -> Uscbfal {
        Uscbfal::from_bits(val)
    }
}
impl From<Uscbfal> for u8 {
    #[inline(always)]
    fn from(val: Uscbfal) -> u8 {
        Uscbfal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uscbrah {
    #[doc = "Counter count up is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 0x01,
}
impl Uscbrah {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uscbrah {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uscbrah {
    #[inline(always)]
    fn from(val: u8) -> Uscbrah {
        Uscbrah::from_bits(val)
    }
}
impl From<Uscbrah> for u8 {
    #[inline(always)]
    fn from(val: Uscbrah) -> u8 {
        Uscbrah::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uscbral {
    #[doc = "Counter count up is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 0x01,
}
impl Uscbral {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uscbral {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uscbral {
    #[inline(always)]
    fn from(val: u8) -> Uscbral {
        Uscbral::from_bits(val)
    }
}
impl From<Uscbral> for u8 {
    #[inline(always)]
    fn from(val: Uscbral) -> u8 {
        Uscbral::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uselca {
    #[doc = "Counter count up is disable at the ELC_GPTA input"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the ELC_GPTA input"]
    _1 = 0x01,
}
impl Uselca {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uselca {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uselca {
    #[inline(always)]
    fn from(val: u8) -> Uselca {
        Uselca::from_bits(val)
    }
}
impl From<Uselca> for u8 {
    #[inline(always)]
    fn from(val: Uselca) -> u8 {
        Uselca::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uselcb {
    #[doc = "Counter count up is disable at the ELC_GPTB input"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the ELC_GPTB input"]
    _1 = 0x01,
}
impl Uselcb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uselcb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uselcb {
    #[inline(always)]
    fn from(val: u8) -> Uselcb {
        Uselcb::from_bits(val)
    }
}
impl From<Uselcb> for u8 {
    #[inline(always)]
    fn from(val: Uselcb) -> u8 {
        Uselcb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uselcc {
    #[doc = "Counter count up is disable at the ELC_GPTC input"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the ELC_GPTC input"]
    _1 = 0x01,
}
impl Uselcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uselcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uselcc {
    #[inline(always)]
    fn from(val: u8) -> Uselcc {
        Uselcc::from_bits(val)
    }
}
impl From<Uselcc> for u8 {
    #[inline(always)]
    fn from(val: Uselcc) -> u8 {
        Uselcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uselcd {
    #[doc = "Counter count up is disable at the ELC_GPTD input"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the ELC_GPTD input"]
    _1 = 0x01,
}
impl Uselcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uselcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uselcd {
    #[inline(always)]
    fn from(val: u8) -> Uselcd {
        Uselcd::from_bits(val)
    }
}
impl From<Uselcd> for u8 {
    #[inline(always)]
    fn from(val: Uselcd) -> u8 {
        Uselcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uselce {
    #[doc = "Counter count up is disable at the ELC_GPTE input"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the ELC_GPTE input"]
    _1 = 0x01,
}
impl Uselce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uselce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uselce {
    #[inline(always)]
    fn from(val: u8) -> Uselce {
        Uselce::from_bits(val)
    }
}
impl From<Uselce> for u8 {
    #[inline(always)]
    fn from(val: Uselce) -> u8 {
        Uselce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uselcf {
    #[doc = "Counter count up is disable at the ELC_GPTF input"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the ELC_GPTF input"]
    _1 = 0x01,
}
impl Uselcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uselcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uselcf {
    #[inline(always)]
    fn from(val: u8) -> Uselcf {
        Uselcf::from_bits(val)
    }
}
impl From<Uselcf> for u8 {
    #[inline(always)]
    fn from(val: Uselcf) -> u8 {
        Uselcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uselcg {
    #[doc = "Counter count up is disable at the ELC_GPTG input"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the ELC_GPTG input"]
    _1 = 0x01,
}
impl Uselcg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uselcg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uselcg {
    #[inline(always)]
    fn from(val: u8) -> Uselcg {
        Uselcg::from_bits(val)
    }
}
impl From<Uselcg> for u8 {
    #[inline(always)]
    fn from(val: Uselcg) -> u8 {
        Uselcg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uselch {
    #[doc = "Counter count up is disable at the ELC_GPTH input"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the ELC_GPTH input"]
    _1 = 0x01,
}
impl Uselch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uselch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uselch {
    #[inline(always)]
    fn from(val: u8) -> Uselch {
        Uselch::from_bits(val)
    }
}
impl From<Uselch> for u8 {
    #[inline(always)]
    fn from(val: Uselch) -> u8 {
        Uselch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usgtrgaf {
    #[doc = "Counter count up is disable at the falling edge of GTETRGA input"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the falling edge of GTETRGA input"]
    _1 = 0x01,
}
impl Usgtrgaf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usgtrgaf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usgtrgaf {
    #[inline(always)]
    fn from(val: u8) -> Usgtrgaf {
        Usgtrgaf::from_bits(val)
    }
}
impl From<Usgtrgaf> for u8 {
    #[inline(always)]
    fn from(val: Usgtrgaf) -> u8 {
        Usgtrgaf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usgtrgar {
    #[doc = "Counter count up is disable at the rising edge of GTETRGA input"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the rising edge of GTETRGA input"]
    _1 = 0x01,
}
impl Usgtrgar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usgtrgar {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usgtrgar {
    #[inline(always)]
    fn from(val: u8) -> Usgtrgar {
        Usgtrgar::from_bits(val)
    }
}
impl From<Usgtrgar> for u8 {
    #[inline(always)]
    fn from(val: Usgtrgar) -> u8 {
        Usgtrgar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usgtrgbf {
    #[doc = "Counter count up is disable at the falling edge of GTETRGB input"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the falling edge of GTETRGB input"]
    _1 = 0x01,
}
impl Usgtrgbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usgtrgbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usgtrgbf {
    #[inline(always)]
    fn from(val: u8) -> Usgtrgbf {
        Usgtrgbf::from_bits(val)
    }
}
impl From<Usgtrgbf> for u8 {
    #[inline(always)]
    fn from(val: Usgtrgbf) -> u8 {
        Usgtrgbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usgtrgbr {
    #[doc = "Counter count up is disable at the rising edge of GTETRGB input"]
    _0 = 0x0,
    #[doc = "Counter count up is enable at the rising edge of GTETRGB input"]
    _1 = 0x01,
}
impl Usgtrgbr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usgtrgbr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usgtrgbr {
    #[inline(always)]
    fn from(val: u8) -> Usgtrgbr {
        Usgtrgbr::from_bits(val)
    }
}
impl From<Usgtrgbr> for u8 {
    #[inline(always)]
    fn from(val: Usgtrgbr) -> u8 {
        Usgtrgbr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wp {
    #[doc = "Write to the register is enabled"]
    _0 = 0x0,
    #[doc = "Write to the register is disabled"]
    _1 = 0x01,
}
impl Wp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wp {
    #[inline(always)]
    fn from(val: u8) -> Wp {
        Wp::from_bits(val)
    }
}
impl From<Wp> for u8 {
    #[inline(always)]
    fn from(val: Wp) -> u8 {
        Wp::to_bits(val)
    }
}
