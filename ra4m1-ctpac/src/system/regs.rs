#[doc = "Backup Register Access Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkracr(pub u8);
impl Bkracr {
    #[doc = "Backup Register Access Control Register"]
    #[must_use]
    #[inline(always)]
    pub const fn bkracs(&self) -> super::vals::Bkracs {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Bkracs::from_bits(val as u8)
    }
    #[doc = "Backup Register Access Control Register"]
    #[inline(always)]
    pub const fn set_bkracs(&mut self, val: super::vals::Bkracs) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u8) & 0x07) << 0usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for Bkracr {
    #[inline(always)]
    fn default() -> Bkracr {
        Bkracr(0)
    }
}
impl core::fmt::Debug for Bkracr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bkracr")
            .field("bkracs", &self.bkracs())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bkracr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bkracr {{ bkracs: {:?}, reserved: {=u8:?} }}",
            self.bkracs(),
            self.reserved()
        )
    }
}
#[doc = "Clock Out Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ckocr(pub u8);
impl Ckocr {
    #[doc = "Clock out source select"]
    #[must_use]
    #[inline(always)]
    pub const fn ckosel(&self) -> super::vals::Ckosel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ckosel::from_bits(val as u8)
    }
    #[doc = "Clock out source select"]
    #[inline(always)]
    pub const fn set_ckosel(&mut self, val: super::vals::Ckosel) {
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
    #[doc = "Clock out input frequency Division Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ckodiv(&self) -> super::vals::Ckodiv {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Ckodiv::from_bits(val as u8)
    }
    #[doc = "Clock out input frequency Division Select"]
    #[inline(always)]
    pub const fn set_ckodiv(&mut self, val: super::vals::Ckodiv) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u8) & 0x07) << 4usize);
    }
    #[doc = "Clock out enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ckoen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Clock out enable"]
    #[inline(always)]
    pub const fn set_ckoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Ckocr {
    #[inline(always)]
    fn default() -> Ckocr {
        Ckocr(0)
    }
}
impl core::fmt::Debug for Ckocr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ckocr")
            .field("ckosel", &self.ckosel())
            .field("reserved", &self.reserved())
            .field("ckodiv", &self.ckodiv())
            .field("ckoen", &self.ckoen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ckocr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ckocr {{ ckosel: {:?}, reserved: {=bool:?}, ckodiv: {:?}, ckoen: {=bool:?} }}",
            self.ckosel(),
            self.reserved(),
            self.ckodiv(),
            self.ckoen()
        )
    }
}
#[doc = "Flash Operation Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flstop(pub u8);
impl Flstop {
    #[doc = "Selecting ON/OFF of the Flash Memory Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn flstop(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting ON/OFF of the Flash Memory Operation"]
    #[inline(always)]
    pub const fn set_flstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u8) & 0x07) << 1usize);
    }
    #[doc = "Flash Memory Operation Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flstpf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Memory Operation Status Flag"]
    #[inline(always)]
    pub const fn set_flstpf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Flstop {
    #[inline(always)]
    fn default() -> Flstop {
        Flstop(0)
    }
}
impl core::fmt::Debug for Flstop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flstop")
            .field("flstop", &self.flstop())
            .field("reserved", &self.reserved())
            .field("flstpf", &self.flstpf())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flstop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flstop {{ flstop: {=bool:?}, reserved: {=u8:?}, flstpf: {=bool:?}, reserved_2: {=u8:?} }}",
            self.flstop(),
            self.reserved(),
            self.flstpf(),
            self.reserved_2()
        )
    }
}
#[doc = "High-Speed On-Chip Oscillator Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hococr(pub u8);
impl Hococr {
    #[doc = "HOCO Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn hcstp(&self) -> super::vals::Hcstp {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hcstp::from_bits(val as u8)
    }
    #[doc = "HOCO Stop"]
    #[inline(always)]
    pub const fn set_hcstp(&mut self, val: super::vals::Hcstp) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Hococr {
    #[inline(always)]
    fn default() -> Hococr {
        Hococr(0)
    }
}
impl core::fmt::Debug for Hococr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hococr")
            .field("hcstp", &self.hcstp())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hococr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hococr {{ hcstp: {:?}, reserved: {=u8:?} }}",
            self.hcstp(),
            self.reserved()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hococr2(pub u8);
