#[doc = "Debug Stop Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgstopcr(pub u32);
impl Dbgstopcr {
    #[doc = "Mask bit for IWDT reset/interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgstop_iwdt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for IWDT reset/interrupt"]
    #[inline(always)]
    pub const fn set_dbgstop_iwdt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for WDT reset/interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgstop_wdt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for WDT reset/interrupt"]
    #[inline(always)]
    pub const fn set_dbgstop_wdt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "These bits are read as 00000000000000. The write value should be 00000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "These bits are read as 00000000000000. The write value should be 00000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask) b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask) b16: Mask bit for LVD0 reset (0:enable / 1:Mask)"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgstop_lvd(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask) b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask) b16: Mask bit for LVD0 reset (0:enable / 1:Mask)"]
    #[inline(always)]
    pub const fn set_dbgstop_lvd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "Mask bit for RAM parity error reset/interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgstop_rper(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for RAM parity error reset/interrupt"]
    #[inline(always)]
    pub const fn set_dbgstop_rper(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Mask bit for RAM ECC error reset/interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgstop_reccr(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub const fn set_dbgstop_reccr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for Dbgstopcr {
    #[inline(always)]
    fn default() -> Dbgstopcr {
        Dbgstopcr(0)
    }
}
impl core::fmt::Debug for Dbgstopcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgstopcr")
            .field("dbgstop_iwdt", &self.dbgstop_iwdt())
            .field("dbgstop_wdt", &self.dbgstop_wdt())
            .field("reserved", &self.reserved())
            .field("dbgstop_lvd", &self.dbgstop_lvd())
            .field("reserved_2", &self.reserved_2())
            .field("dbgstop_rper", &self.dbgstop_rper())
            .field("dbgstop_reccr", &self.dbgstop_reccr())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgstopcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dbgstopcr {{ dbgstop_iwdt: {=bool:?}, dbgstop_wdt: {=bool:?}, reserved: {=u16:?}, dbgstop_lvd: {=u8:?}, reserved_2: {=u8:?}, dbgstop_rper: {=bool:?}, dbgstop_reccr: {=bool:?}, reserved_3: {=u8:?} }}",
            self.dbgstop_iwdt(),
            self.dbgstop_wdt(),
            self.reserved(),
            self.dbgstop_lvd(),
            self.reserved_2(),
            self.dbgstop_rper(),
            self.dbgstop_reccr(),
            self.reserved_3()
        )
    }
}
#[doc = "Debug Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgstr(pub u32);
impl Dbgstr {
    #[doc = "These bits are read as 0000000000000000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "These bits are read as 0000000000000000000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "Debug power-up request"]
    #[must_use]
    #[inline(always)]
    pub const fn cdbgpwrupreq(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Debug power-up request"]
    #[inline(always)]
    pub const fn set_cdbgpwrupreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Debug power-up acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn cdbgpwrupack(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Debug power-up acknowledge"]
    #[inline(always)]
    pub const fn set_cdbgpwrupack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "These bits are read as 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Dbgstr {
    #[inline(always)]
    fn default() -> Dbgstr {
        Dbgstr(0)
    }
}
impl core::fmt::Debug for Dbgstr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgstr")
            .field("reserved", &self.reserved())
            .field("cdbgpwrupreq", &self.cdbgpwrupreq())
            .field("cdbgpwrupack", &self.cdbgpwrupack())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgstr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dbgstr {{ reserved: {=u32:?}, cdbgpwrupreq: {=bool:?}, cdbgpwrupack: {=bool:?}, reserved_2: {=u8:?} }}",
            self.reserved(),
            self.cdbgpwrupreq(),
            self.cdbgpwrupack(),
            self.reserved_2()
        )
    }
}
#[doc = "Trace Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tracectr(pub u32);
impl Tracectr {
    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Enable bit for halt request by ETB full"]
    #[must_use]
    #[inline(always)]
    pub const fn enetbfull(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable bit for halt request by ETB full"]
    #[inline(always)]
    pub const fn set_enetbfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tracectr {
    #[inline(always)]
    fn default() -> Tracectr {
        Tracectr(0)
    }
}
impl core::fmt::Debug for Tracectr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tracectr")
            .field("reserved", &self.reserved())
            .field("enetbfull", &self.enetbfull())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tracectr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tracectr {{ reserved: {=u32:?}, enetbfull: {=bool:?} }}",
            self.reserved(),
            self.enetbfull()
        )
    }
}
