#[doc = "Bus Error Address Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserradd(pub u32);
impl Buserradd {
    #[doc = "Bus Error Address When a bus error occurs, It stores an error address."]
    #[must_use]
    #[inline(always)]
    pub const fn berad(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Bus Error Address When a bus error occurs, It stores an error address."]
    #[inline(always)]
    pub const fn set_berad(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Buserradd {
    #[inline(always)]
    fn default() -> Buserradd {
        Buserradd(0)
    }
}
impl core::fmt::Debug for Buserradd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buserradd")
            .field("berad", &self.berad())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buserradd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Buserradd {{ berad: {=u32:?} }}", self.berad())
    }
}
#[doc = "Bus Error Status Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstat(pub u8);
impl Buserrstat {
    #[doc = "Error Access Status The status at the time of the error"]
    #[must_use]
    #[inline(always)]
    pub const fn accstat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Error Access Status The status at the time of the error"]
    #[inline(always)]
    pub const fn set_accstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u8) & 0x3f) << 1usize);
    }
    #[doc = "Bus Error Status When bus error assert, error flag occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn errstat(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Error Status When bus error assert, error flag occurs."]
    #[inline(always)]
    pub const fn set_errstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Buserrstat {
    #[inline(always)]
    fn default() -> Buserrstat {
        Buserrstat(0)
    }
}
impl core::fmt::Debug for Buserrstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buserrstat")
            .field("accstat", &self.accstat())
            .field("reserved", &self.reserved())
            .field("errstat", &self.errstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buserrstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buserrstat {{ accstat: {=bool:?}, reserved: {=u8:?}, errstat: {=bool:?} }}",
            self.accstat(),
            self.reserved(),
            self.errstat()
        )
    }
}
#[doc = "Master Bus Control Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busmcnt(pub u16);
impl Busmcnt {
    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Ignore Error Responses"]
    #[must_use]
    #[inline(always)]
    pub const fn ieres(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore Error Responses"]
    #[inline(always)]
    pub const fn set_ieres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Busmcnt {
    #[inline(always)]
    fn default() -> Busmcnt {
        Busmcnt(0)
    }
}
impl core::fmt::Debug for Busmcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busmcnt")
            .field("reserved", &self.reserved())
            .field("ieres", &self.ieres())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busmcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Busmcnt {{ reserved: {=u16:?}, ieres: {=bool:?} }}",
            self.reserved(),
            self.ieres()
        )
    }
}
#[doc = "Slave Bus Control Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscnt(pub u16);
impl Busscnt {
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[must_use]
    #[inline(always)]
    pub const fn arbmet(&self) -> super::vals::BusscntArbmet {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::BusscntArbmet::from_bits(val as u8)
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub const fn set_arbmet(&mut self, val: super::vals::BusscntArbmet) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x03ff;
        val as u16
    }
    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u16) & 0x03ff) << 6usize);
    }
}
impl Default for Busscnt {
    #[inline(always)]
    fn default() -> Busscnt {
        Busscnt(0)
    }
}
impl core::fmt::Debug for Busscnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busscnt")
            .field("reserved", &self.reserved())
            .field("arbmet", &self.arbmet())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busscnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Busscnt {{ reserved: {=u8:?}, arbmet: {:?}, reserved_2: {=u16:?} }}",
            self.reserved(),
            self.arbmet(),
            self.reserved_2()
        )
    }
}
#[doc = "Slave Bus Control Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscnt2(pub u16);
impl Busscnt2 {
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[must_use]
    #[inline(always)]
    pub const fn arbmet(&self) -> super::vals::Busscnt2Arbmet {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Busscnt2Arbmet::from_bits(val as u8)
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub const fn set_arbmet(&mut self, val: super::vals::Busscnt2Arbmet) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x03ff;
        val as u16
    }
    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u16) & 0x03ff) << 6usize);
    }
}
impl Default for Busscnt2 {
    #[inline(always)]
    fn default() -> Busscnt2 {
        Busscnt2(0)
    }
}
impl core::fmt::Debug for Busscnt2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busscnt2")
            .field("reserved", &self.reserved())
            .field("arbmet", &self.arbmet())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busscnt2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Busscnt2 {{ reserved: {=u8:?}, arbmet: {:?}, reserved_2: {=u16:?} }}",
            self.reserved(),
            self.arbmet(),
            self.reserved_2()
        )
    }
}
#[doc = "Slave Bus Control Register FBU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntfbu(pub u16);
impl Busscntfbu {
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[must_use]
    #[inline(always)]
    pub const fn arbmet(&self) -> super::vals::BusscntfbuArbmet {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::BusscntfbuArbmet::from_bits(val as u8)
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub const fn set_arbmet(&mut self, val: super::vals::BusscntfbuArbmet) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x03ff;
        val as u16
    }
    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u16) & 0x03ff) << 6usize);
    }
}
impl Default for Busscntfbu {
    #[inline(always)]
    fn default() -> Busscntfbu {
        Busscntfbu(0)
    }
}
impl core::fmt::Debug for Busscntfbu {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busscntfbu")
            .field("reserved", &self.reserved())
            .field("arbmet", &self.arbmet())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busscntfbu {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Busscntfbu {{ reserved: {=u8:?}, arbmet: {:?}, reserved_2: {=u16:?} }}",
            self.reserved(),
            self.arbmet(),
            self.reserved_2()
        )
    }
}
#[doc = "Slave Bus Control Register FLI"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntfli(pub u16);
impl Busscntfli {
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[must_use]
    #[inline(always)]
    pub const fn arbmet(&self) -> super::vals::BusscntfliArbmet {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::BusscntfliArbmet::from_bits(val as u8)
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub const fn set_arbmet(&mut self, val: super::vals::BusscntfliArbmet) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x03ff;
        val as u16
    }
    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u16) & 0x03ff) << 6usize);
    }
}
impl Default for Busscntfli {
    #[inline(always)]
    fn default() -> Busscntfli {
        Busscntfli(0)
    }
}
impl core::fmt::Debug for Busscntfli {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busscntfli")
            .field("reserved", &self.reserved())
            .field("arbmet", &self.arbmet())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busscntfli {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Busscntfli {{ reserved: {=u8:?}, arbmet: {:?}, reserved_2: {=u16:?} }}",
            self.reserved(),
            self.arbmet(),
            self.reserved_2()
        )
    }
}
#[doc = "Slave Bus Control Register P6B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntp6b(pub u16);
impl Busscntp6b {
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[must_use]
    #[inline(always)]
    pub const fn arbmet(&self) -> super::vals::Busscntp6bArbmet {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Busscntp6bArbmet::from_bits(val as u8)
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub const fn set_arbmet(&mut self, val: super::vals::Busscntp6bArbmet) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x03ff;
        val as u16
    }
    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u16) & 0x03ff) << 6usize);
    }
}
impl Default for Busscntp6b {
    #[inline(always)]
    fn default() -> Busscntp6b {
        Busscntp6b(0)
    }
}
impl core::fmt::Debug for Busscntp6b {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busscntp6b")
            .field("reserved", &self.reserved())
            .field("arbmet", &self.arbmet())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busscntp6b {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Busscntp6b {{ reserved: {=u8:?}, arbmet: {:?}, reserved_2: {=u16:?} }}",
            self.reserved(),
            self.arbmet(),
            self.reserved_2()
        )
    }
}
