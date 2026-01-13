#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bkracs {
    #[doc = "Access control disable. When System clock source is SOSC or LOCO."]
    _000 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Access control enable. System clock source is other than SOSC or LOCO."]
    _110 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Bkracs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bkracs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bkracs {
    #[inline(always)]
    fn from(val: u8) -> Bkracs {
        Bkracs::from_bits(val)
    }
}
impl From<Bkracs> for u8 {
    #[inline(always)]
    fn from(val: Bkracs) -> u8 {
        Bkracs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ckodiv {
    #[doc = "/1"]
    _000 = 0x0,
    #[doc = "/2"]
    _001 = 0x01,
    #[doc = "/4"]
    _010 = 0x02,
    #[doc = "/8"]
    _011 = 0x03,
    #[doc = "/16"]
    _100 = 0x04,
    #[doc = "/32"]
    _101 = 0x05,
    #[doc = "/64"]
    _110 = 0x06,
    #[doc = "/128"]
    _111 = 0x07,
}
impl Ckodiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ckodiv {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ckodiv {
    #[inline(always)]
    fn from(val: u8) -> Ckodiv {
        Ckodiv::from_bits(val)
    }
}
impl From<Ckodiv> for u8 {
    #[inline(always)]
    fn from(val: Ckodiv) -> u8 {
        Ckodiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ckosel {
    #[doc = "HOCO"]
    _000 = 0x0,
    #[doc = "MOCO"]
    _001 = 0x01,
    #[doc = "LOCO"]
    _010 = 0x02,
    #[doc = "MOSC"]
    _011 = 0x03,
    #[doc = "SOSC"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ckosel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ckosel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ckosel {
    #[inline(always)]
    fn from(val: u8) -> Ckosel {
        Ckosel::from_bits(val)
    }
}
impl From<Ckosel> for u8 {
    #[inline(always)]
    fn from(val: Ckosel) -> u8 {
        Ckosel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cksel {
    #[doc = "HOCO"]
    Hoco = 0x0,
    #[doc = "MOCO"]
    Moco = 0x01,
    #[doc = "LOCO"]
    Loco = 0x02,
    #[doc = "Main clock oscillator"]
    Mosc = 0x03,
    #[doc = "Sub-clock oscillator"]
    Sosc = 0x04,
    #[doc = "PLL"]
    Pll = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cksel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cksel {
    #[inline(always)]
    fn from(val: u8) -> Cksel {
        Cksel::from_bits(val)
    }
}
impl From<Cksel> for u8 {
    #[inline(always)]
    fn from(val: Cksel) -> u8 {
        Cksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fck {
    #[doc = "/1"]
    DIV_1 = 0x0,
    #[doc = "/2"]
    DIV_2 = 0x01,
    #[doc = "/4"]
    DIV_4 = 0x02,
    #[doc = "/8"]
    DIV_8 = 0x03,
    #[doc = "/16"]
    DIV_16 = 0x04,
    #[doc = "/32"]
    DIV_32 = 0x05,
    #[doc = "/64"]
    DIV_64 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Fck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fck {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fck {
    #[inline(always)]
    fn from(val: u8) -> Fck {
        Fck::from_bits(val)
    }
}
impl From<Fck> for u8 {
    #[inline(always)]
    fn from(val: Fck) -> u8 {
        Fck::to_bits(val)
    }
}
#[doc = "Reserved values are prohibited"]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hcfrq1 {
    #[doc = "24 MHz"]
    _24mhz = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "32 MHz"]
    _32mhz = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "48 MHz"]
    _48mhz = 0x04,
    #[doc = "64 MHz"]
    _64mhz = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Hcfrq1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hcfrq1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hcfrq1 {
    #[inline(always)]
    fn from(val: u8) -> Hcfrq1 {
        Hcfrq1::from_bits(val)
    }
}
impl From<Hcfrq1> for u8 {
    #[inline(always)]
    fn from(val: Hcfrq1) -> u8 {
        Hcfrq1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hcstp {
    #[doc = "HOCO is operating."]
    Start = 0x0,
    #[doc = "HOCO is stopped."]
    Stop = 0x01,
}
impl Hcstp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hcstp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hcstp {
    #[inline(always)]
    fn from(val: u8) -> Hcstp {
        Hcstp::from_bits(val)
    }
}
impl From<Hcstp> for u8 {
    #[inline(always)]
    fn from(val: Hcstp) -> u8 {
        Hcstp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsts {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "If HOCO frequency is other than 64MHz, should set the value to 101b."]
    _101 = 0x05,
    #[doc = "If HOCO frequency = 64MHz, should set the value to 110b."]
    _110 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Hsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsts {
    #[inline(always)]
    fn from(val: u8) -> Hsts {
        Hsts::from_bits(val)
    }
}
impl From<Hsts> for u8 {
    #[inline(always)]
    fn from(val: Hsts) -> u8 {
        Hsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ick {
    #[doc = "/1"]
    DIV_1 = 0x0,
    #[doc = "/2"]
    DIV_2 = 0x01,
    #[doc = "/4"]
    DIV_4 = 0x02,
    #[doc = "/8"]
    DIV_8 = 0x03,
    #[doc = "/16"]
    DIV_16 = 0x04,
    #[doc = "/32"]
    DIV_32 = 0x05,
    #[doc = "/64"]
    DIV_64 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ick {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ick {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ick {
    #[inline(always)]
    fn from(val: u8) -> Ick {
        Ick::from_bits(val)
    }
}
impl From<Ick> for u8 {
    #[inline(always)]
    fn from(val: Ick) -> u8 {
        Ick::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idtsel {
    #[doc = "When VCC>=Vdet (rise) is detected"]
    _00 = 0x0,
    #[doc = "When VCC<Vdet (drop) is detected"]
    _01 = 0x01,
    #[doc = "When drop and rise are detected"]
    _10 = 0x02,
    #[doc = "Settings prohibited"]
    _11 = 0x03,
}
impl Idtsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idtsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idtsel {
    #[inline(always)]
    fn from(val: u8) -> Idtsel {
        Idtsel::from_bits(val)
    }
}
impl From<Idtsel> for u8 {
    #[inline(always)]
    fn from(val: Idtsel) -> u8 {
        Idtsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lcdscksel {
    #[doc = "LOCO"]
    _000 = 0x0,
    #[doc = "SOSC"]
    _001 = 0x01,
    #[doc = "MOSC"]
    _010 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "HOCO"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lcdscksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcdscksel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcdscksel {
    #[inline(always)]
    fn from(val: u8) -> Lcdscksel {
        Lcdscksel::from_bits(val)
    }
}
impl From<Lcdscksel> for u8 {
    #[inline(always)]
    fn from(val: Lcdscksel) -> u8 {
        Lcdscksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvd1lvl {
    #[doc = "4.29V (Vdet1_0)"]
    _00000 = 0x0,
    #[doc = "4.14V (Vdet1_1)"]
    _00001 = 0x01,
    #[doc = "4.02V (Vdet1_2)"]
    _00010 = 0x02,
    #[doc = "3.84V (Vdet1_3)"]
    _00011 = 0x03,
    #[doc = "3.10V (Vdet1_4)"]
    _00100 = 0x04,
    #[doc = "3.00V (Vdet1_5)"]
    _00101 = 0x05,
    #[doc = "2.90V (Vdet1_6)"]
    _00110 = 0x06,
    #[doc = "2.79V (Vdet1_7)"]
    _00111 = 0x07,
    #[doc = "2.68V (Vdet1_8)"]
    _01000 = 0x08,
    #[doc = "2.58V (Vdet1_9)"]
    _01001 = 0x09,
    #[doc = "2.48V (Vdet1_A)"]
    _01010 = 0x0a,
    #[doc = "2.20V (Vdet1_B)"]
    _01011 = 0x0b,
    #[doc = "1.96V (Vdet1_C)"]
    _01100 = 0x0c,
    #[doc = "1.86V (Vdet1_D)"]
    _01101 = 0x0d,
    #[doc = "1.75V (Vdet1_E)"]
    _01110 = 0x0e,
    #[doc = "1.65V (Vdet1_F)"]
    _01111 = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
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
impl Lvd1lvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd1lvl {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd1lvl {
    #[inline(always)]
    fn from(val: u8) -> Lvd1lvl {
        Lvd1lvl::from_bits(val)
    }
}
impl From<Lvd1lvl> for u8 {
    #[inline(always)]
    fn from(val: Lvd1lvl) -> u8 {
        Lvd1lvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvd2lvl {
    #[doc = "4.29V (Vdet2_0)"]
    _000 = 0x0,
    #[doc = "4.14V (Vdet2_1)"]
    _001 = 0x01,
    #[doc = "4.02V (Vdet2_2)"]
    _010 = 0x02,
    #[doc = "3.84V (Vdet2_3)"]
    _011 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lvd2lvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd2lvl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd2lvl {
    #[inline(always)]
    fn from(val: u8) -> Lvd2lvl {
        Lvd2lvl::from_bits(val)
    }
}
impl From<Lvd2lvl> for u8 {
    #[inline(always)]
    fn from(val: Lvd2lvl) -> u8 {
        Lvd2lvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Msts {
    #[doc = "Wait time = 2 cycles (0.25 us)"]
    _0000 = 0x0,
    #[doc = "Wait time = 1024 cycles (128 us)"]
    _0001 = 0x01,
    #[doc = "Wait time = 2048 cycles (256 us)"]
    _0010 = 0x02,
    #[doc = "Wait time = 4096 cycles (512 us)"]
    _0011 = 0x03,
    #[doc = "Wait time = 8192 cycles (1024 us)"]
    _0100 = 0x04,
    #[doc = "Wait time = 16384 cycles (2048 us) (value after reset)"]
    _0101 = 0x05,
    #[doc = "Wait time = 32768 cycles (4096 us)"]
    _0110 = 0x06,
    #[doc = "Wait time = 65536 cycles (8192 us)"]
    _0111 = 0x07,
    #[doc = "Wait time = 131072 cycles (16384 us)"]
    _1000 = 0x08,
    #[doc = "Wait time = 262144 cycles (32768 us)."]
    _1001 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Msts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msts {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msts {
    #[inline(always)]
    fn from(val: u8) -> Msts {
        Msts::from_bits(val)
    }
}
impl From<Msts> for u8 {
    #[inline(always)]
    fn from(val: Msts) -> u8 {
        Msts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opcm {
    #[doc = "High-speed mode"]
    HighSpeed = 0x0,
    #[doc = "Middle-speed mode"]
    MidSpeed = 0x01,
    #[doc = "Low-voltage mode"]
    LowVoltage = 0x02,
    #[doc = "Low-speed mode"]
    LowSpeed = 0x03,
}
impl Opcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opcm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opcm {
    #[inline(always)]
    fn from(val: u8) -> Opcm {
        Opcm::from_bits(val)
    }
}
impl From<Opcm> for u8 {
    #[inline(always)]
    fn from(val: Opcm) -> u8 {
        Opcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcka {
    #[doc = "/1"]
    DIV_1 = 0x0,
    #[doc = "/2"]
    DIV_2 = 0x01,
    #[doc = "/4"]
    DIV_4 = 0x02,
    #[doc = "/8"]
    DIV_8 = 0x03,
    #[doc = "/16"]
    DIV_16 = 0x04,
    #[doc = "/32"]
    DIV_32 = 0x05,
    #[doc = "/64"]
    DIV_64 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pcka {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcka {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcka {
    #[inline(always)]
    fn from(val: u8) -> Pcka {
        Pcka::from_bits(val)
    }
}
impl From<Pcka> for u8 {
    #[inline(always)]
    fn from(val: Pcka) -> u8 {
        Pcka::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pckb {
    #[doc = "/1"]
    DIV_1 = 0x0,
    #[doc = "/2"]
    DIV_2 = 0x01,
    #[doc = "/4"]
    DIV_4 = 0x02,
    #[doc = "/8"]
    DIV_8 = 0x03,
    #[doc = "/16"]
    DIV_16 = 0x04,
    #[doc = "/32"]
    DIV_32 = 0x05,
    #[doc = "/64"]
    DIV_64 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pckb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pckb {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pckb {
    #[inline(always)]
    fn from(val: u8) -> Pckb {
        Pckb::from_bits(val)
    }
}
impl From<Pckb> for u8 {
    #[inline(always)]
    fn from(val: Pckb) -> u8 {
        Pckb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pckc {
    #[doc = "/1"]
    DIV_1 = 0x0,
    #[doc = "/2"]
    DIV_2 = 0x01,
    #[doc = "/4"]
    DIV_4 = 0x02,
    #[doc = "/8"]
    DIV_8 = 0x03,
    #[doc = "/16"]
    DIV_16 = 0x04,
    #[doc = "/32"]
    DIV_32 = 0x05,
    #[doc = "/64"]
    DIV_64 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pckc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pckc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pckc {
    #[inline(always)]
    fn from(val: u8) -> Pckc {
        Pckc::from_bits(val)
    }
}
impl From<Pckc> for u8 {
    #[inline(always)]
    fn from(val: Pckc) -> u8 {
        Pckc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pckd {
    #[doc = "/1"]
    DIV_1 = 0x0,
    #[doc = "/2"]
    DIV_2 = 0x01,
    #[doc = "/4"]
    DIV_4 = 0x02,
    #[doc = "/8"]
    DIV_8 = 0x03,
    #[doc = "/16"]
    DIV_16 = 0x04,
    #[doc = "/32"]
    DIV_32 = 0x05,
    #[doc = "/64"]
    DIV_64 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pckd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pckd {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pckd {
    #[inline(always)]
    fn from(val: u8) -> Pckd {
        Pckd::from_bits(val)
    }
}
impl From<Pckd> for u8 {
    #[inline(always)]
    fn from(val: Pckd) -> u8 {
        Pckd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pllmul {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
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
    #[doc = "Settings prohibited."]
    _1111 = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
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
impl Pllmul {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllmul {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllmul {
    #[inline(always)]
    fn from(val: u8) -> Pllmul {
        Pllmul::from_bits(val)
    }
}
impl From<Pllmul> for u8 {
    #[inline(always)]
    fn from(val: Pllmul) -> u8 {
        Pllmul::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plodiv {
    #[doc = "/1."]
    _00 = 0x0,
    #[doc = "/2."]
    _01 = 0x01,
    #[doc = "/4."]
    _10 = 0x02,
    #[doc = "Setting prohibited."]
    _11 = 0x03,
}
impl Plodiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plodiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plodiv {
    #[inline(always)]
    fn from(val: u8) -> Plodiv {
        Plodiv::from_bits(val)
    }
}
impl From<Plodiv> for u8 {
    #[inline(always)]
    fn from(val: Plodiv) -> u8 {
        Plodiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prc0 {
    #[doc = "Writes protected."]
    Protected = 0x0,
    #[doc = "Writes not protected."]
    NotProtected = 0x01,
}
impl Prc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prc0 {
    #[inline(always)]
    fn from(val: u8) -> Prc0 {
        Prc0::from_bits(val)
    }
}
impl From<Prc0> for u8 {
    #[inline(always)]
    fn from(val: Prc0) -> u8 {
        Prc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prc1 {
    #[doc = "Writes protected."]
    Protected = 0x0,
    #[doc = "Writes not protected."]
    NotProtected = 0x01,
}
impl Prc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prc1 {
    #[inline(always)]
    fn from(val: u8) -> Prc1 {
        Prc1::from_bits(val)
    }
}
impl From<Prc1> for u8 {
    #[inline(always)]
    fn from(val: Prc1) -> u8 {
        Prc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prc3 {
    #[doc = "Writes protected."]
    Protected = 0x0,
    #[doc = "Writes not protected."]
    NotProtected = 0x01,
}
impl Prc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prc3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prc3 {
    #[inline(always)]
    fn from(val: u8) -> Prc3 {
        Prc3::from_bits(val)
    }
}
impl From<Prc3> for u8 {
    #[inline(always)]
    fn from(val: Prc3) -> u8 {
        Prc3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Prkey(u8);
impl Prkey {
    pub const PROTECT_KEY: Self = Self(0xa5);
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
            0xa5 => f.write_str("PROTECT_KEY"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prkey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xa5 => defmt::write!(f, "PROTECT_KEY"),
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
pub enum Sodrv {
    #[doc = "Normal mode"]
    _00 = 0x0,
    #[doc = "Low power mode 1"]
    _01 = 0x01,
    #[doc = "Low power mode 2"]
    _10 = 0x02,
    #[doc = "Low power mode 3."]
    _11 = 0x03,
}
impl Sodrv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sodrv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sodrv {
    #[inline(always)]
    fn from(val: u8) -> Sodrv {
        Sodrv::from_bits(val)
    }
}
impl From<Sodrv> for u8 {
    #[inline(always)]
    fn from(val: Sodrv) -> u8 {
        Sodrv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trck {
    #[doc = "/1"]
    _0000 = 0x0,
    #[doc = "/2(value after reset)"]
    _0001 = 0x01,
    #[doc = "/4"]
    _0010 = 0x02,
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
impl Trck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trck {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trck {
    #[inline(always)]
    fn from(val: u8) -> Trck {
        Trck::from_bits(val)
    }
}
impl From<Trck> for u8 {
    #[inline(always)]
    fn from(val: Trck) -> u8 {
        Trck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbtlvdlvl {
    #[doc = "2.7V"]
    _00 = 0x0,
    #[doc = "Setting prohibited"]
    _01 = 0x01,
    #[doc = "2.3V"]
    _10 = 0x02,
    #[doc = "2.1V"]
    _11 = 0x03,
}
impl Vbtlvdlvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbtlvdlvl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbtlvdlvl {
    #[inline(always)]
    fn from(val: u8) -> Vbtlvdlvl {
        Vbtlvdlvl::from_bits(val)
    }
}
impl From<Vbtlvdlvl> for u8 {
    #[inline(always)]
    fn from(val: Vbtlvdlvl) -> u8 {
        Vbtlvdlvl::to_bits(val)
    }
}
