#[doc = "I2C Bus Bit Rate High-Level Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icbrh(pub u8);
impl Icbrh {
    #[doc = "Bit Rate High-Level Period (High-level period of SCL clock)"]
    #[must_use]
    #[inline(always)]
    pub const fn brh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Bit Rate High-Level Period (High-level period of SCL clock)"]
    #[inline(always)]
    pub const fn set_brh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "These bits are read as 111. The write value should be 111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 111. The write value should be 111."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Icbrh {
    #[inline(always)]
    fn default() -> Icbrh {
        Icbrh(0)
    }
}
impl core::fmt::Debug for Icbrh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icbrh")
            .field("brh", &self.brh())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icbrh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icbrh {{ brh: {=u8:?}, reserved: {=u8:?} }}",
            self.brh(),
            self.reserved()
        )
    }
}
#[doc = "I2C Bus Bit Rate Low-Level Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icbrl(pub u8);
impl Icbrl {
    #[doc = "Bit Rate Low-Level Period (Low-level period of SCL clock)"]
    #[must_use]
    #[inline(always)]
    pub const fn brl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Bit Rate Low-Level Period (Low-level period of SCL clock)"]
    #[inline(always)]
    pub const fn set_brl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "These bits are read as 111. The write value should be 111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 111. The write value should be 111."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for Icbrl {
    #[inline(always)]
    fn default() -> Icbrl {
        Icbrl(0)
    }
}
impl core::fmt::Debug for Icbrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icbrl")
            .field("brl", &self.brl())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icbrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icbrl {{ brl: {=u8:?}, reserved: {=u8:?} }}",
            self.brl(),
            self.reserved()
        )
    }
}
#[doc = "I2C Bus Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iccr1(pub u8);
impl Iccr1 {
    #[doc = "SDA Line Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn sdai(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SDA Line Monitor"]
    #[inline(always)]
    pub const fn set_sdai(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "SCL Line Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn scli(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SCL Line Monitor"]
    #[inline(always)]
    pub const fn set_scli(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "SDA Output Control/Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn sdao(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SDA Output Control/Monitor"]
    #[inline(always)]
    pub const fn set_sdao(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "SCL Output Control/Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn sclo(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SCL Output Control/Monitor"]
    #[inline(always)]
    pub const fn set_sclo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "SCLO/SDAO Write Protect"]
    #[must_use]
    #[inline(always)]
    pub const fn sowp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SCLO/SDAO Write Protect"]
    #[inline(always)]
    pub const fn set_sowp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Extra SCL Clock Cycle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn clo(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Extra SCL Clock Cycle Output"]
    #[inline(always)]
    pub const fn set_clo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "I2C Bus Interface Internal Reset Note:If an internal reset is initiated using the IICRST bit for a bus hang-up occurred during communication with the master device in slave mode, the states may become different between the slave device and the master device (due to the difference in the bit counter information)."]
    #[must_use]
    #[inline(always)]
    pub const fn iicrst(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Bus Interface Internal Reset Note:If an internal reset is initiated using the IICRST bit for a bus hang-up occurred during communication with the master device in slave mode, the states may become different between the slave device and the master device (due to the difference in the bit counter information)."]
    #[inline(always)]
    pub const fn set_iicrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "I2C Bus Interface Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ice(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Bus Interface Enable"]
    #[inline(always)]
    pub const fn set_ice(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Iccr1 {
    #[inline(always)]
    fn default() -> Iccr1 {
        Iccr1(0)
    }
}
impl core::fmt::Debug for Iccr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iccr1")
            .field("sdai", &self.sdai())
            .field("scli", &self.scli())
            .field("sdao", &self.sdao())
            .field("sclo", &self.sclo())
            .field("sowp", &self.sowp())
            .field("clo", &self.clo())
            .field("iicrst", &self.iicrst())
            .field("ice", &self.ice())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iccr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iccr1 {{ sdai: {=bool:?}, scli: {=bool:?}, sdao: {=bool:?}, sclo: {=bool:?}, sowp: {=bool:?}, clo: {=bool:?}, iicrst: {=bool:?}, ice: {=bool:?} }}",
            self.sdai(),
            self.scli(),
            self.sdao(),
            self.sclo(),
            self.sowp(),
            self.clo(),
            self.iicrst(),
            self.ice()
        )
    }
}
#[doc = "I2C Bus Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iccr2(pub u8);
impl Iccr2 {
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
    #[doc = "Start Condition Issuance Request Set the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state)."]
    #[must_use]
    #[inline(always)]
    pub const fn st(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Start Condition Issuance Request Set the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state)."]
    #[inline(always)]
    pub const fn set_st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Restart Condition Issuance Request Note: Do not set the RS bit to 1 while issuing a stop condition."]
    #[must_use]
    #[inline(always)]
    pub const fn rs(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Restart Condition Issuance Request Note: Do not set the RS bit to 1 while issuing a stop condition."]
    #[inline(always)]
    pub const fn set_rs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Stop Condition Issuance Request Note: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state). Note: Do not set the SP bit to 1 while a restart condition is being issued."]
    #[must_use]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Condition Issuance Request Note: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state). Note: Do not set the SP bit to 1 while a restart condition is being issued."]
    #[inline(always)]
    pub const fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Transmit/Receive Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn trs(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit/Receive Mode"]
    #[inline(always)]
    pub const fn set_trs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Master/Slave Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mst(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Master/Slave Mode"]
    #[inline(always)]
    pub const fn set_mst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Bus Busy Detection Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bbsy(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Busy Detection Flag"]
    #[inline(always)]
    pub const fn set_bbsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Iccr2 {
    #[inline(always)]
    fn default() -> Iccr2 {
        Iccr2(0)
    }
}
impl core::fmt::Debug for Iccr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iccr2")
            .field("reserved", &self.reserved())
            .field("st", &self.st())
            .field("rs", &self.rs())
            .field("sp", &self.sp())
            .field("reserved_2", &self.reserved_2())
            .field("trs", &self.trs())
            .field("mst", &self.mst())
            .field("bbsy", &self.bbsy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iccr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iccr2 {{ reserved: {=bool:?}, st: {=bool:?}, rs: {=bool:?}, sp: {=bool:?}, reserved_2: {=bool:?}, trs: {=bool:?}, mst: {=bool:?}, bbsy: {=bool:?} }}",
            self.reserved(),
            self.st(),
            self.rs(),
            self.sp(),
            self.reserved_2(),
            self.trs(),
            self.mst(),
            self.bbsy()
        )
    }
}
#[doc = "I2C Bus Receive Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icdrr(pub u8);
impl Icdrr {
    #[doc = "8-bit register that stores the received data"]
    #[must_use]
    #[inline(always)]
    pub const fn icdrr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit register that stores the received data"]
    #[inline(always)]
    pub const fn set_icdrr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Icdrr {
    #[inline(always)]
    fn default() -> Icdrr {
        Icdrr(0)
    }
}
impl core::fmt::Debug for Icdrr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icdrr")
            .field("icdrr", &self.icdrr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icdrr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Icdrr {{ icdrr: {=u8:?} }}", self.icdrr())
    }
}
#[doc = "I2C Bus Transmit Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icdrt(pub u8);
impl Icdrt {
    #[doc = "8-bit read-write register that stores transmit data."]
    #[must_use]
    #[inline(always)]
    pub const fn icdrt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit read-write register that stores transmit data."]
    #[inline(always)]
    pub const fn set_icdrt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Icdrt {
    #[inline(always)]
    fn default() -> Icdrt {
        Icdrt(0)
    }
}
impl core::fmt::Debug for Icdrt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icdrt")
            .field("icdrt", &self.icdrt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icdrt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Icdrt {{ icdrt: {=u8:?} }}", self.icdrt())
    }
}
#[doc = "I2C Bus Function Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icfer(pub u8);
impl Icfer {
    #[doc = "Timeout Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tmoe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Function Enable"]
    #[inline(always)]
    pub const fn set_tmoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Master Arbitration-Lost Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn male(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Master Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub const fn set_male(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "NACK Transmission Arbitration-Lost Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nale(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NACK Transmission Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub const fn set_nale(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Slave Arbitration-Lost Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sale(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub const fn set_sale(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "NACK Reception Transfer Suspension Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nacke(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NACK Reception Transfer Suspension Enable"]
    #[inline(always)]
    pub const fn set_nacke(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Digital Noise Filter Circuit Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nfe(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Noise Filter Circuit Enable"]
    #[inline(always)]
    pub const fn set_nfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "SCL Synchronous Circuit Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn scle(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SCL Synchronous Circuit Enable"]
    #[inline(always)]
    pub const fn set_scle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
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
impl Default for Icfer {
    #[inline(always)]
    fn default() -> Icfer {
        Icfer(0)
    }
}
impl core::fmt::Debug for Icfer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icfer")
            .field("tmoe", &self.tmoe())
            .field("male", &self.male())
            .field("nale", &self.nale())
            .field("sale", &self.sale())
            .field("nacke", &self.nacke())
            .field("nfe", &self.nfe())
            .field("scle", &self.scle())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icfer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icfer {{ tmoe: {=bool:?}, male: {=bool:?}, nale: {=bool:?}, sale: {=bool:?}, nacke: {=bool:?}, nfe: {=bool:?}, scle: {=bool:?}, reserved: {=bool:?} }}",
            self.tmoe(),
            self.male(),
            self.nale(),
            self.sale(),
            self.nacke(),
            self.nfe(),
            self.scle(),
            self.reserved()
        )
    }
}
#[doc = "I2C Bus Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icier(pub u8);
impl Icier {
    #[doc = "Timeout Interrupt Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tmoie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Interrupt Request Enable"]
    #[inline(always)]
    pub const fn set_tmoie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Arbitration-Lost Interrupt Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration-Lost Interrupt Request Enable"]
    #[inline(always)]
    pub const fn set_alie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Start Condition Detection Interrupt Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn stie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Start Condition Detection Interrupt Request Enable"]
    #[inline(always)]
    pub const fn set_stie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Stop Condition Detection Interrupt Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Condition Detection Interrupt Request Enable"]
    #[inline(always)]
    pub const fn set_spie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "NACK Reception Interrupt Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nakie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NACK Reception Interrupt Request Enable"]
    #[inline(always)]
    pub const fn set_nakie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Receive Data Full Interrupt Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Full Interrupt Request Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Transmit End Interrupt Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit End Interrupt Request Enable"]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Transmit Data Empty Interrupt Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Empty Interrupt Request Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Icier {
    #[inline(always)]
    fn default() -> Icier {
        Icier(0)
    }
}
impl core::fmt::Debug for Icier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icier")
            .field("tmoie", &self.tmoie())
            .field("alie", &self.alie())
            .field("stie", &self.stie())
            .field("spie", &self.spie())
            .field("nakie", &self.nakie())
            .field("rie", &self.rie())
            .field("teie", &self.teie())
            .field("tie", &self.tie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icier {{ tmoie: {=bool:?}, alie: {=bool:?}, stie: {=bool:?}, spie: {=bool:?}, nakie: {=bool:?}, rie: {=bool:?}, teie: {=bool:?}, tie: {=bool:?} }}",
            self.tmoie(),
            self.alie(),
            self.stie(),
            self.spie(),
            self.nakie(),
            self.rie(),
            self.teie(),
            self.tie()
        )
    }
}
#[doc = "I2C Bus Mode Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icmr1(pub u8);
impl Icmr1 {
    #[doc = "Bit Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn bc(&self) -> super::vals::Bc {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Bc::from_bits(val as u8)
    }
    #[doc = "Bit Counter"]
    #[inline(always)]
    pub const fn set_bc(&mut self, val: super::vals::Bc) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u8) & 0x07) << 0usize);
    }
    #[doc = "BC Write Protect (This bit is read as 1.)"]
    #[must_use]
    #[inline(always)]
    pub const fn bcwp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BC Write Protect (This bit is read as 1.)"]
    #[inline(always)]
    pub const fn set_bcwp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Internal Reference Clock (fIIC) Selection ( fIIC = PCLKB / 2^CKS )"]
    #[must_use]
    #[inline(always)]
    pub const fn cks(&self) -> super::vals::Cks {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Cks::from_bits(val as u8)
    }
    #[doc = "Internal Reference Clock (fIIC) Selection ( fIIC = PCLKB / 2^CKS )"]
    #[inline(always)]
    pub const fn set_cks(&mut self, val: super::vals::Cks) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u8) & 0x07) << 4usize);
    }
    #[doc = "MST/TRS Write Protect"]
    #[must_use]
    #[inline(always)]
    pub const fn mtwp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "MST/TRS Write Protect"]
    #[inline(always)]
    pub const fn set_mtwp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Icmr1 {
    #[inline(always)]
    fn default() -> Icmr1 {
        Icmr1(0)
    }
}
impl core::fmt::Debug for Icmr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icmr1")
            .field("bc", &self.bc())
            .field("bcwp", &self.bcwp())
            .field("cks", &self.cks())
            .field("mtwp", &self.mtwp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icmr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icmr1 {{ bc: {:?}, bcwp: {=bool:?}, cks: {:?}, mtwp: {=bool:?} }}",
            self.bc(),
            self.bcwp(),
            self.cks(),
            self.mtwp()
        )
    }
}
#[doc = "I2C Bus Mode Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icmr2(pub u8);
impl Icmr2 {
    #[doc = "Timeout Detection Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tmos(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Detection Time Select"]
    #[inline(always)]
    pub const fn set_tmos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Timeout L Count Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tmol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout L Count Control"]
    #[inline(always)]
    pub const fn set_tmol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Timeout H Count Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tmoh(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout H Count Control"]
    #[inline(always)]
    pub const fn set_tmoh(&mut self, val: bool) {
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
    #[doc = "SDA Output Delay Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sddl(&self) -> super::vals::Sddl {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sddl::from_bits(val as u8)
    }
    #[doc = "SDA Output Delay Counter"]
    #[inline(always)]
    pub const fn set_sddl(&mut self, val: super::vals::Sddl) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u8) & 0x07) << 4usize);
    }
    #[doc = "SDA Output Delay Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn dlcs(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SDA Output Delay Clock Source Select"]
    #[inline(always)]
    pub const fn set_dlcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Icmr2 {
    #[inline(always)]
    fn default() -> Icmr2 {
        Icmr2(0)
    }
}
impl core::fmt::Debug for Icmr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icmr2")
            .field("tmos", &self.tmos())
            .field("tmol", &self.tmol())
            .field("tmoh", &self.tmoh())
            .field("reserved", &self.reserved())
            .field("sddl", &self.sddl())
            .field("dlcs", &self.dlcs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icmr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icmr2 {{ tmos: {=bool:?}, tmol: {=bool:?}, tmoh: {=bool:?}, reserved: {=bool:?}, sddl: {:?}, dlcs: {=bool:?} }}",
            self.tmos(),
            self.tmol(),
            self.tmoh(),
            self.reserved(),
            self.sddl(),
            self.dlcs()
        )
    }
}
#[doc = "I2C Bus Mode Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icmr3(pub u8);
impl Icmr3 {
    #[doc = "Noise Filter Stage Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn nf(&self) -> super::vals::Nf {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Nf::from_bits(val as u8)
    }
    #[doc = "Noise Filter Stage Selection"]
    #[inline(always)]
    pub const fn set_nf(&mut self, val: super::vals::Nf) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "Receive Acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn ackbr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Acknowledge"]
    #[inline(always)]
    pub const fn set_ackbr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Transmit Acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn ackbt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Acknowledge"]
    #[inline(always)]
    pub const fn set_ackbt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "ACKBT Write Protect"]
    #[must_use]
    #[inline(always)]
    pub const fn ackwp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ACKBT Write Protect"]
    #[inline(always)]
    pub const fn set_ackwp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "RDRF Flag Set Timing Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn rdrfs(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RDRF Flag Set Timing Selection"]
    #[inline(always)]
    pub const fn set_rdrfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "WAIT Note: When the value of the WAIT bit is to be read, be sure to read the ICDRR beforehand."]
    #[must_use]
    #[inline(always)]
    pub const fn wait(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "WAIT Note: When the value of the WAIT bit is to be read, be sure to read the ICDRR beforehand."]
    #[inline(always)]
    pub const fn set_wait(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "SMBus/I2C Bus Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn smbs(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus/I2C Bus Selection"]
    #[inline(always)]
    pub const fn set_smbs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Icmr3 {
    #[inline(always)]
    fn default() -> Icmr3 {
        Icmr3(0)
    }
}
impl core::fmt::Debug for Icmr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icmr3")
            .field("nf", &self.nf())
            .field("ackbr", &self.ackbr())
            .field("ackbt", &self.ackbt())
            .field("ackwp", &self.ackwp())
            .field("rdrfs", &self.rdrfs())
            .field("wait", &self.wait())
            .field("smbs", &self.smbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icmr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icmr3 {{ nf: {:?}, ackbr: {=bool:?}, ackbt: {=bool:?}, ackwp: {=bool:?}, rdrfs: {=bool:?}, wait: {=bool:?}, smbs: {=bool:?} }}",
            self.nf(),
            self.ackbr(),
            self.ackbt(),
            self.ackwp(),
            self.rdrfs(),
            self.wait(),
            self.smbs()
        )
    }
}
#[doc = "I2C Bus Status Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icser(pub u8);
impl Icser {
    #[doc = "Slave Address Register 0 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sar0e(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Address Register 0 Enable"]
    #[inline(always)]
    pub const fn set_sar0e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Slave Address Register 1 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sar1e(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Address Register 1 Enable"]
    #[inline(always)]
    pub const fn set_sar1e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Slave Address Register 2 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sar2e(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Address Register 2 Enable"]
    #[inline(always)]
    pub const fn set_sar2e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "General Call Address Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gcae(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "General Call Address Enable"]
    #[inline(always)]
    pub const fn set_gcae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Device-ID Address Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dide(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device-ID Address Detection Enable"]
    #[inline(always)]
    pub const fn set_dide(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Host Address Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hoae(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Host Address Enable"]
    #[inline(always)]
    pub const fn set_hoae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Icser {
    #[inline(always)]
    fn default() -> Icser {
        Icser(0)
    }
}
impl core::fmt::Debug for Icser {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icser")
            .field("sar0e", &self.sar0e())
            .field("sar1e", &self.sar1e())
            .field("sar2e", &self.sar2e())
            .field("gcae", &self.gcae())
            .field("reserved", &self.reserved())
            .field("dide", &self.dide())
            .field("reserved_2", &self.reserved_2())
            .field("hoae", &self.hoae())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icser {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icser {{ sar0e: {=bool:?}, sar1e: {=bool:?}, sar2e: {=bool:?}, gcae: {=bool:?}, reserved: {=bool:?}, dide: {=bool:?}, reserved_2: {=bool:?}, hoae: {=bool:?} }}",
            self.sar0e(),
            self.sar1e(),
            self.sar2e(),
            self.gcae(),
            self.reserved(),
            self.dide(),
            self.reserved_2(),
            self.hoae()
        )
    }
}
#[doc = "I2C Bus Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icsr1(pub u8);
impl Icsr1 {
    #[doc = "Slave Address 0 Detection Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn aas0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Address 0 Detection Flag"]
    #[inline(always)]
    pub const fn set_aas0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Slave Address 1 Detection Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn aas1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Address 1 Detection Flag"]
    #[inline(always)]
    pub const fn set_aas1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Slave Address 2 Detection Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn aas2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Address 2 Detection Flag"]
    #[inline(always)]
    pub const fn set_aas2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "General Call Address Detection Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gca(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "General Call Address Detection Flag"]
    #[inline(always)]
    pub const fn set_gca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Device-ID Address Detection Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn did(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device-ID Address Detection Flag"]
    #[inline(always)]
    pub const fn set_did(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Host Address Detection Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn hoa(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Host Address Detection Flag"]
    #[inline(always)]
    pub const fn set_hoa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Icsr1 {
    #[inline(always)]
    fn default() -> Icsr1 {
        Icsr1(0)
    }
}
impl core::fmt::Debug for Icsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icsr1")
            .field("aas0", &self.aas0())
            .field("aas1", &self.aas1())
            .field("aas2", &self.aas2())
            .field("gca", &self.gca())
            .field("reserved", &self.reserved())
            .field("did", &self.did())
            .field("reserved_2", &self.reserved_2())
            .field("hoa", &self.hoa())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icsr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icsr1 {{ aas0: {=bool:?}, aas1: {=bool:?}, aas2: {=bool:?}, gca: {=bool:?}, reserved: {=bool:?}, did: {=bool:?}, reserved_2: {=bool:?}, hoa: {=bool:?} }}",
            self.aas0(),
            self.aas1(),
            self.aas2(),
            self.gca(),
            self.reserved(),
            self.did(),
            self.reserved_2(),
            self.hoa()
        )
    }
}
#[doc = "I2C Bus Status Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icsr2(pub u8);
impl Icsr2 {
    #[doc = "Timeout Detection Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tmof(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Detection Flag"]
    #[inline(always)]
    pub const fn set_tmof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Arbitration-Lost Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn al(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration-Lost Flag"]
    #[inline(always)]
    pub const fn set_al(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Start Condition Detection Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Start Condition Detection Flag"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Stop Condition Detection Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Condition Detection Flag"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "NACK Detection Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn nackf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NACK Detection Flag"]
    #[inline(always)]
    pub const fn set_nackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Receive Data Full Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rdrf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Full Flag"]
    #[inline(always)]
    pub const fn set_rdrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Transmit End Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tend(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub const fn set_tend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Transmit Data Empty Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tdre(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Empty Flag"]
    #[inline(always)]
    pub const fn set_tdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Icsr2 {
    #[inline(always)]
    fn default() -> Icsr2 {
        Icsr2(0)
    }
}
impl core::fmt::Debug for Icsr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icsr2")
            .field("tmof", &self.tmof())
            .field("al", &self.al())
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("nackf", &self.nackf())
            .field("rdrf", &self.rdrf())
            .field("tend", &self.tend())
            .field("tdre", &self.tdre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icsr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icsr2 {{ tmof: {=bool:?}, al: {=bool:?}, start: {=bool:?}, stop: {=bool:?}, nackf: {=bool:?}, rdrf: {=bool:?}, tend: {=bool:?}, tdre: {=bool:?} }}",
            self.tmof(),
            self.al(),
            self.start(),
            self.stop(),
            self.nackf(),
            self.rdrf(),
            self.tend(),
            self.tdre()
        )
    }
}
#[doc = "I2C Bus Wake Up Unit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icwur(pub u8);
impl Icwur {
    #[doc = "Wakeup Analog Filter Additional Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn wuafa(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup Analog Filter Additional Selection"]
    #[inline(always)]
    pub const fn set_wuafa(&mut self, val: bool) {
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
    #[doc = "ACK bit for Wakeup Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn wuack(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ACK bit for Wakeup Mode"]
    #[inline(always)]
    pub const fn set_wuack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Wakeup Event Occurrence Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wuf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup Event Occurrence Flag"]
    #[inline(always)]
    pub const fn set_wuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Wakeup Interrupt Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wuie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup Interrupt Request Enable"]
    #[inline(always)]
    pub const fn set_wuie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Wakeup Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wue(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup Function Enable"]
    #[inline(always)]
    pub const fn set_wue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Icwur {
    #[inline(always)]
    fn default() -> Icwur {
        Icwur(0)
    }
}
impl core::fmt::Debug for Icwur {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icwur")
            .field("wuafa", &self.wuafa())
            .field("reserved", &self.reserved())
            .field("wuack", &self.wuack())
            .field("wuf", &self.wuf())
            .field("wuie", &self.wuie())
            .field("wue", &self.wue())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icwur {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icwur {{ wuafa: {=bool:?}, reserved: {=u8:?}, wuack: {=bool:?}, wuf: {=bool:?}, wuie: {=bool:?}, wue: {=bool:?} }}",
            self.wuafa(),
            self.reserved(),
            self.wuack(),
            self.wuf(),
            self.wuie(),
            self.wue()
        )
    }
}
#[doc = "I2C Bus Wake up Unit Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icwur2(pub u8);
impl Icwur2 {
    #[doc = "Wake-up Function Synchronous Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wusen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Function Synchronous Enable"]
    #[inline(always)]
    pub const fn set_wusen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Wake-up Function Asynchronous Operation Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wuasyf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Function Asynchronous Operation Status Flag"]
    #[inline(always)]
    pub const fn set_wuasyf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Wake-up Function Synchronous Operation Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wusyf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Function Synchronous Operation Status Flag"]
    #[inline(always)]
    pub const fn set_wusyf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "These bits are read as 11111. The write value should be 11111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 11111. The write value should be 11111."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for Icwur2 {
    #[inline(always)]
    fn default() -> Icwur2 {
        Icwur2(0)
    }
}
impl core::fmt::Debug for Icwur2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icwur2")
            .field("wusen", &self.wusen())
            .field("wuasyf", &self.wuasyf())
            .field("wusyf", &self.wusyf())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icwur2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icwur2 {{ wusen: {=bool:?}, wuasyf: {=bool:?}, wusyf: {=bool:?}, reserved: {=u8:?} }}",
            self.wusen(),
            self.wuasyf(),
            self.wusyf(),
            self.reserved()
        )
    }
}
#[doc = "Slave Address Register L%s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sarl(pub u8);
impl Sarl {
    #[doc = "A slave address is set. 7-Bit Address = SVA\\[7:1\\] 10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\] }"]
    #[must_use]
    #[inline(always)]
    pub const fn sva(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "A slave address is set. 7-Bit Address = SVA\\[7:1\\] 10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\] }"]
    #[inline(always)]
    pub const fn set_sva(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Sarl {
    #[inline(always)]
    fn default() -> Sarl {
        Sarl(0)
    }
}
impl core::fmt::Debug for Sarl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sarl").field("sva", &self.sva()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sarl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sarl {{ sva: {=u8:?} }}", self.sva())
    }
}
#[doc = "Slave Address Register U%s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Saru(pub u8);
impl Saru {
    #[doc = "7-Bit/10-Bit Address Format Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn fs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "7-Bit/10-Bit Address Format Selection"]
    #[inline(always)]
    pub const fn set_fs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "10-Bit Address(bit8)"]
    #[must_use]
    #[inline(always)]
    pub const fn sva8(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "10-Bit Address(bit8)"]
    #[inline(always)]
    pub const fn set_sva8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "10-Bit Address(bit9)"]
    #[must_use]
    #[inline(always)]
    pub const fn sva9(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "10-Bit Address(bit9)"]
    #[inline(always)]
    pub const fn set_sva9(&mut self, val: bool) {
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
impl Default for Saru {
    #[inline(always)]
    fn default() -> Saru {
        Saru(0)
    }
}
impl core::fmt::Debug for Saru {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Saru")
            .field("fs", &self.fs())
            .field("sva8", &self.sva8())
            .field("sva9", &self.sva9())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Saru {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Saru {{ fs: {=bool:?}, sva8: {=bool:?}, sva9: {=bool:?}, reserved: {=u8:?} }}",
            self.fs(),
            self.sva8(),
            self.sva9(),
            self.reserved()
        )
    }
}
