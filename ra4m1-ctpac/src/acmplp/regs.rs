#[doc = "ACMPLP Filter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compfir(pub u8);
impl Compfir {
    #[doc = "ACMPLP0 Filter Select"]
    #[must_use]
    #[inline(always)]
    pub const fn c0fck(&self) -> super::vals::C0fck {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::C0fck::from_bits(val as u8)
    }
    #[doc = "ACMPLP0 Filter Select"]
    #[inline(always)]
    pub const fn set_c0fck(&mut self, val: super::vals::C0fck) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "ACMPLP0 Edge Polarity Switching"]
    #[must_use]
    #[inline(always)]
    pub const fn c0epo(&self) -> super::vals::C0epo {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::C0epo::from_bits(val as u8)
    }
    #[doc = "ACMPLP0 Edge Polarity Switching"]
    #[inline(always)]
    pub const fn set_c0epo(&mut self, val: super::vals::C0epo) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "ACMPLP0 Edge Detection Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn c0edg(&self) -> super::vals::C0edg {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::C0edg::from_bits(val as u8)
    }
    #[doc = "ACMPLP0 Edge Detection Selection"]
    #[inline(always)]
    pub const fn set_c0edg(&mut self, val: super::vals::C0edg) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "ACMPLP1 Filter Select"]
    #[must_use]
    #[inline(always)]
    pub const fn c1fck(&self) -> super::vals::C1fck {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::C1fck::from_bits(val as u8)
    }
    #[doc = "ACMPLP1 Filter Select"]
    #[inline(always)]
    pub const fn set_c1fck(&mut self, val: super::vals::C1fck) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
    }
    #[doc = "ACMPLP1 Edge Polarity Switching"]
    #[must_use]
    #[inline(always)]
    pub const fn c1epo(&self) -> super::vals::C1epo {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::C1epo::from_bits(val as u8)
    }
    #[doc = "ACMPLP1 Edge Polarity Switching"]
    #[inline(always)]
    pub const fn set_c1epo(&mut self, val: super::vals::C1epo) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "ACMPLP1 Edge Detection Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn c1edg(&self) -> super::vals::C1edg {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::C1edg::from_bits(val as u8)
    }
    #[doc = "ACMPLP1 Edge Detection Selection"]
    #[inline(always)]
    pub const fn set_c1edg(&mut self, val: super::vals::C1edg) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Compfir {
    #[inline(always)]
    fn default() -> Compfir {
        Compfir(0)
    }
}
impl core::fmt::Debug for Compfir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Compfir")
            .field("c0fck", &self.c0fck())
            .field("c0epo", &self.c0epo())
            .field("c0edg", &self.c0edg())
            .field("c1fck", &self.c1fck())
            .field("c1epo", &self.c1epo())
            .field("c1edg", &self.c1edg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Compfir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Compfir {{ c0fck: {:?}, c0epo: {:?}, c0edg: {:?}, c1fck: {:?}, c1epo: {:?}, c1edg: {:?} }}",
            self.c0fck(),
            self.c0epo(),
            self.c0edg(),
            self.c1fck(),
            self.c1epo(),
            self.c1edg()
        )
    }
}
#[doc = "ACMPLP Mode Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compmdr(pub u8);
impl Compmdr {
    #[doc = "ACMPLP0 Operation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn c0enb(&self) -> super::vals::C0enb {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::C0enb::from_bits(val as u8)
    }
    #[doc = "ACMPLP0 Operation Enable"]
    #[inline(always)]
    pub const fn set_c0enb(&mut self, val: super::vals::C0enb) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "ACMPLP0 Window Function Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn c0wde(&self) -> super::vals::C0wde {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::C0wde::from_bits(val as u8)
    }
    #[doc = "ACMPLP0 Window Function Mode Enable"]
    #[inline(always)]
    pub const fn set_c0wde(&mut self, val: super::vals::C0wde) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "ACMPLP0 Reference Voltage Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn c0vrf(&self) -> super::vals::C0vrf {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::C0vrf::from_bits(val as u8)
    }
    #[doc = "ACMPLP0 Reference Voltage Selection"]
    #[inline(always)]
    pub const fn set_c0vrf(&mut self, val: super::vals::C0vrf) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "ACMPLP0 Monitor Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn c0mon(&self) -> super::vals::C0mon {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::C0mon::from_bits(val as u8)
    }
    #[doc = "ACMPLP0 Monitor Flag"]
    #[inline(always)]
    pub const fn set_c0mon(&mut self, val: super::vals::C0mon) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "ACMPLP1 Operation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn c1enb(&self) -> super::vals::C1enb {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::C1enb::from_bits(val as u8)
    }
    #[doc = "ACMPLP1 Operation Enable"]
    #[inline(always)]
    pub const fn set_c1enb(&mut self, val: super::vals::C1enb) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "ACMPLP1 Window Function Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn c1wde(&self) -> super::vals::C1wde {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::C1wde::from_bits(val as u8)
    }
    #[doc = "ACMPLP1 Window Function Mode Enable"]
    #[inline(always)]
    pub const fn set_c1wde(&mut self, val: super::vals::C1wde) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "ACMPLP1 Reference Voltage Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn c1vrf(&self) -> super::vals::C1vrf {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::C1vrf::from_bits(val as u8)
    }
    #[doc = "ACMPLP1 Reference Voltage Selection"]
    #[inline(always)]
    pub const fn set_c1vrf(&mut self, val: super::vals::C1vrf) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "ACMPLP1 Monitor Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn c1mon(&self) -> super::vals::C1mon {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::C1mon::from_bits(val as u8)
    }
    #[doc = "ACMPLP1 Monitor Flag"]
    #[inline(always)]
    pub const fn set_c1mon(&mut self, val: super::vals::C1mon) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Compmdr {
    #[inline(always)]
    fn default() -> Compmdr {
        Compmdr(0)
    }
}
impl core::fmt::Debug for Compmdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Compmdr")
            .field("c0enb", &self.c0enb())
            .field("c0wde", &self.c0wde())
            .field("c0vrf", &self.c0vrf())
            .field("c0mon", &self.c0mon())
            .field("c1enb", &self.c1enb())
            .field("c1wde", &self.c1wde())
            .field("c1vrf", &self.c1vrf())
            .field("c1mon", &self.c1mon())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Compmdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Compmdr {{ c0enb: {:?}, c0wde: {:?}, c0vrf: {:?}, c0mon: {:?}, c1enb: {:?}, c1wde: {:?}, c1vrf: {:?}, c1mon: {:?} }}",
            self.c0enb(),
            self.c0wde(),
            self.c0vrf(),
            self.c0mon(),
            self.c1enb(),
            self.c1wde(),
            self.c1vrf(),
            self.c1mon()
        )
    }
}
#[doc = "ACMPLP Output Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compocr(pub u8);
impl Compocr {
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "ACMPLP0 VCOUT Pin Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn c0oe(&self) -> super::vals::C0oe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::C0oe::from_bits(val as u8)
    }
    #[doc = "ACMPLP0 VCOUT Pin Output Enable"]
    #[inline(always)]
    pub const fn set_c0oe(&mut self, val: super::vals::C0oe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "ACMPLP0 VCOUT Output Polarity Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn c0op(&self) -> super::vals::C0op {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::C0op::from_bits(val as u8)
    }
    #[doc = "ACMPLP0 VCOUT Output Polarity Selection"]
    #[inline(always)]
    pub const fn set_c0op(&mut self, val: super::vals::C0op) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
    }
    #[doc = "ACMPLP1 VCOUT Pin Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn c1oe(&self) -> super::vals::C1oe {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::C1oe::from_bits(val as u8)
    }
    #[doc = "ACMPLP1 VCOUT Pin Output Enable"]
    #[inline(always)]
    pub const fn set_c1oe(&mut self, val: super::vals::C1oe) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "ACMPLP1 VCOUT Output Polarity Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn c1op(&self) -> super::vals::C1op {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::C1op::from_bits(val as u8)
    }
    #[doc = "ACMPLP1 VCOUT Output Polarity Selection"]
    #[inline(always)]
    pub const fn set_c1op(&mut self, val: super::vals::C1op) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "ACMPLP0/ACMPLP1 Speed Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn spdmd(&self) -> super::vals::Spdmd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Spdmd::from_bits(val as u8)
    }
    #[doc = "ACMPLP0/ACMPLP1 Speed Selection"]
    #[inline(always)]
    pub const fn set_spdmd(&mut self, val: super::vals::Spdmd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Compocr {
    #[inline(always)]
    fn default() -> Compocr {
        Compocr(0)
    }
}
impl core::fmt::Debug for Compocr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Compocr")
            .field("reserved", &self.reserved())
            .field("c0oe", &self.c0oe())
            .field("c0op", &self.c0op())
            .field("reserved_2", &self.reserved_2())
            .field("c1oe", &self.c1oe())
            .field("c1op", &self.c1op())
            .field("spdmd", &self.spdmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Compocr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Compocr {{ reserved: {=bool:?}, c0oe: {:?}, c0op: {:?}, reserved_2: {=u8:?}, c1oe: {:?}, c1op: {:?}, spdmd: {:?} }}",
            self.reserved(),
            self.c0oe(),
            self.c0op(),
            self.reserved_2(),
            self.c1oe(),
            self.c1op(),
            self.spdmd()
        )
    }
}
#[doc = "Comparator Input Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compsel0(pub u8);
impl Compsel0 {
    #[doc = "ACMPLP0 Input(IVCMP0) Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpsel20(&self) -> super::vals::Cmpsel20 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cmpsel20::from_bits(val as u8)
    }
    #[doc = "ACMPLP0 Input(IVCMP0) Selection"]
    #[inline(always)]
    pub const fn set_cmpsel20(&mut self, val: super::vals::Cmpsel20) {
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
    #[doc = "ACMPLP1 Input (IVCMP1) Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpsel64(&self) -> super::vals::Cmpsel64 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Cmpsel64::from_bits(val as u8)
    }
    #[doc = "ACMPLP1 Input (IVCMP1) Selection"]
    #[inline(always)]
    pub const fn set_cmpsel64(&mut self, val: super::vals::Cmpsel64) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u8) & 0x07) << 4usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Compsel0 {
    #[inline(always)]
    fn default() -> Compsel0 {
        Compsel0(0)
    }
}
impl core::fmt::Debug for Compsel0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Compsel0")
            .field("cmpsel20", &self.cmpsel20())
            .field("reserved", &self.reserved())
            .field("cmpsel64", &self.cmpsel64())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Compsel0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Compsel0 {{ cmpsel20: {:?}, reserved: {=bool:?}, cmpsel64: {:?}, reserved_2: {=bool:?} }}",
            self.cmpsel20(),
            self.reserved(),
            self.cmpsel64(),
            self.reserved_2()
        )
    }
}
#[doc = "Comparator Reference Voltage Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compsel1(pub u8);
impl Compsel1 {
    #[doc = "ACMPLP0 Reference Voltage(IVREF0) Selection*"]
    #[must_use]
    #[inline(always)]
    pub const fn crvs20(&self) -> super::vals::Crvs20 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Crvs20::from_bits(val as u8)
    }
    #[doc = "ACMPLP0 Reference Voltage(IVREF0) Selection*"]
    #[inline(always)]
    pub const fn set_crvs20(&mut self, val: super::vals::Crvs20) {
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
    #[doc = "ACMPLP1 Reference Voltage(IVREF1) Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn crvs64(&self) -> super::vals::Crvs64 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Crvs64::from_bits(val as u8)
    }
    #[doc = "ACMPLP1 Reference Voltage(IVREF1) Selection"]
    #[inline(always)]
    pub const fn set_crvs64(&mut self, val: super::vals::Crvs64) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u8) & 0x07) << 4usize);
    }
    #[doc = "ACMPLP1 Reference Voltage Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn c1vrf2(&self) -> super::vals::C1vrf2 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::C1vrf2::from_bits(val as u8)
    }
    #[doc = "ACMPLP1 Reference Voltage Selection"]
    #[inline(always)]
    pub const fn set_c1vrf2(&mut self, val: super::vals::C1vrf2) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Compsel1 {
    #[inline(always)]
    fn default() -> Compsel1 {
        Compsel1(0)
    }
}
impl core::fmt::Debug for Compsel1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Compsel1")
            .field("crvs20", &self.crvs20())
            .field("reserved", &self.reserved())
            .field("crvs64", &self.crvs64())
            .field("c1vrf2", &self.c1vrf2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Compsel1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Compsel1 {{ crvs20: {:?}, reserved: {=bool:?}, crvs64: {:?}, c1vrf2: {:?} }}",
            self.crvs20(),
            self.reserved(),
            self.crvs64(),
            self.c1vrf2()
        )
    }
}
