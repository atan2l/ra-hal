#[doc = "Bit Rate Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brr(pub u8);
impl Brr {
    #[doc = "BRR is an 8-bit register that adjusts the bit rate."]
    #[must_use]
    #[inline(always)]
    pub const fn brr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "BRR is an 8-bit register that adjusts the bit rate."]
    #[inline(always)]
    pub const fn set_brr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Brr {
    #[inline(always)]
    fn default() -> Brr {
        Brr(0)
    }
}
impl core::fmt::Debug for Brr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Brr").field("brr", &self.brr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Brr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Brr {{ brr: {=u8:?} }}", self.brr())
    }
}
#[doc = "Compare Match Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdr(pub u16);
impl Cdr {
    #[doc = "Compare Match Data Compare data pattern for address match wake-up function"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Compare Match Data Compare data pattern for address match wake-up function"]
    #[inline(always)]
    pub const fn set_cmpd(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u16) & 0x7f) << 9usize);
    }
}
impl Default for Cdr {
    #[inline(always)]
    fn default() -> Cdr {
        Cdr(0)
    }
}
impl core::fmt::Debug for Cdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdr")
            .field("cmpd", &self.cmpd())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cdr {{ cmpd: {=u16:?}, reserved: {=u8:?} }}",
            self.cmpd(),
            self.reserved()
        )
    }
}
#[doc = "Data Compare Match Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dccr(pub u8);
impl Dccr {
    #[doc = "Data Compare Match Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn dcmf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Data Compare Match Flag"]
    #[inline(always)]
    pub const fn set_dcmf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u8) & 0x03) << 1usize);
    }
    #[doc = "Data Compare Match Parity Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn dper(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data Compare Match Parity Error Flag"]
    #[inline(always)]
    pub const fn set_dper(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Data Compare Match Framing Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn dfer(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Data Compare Match Framing Error Flag"]
    #[inline(always)]
    pub const fn set_dfer(&mut self, val: bool) {
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
    #[doc = "ID frame select (Valid only in asynchronous mode(including multi-processor)"]
    #[must_use]
    #[inline(always)]
    pub const fn idsel(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "ID frame select (Valid only in asynchronous mode(including multi-processor)"]
    #[inline(always)]
    pub const fn set_idsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Data Compare Match Enable (Valid only in asynchronous mode(including multi-processor)"]
    #[must_use]
    #[inline(always)]
    pub const fn dcme(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data Compare Match Enable (Valid only in asynchronous mode(including multi-processor)"]
    #[inline(always)]
    pub const fn set_dcme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dccr {
    #[inline(always)]
    fn default() -> Dccr {
        Dccr(0)
    }
}
impl core::fmt::Debug for Dccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dccr")
            .field("dcmf", &self.dcmf())
            .field("reserved", &self.reserved())
            .field("dper", &self.dper())
            .field("dfer", &self.dfer())
            .field("reserved_2", &self.reserved_2())
            .field("idsel", &self.idsel())
            .field("dcme", &self.dcme())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dccr {{ dcmf: {=bool:?}, reserved: {=u8:?}, dper: {=bool:?}, dfer: {=bool:?}, reserved_2: {=bool:?}, idsel: {=bool:?}, dcme: {=bool:?} }}",
            self.dcmf(),
            self.reserved(),
            self.dper(),
            self.dfer(),
            self.reserved_2(),
            self.idsel(),
            self.dcme()
        )
    }
}
#[doc = "FIFO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcr(pub u16);
impl Fcr {
    #[doc = "FIFO Mode Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn fm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Mode Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub const fn set_fm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Receive FIFO Data Register Reset (Valid only in FCR.FM=1)"]
    #[must_use]
    #[inline(always)]
    pub const fn rfrst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Data Register Reset (Valid only in FCR.FM=1)"]
    #[inline(always)]
    pub const fn set_rfrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Transmit FIFO Data Register Reset (Valid only in FCR.FM=1)"]
    #[must_use]
    #[inline(always)]
    pub const fn tfrst(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Data Register Reset (Valid only in FCR.FM=1)"]
    #[inline(always)]
    pub const fn set_tfrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Receive data ready error select bit (When detecting a reception data ready, the interrupt request is selected.)"]
    #[must_use]
    #[inline(always)]
    pub const fn dres(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive data ready error select bit (When detecting a reception data ready, the interrupt request is selected.)"]
    #[inline(always)]
    pub const fn set_dres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn ttrg(&self) -> super::vals::Ttrg {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Ttrg::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub const fn set_ttrg(&mut self, val: super::vals::Ttrg) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u16) & 0x0f) << 4usize);
    }
    #[doc = "Receive FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn rtrg(&self) -> super::vals::Rtrg {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Rtrg::from_bits(val as u8)
    }
    #[doc = "Receive FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub const fn set_rtrg(&mut self, val: super::vals::Rtrg) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "RTS Output Active Trigger Number Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn rstrg(&self) -> super::vals::Rstrg {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Rstrg::from_bits(val as u8)
    }
    #[doc = "RTS Output Active Trigger Number Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub const fn set_rstrg(&mut self, val: super::vals::Rstrg) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Fcr {
    #[inline(always)]
    fn default() -> Fcr {
        Fcr(0)
    }
}
impl core::fmt::Debug for Fcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcr")
            .field("fm", &self.fm())
            .field("rfrst", &self.rfrst())
            .field("tfrst", &self.tfrst())
            .field("dres", &self.dres())
            .field("ttrg", &self.ttrg())
            .field("rtrg", &self.rtrg())
            .field("rstrg", &self.rstrg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fcr {{ fm: {=bool:?}, rfrst: {=bool:?}, tfrst: {=bool:?}, dres: {=bool:?}, ttrg: {:?}, rtrg: {:?}, rstrg: {:?} }}",
            self.fm(),
            self.rfrst(),
            self.tfrst(),
            self.dres(),
            self.ttrg(),
            self.rtrg(),
            self.rstrg()
        )
    }
}
#[doc = "FIFO Data Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdr(pub u16);
impl Fdr {
    #[doc = "Receive FIFO Data Count Indicate the quantity of receive data stored in FRDRH and FRDRL (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)"]
    #[must_use]
    #[inline(always)]
    pub const fn r(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Receive FIFO Data Count Indicate the quantity of receive data stored in FRDRH and FRDRL (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)"]
    #[inline(always)]
    pub const fn set_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
    }
    #[doc = "These bits are read as 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u16) & 0x07) << 5usize);
    }
    #[doc = "Transmit FIFO Data Count Indicate the quantity of non-transmit data stored in FTDRH and FTDRL (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)"]
    #[must_use]
    #[inline(always)]
    pub const fn t(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit FIFO Data Count Indicate the quantity of non-transmit data stored in FTDRH and FTDRL (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)"]
    #[inline(always)]
    pub const fn set_t(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
    }
    #[doc = "These bits are read as 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Fdr {
    #[inline(always)]
    fn default() -> Fdr {
        Fdr(0)
    }
}
impl core::fmt::Debug for Fdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fdr")
            .field("r", &self.r())
            .field("reserved", &self.reserved())
            .field("t", &self.t())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fdr {{ r: {=u8:?}, reserved: {=u8:?}, t: {=u8:?}, reserved_2: {=u8:?} }}",
            self.r(),
            self.reserved(),
            self.t(),
            self.reserved_2()
        )
    }
}
#[doc = "Receive FIFO Data Register H"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frdrh(pub u8);
impl Frdrh {
    #[doc = "Serial receive data(b8) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[must_use]
    #[inline(always)]
    pub const fn rdath(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Serial receive data(b8) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub const fn set_rdath(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Multi-processor bit flag (Valid only in asynchronous mode with SMR.MP=1 and FIFO selected) It can read multi-processor bit corresponded to serial receive data(RDATA\\[8:0\\])"]
    #[must_use]
    #[inline(always)]
    pub const fn mpb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-processor bit flag (Valid only in asynchronous mode with SMR.MP=1 and FIFO selected) It can read multi-processor bit corresponded to serial receive data(RDATA\\[8:0\\])"]
    #[inline(always)]
    pub const fn set_mpb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Receive data ready flag (It is same as SSR.DR)"]
    #[must_use]
    #[inline(always)]
    pub const fn dr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive data ready flag (It is same as SSR.DR)"]
    #[inline(always)]
    pub const fn set_dr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Parity error flag"]
    #[must_use]
    #[inline(always)]
    pub const fn per(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error flag"]
    #[inline(always)]
    pub const fn set_per(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Framing error flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fer(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error flag"]
    #[inline(always)]
    pub const fn set_fer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Overrun error flag (It is same as SSR.ORER)"]
    #[must_use]
    #[inline(always)]
    pub const fn orer(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error flag (It is same as SSR.ORER)"]
    #[inline(always)]
    pub const fn set_orer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Receive FIFO data full flag (It is same as SSR.RDF)"]
    #[must_use]
    #[inline(always)]
    pub const fn rdf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO data full flag (It is same as SSR.RDF)"]
    #[inline(always)]
    pub const fn set_rdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "This bit is read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Frdrh {
    #[inline(always)]
    fn default() -> Frdrh {
        Frdrh(0)
    }
}
impl core::fmt::Debug for Frdrh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frdrh")
            .field("rdath", &self.rdath())
            .field("mpb", &self.mpb())
            .field("dr", &self.dr())
            .field("per", &self.per())
            .field("fer", &self.fer())
            .field("orer", &self.orer())
            .field("rdf", &self.rdf())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frdrh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frdrh {{ rdath: {=bool:?}, mpb: {=bool:?}, dr: {=bool:?}, per: {=bool:?}, fer: {=bool:?}, orer: {=bool:?}, rdf: {=bool:?}, reserved: {=bool:?} }}",
            self.rdath(),
            self.mpb(),
            self.dr(),
            self.per(),
            self.fer(),
            self.orer(),
            self.rdf(),
            self.reserved()
        )
    }
}
#[doc = "Receive FIFO Data Register HL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frdrhl(pub u16);
impl Frdrhl {
    #[doc = "Serial receive data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[must_use]
    #[inline(always)]
    pub const fn rdat(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Serial receive data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub const fn set_rdat(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Multi-processor bit flag (Valid only in asynchronous mode with SMR.MP=1 and FIFO selected) It can read multi-processor bit corresponded to serial receive data(RDATA\\[8:0\\])"]
    #[must_use]
    #[inline(always)]
    pub const fn mpb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-processor bit flag (Valid only in asynchronous mode with SMR.MP=1 and FIFO selected) It can read multi-processor bit corresponded to serial receive data(RDATA\\[8:0\\])"]
    #[inline(always)]
    pub const fn set_mpb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Receive data ready flag (It is same as SSR.DR)"]
    #[must_use]
    #[inline(always)]
    pub const fn dr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Receive data ready flag (It is same as SSR.DR)"]
    #[inline(always)]
    pub const fn set_dr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Parity error flag"]
    #[must_use]
    #[inline(always)]
    pub const fn per(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error flag"]
    #[inline(always)]
    pub const fn set_per(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Framing error flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fer(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error flag"]
    #[inline(always)]
    pub const fn set_fer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Overrun error flag (It is same as SSR.ORER)"]
    #[must_use]
    #[inline(always)]
    pub const fn orer(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error flag (It is same as SSR.ORER)"]
    #[inline(always)]
    pub const fn set_orer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Receive FIFO data full flag (It is same as SSR.RDF)"]
    #[must_use]
    #[inline(always)]
    pub const fn rdf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO data full flag (It is same as SSR.RDF)"]
    #[inline(always)]
    pub const fn set_rdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "This bit is read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Frdrhl {
    #[inline(always)]
    fn default() -> Frdrhl {
        Frdrhl(0)
    }
}
impl core::fmt::Debug for Frdrhl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frdrhl")
            .field("rdat", &self.rdat())
            .field("mpb", &self.mpb())
            .field("dr", &self.dr())
            .field("per", &self.per())
            .field("fer", &self.fer())
            .field("orer", &self.orer())
            .field("rdf", &self.rdf())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frdrhl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frdrhl {{ rdat: {=u16:?}, mpb: {=bool:?}, dr: {=bool:?}, per: {=bool:?}, fer: {=bool:?}, orer: {=bool:?}, rdf: {=bool:?}, reserved: {=bool:?} }}",
            self.rdat(),
            self.mpb(),
            self.dr(),
            self.per(),
            self.fer(),
            self.orer(),
            self.rdf(),
            self.reserved()
        )
    }
}
#[doc = "Receive FIFO Data Register L"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frdrl(pub u8);
impl Frdrl {
    #[doc = "Serial receive data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected) NOTE: When reading both of FRDRH register and FRDRL register, please read by an order of the FRDRH register and the FRDRL register."]
    #[must_use]
    #[inline(always)]
    pub const fn rdatl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Serial receive data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected) NOTE: When reading both of FRDRH register and FRDRL register, please read by an order of the FRDRH register and the FRDRL register."]
    #[inline(always)]
    pub const fn set_rdatl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Frdrl {
    #[inline(always)]
    fn default() -> Frdrl {
        Frdrl(0)
    }
}
impl core::fmt::Debug for Frdrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frdrl")
            .field("rdatl", &self.rdatl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frdrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frdrl {{ rdatl: {=u8:?} }}", self.rdatl())
    }
}
#[doc = "Transmit FIFO Data Register H"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftdrh(pub u8);
impl Ftdrh {
    #[doc = "Serial transmit data (b8) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[must_use]
    #[inline(always)]
    pub const fn tdath(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Serial transmit data (b8) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub const fn set_tdath(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
    #[must_use]
    #[inline(always)]
    pub const fn mpbt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
    #[inline(always)]
    pub const fn set_mpbt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "The write value should be 111111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "The write value should be 111111."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for Ftdrh {
    #[inline(always)]
    fn default() -> Ftdrh {
        Ftdrh(0)
    }
}
impl core::fmt::Debug for Ftdrh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ftdrh")
            .field("tdath", &self.tdath())
            .field("mpbt", &self.mpbt())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ftdrh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ftdrh {{ tdath: {=bool:?}, mpbt: {=bool:?}, reserved: {=u8:?} }}",
            self.tdath(),
            self.mpbt(),
            self.reserved()
        )
    }
}
#[doc = "Transmit FIFO Data Register HL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftdrhl(pub u16);
impl Ftdrhl {
    #[doc = "Serial transmit data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[must_use]
    #[inline(always)]
    pub const fn tdat(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Serial transmit data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub const fn set_tdat(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
    #[must_use]
    #[inline(always)]
    pub const fn mpbt(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
    #[inline(always)]
    pub const fn set_mpbt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "The write value should be 111111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "The write value should be 111111."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Ftdrhl {
    #[inline(always)]
    fn default() -> Ftdrhl {
        Ftdrhl(0)
    }
}
impl core::fmt::Debug for Ftdrhl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ftdrhl")
            .field("tdat", &self.tdat())
            .field("mpbt", &self.mpbt())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ftdrhl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ftdrhl {{ tdat: {=u16:?}, mpbt: {=bool:?}, reserved: {=u8:?} }}",
            self.tdat(),
            self.mpbt(),
            self.reserved()
        )
    }
}
#[doc = "Transmit FIFO Data Register L"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftdrl(pub u8);
impl Ftdrl {
    #[doc = "Serial transmit data(b7-b0) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[must_use]
    #[inline(always)]
    pub const fn tdatl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Serial transmit data(b7-b0) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub const fn set_tdatl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Ftdrl {
    #[inline(always)]
    fn default() -> Ftdrl {
        Ftdrl(0)
    }
}
impl core::fmt::Debug for Ftdrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ftdrl")
            .field("tdatl", &self.tdatl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ftdrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ftdrl {{ tdatl: {=u8:?} }}", self.tdatl())
    }
}
#[doc = "Line Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lsr(pub u16);
impl Lsr {
    #[doc = "Overrun Error Flag (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[must_use]
    #[inline(always)]
    pub const fn orer(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Error Flag (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub const fn set_orer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "This bit is read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Framing Error Count Indicates the quantity of data with a framing error among the receive data stored in the receive FIFO data register (FRDRH and FRDRL)."]
    #[must_use]
    #[inline(always)]
    pub const fn fnum(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x1f;
        val as u8
    }
    #[doc = "Framing Error Count Indicates the quantity of data with a framing error among the receive data stored in the receive FIFO data register (FRDRH and FRDRL)."]
    #[inline(always)]
    pub const fn set_fnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u16) & 0x1f) << 2usize);
    }
    #[doc = "This bit is read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Parity Error Count Indicates the quantity of data with a parity error among the receive data stored in the receive FIFO data register (FRDRH and FRDRL)."]
    #[must_use]
    #[inline(always)]
    pub const fn pnum(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Parity Error Count Indicates the quantity of data with a parity error among the receive data stored in the receive FIFO data register (FRDRH and FRDRL)."]
    #[inline(always)]
    pub const fn set_pnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
    }
    #[doc = "These bits are read as 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Lsr {
    #[inline(always)]
    fn default() -> Lsr {
        Lsr(0)
    }
}
impl core::fmt::Debug for Lsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lsr")
            .field("orer", &self.orer())
            .field("reserved", &self.reserved())
            .field("fnum", &self.fnum())
            .field("reserved_2", &self.reserved_2())
            .field("pnum", &self.pnum())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lsr {{ orer: {=bool:?}, reserved: {=bool:?}, fnum: {=u8:?}, reserved_2: {=bool:?}, pnum: {=u8:?}, reserved_3: {=u8:?} }}",
            self.orer(),
            self.reserved(),
            self.fnum(),
            self.reserved_2(),
            self.pnum(),
            self.reserved_3()
        )
    }
}
#[doc = "Modulation Duty Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mddr(pub u8);
impl Mddr {
    #[doc = "MDDR corrects the bit rate adjusted by the BRR register."]
    #[must_use]
    #[inline(always)]
    pub const fn mddr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "MDDR corrects the bit rate adjusted by the BRR register."]
    #[inline(always)]
    pub const fn set_mddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Mddr {
    #[inline(always)]
    fn default() -> Mddr {
        Mddr(0)
    }
}
impl core::fmt::Debug for Mddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mddr").field("mddr", &self.mddr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mddr {{ mddr: {=u8:?} }}", self.mddr())
    }
}
#[doc = "Receive Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdr(pub u8);
impl Rdr {
    #[doc = "RDR is an 8-bit register that stores receive data."]
    #[must_use]
    #[inline(always)]
    pub const fn rdr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RDR is an 8-bit register that stores receive data."]
    #[inline(always)]
    pub const fn set_rdr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Rdr {
    #[inline(always)]
    fn default() -> Rdr {
        Rdr(0)
    }
}
impl core::fmt::Debug for Rdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdr").field("rdr", &self.rdr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdr {{ rdr: {=u8:?} }}", self.rdr())
    }
}
#[doc = "Receive 9-bit Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdrhl(pub u16);
impl Rdrhl {
    #[doc = "RDRHL is an 16-bit register that stores receive data."]
    #[must_use]
    #[inline(always)]
    pub const fn rdrhl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "RDRHL is an 16-bit register that stores receive data."]
    #[inline(always)]
    pub const fn set_rdrhl(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Rdrhl {
    #[inline(always)]
    fn default() -> Rdrhl {
        Rdrhl(0)
    }
}
impl core::fmt::Debug for Rdrhl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdrhl")
            .field("rdrhl", &self.rdrhl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdrhl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdrhl {{ rdrhl: {=u16:?} }}", self.rdrhl())
    }
}
#[doc = "Smart Card Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scmr(pub u8);
impl Scmr {
    #[doc = "Smart Card Interface Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn smif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Smart Card Interface Mode Select"]
    #[inline(always)]
    pub const fn set_smif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Transmitted/Received Data Invert Set this bit to 0 if operation is to be in simple I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn sinv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitted/Received Data Invert Set this bit to 0 if operation is to be in simple I2C mode."]
    #[inline(always)]
    pub const fn set_sinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Transmitted/Received Data Transfer Direction NOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode. Set this bit to 1 if operation is to be in simple I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn sdir(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitted/Received Data Transfer Direction NOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode. Set this bit to 1 if operation is to be in simple I2C mode."]
    #[inline(always)]
    pub const fn set_sdir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Character Length 1 (Only valid in asynchronous mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn chr1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Character Length 1 (Only valid in asynchronous mode)"]
    #[inline(always)]
    pub const fn set_chr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 11. The write value should be 11."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 11. The write value should be 11."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u8) & 0x03) << 5usize);
    }
    #[doc = "Base Clock Pulse 2 Selects the number of base clock cycles in combination with the SMR.BCP\\[1:0\\] bits"]
    #[must_use]
    #[inline(always)]
    pub const fn bcp2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Base Clock Pulse 2 Selects the number of base clock cycles in combination with the SMR.BCP\\[1:0\\] bits"]
    #[inline(always)]
    pub const fn set_bcp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Scmr {
    #[inline(always)]
    fn default() -> Scmr {
        Scmr(0)
    }
}
impl core::fmt::Debug for Scmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scmr")
            .field("smif", &self.smif())
            .field("reserved", &self.reserved())
            .field("sinv", &self.sinv())
            .field("sdir", &self.sdir())
            .field("chr1", &self.chr1())
            .field("reserved_2", &self.reserved_2())
            .field("bcp2", &self.bcp2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scmr {{ smif: {=bool:?}, reserved: {=bool:?}, sinv: {=bool:?}, sdir: {=bool:?}, chr1: {=bool:?}, reserved_2: {=u8:?}, bcp2: {=bool:?} }}",
            self.smif(),
            self.reserved(),
            self.sinv(),
            self.sdir(),
            self.chr1(),
            self.reserved_2(),
            self.bcp2()
        )
    }
}
#[doc = "Serial Control Register (SCMR.SMIF = 0)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u8);
impl Scr {
    #[doc = "Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cke(&self) -> super::vals::ScrCke {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::ScrCke::from_bits(val as u8)
    }
    #[doc = "Clock Enable"]
    #[inline(always)]
    pub const fn set_cke(&mut self, val: super::vals::ScrCke) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "Transmit End Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit End Interrupt Enable"]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Multi-Processor Interrupt Enable (Valid in asynchronous mode when SMR.MP = 1)"]
    #[must_use]
    #[inline(always)]
    pub const fn mpie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Processor Interrupt Enable (Valid in asynchronous mode when SMR.MP = 1)"]
    #[inline(always)]
    pub const fn set_mpie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Receive Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Enable"]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Transmit Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn te(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Enable"]
    #[inline(always)]
    pub const fn set_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Receive Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Transmit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0)
    }
}
impl core::fmt::Debug for Scr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr")
            .field("cke", &self.cke())
            .field("teie", &self.teie())
            .field("mpie", &self.mpie())
            .field("re", &self.re())
            .field("te", &self.te())
            .field("rie", &self.rie())
            .field("tie", &self.tie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr {{ cke: {:?}, teie: {=bool:?}, mpie: {=bool:?}, re: {=bool:?}, te: {=bool:?}, rie: {=bool:?}, tie: {=bool:?} }}",
            self.cke(),
            self.teie(),
            self.mpie(),
            self.re(),
            self.te(),
            self.rie(),
            self.tie()
        )
    }
}
#[doc = "Serial Control Register (SCMR.SMIF =1)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScrSmci(pub u8);
impl ScrSmci {
    #[doc = "Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cke(&self) -> super::vals::ScrSmciCke {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::ScrSmciCke::from_bits(val as u8)
    }
    #[doc = "Clock Enable"]
    #[inline(always)]
    pub const fn set_cke(&mut self, val: super::vals::ScrSmciCke) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "Transmit End Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit End Interrupt Enable"]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Multi-Processor Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mpie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Processor Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mpie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Receive Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Enable"]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Transmit Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn te(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Enable"]
    #[inline(always)]
    pub const fn set_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Receive Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Transmit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for ScrSmci {
    #[inline(always)]
    fn default() -> ScrSmci {
        ScrSmci(0)
    }
}
impl core::fmt::Debug for ScrSmci {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ScrSmci")
            .field("cke", &self.cke())
            .field("teie", &self.teie())
            .field("mpie", &self.mpie())
            .field("re", &self.re())
            .field("te", &self.te())
            .field("rie", &self.rie())
            .field("tie", &self.tie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ScrSmci {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ScrSmci {{ cke: {:?}, teie: {=bool:?}, mpie: {=bool:?}, re: {=bool:?}, te: {=bool:?}, rie: {=bool:?}, tie: {=bool:?} }}",
            self.cke(),
            self.teie(),
            self.mpie(),
            self.re(),
            self.te(),
            self.rie(),
            self.tie()
        )
    }
}
#[doc = "Serial Extended Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Semr(pub u8);
impl Semr {
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
    #[doc = "Bit Rate Modulation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn brme(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Rate Modulation Enable"]
    #[inline(always)]
    pub const fn set_brme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Asynchronous Mode Extended Base Clock Select 1 (Valid only in asynchronous mode and SCR.CKE\\[1\\]=0)"]
    #[must_use]
    #[inline(always)]
    pub const fn abcse(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Mode Extended Base Clock Select 1 (Valid only in asynchronous mode and SCR.CKE\\[1\\]=0)"]
    #[inline(always)]
    pub const fn set_abcse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Asynchronous Mode Base Clock Select (Valid only in asynchronous mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn abcs(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Mode Base Clock Select (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub const fn set_abcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Digital Noise Filter Function Enable (The NFEN bit should be 0 without simple I2C mode and asynchronous mode.) In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input."]
    #[must_use]
    #[inline(always)]
    pub const fn nfen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Noise Filter Function Enable (The NFEN bit should be 0 without simple I2C mode and asynchronous mode.) In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input."]
    #[inline(always)]
    pub const fn set_nfen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Baud Rate Generator Double-Speed Mode Select (Only valid the CKE\\[1\\] bit in SCR is 0 in asynchronous mode)."]
    #[must_use]
    #[inline(always)]
    pub const fn bgdm(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Baud Rate Generator Double-Speed Mode Select (Only valid the CKE\\[1\\] bit in SCR is 0 in asynchronous mode)."]
    #[inline(always)]
    pub const fn set_bgdm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Asynchronous Start Bit Edge Detection Select (Valid only in asynchronous mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdesel(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Start Bit Edge Detection Select (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub const fn set_rxdesel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Semr {
    #[inline(always)]
    fn default() -> Semr {
        Semr(0)
    }
}
impl core::fmt::Debug for Semr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Semr")
            .field("reserved", &self.reserved())
            .field("brme", &self.brme())
            .field("abcse", &self.abcse())
            .field("abcs", &self.abcs())
            .field("nfen", &self.nfen())
            .field("bgdm", &self.bgdm())
            .field("rxdesel", &self.rxdesel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Semr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Semr {{ reserved: {=u8:?}, brme: {=bool:?}, abcse: {=bool:?}, abcs: {=bool:?}, nfen: {=bool:?}, bgdm: {=bool:?}, rxdesel: {=bool:?} }}",
            self.reserved(),
            self.brme(),
            self.abcse(),
            self.abcs(),
            self.nfen(),
            self.bgdm(),
            self.rxdesel()
        )
    }
}
#[doc = "I2C Mode Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr1(pub u8);
impl Simr1 {
    #[doc = "Simple I2C Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iicm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Simple I2C Mode Select"]
    #[inline(always)]
    pub const fn set_iicm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u8) & 0x03) << 1usize);
    }
    #[doc = "SDA Delay Output Select Cycles below are of the clock signal from the on-chip baud rate generator."]
    #[must_use]
    #[inline(always)]
    pub const fn iicdl(&self) -> super::vals::Iicdl {
        let val = (self.0 >> 3usize) & 0x1f;
        super::vals::Iicdl::from_bits(val as u8)
    }
    #[doc = "SDA Delay Output Select Cycles below are of the clock signal from the on-chip baud rate generator."]
    #[inline(always)]
    pub const fn set_iicdl(&mut self, val: super::vals::Iicdl) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val.to_bits() as u8) & 0x1f) << 3usize);
    }
}
impl Default for Simr1 {
    #[inline(always)]
    fn default() -> Simr1 {
        Simr1(0)
    }
}
impl core::fmt::Debug for Simr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Simr1")
            .field("iicm", &self.iicm())
            .field("reserved", &self.reserved())
            .field("iicdl", &self.iicdl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Simr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Simr1 {{ iicm: {=bool:?}, reserved: {=u8:?}, iicdl: {:?} }}",
            self.iicm(),
            self.reserved(),
            self.iicdl()
        )
    }
}
#[doc = "I2C Mode Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr2(pub u8);
impl Simr2 {
    #[doc = "I2C Interrupt Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iicintm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Interrupt Mode Select"]
    #[inline(always)]
    pub const fn set_iicintm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Clock Synchronization"]
    #[must_use]
    #[inline(always)]
    pub const fn iiccsc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Synchronization"]
    #[inline(always)]
    pub const fn set_iiccsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u8) & 0x07) << 2usize);
    }
    #[doc = "ACK Transmission Data"]
    #[must_use]
    #[inline(always)]
    pub const fn iicackt(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ACK Transmission Data"]
    #[inline(always)]
    pub const fn set_iicackt(&mut self, val: bool) {
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
impl Default for Simr2 {
    #[inline(always)]
    fn default() -> Simr2 {
        Simr2(0)
    }
}
impl core::fmt::Debug for Simr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Simr2")
            .field("iicintm", &self.iicintm())
            .field("iiccsc", &self.iiccsc())
            .field("reserved", &self.reserved())
            .field("iicackt", &self.iicackt())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Simr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Simr2 {{ iicintm: {=bool:?}, iiccsc: {=bool:?}, reserved: {=u8:?}, iicackt: {=bool:?}, reserved_2: {=u8:?} }}",
            self.iicintm(),
            self.iiccsc(),
            self.reserved(),
            self.iicackt(),
            self.reserved_2()
        )
    }
}
#[doc = "I2C Mode Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr3(pub u8);
impl Simr3 {
    #[doc = "Start Condition Generation"]
    #[must_use]
    #[inline(always)]
    pub const fn iicstareq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start Condition Generation"]
    #[inline(always)]
    pub const fn set_iicstareq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Restart Condition Generation"]
    #[must_use]
    #[inline(always)]
    pub const fn iicrstareq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Restart Condition Generation"]
    #[inline(always)]
    pub const fn set_iicrstareq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Stop Condition Generation"]
    #[must_use]
    #[inline(always)]
    pub const fn iicstpreq(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Condition Generation"]
    #[inline(always)]
    pub const fn set_iicstpreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Issuing of Start, Restart, or Stop Condition Completed Flag (When 0 is written to IICSTIF, it is cleared to 0.)"]
    #[must_use]
    #[inline(always)]
    pub const fn iicstif(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Issuing of Start, Restart, or Stop Condition Completed Flag (When 0 is written to IICSTIF, it is cleared to 0.)"]
    #[inline(always)]
    pub const fn set_iicstif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "SDA Output Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iicsdas(&self) -> super::vals::Iicsdas {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Iicsdas::from_bits(val as u8)
    }
    #[doc = "SDA Output Select"]
    #[inline(always)]
    pub const fn set_iicsdas(&mut self, val: super::vals::Iicsdas) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
    }
    #[doc = "SCL Output Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iicscls(&self) -> super::vals::Iicscls {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Iicscls::from_bits(val as u8)
    }
    #[doc = "SCL Output Select"]
    #[inline(always)]
    pub const fn set_iicscls(&mut self, val: super::vals::Iicscls) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u8) & 0x03) << 6usize);
    }
}
impl Default for Simr3 {
    #[inline(always)]
    fn default() -> Simr3 {
        Simr3(0)
    }
}
impl core::fmt::Debug for Simr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Simr3")
            .field("iicstareq", &self.iicstareq())
            .field("iicrstareq", &self.iicrstareq())
            .field("iicstpreq", &self.iicstpreq())
            .field("iicstif", &self.iicstif())
            .field("iicsdas", &self.iicsdas())
            .field("iicscls", &self.iicscls())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Simr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Simr3 {{ iicstareq: {=bool:?}, iicrstareq: {=bool:?}, iicstpreq: {=bool:?}, iicstif: {=bool:?}, iicsdas: {:?}, iicscls: {:?} }}",
            self.iicstareq(),
            self.iicrstareq(),
            self.iicstpreq(),
            self.iicstif(),
            self.iicsdas(),
            self.iicscls()
        )
    }
}
#[doc = "I2C Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sisr(pub u8);
impl Sisr {
    #[doc = "ACK Reception Data Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn iicackr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ACK Reception Data Flag"]
    #[inline(always)]
    pub const fn set_iicackr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "This bit is read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "This bit is read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "This bit is read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "This bit is read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "This bit is read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0."]
    #[inline(always)]
    pub const fn set_reserved_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "These bits are read as 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_6(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00."]
    #[inline(always)]
    pub const fn set_reserved_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
    }
}
impl Default for Sisr {
    #[inline(always)]
    fn default() -> Sisr {
        Sisr(0)
    }
}
impl core::fmt::Debug for Sisr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sisr")
            .field("iicackr", &self.iicackr())
            .field("reserved", &self.reserved())
            .field("reserved_2", &self.reserved_2())
            .field("reserved_3", &self.reserved_3())
            .field("reserved_4", &self.reserved_4())
            .field("reserved_5", &self.reserved_5())
            .field("reserved_6", &self.reserved_6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sisr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sisr {{ iicackr: {=bool:?}, reserved: {=bool:?}, reserved_2: {=bool:?}, reserved_3: {=bool:?}, reserved_4: {=bool:?}, reserved_5: {=bool:?}, reserved_6: {=u8:?} }}",
            self.iicackr(),
            self.reserved(),
            self.reserved_2(),
            self.reserved_3(),
            self.reserved_4(),
            self.reserved_5(),
            self.reserved_6()
        )
    }
}
#[doc = "Serial Mode Register (SCMR.SMIF = 0)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smr(pub u8);
impl Smr {
    #[doc = "Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cks(&self) -> super::vals::SmrCks {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SmrCks::from_bits(val as u8)
    }
    #[doc = "Clock Select"]
    #[inline(always)]
    pub const fn set_cks(&mut self, val: super::vals::SmrCks) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "Multi-Processor Mode (Valid only in asynchronous mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn mp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Processor Mode (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub const fn set_mp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Stop Bit Length (Valid only in asynchronous mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Bit Length (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Parity Mode (Valid only when the PE bit is 1)"]
    #[must_use]
    #[inline(always)]
    pub const fn pm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Mode (Valid only when the PE bit is 1)"]
    #[inline(always)]
    pub const fn set_pm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Parity Enable (Valid only in asynchronous mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Enable (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Character Length (Valid only in asynchronous mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn chr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Character Length (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub const fn set_chr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Communication Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Communication Mode"]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Smr {
    #[inline(always)]
    fn default() -> Smr {
        Smr(0)
    }
}
impl core::fmt::Debug for Smr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smr")
            .field("cks", &self.cks())
            .field("mp", &self.mp())
            .field("stop", &self.stop())
            .field("pm", &self.pm())
            .field("pe", &self.pe())
            .field("chr", &self.chr())
            .field("cm", &self.cm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smr {{ cks: {:?}, mp: {=bool:?}, stop: {=bool:?}, pm: {=bool:?}, pe: {=bool:?}, chr: {=bool:?}, cm: {=bool:?} }}",
            self.cks(),
            self.mp(),
            self.stop(),
            self.pm(),
            self.pe(),
            self.chr(),
            self.cm()
        )
    }
}
#[doc = "Serial mode register (SCMR.SMIF = 1)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmrSmci(pub u8);
impl SmrSmci {
    #[doc = "Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cks(&self) -> super::vals::SmrSmciCks {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SmrSmciCks::from_bits(val as u8)
    }
    #[doc = "Clock Select"]
    #[inline(always)]
    pub const fn set_cks(&mut self, val: super::vals::SmrSmciCks) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "Base Clock Pulse (Valid only in asynchronous mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn bcp(&self) -> super::vals::Bcp {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bcp::from_bits(val as u8)
    }
    #[doc = "Base Clock Pulse (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub const fn set_bcp(&mut self, val: super::vals::Bcp) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u8) & 0x03) << 2usize);
    }
    #[doc = "Parity Mode (Valid only when the PE bit is 1)"]
    #[must_use]
    #[inline(always)]
    pub const fn pm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Mode (Valid only when the PE bit is 1)"]
    #[inline(always)]
    pub const fn set_pm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Parity Enable (Valid only in asynchronous mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Enable (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Block Transfer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn blk(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Block Transfer Mode"]
    #[inline(always)]
    pub const fn set_blk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "GSM Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn gm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GSM Mode"]
    #[inline(always)]
    pub const fn set_gm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for SmrSmci {
    #[inline(always)]
    fn default() -> SmrSmci {
        SmrSmci(0)
    }
}
impl core::fmt::Debug for SmrSmci {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmrSmci")
            .field("cks", &self.cks())
            .field("bcp", &self.bcp())
            .field("pm", &self.pm())
            .field("pe", &self.pe())
            .field("blk", &self.blk())
            .field("gm", &self.gm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmrSmci {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmrSmci {{ cks: {:?}, bcp: {:?}, pm: {=bool:?}, pe: {=bool:?}, blk: {=bool:?}, gm: {=bool:?} }}",
            self.cks(),
            self.bcp(),
            self.pm(),
            self.pe(),
            self.blk(),
            self.gm()
        )
    }
}
#[doc = "Noise Filter Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snfr(pub u8);
impl Snfr {
    #[doc = "Noise Filter Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn nfcs(&self) -> super::vals::Nfcs {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Nfcs::from_bits(val as u8)
    }
    #[doc = "Noise Filter Clock Select"]
    #[inline(always)]
    pub const fn set_nfcs(&mut self, val: super::vals::Nfcs) {
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
impl Default for Snfr {
    #[inline(always)]
    fn default() -> Snfr {
        Snfr(0)
    }
}
impl core::fmt::Debug for Snfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Snfr")
            .field("nfcs", &self.nfcs())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Snfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Snfr {{ nfcs: {:?}, reserved: {=u8:?} }}",
            self.nfcs(),
            self.reserved()
        )
    }
}
#[doc = "SPI Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spmr(pub u8);
impl Spmr {
    #[doc = "SSn Pin Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SSn Pin Function Enable"]
    #[inline(always)]
    pub const fn set_sse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "CTS Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ctse(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CTS Enable"]
    #[inline(always)]
    pub const fn set_ctse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Master Slave Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mss(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Master Slave Select"]
    #[inline(always)]
    pub const fn set_mss(&mut self, val: bool) {
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
    #[doc = "Mode Fault Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn mff(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Fault Flag"]
    #[inline(always)]
    pub const fn set_mff(&mut self, val: bool) {
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
    #[doc = "Clock Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ckpol(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Polarity Select"]
    #[inline(always)]
    pub const fn set_ckpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Clock Phase Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ckph(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Phase Select"]
    #[inline(always)]
    pub const fn set_ckph(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Spmr {
    #[inline(always)]
    fn default() -> Spmr {
        Spmr(0)
    }
}
impl core::fmt::Debug for Spmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spmr")
            .field("sse", &self.sse())
            .field("ctse", &self.ctse())
            .field("mss", &self.mss())
            .field("reserved", &self.reserved())
            .field("mff", &self.mff())
            .field("reserved_2", &self.reserved_2())
            .field("ckpol", &self.ckpol())
            .field("ckph", &self.ckph())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spmr {{ sse: {=bool:?}, ctse: {=bool:?}, mss: {=bool:?}, reserved: {=bool:?}, mff: {=bool:?}, reserved_2: {=bool:?}, ckpol: {=bool:?}, ckph: {=bool:?} }}",
            self.sse(),
            self.ctse(),
            self.mss(),
            self.reserved(),
            self.mff(),
            self.reserved_2(),
            self.ckpol(),
            self.ckph()
        )
    }
}
#[doc = "Serial Port Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sptr(pub u8);
impl Sptr {
    #[doc = "Serial input data monitor bit (The state of the RXD terminal is shown.)"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdmon(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Serial input data monitor bit (The state of the RXD terminal is shown.)"]
    #[inline(always)]
    pub const fn set_rxdmon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Serial port break data select bit (The output level of TxD terminal is selected when SCR.TE = 0.)"]
    #[must_use]
    #[inline(always)]
    pub const fn spb2dt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Serial port break data select bit (The output level of TxD terminal is selected when SCR.TE = 0.)"]
    #[inline(always)]
    pub const fn set_spb2dt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Serial port break I/O bit (It's selected whether the value of SPB2DT is output to TxD terminal.)"]
    #[must_use]
    #[inline(always)]
    pub const fn spb2io(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Serial port break I/O bit (It's selected whether the value of SPB2DT is output to TxD terminal.)"]
    #[inline(always)]
    pub const fn set_spb2io(&mut self, val: bool) {
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
impl Default for Sptr {
    #[inline(always)]
    fn default() -> Sptr {
        Sptr(0)
    }
}
impl core::fmt::Debug for Sptr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sptr")
            .field("rxdmon", &self.rxdmon())
            .field("spb2dt", &self.spb2dt())
            .field("spb2io", &self.spb2io())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sptr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sptr {{ rxdmon: {=bool:?}, spb2dt: {=bool:?}, spb2io: {=bool:?}, reserved: {=u8:?} }}",
            self.rxdmon(),
            self.spb2dt(),
            self.spb2io(),
            self.reserved()
        )
    }
}
#[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssr(pub u8);
impl Ssr {
    #[doc = "Multi-Processor Bit Transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn mpbt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Processor Bit Transfer"]
    #[inline(always)]
    pub const fn set_mpbt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Multi-Processor"]
    #[must_use]
    #[inline(always)]
    pub const fn mpb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Processor"]
    #[inline(always)]
    pub const fn set_mpb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Transmit End Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tend(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub const fn set_tend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Parity Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn per(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub const fn set_per(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Framing Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fer(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error Flag"]
    #[inline(always)]
    pub const fn set_fer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Overrun Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn orer(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Error Flag"]
    #[inline(always)]
    pub const fn set_orer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Receive Data Full Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rdrf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Full Flag"]
    #[inline(always)]
    pub const fn set_rdrf(&mut self, val: bool) {
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
impl Default for Ssr {
    #[inline(always)]
    fn default() -> Ssr {
        Ssr(0)
    }
}
impl core::fmt::Debug for Ssr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssr")
            .field("mpbt", &self.mpbt())
            .field("mpb", &self.mpb())
            .field("tend", &self.tend())
            .field("per", &self.per())
            .field("fer", &self.fer())
            .field("orer", &self.orer())
            .field("rdrf", &self.rdrf())
            .field("tdre", &self.tdre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssr {{ mpbt: {=bool:?}, mpb: {=bool:?}, tend: {=bool:?}, per: {=bool:?}, fer: {=bool:?}, orer: {=bool:?}, rdrf: {=bool:?}, tdre: {=bool:?} }}",
            self.mpbt(),
            self.mpb(),
            self.tend(),
            self.per(),
            self.fer(),
            self.orer(),
            self.rdrf(),
            self.tdre()
        )
    }
}
#[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=1)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsrFifo(pub u8);
impl SsrFifo {
    #[doc = "Receive Data Ready flag (Valid only in asynchronous mode(including multi-processor) and FIFO selected)"]
    #[must_use]
    #[inline(always)]
    pub const fn dr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Ready flag (Valid only in asynchronous mode(including multi-processor) and FIFO selected)"]
    #[inline(always)]
    pub const fn set_dr(&mut self, val: bool) {
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
    #[doc = "Transmit End Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tend(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub const fn set_tend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Parity Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn per(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub const fn set_per(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Framing Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fer(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error Flag"]
    #[inline(always)]
    pub const fn set_fer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Overrun Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn orer(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Error Flag"]
    #[inline(always)]
    pub const fn set_orer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Receive FIFO data full flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rdf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO data full flag"]
    #[inline(always)]
    pub const fn set_rdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Transmit FIFO data empty flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tdfe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO data empty flag"]
    #[inline(always)]
    pub const fn set_tdfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for SsrFifo {
    #[inline(always)]
    fn default() -> SsrFifo {
        SsrFifo(0)
    }
}
impl core::fmt::Debug for SsrFifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsrFifo")
            .field("dr", &self.dr())
            .field("reserved", &self.reserved())
            .field("tend", &self.tend())
            .field("per", &self.per())
            .field("fer", &self.fer())
            .field("orer", &self.orer())
            .field("rdf", &self.rdf())
            .field("tdfe", &self.tdfe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsrFifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SsrFifo {{ dr: {=bool:?}, reserved: {=bool:?}, tend: {=bool:?}, per: {=bool:?}, fer: {=bool:?}, orer: {=bool:?}, rdf: {=bool:?}, tdfe: {=bool:?} }}",
            self.dr(),
            self.reserved(),
            self.tend(),
            self.per(),
            self.fer(),
            self.orer(),
            self.rdf(),
            self.tdfe()
        )
    }
}
#[doc = "Serial Status Register(SCMR.SMIF = 1)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsrSmci(pub u8);
impl SsrSmci {
    #[doc = "Multi-Processor Bit Transfer This bit should be 0 in smart card interface mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mpbt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Processor Bit Transfer This bit should be 0 in smart card interface mode."]
    #[inline(always)]
    pub const fn set_mpbt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Multi-Processor This bit should be 0 in smart card interface mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mpb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Processor This bit should be 0 in smart card interface mode."]
    #[inline(always)]
    pub const fn set_mpb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Transmit End Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tend(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub const fn set_tend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Parity Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn per(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub const fn set_per(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Error Signal Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ers(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Error Signal Status Flag"]
    #[inline(always)]
    pub const fn set_ers(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Overrun Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn orer(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Error Flag"]
    #[inline(always)]
    pub const fn set_orer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Receive Data Full Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rdrf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Full Flag"]
    #[inline(always)]
    pub const fn set_rdrf(&mut self, val: bool) {
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
impl Default for SsrSmci {
    #[inline(always)]
    fn default() -> SsrSmci {
        SsrSmci(0)
    }
}
impl core::fmt::Debug for SsrSmci {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsrSmci")
            .field("mpbt", &self.mpbt())
            .field("mpb", &self.mpb())
            .field("tend", &self.tend())
            .field("per", &self.per())
            .field("ers", &self.ers())
            .field("orer", &self.orer())
            .field("rdrf", &self.rdrf())
            .field("tdre", &self.tdre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsrSmci {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SsrSmci {{ mpbt: {=bool:?}, mpb: {=bool:?}, tend: {=bool:?}, per: {=bool:?}, ers: {=bool:?}, orer: {=bool:?}, rdrf: {=bool:?}, tdre: {=bool:?} }}",
            self.mpbt(),
            self.mpb(),
            self.tend(),
            self.per(),
            self.ers(),
            self.orer(),
            self.rdrf(),
            self.tdre()
        )
    }
}
#[doc = "Transmit Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr(pub u8);
impl Tdr {
    #[doc = "TDR is an 8-bit register that stores transmit data."]
    #[must_use]
    #[inline(always)]
    pub const fn tdr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TDR is an 8-bit register that stores transmit data."]
    #[inline(always)]
    pub const fn set_tdr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Tdr {
    #[inline(always)]
    fn default() -> Tdr {
        Tdr(0)
    }
}
impl core::fmt::Debug for Tdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdr").field("tdr", &self.tdr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdr {{ tdr: {=u8:?} }}", self.tdr())
    }
}
#[doc = "Transmit 9-bit Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdrhl(pub u16);
impl Tdrhl {
    #[doc = "TDRHL is a 16-bit register that stores transmit data."]
    #[must_use]
    #[inline(always)]
    pub const fn tdrhl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TDRHL is a 16-bit register that stores transmit data."]
    #[inline(always)]
    pub const fn set_tdrhl(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tdrhl {
    #[inline(always)]
    fn default() -> Tdrhl {
        Tdrhl(0)
    }
}
impl core::fmt::Debug for Tdrhl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdrhl")
            .field("tdrhl", &self.tdrhl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdrhl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdrhl {{ tdrhl: {=u16:?} }}", self.tdrhl())
    }
}
