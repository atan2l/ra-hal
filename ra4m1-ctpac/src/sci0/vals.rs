#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bcp {
    #[doc = "93 clock cycles(S=93) (SCMR.BCP2=0) / 32 clock cycles(S=32) (SCMR.BCP2=1)"]
    _00 = 0x0,
    #[doc = "128 clock cycles(S=128) (SCMR.BCP2=0) / 64 clock cycles(S=64) (SCMR.BCP2=1)"]
    _01 = 0x01,
    #[doc = "186 clock cycles(S=186) (SCMR.BCP2=0) / 372 clock cycles(S=372) (SCMR.BCP2=1)"]
    _10 = 0x02,
    #[doc = "512 clock cycles(S=512) (SCMR.BCP2=0) / 256 clock cycles(S=256) (SCMR.BCP2=1)"]
    _11 = 0x03,
}
impl Bcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bcp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bcp {
    #[inline(always)]
    fn from(val: u8) -> Bcp {
        Bcp::from_bits(val)
    }
}
impl From<Bcp> for u8 {
    #[inline(always)]
    fn from(val: Bcp) -> u8 {
        Bcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicdl {
    #[doc = "No output delay"]
    _00000 = 0x0,
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
    _RESERVED_f = 0x0f,
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
impl Iicdl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicdl {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicdl {
    #[inline(always)]
    fn from(val: u8) -> Iicdl {
        Iicdl::from_bits(val)
    }
}
impl From<Iicdl> for u8 {
    #[inline(always)]
    fn from(val: Iicdl) -> u8 {
        Iicdl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicscls {
    #[doc = "Serial clock output"]
    _00 = 0x0,
    #[doc = "Generate a start, restart, or stop condition."]
    _01 = 0x01,
    #[doc = "Output the low level on the SSCLn pin."]
    _10 = 0x02,
    #[doc = "Place the SSCLn pin in the high-impedance state."]
    _11 = 0x03,
}
impl Iicscls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicscls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicscls {
    #[inline(always)]
    fn from(val: u8) -> Iicscls {
        Iicscls::from_bits(val)
    }
}
impl From<Iicscls> for u8 {
    #[inline(always)]
    fn from(val: Iicscls) -> u8 {
        Iicscls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicsdas {
    #[doc = "Serial data output"]
    _00 = 0x0,
    #[doc = "Generate a start, restart, or stop condition."]
    _01 = 0x01,
    #[doc = "Output the low level on the SSDAn pin."]
    _10 = 0x02,
    #[doc = "Place the SSDAn pin in the high-impedance state."]
    _11 = 0x03,
}
impl Iicsdas {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicsdas {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicsdas {
    #[inline(always)]
    fn from(val: u8) -> Iicsdas {
        Iicsdas::from_bits(val)
    }
}
impl From<Iicsdas> for u8 {
    #[inline(always)]
    fn from(val: Iicsdas) -> u8 {
        Iicsdas::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nfcs {
    #[doc = "The clock signal divided by 1 is used with the noise filter.(In asynchronous mode)"]
    _000 = 0x0,
    #[doc = "The clock signal divided by 1 is used with the noise filter.(In simple I2C mode)"]
    _001 = 0x01,
    #[doc = "The clock signal divided by 2 is used with the noise filter.(In simple I2C mode)"]
    _010 = 0x02,
    #[doc = "The clock signal divided by 4 is used with the noise filter.(In simple I2C mode)"]
    _011 = 0x03,
    #[doc = "The clock signal divided by 8 is used with the noise filter.(In simple I2C mode)"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Nfcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfcs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfcs {
    #[inline(always)]
    fn from(val: u8) -> Nfcs {
        Nfcs::from_bits(val)
    }
}
impl From<Nfcs> for u8 {
    #[inline(always)]
    fn from(val: Nfcs) -> u8 {
        Nfcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstrg {
    #[doc = "Trigger number 0"]
    _0000 = 0x0,
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
    _RESERVED_f = 0x0f,
}
impl Rstrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstrg {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstrg {
    #[inline(always)]
    fn from(val: u8) -> Rstrg {
        Rstrg::from_bits(val)
    }
}
impl From<Rstrg> for u8 {
    #[inline(always)]
    fn from(val: Rstrg) -> u8 {
        Rstrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtrg {
    #[doc = "Trigger number 0"]
    _0000 = 0x0,
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
    _RESERVED_f = 0x0f,
}
impl Rtrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtrg {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtrg {
    #[inline(always)]
    fn from(val: u8) -> Rtrg {
        Rtrg::from_bits(val)
    }
}
impl From<Rtrg> for u8 {
    #[inline(always)]
    fn from(val: Rtrg) -> u8 {
        Rtrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrCke {
    #[doc = "The SCKn pin is available for use as an I/O port in accord with the I/O port settings.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    _00 = 0x0,
    #[doc = "The clock with the same frequency as the bit rate is output from the SCKn pin.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    _01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl ScrCke {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrCke {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrCke {
    #[inline(always)]
    fn from(val: u8) -> ScrCke {
        ScrCke::from_bits(val)
    }
}
impl From<ScrCke> for u8 {
    #[inline(always)]
    fn from(val: ScrCke) -> u8 {
        ScrCke::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrSmciCke {
    #[doc = "Output disabled(SMR_SMCI.GM=0) / Output fixed low(SMR_SMCI.GM=1)"]
    _00 = 0x0,
    #[doc = "Clock Output"]
    _01 = 0x01,
    #[doc = "Setting prohibited(SMR_SMCI.GM=0) / Output fixed High(SMR_SMCI.GM=1)"]
    _10 = 0x02,
    #[doc = "Setting prohibited(SMR_SMCI.GM=0) / Clock Output(SMR_SMCI.GM=1)"]
    _11 = 0x03,
}
impl ScrSmciCke {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrSmciCke {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrSmciCke {
    #[inline(always)]
    fn from(val: u8) -> ScrSmciCke {
        ScrSmciCke::from_bits(val)
    }
}
impl From<ScrSmciCke> for u8 {
    #[inline(always)]
    fn from(val: ScrSmciCke) -> u8 {
        ScrSmciCke::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmrCks {
    #[doc = "PCLK clock"]
    _00 = 0x0,
    #[doc = "PCLK/4 clock"]
    _01 = 0x01,
    #[doc = "PCLK/16 clock"]
    _10 = 0x02,
    #[doc = "PCLK/64 clock"]
    _11 = 0x03,
}
impl SmrCks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmrCks {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmrCks {
    #[inline(always)]
    fn from(val: u8) -> SmrCks {
        SmrCks::from_bits(val)
    }
}
impl From<SmrCks> for u8 {
    #[inline(always)]
    fn from(val: SmrCks) -> u8 {
        SmrCks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmrSmciCks {
    #[doc = "PCLK clock"]
    _00 = 0x0,
    #[doc = "PCLK/4 clock"]
    _01 = 0x01,
    #[doc = "PCLK/16 clock"]
    _10 = 0x02,
    #[doc = "PCLK/64 clock"]
    _11 = 0x03,
}
impl SmrSmciCks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmrSmciCks {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmrSmciCks {
    #[inline(always)]
    fn from(val: u8) -> SmrSmciCks {
        SmrSmciCks::from_bits(val)
    }
}
impl From<SmrSmciCks> for u8 {
    #[inline(always)]
    fn from(val: SmrSmciCks) -> u8 {
        SmrSmciCks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ttrg {
    #[doc = "Trigger number 0"]
    _0000 = 0x0,
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
    _RESERVED_f = 0x0f,
}
impl Ttrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ttrg {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ttrg {
    #[inline(always)]
    fn from(val: u8) -> Ttrg {
        Ttrg::from_bits(val)
    }
}
impl From<Ttrg> for u8 {
    #[inline(always)]
    fn from(val: Ttrg) -> u8 {
        Ttrg::to_bits(val)
    }
}
