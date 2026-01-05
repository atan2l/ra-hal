#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key(u8);
impl Key {
    #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
    pub const _0X_A5: Self = Self(0xa5);
}
impl Key {
    pub const fn from_bits(val: u8) -> Key {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Key {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xa5 => f.write_str("_0X_A5"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Key {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xa5 => defmt::write!(f, "_0X_A5"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Key {
    #[inline(always)]
    fn from(val: u8) -> Key {
        Key::from_bits(val)
    }
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(val: Key) -> u8 {
        Key::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MmpuacaEnable {
    #[doc = "Group m Region n unit is disabled"]
    _0 = 0x0,
    #[doc = "Group m Region n unit is enabled"]
    _1 = 0x01,
}
impl MmpuacaEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MmpuacaEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MmpuacaEnable {
    #[inline(always)]
    fn from(val: u8) -> MmpuacaEnable {
        MmpuacaEnable::from_bits(val)
    }
}
impl From<MmpuacaEnable> for u8 {
    #[inline(always)]
    fn from(val: MmpuacaEnable) -> u8 {
        MmpuacaEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MmpuctlaEnable {
    #[doc = "Master Group A disabled"]
    _0 = 0x0,
    #[doc = "Master Group A enabled."]
    _1 = 0x01,
}
impl MmpuctlaEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MmpuctlaEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MmpuctlaEnable {
    #[inline(always)]
    fn from(val: u8) -> MmpuctlaEnable {
        MmpuctlaEnable::from_bits(val)
    }
}
impl From<MmpuctlaEnable> for u8 {
    #[inline(always)]
    fn from(val: MmpuctlaEnable) -> u8 {
        MmpuctlaEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oad {
    #[doc = "Non-maskable interrupt."]
    _0 = 0x0,
    #[doc = "Internal reset."]
    _1 = 0x01,
}
impl Oad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oad {
    #[inline(always)]
    fn from(val: u8) -> Oad {
        Oad::from_bits(val)
    }
}
impl From<Oad> for u8 {
    #[inline(always)]
    fn from(val: Oad) -> u8 {
        Oad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Protect {
    #[doc = "All Bus Master MPU Group A register writing is possible."]
    _0 = 0x0,
    #[doc = "All Bus Master MPU Group A register writing is protected. Read is possible."]
    _1 = 0x01,
}
impl Protect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Protect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Protect {
    #[inline(always)]
    fn from(val: u8) -> Protect {
        Protect::from_bits(val)
    }
}
impl From<Protect> for u8 {
    #[inline(always)]
    fn from(val: Protect) -> u8 {
        Protect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rp {
    #[doc = "Read permission"]
    _0 = 0x0,
    #[doc = "Read protection"]
    _1 = 0x01,
}
impl Rp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rp {
    #[inline(always)]
    fn from(val: u8) -> Rp {
        Rp::from_bits(val)
    }
}
impl From<Rp> for u8 {
    #[inline(always)]
    fn from(val: Rp) -> u8 {
        Rp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wp {
    #[doc = "Write permission"]
    _0 = 0x0,
    #[doc = "Write protection"]
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
