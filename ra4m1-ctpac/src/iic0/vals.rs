#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aas0 {
    #[doc = "Slave address 0 is not detected."]
    _0 = 0x0,
    #[doc = "Slave address 0 is detected."]
    _1 = 0x01,
}
impl Aas0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aas0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aas0 {
    #[inline(always)]
    fn from(val: u8) -> Aas0 {
        Aas0::from_bits(val)
    }
}
impl From<Aas0> for u8 {
    #[inline(always)]
    fn from(val: Aas0) -> u8 {
        Aas0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aas1 {
    #[doc = "Slave address 1 is not detected."]
    _0 = 0x0,
    #[doc = "Slave address 1 is detected."]
    _1 = 0x01,
}
impl Aas1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aas1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aas1 {
    #[inline(always)]
    fn from(val: u8) -> Aas1 {
        Aas1::from_bits(val)
    }
}
impl From<Aas1> for u8 {
    #[inline(always)]
    fn from(val: Aas1) -> u8 {
        Aas1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aas2 {
    #[doc = "Slave address 2 is not detected."]
    _0 = 0x0,
    #[doc = "Slave address 2 is detected"]
    _1 = 0x01,
}
impl Aas2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aas2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aas2 {
    #[inline(always)]
    fn from(val: u8) -> Aas2 {
        Aas2::from_bits(val)
    }
}
impl From<Aas2> for u8 {
    #[inline(always)]
    fn from(val: Aas2) -> u8 {
        Aas2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ackbr {
    #[doc = "A 0 is received as the acknowledge bit (ACK reception)."]
    _0 = 0x0,
    #[doc = "A 1 is received as the acknowledge bit (NACK reception)."]
    _1 = 0x01,
}
impl Ackbr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ackbr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ackbr {
    #[inline(always)]
    fn from(val: u8) -> Ackbr {
        Ackbr::from_bits(val)
    }
}
impl From<Ackbr> for u8 {
    #[inline(always)]
    fn from(val: Ackbr) -> u8 {
        Ackbr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ackbt {
    #[doc = "A 0 is sent as the acknowledge bit (ACK transmission)."]
    _0 = 0x0,
    #[doc = "A 1 is sent as the acknowledge bit (NACK transmission)."]
    _1 = 0x01,
}
impl Ackbt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ackbt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ackbt {
    #[inline(always)]
    fn from(val: u8) -> Ackbt {
        Ackbt::from_bits(val)
    }
}
impl From<Ackbt> for u8 {
    #[inline(always)]
    fn from(val: Ackbt) -> u8 {
        Ackbt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ackwp {
    #[doc = "Modification of the ACKBT bit is disabled."]
    _0 = 0x0,
    #[doc = "Modification of the ACKBT bit is enabled."]
    _1 = 0x01,
}
impl Ackwp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ackwp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ackwp {
    #[inline(always)]
    fn from(val: u8) -> Ackwp {
        Ackwp::from_bits(val)
    }
}
impl From<Ackwp> for u8 {
    #[inline(always)]
    fn from(val: Ackwp) -> u8 {
        Ackwp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Al {
    #[doc = "Arbitration is not lost."]
    _0 = 0x0,
    #[doc = "Arbitration is lost."]
    _1 = 0x01,
}
impl Al {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Al {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Al {
    #[inline(always)]
    fn from(val: u8) -> Al {
        Al::from_bits(val)
    }
}
impl From<Al> for u8 {
    #[inline(always)]
    fn from(val: Al) -> u8 {
        Al::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Alie {
    #[doc = "Arbitration-lost interrupt request (ALI) is disabled."]
    _0 = 0x0,
    #[doc = "Arbitration-lost interrupt request (ALI) is enabled."]
    _1 = 0x01,
}
impl Alie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Alie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Alie {
    #[inline(always)]
    fn from(val: u8) -> Alie {
        Alie::from_bits(val)
    }
}
impl From<Alie> for u8 {
    #[inline(always)]
    fn from(val: Alie) -> u8 {
        Alie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bbsy {
    #[doc = "The I2C bus is released (bus free state)."]
    _0 = 0x0,
    #[doc = "The I2C bus is occupied (bus busy state)."]
    _1 = 0x01,
}
impl Bbsy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bbsy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bbsy {
    #[inline(always)]
    fn from(val: u8) -> Bbsy {
        Bbsy::from_bits(val)
    }
}
impl From<Bbsy> for u8 {
    #[inline(always)]
    fn from(val: Bbsy) -> u8 {
        Bbsy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bc {
    #[doc = "9 bits"]
    _000 = 0x0,
    #[doc = "2 bits"]
    _001 = 0x01,
    #[doc = "3 bits"]
    _010 = 0x02,
    #[doc = "4 bits"]
    _011 = 0x03,
    #[doc = "5 bits"]
    _100 = 0x04,
    #[doc = "6 bits"]
    _101 = 0x05,
    #[doc = "7 bits"]
    _110 = 0x06,
    #[doc = "8 bits"]
    _111 = 0x07,
}
impl Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bc {
    #[inline(always)]
    fn from(val: u8) -> Bc {
        Bc::from_bits(val)
    }
}
impl From<Bc> for u8 {
    #[inline(always)]
    fn from(val: Bc) -> u8 {
        Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bcwp {
    #[doc = "Enables a value to be written in the BC\\[2:0\\] bits."]
    _0 = 0x0,
    #[doc = "Disables a value to be written in the BC\\[2:0\\] bits."]
    _1 = 0x01,
}
impl Bcwp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bcwp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bcwp {
    #[inline(always)]
    fn from(val: u8) -> Bcwp {
        Bcwp::from_bits(val)
    }
}
impl From<Bcwp> for u8 {
    #[inline(always)]
    fn from(val: Bcwp) -> u8 {
        Bcwp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cks {
    #[doc = "PCLKB/1 clock"]
    _000 = 0x0,
    #[doc = "PCLKB/2 clock"]
    _001 = 0x01,
    #[doc = "PCLKB/4 clock"]
    _010 = 0x02,
    #[doc = "PCLKB/8 clock"]
    _011 = 0x03,
    #[doc = "PCLKB/16 clock"]
    _100 = 0x04,
    #[doc = "PCLKB/32 clock"]
    _101 = 0x05,
    #[doc = "PCLKB/64 clock"]
    _110 = 0x06,
    #[doc = "PCLKB/128 clock"]
    _111 = 0x07,
}
impl Cks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cks {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cks {
    #[inline(always)]
    fn from(val: u8) -> Cks {
        Cks::from_bits(val)
    }
}
impl From<Cks> for u8 {
    #[inline(always)]
    fn from(val: Cks) -> u8 {
        Cks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clo {
    #[doc = "Does not output an extra SCL clock cycle."]
    _0 = 0x0,
    #[doc = "Outputs an extra SCL clock cycle."]
    _1 = 0x01,
}
impl Clo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clo {
    #[inline(always)]
    fn from(val: u8) -> Clo {
        Clo::from_bits(val)
    }
}
impl From<Clo> for u8 {
    #[inline(always)]
    fn from(val: Clo) -> u8 {
        Clo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Did {
    #[doc = "Device-ID command is not detected."]
    _0 = 0x0,
    #[doc = "Device-ID command is detected."]
    _1 = 0x01,
}
impl Did {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Did {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Did {
    #[inline(always)]
    fn from(val: u8) -> Did {
        Did::from_bits(val)
    }
}
impl From<Did> for u8 {
    #[inline(always)]
    fn from(val: Did) -> u8 {
        Did::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dide {
    #[doc = "Device-ID address detection is disabled."]
    _0 = 0x0,
    #[doc = "Device-ID address detection is enabled."]
    _1 = 0x01,
}
impl Dide {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dide {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dide {
    #[inline(always)]
    fn from(val: u8) -> Dide {
        Dide::from_bits(val)
    }
}
impl From<Dide> for u8 {
    #[inline(always)]
    fn from(val: Dide) -> u8 {
        Dide::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dlcs {
    #[doc = "The internal reference clock (fIIC) is selected as the clock source of the SDA output delay counter."]
    _0 = 0x0,
    #[doc = "The internal reference clock divided by 2 (fIIC/2) is selected as the clock source of the SDA output delay counter."]
    _1 = 0x01,
}
impl Dlcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dlcs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dlcs {
    #[inline(always)]
    fn from(val: u8) -> Dlcs {
        Dlcs::from_bits(val)
    }
}
impl From<Dlcs> for u8 {
    #[inline(always)]
    fn from(val: Dlcs) -> u8 {
        Dlcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fs {
    #[doc = "The 7-bit address format is selected."]
    _0 = 0x0,
    #[doc = "The 10-bit address format is selected."]
    _1 = 0x01,
}
impl Fs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fs {
    #[inline(always)]
    fn from(val: u8) -> Fs {
        Fs::from_bits(val)
    }
}
impl From<Fs> for u8 {
    #[inline(always)]
    fn from(val: Fs) -> u8 {
        Fs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gca {
    #[doc = "General call address is not detected."]
    _0 = 0x0,
    #[doc = "General call address is detected."]
    _1 = 0x01,
}
impl Gca {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gca {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gca {
    #[inline(always)]
    fn from(val: u8) -> Gca {
        Gca::from_bits(val)
    }
}
impl From<Gca> for u8 {
    #[inline(always)]
    fn from(val: Gca) -> u8 {
        Gca::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gcae {
    #[doc = "General call address detection is disabled."]
    _0 = 0x0,
    #[doc = "General call address detection is enabled."]
    _1 = 0x01,
}
impl Gcae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gcae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gcae {
    #[inline(always)]
    fn from(val: u8) -> Gcae {
        Gcae::from_bits(val)
    }
}
impl From<Gcae> for u8 {
    #[inline(always)]
    fn from(val: Gcae) -> u8 {
        Gcae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hoa {
    #[doc = "Host address is not detected."]
    _0 = 0x0,
    #[doc = "Host address is detected."]
    _1 = 0x01,
}
impl Hoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hoa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hoa {
    #[inline(always)]
    fn from(val: u8) -> Hoa {
        Hoa::from_bits(val)
    }
}
impl From<Hoa> for u8 {
    #[inline(always)]
    fn from(val: Hoa) -> u8 {
        Hoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hoae {
    #[doc = "Host address detection is disabled."]
    _0 = 0x0,
    #[doc = "Host address detection is enabled."]
    _1 = 0x01,
}
impl Hoae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hoae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hoae {
    #[inline(always)]
    fn from(val: u8) -> Hoae {
        Hoae::from_bits(val)
    }
}
impl From<Hoae> for u8 {
    #[inline(always)]
    fn from(val: Hoae) -> u8 {
        Hoae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ice {
    #[doc = "Disable (SCLn and SDAn pins in inactive state)"]
    _0 = 0x0,
    #[doc = "Enable (SCLn and SDAn pins in active state)"]
    _1 = 0x01,
}
impl Ice {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ice {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ice {
    #[inline(always)]
    fn from(val: u8) -> Ice {
        Ice::from_bits(val)
    }
}
impl From<Ice> for u8 {
    #[inline(always)]
    fn from(val: Ice) -> u8 {
        Ice::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iicrst {
    #[doc = "Releases the RIIC reset or internal reset."]
    _0 = 0x0,
    #[doc = "Initiates the RIIC reset or internal reset."]
    _1 = 0x01,
}
impl Iicrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iicrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iicrst {
    #[inline(always)]
    fn from(val: u8) -> Iicrst {
        Iicrst::from_bits(val)
    }
}
impl From<Iicrst> for u8 {
    #[inline(always)]
    fn from(val: Iicrst) -> u8 {
        Iicrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Male {
    #[doc = "Master arbitration-lost detection is disabled."]
    _0 = 0x0,
    #[doc = "Master arbitration-lost detection is enabled."]
    _1 = 0x01,
}
impl Male {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Male {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Male {
    #[inline(always)]
    fn from(val: u8) -> Male {
        Male::from_bits(val)
    }
}
impl From<Male> for u8 {
    #[inline(always)]
    fn from(val: Male) -> u8 {
        Male::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mst {
    #[doc = "Slave mode"]
    _0 = 0x0,
    #[doc = "Master mode"]
    _1 = 0x01,
}
impl Mst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mst {
    #[inline(always)]
    fn from(val: u8) -> Mst {
        Mst::from_bits(val)
    }
}
impl From<Mst> for u8 {
    #[inline(always)]
    fn from(val: Mst) -> u8 {
        Mst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mtwp {
    #[doc = "Disables writing to the MST and TRS bits in ICCR2."]
    _0 = 0x0,
    #[doc = "Enables writing to the MST and TRS bits in ICCR2."]
    _1 = 0x01,
}
impl Mtwp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mtwp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mtwp {
    #[inline(always)]
    fn from(val: u8) -> Mtwp {
        Mtwp::from_bits(val)
    }
}
impl From<Mtwp> for u8 {
    #[inline(always)]
    fn from(val: Mtwp) -> u8 {
        Mtwp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nacke {
    #[doc = "Transfer operation is not suspended during NACK reception (transfer suspension disabled)."]
    _0 = 0x0,
    #[doc = "Transfer operation is suspended during NACK reception (transfer suspension enabled)."]
    _1 = 0x01,
}
impl Nacke {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nacke {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nacke {
    #[inline(always)]
    fn from(val: u8) -> Nacke {
        Nacke::from_bits(val)
    }
}
impl From<Nacke> for u8 {
    #[inline(always)]
    fn from(val: Nacke) -> u8 {
        Nacke::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nackf {
    #[doc = "NACK is not detected."]
    _0 = 0x0,
    #[doc = "NACK is detected."]
    _1 = 0x01,
}
impl Nackf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nackf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nackf {
    #[inline(always)]
    fn from(val: u8) -> Nackf {
        Nackf::from_bits(val)
    }
}
impl From<Nackf> for u8 {
    #[inline(always)]
    fn from(val: Nackf) -> u8 {
        Nackf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nakie {
    #[doc = "NACK reception interrupt request (NAKI) is disabled."]
    _0 = 0x0,
    #[doc = "NACK reception interrupt request (NAKI) is enabled."]
    _1 = 0x01,
}
impl Nakie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nakie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nakie {
    #[inline(always)]
    fn from(val: u8) -> Nakie {
        Nakie::from_bits(val)
    }
}
impl From<Nakie> for u8 {
    #[inline(always)]
    fn from(val: Nakie) -> u8 {
        Nakie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nale {
    #[doc = "NACK transmission arbitration-lost detection is disabled."]
    _0 = 0x0,
    #[doc = "NACK transmission arbitration-lost detection is enabled."]
    _1 = 0x01,
}
impl Nale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nale {
    #[inline(always)]
    fn from(val: u8) -> Nale {
        Nale::from_bits(val)
    }
}
impl From<Nale> for u8 {
    #[inline(always)]
    fn from(val: Nale) -> u8 {
        Nale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nf {
    #[doc = "Noise of up to one fIIC cycle is filtered out (single-stage filter)."]
    _00 = 0x0,
    #[doc = "Noise of up to two fIIC cycles is filtered out (2-stage filter)."]
    _01 = 0x01,
    #[doc = "Noise of up to three fIIC cycles is filtered out (3-stage filter)."]
    _10 = 0x02,
    #[doc = "Noise of up to four fIIC cycles is filtered out (4-stage filter)"]
    _11 = 0x03,
}
impl Nf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nf {
    #[inline(always)]
    fn from(val: u8) -> Nf {
        Nf::from_bits(val)
    }
}
impl From<Nf> for u8 {
    #[inline(always)]
    fn from(val: Nf) -> u8 {
        Nf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nfe {
    #[doc = "No digital noise filter circuit is used."]
    _0 = 0x0,
    #[doc = "A digital noise filter circuit is used."]
    _1 = 0x01,
}
impl Nfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfe {
    #[inline(always)]
    fn from(val: u8) -> Nfe {
        Nfe::from_bits(val)
    }
}
impl From<Nfe> for u8 {
    #[inline(always)]
    fn from(val: Nfe) -> u8 {
        Nfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdrf {
    #[doc = "ICDRR contains no receive data."]
    _0 = 0x0,
    #[doc = "ICDRR contains receive data."]
    _1 = 0x01,
}
impl Rdrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdrf {
    #[inline(always)]
    fn from(val: u8) -> Rdrf {
        Rdrf::from_bits(val)
    }
}
impl From<Rdrf> for u8 {
    #[inline(always)]
    fn from(val: Rdrf) -> u8 {
        Rdrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdrfs {
    #[doc = "The RDRF flag is set at the rising edge of the ninth SCL clock cycle. (The SCLn line is not held low at the falling edge of the eighth clock cycle.)"]
    _0 = 0x0,
    #[doc = "The RDRF flag is set at the rising edge of the eighth SCL clock cycle. (The SCLn line is held low at the falling edge of the eighth clock cycle.)"]
    _1 = 0x01,
}
impl Rdrfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdrfs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdrfs {
    #[inline(always)]
    fn from(val: u8) -> Rdrfs {
        Rdrfs::from_bits(val)
    }
}
impl From<Rdrfs> for u8 {
    #[inline(always)]
    fn from(val: Rdrfs) -> u8 {
        Rdrfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rie {
    #[doc = "Receive data full interrupt request (IIC_RXI) is disabled."]
    _0 = 0x0,
    #[doc = "Receive data full interrupt request (IIC_RXI) is enabled."]
    _1 = 0x01,
}
impl Rie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rie {
    #[inline(always)]
    fn from(val: u8) -> Rie {
        Rie::from_bits(val)
    }
}
impl From<Rie> for u8 {
    #[inline(always)]
    fn from(val: Rie) -> u8 {
        Rie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rs {
    #[doc = "Does not request to issue a restart condition."]
    _0 = 0x0,
    #[doc = "Requests to issue a restart condition."]
    _1 = 0x01,
}
impl Rs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rs {
    #[inline(always)]
    fn from(val: u8) -> Rs {
        Rs::from_bits(val)
    }
}
impl From<Rs> for u8 {
    #[inline(always)]
    fn from(val: Rs) -> u8 {
        Rs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sale {
    #[doc = "Slave arbitration-lost detection is disabled."]
    _0 = 0x0,
    #[doc = "Slave arbitration-lost detection is enabled."]
    _1 = 0x01,
}
impl Sale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sale {
    #[inline(always)]
    fn from(val: u8) -> Sale {
        Sale::from_bits(val)
    }
}
impl From<Sale> for u8 {
    #[inline(always)]
    fn from(val: Sale) -> u8 {
        Sale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sar0e {
    #[doc = "Slave address in SARL0 and SARU0 is disabled."]
    _0 = 0x0,
    #[doc = "Slave address in SARL0 and SARU0 is enabled."]
    _1 = 0x01,
}
impl Sar0e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sar0e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sar0e {
    #[inline(always)]
    fn from(val: u8) -> Sar0e {
        Sar0e::from_bits(val)
    }
}
impl From<Sar0e> for u8 {
    #[inline(always)]
    fn from(val: Sar0e) -> u8 {
        Sar0e::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sar1e {
    #[doc = "Slave address in SARL1 and SARU1 is disabled."]
    _0 = 0x0,
    #[doc = "Slave address in SARL1 and SARU1 is enabled."]
    _1 = 0x01,
}
impl Sar1e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sar1e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sar1e {
    #[inline(always)]
    fn from(val: u8) -> Sar1e {
        Sar1e::from_bits(val)
    }
}
impl From<Sar1e> for u8 {
    #[inline(always)]
    fn from(val: Sar1e) -> u8 {
        Sar1e::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sar2e {
    #[doc = "Slave address in SARL2 and SARU2 is disabled."]
    _0 = 0x0,
    #[doc = "Slave address in SARL2 and SARU2 is enabled"]
    _1 = 0x01,
}
impl Sar2e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sar2e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sar2e {
    #[inline(always)]
    fn from(val: u8) -> Sar2e {
        Sar2e::from_bits(val)
    }
}
impl From<Sar2e> for u8 {
    #[inline(always)]
    fn from(val: Sar2e) -> u8 {
        Sar2e::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scle {
    #[doc = "No SCL synchronous circuit is used."]
    _0 = 0x0,
    #[doc = "An SCL synchronous circuit is used."]
    _1 = 0x01,
}
impl Scle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scle {
    #[inline(always)]
    fn from(val: u8) -> Scle {
        Scle::from_bits(val)
    }
}
impl From<Scle> for u8 {
    #[inline(always)]
    fn from(val: Scle) -> u8 {
        Scle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scli {
    #[doc = "SCLn line is low."]
    _0 = 0x0,
    #[doc = "SCLn line is high."]
    _1 = 0x01,
}
impl Scli {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scli {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scli {
    #[inline(always)]
    fn from(val: u8) -> Scli {
        Scli::from_bits(val)
    }
}
impl From<Scli> for u8 {
    #[inline(always)]
    fn from(val: Scli) -> u8 {
        Scli::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sclo {
    #[doc = "(Read)The RIIC has driven the SCLn pin low. / (Write)The RIIC drives the SCLn pin low."]
    _0 = 0x0,
    #[doc = "(Read)The RIIC has released the SCLn pin. / (Write)The RIIC releases the SCLn pin."]
    _1 = 0x01,
}
impl Sclo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sclo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sclo {
    #[inline(always)]
    fn from(val: u8) -> Sclo {
        Sclo::from_bits(val)
    }
}
impl From<Sclo> for u8 {
    #[inline(always)]
    fn from(val: Sclo) -> u8 {
        Sclo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdai {
    #[doc = "SDAn line is low."]
    _0 = 0x0,
    #[doc = "SDAn line is high."]
    _1 = 0x01,
}
impl Sdai {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdai {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdai {
    #[inline(always)]
    fn from(val: u8) -> Sdai {
        Sdai::from_bits(val)
    }
}
impl From<Sdai> for u8 {
    #[inline(always)]
    fn from(val: Sdai) -> u8 {
        Sdai::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdao {
    #[doc = "(Read)The RIIC has driven the SDAn pin low. / (Write)The RIIC drives the SDAn pin low."]
    _0 = 0x0,
    #[doc = "(Read)The RIIC has released the SDAn pin./ (Write)The RIIC releases the SDAn pin."]
    _1 = 0x01,
}
impl Sdao {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdao {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdao {
    #[inline(always)]
    fn from(val: u8) -> Sdao {
        Sdao::from_bits(val)
    }
}
impl From<Sdao> for u8 {
    #[inline(always)]
    fn from(val: Sdao) -> u8 {
        Sdao::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sddl {
    #[doc = "No output delay"]
    _000 = 0x0,
    #[doc = "1 fIIC cycle (ICMR2.DLCS=0) / 1 or 2 fIIC cycles (ICMR2.DLCS=1)"]
    _001 = 0x01,
    #[doc = "2 fIIC cycles (ICMR2.DLCS=0) / 3 or 4 fIIC cycles (ICMR2.DLCS=1)"]
    _010 = 0x02,
    #[doc = "3 fIIC cycles (ICMR2.DLCS=0) / 5 or 6 fIIC cycles (ICMR2.DLCS=1)"]
    _011 = 0x03,
    #[doc = "4 fIIC cycles (ICMR2.DLCS=0) / 7 or 8 fIIC cycles (ICMR2.DLCS=1)"]
    _100 = 0x04,
    #[doc = "5 fIIC cycles (ICMR2.DLCS=0) / 9 or 10 fIIC cycles (ICMR2.DLCS=1)"]
    _101 = 0x05,
    #[doc = "6 fIIC cycles (ICMR2.DLCS=0) / 11 or 12 fIIC cycles (ICMR2.DLCS=1)"]
    _110 = 0x06,
    #[doc = "7 fIIC cycles (ICMR2.DLCS=0) / 13 or 14 fIIC cycles (ICMR2.DLCS=1)"]
    _111 = 0x07,
}
impl Sddl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sddl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sddl {
    #[inline(always)]
    fn from(val: u8) -> Sddl {
        Sddl::from_bits(val)
    }
}
impl From<Sddl> for u8 {
    #[inline(always)]
    fn from(val: Sddl) -> u8 {
        Sddl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smbs {
    #[doc = "The I2C bus is selected."]
    _0 = 0x0,
    #[doc = "The SMBus is selected."]
    _1 = 0x01,
}
impl Smbs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smbs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smbs {
    #[inline(always)]
    fn from(val: u8) -> Smbs {
        Smbs::from_bits(val)
    }
}
impl From<Smbs> for u8 {
    #[inline(always)]
    fn from(val: Smbs) -> u8 {
        Smbs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sowp {
    #[doc = "Bits SCLO and SDAO can be written"]
    _0 = 0x0,
    #[doc = "Bits SCLO and SDAO are protected."]
    _1 = 0x01,
}
impl Sowp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sowp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sowp {
    #[inline(always)]
    fn from(val: u8) -> Sowp {
        Sowp::from_bits(val)
    }
}
impl From<Sowp> for u8 {
    #[inline(always)]
    fn from(val: Sowp) -> u8 {
        Sowp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sp {
    #[doc = "Does not request to issue a stop condition."]
    _0 = 0x0,
    #[doc = "Requests to issue a stop condition."]
    _1 = 0x01,
}
impl Sp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sp {
    #[inline(always)]
    fn from(val: u8) -> Sp {
        Sp::from_bits(val)
    }
}
impl From<Sp> for u8 {
    #[inline(always)]
    fn from(val: Sp) -> u8 {
        Sp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spie {
    #[doc = "Stop condition detection interrupt request (SPI) is disabled."]
    _0 = 0x0,
    #[doc = "Stop condition detection interrupt request (SPI) is enabled."]
    _1 = 0x01,
}
impl Spie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spie {
    #[inline(always)]
    fn from(val: u8) -> Spie {
        Spie::from_bits(val)
    }
}
impl From<Spie> for u8 {
    #[inline(always)]
    fn from(val: Spie) -> u8 {
        Spie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum St {
    #[doc = "Does not request to issue a start condition."]
    _0 = 0x0,
    #[doc = "Requests to issue a start condition."]
    _1 = 0x01,
}
impl St {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> St {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for St {
    #[inline(always)]
    fn from(val: u8) -> St {
        St::from_bits(val)
    }
}
impl From<St> for u8 {
    #[inline(always)]
    fn from(val: St) -> u8 {
        St::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Start {
    #[doc = "Start condition is not detected."]
    _0 = 0x0,
    #[doc = "Start condition is detected."]
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
pub enum Stie {
    #[doc = "Start condition detection interrupt request (STI) is disabled."]
    _0 = 0x0,
    #[doc = "Start condition detection interrupt request (STI) is enabled."]
    _1 = 0x01,
}
impl Stie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stie {
    #[inline(always)]
    fn from(val: u8) -> Stie {
        Stie::from_bits(val)
    }
}
impl From<Stie> for u8 {
    #[inline(always)]
    fn from(val: Stie) -> u8 {
        Stie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stop {
    #[doc = "Stop condition is not detected."]
    _0 = 0x0,
    #[doc = "Stop condition is detected."]
    _1 = 0x01,
}
impl Stop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stop {
    #[inline(always)]
    fn from(val: u8) -> Stop {
        Stop::from_bits(val)
    }
}
impl From<Stop> for u8 {
    #[inline(always)]
    fn from(val: Stop) -> u8 {
        Stop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdre {
    #[doc = "ICDRT contains transmit data."]
    _0 = 0x0,
    #[doc = "ICDRT contains no transmit data."]
    _1 = 0x01,
}
impl Tdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdre {
    #[inline(always)]
    fn from(val: u8) -> Tdre {
        Tdre::from_bits(val)
    }
}
impl From<Tdre> for u8 {
    #[inline(always)]
    fn from(val: Tdre) -> u8 {
        Tdre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Teie {
    #[doc = "Transmit end interrupt request (IIC_TEI) is disabled."]
    _0 = 0x0,
    #[doc = "Transmit end interrupt request (IIC_TEI) is enabled."]
    _1 = 0x01,
}
impl Teie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Teie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Teie {
    #[inline(always)]
    fn from(val: u8) -> Teie {
        Teie::from_bits(val)
    }
}
impl From<Teie> for u8 {
    #[inline(always)]
    fn from(val: Teie) -> u8 {
        Teie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tend {
    #[doc = "Data is being transmitted."]
    _0 = 0x0,
    #[doc = "Data has been transmitted."]
    _1 = 0x01,
}
impl Tend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tend {
    #[inline(always)]
    fn from(val: u8) -> Tend {
        Tend::from_bits(val)
    }
}
impl From<Tend> for u8 {
    #[inline(always)]
    fn from(val: Tend) -> u8 {
        Tend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie {
    #[doc = "Transmit data empty interrupt request (IIC_TXI) is disabled."]
    _0 = 0x0,
    #[doc = "Transmit data empty interrupt request (IIC_TXI) is enabled."]
    _1 = 0x01,
}
impl Tie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie {
    #[inline(always)]
    fn from(val: u8) -> Tie {
        Tie::from_bits(val)
    }
}
impl From<Tie> for u8 {
    #[inline(always)]
    fn from(val: Tie) -> u8 {
        Tie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmoe {
    #[doc = "The timeout function is disabled."]
    _0 = 0x0,
    #[doc = "The timeout function is enabled."]
    _1 = 0x01,
}
impl Tmoe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmoe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmoe {
    #[inline(always)]
    fn from(val: u8) -> Tmoe {
        Tmoe::from_bits(val)
    }
}
impl From<Tmoe> for u8 {
    #[inline(always)]
    fn from(val: Tmoe) -> u8 {
        Tmoe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmof {
    #[doc = "Timeout is not detected."]
    _0 = 0x0,
    #[doc = "Timeout is detected."]
    _1 = 0x01,
}
impl Tmof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmof {
    #[inline(always)]
    fn from(val: u8) -> Tmof {
        Tmof::from_bits(val)
    }
}
impl From<Tmof> for u8 {
    #[inline(always)]
    fn from(val: Tmof) -> u8 {
        Tmof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmoh {
    #[doc = "Count is disabled while the SCLn line is at a high level."]
    _0 = 0x0,
    #[doc = "Count is enabled while the SCLn line is at a high level."]
    _1 = 0x01,
}
impl Tmoh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmoh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmoh {
    #[inline(always)]
    fn from(val: u8) -> Tmoh {
        Tmoh::from_bits(val)
    }
}
impl From<Tmoh> for u8 {
    #[inline(always)]
    fn from(val: Tmoh) -> u8 {
        Tmoh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmoie {
    #[doc = "Timeout interrupt request (TMOI) is disabled."]
    _0 = 0x0,
    #[doc = "Timeout interrupt request (TMOI) is enabled."]
    _1 = 0x01,
}
impl Tmoie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmoie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmoie {
    #[inline(always)]
    fn from(val: u8) -> Tmoie {
        Tmoie::from_bits(val)
    }
}
impl From<Tmoie> for u8 {
    #[inline(always)]
    fn from(val: Tmoie) -> u8 {
        Tmoie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmol {
    #[doc = "Count is disabled while the SCLn line is at a low level."]
    _0 = 0x0,
    #[doc = "Count is enabled while the SCLn line is at a low level."]
    _1 = 0x01,
}
impl Tmol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmol {
    #[inline(always)]
    fn from(val: u8) -> Tmol {
        Tmol::from_bits(val)
    }
}
impl From<Tmol> for u8 {
    #[inline(always)]
    fn from(val: Tmol) -> u8 {
        Tmol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmos {
    #[doc = "Long mode is selected."]
    _0 = 0x0,
    #[doc = "Short mode is selected."]
    _1 = 0x01,
}
impl Tmos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmos {
    #[inline(always)]
    fn from(val: u8) -> Tmos {
        Tmos::from_bits(val)
    }
}
impl From<Tmos> for u8 {
    #[inline(always)]
    fn from(val: Tmos) -> u8 {
        Tmos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trs {
    #[doc = "Receive mode"]
    _0 = 0x0,
    #[doc = "Transmit mode"]
    _1 = 0x01,
}
impl Trs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trs {
    #[inline(always)]
    fn from(val: u8) -> Trs {
        Trs::from_bits(val)
    }
}
impl From<Trs> for u8 {
    #[inline(always)]
    fn from(val: Trs) -> u8 {
        Trs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wait {
    #[doc = "No WAIT (The period between ninth clock cycle and first clock cycle is not held low.)"]
    _0 = 0x0,
    #[doc = "WAIT (The period between ninth clock cycle and first clock cycle is held low.)"]
    _1 = 0x01,
}
impl Wait {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wait {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wait {
    #[inline(always)]
    fn from(val: u8) -> Wait {
        Wait::from_bits(val)
    }
}
impl From<Wait> for u8 {
    #[inline(always)]
    fn from(val: Wait) -> u8 {
        Wait::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuack {
    #[doc = "State of synchronous operation"]
    _0 = 0x0,
    #[doc = "State of asynchronous operation"]
    _1 = 0x01,
}
impl Wuack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuack {
    #[inline(always)]
    fn from(val: u8) -> Wuack {
        Wuack::from_bits(val)
    }
}
impl From<Wuack> for u8 {
    #[inline(always)]
    fn from(val: Wuack) -> u8 {
        Wuack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuafa {
    #[doc = "Do not add the wakeup analog filter"]
    _0 = 0x0,
    #[doc = "Add the wakeup analog filter."]
    _1 = 0x01,
}
impl Wuafa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuafa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuafa {
    #[inline(always)]
    fn from(val: u8) -> Wuafa {
        Wuafa::from_bits(val)
    }
}
impl From<Wuafa> for u8 {
    #[inline(always)]
    fn from(val: Wuafa) -> u8 {
        Wuafa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuasyf {
    #[doc = "IIC synchronous circuit enable condition"]
    _0 = 0x0,
    #[doc = "IIC asynchronous circuit enable condition."]
    _1 = 0x01,
}
impl Wuasyf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuasyf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuasyf {
    #[inline(always)]
    fn from(val: u8) -> Wuasyf {
        Wuasyf::from_bits(val)
    }
}
impl From<Wuasyf> for u8 {
    #[inline(always)]
    fn from(val: Wuasyf) -> u8 {
        Wuasyf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wue {
    #[doc = "Wakeup function disabled"]
    _0 = 0x0,
    #[doc = "Wakeup function enabled."]
    _1 = 0x01,
}
impl Wue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wue {
    #[inline(always)]
    fn from(val: u8) -> Wue {
        Wue::from_bits(val)
    }
}
impl From<Wue> for u8 {
    #[inline(always)]
    fn from(val: Wue) -> u8 {
        Wue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf {
    #[doc = "Slave address does not match during wakeup function"]
    _0 = 0x0,
    #[doc = "Slave address matches during wakeup function."]
    _1 = 0x01,
}
impl Wuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf {
    #[inline(always)]
    fn from(val: u8) -> Wuf {
        Wuf::from_bits(val)
    }
}
impl From<Wuf> for u8 {
    #[inline(always)]
    fn from(val: Wuf) -> u8 {
        Wuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuie {
    #[doc = "Wakeup Interrupt Request (IIC0_WUI) disabled"]
    _0 = 0x0,
    #[doc = "Wakeup Interrupt Request (IIC0_WUI) enabled."]
    _1 = 0x01,
}
impl Wuie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuie {
    #[inline(always)]
    fn from(val: u8) -> Wuie {
        Wuie::from_bits(val)
    }
}
impl From<Wuie> for u8 {
    #[inline(always)]
    fn from(val: Wuie) -> u8 {
        Wuie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wusen {
    #[doc = "IIC asynchronous circuit enable"]
    _0 = 0x0,
    #[doc = "IIC synchronous circuit enable"]
    _1 = 0x01,
}
impl Wusen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wusen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wusen {
    #[inline(always)]
    fn from(val: u8) -> Wusen {
        Wusen::from_bits(val)
    }
}
impl From<Wusen> for u8 {
    #[inline(always)]
    fn from(val: Wusen) -> u8 {
        Wusen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wusyf {
    #[doc = "IIC asynchronous circuit enable condition"]
    _0 = 0x0,
    #[doc = "IIC synchronous circuit enable condition."]
    _1 = 0x01,
}
impl Wusyf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wusyf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wusyf {
    #[inline(always)]
    fn from(val: u8) -> Wusyf {
        Wusyf::from_bits(val)
    }
}
impl From<Wusyf> for u8 {
    #[inline(always)]
    fn from(val: Wusyf) -> u8 {
        Wusyf::to_bits(val)
    }
}
