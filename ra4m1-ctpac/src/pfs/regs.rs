#[doc = "P00%s Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P000pfs(pub u32);
impl P000pfs {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> super::vals::P000pfsPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P000pfsPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P000pfsPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P000pfsPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P000pfsPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P000pfsPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P000pfsPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P000pfsPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P000pfsPdr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
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
    pub const fn pcr(&self) -> super::vals::P000pfsPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P000pfsPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P000pfsPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P000pfsNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P000pfsNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P000pfsNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
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
    pub const fn dscr(&self) -> super::vals::P000pfsDscr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P000pfsDscr::from_bits(val as u8)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: super::vals::P000pfsDscr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
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
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> super::vals::P000pfsIsel {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::P000pfsIsel::from_bits(val as u8)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: super::vals::P000pfsIsel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> super::vals::P000pfsAsel {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::P000pfsAsel::from_bits(val as u8)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: super::vals::P000pfsAsel) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Mode Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pmr(&self) -> super::vals::P000pfsPmr {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::P000pfsPmr::from_bits(val as u8)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub const fn set_pmr(&mut self, val: super::vals::P000pfsPmr) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
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
    #[doc = "Port Function Select These bits select the peripheral function. For individual pin functions, see the MPC table"]
    #[must_use]
    #[inline(always)]
    pub const fn psel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Port Function Select These bits select the peripheral function. For individual pin functions, see the MPC table"]
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
impl Default for P000pfs {
    #[inline(always)]
    fn default() -> P000pfs {
        P000pfs(0)
    }
}
impl core::fmt::Debug for P000pfs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P000pfs")
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
            .field("pmr", &self.pmr())
            .field("reserved_5", &self.reserved_5())
            .field("psel", &self.psel())
            .field("reserved_6", &self.reserved_6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P000pfs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P000pfs {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=u8:?}, dscr: {:?}, reserved_4: {=u8:?}, isel: {:?}, asel: {:?}, pmr: {:?}, reserved_5: {=u8:?}, psel: {=u8:?}, reserved_6: {=u8:?} }}",
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
            self.asel(),
            self.pmr(),
            self.reserved_5(),
            self.psel(),
            self.reserved_6()
        )
    }
}
#[doc = "P00%s Pin Function Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P000pfsBy(pub u8);
impl P000pfsBy {
    #[doc = "Port Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn podr(&self) -> super::vals::P000pfsByPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P000pfsByPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P000pfsByPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P000pfsByPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P000pfsByPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P000pfsByPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P000pfsByPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P000pfsByPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P000pfsByPdr) {
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
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> super::vals::P000pfsByPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P000pfsByPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P000pfsByPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P000pfsByNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P000pfsByNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P000pfsByNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
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
            "P000pfsBy {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=bool:?} }}",
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
    pub const fn podr(&self) -> super::vals::P000pfsHaPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P000pfsHaPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P000pfsHaPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P000pfsHaPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P000pfsHaPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P000pfsHaPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P000pfsHaPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P000pfsHaPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P000pfsHaPdr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
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
    pub const fn pcr(&self) -> super::vals::P000pfsHaPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P000pfsHaPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P000pfsHaPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P000pfsHaNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P000pfsHaNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P000pfsHaNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
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
    pub const fn dscr(&self) -> super::vals::P000pfsHaDscr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P000pfsHaDscr::from_bits(val as u8)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: super::vals::P000pfsHaDscr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
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
    pub const fn isel(&self) -> super::vals::P000pfsHaIsel {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::P000pfsHaIsel::from_bits(val as u8)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: super::vals::P000pfsHaIsel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> super::vals::P000pfsHaAsel {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::P000pfsHaAsel::from_bits(val as u8)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: super::vals::P000pfsHaAsel) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "P000pfsHa {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=u8:?}, dscr: {:?}, reserved_4: {=u8:?}, isel: {:?}, asel: {:?} }}",
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
    pub const fn podr(&self) -> super::vals::P108pfsPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P108pfsPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P108pfsPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P108pfsPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P108pfsPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P108pfsPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P108pfsPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P108pfsPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P108pfsPdr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
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
    pub const fn pcr(&self) -> super::vals::P108pfsPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P108pfsPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P108pfsPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P108pfsNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P108pfsNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P108pfsNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
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
    pub const fn dscr(&self) -> super::vals::P108pfsDscr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P108pfsDscr::from_bits(val as u8)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: super::vals::P108pfsDscr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
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
    pub const fn eor(&self) -> super::vals::P108pfsEor {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::P108pfsEor::from_bits(val as u8)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: super::vals::P108pfsEor) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Event on Failing"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> super::vals::P108pfsEof {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::P108pfsEof::from_bits(val as u8)
    }
    #[doc = "Event on Failing"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: super::vals::P108pfsEof) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> super::vals::P108pfsIsel {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::P108pfsIsel::from_bits(val as u8)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: super::vals::P108pfsIsel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> super::vals::P108pfsAsel {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::P108pfsAsel::from_bits(val as u8)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: super::vals::P108pfsAsel) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Mode Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pmr(&self) -> super::vals::P108pfsPmr {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::P108pfsPmr::from_bits(val as u8)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub const fn set_pmr(&mut self, val: super::vals::P108pfsPmr) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
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
            "P108pfs {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=u8:?}, dscr: {:?}, reserved_4: {=bool:?}, eor: {:?}, eof: {:?}, isel: {:?}, asel: {:?}, pmr: {:?}, reserved_5: {=u8:?}, psel: {=u8:?}, reserved_6: {=u8:?} }}",
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
    pub const fn podr(&self) -> super::vals::P108pfsByPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P108pfsByPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P108pfsByPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P108pfsByPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P108pfsByPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P108pfsByPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P108pfsByPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P108pfsByPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P108pfsByPdr) {
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
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> super::vals::P108pfsByPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P108pfsByPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P108pfsByPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P108pfsByNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P108pfsByNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P108pfsByNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
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
            "P108pfsBy {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=bool:?} }}",
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
    pub const fn podr(&self) -> super::vals::P108pfsHaPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P108pfsHaPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P108pfsHaPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P108pfsHaPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P108pfsHaPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P108pfsHaPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P108pfsHaPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P108pfsHaPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P108pfsHaPdr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
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
    pub const fn pcr(&self) -> super::vals::P108pfsHaPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P108pfsHaPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P108pfsHaPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P108pfsHaNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P108pfsHaNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P108pfsHaNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
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
    pub const fn dscr(&self) -> super::vals::P108pfsHaDscr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P108pfsHaDscr::from_bits(val as u8)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: super::vals::P108pfsHaDscr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
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
    pub const fn eor(&self) -> super::vals::P108pfsHaEor {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::P108pfsHaEor::from_bits(val as u8)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: super::vals::P108pfsHaEor) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Event on Failing"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> super::vals::P108pfsHaEof {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::P108pfsHaEof::from_bits(val as u8)
    }
    #[doc = "Event on Failing"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: super::vals::P108pfsHaEof) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> super::vals::P108pfsHaIsel {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::P108pfsHaIsel::from_bits(val as u8)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: super::vals::P108pfsHaIsel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> super::vals::P108pfsHaAsel {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::P108pfsHaAsel::from_bits(val as u8)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: super::vals::P108pfsHaAsel) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "P108pfsHa {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=u8:?}, dscr: {:?}, reserved_4: {=bool:?}, eor: {:?}, eof: {:?}, isel: {:?}, asel: {:?} }}",
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
    pub const fn podr(&self) -> super::vals::P109pfsPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P109pfsPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P109pfsPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P109pfsPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P109pfsPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P109pfsPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P109pfsPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P109pfsPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P109pfsPdr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
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
    pub const fn pcr(&self) -> super::vals::P109pfsPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P109pfsPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P109pfsPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P109pfsNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P109pfsNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P109pfsNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
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
    pub const fn dscr(&self) -> super::vals::P109pfsDscr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P109pfsDscr::from_bits(val as u8)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: super::vals::P109pfsDscr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
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
    pub const fn eor(&self) -> super::vals::P109pfsEor {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::P109pfsEor::from_bits(val as u8)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: super::vals::P109pfsEor) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Event on Failing"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> super::vals::P109pfsEof {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::P109pfsEof::from_bits(val as u8)
    }
    #[doc = "Event on Failing"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: super::vals::P109pfsEof) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> super::vals::P109pfsIsel {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::P109pfsIsel::from_bits(val as u8)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: super::vals::P109pfsIsel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> super::vals::P109pfsAsel {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::P109pfsAsel::from_bits(val as u8)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: super::vals::P109pfsAsel) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Mode Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pmr(&self) -> super::vals::P109pfsPmr {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::P109pfsPmr::from_bits(val as u8)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub const fn set_pmr(&mut self, val: super::vals::P109pfsPmr) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
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
            "P109pfs {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=u8:?}, dscr: {:?}, reserved_4: {=bool:?}, eor: {:?}, eof: {:?}, isel: {:?}, asel: {:?}, pmr: {:?}, reserved_5: {=u8:?}, psel: {=u8:?}, reserved_6: {=u8:?} }}",
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
    pub const fn podr(&self) -> super::vals::P109pfsByPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P109pfsByPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P109pfsByPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P109pfsByPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P109pfsByPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P109pfsByPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P109pfsByPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P109pfsByPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P109pfsByPdr) {
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
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> super::vals::P109pfsByPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P109pfsByPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P109pfsByPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P109pfsByNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P109pfsByNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P109pfsByNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
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
            "P109pfsBy {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=bool:?} }}",
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
    pub const fn podr(&self) -> super::vals::P109pfsHaPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P109pfsHaPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P109pfsHaPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P109pfsHaPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P109pfsHaPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P109pfsHaPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P109pfsHaPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P109pfsHaPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P109pfsHaPdr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
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
    pub const fn pcr(&self) -> super::vals::P109pfsHaPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P109pfsHaPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P109pfsHaPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P109pfsHaNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P109pfsHaNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P109pfsHaNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
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
    pub const fn dscr(&self) -> super::vals::P109pfsHaDscr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P109pfsHaDscr::from_bits(val as u8)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: super::vals::P109pfsHaDscr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
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
    pub const fn eor(&self) -> super::vals::P109pfsHaEor {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::P109pfsHaEor::from_bits(val as u8)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: super::vals::P109pfsHaEor) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Event on Failing"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> super::vals::P109pfsHaEof {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::P109pfsHaEof::from_bits(val as u8)
    }
    #[doc = "Event on Failing"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: super::vals::P109pfsHaEof) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> super::vals::P109pfsHaIsel {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::P109pfsHaIsel::from_bits(val as u8)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: super::vals::P109pfsHaIsel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> super::vals::P109pfsHaAsel {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::P109pfsHaAsel::from_bits(val as u8)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: super::vals::P109pfsHaAsel) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "P109pfsHa {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=u8:?}, dscr: {:?}, reserved_4: {=bool:?}, eor: {:?}, eof: {:?}, isel: {:?}, asel: {:?} }}",
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
    pub const fn podr(&self) -> super::vals::P201pfsPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P201pfsPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P201pfsPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P201pfsPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P201pfsPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P201pfsPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P201pfsPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P201pfsPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P201pfsPdr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
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
    pub const fn pcr(&self) -> super::vals::P201pfsPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P201pfsPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P201pfsPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P201pfsNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P201pfsNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P201pfsNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
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
    pub const fn dscr(&self) -> super::vals::P201pfsDscr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P201pfsDscr::from_bits(val as u8)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: super::vals::P201pfsDscr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
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
    pub const fn eor(&self) -> super::vals::P201pfsEor {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::P201pfsEor::from_bits(val as u8)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: super::vals::P201pfsEor) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Event on Falling"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> super::vals::P201pfsEof {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::P201pfsEof::from_bits(val as u8)
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: super::vals::P201pfsEof) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> super::vals::P201pfsIsel {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::P201pfsIsel::from_bits(val as u8)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: super::vals::P201pfsIsel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> super::vals::P201pfsAsel {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::P201pfsAsel::from_bits(val as u8)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: super::vals::P201pfsAsel) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Mode Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pmr(&self) -> super::vals::P201pfsPmr {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::P201pfsPmr::from_bits(val as u8)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub const fn set_pmr(&mut self, val: super::vals::P201pfsPmr) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
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
            "P201pfs {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=u8:?}, dscr: {:?}, reserved_4: {=bool:?}, eor: {:?}, eof: {:?}, isel: {:?}, asel: {:?}, pmr: {:?}, reserved_5: {=u8:?}, psel: {=u8:?}, reserved_6: {=u8:?} }}",
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
    pub const fn podr(&self) -> super::vals::P201pfsByPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P201pfsByPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P201pfsByPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P201pfsByPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P201pfsByPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P201pfsByPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P201pfsByPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P201pfsByPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P201pfsByPdr) {
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
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> super::vals::P201pfsByPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P201pfsByPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P201pfsByPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P201pfsByNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P201pfsByNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P201pfsByNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
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
            "P201pfsBy {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=bool:?} }}",
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
    pub const fn podr(&self) -> super::vals::P201pfsHaPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P201pfsHaPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P201pfsHaPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P201pfsHaPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P201pfsHaPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P201pfsHaPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P201pfsHaPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P201pfsHaPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P201pfsHaPdr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
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
    pub const fn pcr(&self) -> super::vals::P201pfsHaPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P201pfsHaPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P201pfsHaPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P201pfsHaNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P201pfsHaNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P201pfsHaNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
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
    pub const fn dscr(&self) -> super::vals::P201pfsHaDscr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P201pfsHaDscr::from_bits(val as u8)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: super::vals::P201pfsHaDscr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
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
    pub const fn eor(&self) -> super::vals::P201pfsHaEor {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::P201pfsHaEor::from_bits(val as u8)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: super::vals::P201pfsHaEor) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Event on Falling"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> super::vals::P201pfsHaEof {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::P201pfsHaEof::from_bits(val as u8)
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: super::vals::P201pfsHaEof) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> super::vals::P201pfsHaIsel {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::P201pfsHaIsel::from_bits(val as u8)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: super::vals::P201pfsHaIsel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> super::vals::P201pfsHaAsel {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::P201pfsHaAsel::from_bits(val as u8)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: super::vals::P201pfsHaAsel) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "P201pfsHa {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=u8:?}, dscr: {:?}, reserved_4: {=bool:?}, eor: {:?}, eof: {:?}, isel: {:?}, asel: {:?} }}",
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
    pub const fn podr(&self) -> super::vals::P408pfsPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P408pfsPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P408pfsPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P408pfsPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P408pfsPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P408pfsPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P408pfsPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P408pfsPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P408pfsPdr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
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
    pub const fn pcr(&self) -> super::vals::P408pfsPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P408pfsPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P408pfsPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P408pfsNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P408pfsNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P408pfsNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
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
    pub const fn dscr(&self) -> super::vals::P408pfsDscr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P408pfsDscr::from_bits(val as u8)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: super::vals::P408pfsDscr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Drive Strength Control Register"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr1(&self) -> super::vals::P408pfsDscr1 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::P408pfsDscr1::from_bits(val as u8)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub const fn set_dscr1(&mut self, val: super::vals::P408pfsDscr1) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Event on Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn eor(&self) -> super::vals::P408pfsEor {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::P408pfsEor::from_bits(val as u8)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: super::vals::P408pfsEor) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Event on Falling"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> super::vals::P408pfsEof {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::P408pfsEof::from_bits(val as u8)
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: super::vals::P408pfsEof) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> super::vals::P408pfsIsel {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::P408pfsIsel::from_bits(val as u8)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: super::vals::P408pfsIsel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> super::vals::P408pfsAsel {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::P408pfsAsel::from_bits(val as u8)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: super::vals::P408pfsAsel) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Mode Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pmr(&self) -> super::vals::P408pfsPmr {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::P408pfsPmr::from_bits(val as u8)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub const fn set_pmr(&mut self, val: super::vals::P408pfsPmr) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
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
            "P408pfs {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=u8:?}, dscr: {:?}, dscr1: {:?}, eor: {:?}, eof: {:?}, isel: {:?}, asel: {:?}, pmr: {:?}, reserved_4: {=u8:?}, psel: {=u8:?}, reserved_5: {=u8:?} }}",
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
    pub const fn podr(&self) -> super::vals::P408pfsByPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P408pfsByPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P408pfsByPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P408pfsByPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P408pfsByPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P408pfsByPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P408pfsByPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P408pfsByPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P408pfsByPdr) {
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
    #[doc = "Pull-up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> super::vals::P408pfsByPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P408pfsByPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P408pfsByPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P408pfsByNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P408pfsByNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P408pfsByNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
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
            "P408pfsBy {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=bool:?} }}",
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
    pub const fn podr(&self) -> super::vals::P408pfsHaPodr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P408pfsHaPodr::from_bits(val as u8)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub const fn set_podr(&mut self, val: super::vals::P408pfsHaPodr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Port Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn pidr(&self) -> super::vals::P408pfsHaPidr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P408pfsHaPidr::from_bits(val as u8)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub const fn set_pidr(&mut self, val: super::vals::P408pfsHaPidr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdr(&self) -> super::vals::P408pfsHaPdr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P408pfsHaPdr::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_pdr(&mut self, val: super::vals::P408pfsHaPdr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
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
    pub const fn pcr(&self) -> super::vals::P408pfsHaPcr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::P408pfsHaPcr::from_bits(val as u8)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::P408pfsHaPcr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
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
    pub const fn ncodr(&self) -> super::vals::P408pfsHaNcodr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::P408pfsHaNcodr::from_bits(val as u8)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub const fn set_ncodr(&mut self, val: super::vals::P408pfsHaNcodr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
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
    pub const fn dscr(&self) -> super::vals::P408pfsHaDscr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P408pfsHaDscr::from_bits(val as u8)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub const fn set_dscr(&mut self, val: super::vals::P408pfsHaDscr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Drive Strength Control Register"]
    #[must_use]
    #[inline(always)]
    pub const fn dscr1(&self) -> super::vals::P408pfsHaDscr1 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::P408pfsHaDscr1::from_bits(val as u8)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub const fn set_dscr1(&mut self, val: super::vals::P408pfsHaDscr1) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Event on Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn eor(&self) -> super::vals::P408pfsHaEor {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::P408pfsHaEor::from_bits(val as u8)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub const fn set_eor(&mut self, val: super::vals::P408pfsHaEor) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Event on Falling"]
    #[must_use]
    #[inline(always)]
    pub const fn eof(&self) -> super::vals::P408pfsHaEof {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::P408pfsHaEof::from_bits(val as u8)
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub const fn set_eof(&mut self, val: super::vals::P408pfsHaEof) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "IRQ input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> super::vals::P408pfsHaIsel {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::P408pfsHaIsel::from_bits(val as u8)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: super::vals::P408pfsHaIsel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Analog Input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asel(&self) -> super::vals::P408pfsHaAsel {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::P408pfsHaAsel::from_bits(val as u8)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub const fn set_asel(&mut self, val: super::vals::P408pfsHaAsel) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "P408pfsHa {{ podr: {:?}, pidr: {:?}, pdr: {:?}, reserved: {=bool:?}, pcr: {:?}, reserved_2: {=bool:?}, ncodr: {:?}, reserved_3: {=u8:?}, dscr: {:?}, dscr1: {:?}, eor: {:?}, eof: {:?}, isel: {:?}, asel: {:?} }}",
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
