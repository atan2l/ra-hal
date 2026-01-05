#[doc = "BEMP Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bempenb(pub u16);
impl Bempenb {
    #[doc = "BEMP Interrupt Enable for PIPE0"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe0bempe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BEMP Interrupt Enable for PIPE0"]
    #[inline(always)]
    pub const fn set_pipe0bempe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE1"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe1bempe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BEMP Interrupt Enable for PIPE1"]
    #[inline(always)]
    pub const fn set_pipe1bempe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE2"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe2bempe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BEMP Interrupt Enable for PIPE2"]
    #[inline(always)]
    pub const fn set_pipe2bempe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE3"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe3bempe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BEMP Interrupt Enable for PIPE3"]
    #[inline(always)]
    pub const fn set_pipe3bempe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE4"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe4bempe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "BEMP Interrupt Enable for PIPE4"]
    #[inline(always)]
    pub const fn set_pipe4bempe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE5"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe5bempe(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "BEMP Interrupt Enable for PIPE5"]
    #[inline(always)]
    pub const fn set_pipe5bempe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE6"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe6bempe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "BEMP Interrupt Enable for PIPE6"]
    #[inline(always)]
    pub const fn set_pipe6bempe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE7"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe7bempe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "BEMP Interrupt Enable for PIPE7"]
    #[inline(always)]
    pub const fn set_pipe7bempe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE8"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe8bempe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "BEMP Interrupt Enable for PIPE8"]
    #[inline(always)]
    pub const fn set_pipe8bempe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE9"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe9bempe(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "BEMP Interrupt Enable for PIPE9"]
    #[inline(always)]
    pub const fn set_pipe9bempe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Bempenb {
    #[inline(always)]
    fn default() -> Bempenb {
        Bempenb(0)
    }
}
impl core::fmt::Debug for Bempenb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bempenb")
            .field("pipe0bempe", &self.pipe0bempe())
            .field("pipe1bempe", &self.pipe1bempe())
            .field("pipe2bempe", &self.pipe2bempe())
            .field("pipe3bempe", &self.pipe3bempe())
            .field("pipe4bempe", &self.pipe4bempe())
            .field("pipe5bempe", &self.pipe5bempe())
            .field("pipe6bempe", &self.pipe6bempe())
            .field("pipe7bempe", &self.pipe7bempe())
            .field("pipe8bempe", &self.pipe8bempe())
            .field("pipe9bempe", &self.pipe9bempe())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bempenb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bempenb {{ pipe0bempe: {=bool:?}, pipe1bempe: {=bool:?}, pipe2bempe: {=bool:?}, pipe3bempe: {=bool:?}, pipe4bempe: {=bool:?}, pipe5bempe: {=bool:?}, pipe6bempe: {=bool:?}, pipe7bempe: {=bool:?}, pipe8bempe: {=bool:?}, pipe9bempe: {=bool:?}, reserved: {=u8:?} }}",
            self.pipe0bempe(),
            self.pipe1bempe(),
            self.pipe2bempe(),
            self.pipe3bempe(),
            self.pipe4bempe(),
            self.pipe5bempe(),
            self.pipe6bempe(),
            self.pipe7bempe(),
            self.pipe8bempe(),
            self.pipe9bempe(),
            self.reserved()
        )
    }
}
#[doc = "BEMP Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bempsts(pub u16);
impl Bempsts {
    #[doc = "BEMP Interrupt Status for PIPE0"]
    #[must_use]
    #[inline(always)]
    pub const fn bemp(&self, n: usize) -> bool {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "BEMP Interrupt Status for PIPE0"]
    #[inline(always)]
    pub const fn set_bemp(&mut self, n: usize, val: bool) {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Bempsts {
    #[inline(always)]
    fn default() -> Bempsts {
        Bempsts(0)
    }
}
impl core::fmt::Debug for Bempsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bempsts")
            .field("bemp[0]", &self.bemp(0usize))
            .field("bemp[1]", &self.bemp(1usize))
            .field("bemp[2]", &self.bemp(2usize))
            .field("bemp[3]", &self.bemp(3usize))
            .field("bemp[4]", &self.bemp(4usize))
            .field("bemp[5]", &self.bemp(5usize))
            .field("bemp[6]", &self.bemp(6usize))
            .field("bemp[7]", &self.bemp(7usize))
            .field("bemp[8]", &self.bemp(8usize))
            .field("bemp[9]", &self.bemp(9usize))
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bempsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bempsts {{ bemp[0]: {=bool:?}, bemp[1]: {=bool:?}, bemp[2]: {=bool:?}, bemp[3]: {=bool:?}, bemp[4]: {=bool:?}, bemp[5]: {=bool:?}, bemp[6]: {=bool:?}, bemp[7]: {=bool:?}, bemp[8]: {=bool:?}, bemp[9]: {=bool:?}, reserved: {=u8:?} }}",
            self.bemp(0usize),
            self.bemp(1usize),
            self.bemp(2usize),
            self.bemp(3usize),
            self.bemp(4usize),
            self.bemp(5usize),
            self.bemp(6usize),
            self.bemp(7usize),
            self.bemp(8usize),
            self.bemp(9usize),
            self.reserved()
        )
    }
}
#[doc = "BRDY Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brdyenb(pub u16);
impl Brdyenb {
    #[doc = "BRDY Interrupt Enable for PIPE0"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe0brdye(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BRDY Interrupt Enable for PIPE0"]
    #[inline(always)]
    pub const fn set_pipe0brdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE1"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe1brdye(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BRDY Interrupt Enable for PIPE1"]
    #[inline(always)]
    pub const fn set_pipe1brdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE2"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe2brdye(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BRDY Interrupt Enable for PIPE2"]
    #[inline(always)]
    pub const fn set_pipe2brdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE3"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe3brdye(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BRDY Interrupt Enable for PIPE3"]
    #[inline(always)]
    pub const fn set_pipe3brdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE4"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe4brdye(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "BRDY Interrupt Enable for PIPE4"]
    #[inline(always)]
    pub const fn set_pipe4brdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE5"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe5brdye(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "BRDY Interrupt Enable for PIPE5"]
    #[inline(always)]
    pub const fn set_pipe5brdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE6"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe6brdye(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "BRDY Interrupt Enable for PIPE6"]
    #[inline(always)]
    pub const fn set_pipe6brdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE7"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe7brdye(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "BRDY Interrupt Enable for PIPE7"]
    #[inline(always)]
    pub const fn set_pipe7brdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE8"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe8brdye(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "BRDY Interrupt Enable for PIPE8"]
    #[inline(always)]
    pub const fn set_pipe8brdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE9"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe9brdye(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "BRDY Interrupt Enable for PIPE9"]
    #[inline(always)]
    pub const fn set_pipe9brdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Brdyenb {
    #[inline(always)]
    fn default() -> Brdyenb {
        Brdyenb(0)
    }
}
impl core::fmt::Debug for Brdyenb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Brdyenb")
            .field("pipe0brdye", &self.pipe0brdye())
            .field("pipe1brdye", &self.pipe1brdye())
            .field("pipe2brdye", &self.pipe2brdye())
            .field("pipe3brdye", &self.pipe3brdye())
            .field("pipe4brdye", &self.pipe4brdye())
            .field("pipe5brdye", &self.pipe5brdye())
            .field("pipe6brdye", &self.pipe6brdye())
            .field("pipe7brdye", &self.pipe7brdye())
            .field("pipe8brdye", &self.pipe8brdye())
            .field("pipe9brdye", &self.pipe9brdye())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Brdyenb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Brdyenb {{ pipe0brdye: {=bool:?}, pipe1brdye: {=bool:?}, pipe2brdye: {=bool:?}, pipe3brdye: {=bool:?}, pipe4brdye: {=bool:?}, pipe5brdye: {=bool:?}, pipe6brdye: {=bool:?}, pipe7brdye: {=bool:?}, pipe8brdye: {=bool:?}, pipe9brdye: {=bool:?}, reserved: {=u8:?} }}",
            self.pipe0brdye(),
            self.pipe1brdye(),
            self.pipe2brdye(),
            self.pipe3brdye(),
            self.pipe4brdye(),
            self.pipe5brdye(),
            self.pipe6brdye(),
            self.pipe7brdye(),
            self.pipe8brdye(),
            self.pipe9brdye(),
            self.reserved()
        )
    }
}
#[doc = "BRDY Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brdysts(pub u16);
impl Brdysts {
    #[doc = "BRDY Interrupt Status for PIPE0"]
    #[must_use]
    #[inline(always)]
    pub const fn brdy(&self, n: usize) -> bool {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "BRDY Interrupt Status for PIPE0"]
    #[inline(always)]
    pub const fn set_brdy(&mut self, n: usize, val: bool) {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Brdysts {
    #[inline(always)]
    fn default() -> Brdysts {
        Brdysts(0)
    }
}
impl core::fmt::Debug for Brdysts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Brdysts")
            .field("brdy[0]", &self.brdy(0usize))
            .field("brdy[1]", &self.brdy(1usize))
            .field("brdy[2]", &self.brdy(2usize))
            .field("brdy[3]", &self.brdy(3usize))
            .field("brdy[4]", &self.brdy(4usize))
            .field("brdy[5]", &self.brdy(5usize))
            .field("brdy[6]", &self.brdy(6usize))
            .field("brdy[7]", &self.brdy(7usize))
            .field("brdy[8]", &self.brdy(8usize))
            .field("brdy[9]", &self.brdy(9usize))
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Brdysts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Brdysts {{ brdy[0]: {=bool:?}, brdy[1]: {=bool:?}, brdy[2]: {=bool:?}, brdy[3]: {=bool:?}, brdy[4]: {=bool:?}, brdy[5]: {=bool:?}, brdy[6]: {=bool:?}, brdy[7]: {=bool:?}, brdy[8]: {=bool:?}, brdy[9]: {=bool:?}, reserved: {=u8:?} }}",
            self.brdy(0usize),
            self.brdy(1usize),
            self.brdy(2usize),
            self.brdy(3usize),
            self.brdy(4usize),
            self.brdy(5usize),
            self.brdy(6usize),
            self.brdy(7usize),
            self.brdy(8usize),
            self.brdy(9usize),
            self.reserved()
        )
    }
}
#[doc = "CFIFO Port Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfifo(pub u16);
impl Cfifo {
    #[doc = "FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoport(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub const fn set_fifoport(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Cfifo {
    #[inline(always)]
    fn default() -> Cfifo {
        Cfifo(0)
    }
}
impl core::fmt::Debug for Cfifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfifo")
            .field("fifoport", &self.fifoport())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cfifo {{ fifoport: {=u16:?} }}", self.fifoport())
    }
}
#[doc = "CFIFO Port Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfifoctr(pub u16);
impl Cfifoctr {
    #[doc = "Receive Data Length Indicates the length of the receive data."]
    #[must_use]
    #[inline(always)]
    pub const fn dtln(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Receive Data Length Indicates the length of the receive data."]
    #[inline(always)]
    pub const fn set_dtln(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "FIFO Port Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn frdy(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Port Ready"]
    #[inline(always)]
    pub const fn set_frdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "CPU Buffer Clear Note: Only 0 can be read."]
    #[must_use]
    #[inline(always)]
    pub const fn bclr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CPU Buffer Clear Note: Only 0 can be read."]
    #[inline(always)]
    pub const fn set_bclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Buffer Memory Valid Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bval(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Memory Valid Flag"]
    #[inline(always)]
    pub const fn set_bval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Cfifoctr {
    #[inline(always)]
    fn default() -> Cfifoctr {
        Cfifoctr(0)
    }
}
impl core::fmt::Debug for Cfifoctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfifoctr")
            .field("dtln", &self.dtln())
            .field("reserved", &self.reserved())
            .field("frdy", &self.frdy())
            .field("bclr", &self.bclr())
            .field("bval", &self.bval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfifoctr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfifoctr {{ dtln: {=u16:?}, reserved: {=u8:?}, frdy: {=bool:?}, bclr: {=bool:?}, bval: {=bool:?} }}",
            self.dtln(),
            self.reserved(),
            self.frdy(),
            self.bclr(),
            self.bval()
        )
    }
}
#[doc = "CFIFO Port Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfifosel(pub u16);
impl Cfifosel {
    #[doc = "CFIFO Port Access Pipe Specification"]
    #[must_use]
    #[inline(always)]
    pub const fn curpipe(&self) -> super::vals::CfifoselCurpipe {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::CfifoselCurpipe::from_bits(val as u8)
    }
    #[doc = "CFIFO Port Access Pipe Specification"]
    #[inline(always)]
    pub const fn set_curpipe(&mut self, val: super::vals::CfifoselCurpipe) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
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
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "CFIFO Port Access Direction When DCP is Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CFIFO Port Access Direction When DCP is Selected"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
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
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
    }
    #[doc = "CFIFO Port Endian Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bigend(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CFIFO Port Endian Control"]
    #[inline(always)]
    pub const fn set_bigend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "CFIFO Port Access Bit Width"]
    #[must_use]
    #[inline(always)]
    pub const fn mbw(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CFIFO Port Access Bit Width"]
    #[inline(always)]
    pub const fn set_mbw(&mut self, val: bool) {
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
    #[doc = "Buffer Pointer Rewind"]
    #[must_use]
    #[inline(always)]
    pub const fn rew(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Pointer Rewind"]
    #[inline(always)]
    pub const fn set_rew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Read Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rcnt(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Read Count Mode"]
    #[inline(always)]
    pub const fn set_rcnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Cfifosel {
    #[inline(always)]
    fn default() -> Cfifosel {
        Cfifosel(0)
    }
}
impl core::fmt::Debug for Cfifosel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfifosel")
            .field("curpipe", &self.curpipe())
            .field("reserved", &self.reserved())
            .field("isel", &self.isel())
            .field("reserved_2", &self.reserved_2())
            .field("bigend", &self.bigend())
            .field("reserved_3", &self.reserved_3())
            .field("mbw", &self.mbw())
            .field("reserved_4", &self.reserved_4())
            .field("rew", &self.rew())
            .field("rcnt", &self.rcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfifosel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfifosel {{ curpipe: {:?}, reserved: {=bool:?}, isel: {=bool:?}, reserved_2: {=u8:?}, bigend: {=bool:?}, reserved_3: {=bool:?}, mbw: {=bool:?}, reserved_4: {=u8:?}, rew: {=bool:?}, rcnt: {=bool:?} }}",
            self.curpipe(),
            self.reserved(),
            self.isel(),
            self.reserved_2(),
            self.bigend(),
            self.reserved_3(),
            self.mbw(),
            self.reserved_4(),
            self.rew(),
            self.rcnt()
        )
    }
}
#[doc = "D0FIFO Port Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D0fifo(pub u16);
impl D0fifo {
    #[doc = "FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoport(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub const fn set_fifoport(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for D0fifo {
    #[inline(always)]
    fn default() -> D0fifo {
        D0fifo(0)
    }
}
impl core::fmt::Debug for D0fifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D0fifo")
            .field("fifoport", &self.fifoport())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for D0fifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "D0fifo {{ fifoport: {=u16:?} }}", self.fifoport())
    }
}
#[doc = "D0FIFO Port Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D0fifoctr(pub u16);
impl D0fifoctr {
    #[doc = "Receive Data Length Indicates the length of the receive data."]
    #[must_use]
    #[inline(always)]
    pub const fn dtln(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Receive Data Length Indicates the length of the receive data."]
    #[inline(always)]
    pub const fn set_dtln(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "FIFO Port Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn frdy(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Port Ready"]
    #[inline(always)]
    pub const fn set_frdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "CPU Buffer Clear Note: Only 0 can be read."]
    #[must_use]
    #[inline(always)]
    pub const fn bclr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CPU Buffer Clear Note: Only 0 can be read."]
    #[inline(always)]
    pub const fn set_bclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Buffer Memory Valid Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bval(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Memory Valid Flag"]
    #[inline(always)]
    pub const fn set_bval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for D0fifoctr {
    #[inline(always)]
    fn default() -> D0fifoctr {
        D0fifoctr(0)
    }
}
impl core::fmt::Debug for D0fifoctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D0fifoctr")
            .field("dtln", &self.dtln())
            .field("reserved", &self.reserved())
            .field("frdy", &self.frdy())
            .field("bclr", &self.bclr())
            .field("bval", &self.bval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for D0fifoctr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "D0fifoctr {{ dtln: {=u16:?}, reserved: {=u8:?}, frdy: {=bool:?}, bclr: {=bool:?}, bval: {=bool:?} }}",
            self.dtln(),
            self.reserved(),
            self.frdy(),
            self.bclr(),
            self.bval()
        )
    }
}
#[doc = "D0FIFO Port Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D0fifosel(pub u16);
impl D0fifosel {
    #[doc = "FIFO Port Access Pipe Specification"]
    #[must_use]
    #[inline(always)]
    pub const fn curpipe(&self) -> super::vals::D0fifoselCurpipe {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::D0fifoselCurpipe::from_bits(val as u8)
    }
    #[doc = "FIFO Port Access Pipe Specification"]
    #[inline(always)]
    pub const fn set_curpipe(&mut self, val: super::vals::D0fifoselCurpipe) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
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
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "FIFO Port Endian Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bigend(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Port Endian Control"]
    #[inline(always)]
    pub const fn set_bigend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "FIFO Port Access Bit Width"]
    #[must_use]
    #[inline(always)]
    pub const fn mbw(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Port Access Bit Width"]
    #[inline(always)]
    pub const fn set_mbw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
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
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "DMA/DTC Transfer Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dreqe(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA/DTC Transfer Request Enable"]
    #[inline(always)]
    pub const fn set_dreqe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read"]
    #[must_use]
    #[inline(always)]
    pub const fn dclrm(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read"]
    #[inline(always)]
    pub const fn set_dclrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Buffer Pointer Rewind Note: Only 0 can be read."]
    #[must_use]
    #[inline(always)]
    pub const fn rew(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Pointer Rewind Note: Only 0 can be read."]
    #[inline(always)]
    pub const fn set_rew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Read Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rcnt(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Read Count Mode"]
    #[inline(always)]
    pub const fn set_rcnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for D0fifosel {
    #[inline(always)]
    fn default() -> D0fifosel {
        D0fifosel(0)
    }
}
impl core::fmt::Debug for D0fifosel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D0fifosel")
            .field("curpipe", &self.curpipe())
            .field("reserved", &self.reserved())
            .field("bigend", &self.bigend())
            .field("reserved_2", &self.reserved_2())
            .field("mbw", &self.mbw())
            .field("reserved_3", &self.reserved_3())
            .field("dreqe", &self.dreqe())
            .field("dclrm", &self.dclrm())
            .field("rew", &self.rew())
            .field("rcnt", &self.rcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for D0fifosel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "D0fifosel {{ curpipe: {:?}, reserved: {=u8:?}, bigend: {=bool:?}, reserved_2: {=bool:?}, mbw: {=bool:?}, reserved_3: {=bool:?}, dreqe: {=bool:?}, dclrm: {=bool:?}, rew: {=bool:?}, rcnt: {=bool:?} }}",
            self.curpipe(),
            self.reserved(),
            self.bigend(),
            self.reserved_2(),
            self.mbw(),
            self.reserved_3(),
            self.dreqe(),
            self.dclrm(),
            self.rew(),
            self.rcnt()
        )
    }
}
#[doc = "D1FIFO Port Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D1fifo(pub u16);
impl D1fifo {
    #[doc = "FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoport(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub const fn set_fifoport(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for D1fifo {
    #[inline(always)]
    fn default() -> D1fifo {
        D1fifo(0)
    }
}
impl core::fmt::Debug for D1fifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D1fifo")
            .field("fifoport", &self.fifoport())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for D1fifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "D1fifo {{ fifoport: {=u16:?} }}", self.fifoport())
    }
}
#[doc = "D1FIFO Port Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D1fifoctr(pub u16);
impl D1fifoctr {
    #[doc = "Receive Data Length Indicates the length of the receive data."]
    #[must_use]
    #[inline(always)]
    pub const fn dtln(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Receive Data Length Indicates the length of the receive data."]
    #[inline(always)]
    pub const fn set_dtln(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "FIFO Port Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn frdy(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Port Ready"]
    #[inline(always)]
    pub const fn set_frdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "CPU Buffer Clear Note: Only 0 can be read."]
    #[must_use]
    #[inline(always)]
    pub const fn bclr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CPU Buffer Clear Note: Only 0 can be read."]
    #[inline(always)]
    pub const fn set_bclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Buffer Memory Valid Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bval(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Memory Valid Flag"]
    #[inline(always)]
    pub const fn set_bval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for D1fifoctr {
    #[inline(always)]
    fn default() -> D1fifoctr {
        D1fifoctr(0)
    }
}
impl core::fmt::Debug for D1fifoctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D1fifoctr")
            .field("dtln", &self.dtln())
            .field("reserved", &self.reserved())
            .field("frdy", &self.frdy())
            .field("bclr", &self.bclr())
            .field("bval", &self.bval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for D1fifoctr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "D1fifoctr {{ dtln: {=u16:?}, reserved: {=u8:?}, frdy: {=bool:?}, bclr: {=bool:?}, bval: {=bool:?} }}",
            self.dtln(),
            self.reserved(),
            self.frdy(),
            self.bclr(),
            self.bval()
        )
    }
}
#[doc = "D1FIFO Port Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D1fifosel(pub u16);
impl D1fifosel {
    #[doc = "FIFO Port Access Pipe Specification"]
    #[must_use]
    #[inline(always)]
    pub const fn curpipe(&self) -> super::vals::D1fifoselCurpipe {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::D1fifoselCurpipe::from_bits(val as u8)
    }
    #[doc = "FIFO Port Access Pipe Specification"]
    #[inline(always)]
    pub const fn set_curpipe(&mut self, val: super::vals::D1fifoselCurpipe) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
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
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "FIFO Port Endian Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bigend(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Port Endian Control"]
    #[inline(always)]
    pub const fn set_bigend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "FIFO Port Access Bit Width"]
    #[must_use]
    #[inline(always)]
    pub const fn mbw(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Port Access Bit Width"]
    #[inline(always)]
    pub const fn set_mbw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
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
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "DMA/DTC Transfer Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dreqe(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA/DTC Transfer Request Enable"]
    #[inline(always)]
    pub const fn set_dreqe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read"]
    #[must_use]
    #[inline(always)]
    pub const fn dclrm(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read"]
    #[inline(always)]
    pub const fn set_dclrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Buffer Pointer Rewind"]
    #[must_use]
    #[inline(always)]
    pub const fn rew(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Pointer Rewind"]
    #[inline(always)]
    pub const fn set_rew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Read Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rcnt(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Read Count Mode"]
    #[inline(always)]
    pub const fn set_rcnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for D1fifosel {
    #[inline(always)]
    fn default() -> D1fifosel {
        D1fifosel(0)
    }
}
impl core::fmt::Debug for D1fifosel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D1fifosel")
            .field("curpipe", &self.curpipe())
            .field("reserved", &self.reserved())
            .field("bigend", &self.bigend())
            .field("reserved_2", &self.reserved_2())
            .field("mbw", &self.mbw())
            .field("reserved_3", &self.reserved_3())
            .field("dreqe", &self.dreqe())
            .field("dclrm", &self.dclrm())
            .field("rew", &self.rew())
            .field("rcnt", &self.rcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for D1fifosel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "D1fifosel {{ curpipe: {:?}, reserved: {=u8:?}, bigend: {=bool:?}, reserved_2: {=bool:?}, mbw: {=bool:?}, reserved_3: {=bool:?}, dreqe: {=bool:?}, dclrm: {=bool:?}, rew: {=bool:?}, rcnt: {=bool:?} }}",
            self.curpipe(),
            self.reserved(),
            self.bigend(),
            self.reserved_2(),
            self.mbw(),
            self.reserved_3(),
            self.dreqe(),
            self.dclrm(),
            self.rew(),
            self.rcnt()
        )
    }
}
#[doc = "DCP Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcpcfg(pub u16);
impl Dcpcfg {
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
    #[doc = "Transfer Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u16) & 0x03) << 5usize);
    }
    #[doc = "Pipe Disabled at End of Transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn shtnak(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Pipe Disabled at End of Transfer"]
    #[inline(always)]
    pub const fn set_shtnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Dcpcfg {
    #[inline(always)]
    fn default() -> Dcpcfg {
        Dcpcfg(0)
    }
}
impl core::fmt::Debug for Dcpcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcpcfg")
            .field("reserved", &self.reserved())
            .field("dir", &self.dir())
            .field("reserved_2", &self.reserved_2())
            .field("shtnak", &self.shtnak())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcpcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcpcfg {{ reserved: {=u8:?}, dir: {=bool:?}, reserved_2: {=u8:?}, shtnak: {=bool:?}, reserved_3: {=u8:?} }}",
            self.reserved(),
            self.dir(),
            self.reserved_2(),
            self.shtnak(),
            self.reserved_3()
        )
    }
}
#[doc = "DCP Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcpctr(pub u16);
impl Dcpctr {
    #[doc = "Response PID"]
    #[must_use]
    #[inline(always)]
    pub const fn pid(&self) -> super::vals::DcpctrPid {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DcpctrPid::from_bits(val as u8)
    }
    #[doc = "Response PID"]
    #[inline(always)]
    pub const fn set_pid(&mut self, val: super::vals::DcpctrPid) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Control Transfer End Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ccpl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control Transfer End Enable"]
    #[inline(always)]
    pub const fn set_ccpl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u16) & 0x03) << 3usize);
    }
    #[doc = "Pipe Busy"]
    #[must_use]
    #[inline(always)]
    pub const fn pbusy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pipe Busy"]
    #[inline(always)]
    pub const fn set_pbusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Sequence Toggle Bit Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn sqmon(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Toggle Bit Monitor"]
    #[inline(always)]
    pub const fn set_sqmon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Sequence Toggle Bit Set"]
    #[must_use]
    #[inline(always)]
    pub const fn sqset(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Toggle Bit Set"]
    #[inline(always)]
    pub const fn set_sqset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Sequence Toggle Bit Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sqclr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Toggle Bit Clear"]
    #[inline(always)]
    pub const fn set_sqclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u16) & 0x03) << 9usize);
    }
    #[doc = "SUREQ Bit Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sureqclr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SUREQ Bit Clear"]
    #[inline(always)]
    pub const fn set_sureqclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u16) & 0x03) << 12usize);
    }
    #[doc = "Setup Token Transmission"]
    #[must_use]
    #[inline(always)]
    pub const fn sureq(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Setup Token Transmission"]
    #[inline(always)]
    pub const fn set_sureq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Buffer Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bsts(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Status"]
    #[inline(always)]
    pub const fn set_bsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Dcpctr {
    #[inline(always)]
    fn default() -> Dcpctr {
        Dcpctr(0)
    }
}
impl core::fmt::Debug for Dcpctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcpctr")
            .field("pid", &self.pid())
            .field("ccpl", &self.ccpl())
            .field("reserved", &self.reserved())
            .field("pbusy", &self.pbusy())
            .field("sqmon", &self.sqmon())
            .field("sqset", &self.sqset())
            .field("sqclr", &self.sqclr())
            .field("reserved_2", &self.reserved_2())
            .field("sureqclr", &self.sureqclr())
            .field("reserved_3", &self.reserved_3())
            .field("sureq", &self.sureq())
            .field("bsts", &self.bsts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcpctr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcpctr {{ pid: {:?}, ccpl: {=bool:?}, reserved: {=u8:?}, pbusy: {=bool:?}, sqmon: {=bool:?}, sqset: {=bool:?}, sqclr: {=bool:?}, reserved_2: {=u8:?}, sureqclr: {=bool:?}, reserved_3: {=u8:?}, sureq: {=bool:?}, bsts: {=bool:?} }}",
            self.pid(),
            self.ccpl(),
            self.reserved(),
            self.pbusy(),
            self.sqmon(),
            self.sqset(),
            self.sqclr(),
            self.reserved_2(),
            self.sureqclr(),
            self.reserved_3(),
            self.sureq(),
            self.bsts()
        )
    }
}
#[doc = "DCP Maximum Packet Size Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcpmaxp(pub u16);
impl Dcpmaxp {
    #[doc = "Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
    #[must_use]
    #[inline(always)]
    pub const fn mxps(&self) -> super::vals::Mxps {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Mxps::from_bits(val as u8)
    }
    #[doc = "Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
    #[inline(always)]
    pub const fn set_mxps(&mut self, val: super::vals::Mxps) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u16) & 0x7f) << 0usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u16) & 0x1f) << 7usize);
    }
    #[doc = "Device Select"]
    #[must_use]
    #[inline(always)]
    pub const fn devsel(&self) -> super::vals::DcpmaxpDevsel {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::DcpmaxpDevsel::from_bits(val as u8)
    }
    #[doc = "Device Select"]
    #[inline(always)]
    pub const fn set_devsel(&mut self, val: super::vals::DcpmaxpDevsel) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Dcpmaxp {
    #[inline(always)]
    fn default() -> Dcpmaxp {
        Dcpmaxp(0)
    }
}
impl core::fmt::Debug for Dcpmaxp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcpmaxp")
            .field("mxps", &self.mxps())
            .field("reserved", &self.reserved())
            .field("devsel", &self.devsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcpmaxp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcpmaxp {{ mxps: {:?}, reserved: {=u8:?}, devsel: {:?} }}",
            self.mxps(),
            self.reserved(),
            self.devsel()
        )
    }
}
#[doc = "Device Address %s Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devadd(pub u16);
impl Devadd {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "Transfer Speed of Communication Target Device"]
    #[must_use]
    #[inline(always)]
    pub const fn usbspd(&self) -> super::vals::Usbspd {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Usbspd::from_bits(val as u8)
    }
    #[doc = "Transfer Speed of Communication Target Device"]
    #[inline(always)]
    pub const fn set_usbspd(&mut self, val: super::vals::Usbspd) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Devadd {
    #[inline(always)]
    fn default() -> Devadd {
        Devadd(0)
    }
}
impl core::fmt::Debug for Devadd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Devadd")
            .field("reserved", &self.reserved())
            .field("usbspd", &self.usbspd())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Devadd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Devadd {{ reserved: {=u8:?}, usbspd: {:?}, reserved_2: {=u8:?} }}",
            self.reserved(),
            self.usbspd(),
            self.reserved_2()
        )
    }
}
#[doc = "Device State Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dvstctr0(pub u16);
impl Dvstctr0 {
    #[doc = "USB Bus Reset Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rhst(&self) -> super::vals::Rhst {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Rhst::from_bits(val as u8)
    }
    #[doc = "USB Bus Reset Status"]
    #[inline(always)]
    pub const fn set_rhst(&mut self, val: super::vals::Rhst) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u16) & 0x07) << 0usize);
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
    #[doc = "USB Bus Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uact(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "USB Bus Enable"]
    #[inline(always)]
    pub const fn set_uact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Resume Output"]
    #[must_use]
    #[inline(always)]
    pub const fn resume(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Output"]
    #[inline(always)]
    pub const fn set_resume(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "USB Bus Reset Output"]
    #[must_use]
    #[inline(always)]
    pub const fn usbrst(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "USB Bus Reset Output"]
    #[inline(always)]
    pub const fn set_usbrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Wakeup Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rwupe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup Detection Enable"]
    #[inline(always)]
    pub const fn set_rwupe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Wakeup Output"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup Output"]
    #[inline(always)]
    pub const fn set_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "USB_VBUSEN Output Pin Control"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "USB_VBUSEN Output Pin Control"]
    #[inline(always)]
    pub const fn set_vbusen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "USB_EXICEN Output Pin Control"]
    #[must_use]
    #[inline(always)]
    pub const fn exicen(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "USB_EXICEN Output Pin Control"]
    #[inline(always)]
    pub const fn set_exicen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Host Negotiation Protocol (HNP) Control This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set."]
    #[must_use]
    #[inline(always)]
    pub const fn hnpbtoa(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Host Negotiation Protocol (HNP) Control This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set."]
    #[inline(always)]
    pub const fn set_hnpbtoa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
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
impl Default for Dvstctr0 {
    #[inline(always)]
    fn default() -> Dvstctr0 {
        Dvstctr0(0)
    }
}
impl core::fmt::Debug for Dvstctr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dvstctr0")
            .field("rhst", &self.rhst())
            .field("reserved", &self.reserved())
            .field("uact", &self.uact())
            .field("resume", &self.resume())
            .field("usbrst", &self.usbrst())
            .field("rwupe", &self.rwupe())
            .field("wkup", &self.wkup())
            .field("vbusen", &self.vbusen())
            .field("exicen", &self.exicen())
            .field("hnpbtoa", &self.hnpbtoa())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dvstctr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dvstctr0 {{ rhst: {:?}, reserved: {=bool:?}, uact: {=bool:?}, resume: {=bool:?}, usbrst: {=bool:?}, rwupe: {=bool:?}, wkup: {=bool:?}, vbusen: {=bool:?}, exicen: {=bool:?}, hnpbtoa: {=bool:?}, reserved_2: {=u8:?} }}",
            self.rhst(),
            self.reserved(),
            self.uact(),
            self.resume(),
            self.usbrst(),
            self.rwupe(),
            self.wkup(),
            self.vbusen(),
            self.exicen(),
            self.hnpbtoa(),
            self.reserved_2()
        )
    }
}
#[doc = "Frame Number Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frmnum(pub u16);
impl Frmnum {
    #[doc = "Frame Number Latest frame number"]
    #[must_use]
    #[inline(always)]
    pub const fn frnm(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Frame Number Latest frame number"]
    #[inline(always)]
    pub const fn set_frnm(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u16) & 0x07) << 11usize);
    }
    #[doc = "Receive Data Error"]
    #[must_use]
    #[inline(always)]
    pub const fn crce(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Error"]
    #[inline(always)]
    pub const fn set_crce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Overrun/Underrun Detection Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun/Underrun Detection Status"]
    #[inline(always)]
    pub const fn set_ovrn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Frmnum {
    #[inline(always)]
    fn default() -> Frmnum {
        Frmnum(0)
    }
}
impl core::fmt::Debug for Frmnum {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frmnum")
            .field("frnm", &self.frnm())
            .field("reserved", &self.reserved())
            .field("crce", &self.crce())
            .field("ovrn", &self.ovrn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frmnum {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frmnum {{ frnm: {=u16:?}, reserved: {=u8:?}, crce: {=bool:?}, ovrn: {=bool:?} }}",
            self.frnm(),
            self.reserved(),
            self.crce(),
            self.ovrn()
        )
    }
}
#[doc = "Interrupt Enable Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenb0(pub u16);
impl Intenb0 {
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
    #[doc = "Buffer Ready Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn brdye(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Ready Interrupt Enable"]
    #[inline(always)]
    pub const fn set_brdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Buffer Not Ready Response Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nrdye(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Not Ready Response Interrupt Enable"]
    #[inline(always)]
    pub const fn set_nrdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Buffer Empty Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bempe(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub const fn set_bempe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Control Transfer Stage Transition Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ctre(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Control Transfer Stage Transition Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ctre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Device State Transition Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dvse(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device State Transition Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dvse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Frame Number Update Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sofe(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Number Update Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sofe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Resume Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rsme(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rsme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "VBUS Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbse(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Interrupt Enable"]
    #[inline(always)]
    pub const fn set_vbse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Intenb0 {
    #[inline(always)]
    fn default() -> Intenb0 {
        Intenb0(0)
    }
}
impl core::fmt::Debug for Intenb0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenb0")
            .field("reserved", &self.reserved())
            .field("brdye", &self.brdye())
            .field("nrdye", &self.nrdye())
            .field("bempe", &self.bempe())
            .field("ctre", &self.ctre())
            .field("dvse", &self.dvse())
            .field("sofe", &self.sofe())
            .field("rsme", &self.rsme())
            .field("vbse", &self.vbse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenb0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenb0 {{ reserved: {=u8:?}, brdye: {=bool:?}, nrdye: {=bool:?}, bempe: {=bool:?}, ctre: {=bool:?}, dvse: {=bool:?}, sofe: {=bool:?}, rsme: {=bool:?}, vbse: {=bool:?} }}",
            self.reserved(),
            self.brdye(),
            self.nrdye(),
            self.bempe(),
            self.ctre(),
            self.dvse(),
            self.sofe(),
            self.rsme(),
            self.vbse()
        )
    }
}
#[doc = "Interrupt Enable Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenb1(pub u16);
impl Intenb1 {
    #[doc = "PDDETINT0 Detection Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pddetinte0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PDDETINT0 Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_pddetinte0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
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
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u16) & 0x07) << 1usize);
    }
    #[doc = "Setup Transaction Normal Response Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sacke(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Setup Transaction Normal Response Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sacke(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Setup Transaction Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn signe(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Setup Transaction Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_signe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "EOF Error Detection Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eoferre(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "EOF Error Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_eoferre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u16) & 0x0f) << 7usize);
    }
    #[doc = "Connection Detection Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn attche(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Connection Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_attche(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Disconnection Detection Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtche(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Disconnection Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dtche(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "USB Bus Change Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bchge(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "USB Bus Change Interrupt Enable"]
    #[inline(always)]
    pub const fn set_bchge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Overcurrent Input Change Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrcre(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Overcurrent Input Change Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ovrcre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Intenb1 {
    #[inline(always)]
    fn default() -> Intenb1 {
        Intenb1(0)
    }
}
impl core::fmt::Debug for Intenb1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenb1")
            .field("pddetinte0", &self.pddetinte0())
            .field("reserved", &self.reserved())
            .field("sacke", &self.sacke())
            .field("signe", &self.signe())
            .field("eoferre", &self.eoferre())
            .field("reserved_2", &self.reserved_2())
            .field("attche", &self.attche())
            .field("dtche", &self.dtche())
            .field("reserved_3", &self.reserved_3())
            .field("bchge", &self.bchge())
            .field("ovrcre", &self.ovrcre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenb1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenb1 {{ pddetinte0: {=bool:?}, reserved: {=u8:?}, sacke: {=bool:?}, signe: {=bool:?}, eoferre: {=bool:?}, reserved_2: {=u8:?}, attche: {=bool:?}, dtche: {=bool:?}, reserved_3: {=bool:?}, bchge: {=bool:?}, ovrcre: {=bool:?} }}",
            self.pddetinte0(),
            self.reserved(),
            self.sacke(),
            self.signe(),
            self.eoferre(),
            self.reserved_2(),
            self.attche(),
            self.dtche(),
            self.reserved_3(),
            self.bchge(),
            self.ovrcre()
        )
    }
}
#[doc = "Interrupt Status Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intsts0(pub u16);
impl Intsts0 {
    #[doc = "Control Transfer Stage"]
    #[must_use]
    #[inline(always)]
    pub const fn ctsq(&self) -> super::vals::Ctsq {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctsq::from_bits(val as u8)
    }
    #[doc = "Control Transfer Stage"]
    #[inline(always)]
    pub const fn set_ctsq(&mut self, val: super::vals::Ctsq) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u16) & 0x07) << 0usize);
    }
    #[doc = "USB Request Reception"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "USB Request Reception"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Device State"]
    #[must_use]
    #[inline(always)]
    pub const fn dvsq(&self) -> super::vals::Dvsq {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Dvsq::from_bits(val as u8)
    }
    #[doc = "Device State"]
    #[inline(always)]
    pub const fn set_dvsq(&mut self, val: super::vals::Dvsq) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "VBUS Input Status"]
    #[must_use]
    #[inline(always)]
    pub const fn vbsts(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Input Status"]
    #[inline(always)]
    pub const fn set_vbsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Buffer Ready Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn brdy(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Ready Interrupt Status"]
    #[inline(always)]
    pub const fn set_brdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Buffer Not Ready Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn nrdy(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Not Ready Interrupt Status"]
    #[inline(always)]
    pub const fn set_nrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Buffer Empty Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bemp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Empty Interrupt Status"]
    #[inline(always)]
    pub const fn set_bemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Control Transfer Stage Transition Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrt(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Control Transfer Stage Transition Interrupt Status"]
    #[inline(always)]
    pub const fn set_ctrt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Device State Transition Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn dvst(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device State Transition Interrupt Status"]
    #[inline(always)]
    pub const fn set_dvst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Frame Number Refresh Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sofr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Number Refresh Interrupt Status"]
    #[inline(always)]
    pub const fn set_sofr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Resume Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn resm(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt Status"]
    #[inline(always)]
    pub const fn set_resm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "VBUS Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn vbint(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Interrupt Status"]
    #[inline(always)]
    pub const fn set_vbint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Intsts0 {
    #[inline(always)]
    fn default() -> Intsts0 {
        Intsts0(0)
    }
}
impl core::fmt::Debug for Intsts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intsts0")
            .field("ctsq", &self.ctsq())
            .field("valid", &self.valid())
            .field("dvsq", &self.dvsq())
            .field("vbsts", &self.vbsts())
            .field("brdy", &self.brdy())
            .field("nrdy", &self.nrdy())
            .field("bemp", &self.bemp())
            .field("ctrt", &self.ctrt())
            .field("dvst", &self.dvst())
            .field("sofr", &self.sofr())
            .field("resm", &self.resm())
            .field("vbint", &self.vbint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intsts0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intsts0 {{ ctsq: {:?}, valid: {=bool:?}, dvsq: {:?}, vbsts: {=bool:?}, brdy: {=bool:?}, nrdy: {=bool:?}, bemp: {=bool:?}, ctrt: {=bool:?}, dvst: {=bool:?}, sofr: {=bool:?}, resm: {=bool:?}, vbint: {=bool:?} }}",
            self.ctsq(),
            self.valid(),
            self.dvsq(),
            self.vbsts(),
            self.brdy(),
            self.nrdy(),
            self.bemp(),
            self.ctrt(),
            self.dvst(),
            self.sofr(),
            self.resm(),
            self.vbint()
        )
    }
}
#[doc = "Interrupt Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intsts1(pub u16);
impl Intsts1 {
    #[doc = "PDDET0 Detection Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn pddetint0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PDDET0 Detection Interrupt Status"]
    #[inline(always)]
    pub const fn set_pddetint0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
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
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u16) & 0x07) << 1usize);
    }
    #[doc = "Setup Transaction Normal Response Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sack(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Setup Transaction Normal Response Interrupt Status"]
    #[inline(always)]
    pub const fn set_sack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Setup Transaction Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sign(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Setup Transaction Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_sign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "EOF Error Detection Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn eoferr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "EOF Error Detection Interrupt Status"]
    #[inline(always)]
    pub const fn set_eoferr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u16) & 0x0f) << 7usize);
    }
    #[doc = "ATTCH Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn attch(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "ATTCH Interrupt Status"]
    #[inline(always)]
    pub const fn set_attch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "USB Disconnection Detection Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn dtch(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "USB Disconnection Detection Interrupt Status"]
    #[inline(always)]
    pub const fn set_dtch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "USB Bus Change Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bchg(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "USB Bus Change Interrupt Status"]
    #[inline(always)]
    pub const fn set_bchg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Overcurrent Input Change Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrcr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Overcurrent Input Change Interrupt Status"]
    #[inline(always)]
    pub const fn set_ovrcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Intsts1 {
    #[inline(always)]
    fn default() -> Intsts1 {
        Intsts1(0)
    }
}
impl core::fmt::Debug for Intsts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intsts1")
            .field("pddetint0", &self.pddetint0())
            .field("reserved", &self.reserved())
            .field("sack", &self.sack())
            .field("sign", &self.sign())
            .field("eoferr", &self.eoferr())
            .field("reserved_2", &self.reserved_2())
            .field("attch", &self.attch())
            .field("dtch", &self.dtch())
            .field("reserved_3", &self.reserved_3())
            .field("bchg", &self.bchg())
            .field("ovrcr", &self.ovrcr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intsts1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intsts1 {{ pddetint0: {=bool:?}, reserved: {=u8:?}, sack: {=bool:?}, sign: {=bool:?}, eoferr: {=bool:?}, reserved_2: {=u8:?}, attch: {=bool:?}, dtch: {=bool:?}, reserved_3: {=bool:?}, bchg: {=bool:?}, ovrcr: {=bool:?} }}",
            self.pddetint0(),
            self.reserved(),
            self.sack(),
            self.sign(),
            self.eoferr(),
            self.reserved_2(),
            self.attch(),
            self.dtch(),
            self.reserved_3(),
            self.bchg(),
            self.ovrcr()
        )
    }
}
#[doc = "NRDY Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nrdyenb(pub u16);
impl Nrdyenb {
    #[doc = "NRDY Interrupt Enable for PIPE0"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe0nrdye(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NRDY Interrupt Enable for PIPE0"]
    #[inline(always)]
    pub const fn set_pipe0nrdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE1"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe1nrdye(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NRDY Interrupt Enable for PIPE1"]
    #[inline(always)]
    pub const fn set_pipe1nrdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE2"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe2nrdye(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NRDY Interrupt Enable for PIPE2"]
    #[inline(always)]
    pub const fn set_pipe2nrdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE3"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe3nrdye(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "NRDY Interrupt Enable for PIPE3"]
    #[inline(always)]
    pub const fn set_pipe3nrdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE4"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe4nrdye(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NRDY Interrupt Enable for PIPE4"]
    #[inline(always)]
    pub const fn set_pipe4nrdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE5"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe5nrdye(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NRDY Interrupt Enable for PIPE5"]
    #[inline(always)]
    pub const fn set_pipe5nrdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE6"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe6nrdye(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NRDY Interrupt Enable for PIPE6"]
    #[inline(always)]
    pub const fn set_pipe6nrdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE7"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe7nrdye(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "NRDY Interrupt Enable for PIPE7"]
    #[inline(always)]
    pub const fn set_pipe7nrdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE8"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe8nrdye(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "NRDY Interrupt Enable for PIPE8"]
    #[inline(always)]
    pub const fn set_pipe8nrdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE9"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe9nrdye(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "NRDY Interrupt Enable for PIPE9"]
    #[inline(always)]
    pub const fn set_pipe9nrdye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Nrdyenb {
    #[inline(always)]
    fn default() -> Nrdyenb {
        Nrdyenb(0)
    }
}
impl core::fmt::Debug for Nrdyenb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nrdyenb")
            .field("pipe0nrdye", &self.pipe0nrdye())
            .field("pipe1nrdye", &self.pipe1nrdye())
            .field("pipe2nrdye", &self.pipe2nrdye())
            .field("pipe3nrdye", &self.pipe3nrdye())
            .field("pipe4nrdye", &self.pipe4nrdye())
            .field("pipe5nrdye", &self.pipe5nrdye())
            .field("pipe6nrdye", &self.pipe6nrdye())
            .field("pipe7nrdye", &self.pipe7nrdye())
            .field("pipe8nrdye", &self.pipe8nrdye())
            .field("pipe9nrdye", &self.pipe9nrdye())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nrdyenb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nrdyenb {{ pipe0nrdye: {=bool:?}, pipe1nrdye: {=bool:?}, pipe2nrdye: {=bool:?}, pipe3nrdye: {=bool:?}, pipe4nrdye: {=bool:?}, pipe5nrdye: {=bool:?}, pipe6nrdye: {=bool:?}, pipe7nrdye: {=bool:?}, pipe8nrdye: {=bool:?}, pipe9nrdye: {=bool:?}, reserved: {=u8:?} }}",
            self.pipe0nrdye(),
            self.pipe1nrdye(),
            self.pipe2nrdye(),
            self.pipe3nrdye(),
            self.pipe4nrdye(),
            self.pipe5nrdye(),
            self.pipe6nrdye(),
            self.pipe7nrdye(),
            self.pipe8nrdye(),
            self.pipe9nrdye(),
            self.reserved()
        )
    }
}
#[doc = "NRDY Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nrdysts(pub u16);
impl Nrdysts {
    #[doc = "NRDY Interrupt Status for PIPE0"]
    #[must_use]
    #[inline(always)]
    pub const fn nrdy(&self, n: usize) -> bool {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "NRDY Interrupt Status for PIPE0"]
    #[inline(always)]
    pub const fn set_nrdy(&mut self, n: usize, val: bool) {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Nrdysts {
    #[inline(always)]
    fn default() -> Nrdysts {
        Nrdysts(0)
    }
}
impl core::fmt::Debug for Nrdysts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nrdysts")
            .field("nrdy[0]", &self.nrdy(0usize))
            .field("nrdy[1]", &self.nrdy(1usize))
            .field("nrdy[2]", &self.nrdy(2usize))
            .field("nrdy[3]", &self.nrdy(3usize))
            .field("nrdy[4]", &self.nrdy(4usize))
            .field("nrdy[5]", &self.nrdy(5usize))
            .field("nrdy[6]", &self.nrdy(6usize))
            .field("nrdy[7]", &self.nrdy(7usize))
            .field("nrdy[8]", &self.nrdy(8usize))
            .field("nrdy[9]", &self.nrdy(9usize))
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nrdysts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nrdysts {{ nrdy[0]: {=bool:?}, nrdy[1]: {=bool:?}, nrdy[2]: {=bool:?}, nrdy[3]: {=bool:?}, nrdy[4]: {=bool:?}, nrdy[5]: {=bool:?}, nrdy[6]: {=bool:?}, nrdy[7]: {=bool:?}, nrdy[8]: {=bool:?}, nrdy[9]: {=bool:?}, reserved: {=u8:?} }}",
            self.nrdy(0usize),
            self.nrdy(1usize),
            self.nrdy(2usize),
            self.nrdy(3usize),
            self.nrdy(4usize),
            self.nrdy(5usize),
            self.nrdy(6usize),
            self.nrdy(7usize),
            self.nrdy(8usize),
            self.nrdy(9usize),
            self.reserved()
        )
    }
}
#[doc = "Pipe Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipecfg(pub u16);
impl Pipecfg {
    #[doc = "Endpoint Number These bits specify the endpoint number for the selected pipe. Setting 0000b means unused pipe."]
    #[must_use]
    #[inline(always)]
    pub const fn epnum(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Endpoint Number These bits specify the endpoint number for the selected pipe. Setting 0000b means unused pipe."]
    #[inline(always)]
    pub const fn set_epnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Transfer Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
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
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
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
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Pipe Disabled at End of Transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn shtnak(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Pipe Disabled at End of Transfer"]
    #[inline(always)]
    pub const fn set_shtnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Double Buffer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dblb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Double Buffer Mode"]
    #[inline(always)]
    pub const fn set_dblb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "BRDY Interrupt Operation Specification"]
    #[must_use]
    #[inline(always)]
    pub const fn bfre(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "BRDY Interrupt Operation Specification"]
    #[inline(always)]
    pub const fn set_bfre(&mut self, val: bool) {
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
    #[doc = "Transfer Type"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::Type {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Type::from_bits(val as u8)
    }
    #[doc = "Transfer Type"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: super::vals::Type) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Pipecfg {
    #[inline(always)]
    fn default() -> Pipecfg {
        Pipecfg(0)
    }
}
impl core::fmt::Debug for Pipecfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pipecfg")
            .field("epnum", &self.epnum())
            .field("dir", &self.dir())
            .field("reserved", &self.reserved())
            .field("reserved_2", &self.reserved_2())
            .field("shtnak", &self.shtnak())
            .field("reserved_3", &self.reserved_3())
            .field("dblb", &self.dblb())
            .field("bfre", &self.bfre())
            .field("reserved_4", &self.reserved_4())
            .field("type_", &self.type_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pipecfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pipecfg {{ epnum: {=u8:?}, dir: {=bool:?}, reserved: {=bool:?}, reserved_2: {=bool:?}, shtnak: {=bool:?}, reserved_3: {=bool:?}, dblb: {=bool:?}, bfre: {=bool:?}, reserved_4: {=u8:?}, type_: {:?} }}",
            self.epnum(),
            self.dir(),
            self.reserved(),
            self.reserved_2(),
            self.shtnak(),
            self.reserved_3(),
            self.dblb(),
            self.bfre(),
            self.reserved_4(),
            self.type_()
        )
    }
}
#[doc = "Pipe %s Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipectr(pub u16);
impl Pipectr {
    #[doc = "Response PID"]
    #[must_use]
    #[inline(always)]
    pub const fn pid(&self) -> super::vals::PipectrPid {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::PipectrPid::from_bits(val as u8)
    }
    #[doc = "Response PID"]
    #[inline(always)]
    pub const fn set_pid(&mut self, val: super::vals::PipectrPid) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
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
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u16) & 0x07) << 2usize);
    }
    #[doc = "Pipe Busy"]
    #[must_use]
    #[inline(always)]
    pub const fn pbusy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pipe Busy"]
    #[inline(always)]
    pub const fn set_pbusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Sequence Toggle Bit Confirmation"]
    #[must_use]
    #[inline(always)]
    pub const fn sqmon(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Toggle Bit Confirmation"]
    #[inline(always)]
    pub const fn set_sqmon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Sequence Toggle Bit Set"]
    #[must_use]
    #[inline(always)]
    pub const fn sqset(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Toggle Bit Set"]
    #[inline(always)]
    pub const fn set_sqset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Sequence Toggle Bit Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sqclr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Toggle Bit Clear"]
    #[inline(always)]
    pub const fn set_sqclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Auto Buffer Clear Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn aclrm(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Buffer Clear Mode"]
    #[inline(always)]
    pub const fn set_aclrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Auto Response Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn atrepm(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Response Mode"]
    #[inline(always)]
    pub const fn set_atrepm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u16) & 0x07) << 11usize);
    }
    #[doc = "Transmit Buffer Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn inbufm(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer Monitor"]
    #[inline(always)]
    pub const fn set_inbufm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Buffer Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bsts(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Status"]
    #[inline(always)]
    pub const fn set_bsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Pipectr {
    #[inline(always)]
    fn default() -> Pipectr {
        Pipectr(0)
    }
}
impl core::fmt::Debug for Pipectr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pipectr")
            .field("pid", &self.pid())
            .field("reserved", &self.reserved())
            .field("pbusy", &self.pbusy())
            .field("sqmon", &self.sqmon())
            .field("sqset", &self.sqset())
            .field("sqclr", &self.sqclr())
            .field("aclrm", &self.aclrm())
            .field("atrepm", &self.atrepm())
            .field("reserved_2", &self.reserved_2())
            .field("inbufm", &self.inbufm())
            .field("bsts", &self.bsts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pipectr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pipectr {{ pid: {:?}, reserved: {=u8:?}, pbusy: {=bool:?}, sqmon: {=bool:?}, sqset: {=bool:?}, sqclr: {=bool:?}, aclrm: {=bool:?}, atrepm: {=bool:?}, reserved_2: {=u8:?}, inbufm: {=bool:?}, bsts: {=bool:?} }}",
            self.pid(),
            self.reserved(),
            self.pbusy(),
            self.sqmon(),
            self.sqset(),
            self.sqclr(),
            self.aclrm(),
            self.atrepm(),
            self.reserved_2(),
            self.inbufm(),
            self.bsts()
        )
    }
}
#[doc = "Pipe %s Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipectr2(pub u16);
impl Pipectr2 {
    #[doc = "Response PID"]
    #[must_use]
    #[inline(always)]
    pub const fn pid(&self) -> super::vals::Pipectr2Pid {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Pipectr2Pid::from_bits(val as u8)
    }
    #[doc = "Response PID"]
    #[inline(always)]
    pub const fn set_pid(&mut self, val: super::vals::Pipectr2Pid) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
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
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u16) & 0x07) << 2usize);
    }
    #[doc = "Pipe Busy"]
    #[must_use]
    #[inline(always)]
    pub const fn pbusy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pipe Busy"]
    #[inline(always)]
    pub const fn set_pbusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Sequence Toggle Bit Confirmation"]
    #[must_use]
    #[inline(always)]
    pub const fn sqmon(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Toggle Bit Confirmation"]
    #[inline(always)]
    pub const fn set_sqmon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Sequence Toggle Bit Set"]
    #[must_use]
    #[inline(always)]
    pub const fn sqset(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Toggle Bit Set"]
    #[inline(always)]
    pub const fn set_sqset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Sequence Toggle Bit Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sqclr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Toggle Bit Clear"]
    #[inline(always)]
    pub const fn set_sqclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Auto Buffer Clear Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn aclrm(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Buffer Clear Mode"]
    #[inline(always)]
    pub const fn set_aclrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u16) & 0x1f) << 10usize);
    }
    #[doc = "Buffer Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bsts(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Status"]
    #[inline(always)]
    pub const fn set_bsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Pipectr2 {
    #[inline(always)]
    fn default() -> Pipectr2 {
        Pipectr2(0)
    }
}
impl core::fmt::Debug for Pipectr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pipectr2")
            .field("pid", &self.pid())
            .field("reserved", &self.reserved())
            .field("pbusy", &self.pbusy())
            .field("sqmon", &self.sqmon())
            .field("sqset", &self.sqset())
            .field("sqclr", &self.sqclr())
            .field("aclrm", &self.aclrm())
            .field("reserved_2", &self.reserved_2())
            .field("bsts", &self.bsts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pipectr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pipectr2 {{ pid: {:?}, reserved: {=u8:?}, pbusy: {=bool:?}, sqmon: {=bool:?}, sqset: {=bool:?}, sqclr: {=bool:?}, aclrm: {=bool:?}, reserved_2: {=u8:?}, bsts: {=bool:?} }}",
            self.pid(),
            self.reserved(),
            self.pbusy(),
            self.sqmon(),
            self.sqset(),
            self.sqclr(),
            self.aclrm(),
            self.reserved_2(),
            self.bsts()
        )
    }
}
#[doc = "Pipe Maximum Packet Size Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipemaxp(pub u16);
impl Pipemaxp {
    #[doc = "Maximum Packet Size PIPE1 and PIPE2: 1 byte (001h) to 256 bytes (100h) PIPE3 to PIPE5: 8 bytes (008h), 16 bytes (010h), 32 bytes (020h), 64 bytes (040h) (Bits \\[8:7\\] and \\[2:0\\] are not provided.) PIPE6 to PIPE9: 1 byte (001h) to 64 bytes (040h) (Bits \\[8:7\\] are not provided.)"]
    #[must_use]
    #[inline(always)]
    pub const fn mxps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Maximum Packet Size PIPE1 and PIPE2: 1 byte (001h) to 256 bytes (100h) PIPE3 to PIPE5: 8 bytes (008h), 16 bytes (010h), 32 bytes (020h), 64 bytes (040h) (Bits \\[8:7\\] and \\[2:0\\] are not provided.) PIPE6 to PIPE9: 1 byte (001h) to 64 bytes (040h) (Bits \\[8:7\\] are not provided.)"]
    #[inline(always)]
    pub const fn set_mxps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u16) & 0x07) << 9usize);
    }
    #[doc = "Device Select"]
    #[must_use]
    #[inline(always)]
    pub const fn devsel(&self) -> super::vals::PipemaxpDevsel {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::PipemaxpDevsel::from_bits(val as u8)
    }
    #[doc = "Device Select"]
    #[inline(always)]
    pub const fn set_devsel(&mut self, val: super::vals::PipemaxpDevsel) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Pipemaxp {
    #[inline(always)]
    fn default() -> Pipemaxp {
        Pipemaxp(0)
    }
}
impl core::fmt::Debug for Pipemaxp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pipemaxp")
            .field("mxps", &self.mxps())
            .field("reserved", &self.reserved())
            .field("devsel", &self.devsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pipemaxp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pipemaxp {{ mxps: {=u16:?}, reserved: {=u8:?}, devsel: {:?} }}",
            self.mxps(),
            self.reserved(),
            self.devsel()
        )
    }
}
#[doc = "Pipe Cycle Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipeperi(pub u16);
impl Pipeperi {
    #[doc = "Interval Error Detection Interval Specifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
    #[must_use]
    #[inline(always)]
    pub const fn iitv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Interval Error Detection Interval Specifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
    #[inline(always)]
    pub const fn set_iitv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "These bits are read as 000000000. The write value should be 000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x01ff;
        val as u16
    }
    #[doc = "These bits are read as 000000000. The write value should be 000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 3usize)) | (((val as u16) & 0x01ff) << 3usize);
    }
    #[doc = "Isochronous IN Buffer Flush"]
    #[must_use]
    #[inline(always)]
    pub const fn ifis(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Isochronous IN Buffer Flush"]
    #[inline(always)]
    pub const fn set_ifis(&mut self, val: bool) {
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
impl Default for Pipeperi {
    #[inline(always)]
    fn default() -> Pipeperi {
        Pipeperi(0)
    }
}
impl core::fmt::Debug for Pipeperi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pipeperi")
            .field("iitv", &self.iitv())
            .field("reserved", &self.reserved())
            .field("ifis", &self.ifis())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pipeperi {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pipeperi {{ iitv: {=u8:?}, reserved: {=u16:?}, ifis: {=bool:?}, reserved_2: {=u8:?} }}",
            self.iitv(),
            self.reserved(),
            self.ifis(),
            self.reserved_2()
        )
    }
}
#[doc = "Pipe Window Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipesel(pub u16);
impl Pipesel {
    #[doc = "Pipe Window Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pipesel(&self) -> super::vals::Pipesel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pipesel::from_bits(val as u8)
    }
    #[doc = "Pipe Window Select"]
    #[inline(always)]
    pub const fn set_pipesel(&mut self, val: super::vals::Pipesel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u16) & 0x0fff) << 4usize);
    }
}
impl Default for Pipesel {
    #[inline(always)]
    fn default() -> Pipesel {
        Pipesel(0)
    }
}
impl core::fmt::Debug for Pipesel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pipesel")
            .field("pipesel", &self.pipesel())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pipesel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pipesel {{ pipesel: {:?}, reserved: {=u16:?} }}",
            self.pipesel(),
            self.reserved()
        )
    }
}
#[doc = "Pipe %s Transaction Counter Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipetre(pub u16);
impl Pipetre {
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
    #[doc = "Transaction Counter Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn trclr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transaction Counter Clear"]
    #[inline(always)]
    pub const fn set_trclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Transaction Counter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn trenb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transaction Counter Enable"]
    #[inline(always)]
    pub const fn set_trenb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Pipetre {
    #[inline(always)]
    fn default() -> Pipetre {
        Pipetre(0)
    }
}
impl core::fmt::Debug for Pipetre {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pipetre")
            .field("reserved", &self.reserved())
            .field("trclr", &self.trclr())
            .field("trenb", &self.trenb())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pipetre {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pipetre {{ reserved: {=u8:?}, trclr: {=bool:?}, trenb: {=bool:?}, reserved_2: {=u8:?} }}",
            self.reserved(),
            self.trclr(),
            self.trenb(),
            self.reserved_2()
        )
    }
}
#[doc = "Pipe %s Transaction Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipetrn(pub u16);
impl Pipetrn {
    #[doc = "Transaction Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn trncnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transaction Counter"]
    #[inline(always)]
    pub const fn set_trncnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Pipetrn {
    #[inline(always)]
    fn default() -> Pipetrn {
        Pipetrn(0)
    }
}
impl core::fmt::Debug for Pipetrn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pipetrn")
            .field("trncnt", &self.trncnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pipetrn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pipetrn {{ trncnt: {=u16:?} }}", self.trncnt())
    }
}
#[doc = "SOF Output Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sofcfg(pub u16);
impl Sofcfg {
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
    #[doc = "Edge Interrupt Output Status Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn edgests(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Interrupt Output Status Monitor"]
    #[inline(always)]
    pub const fn set_edgests(&mut self, val: bool) {
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
    #[doc = "BRDY Interrupt Status Clear Timing"]
    #[must_use]
    #[inline(always)]
    pub const fn brdym(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "BRDY Interrupt Status Clear Timing"]
    #[inline(always)]
    pub const fn set_brdym(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
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
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Transaction-Enabled Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn trnensel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transaction-Enabled Time Select"]
    #[inline(always)]
    pub const fn set_trnensel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u16) & 0x7f) << 9usize);
    }
}
impl Default for Sofcfg {
    #[inline(always)]
    fn default() -> Sofcfg {
        Sofcfg(0)
    }
}
impl core::fmt::Debug for Sofcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sofcfg")
            .field("reserved", &self.reserved())
            .field("edgests", &self.edgests())
            .field("reserved_2", &self.reserved_2())
            .field("brdym", &self.brdym())
            .field("reserved_3", &self.reserved_3())
            .field("trnensel", &self.trnensel())
            .field("reserved_4", &self.reserved_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sofcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sofcfg {{ reserved: {=u8:?}, edgests: {=bool:?}, reserved_2: {=bool:?}, brdym: {=bool:?}, reserved_3: {=bool:?}, trnensel: {=bool:?}, reserved_4: {=u8:?} }}",
            self.reserved(),
            self.edgests(),
            self.reserved_2(),
            self.brdym(),
            self.reserved_3(),
            self.trnensel(),
            self.reserved_4()
        )
    }
}
#[doc = "System Configuration Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg(pub u16);
impl Syscfg {
    #[doc = "USB Operation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usbe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB Operation Enable"]
    #[inline(always)]
    pub const fn set_usbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "D- Line Resistor Control"]
    #[must_use]
    #[inline(always)]
    pub const fn dmrpu(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "D- Line Resistor Control"]
    #[inline(always)]
    pub const fn set_dmrpu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "D+ Line Resistor Control"]
    #[must_use]
    #[inline(always)]
    pub const fn dprpu(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "D+ Line Resistor Control"]
    #[inline(always)]
    pub const fn set_dprpu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "D+/D- Line Resistor Control"]
    #[must_use]
    #[inline(always)]
    pub const fn drpd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "D+/D- Line Resistor Control"]
    #[inline(always)]
    pub const fn set_drpd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Controller Function Select"]
    #[must_use]
    #[inline(always)]
    pub const fn dcfm(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Controller Function Select"]
    #[inline(always)]
    pub const fn set_dcfm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
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
    #[doc = "CNEN Single End Receiver Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cnen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CNEN Single End Receiver Enable"]
    #[inline(always)]
    pub const fn set_cnen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "USB Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn scke(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "USB Clock Enable"]
    #[inline(always)]
    pub const fn set_scke(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Syscfg {
    #[inline(always)]
    fn default() -> Syscfg {
        Syscfg(0)
    }
}
impl core::fmt::Debug for Syscfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syscfg")
            .field("usbe", &self.usbe())
            .field("dmrpu", &self.dmrpu())
            .field("dprpu", &self.dprpu())
            .field("drpd", &self.drpd())
            .field("dcfm", &self.dcfm())
            .field("reserved", &self.reserved())
            .field("cnen", &self.cnen())
            .field("reserved_2", &self.reserved_2())
            .field("scke", &self.scke())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syscfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Syscfg {{ usbe: {=bool:?}, dmrpu: {=bool:?}, dprpu: {=bool:?}, drpd: {=bool:?}, dcfm: {=bool:?}, reserved: {=bool:?}, cnen: {=bool:?}, reserved_2: {=bool:?}, scke: {=bool:?}, reserved_3: {=u8:?} }}",
            self.usbe(),
            self.dmrpu(),
            self.dprpu(),
            self.drpd(),
            self.dcfm(),
            self.reserved(),
            self.cnen(),
            self.reserved_2(),
            self.scke(),
            self.reserved_3()
        )
    }
}
#[doc = "System Configuration Status Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syssts0(pub u16);
impl Syssts0 {
    #[doc = "USB Data Line Status Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn lnst(&self) -> super::vals::Lnst {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lnst::from_bits(val as u8)
    }
    #[doc = "USB Data Line Status Monitor"]
    #[inline(always)]
    pub const fn set_lnst(&mut self, val: super::vals::Lnst) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "External ID0 Input Pin Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn idmon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "External ID0 Input Pin Monitor"]
    #[inline(always)]
    pub const fn set_idmon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "These bits are read as 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u16) & 0x07) << 3usize);
    }
    #[doc = "USB Host Sequencer Status Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn htact(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "USB Host Sequencer Status Monitor"]
    #[inline(always)]
    pub const fn set_htact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "These bits are read as 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u16) & 0x7f) << 7usize);
    }
    #[doc = "External USB0_OVRCURA/ USB0_OVRCURB Input Pin Monitor The OCVMON\\[1\\] bit indicates the status of the USBHS_OVRCURA pin. The OCVMON\\[0\\] bit indicates the status of the USBHS_OVRCURB pin."]
    #[must_use]
    #[inline(always)]
    pub const fn ovcmon(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "External USB0_OVRCURA/ USB0_OVRCURB Input Pin Monitor The OCVMON\\[1\\] bit indicates the status of the USBHS_OVRCURA pin. The OCVMON\\[0\\] bit indicates the status of the USBHS_OVRCURB pin."]
    #[inline(always)]
    pub const fn set_ovcmon(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
    }
}
impl Default for Syssts0 {
    #[inline(always)]
    fn default() -> Syssts0 {
        Syssts0(0)
    }
}
impl core::fmt::Debug for Syssts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syssts0")
            .field("lnst", &self.lnst())
            .field("idmon", &self.idmon())
            .field("reserved", &self.reserved())
            .field("htact", &self.htact())
            .field("reserved_2", &self.reserved_2())
            .field("ovcmon", &self.ovcmon())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syssts0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Syssts0 {{ lnst: {:?}, idmon: {=bool:?}, reserved: {=u8:?}, htact: {=bool:?}, reserved_2: {=u8:?}, ovcmon: {=u8:?} }}",
            self.lnst(),
            self.idmon(),
            self.reserved(),
            self.htact(),
            self.reserved_2(),
            self.ovcmon()
        )
    }
}
#[doc = "BC Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbbcctrl0(pub u16);
impl Usbbcctrl0 {
    #[doc = "D- Pin Pull-Down Control"]
    #[must_use]
    #[inline(always)]
    pub const fn rpdme0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "D- Pin Pull-Down Control"]
    #[inline(always)]
    pub const fn set_rpdme0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "D+ Pin IDPSRC Output Control"]
    #[must_use]
    #[inline(always)]
    pub const fn idpsrce0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "D+ Pin IDPSRC Output Control"]
    #[inline(always)]
    pub const fn set_idpsrce0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "D- Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[must_use]
    #[inline(always)]
    pub const fn idmsinke0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "D- Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[inline(always)]
    pub const fn set_idmsinke0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "D+ Pin VDPSRC (0.6 V) Output Control"]
    #[must_use]
    #[inline(always)]
    pub const fn vdpsrce0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "D+ Pin VDPSRC (0.6 V) Output Control"]
    #[inline(always)]
    pub const fn set_vdpsrce0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "D+ Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[must_use]
    #[inline(always)]
    pub const fn idpsinke0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "D+ Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[inline(always)]
    pub const fn set_idpsinke0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "D- Pin VDMSRC (0.6 V) Output Control"]
    #[must_use]
    #[inline(always)]
    pub const fn vdmsrce0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "D- Pin VDMSRC (0.6 V) Output Control"]
    #[inline(always)]
    pub const fn set_vdmsrce0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
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
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "BC (Battery Charger) Function Ch0 General Enable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn batchge0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "BC (Battery Charger) Function Ch0 General Enable Control"]
    #[inline(always)]
    pub const fn set_batchge0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "D- Pin 0.6 V Input Detection Status"]
    #[must_use]
    #[inline(always)]
    pub const fn chgdetsts0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "D- Pin 0.6 V Input Detection Status"]
    #[inline(always)]
    pub const fn set_chgdetsts0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "D+ Pin 0.6 V Input Detection Status"]
    #[must_use]
    #[inline(always)]
    pub const fn pddetsts0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "D+ Pin 0.6 V Input Detection Status"]
    #[inline(always)]
    pub const fn set_pddetsts0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Usbbcctrl0 {
    #[inline(always)]
    fn default() -> Usbbcctrl0 {
        Usbbcctrl0(0)
    }
}
impl core::fmt::Debug for Usbbcctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbbcctrl0")
            .field("rpdme0", &self.rpdme0())
            .field("idpsrce0", &self.idpsrce0())
            .field("idmsinke0", &self.idmsinke0())
            .field("vdpsrce0", &self.vdpsrce0())
            .field("idpsinke0", &self.idpsinke0())
            .field("vdmsrce0", &self.vdmsrce0())
            .field("reserved", &self.reserved())
            .field("batchge0", &self.batchge0())
            .field("chgdetsts0", &self.chgdetsts0())
            .field("pddetsts0", &self.pddetsts0())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbbcctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbbcctrl0 {{ rpdme0: {=bool:?}, idpsrce0: {=bool:?}, idmsinke0: {=bool:?}, vdpsrce0: {=bool:?}, idpsinke0: {=bool:?}, vdmsrce0: {=bool:?}, reserved: {=bool:?}, batchge0: {=bool:?}, chgdetsts0: {=bool:?}, pddetsts0: {=bool:?}, reserved_2: {=u8:?} }}",
            self.rpdme0(),
            self.idpsrce0(),
            self.idmsinke0(),
            self.vdpsrce0(),
            self.idpsinke0(),
            self.vdmsrce0(),
            self.reserved(),
            self.batchge0(),
            self.chgdetsts0(),
            self.pddetsts0(),
            self.reserved_2()
        )
    }
}
#[doc = "USB Request Index Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbindx(pub u16);
impl Usbindx {
    #[doc = "Index These bits store the USB request wIndex value."]
    #[must_use]
    #[inline(always)]
    pub const fn windex(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Index These bits store the USB request wIndex value."]
    #[inline(always)]
    pub const fn set_windex(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Usbindx {
    #[inline(always)]
    fn default() -> Usbindx {
        Usbindx(0)
    }
}
impl core::fmt::Debug for Usbindx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbindx")
            .field("windex", &self.windex())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbindx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usbindx {{ windex: {=u16:?} }}", self.windex())
    }
}
#[doc = "USB Request Length Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbleng(pub u16);
impl Usbleng {
    #[doc = "Length These bits store the USB request wLength value."]
    #[must_use]
    #[inline(always)]
    pub const fn wlentuh(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Length These bits store the USB request wLength value."]
    #[inline(always)]
    pub const fn set_wlentuh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Usbleng {
    #[inline(always)]
    fn default() -> Usbleng {
        Usbleng(0)
    }
}
impl core::fmt::Debug for Usbleng {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbleng")
            .field("wlentuh", &self.wlentuh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbleng {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usbleng {{ wlentuh: {=u16:?} }}", self.wlentuh())
    }
}
#[doc = "USB Module Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbmc(pub u16);
impl Usbmc {
    #[doc = "USB Reference Power Supply Circuit On/Off Control"]
    #[must_use]
    #[inline(always)]
    pub const fn vddusbe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB Reference Power Supply Circuit On/Off Control"]
    #[inline(always)]
    pub const fn set_vddusbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
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
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u16) & 0x1f) << 2usize);
    }
    #[doc = "USB Regulator On/Off Control"]
    #[must_use]
    #[inline(always)]
    pub const fn vdcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "USB Regulator On/Off Control"]
    #[inline(always)]
    pub const fn set_vdcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Usbmc {
    #[inline(always)]
    fn default() -> Usbmc {
        Usbmc(0)
    }
}
impl core::fmt::Debug for Usbmc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbmc")
            .field("vddusbe", &self.vddusbe())
            .field("reserved", &self.reserved())
            .field("reserved_2", &self.reserved_2())
            .field("vdcen", &self.vdcen())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbmc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbmc {{ vddusbe: {=bool:?}, reserved: {=bool:?}, reserved_2: {=u8:?}, vdcen: {=bool:?}, reserved_3: {=u8:?} }}",
            self.vddusbe(),
            self.reserved(),
            self.reserved_2(),
            self.vdcen(),
            self.reserved_3()
        )
    }
}
#[doc = "USB Request Type Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbreq(pub u16);
impl Usbreq {
    #[doc = "Request Type These bits store the USB request bmRequestType value."]
    #[must_use]
    #[inline(always)]
    pub const fn bmrequesttype(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Request Type These bits store the USB request bmRequestType value."]
    #[inline(always)]
    pub const fn set_bmrequesttype(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Request These bits store the USB request bRequest value."]
    #[must_use]
    #[inline(always)]
    pub const fn brequest(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Request These bits store the USB request bRequest value."]
    #[inline(always)]
    pub const fn set_brequest(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Usbreq {
    #[inline(always)]
    fn default() -> Usbreq {
        Usbreq(0)
    }
}
impl core::fmt::Debug for Usbreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbreq")
            .field("bmrequesttype", &self.bmrequesttype())
            .field("brequest", &self.brequest())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbreq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbreq {{ bmrequesttype: {=u8:?}, brequest: {=u8:?} }}",
            self.bmrequesttype(),
            self.brequest()
        )
    }
}
#[doc = "USB Request Value Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbval(pub u16);
impl Usbval {
    #[doc = "Value These bits store the USB request Value value."]
    #[must_use]
    #[inline(always)]
    pub const fn wvalue(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value These bits store the USB request Value value."]
    #[inline(always)]
    pub const fn set_wvalue(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Usbval {
    #[inline(always)]
    fn default() -> Usbval {
        Usbval(0)
    }
}
impl core::fmt::Debug for Usbval {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbval")
            .field("wvalue", &self.wvalue())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbval {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usbval {{ wvalue: {=u16:?} }}", self.wvalue())
    }
}
