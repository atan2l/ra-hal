#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bom {
    #[doc = "Normal mode (ISO11898-1 compliant)"]
    _00 = 0x0,
    #[doc = "Entry to CAN halt mode automatically at bus-off entry"]
    _01 = 0x01,
    #[doc = "Entry to CAN halt mode automatically at bus-off end"]
    _10 = 0x02,
    #[doc = "Entry to CAN halt mode (during bus-off recovery period)"]
    _11 = 0x03,
}
impl Bom {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bom {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bom {
    #[inline(always)]
    fn from(val: u8) -> Bom {
        Bom::from_bits(val)
    }
}
impl From<Bom> for u8 {
    #[inline(always)]
    fn from(val: Bom) -> u8 {
        Bom::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Canm {
    #[doc = "CAN operation mode"]
    _00 = 0x0,
    #[doc = "CAN reset mode"]
    _01 = 0x01,
    #[doc = "CAN halt mode"]
    _10 = 0x02,
    #[doc = "CAN reset mode (forcible transition)"]
    _11 = 0x03,
}
impl Canm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Canm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Canm {
    #[inline(always)]
    fn from(val: u8) -> Canm {
        Canm::from_bits(val)
    }
}
impl From<Canm> for u8 {
    #[inline(always)]
    fn from(val: Canm) -> u8 {
        Canm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dlc {
    #[doc = "Data length = 0 byte"]
    _0000 = 0x0,
    #[doc = "Data length = 1 byte"]
    _0001 = 0x01,
    #[doc = "Data length = 2 bytes"]
    _0010 = 0x02,
    #[doc = "Data length = 3 bytes"]
    _0011 = 0x03,
    #[doc = "Data length = 4 bytes"]
    _0100 = 0x04,
    #[doc = "Data length = 5 bytes"]
    _0101 = 0x05,
    #[doc = "Data length = 6 bytes"]
    _0110 = 0x06,
    #[doc = "Data length = 7 bytes"]
    _0111 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Dlc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dlc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dlc {
    #[inline(always)]
    fn from(val: u8) -> Dlc {
        Dlc::from_bits(val)
    }
}
impl From<Dlc> for u8 {
    #[inline(always)]
    fn from(val: Dlc) -> u8 {
        Dlc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idfm {
    #[doc = "Standard ID mode.All mailboxes (including FIFO mailboxes) handle only standard Ids."]
    _00 = 0x0,
    #[doc = "Extended ID mode.All mailboxes (including FIFO mailboxes) handle only extended IDs."]
    _01 = 0x01,
    #[doc = "Mixed ID mode.All mailboxes (including FIFO mailboxes) handle both standard IDs and extended IDs. Standard IDs or extended IDs are specified by using the IDE bit in the corresponding mailbox in normal mailbox mode. In FIFO mailbox mode, the IDE bit in the corresponding mailbox is used for mailboxes \\[0\\] to \\[23\\], the IDE bits in FIDCR0 and FIDCR1 are used for the receive FIFO, and the IDE bit in mailbox \\[24\\] is used for the transmit FIFO."]
    _10 = 0x02,
    #[doc = "Do not use this combination"]
    _11 = 0x03,
}
impl Idfm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idfm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idfm {
    #[inline(always)]
    fn from(val: u8) -> Idfm {
        Idfm::from_bits(val)
    }
}
impl From<Idfm> for u8 {
    #[inline(always)]
    fn from(val: Idfm) -> u8 {
        Idfm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbsm {
    #[doc = "Receive mailbox search mode"]
    _00 = 0x0,
    #[doc = "Transmit mailbox search mode"]
    _01 = 0x01,
    #[doc = "Message lost search mode"]
    _10 = 0x02,
    #[doc = "Channel search mode"]
    _11 = 0x03,
}
impl Mbsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbsm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbsm {
    #[inline(always)]
    fn from(val: u8) -> Mbsm {
        Mbsm::from_bits(val)
    }
}
impl From<Mbsm> for u8 {
    #[inline(always)]
    fn from(val: Mbsm) -> u8 {
        Mbsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfust {
    #[doc = "No unread message"]
    _000 = 0x0,
    #[doc = "1 unread message"]
    _001 = 0x01,
    #[doc = "2 unread messages"]
    _010 = 0x02,
    #[doc = "3 unread messages"]
    _011 = 0x03,
    #[doc = "4 unread messages"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Rfust {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfust {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfust {
    #[inline(always)]
    fn from(val: u8) -> Rfust {
        Rfust::from_bits(val)
    }
}
impl From<Rfust> for u8 {
    #[inline(always)]
    fn from(val: Rfust) -> u8 {
        Rfust::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sjw {
    #[doc = "1 Tq"]
    _00 = 0x0,
    #[doc = "2 Tq"]
    _01 = 0x01,
    #[doc = "3 Tq"]
    _10 = 0x02,
    #[doc = "4 Tq"]
    _11 = 0x03,
}
impl Sjw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sjw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sjw {
    #[inline(always)]
    fn from(val: u8) -> Sjw {
        Sjw::from_bits(val)
    }
}
impl From<Sjw> for u8 {
    #[inline(always)]
    fn from(val: Sjw) -> u8 {
        Sjw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfust {
    #[doc = "No unsent message"]
    _000 = 0x0,
    #[doc = "1 unsent message"]
    _001 = 0x01,
    #[doc = "2 unsent messages"]
    _010 = 0x02,
    #[doc = "3 unsent messages"]
    _011 = 0x03,
    #[doc = "4 unsent messages"]
    _100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tfust {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfust {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfust {
    #[inline(always)]
    fn from(val: u8) -> Tfust {
        Tfust::from_bits(val)
    }
}
impl From<Tfust> for u8 {
    #[inline(always)]
    fn from(val: Tfust) -> u8 {
        Tfust::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tseg1 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "4 Tq"]
    _0011 = 0x03,
    #[doc = "5 Tq"]
    _0100 = 0x04,
    #[doc = "6 Tq"]
    _0101 = 0x05,
    #[doc = "7 Tq"]
    _0110 = 0x06,
    #[doc = "8 Tq"]
    _0111 = 0x07,
    #[doc = "9 Tq"]
    _1000 = 0x08,
    #[doc = "10 Tq"]
    _1001 = 0x09,
    #[doc = "11 Tq"]
    _1010 = 0x0a,
    #[doc = "12 Tq"]
    _1011 = 0x0b,
    #[doc = "13 Tq"]
    _1100 = 0x0c,
    #[doc = "14 Tq"]
    _1101 = 0x0d,
    #[doc = "15 Tq"]
    _1110 = 0x0e,
    #[doc = "16 Tq"]
    _1111 = 0x0f,
}
impl Tseg1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tseg1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tseg1 {
    #[inline(always)]
    fn from(val: u8) -> Tseg1 {
        Tseg1::from_bits(val)
    }
}
impl From<Tseg1> for u8 {
    #[inline(always)]
    fn from(val: Tseg1) -> u8 {
        Tseg1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tseg2 {
    #[doc = "Setting prohibited"]
    _000 = 0x0,
    #[doc = "2 Tq"]
    _001 = 0x01,
    #[doc = "3 Tq"]
    _010 = 0x02,
    #[doc = "4 Tq"]
    _011 = 0x03,
    #[doc = "5 Tq"]
    _100 = 0x04,
    #[doc = "6 Tq"]
    _101 = 0x05,
    #[doc = "7 Tq"]
    _110 = 0x06,
    #[doc = "8 Tq"]
    _111 = 0x07,
}
impl Tseg2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tseg2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tseg2 {
    #[inline(always)]
    fn from(val: u8) -> Tseg2 {
        Tseg2::from_bits(val)
    }
}
impl From<Tseg2> for u8 {
    #[inline(always)]
    fn from(val: Tseg2) -> u8 {
        Tseg2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsps {
    #[doc = "Every bit time"]
    _00 = 0x0,
    #[doc = "Every 2-bit time"]
    _01 = 0x01,
    #[doc = "Every 4-bit time"]
    _10 = 0x02,
    #[doc = "Every 8-bit time"]
    _11 = 0x03,
}
impl Tsps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsps {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsps {
    #[inline(always)]
    fn from(val: u8) -> Tsps {
        Tsps::from_bits(val)
    }
}
impl From<Tsps> for u8 {
    #[inline(always)]
    fn from(val: Tsps) -> u8 {
        Tsps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstm {
    #[doc = "Other than CAN test mode"]
    _00 = 0x0,
    #[doc = "Listen-only mode"]
    _01 = 0x01,
    #[doc = "Self-test mode 0 (external loopback)"]
    _10 = 0x02,
    #[doc = "Self-test mode 1 (internal loopback)"]
    _11 = 0x03,
}
impl Tstm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstm {
    #[inline(always)]
    fn from(val: u8) -> Tstm {
        Tstm::from_bits(val)
    }
}
impl From<Tstm> for u8 {
    #[inline(always)]
    fn from(val: Tstm) -> u8 {
        Tstm::to_bits(val)
    }
}
