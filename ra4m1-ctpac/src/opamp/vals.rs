#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amptrs {
    #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 1.Operational amplifier 2: Operational amplifier An activation trigger 2.Operational amplifier 3: Operational amplifier An activation trigger 3"]
    _00 = 0x0,
    #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 1.Operational amplifier 3: Operational amplifier An activation trigger 1"]
    _01 = 0x01,
    #[doc = "Setting prohibited"]
    _10 = 0x02,
    #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 0.Operational amplifier 3: Operational amplifier An activation trigger 0"]
    _11 = 0x03,
}
impl Amptrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amptrs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amptrs {
    #[inline(always)]
    fn from(val: u8) -> Amptrs {
        Amptrs::from_bits(val)
    }
}
impl From<Amptrs> for u8 {
    #[inline(always)]
    fn from(val: Amptrs) -> u8 {
        Amptrs::to_bits(val)
    }
}
