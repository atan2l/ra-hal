#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ad0mated {
    #[doc = "Disable the Snooze End request"]
    _0 = 0x0,
    #[doc = "Enable the Snooze End request"]
    _1 = 0x01,
}
impl Ad0mated {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ad0mated {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ad0mated {
    #[inline(always)]
    fn from(val: u8) -> Ad0mated {
        Ad0mated::from_bits(val)
    }
}
impl From<Ad0mated> for u8 {
    #[inline(always)]
    fn from(val: Ad0mated) -> u8 {
        Ad0mated::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ad0umted {
    #[doc = "Disable the Snooze End request"]
    _0 = 0x0,
    #[doc = "Enable the Snooze End request"]
    _1 = 0x01,
}
impl Ad0umted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ad0umted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ad0umted {
    #[inline(always)]
    fn from(val: u8) -> Ad0umted {
        Ad0umted::from_bits(val)
    }
}
impl From<Ad0umted> for u8 {
    #[inline(always)]
    fn from(val: Ad0umted) -> u8 {
        Ad0umted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Agtunfed {
    #[doc = "Disable the Snooze End request"]
    _0 = 0x0,
    #[doc = "Enable the Snooze End request"]
    _1 = 0x01,
}
impl Agtunfed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Agtunfed {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Agtunfed {
    #[inline(always)]
    fn from(val: u8) -> Agtunfed {
        Agtunfed::from_bits(val)
    }
}
impl From<Agtunfed> for u8 {
    #[inline(always)]
    fn from(val: Agtunfed) -> u8 {
        Agtunfed::to_bits(val)
    }
}
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
pub enum Bpwswstp {
    #[doc = "Battery Power supply Switch Enable"]
    _0 = 0x0,
    #[doc = "Battery Power supply Switch stop"]
    _1 = 0x01,
}
impl Bpwswstp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bpwswstp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bpwswstp {
    #[inline(always)]
    fn from(val: u8) -> Bpwswstp {
        Bpwswstp::from_bits(val)
    }
}
impl From<Bpwswstp> for u8 {
    #[inline(always)]
    fn from(val: Bpwswstp) -> u8 {
        Bpwswstp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busmrf {
    #[doc = "Bus Master MPU reset not detected."]
    _0 = 0x0,
    #[doc = "Bus Master MPU reset detected."]
    _1 = 0x01,
}
impl Busmrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busmrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busmrf {
    #[inline(always)]
    fn from(val: u8) -> Busmrf {
        Busmrf::from_bits(val)
    }
}
impl From<Busmrf> for u8 {
    #[inline(always)]
    fn from(val: Busmrf) -> u8 {
        Busmrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bussrf {
    #[doc = "Bus Slave MPU reset not detected."]
    _0 = 0x0,
    #[doc = "Bus Slave MPU reset detected."]
    _1 = 0x01,
}
impl Bussrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bussrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bussrf {
    #[inline(always)]
    fn from(val: u8) -> Bussrf {
        Bussrf::from_bits(val)
    }
}
impl From<Bussrf> for u8 {
    #[inline(always)]
    fn from(val: Bussrf) -> u8 {
        Bussrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0vch1te {
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is enabled."]
    _1 = 0x01,
}
impl Ch0vch1te {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0vch1te {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0vch1te {
    #[inline(always)]
    fn from(val: u8) -> Ch0vch1te {
        Ch0vch1te::from_bits(val)
    }
}
impl From<Ch0vch1te> for u8 {
    #[inline(always)]
    fn from(val: Ch0vch1te) -> u8 {
        Ch0vch1te::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0vch2te {
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is enabled."]
    _1 = 0x01,
}
impl Ch0vch2te {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0vch2te {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0vch2te {
    #[inline(always)]
    fn from(val: u8) -> Ch0vch2te {
        Ch0vch2te::from_bits(val)
    }
}
impl From<Ch0vch2te> for u8 {
    #[inline(always)]
    fn from(val: Ch0vch2te) -> u8 {
        Ch0vch2te::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0vrtcate {
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC alarm signal is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC alarm signal is enabled."]
    _1 = 0x01,
}
impl Ch0vrtcate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0vrtcate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0vrtcate {
    #[inline(always)]
    fn from(val: u8) -> Ch0vrtcate {
        Ch0vrtcate::from_bits(val)
    }
}
impl From<Ch0vrtcate> for u8 {
    #[inline(always)]
    fn from(val: Ch0vrtcate) -> u8 {
        Ch0vrtcate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0vrtcte {
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC periodic signal is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC periodic signal is enabled."]
    _1 = 0x01,
}
impl Ch0vrtcte {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0vrtcte {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0vrtcte {
    #[inline(always)]
    fn from(val: u8) -> Ch0vrtcte {
        Ch0vrtcte::from_bits(val)
    }
}
impl From<Ch0vrtcte> for u8 {
    #[inline(always)]
    fn from(val: Ch0vrtcte) -> u8 {
        Ch0vrtcte::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1vch0te {
    #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is enabled."]
    _1 = 0x01,
}
impl Ch1vch0te {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1vch0te {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1vch0te {
    #[inline(always)]
    fn from(val: u8) -> Ch1vch0te {
        Ch1vch0te::from_bits(val)
    }
}
impl From<Ch1vch0te> for u8 {
    #[inline(always)]
    fn from(val: Ch1vch0te) -> u8 {
        Ch1vch0te::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1vch2te {
    #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is enabled."]
    _1 = 0x01,
}
impl Ch1vch2te {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1vch2te {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1vch2te {
    #[inline(always)]
    fn from(val: u8) -> Ch1vch2te {
        Ch1vch2te::from_bits(val)
    }
}
impl From<Ch1vch2te> for u8 {
    #[inline(always)]
    fn from(val: Ch1vch2te) -> u8 {
        Ch1vch2te::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1vrtcate {
    #[doc = "VBATT wakeup I/O 1 output trigger by the RTC alarm signal is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup I/O 1 output trigger by the RTC alarm signal is enabled."]
    _1 = 0x01,
}
impl Ch1vrtcate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1vrtcate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1vrtcate {
    #[inline(always)]
    fn from(val: u8) -> Ch1vrtcate {
        Ch1vrtcate::from_bits(val)
    }
}
impl From<Ch1vrtcate> for u8 {
    #[inline(always)]
    fn from(val: Ch1vrtcate) -> u8 {
        Ch1vrtcate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1vrtcte {
    #[doc = "VBATT wakeup I/O 1 output trigger by the RTC periodic signal is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup I/O 1 output trigger by the RTC periodic signal is enabled"]
    _1 = 0x01,
}
impl Ch1vrtcte {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1vrtcte {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1vrtcte {
    #[inline(always)]
    fn from(val: u8) -> Ch1vrtcte {
        Ch1vrtcte::from_bits(val)
    }
}
impl From<Ch1vrtcte> for u8 {
    #[inline(always)]
    fn from(val: Ch1vrtcte) -> u8 {
        Ch1vrtcte::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2vch0te {
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is enabled."]
    _1 = 0x01,
}
impl Ch2vch0te {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2vch0te {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2vch0te {
    #[inline(always)]
    fn from(val: u8) -> Ch2vch0te {
        Ch2vch0te::from_bits(val)
    }
}
impl From<Ch2vch0te> for u8 {
    #[inline(always)]
    fn from(val: Ch2vch0te) -> u8 {
        Ch2vch0te::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2vch1te {
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is enabled."]
    _1 = 0x01,
}
impl Ch2vch1te {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2vch1te {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2vch1te {
    #[inline(always)]
    fn from(val: u8) -> Ch2vch1te {
        Ch2vch1te::from_bits(val)
    }
}
impl From<Ch2vch1te> for u8 {
    #[inline(always)]
    fn from(val: Ch2vch1te) -> u8 {
        Ch2vch1te::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2vrtcate {
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC alarm signal is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC alarm signal is enabled."]
    _1 = 0x01,
}
impl Ch2vrtcate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2vrtcate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2vrtcate {
    #[inline(always)]
    fn from(val: u8) -> Ch2vrtcate {
        Ch2vrtcate::from_bits(val)
    }
}
impl From<Ch2vrtcate> for u8 {
    #[inline(always)]
    fn from(val: Ch2vrtcate) -> u8 {
        Ch2vrtcate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2vrtcte {
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC periodic signal is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC periodic signal is enabled."]
    _1 = 0x01,
}
impl Ch2vrtcte {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2vrtcte {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2vrtcte {
    #[inline(always)]
    fn from(val: u8) -> Ch2vrtcte {
        Ch2vrtcte::from_bits(val)
    }
}
impl From<Ch2vrtcte> for u8 {
    #[inline(always)]
    fn from(val: Ch2vrtcte) -> u8 {
        Ch2vrtcte::to_bits(val)
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
pub enum Ckoen {
    #[doc = "Clock Out disable"]
    _0 = 0x0,
    #[doc = "Clock Out enable"]
    _1 = 0x01,
}
impl Ckoen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ckoen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ckoen {
    #[inline(always)]
    fn from(val: u8) -> Ckoen {
        Ckoen::from_bits(val)
    }
}
impl From<Ckoen> for u8 {
    #[inline(always)]
    fn from(val: Ckoen) -> u8 {
        Ckoen::to_bits(val)
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
    _000 = 0x0,
    #[doc = "MOCO"]
    _001 = 0x01,
    #[doc = "LOCO"]
    _010 = 0x02,
    #[doc = "Main clock oscillator"]
    _011 = 0x03,
    #[doc = "Sub-clock oscillator"]
    _100 = 0x04,
    #[doc = "PLL"]
    _101 = 0x05,
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
pub enum Cmpe {
    #[doc = "Voltage Monitor circuit comparison result output disabled."]
    _0 = 0x0,
    #[doc = "Voltage Monitor circuit comparison result output enabled."]
    _1 = 0x01,
}
impl Cmpe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpe {
    #[inline(always)]
    fn from(val: u8) -> Cmpe {
        Cmpe::from_bits(val)
    }
}
impl From<Cmpe> for u8 {
    #[inline(always)]
    fn from(val: Cmpe) -> u8 {
        Cmpe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cwsf {
    #[doc = "Cold start"]
    _0 = 0x0,
    #[doc = "Warm start"]
    _1 = 0x01,
}
impl Cwsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cwsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cwsf {
    #[inline(always)]
    fn from(val: u8) -> Cwsf {
        Cwsf::from_bits(val)
    }
}
impl From<Cwsf> for u8 {
    #[inline(always)]
    fn from(val: Cwsf) -> u8 {
        Cwsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbgen {
    #[doc = "On-chip debugger is disabled"]
    _0 = 0x0,
    #[doc = "On-chip debugger is enabled"]
    _1 = 0x01,
}
impl Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbgen {
    #[inline(always)]
    fn from(val: u8) -> Dbgen {
        Dbgen::from_bits(val)
    }
}
impl From<Dbgen> for u8 {
    #[inline(always)]
    fn from(val: Dbgen) -> u8 {
        Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Det {
    #[doc = "Not detected"]
    _0 = 0x0,
    #[doc = "Vdet1 passage detection"]
    _1 = 0x01,
}
impl Det {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Det {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Det {
    #[inline(always)]
    fn from(val: u8) -> Det {
        Det::from_bits(val)
    }
}
impl From<Det> for u8 {
    #[inline(always)]
    fn from(val: Det) -> u8 {
        Det::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtcnzred {
    #[doc = "Disable the Snooze End request"]
    _0 = 0x0,
    #[doc = "Enable the Snooze End request"]
    _1 = 0x01,
}
impl Dtcnzred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtcnzred {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtcnzred {
    #[inline(always)]
    fn from(val: u8) -> Dtcnzred {
        Dtcnzred::from_bits(val)
    }
}
impl From<Dtcnzred> for u8 {
    #[inline(always)]
    fn from(val: Dtcnzred) -> u8 {
        Dtcnzred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtczred {
    #[doc = "Disable the Snooze End request"]
    _0 = 0x0,
    #[doc = "Enable the Snooze End request"]
    _1 = 0x01,
}
impl Dtczred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtczred {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtczred {
    #[inline(always)]
    fn from(val: u8) -> Dtczred {
        Dtczred::from_bits(val)
    }
}
impl From<Dtczred> for u8 {
    #[inline(always)]
    fn from(val: Dtczred) -> u8 {
        Dtczred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fck {
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flstop {
    #[doc = "Code flash and data flash memory operates"]
    _0 = 0x0,
    #[doc = "Code flash and data flash memory stops."]
    _1 = 0x01,
}
impl Flstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flstop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flstop {
    #[inline(always)]
    fn from(val: u8) -> Flstop {
        Flstop::from_bits(val)
    }
}
impl From<Flstop> for u8 {
    #[inline(always)]
    fn from(val: Flstop) -> u8 {
        Flstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flstpf {
    #[doc = "Transition completed"]
    _0 = 0x0,
    #[doc = "During transition (from the flash-stop-status to flash-operating-status or vice versa)"]
    _1 = 0x01,
}
impl Flstpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flstpf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flstpf {
    #[inline(always)]
    fn from(val: u8) -> Flstpf {
        Flstpf::from_bits(val)
    }
}
impl From<Flstpf> for u8 {
    #[inline(always)]
    fn from(val: Flstpf) -> u8 {
        Flstpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hcstp {
    #[doc = "HOCO is operating."]
    _0 = 0x0,
    #[doc = "HOCO is stopped."]
    _1 = 0x01,
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
pub enum Hocosf {
    #[doc = "The HOCO clock is stopped or oscillation of the HOCO clock has not yet become stable."]
    _0 = 0x0,
    #[doc = "Oscillation of the HOCO clock is stable so the clock is available for use as the system clock."]
    _1 = 0x01,
}
impl Hocosf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hocosf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hocosf {
    #[inline(always)]
    fn from(val: u8) -> Hocosf {
        Hocosf::from_bits(val)
    }
}
impl From<Hocosf> for u8 {
    #[inline(always)]
    fn from(val: Hocosf) -> u8 {
        Hocosf::to_bits(val)
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
pub enum Irqsel {
    #[doc = "Non-maskable interrupt"]
    _0 = 0x0,
    #[doc = "Maskable interrupt"]
    _1 = 0x01,
}
impl Irqsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqsel {
    #[inline(always)]
    fn from(val: u8) -> Irqsel {
        Irqsel::from_bits(val)
    }
}
impl From<Irqsel> for u8 {
    #[inline(always)]
    fn from(val: Irqsel) -> u8 {
        Irqsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iwdtrf {
    #[doc = "Independent watchdog timer reset not detected."]
    _0 = 0x0,
    #[doc = "Independent watchdog timer reset detected."]
    _1 = 0x01,
}
impl Iwdtrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iwdtrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iwdtrf {
    #[inline(always)]
    fn from(val: u8) -> Iwdtrf {
        Iwdtrf::from_bits(val)
    }
}
impl From<Iwdtrf> for u8 {
    #[inline(always)]
    fn from(val: Iwdtrf) -> u8 {
        Iwdtrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lcdscken {
    #[doc = "LCD source clock out disabled"]
    _0 = 0x0,
    #[doc = "LCD source clock out enabled."]
    _1 = 0x01,
}
impl Lcdscken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcdscken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcdscken {
    #[inline(always)]
    fn from(val: u8) -> Lcdscken {
        Lcdscken::from_bits(val)
    }
}
impl From<Lcdscken> for u8 {
    #[inline(always)]
    fn from(val: Lcdscken) -> u8 {
        Lcdscken::to_bits(val)
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
pub enum Lcstp {
    #[doc = "LOCO is operating."]
    _0 = 0x0,
    #[doc = "LOCO is stopped."]
    _1 = 0x01,
}
impl Lcstp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcstp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcstp {
    #[inline(always)]
    fn from(val: u8) -> Lcstp {
        Lcstp::from_bits(val)
    }
}
impl From<Lcstp> for u8 {
    #[inline(always)]
    fn from(val: Lcstp) -> u8 {
        Lcstp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvd0rf {
    #[doc = "Voltage Monitor 0 reset not detected."]
    _0 = 0x0,
    #[doc = "Voltage Monitor 0 reset detected."]
    _1 = 0x01,
}
impl Lvd0rf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd0rf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd0rf {
    #[inline(always)]
    fn from(val: u8) -> Lvd0rf {
        Lvd0rf::from_bits(val)
    }
}
impl From<Lvd0rf> for u8 {
    #[inline(always)]
    fn from(val: Lvd0rf) -> u8 {
        Lvd0rf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvd1e {
    #[doc = "Voltage detection 1 circuit disabled"]
    _0 = 0x0,
    #[doc = "Voltage detection 1 circuit enabled"]
    _1 = 0x01,
}
impl Lvd1e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd1e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd1e {
    #[inline(always)]
    fn from(val: u8) -> Lvd1e {
        Lvd1e::from_bits(val)
    }
}
impl From<Lvd1e> for u8 {
    #[inline(always)]
    fn from(val: Lvd1e) -> u8 {
        Lvd1e::to_bits(val)
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
pub enum Lvd1rf {
    #[doc = "Voltage Monitor 1 reset not detected."]
    _0 = 0x0,
    #[doc = "Voltage Monitor 1 reset detected."]
    _1 = 0x01,
}
impl Lvd1rf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd1rf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd1rf {
    #[inline(always)]
    fn from(val: u8) -> Lvd1rf {
        Lvd1rf::from_bits(val)
    }
}
impl From<Lvd1rf> for u8 {
    #[inline(always)]
    fn from(val: Lvd1rf) -> u8 {
        Lvd1rf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvd2e {
    #[doc = "Voltage detection 2 circuit disabled"]
    _0 = 0x0,
    #[doc = "Voltage detection 2 circuit enabled"]
    _1 = 0x01,
}
impl Lvd2e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd2e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd2e {
    #[inline(always)]
    fn from(val: u8) -> Lvd2e {
        Lvd2e::from_bits(val)
    }
}
impl From<Lvd2e> for u8 {
    #[inline(always)]
    fn from(val: Lvd2e) -> u8 {
        Lvd2e::to_bits(val)
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
pub enum Lvd2rf {
    #[doc = "Voltage Monitor 2 reset not detected."]
    _0 = 0x0,
    #[doc = "Voltage Monitor 2 reset detected."]
    _1 = 0x01,
}
impl Lvd2rf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd2rf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd2rf {
    #[inline(always)]
    fn from(val: u8) -> Lvd2rf {
        Lvd2rf::from_bits(val)
    }
}
impl From<Lvd2rf> for u8 {
    #[inline(always)]
    fn from(val: Lvd2rf) -> u8 {
        Lvd2rf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mcstp {
    #[doc = "MOCO is operating."]
    _0 = 0x0,
    #[doc = "MOCO is stopped."]
    _1 = 0x01,
}
impl Mcstp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mcstp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mcstp {
    #[inline(always)]
    fn from(val: u8) -> Mcstp {
        Mcstp::from_bits(val)
    }
}
impl From<Mcstp> for u8 {
    #[inline(always)]
    fn from(val: Mcstp) -> u8 {
        Mcstp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Memwait {
    #[doc = "no wait"]
    _0 = 0x0,
    #[doc = "wait"]
    _1 = 0x01,
}
impl Memwait {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Memwait {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Memwait {
    #[inline(always)]
    fn from(val: u8) -> Memwait {
        Memwait::from_bits(val)
    }
}
impl From<Memwait> for u8 {
    #[inline(always)]
    fn from(val: Memwait) -> u8 {
        Memwait::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Modrv1 {
    #[doc = "10 MHz to 20 MHz"]
    _0 = 0x0,
    #[doc = "1 MHz to 10 MHz."]
    _1 = 0x01,
}
impl Modrv1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Modrv1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Modrv1 {
    #[inline(always)]
    fn from(val: u8) -> Modrv1 {
        Modrv1::from_bits(val)
    }
}
impl From<Modrv1> for u8 {
    #[inline(always)]
    fn from(val: Modrv1) -> u8 {
        Modrv1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mon {
    #[doc = "VCC < Vdet"]
    _0 = 0x0,
    #[doc = "VCC >= Vdet or MON bit is disabled"]
    _1 = 0x01,
}
impl Mon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mon {
    #[inline(always)]
    fn from(val: u8) -> Mon {
        Mon::from_bits(val)
    }
}
impl From<Mon> for u8 {
    #[inline(always)]
    fn from(val: Mon) -> u8 {
        Mon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Moscsf {
    #[doc = "MOSTP = 1 (stopping the main clock oscillator) or oscillation of the main clock has not yet become stable."]
    _0 = 0x0,
    #[doc = "Oscillation of the main clock is stable so the clock is available for use as the system clock."]
    _1 = 0x01,
}
impl Moscsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Moscsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Moscsf {
    #[inline(always)]
    fn from(val: u8) -> Moscsf {
        Moscsf::from_bits(val)
    }
}
impl From<Moscsf> for u8 {
    #[inline(always)]
    fn from(val: Moscsf) -> u8 {
        Moscsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mosel {
    #[doc = "Resonator"]
    _0 = 0x0,
    #[doc = "External clock input"]
    _1 = 0x01,
}
impl Mosel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mosel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mosel {
    #[inline(always)]
    fn from(val: u8) -> Mosel {
        Mosel::from_bits(val)
    }
}
impl From<Mosel> for u8 {
    #[inline(always)]
    fn from(val: Mosel) -> u8 {
        Mosel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mostp {
    #[doc = "Main clock oscillator is operating."]
    _0 = 0x0,
    #[doc = "Main clock oscillator is stopped."]
    _1 = 0x01,
}
impl Mostp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mostp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mostp {
    #[inline(always)]
    fn from(val: u8) -> Mostp {
        Mostp::from_bits(val)
    }
}
impl From<Mostp> for u8 {
    #[inline(always)]
    fn from(val: Mostp) -> u8 {
        Mostp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpa0 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpa0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpa0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpa0 {
    #[inline(always)]
    fn from(val: u8) -> Mstpa0 {
        Mstpa0::from_bits(val)
    }
}
impl From<Mstpa0> for u8 {
    #[inline(always)]
    fn from(val: Mstpa0) -> u8 {
        Mstpa0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpa22 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpa22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpa22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpa22 {
    #[inline(always)]
    fn from(val: u8) -> Mstpa22 {
        Mstpa22::from_bits(val)
    }
}
impl From<Mstpa22> for u8 {
    #[inline(always)]
    fn from(val: Mstpa22) -> u8 {
        Mstpa22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpa6 {
    #[doc = "Cancel the module-stop state"]
    _0 = 0x0,
    #[doc = "Enter the module-stop state"]
    _1 = 0x01,
}
impl Mstpa6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpa6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpa6 {
    #[inline(always)]
    fn from(val: u8) -> Mstpa6 {
        Mstpa6::from_bits(val)
    }
}
impl From<Mstpa6> for u8 {
    #[inline(always)]
    fn from(val: Mstpa6) -> u8 {
        Mstpa6::to_bits(val)
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
    _00 = 0x0,
    #[doc = "Middle-speed mode"]
    _01 = 0x01,
    #[doc = "Low-voltage mode"]
    _10 = 0x02,
    #[doc = "Low-speed mode"]
    _11 = 0x03,
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
pub enum Opcmtsf {
    #[doc = "Transition completed"]
    _0 = 0x0,
    #[doc = "During transition"]
    _1 = 0x01,
}
impl Opcmtsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opcmtsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opcmtsf {
    #[inline(always)]
    fn from(val: u8) -> Opcmtsf {
        Opcmtsf::from_bits(val)
    }
}
impl From<Opcmtsf> for u8 {
    #[inline(always)]
    fn from(val: Opcmtsf) -> u8 {
        Opcmtsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ostde {
    #[doc = "Oscillation stop detection function is disabled."]
    _0 = 0x0,
    #[doc = "Oscillation stop detection function is enabled."]
    _1 = 0x01,
}
impl Ostde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ostde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ostde {
    #[inline(always)]
    fn from(val: u8) -> Ostde {
        Ostde::from_bits(val)
    }
}
impl From<Ostde> for u8 {
    #[inline(always)]
    fn from(val: Ostde) -> u8 {
        Ostde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ostdf {
    #[doc = "The main clock oscillation stop has not been detected."]
    _0 = 0x0,
    #[doc = "The main clock oscillation stop has been detected."]
    _1 = 0x01,
}
impl Ostdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ostdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ostdf {
    #[inline(always)]
    fn from(val: u8) -> Ostdf {
        Ostdf::from_bits(val)
    }
}
impl From<Ostdf> for u8 {
    #[inline(always)]
    fn from(val: Ostdf) -> u8 {
        Ostdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ostdie {
    #[doc = "The oscillation stop detection interrupt is disabled. Oscillation stop detection is not notified to the POEG."]
    _0 = 0x0,
    #[doc = "The oscillation stop detection interrupt is enabled. Oscillation stop detection is notified to the POEG."]
    _1 = 0x01,
}
impl Ostdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ostdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ostdie {
    #[inline(always)]
    fn from(val: u8) -> Ostdie {
        Ostdie::from_bits(val)
    }
}
impl From<Ostdie> for u8 {
    #[inline(always)]
    fn from(val: Ostdie) -> u8 {
        Ostdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcka {
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
pub enum Pllsf {
    #[doc = "The PLL clock is stopped or oscillation of the PLL clock has not yet become stable."]
    _0 = 0x0,
    #[doc = "Oscillation of the PLL clock is stable so the clock is available for use as the system clock."]
    _1 = 0x01,
}
impl Pllsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllsf {
    #[inline(always)]
    fn from(val: u8) -> Pllsf {
        Pllsf::from_bits(val)
    }
}
impl From<Pllsf> for u8 {
    #[inline(always)]
    fn from(val: Pllsf) -> u8 {
        Pllsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pllstp {
    #[doc = "PLL is operating."]
    _0 = 0x0,
    #[doc = "PLL is stopped."]
    _1 = 0x01,
}
impl Pllstp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllstp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllstp {
    #[inline(always)]
    fn from(val: u8) -> Pllstp {
        Pllstp::from_bits(val)
    }
}
impl From<Pllstp> for u8 {
    #[inline(always)]
    fn from(val: Pllstp) -> u8 {
        Pllstp::to_bits(val)
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
pub enum Porf {
    #[doc = "Power-on reset not detected."]
    _0 = 0x0,
    #[doc = "Power-on reset detected."]
    _1 = 0x01,
}
impl Porf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Porf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Porf {
    #[inline(always)]
    fn from(val: u8) -> Porf {
        Porf::from_bits(val)
    }
}
impl From<Porf> for u8 {
    #[inline(always)]
    fn from(val: Porf) -> u8 {
        Porf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prc0 {
    #[doc = "Writes protected."]
    _0 = 0x0,
    #[doc = "Writes not protected."]
    _1 = 0x01,
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
    _0 = 0x0,
    #[doc = "Writes not protected."]
    _1 = 0x01,
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
    _0 = 0x0,
    #[doc = "Writes not protected."]
    _1 = 0x01,
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
    #[doc = "Enables writing to the PRCR register."]
    pub const _0X5A: Self = Self(0x5a);
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
            0x5a => f.write_str("_0X5A"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prkey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x5a => defmt::write!(f, "_0X5A"),
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
pub enum Reerf {
    #[doc = "RAM ECC error reset not detected."]
    _0 = 0x0,
    #[doc = "RAM ECC error reset detected."]
    _1 = 0x01,
}
impl Reerf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reerf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reerf {
    #[inline(always)]
    fn from(val: u8) -> Reerf {
        Reerf::from_bits(val)
    }
}
impl From<Reerf> for u8 {
    #[inline(always)]
    fn from(val: Reerf) -> u8 {
        Reerf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ri {
    #[doc = "Voltage Monitor interrupt during Vdet1 passage"]
    _0 = 0x0,
    #[doc = "Voltage Monitor reset enabled when the voltage falls to and below Vdet1"]
    _1 = 0x01,
}
impl Ri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ri {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ri {
    #[inline(always)]
    fn from(val: u8) -> Ri {
        Ri::from_bits(val)
    }
}
impl From<Ri> for u8 {
    #[inline(always)]
    fn from(val: Ri) -> u8 {
        Ri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rie {
    #[doc = "Disabled"]
    _0 = 0x0,
    #[doc = "Enabled"]
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
pub enum Rn {
    #[doc = "Negation follows a stabilization time (tLVD) after VCC > Vdet1 is detected."]
    _0 = 0x0,
    #[doc = "Negation follows a stabilization time (tLVD) after assertion of the LVD reset."]
    _1 = 0x01,
}
impl Rn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rn {
    #[inline(always)]
    fn from(val: u8) -> Rn {
        Rn::from_bits(val)
    }
}
impl From<Rn> for u8 {
    #[inline(always)]
    fn from(val: Rn) -> u8 {
        Rn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rperf {
    #[doc = "RAM parity error reset not detected."]
    _0 = 0x0,
    #[doc = "RAM parity error reset detected."]
    _1 = 0x01,
}
impl Rperf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rperf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rperf {
    #[inline(always)]
    fn from(val: u8) -> Rperf {
        Rperf::from_bits(val)
    }
}
impl From<Rperf> for u8 {
    #[inline(always)]
    fn from(val: Rperf) -> u8 {
        Rperf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxdreqen {
    #[doc = "Ignore RXD0 falling edge in Software Standby mode."]
    _0 = 0x0,
    #[doc = "Accept RXD0 falling edge in Standby mode as a request to transit to Snooze mode."]
    _1 = 0x01,
}
impl Rxdreqen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxdreqen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxdreqen {
    #[inline(always)]
    fn from(val: u8) -> Rxdreqen {
        Rxdreqen::from_bits(val)
    }
}
impl From<Rxdreqen> for u8 {
    #[inline(always)]
    fn from(val: Rxdreqen) -> u8 {
        Rxdreqen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sci0umted {
    #[doc = "Disable the Snooze End request"]
    _0 = 0x0,
    #[doc = "Enable the Snooze End request"]
    _1 = 0x01,
}
impl Sci0umted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sci0umted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sci0umted {
    #[inline(always)]
    fn from(val: u8) -> Sci0umted {
        Sci0umted::from_bits(val)
    }
}
impl From<Sci0umted> for u8 {
    #[inline(always)]
    fn from(val: Sci0umted) -> u8 {
        Sci0umted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzdtcen {
    #[doc = "Disable DTC operation"]
    _0 = 0x0,
    #[doc = "Enable DTC operation"]
    _1 = 0x01,
}
impl Snzdtcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzdtcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzdtcen {
    #[inline(always)]
    fn from(val: u8) -> Snzdtcen {
        Snzdtcen::from_bits(val)
    }
}
impl From<Snzdtcen> for u8 {
    #[inline(always)]
    fn from(val: Snzdtcen) -> u8 {
        Snzdtcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snze {
    #[doc = "Disable Snooze Mode"]
    _0 = 0x0,
    #[doc = "Enable Snooze Mode"]
    _1 = 0x01,
}
impl Snze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snze {
    #[inline(always)]
    fn from(val: u8) -> Snze {
        Snze::from_bits(val)
    }
}
impl From<Snze> for u8 {
    #[inline(always)]
    fn from(val: Snze) -> u8 {
        Snze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen0 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen0 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen0 {
        Snzreqen0::from_bits(val)
    }
}
impl From<Snzreqen0> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen0) -> u8 {
        Snzreqen0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen1 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen1 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen1 {
        Snzreqen1::from_bits(val)
    }
}
impl From<Snzreqen1> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen1) -> u8 {
        Snzreqen1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen10 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen10 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen10 {
        Snzreqen10::from_bits(val)
    }
}
impl From<Snzreqen10> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen10) -> u8 {
        Snzreqen10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen11 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen11 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen11 {
        Snzreqen11::from_bits(val)
    }
}
impl From<Snzreqen11> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen11) -> u8 {
        Snzreqen11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen12 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen12 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen12 {
        Snzreqen12::from_bits(val)
    }
}
impl From<Snzreqen12> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen12) -> u8 {
        Snzreqen12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen14 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen14 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen14 {
        Snzreqen14::from_bits(val)
    }
}
impl From<Snzreqen14> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen14) -> u8 {
        Snzreqen14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen15 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen15 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen15 {
        Snzreqen15::from_bits(val)
    }
}
impl From<Snzreqen15> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen15) -> u8 {
        Snzreqen15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen17 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen17 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen17 {
        Snzreqen17::from_bits(val)
    }
}
impl From<Snzreqen17> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen17) -> u8 {
        Snzreqen17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen2 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen2 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen2 {
        Snzreqen2::from_bits(val)
    }
}
impl From<Snzreqen2> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen2) -> u8 {
        Snzreqen2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen23 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen23 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen23 {
        Snzreqen23::from_bits(val)
    }
}
impl From<Snzreqen23> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen23) -> u8 {
        Snzreqen23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen24 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen24 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen24 {
        Snzreqen24::from_bits(val)
    }
}
impl From<Snzreqen24> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen24) -> u8 {
        Snzreqen24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen25 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen25 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen25 {
        Snzreqen25::from_bits(val)
    }
}
impl From<Snzreqen25> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen25) -> u8 {
        Snzreqen25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen28 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen28 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen28 {
        Snzreqen28::from_bits(val)
    }
}
impl From<Snzreqen28> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen28) -> u8 {
        Snzreqen28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen29 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen29 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen29 {
        Snzreqen29::from_bits(val)
    }
}
impl From<Snzreqen29> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen29) -> u8 {
        Snzreqen29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen3 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen3 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen3 {
        Snzreqen3::from_bits(val)
    }
}
impl From<Snzreqen3> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen3) -> u8 {
        Snzreqen3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen30 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen30 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen30 {
        Snzreqen30::from_bits(val)
    }
}
impl From<Snzreqen30> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen30) -> u8 {
        Snzreqen30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen4 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen4 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen4 {
        Snzreqen4::from_bits(val)
    }
}
impl From<Snzreqen4> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen4) -> u8 {
        Snzreqen4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen5 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen5 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen5 {
        Snzreqen5::from_bits(val)
    }
}
impl From<Snzreqen5> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen5) -> u8 {
        Snzreqen5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen6 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen6 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen6 {
        Snzreqen6::from_bits(val)
    }
}
impl From<Snzreqen6> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen6) -> u8 {
        Snzreqen6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen7 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen7 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen7 {
        Snzreqen7::from_bits(val)
    }
}
impl From<Snzreqen7> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen7) -> u8 {
        Snzreqen7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen8 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen8 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen8 {
        Snzreqen8::from_bits(val)
    }
}
impl From<Snzreqen8> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen8) -> u8 {
        Snzreqen8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Snzreqen9 {
    #[doc = "Disable snooze request"]
    _0 = 0x0,
    #[doc = "Enable snooze request"]
    _1 = 0x01,
}
impl Snzreqen9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Snzreqen9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Snzreqen9 {
    #[inline(always)]
    fn from(val: u8) -> Snzreqen9 {
        Snzreqen9::from_bits(val)
    }
}
impl From<Snzreqen9> for u8 {
    #[inline(always)]
    fn from(val: Snzreqen9) -> u8 {
        Snzreqen9::to_bits(val)
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
pub enum Sopcm {
    #[doc = "Other than Subosc-speed mode"]
    _0 = 0x0,
    #[doc = "Subosc-speed mode"]
    _1 = 0x01,
}
impl Sopcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sopcm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sopcm {
    #[inline(always)]
    fn from(val: u8) -> Sopcm {
        Sopcm::from_bits(val)
    }
}
impl From<Sopcm> for u8 {
    #[inline(always)]
    fn from(val: Sopcm) -> u8 {
        Sopcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sopcmtsf {
    #[doc = "Transition completed"]
    _0 = 0x0,
    #[doc = "During transition"]
    _1 = 0x01,
}
impl Sopcmtsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sopcmtsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sopcmtsf {
    #[inline(always)]
    fn from(val: u8) -> Sopcmtsf {
        Sopcmtsf::from_bits(val)
    }
}
impl From<Sopcmtsf> for u8 {
    #[inline(always)]
    fn from(val: Sopcmtsf) -> u8 {
        Sopcmtsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sostp {
    #[doc = "Sub-clock oscillator is operating."]
    _0 = 0x0,
    #[doc = "Sub-clock oscillator is stopped."]
    _1 = 0x01,
}
impl Sostp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sostp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sostp {
    #[inline(always)]
    fn from(val: u8) -> Sostp {
        Sostp::from_bits(val)
    }
}
impl From<Sostp> for u8 {
    #[inline(always)]
    fn from(val: Sostp) -> u8 {
        Sostp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sperf {
    #[doc = "SP error reset not detected."]
    _0 = 0x0,
    #[doc = "SP error reset detected."]
    _1 = 0x01,
}
impl Sperf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sperf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sperf {
    #[inline(always)]
    fn from(val: u8) -> Sperf {
        Sperf::from_bits(val)
    }
}
impl From<Sperf> for u8 {
    #[inline(always)]
    fn from(val: Sperf) -> u8 {
        Sperf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssby {
    #[doc = "Sleep mode"]
    _0 = 0x0,
    #[doc = "Software Standby mode"]
    _1 = 0x01,
}
impl Ssby {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssby {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssby {
    #[inline(always)]
    fn from(val: u8) -> Ssby {
        Ssby::from_bits(val)
    }
}
impl From<Ssby> for u8 {
    #[inline(always)]
    fn from(val: Ssby) -> u8 {
        Ssby::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swrf {
    #[doc = "Software reset not detected."]
    _0 = 0x0,
    #[doc = "Software reset detected."]
    _1 = 0x01,
}
impl Swrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swrf {
    #[inline(always)]
    fn from(val: u8) -> Swrf {
        Swrf::from_bits(val)
    }
}
impl From<Swrf> for u8 {
    #[inline(always)]
    fn from(val: Swrf) -> u8 {
        Swrf::to_bits(val)
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
pub enum Trcken {
    #[doc = "Operation disabled"]
    _0 = 0x0,
    #[doc = "Operation enabled."]
    _1 = 0x01,
}
impl Trcken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trcken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trcken {
    #[inline(always)]
    fn from(val: u8) -> Trcken {
        Trcken::from_bits(val)
    }
}
impl From<Trcken> for u8 {
    #[inline(always)]
    fn from(val: Trcken) -> u8 {
        Trcken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbclksel {
    #[doc = "PLL(Value after reset)"]
    _0 = 0x0,
    #[doc = "HOCO"]
    _1 = 0x01,
}
impl Usbclksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbclksel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbclksel {
    #[inline(always)]
    fn from(val: u8) -> Usbclksel {
        Usbclksel::from_bits(val)
    }
}
impl From<Usbclksel> for u8 {
    #[inline(always)]
    fn from(val: Usbclksel) -> u8 {
        Usbclksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbtbldf {
    #[doc = "VBATT pin low voltage not detected"]
    _0 = 0x0,
    #[doc = "VBATT pin low voltage detected."]
    _1 = 0x01,
}
impl Vbtbldf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbtbldf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbtbldf {
    #[inline(always)]
    fn from(val: u8) -> Vbtbldf {
        Vbtbldf::from_bits(val)
    }
}
impl From<Vbtbldf> for u8 {
    #[inline(always)]
    fn from(val: Vbtbldf) -> u8 {
        Vbtbldf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbtcmpe {
    #[doc = "VBATT pin low voltage detect circuit output disabled"]
    _0 = 0x0,
    #[doc = "VBATT pin low voltage detect circuit output enabled"]
    _1 = 0x01,
}
impl Vbtcmpe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbtcmpe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbtcmpe {
    #[inline(always)]
    fn from(val: u8) -> Vbtcmpe {
        Vbtcmpe::from_bits(val)
    }
}
impl From<Vbtcmpe> for u8 {
    #[inline(always)]
    fn from(val: Vbtcmpe) -> u8 {
        Vbtcmpe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbtlvden {
    #[doc = "VBATT pin low voltage detect disable"]
    _0 = 0x0,
    #[doc = "VBATT pin low voltage detect enable"]
    _1 = 0x01,
}
impl Vbtlvden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbtlvden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbtlvden {
    #[inline(always)]
    fn from(val: u8) -> Vbtlvden {
        Vbtlvden::from_bits(val)
    }
}
impl From<Vbtlvden> for u8 {
    #[inline(always)]
    fn from(val: Vbtlvden) -> u8 {
        Vbtlvden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbtlvdie {
    #[doc = "VBATT Pin Low Voltage Detect Interrupt Disable"]
    _0 = 0x0,
    #[doc = "VBATT Pin Low Voltage Detect Interrupt Enable"]
    _1 = 0x01,
}
impl Vbtlvdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbtlvdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbtlvdie {
    #[inline(always)]
    fn from(val: u8) -> Vbtlvdie {
        Vbtlvdie::from_bits(val)
    }
}
impl From<Vbtlvdie> for u8 {
    #[inline(always)]
    fn from(val: Vbtlvdie) -> u8 {
        Vbtlvdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbtlvdisel {
    #[doc = "Non Maskable Interrupt"]
    _0 = 0x0,
    #[doc = "Maskable Interrupt"]
    _1 = 0x01,
}
impl Vbtlvdisel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbtlvdisel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbtlvdisel {
    #[inline(always)]
    fn from(val: u8) -> Vbtlvdisel {
        Vbtlvdisel::from_bits(val)
    }
}
impl From<Vbtlvdisel> for u8 {
    #[inline(always)]
    fn from(val: Vbtlvdisel) -> u8 {
        Vbtlvdisel::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbtrdf {
    #[doc = "VBATT_R voltage power-on reset not detected"]
    _0 = 0x0,
    #[doc = "VBATT_R selected voltage power-on reset detected."]
    _1 = 0x01,
}
impl Vbtrdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbtrdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbtrdf {
    #[inline(always)]
    fn from(val: u8) -> Vbtrdf {
        Vbtrdf::from_bits(val)
    }
}
impl From<Vbtrdf> for u8 {
    #[inline(always)]
    fn from(val: Vbtrdf) -> u8 {
        Vbtrdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbtrvld {
    #[doc = "VBATT_R area not valid"]
    _0 = 0x0,
    #[doc = "VBATT_R area valid"]
    _1 = 0x01,
}
impl Vbtrvld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbtrvld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbtrvld {
    #[inline(always)]
    fn from(val: u8) -> Vbtrvld {
        Vbtrvld::from_bits(val)
    }
}
impl From<Vbtrvld> for u8 {
    #[inline(always)]
    fn from(val: Vbtrvld) -> u8 {
        Vbtrvld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch0e {
    #[doc = "VBATT wakeup triggered by the VBATWIO0 pin is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup triggered by the VBATWIO0 pin is enabled."]
    _1 = 0x01,
}
impl Vch0e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch0e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch0e {
    #[inline(always)]
    fn from(val: u8) -> Vch0e {
        Vch0e::from_bits(val)
    }
}
impl From<Vch0e> for u8 {
    #[inline(always)]
    fn from(val: Vch0e) -> u8 {
        Vch0e::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch0eg {
    #[doc = "Wakeup trigger is generated at a falling edge"]
    _0 = 0x0,
    #[doc = "Wakeup trigger is generated at a rising edge."]
    _1 = 0x01,
}
impl Vch0eg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch0eg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch0eg {
    #[inline(always)]
    fn from(val: u8) -> Vch0eg {
        Vch0eg::from_bits(val)
    }
}
impl From<Vch0eg> for u8 {
    #[inline(always)]
    fn from(val: Vch0eg) -> u8 {
        Vch0eg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch0f {
    #[doc = "No wakeup trigger by the VBATWIO0 pin is generated"]
    _0 = 0x0,
    #[doc = "A wakeup trigger by the VBATWIO0 pin is generated"]
    _1 = 0x01,
}
impl Vch0f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch0f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch0f {
    #[inline(always)]
    fn from(val: u8) -> Vch0f {
        Vch0f::from_bits(val)
    }
}
impl From<Vch0f> for u8 {
    #[inline(always)]
    fn from(val: Vch0f) -> u8 {
        Vch0f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch0inen {
    #[doc = "VBATWIO0, RTCIC0 inputs disabled"]
    _0 = 0x0,
    #[doc = "VBATWIO0, RTCIC0 inputs enabled."]
    _1 = 0x01,
}
impl Vch0inen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch0inen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch0inen {
    #[inline(always)]
    fn from(val: u8) -> Vch0inen {
        Vch0inen::from_bits(val)
    }
}
impl From<Vch0inen> for u8 {
    #[inline(always)]
    fn from(val: Vch0inen) -> u8 {
        Vch0inen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch0oen {
    #[doc = "VBATWIO0 output disabled"]
    _0 = 0x0,
    #[doc = "VBATWIO0 output enabled"]
    _1 = 0x01,
}
impl Vch0oen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch0oen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch0oen {
    #[inline(always)]
    fn from(val: u8) -> Vch0oen {
        Vch0oen::from_bits(val)
    }
}
impl From<Vch0oen> for u8 {
    #[inline(always)]
    fn from(val: Vch0oen) -> u8 {
        Vch0oen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch1e {
    #[doc = "VBATT wakeup triggered by the VBATWIO1 pin is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup triggered by the VBATWIO1 pin is enabled."]
    _1 = 0x01,
}
impl Vch1e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch1e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch1e {
    #[inline(always)]
    fn from(val: u8) -> Vch1e {
        Vch1e::from_bits(val)
    }
}
impl From<Vch1e> for u8 {
    #[inline(always)]
    fn from(val: Vch1e) -> u8 {
        Vch1e::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch1eg {
    #[doc = "Wakeup trigger is generated at a falling edge"]
    _0 = 0x0,
    #[doc = "Wakeup trigger is generated at a rising edge."]
    _1 = 0x01,
}
impl Vch1eg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch1eg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch1eg {
    #[inline(always)]
    fn from(val: u8) -> Vch1eg {
        Vch1eg::from_bits(val)
    }
}
impl From<Vch1eg> for u8 {
    #[inline(always)]
    fn from(val: Vch1eg) -> u8 {
        Vch1eg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch1f {
    #[doc = "No wakeup trigger by the VBATWIO1 pin is generated"]
    _0 = 0x0,
    #[doc = "A wakeup trigger by the VBATWIO1 pin is generated"]
    _1 = 0x01,
}
impl Vch1f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch1f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch1f {
    #[inline(always)]
    fn from(val: u8) -> Vch1f {
        Vch1f::from_bits(val)
    }
}
impl From<Vch1f> for u8 {
    #[inline(always)]
    fn from(val: Vch1f) -> u8 {
        Vch1f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch1inen {
    #[doc = "VBATWIO1, RTCIC1 inputs disabled"]
    _0 = 0x0,
    #[doc = "VBATWIO1, RTCIC1 inputs enabled."]
    _1 = 0x01,
}
impl Vch1inen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch1inen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch1inen {
    #[inline(always)]
    fn from(val: u8) -> Vch1inen {
        Vch1inen::from_bits(val)
    }
}
impl From<Vch1inen> for u8 {
    #[inline(always)]
    fn from(val: Vch1inen) -> u8 {
        Vch1inen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch1oen {
    #[doc = "VBATWIO1 output disabled"]
    _0 = 0x0,
    #[doc = "VBATWIO1 output enabled"]
    _1 = 0x01,
}
impl Vch1oen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch1oen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch1oen {
    #[inline(always)]
    fn from(val: u8) -> Vch1oen {
        Vch1oen::from_bits(val)
    }
}
impl From<Vch1oen> for u8 {
    #[inline(always)]
    fn from(val: Vch1oen) -> u8 {
        Vch1oen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch2e {
    #[doc = "VBATT wakeup triggered by the VBATWIO2 pin is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup triggered by the VBATWIO2 pin is enabled."]
    _1 = 0x01,
}
impl Vch2e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch2e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch2e {
    #[inline(always)]
    fn from(val: u8) -> Vch2e {
        Vch2e::from_bits(val)
    }
}
impl From<Vch2e> for u8 {
    #[inline(always)]
    fn from(val: Vch2e) -> u8 {
        Vch2e::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch2eg {
    #[doc = "Wakeup trigger is generated at a falling edge"]
    _0 = 0x0,
    #[doc = "Wakeup trigger is generated at a rising edge."]
    _1 = 0x01,
}
impl Vch2eg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch2eg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch2eg {
    #[inline(always)]
    fn from(val: u8) -> Vch2eg {
        Vch2eg::from_bits(val)
    }
}
impl From<Vch2eg> for u8 {
    #[inline(always)]
    fn from(val: Vch2eg) -> u8 {
        Vch2eg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch2f {
    #[doc = "No wakeup trigger by the VBATWIO2 pin is generated"]
    _0 = 0x0,
    #[doc = "A wakeup trigger by the VBATWIO2 pin is generated"]
    _1 = 0x01,
}
impl Vch2f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch2f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch2f {
    #[inline(always)]
    fn from(val: u8) -> Vch2f {
        Vch2f::from_bits(val)
    }
}
impl From<Vch2f> for u8 {
    #[inline(always)]
    fn from(val: Vch2f) -> u8 {
        Vch2f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch2inen {
    #[doc = "VBATWIO2 and RTCIC2 inputs disabled"]
    _0 = 0x0,
    #[doc = "VBATWIO2 and RTCIC2 inputs enabled."]
    _1 = 0x01,
}
impl Vch2inen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch2inen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch2inen {
    #[inline(always)]
    fn from(val: u8) -> Vch2inen {
        Vch2inen::from_bits(val)
    }
}
impl From<Vch2inen> for u8 {
    #[inline(always)]
    fn from(val: Vch2inen) -> u8 {
        Vch2inen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vch2oen {
    #[doc = "VBATWIO2 output disabled"]
    _0 = 0x0,
    #[doc = "VBATWIO2 output enabled"]
    _1 = 0x01,
}
impl Vch2oen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vch2oen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vch2oen {
    #[inline(always)]
    fn from(val: u8) -> Vch2oen {
        Vch2oen::from_bits(val)
    }
}
impl From<Vch2oen> for u8 {
    #[inline(always)]
    fn from(val: Vch2oen) -> u8 {
        Vch2oen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vout0lsel {
    #[doc = "Output L before VBATT wakeup trigger"]
    _0 = 0x0,
    #[doc = "Output H before VBATT wakeup trigger"]
    _1 = 0x01,
}
impl Vout0lsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vout0lsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vout0lsel {
    #[inline(always)]
    fn from(val: u8) -> Vout0lsel {
        Vout0lsel::from_bits(val)
    }
}
impl From<Vout0lsel> for u8 {
    #[inline(always)]
    fn from(val: Vout0lsel) -> u8 {
        Vout0lsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vout1lsel {
    #[doc = "Output L before VBATT wake up trigger"]
    _0 = 0x0,
    #[doc = "Output H before VBATT wake up trigger"]
    _1 = 0x01,
}
impl Vout1lsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vout1lsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vout1lsel {
    #[inline(always)]
    fn from(val: u8) -> Vout1lsel {
        Vout1lsel::from_bits(val)
    }
}
impl From<Vout1lsel> for u8 {
    #[inline(always)]
    fn from(val: Vout1lsel) -> u8 {
        Vout1lsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vout2lsel {
    #[doc = "Output L before VBATT wake up trigger"]
    _0 = 0x0,
    #[doc = "Output H before VBATT wake up trigger"]
    _1 = 0x01,
}
impl Vout2lsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vout2lsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vout2lsel {
    #[inline(always)]
    fn from(val: u8) -> Vout2lsel {
        Vout2lsel::from_bits(val)
    }
}
impl From<Vout2lsel> for u8 {
    #[inline(always)]
    fn from(val: Vout2lsel) -> u8 {
        Vout2lsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrtcae {
    #[doc = "VBATT wakeup triggered by RTC alarm signal is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup triggered by RTC alarm signal is enabled."]
    _1 = 0x01,
}
impl Vrtcae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vrtcae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vrtcae {
    #[inline(always)]
    fn from(val: u8) -> Vrtcae {
        Vrtcae::from_bits(val)
    }
}
impl From<Vrtcae> for u8 {
    #[inline(always)]
    fn from(val: Vrtcae) -> u8 {
        Vrtcae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrtcaf {
    #[doc = "No wakeup trigger by the RTC alarm is generated"]
    _0 = 0x0,
    #[doc = "A wakeup trigger by the RTC alarm is generated"]
    _1 = 0x01,
}
impl Vrtcaf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vrtcaf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vrtcaf {
    #[inline(always)]
    fn from(val: u8) -> Vrtcaf {
        Vrtcaf::from_bits(val)
    }
}
impl From<Vrtcaf> for u8 {
    #[inline(always)]
    fn from(val: Vrtcaf) -> u8 {
        Vrtcaf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrtcie {
    #[doc = "VBATT wakeup triggered by RTC periodic signal is disabled"]
    _0 = 0x0,
    #[doc = "VBATT wakeup triggered by RTC periodic signal is enabled."]
    _1 = 0x01,
}
impl Vrtcie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vrtcie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vrtcie {
    #[inline(always)]
    fn from(val: u8) -> Vrtcie {
        Vrtcie::from_bits(val)
    }
}
impl From<Vrtcie> for u8 {
    #[inline(always)]
    fn from(val: Vrtcie) -> u8 {
        Vrtcie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrtcif {
    #[doc = "No wakeup trigger by the RTC interval is generated"]
    _0 = 0x0,
    #[doc = "A wakeup trigger by the RTC interval is generated"]
    _1 = 0x01,
}
impl Vrtcif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vrtcif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vrtcif {
    #[inline(always)]
    fn from(val: u8) -> Vrtcif {
        Vrtcif::from_bits(val)
    }
}
impl From<Vrtcif> for u8 {
    #[inline(always)]
    fn from(val: Vrtcif) -> u8 {
        Vrtcif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vwen {
    #[doc = "Disable Wakeup function"]
    _0 = 0x0,
    #[doc = "Enable Wakeup function"]
    _1 = 0x01,
}
impl Vwen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vwen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vwen {
    #[inline(always)]
    fn from(val: u8) -> Vwen {
        Vwen::from_bits(val)
    }
}
impl From<Vwen> for u8 {
    #[inline(always)]
    fn from(val: Vwen) -> u8 {
        Vwen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdtrf {
    #[doc = "Watchdog timer reset not detected."]
    _0 = 0x0,
    #[doc = "Watchdog timer reset detected."]
    _1 = 0x01,
}
impl Wdtrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdtrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdtrf {
    #[inline(always)]
    fn from(val: u8) -> Wdtrf {
        Wdtrf::from_bits(val)
    }
}
impl From<Wdtrf> for u8 {
    #[inline(always)]
    fn from(val: Wdtrf) -> u8 {
        Wdtrf::to_bits(val)
    }
}
