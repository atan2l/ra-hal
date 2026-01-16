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
    SubClock = 0x0,
    #[doc = "LOCO clock oscillator is selected."]
    Loco = 0x01,
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
pub enum RwkcntDayw {
    #[doc = "Sunday"]
    Sunday = 0x0,
    #[doc = "Monday"]
    Monday = 0x01,
    #[doc = "Tuesday"]
    Tuesday = 0x02,
    #[doc = "Wednesday"]
    Wednesday = 0x03,
    #[doc = "Thursday"]
    Thursday = 0x04,
    #[doc = "Friday"]
    Friday = 0x05,
    #[doc = "Saturday"]
    Saturday = 0x06,
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
