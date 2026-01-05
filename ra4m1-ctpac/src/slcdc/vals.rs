#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Blon {
    #[doc = "Displaying an A-pattern area data (lower four bits of LCD display data register)(LCDSEL=0)/Displaying a B-pattern area data (higher four bits of LCD display data register)(LCDSEL=1)"]
    _0 = 0x0,
    #[doc = "Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))"]
    _1 = 0x01,
}
impl Blon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Blon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Blon {
    #[inline(always)]
    fn from(val: u8) -> Blon {
        Blon::from_bits(val)
    }
}
impl From<Blon> for u8 {
    #[inline(always)]
    fn from(val: Blon) -> u8 {
        Blon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lbas {
    #[doc = "1/2 bias method"]
    _00 = 0x0,
    #[doc = "1/3 bias method"]
    _01 = 0x01,
    #[doc = "1/4 bias method"]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Lbas {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lbas {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lbas {
    #[inline(always)]
    fn from(val: u8) -> Lbas {
        Lbas::from_bits(val)
    }
}
impl From<Lbas> for u8 {
    #[inline(always)]
    fn from(val: Lbas) -> u8 {
        Lbas::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lcdc {
    _RESERVED_0 = 0x0,
    #[doc = "(Sub clock)/22 or (LOCO clock)/22"]
    _000001 = 0x01,
    #[doc = "(Sub clock)/23 or (LOCO clock)/23"]
    _000010 = 0x02,
    #[doc = "(Sub clock)/24 or (LOCO clock)/24"]
    _000011 = 0x03,
    #[doc = "(Sub clock)/25 or (LOCO clock)/25"]
    _000100 = 0x04,
    #[doc = "(Sub clock)/26 or (LOCO clock)/26"]
    _000101 = 0x05,
    #[doc = "(Sub clock)/27 or (LOCO clock)/27"]
    _000110 = 0x06,
    #[doc = "(Sub clock)/28 or (LOCO clock)/28"]
    _000111 = 0x07,
    #[doc = "(Sub clock)/29 or (LOCO clock)/29"]
    _001000 = 0x08,
    #[doc = "(Sub clock)/210 or (LOCO clock)/210"]
    _001001 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "(Main clock)/28 or (HOCO clock)/28"]
    _010001 = 0x11,
    #[doc = "(Main clock)/29 or (HOCO clock)/29"]
    _010010 = 0x12,
    #[doc = "(Main clock)/210 or (HOCO clock)/210"]
    _010011 = 0x13,
    #[doc = "(Main clock)/211 or (HOCO clock)/211"]
    _010100 = 0x14,
    #[doc = "(Main clock)/212 or (HOCO clock)/212"]
    _010101 = 0x15,
    #[doc = "(Main clock)/213 or (HOCO clock)/213"]
    _010110 = 0x16,
    #[doc = "(Main clock)/214 or (HOCO clock)/214"]
    _010111 = 0x17,
    #[doc = "(Main clock)/215 or (HOCO clock)/215"]
    _011000 = 0x18,
    #[doc = "(Main clock)/216 or (HOCO clock)/216"]
    _011001 = 0x19,
    #[doc = "(Main clock)/217 or (HOCO clock)/217"]
    _011010 = 0x1a,
    #[doc = "(Main clock)/218 or (HOCO clock)/218"]
    _011011 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    #[doc = "(Main clock)/219 or (HOCO clock)/219"]
    _101011 = 0x2b,
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
    _RESERVED_3f = 0x3f,
}
impl Lcdc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcdc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcdc {
    #[inline(always)]
    fn from(val: u8) -> Lcdc {
        Lcdc::from_bits(val)
    }
}
impl From<Lcdc> for u8 {
    #[inline(always)]
    fn from(val: Lcdc) -> u8 {
        Lcdc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lcdon {
    #[doc = "Output ground level to segment/common pin(SCOC=0)/Display off (all segment outputs are deselected)(SCOC=1)"]
    _0 = 0x0,
    #[doc = "Output ground level to segment/common pin(SCOC=0)/Display on(SCOC=1)"]
    _1 = 0x01,
}
impl Lcdon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcdon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcdon {
    #[inline(always)]
    fn from(val: u8) -> Lcdon {
        Lcdon::from_bits(val)
    }
}
impl From<Lcdon> for u8 {
    #[inline(always)]
    fn from(val: Lcdon) -> u8 {
        Lcdon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lcdsel {
    #[doc = "Displaying an A-pattern area data (lower four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
    _0 = 0x0,
    #[doc = "Displaying a B-pattern area data (higher four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
    _1 = 0x01,
}
impl Lcdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcdsel {
    #[inline(always)]
    fn from(val: u8) -> Lcdsel {
        Lcdsel::from_bits(val)
    }
}
impl From<Lcdsel> for u8 {
    #[inline(always)]
    fn from(val: Lcdsel) -> u8 {
        Lcdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lcdvlm {
    #[doc = "Set when VDD >= 2.7 V"]
    _0 = 0x0,
    #[doc = "Set when VDD <= 4.2 V"]
    _1 = 0x01,
}
impl Lcdvlm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcdvlm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcdvlm {
    #[inline(always)]
    fn from(val: u8) -> Lcdvlm {
        Lcdvlm::from_bits(val)
    }
}
impl From<Lcdvlm> for u8 {
    #[inline(always)]
    fn from(val: Lcdvlm) -> u8 {
        Lcdvlm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ldty {
    #[doc = "Static"]
    _000 = 0x0,
    #[doc = "2-time slice"]
    _001 = 0x01,
    #[doc = "3-time slice"]
    _010 = 0x02,
    #[doc = "4-time slice"]
    _011 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "8-time slice"]
    _101 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ldty {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ldty {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ldty {
    #[inline(always)]
    fn from(val: u8) -> Ldty {
        Ldty::from_bits(val)
    }
}
impl From<Ldty> for u8 {
    #[inline(always)]
    fn from(val: Ldty) -> u8 {
        Ldty::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lwave {
    #[doc = "Waveform A"]
    _0 = 0x0,
    #[doc = "Waveform B"]
    _1 = 0x01,
}
impl Lwave {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lwave {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lwave {
    #[inline(always)]
    fn from(val: u8) -> Lwave {
        Lwave::from_bits(val)
    }
}
impl From<Lwave> for u8 {
    #[inline(always)]
    fn from(val: Lwave) -> u8 {
        Lwave::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mdset {
    #[doc = "External resistance division method"]
    _00 = 0x0,
    #[doc = "Internal voltage boosting method"]
    _01 = 0x01,
    #[doc = "Capacitor split method"]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Mdset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mdset {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mdset {
    #[inline(always)]
    fn from(val: u8) -> Mdset {
        Mdset::from_bits(val)
    }
}
impl From<Mdset> for u8 {
    #[inline(always)]
    fn from(val: Mdset) -> u8 {
        Mdset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scoc {
    #[doc = "Output ground level to segment/common pin(LCDON=0)/Output ground level to segment/common pin(LCDON=1)"]
    _0 = 0x0,
    #[doc = "Display off (all segment outputs are deselected)(LCDON=0)/Display on(LCDON=1)"]
    _1 = 0x01,
}
impl Scoc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scoc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scoc {
    #[inline(always)]
    fn from(val: u8) -> Scoc {
        Scoc::from_bits(val)
    }
}
impl From<Scoc> for u8 {
    #[inline(always)]
    fn from(val: Scoc) -> u8 {
        Scoc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vlcd {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Reference voltageselection(contrast adjustment): 1.00 V (default) VL4 voltage: 3.00 V(1/3 bias method)/4.00 V(1/4 bias method)"]
    _00100 = 0x04,
    #[doc = "Reference voltageselection(contrast adjustment): 1.05 V VL4 voltage: 3.15 V(1/3 bias method)/4.20 V(1/4 bias method)"]
    _00101 = 0x05,
    #[doc = "Reference voltageselection(contrast adjustment): 1.10 V VL4 voltage: 3.30 V(1/3 bias method)/4.40 V(1/4 bias method)"]
    _00110 = 0x06,
    #[doc = "Reference voltageselection(contrast adjustment): 1.15 V VL4 voltage: 3.45 V(1/3 bias method)/4.60 V(1/4 bias method)"]
    _00111 = 0x07,
    #[doc = "Reference voltageselection(contrast adjustment): 1.20 V VL4 voltage: 3.60 V(1/3 bias method)/4.80 V(1/4 bias method)"]
    _01000 = 0x08,
    #[doc = "Reference voltageselection(contrast adjustment): 1.25 V VL4 voltage: 3.75 V(1/3 bias method)/5.00 V(1/4 bias method)"]
    _01001 = 0x09,
    #[doc = "Reference voltageselection(contrast adjustment): 1.30 V VL4 voltage: 3.90 V(1/3 bias method)/5.20 V(1/4 bias method)"]
    _01010 = 0x0a,
    #[doc = "Reference voltageselection(contrast adjustment): 1.35 V VL4 voltage: 4.05 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _01011 = 0x0b,
    #[doc = "Reference voltageselection(contrast adjustment): 1.40 V VL4 voltage: 4.20 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _01100 = 0x0c,
    #[doc = "Reference voltageselection(contrast adjustment): 1.45 V VL4 voltage: 4.35 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _01101 = 0x0d,
    #[doc = "Reference voltageselection(contrast adjustment): 1.50 V VL4 voltage: 4.50 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _01110 = 0x0e,
    #[doc = "Reference voltageselection(contrast adjustment): 1.55 V VL4 voltage: 4.65 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _01111 = 0x0f,
    #[doc = "Reference voltageselection(contrast adjustment): 1.60 V VL4 voltage: 4.80 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _10000 = 0x10,
    #[doc = "Reference voltageselection(contrast adjustment): 1.65 V VL4 voltage: 4.95 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _10001 = 0x11,
    #[doc = "Reference voltageselection(contrast adjustment): 1.70 V VL4 voltage: 5.10 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _10010 = 0x12,
    #[doc = "Reference voltageselection(contrast adjustment): 1.75 V VL4 voltage: 5.25 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _10011 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Vlcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vlcd {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vlcd {
    #[inline(always)]
    fn from(val: u8) -> Vlcd {
        Vlcd::from_bits(val)
    }
}
impl From<Vlcd> for u8 {
    #[inline(always)]
    fn from(val: Vlcd) -> u8 {
        Vlcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vlcon {
    #[doc = "Stops voltage boost circuit or capacitor split circuit operation"]
    _0 = 0x0,
    #[doc = "Enables voltage boost circuit or capacitor split circuit operation"]
    _1 = 0x01,
}
impl Vlcon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vlcon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vlcon {
    #[inline(always)]
    fn from(val: u8) -> Vlcon {
        Vlcon::from_bits(val)
    }
}
impl From<Vlcon> for u8 {
    #[inline(always)]
    fn from(val: Vlcon) -> u8 {
        Vlcon::to_bits(val)
    }
}
