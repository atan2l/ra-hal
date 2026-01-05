#[doc = "CTSU Channel Enable Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac0(pub u8);
impl Ctsuchac0 {
    #[doc = "CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\] corresponds to TS00 and CTSUCHAC0\\[7\\] corresponds to TS07. but the write value of CTSUCHAC0\\[2\\] should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuchac0(&self) -> super::vals::Ctsuchac0 {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Ctsuchac0::from_bits(val as u8)
    }
    #[doc = "CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\] corresponds to TS00 and CTSUCHAC0\\[7\\] corresponds to TS07. but the write value of CTSUCHAC0\\[2\\] should be 0."]
    #[inline(always)]
    pub const fn set_ctsuchac0(&mut self, val: super::vals::Ctsuchac0) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u8) & 0xff) << 0usize);
    }
}
impl Default for Ctsuchac0 {
    #[inline(always)]
    fn default() -> Ctsuchac0 {
        Ctsuchac0(0)
    }
}
impl core::fmt::Debug for Ctsuchac0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsuchac0")
            .field("ctsuchac0", &self.ctsuchac0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchac0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctsuchac0 {{ ctsuchac0: {:?} }}", self.ctsuchac0())
    }
}
#[doc = "CTSU Channel Enable Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac1(pub u8);
impl Ctsuchac1 {
    #[doc = "CTSU Channel Enable Control 1. 0: Not measurement target 1: Measurement target Note: CTSUCHAC1\\[0\\] corresponds to TS08 and CTSUCHAC1\\[7\\] corresponds to TS15."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuchac1(&self) -> super::vals::Ctsuchac1 {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Ctsuchac1::from_bits(val as u8)
    }
    #[doc = "CTSU Channel Enable Control 1. 0: Not measurement target 1: Measurement target Note: CTSUCHAC1\\[0\\] corresponds to TS08 and CTSUCHAC1\\[7\\] corresponds to TS15."]
    #[inline(always)]
    pub const fn set_ctsuchac1(&mut self, val: super::vals::Ctsuchac1) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u8) & 0xff) << 0usize);
    }
}
impl Default for Ctsuchac1 {
    #[inline(always)]
    fn default() -> Ctsuchac1 {
        Ctsuchac1(0)
    }
}
impl core::fmt::Debug for Ctsuchac1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsuchac1")
            .field("ctsuchac1", &self.ctsuchac1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchac1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctsuchac1 {{ ctsuchac1: {:?} }}", self.ctsuchac1())
    }
}
#[doc = "CTSU Channel Enable Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac2(pub u8);
impl Ctsuchac2 {
    #[doc = "CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\\[0\\] corresponds to TS16 and CTSUCHAC2\\[7\\] corresponds to TS23."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuchac2(&self) -> super::vals::Ctsuchac2 {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Ctsuchac2::from_bits(val as u8)
    }
    #[doc = "CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\\[0\\] corresponds to TS16 and CTSUCHAC2\\[7\\] corresponds to TS23."]
    #[inline(always)]
    pub const fn set_ctsuchac2(&mut self, val: super::vals::Ctsuchac2) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u8) & 0xff) << 0usize);
    }
}
impl Default for Ctsuchac2 {
    #[inline(always)]
    fn default() -> Ctsuchac2 {
        Ctsuchac2(0)
    }
}
impl core::fmt::Debug for Ctsuchac2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsuchac2")
            .field("ctsuchac2", &self.ctsuchac2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchac2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctsuchac2 {{ ctsuchac2: {:?} }}", self.ctsuchac2())
    }
}
#[doc = "CTSU Channel Enable Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac3(pub u8);
impl Ctsuchac3 {
    #[doc = "CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\\[0\\] corresponds to TS24 and CTSUCHAC3\\[7\\] corresponds to TS31."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuchac3(&self) -> super::vals::Ctsuchac3 {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Ctsuchac3::from_bits(val as u8)
    }
    #[doc = "CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\\[0\\] corresponds to TS24 and CTSUCHAC3\\[7\\] corresponds to TS31."]
    #[inline(always)]
    pub const fn set_ctsuchac3(&mut self, val: super::vals::Ctsuchac3) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u8) & 0xff) << 0usize);
    }
}
impl Default for Ctsuchac3 {
    #[inline(always)]
    fn default() -> Ctsuchac3 {
        Ctsuchac3(0)
    }
}
impl core::fmt::Debug for Ctsuchac3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsuchac3")
            .field("ctsuchac3", &self.ctsuchac3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchac3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctsuchac3 {{ ctsuchac3: {:?} }}", self.ctsuchac3())
    }
}
#[doc = "CTSU Channel Enable Control Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac4(pub u8);
impl Ctsuchac4 {
    #[doc = "CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\] corresponds to TS32 and CTSUCHAC4\\[3\\] corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\] should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuchac4(&self) -> super::vals::Ctsuchac4Ctsuchac4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Ctsuchac4Ctsuchac4::from_bits(val as u8)
    }
    #[doc = "CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\] corresponds to TS32 and CTSUCHAC4\\[3\\] corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\] should be 0."]
    #[inline(always)]
    pub const fn set_ctsuchac4(&mut self, val: super::vals::Ctsuchac4Ctsuchac4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Ctsuchac4 {
    #[inline(always)]
    fn default() -> Ctsuchac4 {
        Ctsuchac4(0)
    }
}
impl core::fmt::Debug for Ctsuchac4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsuchac4")
            .field("ctsuchac4", &self.ctsuchac4())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchac4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctsuchac4 {{ ctsuchac4: {:?}, reserved: {=u8:?} }}",
            self.ctsuchac4(),
            self.reserved()
        )
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc0(pub u8);
impl Ctsuchtrc0 {
    #[doc = "CTSU Channel Transmit/Receive Control 0"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuchtrc0(&self) -> super::vals::Ctsuchtrc0 {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Ctsuchtrc0::from_bits(val as u8)
    }
    #[doc = "CTSU Channel Transmit/Receive Control 0"]
    #[inline(always)]
    pub const fn set_ctsuchtrc0(&mut self, val: super::vals::Ctsuchtrc0) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u8) & 0xff) << 0usize);
    }
}
impl Default for Ctsuchtrc0 {
    #[inline(always)]
    fn default() -> Ctsuchtrc0 {
        Ctsuchtrc0(0)
    }
}
impl core::fmt::Debug for Ctsuchtrc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsuchtrc0")
            .field("ctsuchtrc0", &self.ctsuchtrc0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchtrc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctsuchtrc0 {{ ctsuchtrc0: {:?} }}", self.ctsuchtrc0())
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc1(pub u8);
impl Ctsuchtrc1 {
    #[doc = "CTSU Channel Transmit/Receive Control 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuchtrc1(&self) -> super::vals::Ctsuchtrc1 {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Ctsuchtrc1::from_bits(val as u8)
    }
    #[doc = "CTSU Channel Transmit/Receive Control 1"]
    #[inline(always)]
    pub const fn set_ctsuchtrc1(&mut self, val: super::vals::Ctsuchtrc1) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u8) & 0xff) << 0usize);
    }
}
impl Default for Ctsuchtrc1 {
    #[inline(always)]
    fn default() -> Ctsuchtrc1 {
        Ctsuchtrc1(0)
    }
}
impl core::fmt::Debug for Ctsuchtrc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsuchtrc1")
            .field("ctsuchtrc1", &self.ctsuchtrc1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchtrc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctsuchtrc1 {{ ctsuchtrc1: {:?} }}", self.ctsuchtrc1())
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc2(pub u8);
impl Ctsuchtrc2 {
    #[doc = "CTSU Channel Transmit/Receive Control 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuchtrc2(&self) -> super::vals::Ctsuchtrc2 {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Ctsuchtrc2::from_bits(val as u8)
    }
    #[doc = "CTSU Channel Transmit/Receive Control 2"]
    #[inline(always)]
    pub const fn set_ctsuchtrc2(&mut self, val: super::vals::Ctsuchtrc2) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u8) & 0xff) << 0usize);
    }
}
impl Default for Ctsuchtrc2 {
    #[inline(always)]
    fn default() -> Ctsuchtrc2 {
        Ctsuchtrc2(0)
    }
}
impl core::fmt::Debug for Ctsuchtrc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsuchtrc2")
            .field("ctsuchtrc2", &self.ctsuchtrc2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchtrc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctsuchtrc2 {{ ctsuchtrc2: {:?} }}", self.ctsuchtrc2())
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc3(pub u8);
impl Ctsuchtrc3 {
    #[doc = "CTSU Channel Transmit/Receive Control 3"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuchtrc3(&self) -> super::vals::Ctsuchtrc3 {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Ctsuchtrc3::from_bits(val as u8)
    }
    #[doc = "CTSU Channel Transmit/Receive Control 3"]
    #[inline(always)]
    pub const fn set_ctsuchtrc3(&mut self, val: super::vals::Ctsuchtrc3) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u8) & 0xff) << 0usize);
    }
}
impl Default for Ctsuchtrc3 {
    #[inline(always)]
    fn default() -> Ctsuchtrc3 {
        Ctsuchtrc3(0)
    }
}
impl core::fmt::Debug for Ctsuchtrc3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsuchtrc3")
            .field("ctsuchtrc3", &self.ctsuchtrc3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchtrc3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctsuchtrc3 {{ ctsuchtrc3: {:?} }}", self.ctsuchtrc3())
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc4(pub u8);
impl Ctsuchtrc4 {
    #[doc = "CTSU Channel Transmit/Receive Control 4"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuchac4(&self) -> super::vals::Ctsuchtrc4Ctsuchac4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Ctsuchtrc4Ctsuchac4::from_bits(val as u8)
    }
    #[doc = "CTSU Channel Transmit/Receive Control 4"]
    #[inline(always)]
    pub const fn set_ctsuchac4(&mut self, val: super::vals::Ctsuchtrc4Ctsuchac4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Ctsuchtrc4 {
    #[inline(always)]
    fn default() -> Ctsuchtrc4 {
        Ctsuchtrc4(0)
    }
}
impl core::fmt::Debug for Ctsuchtrc4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsuchtrc4")
            .field("ctsuchac4", &self.ctsuchac4())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuchtrc4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctsuchtrc4 {{ ctsuchac4: {:?}, reserved: {=u8:?} }}",
            self.ctsuchac4(),
            self.reserved()
        )
    }
}
#[doc = "CTSU Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucr0(pub u8);
impl Ctsucr0 {
    #[doc = "CTSU Measurement Operation Start"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsustrt(&self) -> super::vals::Ctsustrt {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ctsustrt::from_bits(val as u8)
    }
    #[doc = "CTSU Measurement Operation Start"]
    #[inline(always)]
    pub const fn set_ctsustrt(&mut self, val: super::vals::Ctsustrt) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "CTSU Measurement Operation Start Trigger Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsucap(&self) -> super::vals::Ctsucap {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ctsucap::from_bits(val as u8)
    }
    #[doc = "CTSU Measurement Operation Start Trigger Select"]
    #[inline(always)]
    pub const fn set_ctsucap(&mut self, val: super::vals::Ctsucap) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "CTSU Wait State Power-Saving Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsusnz(&self) -> super::vals::Ctsusnz {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ctsusnz::from_bits(val as u8)
    }
    #[doc = "CTSU Wait State Power-Saving Enable"]
    #[inline(always)]
    pub const fn set_ctsusnz(&mut self, val: super::vals::Ctsusnz) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "CTSU Control Block Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuinit(&self) -> super::vals::Ctsuinit {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ctsuinit::from_bits(val as u8)
    }
    #[doc = "CTSU Control Block Initialization"]
    #[inline(always)]
    pub const fn set_ctsuinit(&mut self, val: super::vals::Ctsuinit) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
}
impl Default for Ctsucr0 {
    #[inline(always)]
    fn default() -> Ctsucr0 {
        Ctsucr0(0)
    }
}
impl core::fmt::Debug for Ctsucr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsucr0")
            .field("ctsustrt", &self.ctsustrt())
            .field("ctsucap", &self.ctsucap())
            .field("ctsusnz", &self.ctsusnz())
            .field("reserved", &self.reserved())
            .field("ctsuinit", &self.ctsuinit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsucr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctsucr0 {{ ctsustrt: {:?}, ctsucap: {:?}, ctsusnz: {:?}, reserved: {=bool:?}, ctsuinit: {:?} }}",
            self.ctsustrt(),
            self.ctsucap(),
            self.ctsusnz(),
            self.reserved(),
            self.ctsuinit()
        )
    }
}
#[doc = "CTSU Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucr1(pub u8);
impl Ctsucr1 {
    #[doc = "CTSU Power Supply Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsupon(&self) -> super::vals::Ctsupon {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ctsupon::from_bits(val as u8)
    }
    #[doc = "CTSU Power Supply Enable"]
    #[inline(always)]
    pub const fn set_ctsupon(&mut self, val: super::vals::Ctsupon) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "CTSU LPF Capacitance Charging Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsucsw(&self) -> super::vals::Ctsucsw {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ctsucsw::from_bits(val as u8)
    }
    #[doc = "CTSU LPF Capacitance Charging Control"]
    #[inline(always)]
    pub const fn set_ctsucsw(&mut self, val: super::vals::Ctsucsw) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "CTSU Power Supply Operating Mode Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuatune0(&self) -> super::vals::Ctsuatune0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ctsuatune0::from_bits(val as u8)
    }
    #[doc = "CTSU Power Supply Operating Mode Setting"]
    #[inline(always)]
    pub const fn set_ctsuatune0(&mut self, val: super::vals::Ctsuatune0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "CTSU Power Supply Capacity Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuatune1(&self) -> super::vals::Ctsuatune1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ctsuatune1::from_bits(val as u8)
    }
    #[doc = "CTSU Power Supply Capacity Adjustment"]
    #[inline(always)]
    pub const fn set_ctsuatune1(&mut self, val: super::vals::Ctsuatune1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "CTSU Operating Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuclk(&self) -> super::vals::Ctsuclk {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ctsuclk::from_bits(val as u8)
    }
    #[doc = "CTSU Operating Clock Select"]
    #[inline(always)]
    pub const fn set_ctsuclk(&mut self, val: super::vals::Ctsuclk) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
    }
    #[doc = "CTSU Measurement Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsumd(&self) -> super::vals::Ctsumd {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Ctsumd::from_bits(val as u8)
    }
    #[doc = "CTSU Measurement Mode Select"]
    #[inline(always)]
    pub const fn set_ctsumd(&mut self, val: super::vals::Ctsumd) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u8) & 0x03) << 6usize);
    }
}
impl Default for Ctsucr1 {
    #[inline(always)]
    fn default() -> Ctsucr1 {
        Ctsucr1(0)
    }
}
impl core::fmt::Debug for Ctsucr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsucr1")
            .field("ctsupon", &self.ctsupon())
            .field("ctsucsw", &self.ctsucsw())
            .field("ctsuatune0", &self.ctsuatune0())
            .field("ctsuatune1", &self.ctsuatune1())
            .field("ctsuclk", &self.ctsuclk())
            .field("ctsumd", &self.ctsumd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsucr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctsucr1 {{ ctsupon: {:?}, ctsucsw: {:?}, ctsuatune0: {:?}, ctsuatune1: {:?}, ctsuclk: {:?}, ctsumd: {:?} }}",
            self.ctsupon(),
            self.ctsucsw(),
            self.ctsuatune0(),
            self.ctsuatune1(),
            self.ctsuclk(),
            self.ctsumd()
        )
    }
}
#[doc = "CTSU High-Pass Noise Reduction Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsudclkc(pub u8);
impl Ctsudclkc {
    #[doc = "CTSU Diffusion Clock Mode Select NOTE: This bit should be set to 00b."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsussmod(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "CTSU Diffusion Clock Mode Select NOTE: This bit should be set to 00b."]
    #[inline(always)]
    pub const fn set_ctsussmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
    }
    #[doc = "CTSU Diffusion Clock Mode Control NOTE: This bit should be set to 11b."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsusscnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "CTSU Diffusion Clock Mode Control NOTE: This bit should be set to 11b."]
    #[inline(always)]
    pub const fn set_ctsusscnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
    }
}
impl Default for Ctsudclkc {
    #[inline(always)]
    fn default() -> Ctsudclkc {
        Ctsudclkc(0)
    }
}
impl core::fmt::Debug for Ctsudclkc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsudclkc")
            .field("ctsussmod", &self.ctsussmod())
            .field("reserved", &self.reserved())
            .field("ctsusscnt", &self.ctsusscnt())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsudclkc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctsudclkc {{ ctsussmod: {=u8:?}, reserved: {=u8:?}, ctsusscnt: {=u8:?}, reserved_2: {=u8:?} }}",
            self.ctsussmod(),
            self.reserved(),
            self.ctsusscnt(),
            self.reserved_2()
        )
    }
}
#[doc = "CTSU Error Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuerrs(pub u16);
impl Ctsuerrs {
    #[doc = "These bits are read as 000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "These bits are read as 000000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "TSCAP Voltage Error Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuicomp(&self) -> super::vals::Ctsuicomp {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Ctsuicomp::from_bits(val as u8)
    }
    #[doc = "TSCAP Voltage Error Monitor"]
    #[inline(always)]
    pub const fn set_ctsuicomp(&mut self, val: super::vals::Ctsuicomp) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Ctsuerrs {
    #[inline(always)]
    fn default() -> Ctsuerrs {
        Ctsuerrs(0)
    }
}
impl core::fmt::Debug for Ctsuerrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsuerrs")
            .field("reserved", &self.reserved())
            .field("ctsuicomp", &self.ctsuicomp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuerrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctsuerrs {{ reserved: {=u16:?}, ctsuicomp: {:?} }}",
            self.reserved(),
            self.ctsuicomp()
        )
    }
}
#[doc = "CTSU Measurement Channel Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumch0(pub u8);
impl Ctsumch0 {
    #[doc = "CTSU Measurement Channel 0. Note1: Writing to these bits is only enabled in self-capacitance single-scan mode (CTSUCR1.CTSUMD\\[1:0\\] bits = 00b). Note2: If the value of CTSUMCH0 was set to b'111111 in mode other than self-capacitor single scan mode, the measurement is stopped."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsumch0(&self) -> super::vals::Ctsumch0 {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Ctsumch0::from_bits(val as u8)
    }
    #[doc = "CTSU Measurement Channel 0. Note1: Writing to these bits is only enabled in self-capacitance single-scan mode (CTSUCR1.CTSUMD\\[1:0\\] bits = 00b). Note2: If the value of CTSUMCH0 was set to b'111111 in mode other than self-capacitor single scan mode, the measurement is stopped."]
    #[inline(always)]
    pub const fn set_ctsumch0(&mut self, val: super::vals::Ctsumch0) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u8) & 0x3f) << 0usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
    }
}
impl Default for Ctsumch0 {
    #[inline(always)]
    fn default() -> Ctsumch0 {
        Ctsumch0(0)
    }
}
impl core::fmt::Debug for Ctsumch0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsumch0")
            .field("ctsumch0", &self.ctsumch0())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsumch0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctsumch0 {{ ctsumch0: {:?}, reserved: {=u8:?} }}",
            self.ctsumch0(),
            self.reserved()
        )
    }
}
#[doc = "CTSU Measurement Channel Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumch1(pub u8);
impl Ctsumch1 {
    #[doc = "CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'111111, the measurement is stopped."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsumch1(&self) -> super::vals::Ctsumch1 {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Ctsumch1::from_bits(val as u8)
    }
    #[doc = "CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'111111, the measurement is stopped."]
    #[inline(always)]
    pub const fn set_ctsumch1(&mut self, val: super::vals::Ctsumch1) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u8) & 0x3f) << 0usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
    }
}
impl Default for Ctsumch1 {
    #[inline(always)]
    fn default() -> Ctsumch1 {
        Ctsumch1(0)
    }
}
impl core::fmt::Debug for Ctsumch1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsumch1")
            .field("ctsumch1", &self.ctsumch1())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsumch1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctsumch1 {{ ctsumch1: {:?}, reserved: {=u8:?} }}",
            self.ctsumch1(),
            self.reserved()
        )
    }
}
#[doc = "CTSU Reference Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsurc(pub u16);
impl Ctsurc {
    #[doc = "CTSU Reference Counter These bits indicate the measurement result of the reference ICO. These bits indicate FFFFh when an overflow occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsurc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CTSU Reference Counter These bits indicate the measurement result of the reference ICO. These bits indicate FFFFh when an overflow occurs."]
    #[inline(always)]
    pub const fn set_ctsurc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Ctsurc {
    #[inline(always)]
    fn default() -> Ctsurc {
        Ctsurc(0)
    }
}
impl core::fmt::Debug for Ctsurc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsurc")
            .field("ctsurc", &self.ctsurc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsurc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctsurc {{ ctsurc: {=u16:?} }}", self.ctsurc())
    }
}
#[doc = "CTSU Sensor Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusc(pub u16);
impl Ctsusc {
    #[doc = "CTSU Sensor Counter These bits indicate the measurement result of the CTSU. These bits indicate FFFFh when an overflow occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsusc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CTSU Sensor Counter These bits indicate the measurement result of the CTSU. These bits indicate FFFFh when an overflow occurs."]
    #[inline(always)]
    pub const fn set_ctsusc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Ctsusc {
    #[inline(always)]
    fn default() -> Ctsusc {
        Ctsusc(0)
    }
}
impl core::fmt::Debug for Ctsusc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsusc")
            .field("ctsusc", &self.ctsusc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsusc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctsusc {{ ctsusc: {=u16:?} }}", self.ctsusc())
    }
}
#[doc = "CTSU Synchronous Noise Reduction Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusdprs(pub u8);
impl Ctsusdprs {
    #[doc = "CTSU Measurement Time and Pulse Count Adjustment Recommended setting: 3 (0011b)"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuprratio(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CTSU Measurement Time and Pulse Count Adjustment Recommended setting: 3 (0011b)"]
    #[inline(always)]
    pub const fn set_ctsuprratio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "CTSU Base Period and Pulse Count Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuprmode(&self) -> super::vals::Ctsuprmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ctsuprmode::from_bits(val as u8)
    }
    #[doc = "CTSU Base Period and Pulse Count Setting"]
    #[inline(always)]
    pub const fn set_ctsuprmode(&mut self, val: super::vals::Ctsuprmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
    }
    #[doc = "CTSU High-Pass Noise Reduction Function Off Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsusoff(&self) -> super::vals::Ctsusoff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ctsusoff::from_bits(val as u8)
    }
    #[doc = "CTSU High-Pass Noise Reduction Function Off Setting"]
    #[inline(always)]
    pub const fn set_ctsusoff(&mut self, val: super::vals::Ctsusoff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Ctsusdprs {
    #[inline(always)]
    fn default() -> Ctsusdprs {
        Ctsusdprs(0)
    }
}
impl core::fmt::Debug for Ctsusdprs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsusdprs")
            .field("ctsuprratio", &self.ctsuprratio())
            .field("ctsuprmode", &self.ctsuprmode())
            .field("ctsusoff", &self.ctsusoff())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsusdprs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctsusdprs {{ ctsuprratio: {=u8:?}, ctsuprmode: {:?}, ctsusoff: {:?}, reserved: {=bool:?} }}",
            self.ctsuprratio(),
            self.ctsuprmode(),
            self.ctsusoff(),
            self.reserved()
        )
    }
}
#[doc = "CTSU Sensor Offset Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuso0(pub u16);
impl Ctsuso0 {
    #[doc = "CTSU Sensor Offset Adjustment Current offset amount is CTSUSO ( 0 to 1023 )"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuso(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "CTSU Sensor Offset Adjustment Current offset amount is CTSUSO ( 0 to 1023 )"]
    #[inline(always)]
    pub const fn set_ctsuso(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u16) & 0x03ff) << 0usize);
    }
    #[doc = "CTSU Measurement Count Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsusnum(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "CTSU Measurement Count Setting"]
    #[inline(always)]
    pub const fn set_ctsusnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Ctsuso0 {
    #[inline(always)]
    fn default() -> Ctsuso0 {
        Ctsuso0(0)
    }
}
impl core::fmt::Debug for Ctsuso0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsuso0")
            .field("ctsuso", &self.ctsuso())
            .field("ctsusnum", &self.ctsusnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuso0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctsuso0 {{ ctsuso: {=u16:?}, ctsusnum: {=u8:?} }}",
            self.ctsuso(),
            self.ctsusnum()
        )
    }
}
#[doc = "CTSU Sensor Offset Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuso1(pub u16);
impl Ctsuso1 {
    #[doc = "CTSU Reference ICO Current Adjustment Current offset amount is CTSUSO ( 0 to 255 )"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuricoa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "CTSU Reference ICO Current Adjustment Current offset amount is CTSUSO ( 0 to 255 )"]
    #[inline(always)]
    pub const fn set_ctsuricoa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "CTSU Base Clock Setting Operating clock divided by ( CTSUSDPA + 1 ) x 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsusdpa(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "CTSU Base Clock Setting Operating clock divided by ( CTSUSDPA + 1 ) x 2"]
    #[inline(always)]
    pub const fn set_ctsusdpa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
    }
    #[doc = "CTSU ICO Gain Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsuicog(&self) -> super::vals::Ctsuicog {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Ctsuicog::from_bits(val as u8)
    }
    #[doc = "CTSU ICO Gain Adjustment"]
    #[inline(always)]
    pub const fn set_ctsuicog(&mut self, val: super::vals::Ctsuicog) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u16) & 0x03) << 13usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Ctsuso1 {
    #[inline(always)]
    fn default() -> Ctsuso1 {
        Ctsuso1(0)
    }
}
impl core::fmt::Debug for Ctsuso1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsuso1")
            .field("ctsuricoa", &self.ctsuricoa())
            .field("ctsusdpa", &self.ctsusdpa())
            .field("ctsuicog", &self.ctsuicog())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsuso1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctsuso1 {{ ctsuricoa: {=u8:?}, ctsusdpa: {=u8:?}, ctsuicog: {:?}, reserved: {=bool:?} }}",
            self.ctsuricoa(),
            self.ctsusdpa(),
            self.ctsuicog(),
            self.reserved()
        )
    }
}
#[doc = "CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsussc(pub u16);
impl Ctsussc {
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "CTSU Spectrum Diffusion Frequency Division Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsussdiv(&self) -> super::vals::Ctsussdiv {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Ctsussdiv::from_bits(val as u8)
    }
    #[doc = "CTSU Spectrum Diffusion Frequency Division Setting"]
    #[inline(always)]
    pub const fn set_ctsussdiv(&mut self, val: super::vals::Ctsussdiv) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
    }
}
impl Default for Ctsussc {
    #[inline(always)]
    fn default() -> Ctsussc {
        Ctsussc(0)
    }
}
impl core::fmt::Debug for Ctsussc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsussc")
            .field("reserved", &self.reserved())
            .field("ctsussdiv", &self.ctsussdiv())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsussc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctsussc {{ reserved: {=u8:?}, ctsussdiv: {:?}, reserved_2: {=u8:?} }}",
            self.reserved(),
            self.ctsussdiv(),
            self.reserved_2()
        )
    }
}
#[doc = "CTSU Sensor Stabilization Wait Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusst(pub u8);
impl Ctsusst {
    #[doc = "CTSU Sensor Stabilization Wait Control NOTE: The value of these bits should be fixed to 00010000b."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsusst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "CTSU Sensor Stabilization Wait Control NOTE: The value of these bits should be fixed to 00010000b."]
    #[inline(always)]
    pub const fn set_ctsusst(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Ctsusst {
    #[inline(always)]
    fn default() -> Ctsusst {
        Ctsusst(0)
    }
}
impl core::fmt::Debug for Ctsusst {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsusst")
            .field("ctsusst", &self.ctsusst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsusst {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctsusst {{ ctsusst: {=u8:?} }}", self.ctsusst())
    }
}
#[doc = "CTSU Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsust(pub u8);
impl Ctsust {
    #[doc = "CTSU Measurement Status Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsustc(&self) -> super::vals::Ctsustc {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctsustc::from_bits(val as u8)
    }
    #[doc = "CTSU Measurement Status Counter"]
    #[inline(always)]
    pub const fn set_ctsustc(&mut self, val: super::vals::Ctsustc) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u8) & 0x07) << 0usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "CTSU Data Transfer Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsudtsr(&self) -> super::vals::Ctsudtsr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ctsudtsr::from_bits(val as u8)
    }
    #[doc = "CTSU Data Transfer Status Flag"]
    #[inline(always)]
    pub const fn set_ctsudtsr(&mut self, val: super::vals::Ctsudtsr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "CTSU Sensor Counter Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsusovf(&self) -> super::vals::Ctsusovf {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ctsusovf::from_bits(val as u8)
    }
    #[doc = "CTSU Sensor Counter Overflow Flag"]
    #[inline(always)]
    pub const fn set_ctsusovf(&mut self, val: super::vals::Ctsusovf) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "CTSU Reference Counter Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsurovf(&self) -> super::vals::Ctsurovf {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ctsurovf::from_bits(val as u8)
    }
    #[doc = "CTSU Reference Counter Overflow Flag"]
    #[inline(always)]
    pub const fn set_ctsurovf(&mut self, val: super::vals::Ctsurovf) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "CTSU Mutual Capacitance Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsups(&self) -> super::vals::Ctsups {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ctsups::from_bits(val as u8)
    }
    #[doc = "CTSU Mutual Capacitance Status Flag"]
    #[inline(always)]
    pub const fn set_ctsups(&mut self, val: super::vals::Ctsups) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Ctsust {
    #[inline(always)]
    fn default() -> Ctsust {
        Ctsust(0)
    }
}
impl core::fmt::Debug for Ctsust {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctsust")
            .field("ctsustc", &self.ctsustc())
            .field("reserved", &self.reserved())
            .field("ctsudtsr", &self.ctsudtsr())
            .field("ctsusovf", &self.ctsusovf())
            .field("ctsurovf", &self.ctsurovf())
            .field("ctsups", &self.ctsups())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctsust {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctsust {{ ctsustc: {:?}, reserved: {=bool:?}, ctsudtsr: {:?}, ctsusovf: {:?}, ctsurovf: {:?}, ctsups: {:?} }}",
            self.ctsustc(),
            self.reserved(),
            self.ctsudtsr(),
            self.ctsusovf(),
            self.ctsurovf(),
            self.ctsups()
        )
    }
}
