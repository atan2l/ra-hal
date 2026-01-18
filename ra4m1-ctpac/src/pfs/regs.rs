#[doc = "P00%s Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P000pfsBy(pub u8);
impl P000pfsBy {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
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
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
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
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: bool) {
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
impl Default for P000pfsBy {
    #[inline(always)]
    fn default() -> P000pfsBy {
        P000pfsBy(0)
    }
}
impl core::fmt::Debug for P000pfsBy {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P000pfsBy")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("reserved", &self.reserved())
            .field("pcr", &self.pcr())
            .field("reserved_2", &self.reserved_2())
            .field("ncodr", &self.ncodr())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P000pfsBy {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P000pfsBy {{ podr: {=bool:?}, pidr: {=bool:?}, pdr: {=bool:?}, reserved: {=bool:?}, pcr: {=bool:?}, reserved_2: {=bool:?}, ncodr: {=bool:?}, reserved_3: {=bool:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.reserved(),
            self.pcr(),
            self.reserved_2(),
            self.ncodr(),
            self.reserved_3()
        )
    }
}
#[doc = "P00%s Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P000pfsHa(pub u16);
impl P000pfsHa {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
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
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
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
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u16) & 0x07) << 7usize);
    }
    #[doc = "Port Drive Capability"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u16) & 0x07) << 11usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for P000pfsHa {
    #[inline(always)]
    fn default() -> P000pfsHa {
        P000pfsHa(0)
    }
}
impl core::fmt::Debug for P000pfsHa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P000pfsHa")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("reserved", &self.reserved())
            .field("pcr", &self.pcr())
            .field("reserved_2", &self.reserved_2())
            .field("ncodr", &self.ncodr())
            .field("reserved_3", &self.reserved_3())
            .field("dscr", &self.dscr())
            .field("reserved_4", &self.reserved_4())
            .field("isel", &self.isel())
            .field("asel", &self.asel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P000pfsHa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P000pfsHa {{ podr: {=bool:?}, pidr: {=bool:?}, pdr: {=bool:?}, reserved: {=bool:?}, pcr: {=bool:?}, reserved_2: {=bool:?}, ncodr: {=bool:?}, reserved_3: {=u8:?}, dscr: {=bool:?}, reserved_4: {=u8:?}, isel: {=bool:?}, asel: {=bool:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.reserved(),
            self.pcr(),
            self.reserved_2(),
            self.ncodr(),
            self.reserved_3(),
            self.dscr(),
            self.reserved_4(),
            self.isel(),
            self.asel()
        )
    }
}
#[doc = "P108 Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P108pfs(pub u32);
impl P108pfs {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[doc = "Port Drive Capability"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Event on Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn eor(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Event on Failing"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Failing"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Mode Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pmr(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub const fn set_pmr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_5(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "Port Function Select These bits select the peripheral function. For individual pin functions, see the setting table."]
    #[must_use]
    #[inline(always)]
    pub const fn psel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Port Function Select These bits select the peripheral function. For individual pin functions, see the setting table."]
    #[inline(always)]
    pub const fn set_psel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_6(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for P108pfs {
    #[inline(always)]
    fn default() -> P108pfs {
        P108pfs(0)
    }
}
impl core::fmt::Debug for P108pfs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P108pfs")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("reserved", &self.reserved())
            .field("pcr", &self.pcr())
            .field("reserved_2", &self.reserved_2())
            .field("ncodr", &self.ncodr())
            .field("reserved_3", &self.reserved_3())
            .field("dscr", &self.dscr())
            .field("reserved_4", &self.reserved_4())
            .field("eor", &self.eor())
            .field("eof", &self.eof())
            .field("isel", &self.isel())
            .field("asel", &self.asel())
            .field("pmr", &self.pmr())
            .field("reserved_5", &self.reserved_5())
            .field("psel", &self.psel())
            .field("reserved_6", &self.reserved_6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P108pfs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P108pfs {{ podr: {=bool:?}, pidr: {=bool:?}, pdr: {=bool:?}, reserved: {=bool:?}, pcr: {=bool:?}, reserved_2: {=bool:?}, ncodr: {=bool:?}, reserved_3: {=u8:?}, dscr: {=bool:?}, reserved_4: {=bool:?}, eor: {=bool:?}, eof: {=bool:?}, isel: {=bool:?}, asel: {=bool:?}, pmr: {=bool:?}, reserved_5: {=u8:?}, psel: {=u8:?}, reserved_6: {=u8:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.reserved(),
            self.pcr(),
            self.reserved_2(),
            self.ncodr(),
            self.reserved_3(),
            self.dscr(),
            self.reserved_4(),
            self.eor(),
            self.eof(),
            self.isel(),
            self.asel(),
            self.pmr(),
            self.reserved_5(),
            self.psel(),
            self.reserved_6()
        )
    }
}
#[doc = "P108 Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P108pfsBy(pub u8);
impl P108pfsBy {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
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
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
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
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: bool) {
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
impl Default for P108pfsBy {
    #[inline(always)]
    fn default() -> P108pfsBy {
        P108pfsBy(0)
    }
}
impl core::fmt::Debug for P108pfsBy {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P108pfsBy")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("reserved", &self.reserved())
            .field("pcr", &self.pcr())
            .field("reserved_2", &self.reserved_2())
            .field("ncodr", &self.ncodr())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P108pfsBy {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P108pfsBy {{ podr: {=bool:?}, pidr: {=bool:?}, pdr: {=bool:?}, reserved: {=bool:?}, pcr: {=bool:?}, reserved_2: {=bool:?}, ncodr: {=bool:?}, reserved_3: {=bool:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.reserved(),
            self.pcr(),
            self.reserved_2(),
            self.ncodr(),
            self.reserved_3()
        )
    }
}
#[doc = "P108 Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P108pfsHa(pub u16);
impl P108pfsHa {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
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
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
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
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u16) & 0x07) << 7usize);
    }
    #[doc = "Port Drive Capability"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Event on Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn eor(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Event on Failing"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Failing"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for P108pfsHa {
    #[inline(always)]
    fn default() -> P108pfsHa {
        P108pfsHa(0)
    }
}
impl core::fmt::Debug for P108pfsHa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P108pfsHa")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("reserved", &self.reserved())
            .field("pcr", &self.pcr())
            .field("reserved_2", &self.reserved_2())
            .field("ncodr", &self.ncodr())
            .field("reserved_3", &self.reserved_3())
            .field("dscr", &self.dscr())
            .field("reserved_4", &self.reserved_4())
            .field("eor", &self.eor())
            .field("eof", &self.eof())
            .field("isel", &self.isel())
            .field("asel", &self.asel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P108pfsHa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P108pfsHa {{ podr: {=bool:?}, pidr: {=bool:?}, pdr: {=bool:?}, reserved: {=bool:?}, pcr: {=bool:?}, reserved_2: {=bool:?}, ncodr: {=bool:?}, reserved_3: {=u8:?}, dscr: {=bool:?}, reserved_4: {=bool:?}, eor: {=bool:?}, eof: {=bool:?}, isel: {=bool:?}, asel: {=bool:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.reserved(),
            self.pcr(),
            self.reserved_2(),
            self.ncodr(),
            self.reserved_3(),
            self.dscr(),
            self.reserved_4(),
            self.eor(),
            self.eof(),
            self.isel(),
            self.asel()
        )
    }
}
#[doc = "P109 Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P109pfs(pub u32);
impl P109pfs {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[doc = "Port Drive Capability"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Event on Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn eor(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Event on Failing"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Failing"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Mode Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pmr(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub const fn set_pmr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_5(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "Port Function Select These bits select the peripheral function. For individual pin functions, see the setting table."]
    #[must_use]
    #[inline(always)]
    pub const fn psel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Port Function Select These bits select the peripheral function. For individual pin functions, see the setting table."]
    #[inline(always)]
    pub const fn set_psel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_6(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for P109pfs {
    #[inline(always)]
    fn default() -> P109pfs {
        P109pfs(0)
    }
}
impl core::fmt::Debug for P109pfs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P109pfs")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("reserved", &self.reserved())
            .field("pcr", &self.pcr())
            .field("reserved_2", &self.reserved_2())
            .field("ncodr", &self.ncodr())
            .field("reserved_3", &self.reserved_3())
            .field("dscr", &self.dscr())
            .field("reserved_4", &self.reserved_4())
            .field("eor", &self.eor())
            .field("eof", &self.eof())
            .field("isel", &self.isel())
            .field("asel", &self.asel())
            .field("pmr", &self.pmr())
            .field("reserved_5", &self.reserved_5())
            .field("psel", &self.psel())
            .field("reserved_6", &self.reserved_6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P109pfs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P109pfs {{ podr: {=bool:?}, pidr: {=bool:?}, pdr: {=bool:?}, reserved: {=bool:?}, pcr: {=bool:?}, reserved_2: {=bool:?}, ncodr: {=bool:?}, reserved_3: {=u8:?}, dscr: {=bool:?}, reserved_4: {=bool:?}, eor: {=bool:?}, eof: {=bool:?}, isel: {=bool:?}, asel: {=bool:?}, pmr: {=bool:?}, reserved_5: {=u8:?}, psel: {=u8:?}, reserved_6: {=u8:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.reserved(),
            self.pcr(),
            self.reserved_2(),
            self.ncodr(),
            self.reserved_3(),
            self.dscr(),
            self.reserved_4(),
            self.eor(),
            self.eof(),
            self.isel(),
            self.asel(),
            self.pmr(),
            self.reserved_5(),
            self.psel(),
            self.reserved_6()
        )
    }
}
#[doc = "P109 Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P109pfsBy(pub u8);
impl P109pfsBy {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
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
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
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
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: bool) {
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
impl Default for P109pfsBy {
    #[inline(always)]
    fn default() -> P109pfsBy {
        P109pfsBy(0)
    }
}
impl core::fmt::Debug for P109pfsBy {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P109pfsBy")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("reserved", &self.reserved())
            .field("pcr", &self.pcr())
            .field("reserved_2", &self.reserved_2())
            .field("ncodr", &self.ncodr())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P109pfsBy {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P109pfsBy {{ podr: {=bool:?}, pidr: {=bool:?}, pdr: {=bool:?}, reserved: {=bool:?}, pcr: {=bool:?}, reserved_2: {=bool:?}, ncodr: {=bool:?}, reserved_3: {=bool:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.reserved(),
            self.pcr(),
            self.reserved_2(),
            self.ncodr(),
            self.reserved_3()
        )
    }
}
#[doc = "P109 Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P109pfsHa(pub u16);
impl P109pfsHa {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
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
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
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
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u16) & 0x07) << 7usize);
    }
    #[doc = "Port Drive Capability"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Event on Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn eor(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Event on Failing"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Failing"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for P109pfsHa {
    #[inline(always)]
    fn default() -> P109pfsHa {
        P109pfsHa(0)
    }
}
impl core::fmt::Debug for P109pfsHa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P109pfsHa")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("reserved", &self.reserved())
            .field("pcr", &self.pcr())
            .field("reserved_2", &self.reserved_2())
            .field("ncodr", &self.ncodr())
            .field("reserved_3", &self.reserved_3())
            .field("dscr", &self.dscr())
            .field("reserved_4", &self.reserved_4())
            .field("eor", &self.eor())
            .field("eof", &self.eof())
            .field("isel", &self.isel())
            .field("asel", &self.asel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P109pfsHa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P109pfsHa {{ podr: {=bool:?}, pidr: {=bool:?}, pdr: {=bool:?}, reserved: {=bool:?}, pcr: {=bool:?}, reserved_2: {=bool:?}, ncodr: {=bool:?}, reserved_3: {=u8:?}, dscr: {=bool:?}, reserved_4: {=bool:?}, eor: {=bool:?}, eof: {=bool:?}, isel: {=bool:?}, asel: {=bool:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.reserved(),
            self.pcr(),
            self.reserved_2(),
            self.ncodr(),
            self.reserved_3(),
            self.dscr(),
            self.reserved_4(),
            self.eor(),
            self.eof(),
            self.isel(),
            self.asel()
        )
    }
}
#[doc = "P201 Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201pfs(pub u32);
impl P201pfs {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[doc = "Drive Strength Control Register"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Event on Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn eor(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Event on Falling"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Mode Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pmr(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub const fn set_pmr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_5(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "Port Function Select These bits select the peripheral function. For individual pin functions, see the setting table."]
    #[must_use]
    #[inline(always)]
    pub const fn psel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Port Function Select These bits select the peripheral function. For individual pin functions, see the setting table."]
    #[inline(always)]
    pub const fn set_psel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_6(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for P201pfs {
    #[inline(always)]
    fn default() -> P201pfs {
        P201pfs(0)
    }
}
impl core::fmt::Debug for P201pfs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P201pfs")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("reserved", &self.reserved())
            .field("pcr", &self.pcr())
            .field("reserved_2", &self.reserved_2())
            .field("ncodr", &self.ncodr())
            .field("reserved_3", &self.reserved_3())
            .field("dscr", &self.dscr())
            .field("reserved_4", &self.reserved_4())
            .field("eor", &self.eor())
            .field("eof", &self.eof())
            .field("isel", &self.isel())
            .field("asel", &self.asel())
            .field("pmr", &self.pmr())
            .field("reserved_5", &self.reserved_5())
            .field("psel", &self.psel())
            .field("reserved_6", &self.reserved_6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P201pfs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P201pfs {{ podr: {=bool:?}, pidr: {=bool:?}, pdr: {=bool:?}, reserved: {=bool:?}, pcr: {=bool:?}, reserved_2: {=bool:?}, ncodr: {=bool:?}, reserved_3: {=u8:?}, dscr: {=bool:?}, reserved_4: {=bool:?}, eor: {=bool:?}, eof: {=bool:?}, isel: {=bool:?}, asel: {=bool:?}, pmr: {=bool:?}, reserved_5: {=u8:?}, psel: {=u8:?}, reserved_6: {=u8:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.reserved(),
            self.pcr(),
            self.reserved_2(),
            self.ncodr(),
            self.reserved_3(),
            self.dscr(),
            self.reserved_4(),
            self.eor(),
            self.eof(),
            self.isel(),
            self.asel(),
            self.pmr(),
            self.reserved_5(),
            self.psel(),
            self.reserved_6()
        )
    }
}
#[doc = "P201 Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201pfsBy(pub u8);
impl P201pfsBy {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
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
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
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
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: bool) {
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
impl Default for P201pfsBy {
    #[inline(always)]
    fn default() -> P201pfsBy {
        P201pfsBy(0)
    }
}
impl core::fmt::Debug for P201pfsBy {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P201pfsBy")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("reserved", &self.reserved())
            .field("pcr", &self.pcr())
            .field("reserved_2", &self.reserved_2())
            .field("ncodr", &self.ncodr())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P201pfsBy {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P201pfsBy {{ podr: {=bool:?}, pidr: {=bool:?}, pdr: {=bool:?}, reserved: {=bool:?}, pcr: {=bool:?}, reserved_2: {=bool:?}, ncodr: {=bool:?}, reserved_3: {=bool:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.reserved(),
            self.pcr(),
            self.reserved_2(),
            self.ncodr(),
            self.reserved_3()
        )
    }
}
#[doc = "P201 Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201pfsHa(pub u16);
impl P201pfsHa {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
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
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
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
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u16) & 0x07) << 7usize);
    }
    #[doc = "Drive Strength Control Register"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Event on Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn eor(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Event on Falling"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for P201pfsHa {
    #[inline(always)]
    fn default() -> P201pfsHa {
        P201pfsHa(0)
    }
}
impl core::fmt::Debug for P201pfsHa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P201pfsHa")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("reserved", &self.reserved())
            .field("pcr", &self.pcr())
            .field("reserved_2", &self.reserved_2())
            .field("ncodr", &self.ncodr())
            .field("reserved_3", &self.reserved_3())
            .field("dscr", &self.dscr())
            .field("reserved_4", &self.reserved_4())
            .field("eor", &self.eor())
            .field("eof", &self.eof())
            .field("isel", &self.isel())
            .field("asel", &self.asel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P201pfsHa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P201pfsHa {{ podr: {=bool:?}, pidr: {=bool:?}, pdr: {=bool:?}, reserved: {=bool:?}, pcr: {=bool:?}, reserved_2: {=bool:?}, ncodr: {=bool:?}, reserved_3: {=u8:?}, dscr: {=bool:?}, reserved_4: {=bool:?}, eor: {=bool:?}, eof: {=bool:?}, isel: {=bool:?}, asel: {=bool:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.reserved(),
            self.pcr(),
            self.reserved_2(),
            self.ncodr(),
            self.reserved_3(),
            self.dscr(),
            self.reserved_4(),
            self.eor(),
            self.eof(),
            self.isel(),
            self.asel()
        )
    }
}
#[doc = "P408 Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P408pfs(pub u32);
impl P408pfs {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[doc = "Drive Strength Control Register"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Drive Strength Control Register"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub const fn set_dscr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Event on Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn eor(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Event on Falling"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Mode Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pmr(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub const fn set_pmr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "Port Function Select These bits select the peripheral function. For individual pin functions, see the setting table."]
    #[must_use]
    #[inline(always)]
    pub const fn psel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Port Function Select These bits select the peripheral function. For individual pin functions, see the setting table."]
    #[inline(always)]
    pub const fn set_psel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_5(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for P408pfs {
    #[inline(always)]
    fn default() -> P408pfs {
        P408pfs(0)
    }
}
impl core::fmt::Debug for P408pfs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P408pfs")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("reserved", &self.reserved())
            .field("pcr", &self.pcr())
            .field("reserved_2", &self.reserved_2())
            .field("ncodr", &self.ncodr())
            .field("reserved_3", &self.reserved_3())
            .field("dscr", &self.dscr())
            .field("dscr1", &self.dscr1())
            .field("eor", &self.eor())
            .field("eof", &self.eof())
            .field("isel", &self.isel())
            .field("asel", &self.asel())
            .field("pmr", &self.pmr())
            .field("reserved_4", &self.reserved_4())
            .field("psel", &self.psel())
            .field("reserved_5", &self.reserved_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P408pfs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P408pfs {{ podr: {=bool:?}, pidr: {=bool:?}, pdr: {=bool:?}, reserved: {=bool:?}, pcr: {=bool:?}, reserved_2: {=bool:?}, ncodr: {=bool:?}, reserved_3: {=u8:?}, dscr: {=bool:?}, dscr1: {=bool:?}, eor: {=bool:?}, eof: {=bool:?}, isel: {=bool:?}, asel: {=bool:?}, pmr: {=bool:?}, reserved_4: {=u8:?}, psel: {=u8:?}, reserved_5: {=u8:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.reserved(),
            self.pcr(),
            self.reserved_2(),
            self.ncodr(),
            self.reserved_3(),
            self.dscr(),
            self.dscr1(),
            self.eor(),
            self.eof(),
            self.isel(),
            self.asel(),
            self.pmr(),
            self.reserved_4(),
            self.psel(),
            self.reserved_5()
        )
    }
}
#[doc = "P408 Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P408pfsBy(pub u8);
impl P408pfsBy {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
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
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
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
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: bool) {
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
impl Default for P408pfsBy {
    #[inline(always)]
    fn default() -> P408pfsBy {
        P408pfsBy(0)
    }
}
impl core::fmt::Debug for P408pfsBy {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P408pfsBy")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("reserved", &self.reserved())
            .field("pcr", &self.pcr())
            .field("reserved_2", &self.reserved_2())
            .field("ncodr", &self.ncodr())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P408pfsBy {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P408pfsBy {{ podr: {=bool:?}, pidr: {=bool:?}, pdr: {=bool:?}, reserved: {=bool:?}, pcr: {=bool:?}, reserved_2: {=bool:?}, ncodr: {=bool:?}, reserved_3: {=bool:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.reserved(),
            self.pcr(),
            self.reserved_2(),
            self.ncodr(),
            self.reserved_3()
        )
    }
}
#[doc = "P408 Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P408pfsHa(pub u16);
impl P408pfsHa {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
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
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
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
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u16) & 0x07) << 7usize);
    }
    #[doc = "Drive Strength Control Register"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Drive Strength Control Register"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub const fn set_dscr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Event on Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn eor(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Event on Falling"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for P408pfsHa {
    #[inline(always)]
    fn default() -> P408pfsHa {
        P408pfsHa(0)
    }
}
impl core::fmt::Debug for P408pfsHa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P408pfsHa")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("reserved", &self.reserved())
            .field("pcr", &self.pcr())
            .field("reserved_2", &self.reserved_2())
            .field("ncodr", &self.ncodr())
            .field("reserved_3", &self.reserved_3())
            .field("dscr", &self.dscr())
            .field("dscr1", &self.dscr1())
            .field("eor", &self.eor())
            .field("eof", &self.eof())
            .field("isel", &self.isel())
            .field("asel", &self.asel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P408pfsHa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P408pfsHa {{ podr: {=bool:?}, pidr: {=bool:?}, pdr: {=bool:?}, reserved: {=bool:?}, pcr: {=bool:?}, reserved_2: {=bool:?}, ncodr: {=bool:?}, reserved_3: {=u8:?}, dscr: {=bool:?}, dscr1: {=bool:?}, eor: {=bool:?}, eof: {=bool:?}, isel: {=bool:?}, asel: {=bool:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.reserved(),
            self.pcr(),
            self.reserved_2(),
            self.ncodr(),
            self.reserved_3(),
            self.dscr(),
            self.dscr1(),
            self.eor(),
            self.eof(),
            self.isel(),
            self.asel()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port0Pfs(pub u32);
impl Port0Pfs {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> super::vals::PortLevel {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PortLevel::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::PortLevel) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::PortLevel {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PortLevel::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::PortLevel) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::PortDirection {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PortDirection::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::PortDirection) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "N-Channel Open Drain Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ncodr(&self) -> super::vals::OutputType {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::OutputType::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::OutputType) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Drive Capability"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr(&self) -> super::vals::PortDrive {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PortDrive::from_bits(val as u8)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: super::vals::PortDrive) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Mode Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pmr(&self) -> super::vals::PortMode {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::PortMode::from_bits(val as u8)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub const fn set_pmr(&mut self, val: super::vals::PortMode) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn psel(&self) -> super::vals::PortFunction {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::PortFunction::from_bits(val as u8)
    }
    #[inline(always)]
    pub const fn set_psel(&mut self, val: super::vals::PortFunction) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Port0Pfs {
    #[inline(always)]
    fn default() -> Port0Pfs {
        Port0Pfs(0)
    }
}
impl core::fmt::Debug for Port0Pfs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Port0Pfs")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("pcr", &self.pcr())
            .field("ncodr", &self.ncodr())
            .field("dscr", &self.dscr())
            .field("isel", &self.isel())
            .field("asel", &self.asel())
            .field("pmr", &self.pmr())
            .field("psel", &self.psel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Port0Pfs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Port0Pfs {{ podr: {:?}, pidr: {:?}, pdr: {:?}, pcr: {=bool:?}, ncodr: {:?}, dscr: {:?}, isel: {=bool:?}, asel: {=bool:?}, pmr: {:?}, psel: {:?} }}",
            self.podr(),
            self.pidr(),
            self.pdr(),
            self.pcr(),
            self.ncodr(),
            self.dscr(),
            self.isel(),
            self.asel(),
            self.pmr(),
            self.psel()
        )
    }
}
