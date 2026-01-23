#[doc = "SPI Bit Rate Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spbr(pub u8);
impl Spbr {
    #[doc = "SPBR sets the bit rate in master mode."]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SPBR sets the bit rate in master mode."]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Spbr {
    #[inline(always)]
    fn default() -> Spbr {
        Spbr(0)
    }
}
impl core::fmt::Debug for Spbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spbr").field("spr", &self.spr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Spbr {{ spr: {=u8:?} }}", self.spr())
    }
}
#[doc = "SPI Clock Delay Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spckd(pub u8);
impl Spckd {
    #[doc = "RSPCK Delay Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn sckdl(&self) -> super::vals::Sckdl {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Sckdl::from_bits(val as u8)
    }
    #[doc = "RSPCK Delay Setting"]
    #[inline(always)]
    pub const fn set_sckdl(&mut self, val: super::vals::Sckdl) {
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
impl Default for Spckd {
    #[inline(always)]
    fn default() -> Spckd {
        Spckd(0)
    }
}
impl core::fmt::Debug for Spckd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spckd")
            .field("sckdl", &self.sckdl())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spckd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spckd {{ sckdl: {:?}, reserved: {=u8:?} }}",
            self.sckdl(),
            self.reserved()
        )
    }
}
#[doc = "SPI Command Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcmd0(pub u16);
impl Spcmd0 {
    #[doc = "RSPCK Phase Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn cpha(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RSPCK Phase Setting"]
    #[inline(always)]
    pub const fn set_cpha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "RSPCK Polarity Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn cpol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RSPCK Polarity Setting"]
    #[inline(always)]
    pub const fn set_cpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Bit Rate Division Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn brdv(&self) -> super::vals::Brdv {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Brdv::from_bits(val as u8)
    }
    #[doc = "Bit Rate Division Setting"]
    #[inline(always)]
    pub const fn set_brdv(&mut self, val: super::vals::Brdv) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "SSL Signal Assertion Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn ssla(&self) -> super::vals::Ssla {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Ssla::from_bits(val as u8)
    }
    #[doc = "SSL Signal Assertion Setting"]
    #[inline(always)]
    pub const fn set_ssla(&mut self, val: super::vals::Ssla) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
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
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "RSPI Data Length Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn spb(&self) -> super::vals::Spb {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Spb::from_bits(val as u8)
    }
    #[doc = "RSPI Data Length Setting"]
    #[inline(always)]
    pub const fn set_spb(&mut self, val: super::vals::Spb) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "RSPI LSB First"]
    #[must_use]
    #[inline(always)]
    pub const fn lsbf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "RSPI LSB First"]
    #[inline(always)]
    pub const fn set_lsbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "RSPI Next-Access Delay Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spnden(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "RSPI Next-Access Delay Enable"]
    #[inline(always)]
    pub const fn set_spnden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "SSL Negation Delay Setting Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn slnden(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SSL Negation Delay Setting Enable"]
    #[inline(always)]
    pub const fn set_slnden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "RSPCK Delay Setting Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sckden(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "RSPCK Delay Setting Enable"]
    #[inline(always)]
    pub const fn set_sckden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Spcmd0 {
    #[inline(always)]
    fn default() -> Spcmd0 {
        Spcmd0(0)
    }
}
impl core::fmt::Debug for Spcmd0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spcmd0")
            .field("cpha", &self.cpha())
            .field("cpol", &self.cpol())
            .field("brdv", &self.brdv())
            .field("ssla", &self.ssla())
            .field("reserved", &self.reserved())
            .field("spb", &self.spb())
            .field("lsbf", &self.lsbf())
            .field("spnden", &self.spnden())
            .field("slnden", &self.slnden())
            .field("sckden", &self.sckden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spcmd0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spcmd0 {{ cpha: {=bool:?}, cpol: {=bool:?}, brdv: {:?}, ssla: {:?}, reserved: {=bool:?}, spb: {:?}, lsbf: {=bool:?}, spnden: {=bool:?}, slnden: {=bool:?}, sckden: {=bool:?} }}",
            self.cpha(),
            self.cpol(),
            self.brdv(),
            self.ssla(),
            self.reserved(),
            self.spb(),
            self.lsbf(),
            self.spnden(),
            self.slnden(),
            self.sckden()
        )
    }
}
#[doc = "SPI Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcr(pub u8);
impl Spcr {
    #[doc = "SPI Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn spms(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Mode Select"]
    #[inline(always)]
    pub const fn set_spms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Communications Operating Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn txmd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Communications Operating Mode Select"]
    #[inline(always)]
    pub const fn set_txmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Mode Fault Error Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn modfen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Fault Error Detection Enable"]
    #[inline(always)]
    pub const fn set_modfen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "SPI Master/Slave Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mstr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Master/Slave Mode Select"]
    #[inline(always)]
    pub const fn set_mstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "SPI Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn speie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_speie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Transmit Buffer Empty Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sptie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sptie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "SPI Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Function Enable"]
    #[inline(always)]
    pub const fn set_spe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "SPI Receive Buffer Full Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sprie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sprie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Spcr {
    #[inline(always)]
    fn default() -> Spcr {
        Spcr(0)
    }
}
impl core::fmt::Debug for Spcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spcr")
            .field("spms", &self.spms())
            .field("txmd", &self.txmd())
            .field("modfen", &self.modfen())
            .field("mstr", &self.mstr())
            .field("speie", &self.speie())
            .field("sptie", &self.sptie())
            .field("spe", &self.spe())
            .field("sprie", &self.sprie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spcr {{ spms: {=bool:?}, txmd: {=bool:?}, modfen: {=bool:?}, mstr: {=bool:?}, speie: {=bool:?}, sptie: {=bool:?}, spe: {=bool:?}, sprie: {=bool:?} }}",
            self.spms(),
            self.txmd(),
            self.modfen(),
            self.mstr(),
            self.speie(),
            self.sptie(),
            self.spe(),
            self.sprie()
        )
    }
}
#[doc = "SPI Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcr2(pub u8);
impl Spcr2 {
    #[doc = "Parity Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sppe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Enable"]
    #[inline(always)]
    pub const fn set_sppe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Parity Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn spoe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Mode"]
    #[inline(always)]
    pub const fn set_spoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "SPI Idle Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spiie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Idle Interrupt Enable"]
    #[inline(always)]
    pub const fn set_spiie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Parity Self-Testing"]
    #[must_use]
    #[inline(always)]
    pub const fn pte(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Self-Testing"]
    #[inline(always)]
    pub const fn set_pte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "RSPCK Auto-Stop Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sckase(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RSPCK Auto-Stop Function Enable"]
    #[inline(always)]
    pub const fn set_sckase(&mut self, val: bool) {
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
impl Default for Spcr2 {
    #[inline(always)]
    fn default() -> Spcr2 {
        Spcr2(0)
    }
}
impl core::fmt::Debug for Spcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spcr2")
            .field("sppe", &self.sppe())
            .field("spoe", &self.spoe())
            .field("spiie", &self.spiie())
            .field("pte", &self.pte())
            .field("sckase", &self.sckase())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spcr2 {{ sppe: {=bool:?}, spoe: {=bool:?}, spiie: {=bool:?}, pte: {=bool:?}, sckase: {=bool:?}, reserved: {=u8:?} }}",
            self.sppe(),
            self.spoe(),
            self.spiie(),
            self.pte(),
            self.sckase(),
            self.reserved()
        )
    }
}
#[doc = "SPI Data Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spdcr(pub u8);
impl Spdcr {
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
    #[doc = "RSPI Receive/Transmit Data Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sprdtd(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RSPI Receive/Transmit Data Selection"]
    #[inline(always)]
    pub const fn set_sprdtd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "SPI Word Access/Halfword Access Specification"]
    #[must_use]
    #[inline(always)]
    pub const fn splw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Word Access/Halfword Access Specification"]
    #[inline(always)]
    pub const fn set_splw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
    }
    #[doc = "SPI Byte Access Specification (TN-RA*-A0033A/E)"]
    #[must_use]
    #[inline(always)]
    pub const fn spbyt(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Byte Access Specification (TN-RA*-A0033A/E)"]
    #[inline(always)]
    pub const fn set_spbyt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
}
impl Default for Spdcr {
    #[inline(always)]
    fn default() -> Spdcr {
        Spdcr(0)
    }
}
impl core::fmt::Debug for Spdcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spdcr")
            .field("reserved", &self.reserved())
            .field("reserved_2", &self.reserved_2())
            .field("sprdtd", &self.sprdtd())
            .field("splw", &self.splw())
            .field("reserved_3", &self.reserved_3())
            .field("spbyt", &self.spbyt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spdcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spdcr {{ reserved: {=u8:?}, reserved_2: {=u8:?}, sprdtd: {=bool:?}, splw: {=bool:?}, reserved_3: {=u8:?}, spbyt: {=bool:?} }}",
            self.reserved(),
            self.reserved_2(),
            self.sprdtd(),
            self.splw(),
            self.reserved_3(),
            self.spbyt()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spdcr2(pub u8);
impl Spdcr2 {
    #[doc = "Byte Swap Operating Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn bysw(&self) -> super::vals::Bysw {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Bysw::from_bits(val as u8)
    }
    #[doc = "Byte Swap Operating Mode Select"]
    #[inline(always)]
    pub const fn set_bysw(&mut self, val: super::vals::Bysw) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Serial data invert bit"]
    #[must_use]
    #[inline(always)]
    pub const fn sinv(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Serial data invert bit"]
    #[inline(always)]
    pub const fn set_sinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
}
impl Default for Spdcr2 {
    #[inline(always)]
    fn default() -> Spdcr2 {
        Spdcr2(0)
    }
}
impl core::fmt::Debug for Spdcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spdcr2")
            .field("bysw", &self.bysw())
            .field("sinv", &self.sinv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spdcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spdcr2 {{ bysw: {:?}, sinv: {=bool:?} }}",
            self.bysw(),
            self.sinv()
        )
    }
}
#[doc = "SPI Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spdr(pub u32);
impl Spdr {
    #[doc = "SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in word (SPDCR.SPLW=1), access SPDR."]
    #[must_use]
    #[inline(always)]
    pub const fn spdr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in word (SPDCR.SPLW=1), access SPDR."]
    #[inline(always)]
    pub const fn set_spdr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Spdr {
    #[inline(always)]
    fn default() -> Spdr {
        Spdr(0)
    }
}
impl core::fmt::Debug for Spdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spdr").field("spdr", &self.spdr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Spdr {{ spdr: {=u32:?} }}", self.spdr())
    }
}
#[doc = "SPI Data Register ( halfword access )"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpdrHa(pub u16);
impl SpdrHa {
    #[doc = "SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in halfword (SPDCR.SPLW=0), access SPDR_HA."]
    #[must_use]
    #[inline(always)]
    pub const fn spdr_ha(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in halfword (SPDCR.SPLW=0), access SPDR_HA."]
    #[inline(always)]
    pub const fn set_spdr_ha(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for SpdrHa {
    #[inline(always)]
    fn default() -> SpdrHa {
        SpdrHa(0)
    }
}
impl core::fmt::Debug for SpdrHa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpdrHa")
            .field("spdr_ha", &self.spdr_ha())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpdrHa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SpdrHa {{ spdr_ha: {=u16:?} }}", self.spdr_ha())
    }
}
#[doc = "SPI Next-Access Delay Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spnd(pub u8);
impl Spnd {
    #[doc = "SPI Next-Access Delay Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn spndl(&self) -> super::vals::Spndl {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Spndl::from_bits(val as u8)
    }
    #[doc = "SPI Next-Access Delay Setting"]
    #[inline(always)]
    pub const fn set_spndl(&mut self, val: super::vals::Spndl) {
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
impl Default for Spnd {
    #[inline(always)]
    fn default() -> Spnd {
        Spnd(0)
    }
}
impl core::fmt::Debug for Spnd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spnd")
            .field("spndl", &self.spndl())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spnd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spnd {{ spndl: {:?}, reserved: {=u8:?} }}",
            self.spndl(),
            self.reserved()
        )
    }
}
#[doc = "SPI Pin Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sppcr(pub u8);
impl Sppcr {
    #[doc = "RSPI Loopback"]
    #[must_use]
    #[inline(always)]
    pub const fn splp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RSPI Loopback"]
    #[inline(always)]
    pub const fn set_splp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "RSPI Loopback 2"]
    #[must_use]
    #[inline(always)]
    pub const fn splp2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RSPI Loopback 2"]
    #[inline(always)]
    pub const fn set_splp2(&mut self, val: bool) {
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
    #[doc = "MOSI Idle Fixed Value"]
    #[must_use]
    #[inline(always)]
    pub const fn moifv(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MOSI Idle Fixed Value"]
    #[inline(always)]
    pub const fn set_moifv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "MOSI Idle Value Fixing Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn moife(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "MOSI Idle Value Fixing Enable"]
    #[inline(always)]
    pub const fn set_moife(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
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
impl Default for Sppcr {
    #[inline(always)]
    fn default() -> Sppcr {
        Sppcr(0)
    }
}
impl core::fmt::Debug for Sppcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sppcr")
            .field("splp", &self.splp())
            .field("splp2", &self.splp2())
            .field("reserved", &self.reserved())
            .field("moifv", &self.moifv())
            .field("moife", &self.moife())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sppcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sppcr {{ splp: {=bool:?}, splp2: {=bool:?}, reserved: {=u8:?}, moifv: {=bool:?}, moife: {=bool:?}, reserved_2: {=u8:?} }}",
            self.splp(),
            self.splp2(),
            self.reserved(),
            self.moifv(),
            self.moife(),
            self.reserved_2()
        )
    }
}
#[doc = "SPI Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spsr(pub u8);
impl Spsr {
    #[doc = "Overrun Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Error Flag"]
    #[inline(always)]
    pub const fn set_ovrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "SPI Idle Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn idlnf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Idle Flag"]
    #[inline(always)]
    pub const fn set_idlnf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Mode Fault Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn modf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Fault Error Flag"]
    #[inline(always)]
    pub const fn set_modf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Parity Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn perf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub const fn set_perf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Underrun Error Flag (When MODF is 0, This bit is invalid.)"]
    #[must_use]
    #[inline(always)]
    pub const fn udrf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Underrun Error Flag (When MODF is 0, This bit is invalid.)"]
    #[inline(always)]
    pub const fn set_udrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "SPI Transmit Buffer Empty Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sptef(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Transmit Buffer Empty Flag"]
    #[inline(always)]
    pub const fn set_sptef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "SPI Receive Buffer Full Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sprf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Receive Buffer Full Flag"]
    #[inline(always)]
    pub const fn set_sprf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Spsr {
    #[inline(always)]
    fn default() -> Spsr {
        Spsr(0)
    }
}
impl core::fmt::Debug for Spsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spsr")
            .field("ovrf", &self.ovrf())
            .field("idlnf", &self.idlnf())
            .field("modf", &self.modf())
            .field("perf", &self.perf())
            .field("udrf", &self.udrf())
            .field("sptef", &self.sptef())
            .field("reserved", &self.reserved())
            .field("sprf", &self.sprf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spsr {{ ovrf: {=bool:?}, idlnf: {=bool:?}, modf: {=bool:?}, perf: {=bool:?}, udrf: {=bool:?}, sptef: {=bool:?}, reserved: {=bool:?}, sprf: {=bool:?} }}",
            self.ovrf(),
            self.idlnf(),
            self.modf(),
            self.perf(),
            self.udrf(),
            self.sptef(),
            self.reserved(),
            self.sprf()
        )
    }
}
#[doc = "SPI Slave Select Negation Delay Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sslnd(pub u8);
impl Sslnd {
    #[doc = "SSL Negation Delay Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn slndl(&self) -> super::vals::Slndl {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Slndl::from_bits(val as u8)
    }
    #[doc = "SSL Negation Delay Setting"]
    #[inline(always)]
    pub const fn set_slndl(&mut self, val: super::vals::Slndl) {
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
impl Default for Sslnd {
    #[inline(always)]
    fn default() -> Sslnd {
        Sslnd(0)
    }
}
impl core::fmt::Debug for Sslnd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sslnd")
            .field("slndl", &self.slndl())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sslnd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sslnd {{ slndl: {:?}, reserved: {=u8:?} }}",
            self.slndl(),
            self.reserved()
        )
    }
}
#[doc = "SPI Slave Select Polarity Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sslp(pub u8);
impl Sslp {
    #[doc = "SSL0 Signal Polarity Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn ssl0p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SSL0 Signal Polarity Setting"]
    #[inline(always)]
    pub const fn set_ssl0p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "SSL1 Signal Polarity Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn ssl1p(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SSL1 Signal Polarity Setting"]
    #[inline(always)]
    pub const fn set_ssl1p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "SSL2 Signal Polarity Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn ssl2p(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SSL2 Signal Polarity Setting"]
    #[inline(always)]
    pub const fn set_ssl2p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "SSL3 Signal Polarity Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn ssl3p(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SSL3 Signal Polarity Setting"]
    #[inline(always)]
    pub const fn set_ssl3p(&mut self, val: bool) {
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
impl Default for Sslp {
    #[inline(always)]
    fn default() -> Sslp {
        Sslp(0)
    }
}
impl core::fmt::Debug for Sslp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sslp")
            .field("ssl0p", &self.ssl0p())
            .field("ssl1p", &self.ssl1p())
            .field("ssl2p", &self.ssl2p())
            .field("ssl3p", &self.ssl3p())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sslp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sslp {{ ssl0p: {=bool:?}, ssl1p: {=bool:?}, ssl2p: {=bool:?}, ssl3p: {=bool:?}, reserved: {=u8:?} }}",
            self.ssl0p(),
            self.ssl1p(),
            self.ssl2p(),
            self.ssl3p(),
            self.reserved()
        )
    }
}