impl Hococr2 {
    #[must_use]
    #[inline(always)]
    pub const fn hcfrqw(&self) -> super::vals::Hcfrq1 {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Hcfrq1::from_bits(val as u8)
    }
    #[inline(always)]
    pub const fn set_hcfrqw(&mut self, val: super::vals::Hcfrq1) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u8) & 0x07) << 3usize);
    }
}
impl Default for Hococr2 {
    #[inline(always)]
    fn default() -> Hococr2 {
        Hococr2(0)
    }
}
impl core::fmt::Debug for Hococr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hococr2")
            .field("hcfrqw", &self.hcfrqw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hococr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hococr2 {{ hcfrqw: {:?} }}", self.hcfrqw())
    }
}
#[doc = "HOCO User Trimming Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hocoutcr(pub u8);
impl Hocoutcr {
    #[doc = "HOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original HOCO trimming bits"]
    #[must_use]
    #[inline(always)]
    pub const fn hocoutrm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "HOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original HOCO trimming bits"]
    #[inline(always)]
    pub const fn set_hocoutrm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Hocoutcr {
    #[inline(always)]
    fn default() -> Hocoutcr {
        Hocoutcr(0)
    }
}
impl core::fmt::Debug for Hocoutcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hocoutcr")
            .field("hocoutrm", &self.hocoutrm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hocoutcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hocoutcr {{ hocoutrm: {=u8:?} }}", self.hocoutrm())
    }
}
#[doc = "High-Speed On-Chip Oscillator Wait Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hocowtcr(pub u8);
impl Hocowtcr {
    #[doc = "HOCO wait time setting"]
    #[must_use]
    #[inline(always)]
    pub const fn hsts(&self) -> super::vals::Hsts {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Hsts::from_bits(val as u8)
    }
    #[doc = "HOCO wait time setting"]
    #[inline(always)]
    pub const fn set_hsts(&mut self, val: super::vals::Hsts) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u8) & 0x07) << 0usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for Hocowtcr {
    #[inline(always)]
    fn default() -> Hocowtcr {
        Hocowtcr(0)
    }
}
impl core::fmt::Debug for Hocowtcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hocowtcr")
            .field("hsts", &self.hsts())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hocowtcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hocowtcr {{ hsts: {:?}, reserved: {=u8:?} }}",
            self.hsts(),
            self.reserved()
        )
    }
}
#[doc = "Low-Speed On-Chip Oscillator Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lococr(pub u8);
impl Lococr {
    #[doc = "LOCO Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn lcstp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LOCO Stop"]
    #[inline(always)]
    pub const fn set_lcstp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Lococr {
    #[inline(always)]
    fn default() -> Lococr {
        Lococr(0)
    }
}
impl core::fmt::Debug for Lococr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lococr")
            .field("lcstp", &self.lcstp())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lococr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lococr {{ lcstp: {=bool:?}, reserved: {=u8:?} }}",
            self.lcstp(),
            self.reserved()
        )
    }
}
#[doc = "LOCO User Trimming Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Locoutcr(pub u8);
impl Locoutcr {
    #[doc = "LOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original LOCO trimming bits"]
    #[must_use]
    #[inline(always)]
    pub const fn locoutrm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "LOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original LOCO trimming bits"]
    #[inline(always)]
    pub const fn set_locoutrm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Locoutcr {
    #[inline(always)]
    fn default() -> Locoutcr {
        Locoutcr(0)
    }
}
impl core::fmt::Debug for Locoutcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Locoutcr")
            .field("locoutrm", &self.locoutrm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Locoutcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Locoutcr {{ locoutrm: {=u8:?} }}", self.locoutrm())
    }
}
#[doc = "Voltage Monitor Circuit Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvcmpcr(pub u8);
impl Lvcmpcr {
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Voltage Detection 1 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd1e(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Detection 1 Enable"]
    #[inline(always)]
    pub const fn set_lvd1e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Voltage Detection 2 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd2e(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Detection 2 Enable"]
    #[inline(always)]
    pub const fn set_lvd2e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Lvcmpcr {
    #[inline(always)]
    fn default() -> Lvcmpcr {
        Lvcmpcr(0)
    }
}
impl core::fmt::Debug for Lvcmpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lvcmpcr")
            .field("reserved", &self.reserved())
            .field("reserved_2", &self.reserved_2())
            .field("reserved_3", &self.reserved_3())
            .field("lvd1e", &self.lvd1e())
            .field("lvd2e", &self.lvd2e())
            .field("reserved_4", &self.reserved_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lvcmpcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lvcmpcr {{ reserved: {=u8:?}, reserved_2: {=u8:?}, reserved_3: {=bool:?}, lvd1e: {=bool:?}, lvd2e: {=bool:?}, reserved_4: {=bool:?} }}",
            self.reserved(),
            self.reserved_2(),
            self.reserved_3(),
            self.lvd1e(),
            self.lvd2e(),
            self.reserved_4()
        )
    }
}
#[doc = "Voltage Monitor %s Circuit Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdcr0(pub u8);
impl Lvdcr0 {
    #[doc = "Voltage Monitor Interrupt/Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Monitor Interrupt/Reset Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Voltage Monitor Circuit Comparison Result Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Monitor Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub const fn set_cmpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u8) & 0x07) << 3usize);
    }
    #[doc = "Voltage Monitor Circuit Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ri(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Monitor Circuit Mode Select"]
    #[inline(always)]
    pub const fn set_ri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Voltage Monitor Reset Negate Select"]
    #[must_use]
    #[inline(always)]
    pub const fn rn(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Monitor Reset Negate Select"]
    #[inline(always)]
    pub const fn set_rn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Lvdcr0 {
    #[inline(always)]
    fn default() -> Lvdcr0 {
        Lvdcr0(0)
    }
}
impl core::fmt::Debug for Lvdcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lvdcr0")
            .field("rie", &self.rie())
            .field("reserved", &self.reserved())
            .field("cmpe", &self.cmpe())
            .field("reserved_2", &self.reserved_2())
            .field("ri", &self.ri())
            .field("rn", &self.rn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lvdcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lvdcr0 {{ rie: {=bool:?}, reserved: {=bool:?}, cmpe: {=bool:?}, reserved_2: {=u8:?}, ri: {=bool:?}, rn: {=bool:?} }}",
            self.rie(),
            self.reserved(),
            self.cmpe(),
            self.reserved_2(),
            self.ri(),
            self.rn()
        )
    }
}
#[doc = "Voltage Monitor %s Circuit Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdcr1(pub u8);
impl Lvdcr1 {
    #[doc = "Voltage Monitor Interrupt Generation Condition Select"]
    #[must_use]
    #[inline(always)]
    pub const fn idtsel(&self) -> super::vals::Idtsel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Idtsel::from_bits(val as u8)
    }
    #[doc = "Voltage Monitor Interrupt Generation Condition Select"]
    #[inline(always)]
    pub const fn set_idtsel(&mut self, val: super::vals::Idtsel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "Voltage Monitor Interrupt Type Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqsel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Monitor Interrupt Type Select"]
    #[inline(always)]
    pub const fn set_irqsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for Lvdcr1 {
    #[inline(always)]
    fn default() -> Lvdcr1 {
        Lvdcr1(0)
    }
}
impl core::fmt::Debug for Lvdcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lvdcr1")
            .field("idtsel", &self.idtsel())
            .field("irqsel", &self.irqsel())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lvdcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lvdcr1 {{ idtsel: {:?}, irqsel: {=bool:?}, reserved: {=u8:?} }}",
            self.idtsel(),
            self.irqsel(),
            self.reserved()
        )
    }
}
#[doc = "Voltage Detection Level Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdlvlr(pub u8);
impl Lvdlvlr {
    #[doc = "Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd1lvl(&self) -> super::vals::Lvd1lvl {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Lvd1lvl::from_bits(val as u8)
    }
    #[doc = "Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub const fn set_lvd1lvl(&mut self, val: super::vals::Lvd1lvl) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u8) & 0x1f) << 0usize);
    }
    #[doc = "Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd2lvl(&self) -> super::vals::Lvd2lvl {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::Lvd2lvl::from_bits(val as u8)
    }
    #[doc = "Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub const fn set_lvd2lvl(&mut self, val: super::vals::Lvd2lvl) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u8) & 0x07) << 5usize);
    }
}
impl Default for Lvdlvlr {
    #[inline(always)]
    fn default() -> Lvdlvlr {
        Lvdlvlr(0)
    }
}
impl core::fmt::Debug for Lvdlvlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lvdlvlr")
            .field("lvd1lvl", &self.lvd1lvl())
            .field("lvd2lvl", &self.lvd2lvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lvdlvlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lvdlvlr {{ lvd1lvl: {:?}, lvd2lvl: {:?} }}",
            self.lvd1lvl(),
            self.lvd2lvl()
        )
    }
}
#[doc = "Voltage Monitor %s Circuit Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdsr(pub u8);
impl Lvdsr {
    #[doc = "Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn det(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0."]
    #[inline(always)]
    pub const fn set_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Voltage Monitor 1 Signal Monitor Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn mon(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Monitor 1 Signal Monitor Flag"]
    #[inline(always)]
    pub const fn set_mon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for Lvdsr {
    #[inline(always)]
    fn default() -> Lvdsr {
        Lvdsr(0)
    }
}
impl core::fmt::Debug for Lvdsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lvdsr")
            .field("det", &self.det())
            .field("mon", &self.mon())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lvdsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lvdsr {{ det: {=bool:?}, mon: {=bool:?}, reserved: {=u8:?} }}",
            self.det(),
            self.mon(),
            self.reserved()
        )
    }
}
#[doc = "Memory Wait Cycle Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memwait(pub u8);
impl Memwait {
    #[doc = "Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\\[2:0\\] bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz)."]
    #[must_use]
    #[inline(always)]
    pub const fn memwait(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\\[2:0\\] bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz)."]
    #[inline(always)]
    pub const fn set_memwait(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Memwait {
    #[inline(always)]
    fn default() -> Memwait {
        Memwait(0)
    }
}
impl core::fmt::Debug for Memwait {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Memwait")
            .field("memwait", &self.memwait())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Memwait {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Memwait {{ memwait: {=bool:?}, reserved: {=u8:?} }}",
            self.memwait(),
            self.reserved()
        )
    }
}
#[doc = "Middle-Speed On-Chip Oscillator Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mococr(pub u8);
impl Mococr {
    #[doc = "MOCO Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mcstp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MOCO Stop"]
    #[inline(always)]
    pub const fn set_mcstp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Mococr {
    #[inline(always)]
    fn default() -> Mococr {
        Mococr(0)
    }
}
impl core::fmt::Debug for Mococr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mococr")
            .field("mcstp", &self.mcstp())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mococr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mococr {{ mcstp: {=bool:?}, reserved: {=u8:?} }}",
            self.mcstp(),
            self.reserved()
        )
    }
}
#[doc = "MOCO User Trimming Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mocoutcr(pub u8);
impl Mocoutcr {
    #[doc = "MOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original MOCO trimming bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mocoutrm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "MOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original MOCO trimming bits"]
    #[inline(always)]
    pub const fn set_mocoutrm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Mocoutcr {
    #[inline(always)]
    fn default() -> Mocoutcr {
        Mocoutcr(0)
    }
}
impl core::fmt::Debug for Mocoutcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mocoutcr")
            .field("mocoutrm", &self.mocoutrm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mocoutcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mocoutcr {{ mocoutrm: {=u8:?} }}", self.mocoutrm())
    }
}
#[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Momcr(pub u8);
impl Momcr {
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
    }
    #[doc = "Main Clock Oscillator Drive Capability 1 Switching"]
    #[must_use]
    #[inline(always)]
    pub const fn modrv1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Main Clock Oscillator Drive Capability 1 Switching"]
    #[inline(always)]
    pub const fn set_modrv1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Main Clock Oscillator Switching"]
    #[must_use]
    #[inline(always)]
    pub const fn mosel(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Main Clock Oscillator Switching"]
    #[inline(always)]
    pub const fn set_mosel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Momcr {
    #[inline(always)]
    fn default() -> Momcr {
        Momcr(0)
    }
}
impl core::fmt::Debug for Momcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Momcr")
            .field("reserved", &self.reserved())
            .field("modrv1", &self.modrv1())
            .field("reserved_2", &self.reserved_2())
            .field("mosel", &self.mosel())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Momcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Momcr {{ reserved: {=u8:?}, modrv1: {=bool:?}, reserved_2: {=u8:?}, mosel: {=bool:?}, reserved_3: {=bool:?} }}",
            self.reserved(),
            self.modrv1(),
            self.reserved_2(),
            self.mosel(),
            self.reserved_3()
        )
    }
}
#[doc = "Main Clock Oscillator Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mosccr(pub u8);
impl Mosccr {
    #[doc = "Main Clock Oscillator Stop Note: MOMCR register must be set before setting MOSTP to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn mostp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Main Clock Oscillator Stop Note: MOMCR register must be set before setting MOSTP to 0."]
    #[inline(always)]
    pub const fn set_mostp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Mosccr {
    #[inline(always)]
    fn default() -> Mosccr {
        Mosccr(0)
    }
}
impl core::fmt::Debug for Mosccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mosccr")
            .field("mostp", &self.mostp())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mosccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mosccr {{ mostp: {=bool:?}, reserved: {=u8:?} }}",
            self.mostp(),
            self.reserved()
        )
    }
}
#[doc = "Main Clock Oscillator Wait Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Moscwtcr(pub u8);
impl Moscwtcr {
    #[doc = "Main clock oscillator wait time setting"]
    #[must_use]
    #[inline(always)]
    pub const fn msts(&self) -> super::vals::Msts {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Msts::from_bits(val as u8)
    }
    #[doc = "Main clock oscillator wait time setting"]
    #[inline(always)]
    pub const fn set_msts(&mut self, val: super::vals::Msts) {
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
impl Default for Moscwtcr {
    #[inline(always)]
    fn default() -> Moscwtcr {
        Moscwtcr(0)
    }
}
impl core::fmt::Debug for Moscwtcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Moscwtcr")
            .field("msts", &self.msts())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Moscwtcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Moscwtcr {{ msts: {:?}, reserved: {=u8:?} }}",
            self.msts(),
            self.reserved()
        )
    }
}
#[doc = "Module Stop Control Register A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcra(pub u32);
impl Mstpcra {
    #[doc = "RAM0 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpa0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RAM0 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpa0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 11111. The write value should be 11111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 11111. The write value should be 11111."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[doc = "ECCRAM Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpa6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "ECCRAM Module Stop"]
    #[inline(always)]
    pub const fn set_mstpa6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "These bits are read as 111111111111111. The write value should be 111111111111111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x7fff;
        val as u16
    }
    #[doc = "These bits are read as 111111111111111. The write value should be 111111111111111."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 7usize)) | (((val as u32) & 0x7fff) << 7usize);
    }
    #[doc = "DMA Controller/Data Transfer Controller Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpa22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Controller/Data Transfer Controller Module Stop"]
    #[inline(always)]
    pub const fn set_mstpa22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "These bits are read as 111111111. The write value should be 111111111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u16 {
        let val = (self.0 >> 23usize) & 0x01ff;
        val as u16
    }
    #[doc = "These bits are read as 111111111. The write value should be 111111111."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 23usize)) | (((val as u32) & 0x01ff) << 23usize);
    }
}
impl Default for Mstpcra {
    #[inline(always)]
    fn default() -> Mstpcra {
        Mstpcra(0)
    }
}
impl core::fmt::Debug for Mstpcra {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mstpcra")
            .field("mstpa0", &self.mstpa0())
            .field("reserved", &self.reserved())
            .field("mstpa6", &self.mstpa6())
            .field("reserved_2", &self.reserved_2())
            .field("mstpa22", &self.mstpa22())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mstpcra {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mstpcra {{ mstpa0: {=bool:?}, reserved: {=u8:?}, mstpa6: {=bool:?}, reserved_2: {=u16:?}, mstpa22: {=bool:?}, reserved_3: {=u16:?} }}",
            self.mstpa0(),
            self.reserved(),
            self.mstpa6(),
            self.reserved_2(),
            self.mstpa22(),
            self.reserved_3()
        )
    }
}
#[doc = "Operating Power Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opccr(pub u8);
impl Opccr {
    #[doc = "Operating Power Control Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn opcm(&self) -> super::vals::Opcm {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Opcm::from_bits(val as u8)
    }
    #[doc = "Operating Power Control Mode Select"]
    #[inline(always)]
    pub const fn set_opcm(&mut self, val: super::vals::Opcm) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
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
    #[doc = "Operating Power Control Mode Transition Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn opcmtsf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Operating Power Control Mode Transition Status Flag"]
    #[inline(always)]
    pub const fn set_opcmtsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Opccr {
    #[inline(always)]
    fn default() -> Opccr {
        Opccr(0)
    }
}
impl core::fmt::Debug for Opccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opccr")
            .field("opcm", &self.opcm())
            .field("reserved", &self.reserved())
            .field("opcmtsf", &self.opcmtsf())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Opccr {{ opcm: {:?}, reserved: {=u8:?}, opcmtsf: {=bool:?}, reserved_2: {=u8:?} }}",
            self.opcm(),
            self.reserved(),
            self.opcmtsf(),
            self.reserved_2()
        )
    }
}
#[doc = "Oscillation Stabilization Flag Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscsf(pub u8);
impl Oscsf {
    #[doc = "HOCO Clock Oscillation Stabilization Flag NOTE: The HOCOSF bit value after a reset is 1 when the OFS1.HOCOEN bit is 0. It is 0 when the OFS1.HOCOEN bit is 1."]
    #[must_use]
    #[inline(always)]
    pub const fn hocosf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HOCO Clock Oscillation Stabilization Flag NOTE: The HOCOSF bit value after a reset is 1 when the OFS1.HOCOEN bit is 0. It is 0 when the OFS1.HOCOEN bit is 1."]
    #[inline(always)]
    pub const fn set_hocosf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u8) & 0x03) << 1usize);
    }
    #[doc = "Main Clock Oscillation Stabilization Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn moscsf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Main Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub const fn set_moscsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "This bit is read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "PLL Clock Oscillation Stabilization Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pllsf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub const fn set_pllsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "These bits are read as 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
    }
}
impl Default for Oscsf {
    #[inline(always)]
    fn default() -> Oscsf {
        Oscsf(0)
    }
}
impl core::fmt::Debug for Oscsf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oscsf")
            .field("hocosf", &self.hocosf())
            .field("reserved", &self.reserved())
            .field("moscsf", &self.moscsf())
            .field("reserved_2", &self.reserved_2())
            .field("pllsf", &self.pllsf())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oscsf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Oscsf {{ hocosf: {=bool:?}, reserved: {=u8:?}, moscsf: {=bool:?}, reserved_2: {=bool:?}, pllsf: {=bool:?}, reserved_3: {=u8:?} }}",
            self.hocosf(),
            self.reserved(),
            self.moscsf(),
            self.reserved_2(),
            self.pllsf(),
            self.reserved_3()
        )
    }
}
#[doc = "Oscillation Stop Detection Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostdcr(pub u8);
impl Ostdcr {
    #[doc = "Oscillation Stop Detection Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ostdie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ostdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u8) & 0x3f) << 1usize);
    }
    #[doc = "Oscillation Stop Detection Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ostde(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillation Stop Detection Function Enable"]
    #[inline(always)]
    pub const fn set_ostde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Ostdcr {
    #[inline(always)]
    fn default() -> Ostdcr {
        Ostdcr(0)
    }
}
impl core::fmt::Debug for Ostdcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ostdcr")
            .field("ostdie", &self.ostdie())
            .field("reserved", &self.reserved())
            .field("ostde", &self.ostde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ostdcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ostdcr {{ ostdie: {=bool:?}, reserved: {=u8:?}, ostde: {=bool:?} }}",
            self.ostdie(),
            self.reserved(),
            self.ostde()
        )
    }
}
#[doc = "Oscillation Stop Detection Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostdsr(pub u8);
impl Ostdsr {
    #[doc = "Oscillation Stop Detection Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ostdf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub const fn set_ostdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Ostdsr {
    #[inline(always)]
    fn default() -> Ostdsr {
        Ostdsr(0)
    }
}
impl core::fmt::Debug for Ostdsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ostdsr")
            .field("ostdf", &self.ostdf())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ostdsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ostdsr {{ ostdf: {=bool:?}, reserved: {=u8:?} }}",
            self.ostdf(),
            self.reserved()
        )
    }
}
#[doc = "PLL Clock Control Register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllccr2(pub u8);
impl Pllccr2 {
    #[doc = "PLL Frequency Multiplication Factor Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pllmul(&self) -> super::vals::Pllmul {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Pllmul::from_bits(val as u8)
    }
    #[doc = "PLL Frequency Multiplication Factor Select"]
    #[inline(always)]
    pub const fn set_pllmul(&mut self, val: super::vals::Pllmul) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u8) & 0x1f) << 0usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "PLL Output Frequency Division Ratio Select"]
    #[must_use]
    #[inline(always)]
    pub const fn plodiv(&self) -> super::vals::Plodiv {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Plodiv::from_bits(val as u8)
    }
    #[doc = "PLL Output Frequency Division Ratio Select"]
    #[inline(always)]
    pub const fn set_plodiv(&mut self, val: super::vals::Plodiv) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u8) & 0x03) << 6usize);
    }
}
impl Default for Pllccr2 {
    #[inline(always)]
    fn default() -> Pllccr2 {
        Pllccr2(0)
    }
}
impl core::fmt::Debug for Pllccr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pllccr2")
            .field("pllmul", &self.pllmul())
            .field("reserved", &self.reserved())
            .field("plodiv", &self.plodiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pllccr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pllccr2 {{ pllmul: {:?}, reserved: {=bool:?}, plodiv: {:?} }}",
            self.pllmul(),
            self.reserved(),
            self.plodiv()
        )
    }
}
#[doc = "PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllcr(pub u8);
impl Pllcr {
    #[doc = "PLL Stop Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pllstp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Stop Control"]
    #[inline(always)]
    pub const fn set_pllstp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Pllcr {
    #[inline(always)]
    fn default() -> Pllcr {
        Pllcr(0)
    }
}
impl core::fmt::Debug for Pllcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pllcr")
            .field("pllstp", &self.pllstp())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pllcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pllcr {{ pllstp: {=bool:?}, reserved: {=u8:?} }}",
            self.pllstp(),
            self.reserved()
        )
    }
}
#[doc = "Protect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prcr(pub u16);
impl Prcr {
    #[doc = "Protect Bit 0"]
    #[must_use]
    #[inline(always)]
    pub const fn prc0(&self) -> super::vals::Prc0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Prc0::from_bits(val as u8)
    }
    #[doc = "Protect Bit 0"]
    #[inline(always)]
    pub const fn set_prc0(&mut self, val: super::vals::Prc0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Protect Bit 1"]
    #[must_use]
    #[inline(always)]
    pub const fn prc1(&self) -> super::vals::Prc1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Prc1::from_bits(val as u8)
    }
    #[doc = "Protect Bit 1"]
    #[inline(always)]
    pub const fn set_prc1(&mut self, val: super::vals::Prc1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Protect Bit 3"]
    #[must_use]
    #[inline(always)]
    pub const fn prc3(&self) -> super::vals::Prc3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Prc3::from_bits(val as u8)
    }
    #[doc = "Protect Bit 3"]
    #[inline(always)]
    pub const fn set_prc3(&mut self, val: super::vals::Prc3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PRC Key Code"]
    #[must_use]
    #[inline(always)]
    pub const fn prkey(&self) -> super::vals::Prkey {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Prkey::from_bits(val as u8)
    }
    #[doc = "PRC Key Code"]
    #[inline(always)]
    pub const fn set_prkey(&mut self, val: super::vals::Prkey) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
    }
}
impl Default for Prcr {
    #[inline(always)]
    fn default() -> Prcr {
        Prcr(0)
    }
}
impl core::fmt::Debug for Prcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prcr")
            .field("prc0", &self.prc0())
            .field("prc1", &self.prc1())
            .field("reserved", &self.reserved())
            .field("prc3", &self.prc3())
            .field("reserved_2", &self.reserved_2())
            .field("prkey", &self.prkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Prcr {{ prc0: {:?}, prc1: {:?}, reserved: {=bool:?}, prc3: {:?}, reserved_2: {=u8:?}, prkey: {:?} }}",
            self.prc0(),
            self.prc1(),
            self.reserved(),
            self.prc3(),
            self.reserved_2(),
            self.prkey()
        )
    }
}
#[doc = "Reset Status Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsr0(pub u8);
impl Rstsr0 {
    #[doc = "Power-On Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn porf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power-On Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub const fn set_porf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Voltage Monitor 0 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn lvd0rf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Monitor 0 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub const fn set_lvd0rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Voltage Monitor 1 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn lvd1rf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Monitor 1 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub const fn set_lvd1rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Voltage Monitor 2 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn lvd2rf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Monitor 2 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub const fn set_lvd2rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
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
impl Default for Rstsr0 {
    #[inline(always)]
    fn default() -> Rstsr0 {
        Rstsr0(0)
    }
}
impl core::fmt::Debug for Rstsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rstsr0")
            .field("porf", &self.porf())
            .field("lvd0rf", &self.lvd0rf())
            .field("lvd1rf", &self.lvd1rf())
            .field("lvd2rf", &self.lvd2rf())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rstsr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rstsr0 {{ porf: {=bool:?}, lvd0rf: {=bool:?}, lvd1rf: {=bool:?}, lvd2rf: {=bool:?}, reserved: {=u8:?} }}",
            self.porf(),
            self.lvd0rf(),
            self.lvd1rf(),
            self.lvd2rf(),
            self.reserved()
        )
    }
}
#[doc = "Reset Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsr1(pub u16);
impl Rstsr1 {
    #[doc = "Independent Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn iwdtrf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Independent Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub const fn set_iwdtrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn wdtrf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub const fn set_wdtrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Software Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn swrf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub const fn set_swrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "RAM Parity Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rperf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "RAM Parity Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub const fn set_rperf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "RAM ECC Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn reerf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "RAM ECC Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub const fn set_reerf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Bus Slave MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn bussrf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Slave MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub const fn set_bussrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Bus Master MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn busmrf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Master MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub const fn set_busmrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "SP Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn sperf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SP Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub const fn set_sperf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Rstsr1 {
    #[inline(always)]
    fn default() -> Rstsr1 {
        Rstsr1(0)
    }
}
impl core::fmt::Debug for Rstsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rstsr1")
            .field("iwdtrf", &self.iwdtrf())
            .field("wdtrf", &self.wdtrf())
            .field("swrf", &self.swrf())
            .field("reserved", &self.reserved())
            .field("rperf", &self.rperf())
            .field("reerf", &self.reerf())
            .field("bussrf", &self.bussrf())
            .field("busmrf", &self.busmrf())
            .field("sperf", &self.sperf())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rstsr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rstsr1 {{ iwdtrf: {=bool:?}, wdtrf: {=bool:?}, swrf: {=bool:?}, reserved: {=u8:?}, rperf: {=bool:?}, reerf: {=bool:?}, bussrf: {=bool:?}, busmrf: {=bool:?}, sperf: {=bool:?}, reserved_2: {=u8:?} }}",
            self.iwdtrf(),
            self.wdtrf(),
            self.swrf(),
            self.reserved(),
            self.rperf(),
            self.reerf(),
            self.bussrf(),
            self.busmrf(),
            self.sperf(),
            self.reserved_2()
        )
    }
}
#[doc = "Reset Status Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsr2(pub u8);
impl Rstsr2 {
    #[doc = "Cold/Warm Start Determination Flag Note: Only 1 can be written to set the flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cwsf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Cold/Warm Start Determination Flag Note: Only 1 can be written to set the flag."]
    #[inline(always)]
    pub const fn set_cwsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Rstsr2 {
    #[inline(always)]
    fn default() -> Rstsr2 {
        Rstsr2(0)
    }
}
impl core::fmt::Debug for Rstsr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rstsr2")
            .field("cwsf", &self.cwsf())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rstsr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rstsr2 {{ cwsf: {=bool:?}, reserved: {=u8:?} }}",
            self.cwsf(),
            self.reserved()
        )
    }
}
#[doc = "Standby Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbycr(pub u16);
impl Sbycr {
    #[doc = "These bits are read as 00000000000000. The write value should be 00000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "These bits are read as 00000000000000. The write value should be 00000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u16) & 0x3fff) << 0usize);
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Software Standby"]
    #[must_use]
    #[inline(always)]
    pub const fn ssby(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Software Standby"]
    #[inline(always)]
    pub const fn set_ssby(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sbycr {
    #[inline(always)]
    fn default() -> Sbycr {
        Sbycr(0)
    }
}
impl core::fmt::Debug for Sbycr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sbycr")
            .field("reserved", &self.reserved())
            .field("reserved_2", &self.reserved_2())
            .field("ssby", &self.ssby())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sbycr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sbycr {{ reserved: {=u16:?}, reserved_2: {=bool:?}, ssby: {=bool:?} }}",
            self.reserved(),
            self.reserved_2(),
            self.ssby()
        )
    }
}
#[doc = "System Clock Division Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sckdivcr(pub u32);
impl Sckdivcr {
    #[doc = "Peripheral Module Clock D (PCLKD) Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pckd(&self) -> super::vals::Pckd {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Pckd::from_bits(val as u8)
    }
    #[doc = "Peripheral Module Clock D (PCLKD) Select"]
    #[inline(always)]
    pub const fn set_pckd(&mut self, val: super::vals::Pckd) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
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
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Peripheral Module Clock C (PCLKC) Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pckc(&self) -> super::vals::Pckc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Pckc::from_bits(val as u8)
    }
    #[doc = "Peripheral Module Clock C (PCLKC) Select"]
    #[inline(always)]
    pub const fn set_pckc(&mut self, val: super::vals::Pckc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
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
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Peripheral Module Clock B (PCLKB) Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pckb(&self) -> super::vals::Pckb {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Pckb::from_bits(val as u8)
    }
    #[doc = "Peripheral Module Clock B (PCLKB) Select"]
    #[inline(always)]
    pub const fn set_pckb(&mut self, val: super::vals::Pckb) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Peripheral Module Clock A (PCLKA) Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pcka(&self) -> super::vals::Pcka {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Pcka::from_bits(val as u8)
    }
    #[doc = "Peripheral Module Clock A (PCLKA) Select"]
    #[inline(always)]
    pub const fn set_pcka(&mut self, val: super::vals::Pcka) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_6(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[inline(always)]
    pub const fn set_reserved_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_7(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "System Clock (ICLK) Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ick(&self) -> super::vals::Ick {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Ick::from_bits(val as u8)
    }
    #[doc = "System Clock (ICLK) Select"]
    #[inline(always)]
    pub const fn set_ick(&mut self, val: super::vals::Ick) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_8(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Flash IF Clock (FCLK) Select"]
    #[must_use]
    #[inline(always)]
    pub const fn fck(&self) -> super::vals::Fck {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Fck::from_bits(val as u8)
    }
    #[doc = "Flash IF Clock (FCLK) Select"]
    #[inline(always)]
    pub const fn set_fck(&mut self, val: super::vals::Fck) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_9(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sckdivcr {
    #[inline(always)]
    fn default() -> Sckdivcr {
        Sckdivcr(0)
    }
}
impl core::fmt::Debug for Sckdivcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sckdivcr")
            .field("pckd", &self.pckd())
            .field("reserved", &self.reserved())
            .field("pckc", &self.pckc())
            .field("reserved_2", &self.reserved_2())
            .field("pckb", &self.pckb())
            .field("reserved_3", &self.reserved_3())
            .field("pcka", &self.pcka())
            .field("reserved_4", &self.reserved_4())
            .field("reserved_5", &self.reserved_5())
            .field("reserved_6", &self.reserved_6())
            .field("reserved_7", &self.reserved_7())
            .field("ick", &self.ick())
            .field("reserved_8", &self.reserved_8())
            .field("fck", &self.fck())
            .field("reserved_9", &self.reserved_9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sckdivcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sckdivcr {{ pckd: {:?}, reserved: {=bool:?}, pckc: {:?}, reserved_2: {=bool:?}, pckb: {:?}, reserved_3: {=bool:?}, pcka: {:?}, reserved_4: {=bool:?}, reserved_5: {=u8:?}, reserved_6: {=bool:?}, reserved_7: {=u8:?}, ick: {:?}, reserved_8: {=bool:?}, fck: {:?}, reserved_9: {=bool:?} }}",
            self.pckd(),
            self.reserved(),
            self.pckc(),
            self.reserved_2(),
            self.pckb(),
            self.reserved_3(),
            self.pcka(),
            self.reserved_4(),
            self.reserved_5(),
            self.reserved_6(),
            self.reserved_7(),
            self.ick(),
            self.reserved_8(),
            self.fck(),
            self.reserved_9()
        )
    }
}
#[doc = "System Clock Source Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sckscr(pub u8);
impl Sckscr {
    #[doc = "Clock Source Select Selecting the system clock source faster than 32MHz(system clock source > 32MHz ) is prohibit when SCKDIVCR.ICK\\[2:0\\] bits select the division-by-1 and MEMWAIT.MEMWAIT =0."]
    #[must_use]
    #[inline(always)]
    pub const fn cksel(&self) -> super::vals::Cksel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cksel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select Selecting the system clock source faster than 32MHz(system clock source > 32MHz ) is prohibit when SCKDIVCR.ICK\\[2:0\\] bits select the division-by-1 and MEMWAIT.MEMWAIT =0."]
    #[inline(always)]
    pub const fn set_cksel(&mut self, val: super::vals::Cksel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u8) & 0x07) << 0usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for Sckscr {
    #[inline(always)]
    fn default() -> Sckscr {
        Sckscr(0)
    }
}
impl core::fmt::Debug for Sckscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sckscr")
            .field("cksel", &self.cksel())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sckscr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sckscr {{ cksel: {:?}, reserved: {=u8:?} }}",
            self.cksel(),
            self.reserved()
        )
    }
}
#[doc = "Segment LCD Source Clock Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slcdsckcr(pub u8);
impl Slcdsckcr {
    #[doc = "LCD Source Clock (LCDSRCCLK) Select"]
    #[must_use]
    #[inline(always)]
    pub const fn lcdscksel(&self) -> super::vals::Lcdscksel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Lcdscksel::from_bits(val as u8)
    }
    #[doc = "LCD Source Clock (LCDSRCCLK) Select"]
    #[inline(always)]
    pub const fn set_lcdscksel(&mut self, val: super::vals::Lcdscksel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u8) & 0x07) << 0usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u8) & 0x0f) << 3usize);
    }
    #[doc = "LCD Source Clock Out Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lcdscken(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LCD Source Clock Out Enable"]
    #[inline(always)]
    pub const fn set_lcdscken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Slcdsckcr {
    #[inline(always)]
    fn default() -> Slcdsckcr {
        Slcdsckcr(0)
    }
}
impl core::fmt::Debug for Slcdsckcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slcdsckcr")
            .field("lcdscksel", &self.lcdscksel())
            .field("reserved", &self.reserved())
            .field("lcdscken", &self.lcdscken())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slcdsckcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slcdsckcr {{ lcdscksel: {:?}, reserved: {=u8:?}, lcdscken: {=bool:?} }}",
            self.lcdscksel(),
            self.reserved(),
            self.lcdscken()
        )
    }
}
#[doc = "Snooze Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snzcr(pub u8);
impl Snzcr {
    #[doc = "RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdreqen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode."]
    #[inline(always)]
    pub const fn set_rxdreqen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "DTC Enable in Snooze Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn snzdtcen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DTC Enable in Snooze Mode"]
    #[inline(always)]
    pub const fn set_snzdtcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u8) & 0x1f) << 2usize);
    }
    #[doc = "Snooze Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn snze(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Mode Enable"]
    #[inline(always)]
    pub const fn set_snze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Snzcr {
    #[inline(always)]
    fn default() -> Snzcr {
        Snzcr(0)
    }
}
impl core::fmt::Debug for Snzcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Snzcr")
            .field("rxdreqen", &self.rxdreqen())
            .field("snzdtcen", &self.snzdtcen())
            .field("reserved", &self.reserved())
            .field("snze", &self.snze())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Snzcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Snzcr {{ rxdreqen: {=bool:?}, snzdtcen: {=bool:?}, reserved: {=u8:?}, snze: {=bool:?} }}",
            self.rxdreqen(),
            self.snzdtcen(),
            self.reserved(),
            self.snze()
        )
    }
}
#[doc = "Snooze End Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snzedcr(pub u8);
impl Snzedcr {
    #[doc = "AGT1 Underflow Snooze End Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn agtunfed(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AGT1 Underflow Snooze End Enable"]
    #[inline(always)]
    pub const fn set_agtunfed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Last DTC Transmission Completion Snooze End Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtczred(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub const fn set_dtczred(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Not Last DTC Transmission Completion Snooze End Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnzred(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Not Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub const fn set_dtcnzred(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "ADC140 Compare Match Snooze End Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ad0mated(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADC140 Compare Match Snooze End Enable"]
    #[inline(always)]
    pub const fn set_ad0mated(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "ADC140 Compare Mismatch Snooze End Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ad0umted(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ADC140 Compare Mismatch Snooze End Enable"]
    #[inline(always)]
    pub const fn set_ad0umted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u8) & 0x03) << 5usize);
    }
    #[doc = "SCI0 Address Mismatch Snooze End Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sci0umted(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SCI0 Address Mismatch Snooze End Enable"]
    #[inline(always)]
    pub const fn set_sci0umted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Snzedcr {
    #[inline(always)]
    fn default() -> Snzedcr {
        Snzedcr(0)
    }
}
impl core::fmt::Debug for Snzedcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Snzedcr")
            .field("agtunfed", &self.agtunfed())
            .field("dtczred", &self.dtczred())
            .field("dtcnzred", &self.dtcnzred())
            .field("ad0mated", &self.ad0mated())
            .field("ad0umted", &self.ad0umted())
            .field("reserved", &self.reserved())
            .field("sci0umted", &self.sci0umted())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Snzedcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Snzedcr {{ agtunfed: {=bool:?}, dtczred: {=bool:?}, dtcnzred: {=bool:?}, ad0mated: {=bool:?}, ad0umted: {=bool:?}, reserved: {=u8:?}, sci0umted: {=bool:?} }}",
            self.agtunfed(),
            self.dtczred(),
            self.dtcnzred(),
            self.ad0mated(),
            self.ad0umted(),
            self.reserved(),
            self.sci0umted()
        )
    }
}
#[doc = "Snooze Request Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snzreqcr(pub u32);
impl Snzreqcr {
    #[doc = "Snooze Request Enable 0 Enable IRQ0 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 0 Enable IRQ0 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Snooze Request Enable 1 Enable IRQ1 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 1 Enable IRQ1 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Snooze Request Enable 2 Enable IRQ2 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 2 Enable IRQ2 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Snooze Request Enable 3 Enable IRQ3 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 3 Enable IRQ3 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Snooze Request Enable 4 Enable IRQ4 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 4 Enable IRQ4 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Snooze Request Enable 5 Enable IRQ5 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 5 Enable IRQ5 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Snooze Request Enable 6 Enable IRQ6 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 6 Enable IRQ6 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Snooze Request Enable 7 Enable IRQ7 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 7 Enable IRQ7 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Snooze Request Enable 8 Enable IRQ8 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 8 Enable IRQ8 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Snooze Request Enable 9 Enable IRQ9 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 9 Enable IRQ9 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Snooze Request Enable 10 Enable IRQ10 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 10 Enable IRQ10 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Snooze Request Enable 11 Enable IRQ11 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 11 Enable IRQ11 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Snooze Request Enable 12 Enable IRQ12 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 12 Enable IRQ12 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Snooze Request Enable 14 Enable IRQ14 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 14 Enable IRQ14 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Snooze Request Enable 15 Enable IRQ15 pin snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 15 Enable IRQ15 pin snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Snooze Request Enable 17 Enable KINT snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 17 Enable KINT snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "Snooze Request Enable 23 Enable RTC alarm snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 23 Enable RTC alarm snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Snooze Request Enable 24 Enable RTC alarm snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 24 Enable RTC alarm snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Snooze Request Enable 25 Enable RTC period snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 25 Enable RTC period snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Snooze Request Enable 28 Enable AGT1 underflow snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 28 Enable AGT1 underflow snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Snooze Request Enable 29 Enable AGT1 compare match A snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 29 Enable AGT1 compare match A snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Snooze Request Enable 30 Enable AGT1 compare match B snooze request"]
    #[must_use]
    #[inline(always)]
    pub const fn snzreqen30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Snooze Request Enable 30 Enable AGT1 compare match B snooze request"]
    #[inline(always)]
    pub const fn set_snzreqen30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_5(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Snzreqcr {
    #[inline(always)]
    fn default() -> Snzreqcr {
        Snzreqcr(0)
    }
}
impl core::fmt::Debug for Snzreqcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Snzreqcr")
            .field("snzreqen0", &self.snzreqen0())
            .field("snzreqen1", &self.snzreqen1())
            .field("snzreqen2", &self.snzreqen2())
            .field("snzreqen3", &self.snzreqen3())
            .field("snzreqen4", &self.snzreqen4())
            .field("snzreqen5", &self.snzreqen5())
            .field("snzreqen6", &self.snzreqen6())
            .field("snzreqen7", &self.snzreqen7())
            .field("snzreqen8", &self.snzreqen8())
            .field("snzreqen9", &self.snzreqen9())
            .field("snzreqen10", &self.snzreqen10())
            .field("snzreqen11", &self.snzreqen11())
            .field("snzreqen12", &self.snzreqen12())
            .field("reserved", &self.reserved())
            .field("snzreqen14", &self.snzreqen14())
            .field("snzreqen15", &self.snzreqen15())
            .field("reserved_2", &self.reserved_2())
            .field("snzreqen17", &self.snzreqen17())
            .field("reserved_3", &self.reserved_3())
            .field("snzreqen23", &self.snzreqen23())
            .field("snzreqen24", &self.snzreqen24())
            .field("snzreqen25", &self.snzreqen25())
            .field("reserved_4", &self.reserved_4())
            .field("snzreqen28", &self.snzreqen28())
            .field("snzreqen29", &self.snzreqen29())
            .field("snzreqen30", &self.snzreqen30())
            .field("reserved_5", &self.reserved_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Snzreqcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Snzreqcr {{ snzreqen0: {=bool:?}, snzreqen1: {=bool:?}, snzreqen2: {=bool:?}, snzreqen3: {=bool:?}, snzreqen4: {=bool:?}, snzreqen5: {=bool:?}, snzreqen6: {=bool:?}, snzreqen7: {=bool:?}, snzreqen8: {=bool:?}, snzreqen9: {=bool:?}, snzreqen10: {=bool:?}, snzreqen11: {=bool:?}, snzreqen12: {=bool:?}, reserved: {=bool:?}, snzreqen14: {=bool:?}, snzreqen15: {=bool:?}, reserved_2: {=bool:?}, snzreqen17: {=bool:?}, reserved_3: {=u8:?}, snzreqen23: {=bool:?}, snzreqen24: {=bool:?}, snzreqen25: {=bool:?}, reserved_4: {=u8:?}, snzreqen28: {=bool:?}, snzreqen29: {=bool:?}, snzreqen30: {=bool:?}, reserved_5: {=bool:?} }}",
            self.snzreqen0(),
            self.snzreqen1(),
            self.snzreqen2(),
            self.snzreqen3(),
            self.snzreqen4(),
            self.snzreqen5(),
            self.snzreqen6(),
            self.snzreqen7(),
            self.snzreqen8(),
            self.snzreqen9(),
            self.snzreqen10(),
            self.snzreqen11(),
            self.snzreqen12(),
            self.reserved(),
            self.snzreqen14(),
            self.snzreqen15(),
            self.reserved_2(),
            self.snzreqen17(),
            self.reserved_3(),
            self.snzreqen23(),
            self.snzreqen24(),
            self.snzreqen25(),
            self.reserved_4(),
            self.snzreqen28(),
            self.snzreqen29(),
            self.snzreqen30(),
            self.reserved_5()
        )
    }
}
#[doc = "Sub Clock Oscillator Mode Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Somcr(pub u8);
impl Somcr {
    #[doc = "Sub-Clock Oscillator Drive Capability Switching"]
    #[must_use]
    #[inline(always)]
    pub const fn sodrv(&self) -> super::vals::Sodrv {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sodrv::from_bits(val as u8)
    }
    #[doc = "Sub-Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    pub const fn set_sodrv(&mut self, val: super::vals::Sodrv) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for Somcr {
    #[inline(always)]
    fn default() -> Somcr {
        Somcr(0)
    }
}
impl core::fmt::Debug for Somcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Somcr")
            .field("sodrv", &self.sodrv())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Somcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Somcr {{ sodrv: {:?}, reserved: {=u8:?} }}",
            self.sodrv(),
            self.reserved()
        )
    }
}
#[doc = "Sub Operating Power Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sopccr(pub u8);
impl Sopccr {
    #[doc = "Sub Operating Power Control Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sopcm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Sub Operating Power Control Mode Select"]
    #[inline(always)]
    pub const fn set_sopcm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u8) & 0x07) << 1usize);
    }
    #[doc = "Sub Operating Power Control Mode Transition Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sopcmtsf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Sub Operating Power Control Mode Transition Status Flag"]
    #[inline(always)]
    pub const fn set_sopcmtsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Sopccr {
    #[inline(always)]
    fn default() -> Sopccr {
        Sopccr(0)
    }
}
impl core::fmt::Debug for Sopccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sopccr")
            .field("sopcm", &self.sopcm())
            .field("reserved", &self.reserved())
            .field("sopcmtsf", &self.sopcmtsf())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sopccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sopccr {{ sopcm: {=bool:?}, reserved: {=u8:?}, sopcmtsf: {=bool:?}, reserved_2: {=u8:?} }}",
            self.sopcm(),
            self.reserved(),
            self.sopcmtsf(),
            self.reserved_2()
        )
    }
}
#[doc = "Sub-Clock Oscillator Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccr(pub u8);
impl Sosccr {
    #[doc = "Sub-Clock Oscillator Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn sostp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Sub-Clock Oscillator Stop"]
    #[inline(always)]
    pub const fn set_sostp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Sosccr {
    #[inline(always)]
    fn default() -> Sosccr {
        Sosccr(0)
    }
}
impl core::fmt::Debug for Sosccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sosccr")
            .field("sostp", &self.sostp())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sosccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sosccr {{ sostp: {=bool:?}, reserved: {=u8:?} }}",
            self.sostp(),
            self.reserved()
        )
    }
}
#[doc = "System Control OCD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syocdcr(pub u8);
impl Syocdcr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
    }
    #[doc = "Debugger Enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Debugger Enable bit"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Syocdcr {
    #[inline(always)]
    fn default() -> Syocdcr {
        Syocdcr(0)
    }
}
impl core::fmt::Debug for Syocdcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syocdcr")
            .field("reserved", &self.reserved())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syocdcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Syocdcr {{ reserved: {=u8:?}, dbgen: {=bool:?} }}",
            self.reserved(),
            self.dbgen()
        )
    }
}
#[doc = "Trace Clock Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trckcr(pub u8);
impl Trckcr {
    #[doc = "Trace Clock operating frequency select"]
    #[must_use]
    #[inline(always)]
    pub const fn trck(&self) -> super::vals::Trck {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trck::from_bits(val as u8)
    }
    #[doc = "Trace Clock operating frequency select"]
    #[inline(always)]
    pub const fn set_trck(&mut self, val: super::vals::Trck) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
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
    #[doc = "Trace Clock operating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn trcken(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Trace Clock operating enable"]
    #[inline(always)]
    pub const fn set_trcken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Trckcr {
    #[inline(always)]
    fn default() -> Trckcr {
        Trckcr(0)
    }
}
impl core::fmt::Debug for Trckcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trckcr")
            .field("trck", &self.trck())
            .field("reserved", &self.reserved())
            .field("trcken", &self.trcken())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trckcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trckcr {{ trck: {:?}, reserved: {=u8:?}, trcken: {=bool:?} }}",
            self.trck(),
            self.reserved(),
            self.trcken()
        )
    }
}
#[doc = "USB Clock Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbckcr(pub u8);
impl Usbckcr {
    #[doc = "USB Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn usbclksel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB Clock Source Select"]
    #[inline(always)]
    pub const fn set_usbclksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Usbckcr {
    #[inline(always)]
    fn default() -> Usbckcr {
        Usbckcr(0)
    }
}
impl core::fmt::Debug for Usbckcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbckcr")
            .field("usbclksel", &self.usbclksel())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbckcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbckcr {{ usbclksel: {=bool:?}, reserved: {=u8:?} }}",
            self.usbclksel(),
            self.reserved()
        )
    }
}
#[doc = "VBATT Backup Register \\[%s\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtbkr(pub u8);
impl Vbtbkr {
    #[doc = "VBTBKR is a 512-byte readable/writable register to store data powered by VBATT. The value of this register is retained even when VCC is not powered but VBATT is powered. VBTBKR is initialized by VBATT selected voltage power-on-reset."]
    #[must_use]
    #[inline(always)]
    pub const fn vbtbkr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "VBTBKR is a 512-byte readable/writable register to store data powered by VBATT. The value of this register is retained even when VCC is not powered but VBATT is powered. VBTBKR is initialized by VBATT selected voltage power-on-reset."]
    #[inline(always)]
    pub const fn set_vbtbkr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Vbtbkr {
    #[inline(always)]
    fn default() -> Vbtbkr {
        Vbtbkr(0)
    }
}
impl core::fmt::Debug for Vbtbkr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtbkr")
            .field("vbtbkr", &self.vbtbkr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtbkr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Vbtbkr {{ vbtbkr: {=u8:?} }}", self.vbtbkr())
    }
}
#[doc = "VBATT Comparator Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtcmpcr(pub u8);
impl Vbtcmpcr {
    #[doc = "VBATT pin low voltage detect circuit output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbtcmpe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT pin low voltage detect circuit output enable"]
    #[inline(always)]
    pub const fn set_vbtcmpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Vbtcmpcr {
    #[inline(always)]
    fn default() -> Vbtcmpcr {
        Vbtcmpcr(0)
    }
}
impl core::fmt::Debug for Vbtcmpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtcmpcr")
            .field("vbtcmpe", &self.vbtcmpe())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtcmpcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vbtcmpcr {{ vbtcmpe: {=bool:?}, reserved: {=u8:?} }}",
            self.vbtcmpe(),
            self.reserved()
        )
    }
}
#[doc = "VBATT Control Register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtcr1(pub u8);
impl Vbtcr1 {
    #[doc = "Battery Power supply Switch Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn bpwswstp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Power supply Switch Stop"]
    #[inline(always)]
    pub const fn set_bpwswstp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Vbtcr1 {
    #[inline(always)]
    fn default() -> Vbtcr1 {
        Vbtcr1(0)
    }
}
impl core::fmt::Debug for Vbtcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtcr1")
            .field("bpwswstp", &self.bpwswstp())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vbtcr1 {{ bpwswstp: {=bool:?}, reserved: {=u8:?} }}",
            self.bpwswstp(),
            self.reserved()
        )
    }
}
#[doc = "VBATT Control Register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtcr2(pub u8);
impl Vbtcr2 {
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
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "VBATT Pin Low Voltage Detect Enable Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn vbtlvden(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT Pin Low Voltage Detect Enable Bit"]
    #[inline(always)]
    pub const fn set_vbtlvden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "VBATT Pin Voltage Low Voltage Detect Level Select Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn vbtlvdlvl(&self) -> super::vals::Vbtlvdlvl {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Vbtlvdlvl::from_bits(val as u8)
    }
    #[doc = "VBATT Pin Voltage Low Voltage Detect Level Select Bit"]
    #[inline(always)]
    pub const fn set_vbtlvdlvl(&mut self, val: super::vals::Vbtlvdlvl) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u8) & 0x03) << 6usize);
    }
}
impl Default for Vbtcr2 {
    #[inline(always)]
    fn default() -> Vbtcr2 {
        Vbtcr2(0)
    }
}
impl core::fmt::Debug for Vbtcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtcr2")
            .field("reserved", &self.reserved())
            .field("vbtlvden", &self.vbtlvden())
            .field("reserved_2", &self.reserved_2())
            .field("vbtlvdlvl", &self.vbtlvdlvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vbtcr2 {{ reserved: {=u8:?}, vbtlvden: {=bool:?}, reserved_2: {=bool:?}, vbtlvdlvl: {:?} }}",
            self.reserved(),
            self.vbtlvden(),
            self.reserved_2(),
            self.vbtlvdlvl()
        )
    }
}
#[doc = "VBATT Input Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtictlr(pub u8);
impl Vbtictlr {
    #[doc = "VBATT Wakeup I/O 0 Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vch0inen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT Wakeup I/O 0 Input Enable"]
    #[inline(always)]
    pub const fn set_vch0inen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "VBATT Wakeup I/O 1 Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vch1inen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT Wakeup I/O 1 Input Enable"]
    #[inline(always)]
    pub const fn set_vch1inen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "VBATT Wakeup I/O 2 Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vch2inen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT Wakeup I/O 2 Input Enable"]
    #[inline(always)]
    pub const fn set_vch2inen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for Vbtictlr {
    #[inline(always)]
    fn default() -> Vbtictlr {
        Vbtictlr(0)
    }
}
impl core::fmt::Debug for Vbtictlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtictlr")
            .field("vch0inen", &self.vch0inen())
            .field("vch1inen", &self.vch1inen())
            .field("vch2inen", &self.vch2inen())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtictlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vbtictlr {{ vch0inen: {=bool:?}, vch1inen: {=bool:?}, vch2inen: {=bool:?}, reserved: {=u8:?} }}",
            self.vch0inen(),
            self.vch1inen(),
            self.vch2inen(),
            self.reserved()
        )
    }
}
#[doc = "VBATT Pin Low Voltage Detect Interrupt Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtlvdicr(pub u8);
impl Vbtlvdicr {
    #[doc = "VBATT Pin Low Voltage Detect Interrupt Enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn vbtlvdie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT Pin Low Voltage Detect Interrupt Enable bit"]
    #[inline(always)]
    pub const fn set_vbtlvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Pin Low Voltage Detect Interrupt Select bit"]
    #[must_use]
    #[inline(always)]
    pub const fn vbtlvdisel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Low Voltage Detect Interrupt Select bit"]
    #[inline(always)]
    pub const fn set_vbtlvdisel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for Vbtlvdicr {
    #[inline(always)]
    fn default() -> Vbtlvdicr {
        Vbtlvdicr(0)
    }
}
impl core::fmt::Debug for Vbtlvdicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtlvdicr")
            .field("vbtlvdie", &self.vbtlvdie())
            .field("vbtlvdisel", &self.vbtlvdisel())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtlvdicr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vbtlvdicr {{ vbtlvdie: {=bool:?}, vbtlvdisel: {=bool:?}, reserved: {=u8:?} }}",
            self.vbtlvdie(),
            self.vbtlvdisel(),
            self.reserved()
        )
    }
}
#[doc = "VBATT Output Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtoctlr(pub u8);
impl Vbtoctlr {
    #[doc = "VBATT Wakeup I/O 0 Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vch0oen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT Wakeup I/O 0 Output Enable"]
    #[inline(always)]
    pub const fn set_vch0oen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "VBATT Wakeup I/O 1 Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vch1oen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT Wakeup I/O 1 Output Enable"]
    #[inline(always)]
    pub const fn set_vch1oen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "VBATT Wakeup I/O 2 Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vch2oen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT Wakeup I/O 2 Output Enable"]
    #[inline(always)]
    pub const fn set_vch2oen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "VBATT Wakeup I/O 0 Output Level Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vout0lsel(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT Wakeup I/O 0 Output Level Selection"]
    #[inline(always)]
    pub const fn set_vout0lsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "VBATT Wakeup I/O 1 Output Level Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vout1lsel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT Wakeup I/O 1 Output Level Selection"]
    #[inline(always)]
    pub const fn set_vout1lsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "VBATT Wakeup I/O 2 Output Level Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vout2lsel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT Wakeup I/O 2 Output Level Selection"]
    #[inline(always)]
    pub const fn set_vout2lsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
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
impl Default for Vbtoctlr {
    #[inline(always)]
    fn default() -> Vbtoctlr {
        Vbtoctlr(0)
    }
}
impl core::fmt::Debug for Vbtoctlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtoctlr")
            .field("vch0oen", &self.vch0oen())
            .field("vch1oen", &self.vch1oen())
            .field("vch2oen", &self.vch2oen())
            .field("vout0lsel", &self.vout0lsel())
            .field("vout1lsel", &self.vout1lsel())
            .field("vout2lsel", &self.vout2lsel())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtoctlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vbtoctlr {{ vch0oen: {=bool:?}, vch1oen: {=bool:?}, vch2oen: {=bool:?}, vout0lsel: {=bool:?}, vout1lsel: {=bool:?}, vout2lsel: {=bool:?}, reserved: {=u8:?} }}",
            self.vch0oen(),
            self.vch1oen(),
            self.vch2oen(),
            self.vout0lsel(),
            self.vout1lsel(),
            self.vout2lsel(),
            self.reserved()
        )
    }
}
#[doc = "VBATT Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtsr(pub u8);
impl Vbtsr {
    #[doc = "VBAT_R Reset Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn vbtrdf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBAT_R Reset Detect Flag"]
    #[inline(always)]
    pub const fn set_vbtrdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "VBATT Battery Low voltage Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn vbtbldf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT Battery Low voltage Detect Flag"]
    #[inline(always)]
    pub const fn set_vbtbldf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
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
    #[doc = "VBATT_R Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn vbtrvld(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT_R Valid"]
    #[inline(always)]
    pub const fn set_vbtrvld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Vbtsr {
    #[inline(always)]
    fn default() -> Vbtsr {
        Vbtsr(0)
    }
}
impl core::fmt::Debug for Vbtsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtsr")
            .field("vbtrdf", &self.vbtrdf())
            .field("vbtbldf", &self.vbtbldf())
            .field("reserved", &self.reserved())
            .field("vbtrvld", &self.vbtrvld())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vbtsr {{ vbtrdf: {=bool:?}, vbtbldf: {=bool:?}, reserved: {=u8:?}, vbtrvld: {=bool:?}, reserved_2: {=u8:?} }}",
            self.vbtrdf(),
            self.vbtbldf(),
            self.reserved(),
            self.vbtrvld(),
            self.reserved_2()
        )
    }
}
#[doc = "VBATT Wakeup I/O 0 Output Trigger Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtwch0otsr(pub u8);
impl Vbtwch0otsr {
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
    #[doc = "VBATWIO0 Output VBATWIO1 Trigger Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch0vch1te(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO0 Output VBATWIO1 Trigger Enable"]
    #[inline(always)]
    pub const fn set_ch0vch1te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "VBATWIO0 Output VBATWIO2 Trigger Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch0vch2te(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO0 Output VBATWIO2 Trigger Enable"]
    #[inline(always)]
    pub const fn set_ch0vch2te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "VBATWIO0 Output RTC Periodic Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch0vrtcte(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO0 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    pub const fn set_ch0vrtcte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "VBATWIO0 Output RTC Alarm Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch0vrtcate(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO0 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    pub const fn set_ch0vrtcate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Vbtwch0otsr {
    #[inline(always)]
    fn default() -> Vbtwch0otsr {
        Vbtwch0otsr(0)
    }
}
impl core::fmt::Debug for Vbtwch0otsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtwch0otsr")
            .field("reserved", &self.reserved())
            .field("ch0vch1te", &self.ch0vch1te())
            .field("ch0vch2te", &self.ch0vch2te())
            .field("ch0vrtcte", &self.ch0vrtcte())
            .field("ch0vrtcate", &self.ch0vrtcate())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtwch0otsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vbtwch0otsr {{ reserved: {=bool:?}, ch0vch1te: {=bool:?}, ch0vch2te: {=bool:?}, ch0vrtcte: {=bool:?}, ch0vrtcate: {=bool:?}, reserved_2: {=u8:?} }}",
            self.reserved(),
            self.ch0vch1te(),
            self.ch0vch2te(),
            self.ch0vrtcte(),
            self.ch0vrtcate(),
            self.reserved_2()
        )
    }
}
#[doc = "VBATT Wakeup I/O 1 Output Trigger Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtwch1otsr(pub u8);
impl Vbtwch1otsr {
    #[doc = "VBATWIO1 Output VBATWIO0 Trigger Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch1vch0te(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO1 Output VBATWIO0 Trigger Enable"]
    #[inline(always)]
    pub const fn set_ch1vch0te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "VBATWIO1 Output VBATWIO2 Trigger Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch1vch2te(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO1 Output VBATWIO2 Trigger Enable"]
    #[inline(always)]
    pub const fn set_ch1vch2te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "VBATWIO1 Output RTC Periodic Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch1vrtcte(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO1 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    pub const fn set_ch1vrtcte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "VBATWIO1 Output RTC Alarm Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch1vrtcate(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO1 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    pub const fn set_ch1vrtcate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Vbtwch1otsr {
    #[inline(always)]
    fn default() -> Vbtwch1otsr {
        Vbtwch1otsr(0)
    }
}
impl core::fmt::Debug for Vbtwch1otsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtwch1otsr")
            .field("ch1vch0te", &self.ch1vch0te())
            .field("reserved", &self.reserved())
            .field("ch1vch2te", &self.ch1vch2te())
            .field("ch1vrtcte", &self.ch1vrtcte())
            .field("ch1vrtcate", &self.ch1vrtcate())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtwch1otsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vbtwch1otsr {{ ch1vch0te: {=bool:?}, reserved: {=bool:?}, ch1vch2te: {=bool:?}, ch1vrtcte: {=bool:?}, ch1vrtcate: {=bool:?}, reserved_2: {=u8:?} }}",
            self.ch1vch0te(),
            self.reserved(),
            self.ch1vch2te(),
            self.ch1vrtcte(),
            self.ch1vrtcate(),
            self.reserved_2()
        )
    }
}
#[doc = "VBATT Wakeup I/O 2 Output Trigger Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtwch2otsr(pub u8);
impl Vbtwch2otsr {
    #[doc = "VBATWIO2 Output VBATWIO0 Trigger Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch2vch0te(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO2 Output VBATWIO0 Trigger Enable"]
    #[inline(always)]
    pub const fn set_ch2vch0te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "VBATWIO2 Output VBATWIO1 Trigger Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch2vch1te(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO2 Output VBATWIO1 Trigger Enable"]
    #[inline(always)]
    pub const fn set_ch2vch1te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "VBATWIO2 Output RTC Periodic Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch2vrtcte(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO2 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    pub const fn set_ch2vrtcte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "VBATWIO2 Output RTC Alarm Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch2vrtcate(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO2 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    pub const fn set_ch2vrtcate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Vbtwch2otsr {
    #[inline(always)]
    fn default() -> Vbtwch2otsr {
        Vbtwch2otsr(0)
    }
}
impl core::fmt::Debug for Vbtwch2otsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtwch2otsr")
            .field("ch2vch0te", &self.ch2vch0te())
            .field("ch2vch1te", &self.ch2vch1te())
            .field("reserved", &self.reserved())
            .field("ch2vrtcte", &self.ch2vrtcte())
            .field("ch2vrtcate", &self.ch2vrtcate())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtwch2otsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vbtwch2otsr {{ ch2vch0te: {=bool:?}, ch2vch1te: {=bool:?}, reserved: {=bool:?}, ch2vrtcte: {=bool:?}, ch2vrtcate: {=bool:?}, reserved_2: {=u8:?} }}",
            self.ch2vch0te(),
            self.ch2vch1te(),
            self.reserved(),
            self.ch2vrtcte(),
            self.ch2vrtcate(),
            self.reserved_2()
        )
    }
}
#[doc = "VBATT Wakeup function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtwctlr(pub u8);
impl Vbtwctlr {
    #[doc = "VBATT wakeup enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vwen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT wakeup enable"]
    #[inline(always)]
    pub const fn set_vwen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Vbtwctlr {
    #[inline(always)]
    fn default() -> Vbtwctlr {
        Vbtwctlr(0)
    }
}
impl core::fmt::Debug for Vbtwctlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtwctlr")
            .field("vwen", &self.vwen())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtwctlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vbtwctlr {{ vwen: {=bool:?}, reserved: {=u8:?} }}",
            self.vwen(),
            self.reserved()
        )
    }
}
#[doc = "VBATT Wakeup Trigger source Edge Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtwegr(pub u8);
impl Vbtwegr {
    #[doc = "VBATWIO0 Wakeup Trigger Source Edge Select"]
    #[must_use]
    #[inline(always)]
    pub const fn vch0eg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO0 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub const fn set_vch0eg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "VBATWIO1 Wakeup Trigger Source Edge Select"]
    #[must_use]
    #[inline(always)]
    pub const fn vch1eg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO1 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub const fn set_vch1eg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "VBATWIO2 Wakeup Trigger Source Edge Select"]
    #[must_use]
    #[inline(always)]
    pub const fn vch2eg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO2 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub const fn set_vch2eg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for Vbtwegr {
    #[inline(always)]
    fn default() -> Vbtwegr {
        Vbtwegr(0)
    }
}
impl core::fmt::Debug for Vbtwegr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtwegr")
            .field("vch0eg", &self.vch0eg())
            .field("vch1eg", &self.vch1eg())
            .field("vch2eg", &self.vch2eg())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtwegr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vbtwegr {{ vch0eg: {=bool:?}, vch1eg: {=bool:?}, vch2eg: {=bool:?}, reserved: {=u8:?} }}",
            self.vch0eg(),
            self.vch1eg(),
            self.vch2eg(),
            self.reserved()
        )
    }
}
#[doc = "VBATT Wakeup trigger source Flag Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtwfr(pub u8);
impl Vbtwfr {
    #[doc = "VBATWIO0 Wakeup Trigger Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn vch0f(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO0 Wakeup Trigger Flag"]
    #[inline(always)]
    pub const fn set_vch0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "VBATWIO1 Wakeup Trigger Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn vch1f(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO1 Wakeup Trigger Flag"]
    #[inline(always)]
    pub const fn set_vch1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "VBATWIO2 Wakeup Trigger Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn vch2f(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO2 Wakeup Trigger Flag"]
    #[inline(always)]
    pub const fn set_vch2f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "VBATT RTC-Interval Wakeup Trigger Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn vrtcif(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT RTC-Interval Wakeup Trigger Flag"]
    #[inline(always)]
    pub const fn set_vrtcif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "VBATT RTC-Alarm Wakeup Trigger Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn vrtcaf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBATT RTC-Alarm Wakeup Trigger Flag"]
    #[inline(always)]
    pub const fn set_vrtcaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Vbtwfr {
    #[inline(always)]
    fn default() -> Vbtwfr {
        Vbtwfr(0)
    }
}
impl core::fmt::Debug for Vbtwfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtwfr")
            .field("vch0f", &self.vch0f())
            .field("vch1f", &self.vch1f())
            .field("vch2f", &self.vch2f())
            .field("vrtcif", &self.vrtcif())
            .field("vrtcaf", &self.vrtcaf())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtwfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vbtwfr {{ vch0f: {=bool:?}, vch1f: {=bool:?}, vch2f: {=bool:?}, vrtcif: {=bool:?}, vrtcaf: {=bool:?}, reserved: {=u8:?} }}",
            self.vch0f(),
            self.vch1f(),
            self.vch2f(),
            self.vrtcif(),
            self.vrtcaf(),
            self.reserved()
        )
    }
}
#[doc = "VBATT Wakeup Trigger source Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtwter(pub u8);
impl Vbtwter {
    #[doc = "VBATWIO0 Pin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vch0e(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO0 Pin Enable"]
    #[inline(always)]
    pub const fn set_vch0e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "VBATWIO1 Pin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vch1e(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO1 Pin Enable"]
    #[inline(always)]
    pub const fn set_vch1e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "VBATWIO2 Pin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vch2e(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VBATWIO2 Pin Enable"]
    #[inline(always)]
    pub const fn set_vch2e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "RTC Periodic Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vrtcie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RTC Periodic Signal Enable"]
    #[inline(always)]
    pub const fn set_vrtcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "RTC Alarm Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vrtcae(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RTC Alarm Signal Enable"]
    #[inline(always)]
    pub const fn set_vrtcae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Vbtwter {
    #[inline(always)]
    fn default() -> Vbtwter {
        Vbtwter(0)
    }
}
impl core::fmt::Debug for Vbtwter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vbtwter")
            .field("vch0e", &self.vch0e())
            .field("vch1e", &self.vch1e())
            .field("vch2e", &self.vch2e())
            .field("vrtcie", &self.vrtcie())
            .field("vrtcae", &self.vrtcae())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vbtwter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vbtwter {{ vch0e: {=bool:?}, vch1e: {=bool:?}, vch2e: {=bool:?}, vrtcie: {=bool:?}, vrtcae: {=bool:?}, reserved: {=u8:?} }}",
            self.vch0e(),
            self.vch1e(),
            self.vch2e(),
            self.vrtcie(),
            self.vrtcae(),
            self.reserved()
        )
    }
}
