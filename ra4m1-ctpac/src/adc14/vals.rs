#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc {
    #[doc = "1-time conversion (no addition; same as normal conversion)"]
    _000 = 0x0,
    #[doc = "2-time conversion (addition once)"]
    _001 = 0x01,
    #[doc = "3-time conversion (addition twice)"]
    _010 = 0x02,
    #[doc = "4-time conversion (addition three times)"]
    _011 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "16-time conversion (addition 15 times), can be set when selecting 12-bit accuracy."]
    _101 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Adc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc {
    #[inline(always)]
    fn from(val: u8) -> Adc {
        Adc::from_bits(val)
    }
}
impl From<Adc> for u8 {
    #[inline(always)]
    fn from(val: Adc) -> u8 {
        Adc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adcs {
    #[doc = "Single scan mode"]
    _00 = 0x0,
    #[doc = "Group scan mode"]
    _01 = 0x01,
    #[doc = "Continuous scan mode"]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Adcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adcs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adcs {
    #[inline(always)]
    fn from(val: u8) -> Adcs {
        Adcs::from_bits(val)
    }
}
impl From<Adcs> for u8 {
    #[inline(always)]
    fn from(val: Adcs) -> u8 {
        Adcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adndis {
    #[doc = "Disconnection detection is disabled"]
    _0000 = 0x0,
    #[doc = "Setting prohibited"]
    _0001 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Adndis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adndis {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adndis {
    #[inline(always)]
    fn from(val: u8) -> Adndis {
        Adndis::from_bits(val)
    }
}
impl From<Adndis> for u8 {
    #[inline(always)]
    fn from(val: Adndis) -> u8 {
        Adndis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adprc {
    #[doc = "A/D conversion is performed with 12-bit accuracy."]
    _00 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "A/D conversion is performed with 14-bit accuracy."]
    _11 = 0x03,
}
impl Adprc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adprc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adprc {
    #[inline(always)]
    fn from(val: u8) -> Adprc {
        Adprc::from_bits(val)
    }
}
impl From<Adprc> for u8 {
    #[inline(always)]
    fn from(val: Adprc) -> u8 {
        Adprc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpab {
    #[doc = "ADC140_WCMPM is output when window A comparison conditions are met OR window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    _00 = 0x0,
    #[doc = "S14ADWMELC0 is output when window A comparison conditions are met EXOR window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    _01 = 0x01,
    #[doc = "ADC140_WCMPM is output when window A comparison conditions are met and window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    _10 = 0x02,
    #[doc = "Setting prohibited."]
    _11 = 0x03,
}
impl Cmpab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpab {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpab {
    #[inline(always)]
    fn from(val: u8) -> Cmpab {
        Cmpab::from_bits(val)
    }
}
impl From<Cmpab> for u8 {
    #[inline(always)]
    fn from(val: Cmpab) -> u8 {
        Cmpab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpchb {
    #[doc = "AN000"]
    _0X00 = 0x0,
    #[doc = "AN001"]
    _0X01 = 0x01,
    #[doc = "AN002"]
    _0X02 = 0x02,
    #[doc = "AN003"]
    _0X03 = 0x03,
    #[doc = "AN004"]
    _0X04 = 0x04,
    #[doc = "AN005"]
    _0X05 = 0x05,
    #[doc = "AN006"]
    _0X06 = 0x06,
    #[doc = "AN007"]
    _0X07 = 0x07,
    #[doc = "AN008"]
    _0X08 = 0x08,
    #[doc = "AN009"]
    _0X09 = 0x09,
    #[doc = "AN010"]
    _0X0A = 0x0a,
    #[doc = "AN011"]
    _0X0B = 0x0b,
    #[doc = "AN012"]
    _0X0C = 0x0c,
    #[doc = "AN013"]
    _0X0D = 0x0d,
    #[doc = "AN014"]
    _0X0E = 0x0e,
    #[doc = "AN015"]
    _0X0F = 0x0f,
    #[doc = "AN016"]
    _0X10 = 0x10,
    #[doc = "AN017"]
    _0X11 = 0x11,
    #[doc = "AN018"]
    _0X12 = 0x12,
    #[doc = "AN019"]
    _0X13 = 0x13,
    #[doc = "AN020"]
    _0X14 = 0x14,
    #[doc = "AN021"]
    _0X15 = 0x15,
    #[doc = "AN022"]
    _0X16 = 0x16,
    #[doc = "AN023"]
    _0X17 = 0x17,
    #[doc = "AN024"]
    _0X18 = 0x18,
    #[doc = "AN025"]
    _0X19 = 0x19,
    #[doc = "AN026"]
    _0X1A = 0x1a,
    #[doc = "AN027"]
    _0X1B = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "Temperature sensor"]
    _0X20 = 0x20,
    #[doc = "Internal reference voltage"]
    _0X21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "No channel is selected"]
    _0X3F = 0x3f,
}
impl Cmpchb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpchb {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpchb {
    #[inline(always)]
    fn from(val: u8) -> Cmpchb {
        Cmpchb::from_bits(val)
    }
}
impl From<Cmpchb> for u8 {
    #[inline(always)]
    fn from(val: Cmpchb) -> u8 {
        Cmpchb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diagst {
    #[doc = "Self-diagnosis has never been executed since power-on."]
    _00 = 0x0,
    #[doc = "Self-diagnosis using the voltage of 0 V has been executed."]
    _01 = 0x01,
    #[doc = "Self-diagnosis using the voltage of reference power supply(VREFH) x 1/2 has been executed."]
    _10 = 0x02,
    #[doc = "Self-diagnosis using the voltage of reference power supply(VREFH) has been executed."]
    _11 = 0x03,
}
impl Diagst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diagst {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diagst {
    #[inline(always)]
    fn from(val: u8) -> Diagst {
        Diagst::from_bits(val)
    }
}
impl From<Diagst> for u8 {
    #[inline(always)]
    fn from(val: Diagst) -> u8 {
        Diagst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diagval {
    #[doc = "When the self-diagnosis fixation mode is selected, it set prohibits it."]
    _00 = 0x0,
    #[doc = "The self-diagnosis by using the voltage of 0V."]
    _01 = 0x01,
    #[doc = "The self-diagnosis by using the voltage of reference supply x 1/2."]
    _10 = 0x02,
    #[doc = "The self-diagnosis by using the voltage of the reference supply."]
    _11 = 0x03,
}
impl Diagval {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diagval {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diagval {
    #[inline(always)]
    fn from(val: u8) -> Diagval {
        Diagval::from_bits(val)
    }
}
impl From<Diagval> for u8 {
    #[inline(always)]
    fn from(val: Diagval) -> u8 {
        Diagval::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hvsel {
    #[doc = "AVCC0 is selected as the high-potential reference voltage"]
    _00 = 0x0,
    #[doc = "VREFH0 is selected as the high-potential reference voltage"]
    _01 = 0x01,
    #[doc = "Internal reference voltage is selected as the high-potential reference voltage"]
    _10 = 0x02,
    #[doc = "Internal node discharge. No reference voltage pin is selected."]
    _11 = 0x03,
}
impl Hvsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hvsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hvsel {
    #[inline(always)]
    fn from(val: u8) -> Hvsel {
        Hvsel::from_bits(val)
    }
}
impl From<Hvsel> for u8 {
    #[inline(always)]
    fn from(val: Hvsel) -> u8 {
        Hvsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvsel {
    #[doc = "AVSS0 is selected as the low-potential reference voltage"]
    _0 = 0x0,
    #[doc = "VREFL0 is selected as the low-potential reference voltage."]
    _1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lvsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvsel {
    #[inline(always)]
    fn from(val: u8) -> Lvsel {
        Lvsel::from_bits(val)
    }
}
impl From<Lvsel> for u8 {
    #[inline(always)]
    fn from(val: Lvsel) -> u8 {
        Lvsel::to_bits(val)
    }
}
