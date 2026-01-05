#[doc = "Operational amplifier control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ampc(pub u8);
impl Ampc {
    #[doc = "Operation control of operational amplifier(UNIT0)"]
    #[must_use]
    #[inline(always)]
    pub const fn ampe0(&self) -> super::vals::Ampe0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ampe0::from_bits(val as u8)
    }
    #[doc = "Operation control of operational amplifier(UNIT0)"]
    #[inline(always)]
    pub const fn set_ampe0(&mut self, val: super::vals::Ampe0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Operation control of operational amplifier(UNIT1)"]
    #[must_use]
    #[inline(always)]
    pub const fn ampe1(&self) -> super::vals::Ampe1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ampe1::from_bits(val as u8)
    }
    #[doc = "Operation control of operational amplifier(UNIT1)"]
    #[inline(always)]
    pub const fn set_ampe1(&mut self, val: super::vals::Ampe1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Operation control of operational amplifier(UNIT2)"]
    #[must_use]
    #[inline(always)]
    pub const fn ampe2(&self) -> super::vals::Ampe2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ampe2::from_bits(val as u8)
    }
    #[doc = "Operation control of operational amplifier(UNIT2)"]
    #[inline(always)]
    pub const fn set_ampe2(&mut self, val: super::vals::Ampe2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Operation control of operational amplifier(UNIT3)"]
    #[must_use]
    #[inline(always)]
    pub const fn ampe3(&self) -> super::vals::Ampe3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ampe3::from_bits(val as u8)
    }
    #[doc = "Operation control of operational amplifier(UNIT3)"]
    #[inline(always)]
    pub const fn set_ampe3(&mut self, val: super::vals::Ampe3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
    }
    #[doc = "Operation control of operational amplifier reference current circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn irefe(&self) -> super::vals::Irefe {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Irefe::from_bits(val as u8)
    }
    #[doc = "Operation control of operational amplifier reference current circuit"]
    #[inline(always)]
    pub const fn set_irefe(&mut self, val: super::vals::Irefe) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Ampc {
    #[inline(always)]
    fn default() -> Ampc {
        Ampc(0)
    }
}
impl core::fmt::Debug for Ampc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ampc")
            .field("ampe0", &self.ampe0())
            .field("ampe1", &self.ampe1())
            .field("ampe2", &self.ampe2())
            .field("ampe3", &self.ampe3())
            .field("reserved", &self.reserved())
            .field("irefe", &self.irefe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ampc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ampc {{ ampe0: {:?}, ampe1: {:?}, ampe2: {:?}, ampe3: {:?}, reserved: {=u8:?}, irefe: {:?} }}",
            self.ampe0(),
            self.ampe1(),
            self.ampe2(),
            self.ampe3(),
            self.reserved(),
            self.irefe()
        )
    }
}
#[doc = "Operational amplifier mode control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ampmc(pub u8);
impl Ampmc {
    #[doc = "Operational amplifier precharge control status"]
    #[must_use]
    #[inline(always)]
    pub const fn amppc0(&self) -> super::vals::Amppc0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Amppc0::from_bits(val as u8)
    }
    #[doc = "Operational amplifier precharge control status"]
    #[inline(always)]
    pub const fn set_amppc0(&mut self, val: super::vals::Amppc0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Operational amplifier precharge control status"]
    #[must_use]
    #[inline(always)]
    pub const fn amppc1(&self) -> super::vals::Amppc1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Amppc1::from_bits(val as u8)
    }
    #[doc = "Operational amplifier precharge control status"]
    #[inline(always)]
    pub const fn set_amppc1(&mut self, val: super::vals::Amppc1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Operational amplifier precharge control status"]
    #[must_use]
    #[inline(always)]
    pub const fn amppc2(&self) -> super::vals::Amppc2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Amppc2::from_bits(val as u8)
    }
    #[doc = "Operational amplifier precharge control status"]
    #[inline(always)]
    pub const fn set_amppc2(&mut self, val: super::vals::Amppc2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Operational amplifier precharge control status"]
    #[must_use]
    #[inline(always)]
    pub const fn amppc3(&self) -> super::vals::Amppc3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Amppc3::from_bits(val as u8)
    }
    #[doc = "Operational amplifier precharge control status"]
    #[inline(always)]
    pub const fn set_amppc3(&mut self, val: super::vals::Amppc3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
    }
    #[doc = "Operation mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ampsp(&self) -> super::vals::Ampsp {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ampsp::from_bits(val as u8)
    }
    #[doc = "Operation mode selection"]
    #[inline(always)]
    pub const fn set_ampsp(&mut self, val: super::vals::Ampsp) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Ampmc {
    #[inline(always)]
    fn default() -> Ampmc {
        Ampmc(0)
    }
}
impl core::fmt::Debug for Ampmc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ampmc")
            .field("amppc0", &self.amppc0())
            .field("amppc1", &self.amppc1())
            .field("amppc2", &self.amppc2())
            .field("amppc3", &self.amppc3())
            .field("reserved", &self.reserved())
            .field("ampsp", &self.ampsp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ampmc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ampmc {{ amppc0: {:?}, amppc1: {:?}, amppc2: {:?}, amppc3: {:?}, reserved: {=u8:?}, ampsp: {:?} }}",
            self.amppc0(),
            self.amppc1(),
            self.amppc2(),
            self.amppc3(),
            self.reserved(),
            self.ampsp()
        )
    }
}
#[doc = "Operational amplifier monitor register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ampmon(pub u8);
impl Ampmon {
    #[doc = "Operational amplifier status(UNIT0)"]
    #[must_use]
    #[inline(always)]
    pub const fn ampmon0(&self) -> super::vals::Ampmon0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ampmon0::from_bits(val as u8)
    }
    #[doc = "Operational amplifier status(UNIT0)"]
    #[inline(always)]
    pub const fn set_ampmon0(&mut self, val: super::vals::Ampmon0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Operational amplifier status(UNIT1)"]
    #[must_use]
    #[inline(always)]
    pub const fn ampmon1(&self) -> super::vals::Ampmon1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ampmon1::from_bits(val as u8)
    }
    #[doc = "Operational amplifier status(UNIT1)"]
    #[inline(always)]
    pub const fn set_ampmon1(&mut self, val: super::vals::Ampmon1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Operational amplifier status(UNIT2)"]
    #[must_use]
    #[inline(always)]
    pub const fn ampmon2(&self) -> super::vals::Ampmon2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ampmon2::from_bits(val as u8)
    }
    #[doc = "Operational amplifier status(UNIT2)"]
    #[inline(always)]
    pub const fn set_ampmon2(&mut self, val: super::vals::Ampmon2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Operational amplifier status(UNIT3)"]
    #[must_use]
    #[inline(always)]
    pub const fn ampmon3(&self) -> super::vals::Ampmon3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ampmon3::from_bits(val as u8)
    }
    #[doc = "Operational amplifier status(UNIT3)"]
    #[inline(always)]
    pub const fn set_ampmon3(&mut self, val: super::vals::Ampmon3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "These bits are read as 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Ampmon {
    #[inline(always)]
    fn default() -> Ampmon {
        Ampmon(0)
    }
}
impl core::fmt::Debug for Ampmon {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ampmon")
            .field("ampmon0", &self.ampmon0())
            .field("ampmon1", &self.ampmon1())
            .field("ampmon2", &self.ampmon2())
            .field("ampmon3", &self.ampmon3())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ampmon {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ampmon {{ ampmon0: {:?}, ampmon1: {:?}, ampmon2: {:?}, ampmon3: {:?}, reserved: {=u8:?} }}",
            self.ampmon0(),
            self.ampmon1(),
            self.ampmon2(),
            self.ampmon3(),
            self.reserved()
        )
    }
}
#[doc = "Operational amplifier trigger mode control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amptrm(pub u8);
impl Amptrm {
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[must_use]
    #[inline(always)]
    pub const fn amptrm00(&self) -> super::vals::Amptrm00 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Amptrm00::from_bits(val as u8)
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub const fn set_amptrm00(&mut self, val: super::vals::Amptrm00) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[must_use]
    #[inline(always)]
    pub const fn amptrm01(&self) -> super::vals::Amptrm01 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Amptrm01::from_bits(val as u8)
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub const fn set_amptrm01(&mut self, val: super::vals::Amptrm01) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[must_use]
    #[inline(always)]
    pub const fn amptrm10(&self) -> super::vals::Amptrm10 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Amptrm10::from_bits(val as u8)
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub const fn set_amptrm10(&mut self, val: super::vals::Amptrm10) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[must_use]
    #[inline(always)]
    pub const fn amptrm11(&self) -> super::vals::Amptrm11 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Amptrm11::from_bits(val as u8)
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub const fn set_amptrm11(&mut self, val: super::vals::Amptrm11) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[must_use]
    #[inline(always)]
    pub const fn amptrm20(&self) -> super::vals::Amptrm20 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Amptrm20::from_bits(val as u8)
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub const fn set_amptrm20(&mut self, val: super::vals::Amptrm20) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[must_use]
    #[inline(always)]
    pub const fn amptrm21(&self) -> super::vals::Amptrm21 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Amptrm21::from_bits(val as u8)
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub const fn set_amptrm21(&mut self, val: super::vals::Amptrm21) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[must_use]
    #[inline(always)]
    pub const fn amptrm30(&self) -> super::vals::Amptrm30 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Amptrm30::from_bits(val as u8)
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub const fn set_amptrm30(&mut self, val: super::vals::Amptrm30) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[must_use]
    #[inline(always)]
    pub const fn amptrm31(&self) -> super::vals::Amptrm31 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Amptrm31::from_bits(val as u8)
    }
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub const fn set_amptrm31(&mut self, val: super::vals::Amptrm31) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Amptrm {
    #[inline(always)]
    fn default() -> Amptrm {
        Amptrm(0)
    }
}
impl core::fmt::Debug for Amptrm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Amptrm")
            .field("amptrm00", &self.amptrm00())
            .field("amptrm01", &self.amptrm01())
            .field("amptrm10", &self.amptrm10())
            .field("amptrm11", &self.amptrm11())
            .field("amptrm20", &self.amptrm20())
            .field("amptrm21", &self.amptrm21())
            .field("amptrm30", &self.amptrm30())
            .field("amptrm31", &self.amptrm31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Amptrm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Amptrm {{ amptrm00: {:?}, amptrm01: {:?}, amptrm10: {:?}, amptrm11: {:?}, amptrm20: {:?}, amptrm21: {:?}, amptrm30: {:?}, amptrm31: {:?} }}",
            self.amptrm00(),
            self.amptrm01(),
            self.amptrm10(),
            self.amptrm11(),
            self.amptrm20(),
            self.amptrm21(),
            self.amptrm30(),
            self.amptrm31()
        )
    }
}
#[doc = "Operational Amplifier Activation Trigger Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amptrs(pub u8);
impl Amptrs {
    #[doc = "ELC trigger selection Do not change the value of the AMPTRS register after setting the AMPTRM register."]
    #[must_use]
    #[inline(always)]
    pub const fn amptrs(&self) -> super::vals::Amptrs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Amptrs::from_bits(val as u8)
    }
    #[doc = "ELC trigger selection Do not change the value of the AMPTRS register after setting the AMPTRM register."]
    #[inline(always)]
    pub const fn set_amptrs(&mut self, val: super::vals::Amptrs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
}
impl Default for Amptrs {
    #[inline(always)]
    fn default() -> Amptrs {
        Amptrs(0)
    }
}
impl core::fmt::Debug for Amptrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Amptrs")
            .field("amptrs", &self.amptrs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Amptrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Amptrs {{ amptrs: {:?} }}", self.amptrs())
    }
}
