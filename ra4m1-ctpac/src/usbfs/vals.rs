#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CfifoselCurpipe {
    #[doc = "DCP (Default control pipe)"]
    _0000 = 0x0,
    #[doc = "Pipe 1"]
    _0001 = 0x01,
    #[doc = "Pipe 2"]
    _0010 = 0x02,
    #[doc = "Pipe 3"]
    _0011 = 0x03,
    #[doc = "Pipe 4"]
    _0100 = 0x04,
    #[doc = "Pipe 5"]
    _0101 = 0x05,
    #[doc = "Pipe 6"]
    _0110 = 0x06,
    #[doc = "Pipe 7"]
    _0111 = 0x07,
    #[doc = "Pipe 8"]
    _1000 = 0x08,
    #[doc = "Pipe 9"]
    _1001 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CfifoselCurpipe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CfifoselCurpipe {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CfifoselCurpipe {
    #[inline(always)]
    fn from(val: u8) -> CfifoselCurpipe {
        CfifoselCurpipe::from_bits(val)
    }
}
impl From<CfifoselCurpipe> for u8 {
    #[inline(always)]
    fn from(val: CfifoselCurpipe) -> u8 {
        CfifoselCurpipe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsq {
    #[doc = "Idle or setup stage"]
    _000 = 0x0,
    #[doc = "Control read data stage"]
    _001 = 0x01,
    #[doc = "Control read status stage"]
    _010 = 0x02,
    #[doc = "Control write data stage"]
    _011 = 0x03,
    #[doc = "Control write status stage"]
    _100 = 0x04,
    #[doc = "Control write (no data) status stage"]
    _101 = 0x05,
    #[doc = "Control transfer sequence error"]
    _110 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctsq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsq {
    #[inline(always)]
    fn from(val: u8) -> Ctsq {
        Ctsq::from_bits(val)
    }
}
impl From<Ctsq> for u8 {
    #[inline(always)]
    fn from(val: Ctsq) -> u8 {
        Ctsq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D0fifoselCurpipe {
    #[doc = "DCP (Default control pipe)"]
    _0000 = 0x0,
    #[doc = "Pipe 1"]
    _0001 = 0x01,
    #[doc = "Pipe 2"]
    _0010 = 0x02,
    #[doc = "Pipe 3"]
    _0011 = 0x03,
    #[doc = "Pipe 4"]
    _0100 = 0x04,
    #[doc = "Pipe 5"]
    _0101 = 0x05,
    #[doc = "Pipe 6"]
    _0110 = 0x06,
    #[doc = "Pipe 7"]
    _0111 = 0x07,
    #[doc = "Pipe 8"]
    _1000 = 0x08,
    #[doc = "Pipe 9"]
    _1001 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl D0fifoselCurpipe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D0fifoselCurpipe {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D0fifoselCurpipe {
    #[inline(always)]
    fn from(val: u8) -> D0fifoselCurpipe {
        D0fifoselCurpipe::from_bits(val)
    }
}
impl From<D0fifoselCurpipe> for u8 {
    #[inline(always)]
    fn from(val: D0fifoselCurpipe) -> u8 {
        D0fifoselCurpipe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D1fifoselCurpipe {
    #[doc = "DCP (Default control pipe)"]
    _0000 = 0x0,
    #[doc = "Pipe 1"]
    _0001 = 0x01,
    #[doc = "Pipe 2"]
    _0010 = 0x02,
    #[doc = "Pipe 3"]
    _0011 = 0x03,
    #[doc = "Pipe 4"]
    _0100 = 0x04,
    #[doc = "Pipe 5"]
    _0101 = 0x05,
    #[doc = "Pipe 6"]
    _0110 = 0x06,
    #[doc = "Pipe 7"]
    _0111 = 0x07,
    #[doc = "Pipe 8"]
    _1000 = 0x08,
    #[doc = "Pipe 9"]
    _1001 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl D1fifoselCurpipe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D1fifoselCurpipe {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D1fifoselCurpipe {
    #[inline(always)]
    fn from(val: u8) -> D1fifoselCurpipe {
        D1fifoselCurpipe::from_bits(val)
    }
}
impl From<D1fifoselCurpipe> for u8 {
    #[inline(always)]
    fn from(val: D1fifoselCurpipe) -> u8 {
        D1fifoselCurpipe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcpctrPid {
    #[doc = "NAK response"]
    _00 = 0x0,
    #[doc = "BUF response (depending on the buffer state)"]
    _01 = 0x01,
    #[doc = "STALL response"]
    _10 = 0x02,
    #[doc = "STALL response"]
    _11 = 0x03,
}
impl DcpctrPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcpctrPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcpctrPid {
    #[inline(always)]
    fn from(val: u8) -> DcpctrPid {
        DcpctrPid::from_bits(val)
    }
}
impl From<DcpctrPid> for u8 {
    #[inline(always)]
    fn from(val: DcpctrPid) -> u8 {
        DcpctrPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcpmaxpDevsel {
    #[doc = "Address 0000"]
    _0000 = 0x0,
    #[doc = "Address 0001"]
    _0001 = 0x01,
    #[doc = "Address 0010"]
    _0010 = 0x02,
    #[doc = "Address 0011"]
    _0011 = 0x03,
    #[doc = "Address 0100"]
    _0100 = 0x04,
    #[doc = "Address 0101"]
    _0101 = 0x05,
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
impl DcpmaxpDevsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcpmaxpDevsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcpmaxpDevsel {
    #[inline(always)]
    fn from(val: u8) -> DcpmaxpDevsel {
        DcpmaxpDevsel::from_bits(val)
    }
}
impl From<DcpmaxpDevsel> for u8 {
    #[inline(always)]
    fn from(val: DcpmaxpDevsel) -> u8 {
        DcpmaxpDevsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dvsq {
    #[doc = "Powered state"]
    _000 = 0x0,
    #[doc = "Default state"]
    _001 = 0x01,
    #[doc = "Address state"]
    _010 = 0x02,
    #[doc = "Configured state"]
    _011 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Dvsq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dvsq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dvsq {
    #[inline(always)]
    fn from(val: u8) -> Dvsq {
        Dvsq::from_bits(val)
    }
}
impl From<Dvsq> for u8 {
    #[inline(always)]
    fn from(val: Dvsq) -> u8 {
        Dvsq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lnst {
    #[doc = "SE0"]
    _00 = 0x0,
    #[doc = "K-State (FS) / J-State(LS)"]
    _01 = 0x01,
    #[doc = "J-State(FS) / K-State(LS)"]
    _10 = 0x02,
    #[doc = "SE1"]
    _11 = 0x03,
}
impl Lnst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lnst {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lnst {
    #[inline(always)]
    fn from(val: u8) -> Lnst {
        Lnst::from_bits(val)
    }
}
impl From<Lnst> for u8 {
    #[inline(always)]
    fn from(val: Lnst) -> u8 {
        Lnst::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mxps(u8);
impl Mxps {
    #[doc = "8 bytes"]
    pub const _0X08: Self = Self(0x08);
    #[doc = "16 bytes"]
    pub const _0X10: Self = Self(0x10);
    #[doc = "24 bytes"]
    pub const _0X18: Self = Self(0x18);
    #[doc = "32 bytes"]
    pub const _0X20: Self = Self(0x20);
    #[doc = "40 bytes"]
    pub const _0X28: Self = Self(0x28);
    #[doc = "48 bytes"]
    pub const _0X30: Self = Self(0x30);
    #[doc = "56 bytes"]
    pub const _0X38: Self = Self(0x38);
    #[doc = "64 bytes"]
    pub const _0X40: Self = Self(0x40);
    #[doc = "72 bytes"]
    pub const _0X48: Self = Self(0x48);
    #[doc = "80 bytes"]
    pub const _0X50: Self = Self(0x50);
    #[doc = "88 bytes"]
    pub const _0X58: Self = Self(0x58);
    #[doc = "96 bytes"]
    pub const _0X60: Self = Self(0x60);
    #[doc = "104 bytes"]
    pub const _0X68: Self = Self(0x68);
    #[doc = "112 bytes"]
    pub const _0X70: Self = Self(0x70);
    #[doc = "120 bytes"]
    pub const _0X78: Self = Self(0x78);
}
impl Mxps {
    pub const fn from_bits(val: u8) -> Mxps {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Mxps {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x08 => f.write_str("_0X08"),
            0x10 => f.write_str("_0X10"),
            0x18 => f.write_str("_0X18"),
            0x20 => f.write_str("_0X20"),
            0x28 => f.write_str("_0X28"),
            0x30 => f.write_str("_0X30"),
            0x38 => f.write_str("_0X38"),
            0x40 => f.write_str("_0X40"),
            0x48 => f.write_str("_0X48"),
            0x50 => f.write_str("_0X50"),
            0x58 => f.write_str("_0X58"),
            0x60 => f.write_str("_0X60"),
            0x68 => f.write_str("_0X68"),
            0x70 => f.write_str("_0X70"),
            0x78 => f.write_str("_0X78"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mxps {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x08 => defmt::write!(f, "_0X08"),
            0x10 => defmt::write!(f, "_0X10"),
            0x18 => defmt::write!(f, "_0X18"),
            0x20 => defmt::write!(f, "_0X20"),
            0x28 => defmt::write!(f, "_0X28"),
            0x30 => defmt::write!(f, "_0X30"),
            0x38 => defmt::write!(f, "_0X38"),
            0x40 => defmt::write!(f, "_0X40"),
            0x48 => defmt::write!(f, "_0X48"),
            0x50 => defmt::write!(f, "_0X50"),
            0x58 => defmt::write!(f, "_0X58"),
            0x60 => defmt::write!(f, "_0X60"),
            0x68 => defmt::write!(f, "_0X68"),
            0x70 => defmt::write!(f, "_0X70"),
            0x78 => defmt::write!(f, "_0X78"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Mxps {
    #[inline(always)]
    fn from(val: u8) -> Mxps {
        Mxps::from_bits(val)
    }
}
impl From<Mxps> for u8 {
    #[inline(always)]
    fn from(val: Mxps) -> u8 {
        Mxps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipectr2Pid {
    #[doc = "NAK response"]
    _00 = 0x0,
    #[doc = "BUF response (depending on the buffer state)"]
    _01 = 0x01,
    #[doc = "STALL response"]
    _10 = 0x02,
    #[doc = "STALL response"]
    _11 = 0x03,
}
impl Pipectr2Pid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipectr2Pid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipectr2Pid {
    #[inline(always)]
    fn from(val: u8) -> Pipectr2Pid {
        Pipectr2Pid::from_bits(val)
    }
}
impl From<Pipectr2Pid> for u8 {
    #[inline(always)]
    fn from(val: Pipectr2Pid) -> u8 {
        Pipectr2Pid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PipectrPid {
    #[doc = "NAK response"]
    _00 = 0x0,
    #[doc = "BUF response (depending on the buffer state)"]
    _01 = 0x01,
    #[doc = "STALL response"]
    _10 = 0x02,
    #[doc = "STALL response"]
    _11 = 0x03,
}
impl PipectrPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PipectrPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PipectrPid {
    #[inline(always)]
    fn from(val: u8) -> PipectrPid {
        PipectrPid::from_bits(val)
    }
}
impl From<PipectrPid> for u8 {
    #[inline(always)]
    fn from(val: PipectrPid) -> u8 {
        PipectrPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PipemaxpDevsel {
    #[doc = "Address 0000"]
    _0000 = 0x0,
    #[doc = "Address 0001"]
    _0001 = 0x01,
    #[doc = "Address 0010"]
    _0010 = 0x02,
    #[doc = "Address 0011"]
    _0011 = 0x03,
    #[doc = "Address 0100"]
    _0100 = 0x04,
    #[doc = "Address 0101"]
    _0101 = 0x05,
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
impl PipemaxpDevsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PipemaxpDevsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PipemaxpDevsel {
    #[inline(always)]
    fn from(val: u8) -> PipemaxpDevsel {
        PipemaxpDevsel::from_bits(val)
    }
}
impl From<PipemaxpDevsel> for u8 {
    #[inline(always)]
    fn from(val: PipemaxpDevsel) -> u8 {
        PipemaxpDevsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pipesel {
    #[doc = "No pipe selected"]
    _0000 = 0x0,
    #[doc = "PIPE1"]
    _0001 = 0x01,
    #[doc = "PIPE2"]
    _0010 = 0x02,
    #[doc = "PIPE3"]
    _0011 = 0x03,
    #[doc = "PIPE4"]
    _0100 = 0x04,
    #[doc = "PIPE5"]
    _0101 = 0x05,
    #[doc = "PIPE6"]
    _0110 = 0x06,
    #[doc = "PIPE7"]
    _0111 = 0x07,
    #[doc = "PIPE8"]
    _1000 = 0x08,
    #[doc = "PIPE9"]
    _1001 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pipesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pipesel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pipesel {
    #[inline(always)]
    fn from(val: u8) -> Pipesel {
        Pipesel::from_bits(val)
    }
}
impl From<Pipesel> for u8 {
    #[inline(always)]
    fn from(val: Pipesel) -> u8 {
        Pipesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rhst {
    #[doc = "Communication speed not determined"]
    _000 = 0x0,
    #[doc = "Low-speed connection(When the host controller is selected) /USB bus reset in progress( When the function controller is selected)"]
    _001 = 0x01,
    #[doc = "Full-speed connection(When the host controller is selected) /USB bus reset in progress or full-speed connection(When the function controller is selected)"]
    _010 = 0x02,
    #[doc = "Setting prohibited"]
    _011 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Rhst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rhst {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rhst {
    #[inline(always)]
    fn from(val: u8) -> Rhst {
        Rhst::from_bits(val)
    }
}
impl From<Rhst> for u8 {
    #[inline(always)]
    fn from(val: Rhst) -> u8 {
        Rhst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Type {
    #[doc = "Pipe not used"]
    _00 = 0x0,
    #[doc = "Bulk transfer(PIPE1 and PIPE5) /Setting prohibited(PIPE6 to PIPE9)"]
    _01 = 0x01,
    #[doc = "Setting prohibited(PIPE1 and PIPE5) /Interrupt transfer(PIPE6 to PIPE9)"]
    _10 = 0x02,
    #[doc = "Isochronous transfer(PIPE1 and PIPE2) /Setting prohibited(PIPE3 to PIPE9)"]
    _11 = 0x03,
}
impl Type {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Type {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Type {
    #[inline(always)]
    fn from(val: u8) -> Type {
        Type::from_bits(val)
    }
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(val: Type) -> u8 {
        Type::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbspd {
    #[doc = "DEVADDn is not used"]
    _00 = 0x0,
    #[doc = "Low speed"]
    _01 = 0x01,
    #[doc = "Full speed"]
    _10 = 0x02,
    #[doc = "Setting prohibited"]
    _11 = 0x03,
}
impl Usbspd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbspd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbspd {
    #[inline(always)]
    fn from(val: u8) -> Usbspd {
        Usbspd::from_bits(val)
    }
}
impl From<Usbspd> for u8 {
    #[inline(always)]
    fn from(val: Usbspd) -> u8 {
        Usbspd::to_bits(val)
    }
}
