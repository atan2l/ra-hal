#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HocoEnable {
    #[doc = "Enable HOCO oscillation after a reset."]
    Enable = 0x0,
    #[doc = "Disable HOCO oscillation after a reset."]
    Disable = 0x01,
}
impl HocoEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HocoEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HocoEnable {
    #[inline(always)]
    fn from(val: u8) -> HocoEnable {
        HocoEnable::from_bits(val)
    }
}
impl From<HocoEnable> for u8 {
    #[inline(always)]
    fn from(val: HocoEnable) -> u8 {
        HocoEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HocoFrq {
    Mhz24 = 0x0,
    _RESERVED_1 = 0x01,
    Mhz32 = 0x02,
    _RESERVED_3 = 0x03,
    Mhz48 = 0x04,
    Mhz64 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl HocoFrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HocoFrq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HocoFrq {
    #[inline(always)]
    fn from(val: u8) -> HocoFrq {
        HocoFrq::from_bits(val)
    }
}
impl From<HocoFrq> for u8 {
    #[inline(always)]
    fn from(val: HocoFrq) -> u8 {
        HocoFrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IwdtCks {
    #[doc = "`IWDTCLK/1`"]
    _1 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "`IWDTCLK/16`"]
    _16 = 0x02,
    #[doc = "`IWDTCLK/32`"]
    _32 = 0x03,
    #[doc = "`IWDTCLK/64`"]
    _64 = 0x04,
    #[doc = "`IWDTCLK/256`"]
    _256 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "`IWDTCLK/128`"]
    _128 = 0x0f,
}
impl IwdtCks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IwdtCks {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IwdtCks {
    #[inline(always)]
    fn from(val: u8) -> IwdtCks {
        IwdtCks::from_bits(val)
    }
}
impl From<IwdtCks> for u8 {
    #[inline(always)]
    fn from(val: IwdtCks) -> u8 {
        IwdtCks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IwdtTops {
    #[doc = "128 cycles (007Fh)."]
    _128 = 0x0,
    #[doc = "512 cycles (01FFh)."]
    _512 = 0x01,
    #[doc = "1024 cycles (03FFh)."]
    _1024 = 0x02,
    #[doc = "2048 cycles (07FFh)."]
    _2048 = 0x03,
}
impl IwdtTops {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IwdtTops {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IwdtTops {
    #[inline(always)]
    fn from(val: u8) -> IwdtTops {
        IwdtTops::from_bits(val)
    }
}
impl From<IwdtTops> for u8 {
    #[inline(always)]
    fn from(val: IwdtTops) -> u8 {
        IwdtTops::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rpes {
    #[doc = "Window end position = 75%."]
    End75Pct = 0x0,
    #[doc = "Window end position = 50%."]
    End50Pct = 0x01,
    #[doc = "Window end position = 25%."]
    End25Pct = 0x02,
    #[doc = "Window end position = 0% (no window end position)."]
    None = 0x03,
}
impl Rpes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rpes {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rpes {
    #[inline(always)]
    fn from(val: u8) -> Rpes {
        Rpes::from_bits(val)
    }
}
impl From<Rpes> for u8 {
    #[inline(always)]
    fn from(val: Rpes) -> u8 {
        Rpes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rpss {
    #[doc = "Window start position = 25%."]
    Start25Pct = 0x0,
    #[doc = "Window start position = 50%."]
    Start50Pct = 0x01,
    #[doc = "Window start position = 75%."]
    Start75Pct = 0x02,
    #[doc = "Window start position = 100% (no window start position)."]
    None = 0x03,
}
impl Rpss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rpss {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rpss {
    #[inline(always)]
    fn from(val: u8) -> Rpss {
        Rpss::from_bits(val)
    }
}
impl From<Rpss> for u8 {
    #[inline(always)]
    fn from(val: Rpss) -> u8 {
        Rpss::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StartMode {
    #[doc = "Automatically activate after a reset (auto-start mode)."]
    AutoStart = 0x0,
    #[doc = "Stop after a reset (register-start mode)."]
    Disabled = 0x01,
}
impl StartMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StartMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StartMode {
    #[inline(always)]
    fn from(val: u8) -> StartMode {
        StartMode::from_bits(val)
    }
}
impl From<StartMode> for u8 {
    #[inline(always)]
    fn from(val: StartMode) -> u8 {
        StartMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UnderflowAction {
    Interrupt = 0x0,
    Reset = 0x01,
}
impl UnderflowAction {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnderflowAction {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnderflowAction {
    #[inline(always)]
    fn from(val: u8) -> UnderflowAction {
        UnderflowAction::from_bits(val)
    }
}
impl From<UnderflowAction> for u8 {
    #[inline(always)]
    fn from(val: UnderflowAction) -> u8 {
        UnderflowAction::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vdsel1 {
    #[doc = "Selects 3.84 V."]
    Volt384 = 0x0,
    #[doc = "Selects 2.82 V."]
    Volt282 = 0x01,
    #[doc = "Selects 2.51 V."]
    Volt251 = 0x02,
    #[doc = "Selects 1.90 V."]
    Volt190 = 0x03,
    #[doc = "Selects 1.70 V."]
    Volt170 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Vdsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vdsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vdsel1 {
    #[inline(always)]
    fn from(val: u8) -> Vdsel1 {
        Vdsel1::from_bits(val)
    }
}
impl From<Vdsel1> for u8 {
    #[inline(always)]
    fn from(val: Vdsel1) -> u8 {
        Vdsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WdtCks {
    _RESERVED_0 = 0x0,
    #[doc = "`PCLKB/1`"]
    _4 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "`PCLKB/64`"]
    _64 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "`PCLKB/512`"]
    _512 = 0x06,
    #[doc = "`PCLKB/2048`"]
    _2048 = 0x07,
    #[doc = "`PCLKB/8192`"]
    _8192 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "`PCLKB/128`"]
    _128 = 0x0f,
}
impl WdtCks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtCks {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtCks {
    #[inline(always)]
    fn from(val: u8) -> WdtCks {
        WdtCks::from_bits(val)
    }
}
impl From<WdtCks> for u8 {
    #[inline(always)]
    fn from(val: WdtCks) -> u8 {
        WdtCks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WdtTops {
    _1024 = 0x0,
    #[doc = "4096 cycles (0FFFh)."]
    _4096 = 0x01,
    #[doc = "8192 cycles (1FFFh)."]
    _8192 = 0x02,
    #[doc = "16384 cycles (3FFFh)."]
    _16384 = 0x03,
}
impl WdtTops {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtTops {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtTops {
    #[inline(always)]
    fn from(val: u8) -> WdtTops {
        WdtTops::from_bits(val)
    }
}
impl From<WdtTops> for u8 {
    #[inline(always)]
    fn from(val: WdtTops) -> u8 {
        WdtTops::to_bits(val)
    }
}
