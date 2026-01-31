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
pub enum Ccra {
    #[doc = "Buffer operation is not performed"]
    NoBuffer = 0x0,
    #[doc = "Single buffer operation (GTCCRA <--> GTCCRC)"]
    SingleBuffer = 0x01,
    #[doc = "Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
    DoubleBuffer = 0x02,
    _RESERVED_3 = 0x03,
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
    NoBuffer = 0x0,
    #[doc = "Single buffer operation (GTCCRB <--> GTCCRE)"]
    SingleBuffer = 0x01,
    #[doc = "Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
    DoubleBuffer = 0x02,
    _RESERVED_3 = 0x03,
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
pub enum Mode {
    #[doc = "Saw-wave PWM mode (single buffer or double buffer possible)"]
    SawWavePwm = 0x0,
    #[doc = "Saw-wave one-shot pulse mode (fixed buffer operation)"]
    SawWaveOneShot = 0x01,
    #[doc = "Setting prohibited"]
    _010 = 0x02,
    #[doc = "Setting prohibited"]
    _011 = 0x03,
    #[doc = "Triangle-wave PWM mode 1 (16-bit transfer at crest) (single buffer or double buffer possible)"]
    TrianglePwm1 = 0x04,
    #[doc = "Triangle-wave PWM mode 2 (16-bit transfer at crest and trough) (single buffer or double buffer possible)"]
    TrianglePwm2 = 0x05,
    #[doc = "Triangle-wave PWM mode 3 (32-bit transfer at trough) fixed buffer operation)"]
    TrianglePwm3 = 0x06,
    #[doc = "Setting prohibited"]
    _111 = 0x07,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
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
pub enum Tpcs {
    #[doc = "PCLK/1"]
    DIV_1 = 0x0,
    #[doc = "PCLK/4"]
    DIV_4 = 0x01,
    #[doc = "PCLK/16"]
    DIV_16 = 0x02,
    #[doc = "PCLK/64"]
    DIV_64 = 0x03,
    #[doc = "PCLK/256"]
    DIV_256 = 0x04,
    #[doc = "PCLK/1024"]
    DIV_1024 = 0x05,
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
pub enum Ud {
    #[doc = "GTCNT counts down."]
    Down = 0x0,
    #[doc = "GTCNT counts up."]
    Up = 0x01,
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
