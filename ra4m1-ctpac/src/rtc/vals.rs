#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aadje {
    #[doc = "Automatic adjustment is disabled."]
    _0 = 0x0,
    #[doc = "Automatic adjustment is enabled."]
    _1 = 0x01,
}
impl Aadje {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aadje {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aadje {
    #[inline(always)]
    fn from(val: u8) -> Aadje {
        Aadje::from_bits(val)
    }
}
impl From<Aadje> for u8 {
    #[inline(always)]
    fn from(val: Aadje) -> u8 {
        Aadje::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aadjp {
    #[doc = "The RADJ.ADJ\\[5:0\\] setting value is adjusted from the count value of the prescaler every minute."]
    _0 = 0x0,
    #[doc = "The RADJ.ADJ\\[5:0\\] setting value is adjusted from the count value of the prescaler every 10 seconds."]
    _1 = 0x01,
}
impl Aadjp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aadjp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aadjp {
    #[inline(always)]
    fn from(val: u8) -> Aadjp {
        Aadjp::from_bits(val)
    }
}
impl From<Aadjp> for u8 {
    #[inline(always)]
    fn from(val: Aadjp) -> u8 {
        Aadjp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adj30 {
    #[doc = "Writing is invalid.(write) / In normal time operation, or 30-second adjustment has completed.(read)"]
    _0 = 0x0,
    #[doc = "30-second adjustment is executed.(write) / During 30-second adjustment.(read)"]
    _1 = 0x01,
}
impl Adj30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adj30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adj30 {
    #[inline(always)]
    fn from(val: u8) -> Adj30 {
        Adj30::from_bits(val)
    }
}
impl From<Adj30> for u8 {
    #[inline(always)]
    fn from(val: Adj30) -> u8 {
        Adj30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aie {
    #[doc = "An alarm interrupt request is disabled."]
    _0 = 0x0,
    #[doc = "An alarm interrupt request is enabled."]
    _1 = 0x01,
}
impl Aie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aie {
    #[inline(always)]
    fn from(val: u8) -> Aie {
        Aie::from_bits(val)
    }
}
impl From<Aie> for u8 {
    #[inline(always)]
    fn from(val: Aie) -> u8 {
        Aie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cie {
    #[doc = "A carry interrupt request is disabled."]
    _0 = 0x0,
    #[doc = "A carry interrupt request is enabled."]
    _1 = 0x01,
}
impl Cie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cie {
    #[inline(always)]
    fn from(val: u8) -> Cie {
        Cie::from_bits(val)
    }
}
impl From<Cie> for u8 {
    #[inline(always)]
    fn from(val: Cie) -> u8 {
        Cie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cntmd {
    #[doc = "The calendar count mode."]
    _0 = 0x0,
    #[doc = "The binary count mode."]
    _1 = 0x01,
}
impl Cntmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cntmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cntmd {
    #[inline(always)]
    fn from(val: u8) -> Cntmd {
        Cntmd::from_bits(val)
    }
}
impl From<Cntmd> for u8 {
    #[inline(always)]
    fn from(val: Cntmd) -> u8 {
        Cntmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hr24 {
    #[doc = "The RTC operates in 12-hour mode."]
    _0 = 0x0,
    #[doc = "The RTC operates in 24-hour mode."]
    _1 = 0x01,
}
impl Hr24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hr24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hr24 {
    #[inline(always)]
    fn from(val: u8) -> Hr24 {
        Hr24::from_bits(val)
    }
}
impl From<Hr24> for u8 {
    #[inline(always)]
    fn from(val: Hr24) -> u8 {
        Hr24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pes {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "A periodic interrupt is generated every 1/256 second((RCR4.RCKSEL = 0)./A periodic interrupt is generated every 1/128 second((RCR4.RCKSEL = 1)."]
    _0110 = 0x06,
    #[doc = "A periodic interrupt is generated every 1/128 second."]
    _0111 = 0x07,
    #[doc = "A periodic interrupt is generated every 1/64 second."]
    _1000 = 0x08,
    #[doc = "A periodic interrupt is generated every 1/32 second."]
    _1001 = 0x09,
    #[doc = "A periodic interrupt is generated every 1/16 second."]
    _1010 = 0x0a,
    #[doc = "A periodic interrupt is generated every 1/8 second."]
    _1011 = 0x0b,
    #[doc = "A periodic interrupt is generated every 1/4 second."]
    _1100 = 0x0c,
    #[doc = "A periodic interrupt is generated every 1/2 second."]
    _1101 = 0x0d,
    #[doc = "A periodic interrupt is generated every 1 second."]
    _1110 = 0x0e,
    #[doc = "A periodic interrupt is generated every 2 seconds."]
    _1111 = 0x0f,
}
impl Pes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pes {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pes {
    #[inline(always)]
    fn from(val: u8) -> Pes {
        Pes::from_bits(val)
    }
}
impl From<Pes> for u8 {
    #[inline(always)]
    fn from(val: Pes) -> u8 {
        Pes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pie {
    #[doc = "A periodic interrupt request is disabled."]
    _0 = 0x0,
    #[doc = "A periodic interrupt request is enabled."]
    _1 = 0x01,
}
impl Pie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pie {
    #[inline(always)]
    fn from(val: u8) -> Pie {
        Pie::from_bits(val)
    }
}
impl From<Pie> for u8 {
    #[inline(always)]
    fn from(val: Pie) -> u8 {
        Pie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pmadj {
    #[doc = "Adjustment is not performed."]
    _00 = 0x0,
    #[doc = "Adjustment is performed by the addition to the prescaler."]
    _01 = 0x01,
    #[doc = "Adjustment is performed by the subtraction from the prescaler."]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Pmadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pmadj {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pmadj {
    #[inline(always)]
    fn from(val: u8) -> Pmadj {
        Pmadj::from_bits(val)
    }
}
impl From<Pmadj> for u8 {
    #[inline(always)]
    fn from(val: Pmadj) -> u8 {
        Pmadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcksel {
    #[doc = "Sub-clock oscillator is selected."]
    _0 = 0x0,
    #[doc = "LOCO clock oscillator is selected."]
    _1 = 0x01,
}
impl Rcksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcksel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcksel {
    #[inline(always)]
    fn from(val: u8) -> Rcksel {
        Rcksel::from_bits(val)
    }
}
impl From<Rcksel> for u8 {
    #[inline(always)]
    fn from(val: Rcksel) -> u8 {
        Rcksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RdayarEnb {
    #[doc = "The register value is not compared with the RDAYCNT counter value."]
    _0 = 0x0,
    #[doc = "The register value is compared with the RDAYCNT counter value."]
    _1 = 0x01,
}
impl RdayarEnb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RdayarEnb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RdayarEnb {
    #[inline(always)]
    fn from(val: u8) -> RdayarEnb {
        RdayarEnb::from_bits(val)
    }
}
impl From<RdayarEnb> for u8 {
    #[inline(always)]
    fn from(val: RdayarEnb) -> u8 {
        RdayarEnb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reset {
    #[doc = "Writing is invalid.(write) / In normal time operation, or an RTC software reset has completed.(read)"]
    _0 = 0x0,
    #[doc = "The prescaler and the target registers for RTC software reset *1 are initialized.(write) / During an RTC software reset.(read)"]
    _1 = 0x01,
}
impl Reset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reset {
    #[inline(always)]
    fn from(val: u8) -> Reset {
        Reset::from_bits(val)
    }
}
impl From<Reset> for u8 {
    #[inline(always)]
    fn from(val: Reset) -> u8 {
        Reset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RhrarEnb {
    #[doc = "The register value is not compared with the RHRCNT counter value."]
    _0 = 0x0,
    #[doc = "The register value is compared with the RHRCNT counter value."]
    _1 = 0x01,
}
impl RhrarEnb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RhrarEnb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RhrarEnb {
    #[inline(always)]
    fn from(val: u8) -> RhrarEnb {
        RhrarEnb::from_bits(val)
    }
}
impl From<RhrarEnb> for u8 {
    #[inline(always)]
    fn from(val: RhrarEnb) -> u8 {
        RhrarEnb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RhrarPm {
    #[doc = "a.m."]
    _0 = 0x0,
    #[doc = "p.m."]
    _1 = 0x01,
}
impl RhrarPm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RhrarPm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RhrarPm {
    #[inline(always)]
    fn from(val: u8) -> RhrarPm {
        RhrarPm::from_bits(val)
    }
}
impl From<RhrarPm> for u8 {
    #[inline(always)]
    fn from(val: RhrarPm) -> u8 {
        RhrarPm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RhrcntPm {
    #[doc = "a.m."]
    _0 = 0x0,
    #[doc = "p.m."]
    _1 = 0x01,
}
impl RhrcntPm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RhrcntPm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RhrcntPm {
    #[inline(always)]
    fn from(val: u8) -> RhrcntPm {
        RhrcntPm::from_bits(val)
    }
}
impl From<RhrcntPm> for u8 {
    #[inline(always)]
    fn from(val: RhrcntPm) -> u8 {
        RhrcntPm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RhrcpPm {
    #[doc = "a.m."]
    _0 = 0x0,
    #[doc = "p.m."]
    _1 = 0x01,
}
impl RhrcpPm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RhrcpPm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RhrcpPm {
    #[inline(always)]
    fn from(val: u8) -> RhrcpPm {
        RhrcpPm::from_bits(val)
    }
}
impl From<RhrcpPm> for u8 {
    #[inline(always)]
    fn from(val: RhrcpPm) -> u8 {
        RhrcpPm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RminarEnb {
    #[doc = "The register value is not compared with the RMINCNT counter value."]
    _0 = 0x0,
    #[doc = "The register value is compared with the RMINCNT counter value."]
    _1 = 0x01,
}
impl RminarEnb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RminarEnb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RminarEnb {
    #[inline(always)]
    fn from(val: u8) -> RminarEnb {
        RminarEnb::from_bits(val)
    }
}
impl From<RminarEnb> for u8 {
    #[inline(always)]
    fn from(val: RminarEnb) -> u8 {
        RminarEnb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RmonarEnb {
    #[doc = "The register value is not compared with the RMONCNT counter value."]
    _0 = 0x0,
    #[doc = "The register value is compared with the RMONCNT counter value."]
    _1 = 0x01,
}
impl RmonarEnb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RmonarEnb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RmonarEnb {
    #[inline(always)]
    fn from(val: u8) -> RmonarEnb {
        RmonarEnb::from_bits(val)
    }
}
impl From<RmonarEnb> for u8 {
    #[inline(always)]
    fn from(val: RmonarEnb) -> u8 {
        RmonarEnb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RsecarEnb {
    #[doc = "The register value is not compared with the RSECCNT counter value."]
    _0 = 0x0,
    #[doc = "The register value is compared with the RSECCNT counter value."]
    _1 = 0x01,
}
impl RsecarEnb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RsecarEnb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RsecarEnb {
    #[inline(always)]
    fn from(val: u8) -> RsecarEnb {
        RsecarEnb::from_bits(val)
    }
}
impl From<RsecarEnb> for u8 {
    #[inline(always)]
    fn from(val: RsecarEnb) -> u8 {
        RsecarEnb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtcoe {
    #[doc = "RTCOUT output disabled."]
    _0 = 0x0,
    #[doc = "RTCOUT output enabled."]
    _1 = 0x01,
}
impl Rtcoe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtcoe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtcoe {
    #[inline(always)]
    fn from(val: u8) -> Rtcoe {
        Rtcoe::from_bits(val)
    }
}
impl From<Rtcoe> for u8 {
    #[inline(always)]
    fn from(val: Rtcoe) -> u8 {
        Rtcoe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtcos {
    #[doc = "RTCOUT outputs 1 Hz."]
    _0 = 0x0,
    #[doc = "RTCOUT outputs 64 Hz."]
    _1 = 0x01,
}
impl Rtcos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtcos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtcos {
    #[inline(always)]
    fn from(val: u8) -> Rtcos {
        Rtcos::from_bits(val)
    }
}
impl From<Rtcos> for u8 {
    #[inline(always)]
    fn from(val: Rtcos) -> u8 {
        Rtcos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RwkarDayw {
    #[doc = "Sunday"]
    _000 = 0x0,
    #[doc = "Monday"]
    _001 = 0x01,
    #[doc = "Tuesday"]
    _010 = 0x02,
    #[doc = "Wednesday"]
    _011 = 0x03,
    #[doc = "Thursday"]
    _100 = 0x04,
    #[doc = "Friday"]
    _101 = 0x05,
    #[doc = "Saturday"]
    _110 = 0x06,
    #[doc = "Setting Prohibited"]
    _111 = 0x07,
}
impl RwkarDayw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RwkarDayw {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RwkarDayw {
    #[inline(always)]
    fn from(val: u8) -> RwkarDayw {
        RwkarDayw::from_bits(val)
    }
}
impl From<RwkarDayw> for u8 {
    #[inline(always)]
    fn from(val: RwkarDayw) -> u8 {
        RwkarDayw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RwkarEnb {
    #[doc = "The register value is not compared with the RWKCNT counter value."]
    _0 = 0x0,
    #[doc = "The register value is compared with the RWKCNT counter value."]
    _1 = 0x01,
}
impl RwkarEnb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RwkarEnb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RwkarEnb {
    #[inline(always)]
    fn from(val: u8) -> RwkarEnb {
        RwkarEnb::from_bits(val)
    }
}
impl From<RwkarEnb> for u8 {
    #[inline(always)]
    fn from(val: RwkarEnb) -> u8 {
        RwkarEnb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RwkcntDayw {
    #[doc = "Sunday"]
    _000 = 0x0,
    #[doc = "Monday"]
    _001 = 0x01,
    #[doc = "Tuesday"]
    _010 = 0x02,
    #[doc = "Wednesday"]
    _011 = 0x03,
    #[doc = "Thursday"]
    _100 = 0x04,
    #[doc = "Friday"]
    _101 = 0x05,
    #[doc = "Saturday"]
    _110 = 0x06,
    #[doc = "Setting Prohibited"]
    _111 = 0x07,
}
impl RwkcntDayw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RwkcntDayw {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RwkcntDayw {
    #[inline(always)]
    fn from(val: u8) -> RwkcntDayw {
        RwkcntDayw::from_bits(val)
    }
}
impl From<RwkcntDayw> for u8 {
    #[inline(always)]
    fn from(val: RwkcntDayw) -> u8 {
        RwkcntDayw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RyrarenEnb {
    #[doc = "The register value is not compared with the RYRCNT counter value."]
    _0 = 0x0,
    #[doc = "The register value is compared with the RYRCNT counter value."]
    _1 = 0x01,
}
impl RyrarenEnb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RyrarenEnb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RyrarenEnb {
    #[inline(always)]
    fn from(val: u8) -> RyrarenEnb {
        RyrarenEnb::from_bits(val)
    }
}
impl From<RyrarenEnb> for u8 {
    #[inline(always)]
    fn from(val: RyrarenEnb) -> u8 {
        RyrarenEnb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Start {
    #[doc = "Prescaler and time counter are stopped."]
    _0 = 0x0,
    #[doc = "Prescaler and time counter operate normally."]
    _1 = 0x01,
}
impl Start {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Start {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Start {
    #[inline(always)]
    fn from(val: u8) -> Start {
        Start::from_bits(val)
    }
}
impl From<Start> for u8 {
    #[inline(always)]
    fn from(val: Start) -> u8 {
        Start::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcct {
    #[doc = "No event is detected."]
    _00 = 0x0,
    #[doc = "Rising edge is detected."]
    _01 = 0x01,
    #[doc = "Falling edge is detected."]
    _10 = 0x02,
    #[doc = "Both edges are detected."]
    _11 = 0x03,
}
impl Tcct {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcct {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcct {
    #[inline(always)]
    fn from(val: u8) -> Tcct {
        Tcct::from_bits(val)
    }
}
impl From<Tcct> for u8 {
    #[inline(always)]
    fn from(val: Tcct) -> u8 {
        Tcct::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcnf {
    #[doc = "The noise filter is off."]
    _00 = 0x0,
    #[doc = "Setting prohibited"]
    _01 = 0x01,
    #[doc = "The noise filter is on (count source)."]
    _10 = 0x02,
    #[doc = "The noise filter is on (count source by divided by 32)."]
    _11 = 0x03,
}
impl Tcnf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcnf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcnf {
    #[inline(always)]
    fn from(val: u8) -> Tcnf {
        Tcnf::from_bits(val)
    }
}
impl From<Tcnf> for u8 {
    #[inline(always)]
    fn from(val: Tcnf) -> u8 {
        Tcnf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcst {
    #[doc = "No event is detected."]
    _0 = 0x0,
    #[doc = "An event is detected."]
    _1 = 0x01,
}
impl Tcst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcst {
    #[inline(always)]
    fn from(val: u8) -> Tcst {
        Tcst::from_bits(val)
    }
}
impl From<Tcst> for u8 {
    #[inline(always)]
    fn from(val: Tcst) -> u8 {
        Tcst::to_bits(val)
    }
}
