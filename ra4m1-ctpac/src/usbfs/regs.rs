#[doc = "BEMP Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bempenb(pub u16);
impl Bempenb {
    #[doc = "BEMP Interrupt Enable for PIPE0"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe0bempe(&self) -> super::vals::Pipe0bempe {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pipe0bempe::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Enable for PIPE0"]
    #[inline(always)]
    pub const fn set_pipe0bempe(&mut self, val: super::vals::Pipe0bempe) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE1"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe1bempe(&self) -> super::vals::Pipe1bempe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pipe1bempe::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Enable for PIPE1"]
    #[inline(always)]
    pub const fn set_pipe1bempe(&mut self, val: super::vals::Pipe1bempe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE2"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe2bempe(&self) -> super::vals::Pipe2bempe {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pipe2bempe::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Enable for PIPE2"]
    #[inline(always)]
    pub const fn set_pipe2bempe(&mut self, val: super::vals::Pipe2bempe) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE3"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe3bempe(&self) -> super::vals::Pipe3bempe {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pipe3bempe::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Enable for PIPE3"]
    #[inline(always)]
    pub const fn set_pipe3bempe(&mut self, val: super::vals::Pipe3bempe) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE4"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe4bempe(&self) -> super::vals::Pipe4bempe {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pipe4bempe::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Enable for PIPE4"]
    #[inline(always)]
    pub const fn set_pipe4bempe(&mut self, val: super::vals::Pipe4bempe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE5"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe5bempe(&self) -> super::vals::Pipe5bempe {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pipe5bempe::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Enable for PIPE5"]
    #[inline(always)]
    pub const fn set_pipe5bempe(&mut self, val: super::vals::Pipe5bempe) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE6"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe6bempe(&self) -> super::vals::Pipe6bempe {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pipe6bempe::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Enable for PIPE6"]
    #[inline(always)]
    pub const fn set_pipe6bempe(&mut self, val: super::vals::Pipe6bempe) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE7"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe7bempe(&self) -> super::vals::Pipe7bempe {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pipe7bempe::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Enable for PIPE7"]
    #[inline(always)]
    pub const fn set_pipe7bempe(&mut self, val: super::vals::Pipe7bempe) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE8"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe8bempe(&self) -> super::vals::Pipe8bempe {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pipe8bempe::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Enable for PIPE8"]
    #[inline(always)]
    pub const fn set_pipe8bempe(&mut self, val: super::vals::Pipe8bempe) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "BEMP Interrupt Enable for PIPE9"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe9bempe(&self) -> super::vals::Pipe9bempe {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pipe9bempe::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Enable for PIPE9"]
    #[inline(always)]
    pub const fn set_pipe9bempe(&mut self, val: super::vals::Pipe9bempe) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
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
            "Bempenb {{ pipe0bempe: {:?}, pipe1bempe: {:?}, pipe2bempe: {:?}, pipe3bempe: {:?}, pipe4bempe: {:?}, pipe5bempe: {:?}, pipe6bempe: {:?}, pipe7bempe: {:?}, pipe8bempe: {:?}, pipe9bempe: {:?}, reserved: {=u8:?} }}",
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
    pub const fn pipe0bemp(&self) -> super::vals::Pipe0bemp {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pipe0bemp::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Status for PIPE0"]
    #[inline(always)]
    pub const fn set_pipe0bemp(&mut self, val: super::vals::Pipe0bemp) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "BEMP Interrupt Status for PIPE1"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe1bemp(&self) -> super::vals::Pipe1bemp {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pipe1bemp::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Status for PIPE1"]
    #[inline(always)]
    pub const fn set_pipe1bemp(&mut self, val: super::vals::Pipe1bemp) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "BEMP Interrupt Status for PIPE2"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe2bemp(&self) -> super::vals::Pipe2bemp {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pipe2bemp::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Status for PIPE2"]
    #[inline(always)]
    pub const fn set_pipe2bemp(&mut self, val: super::vals::Pipe2bemp) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "BEMP Interrupt Status for PIPE3"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe3bemp(&self) -> super::vals::Pipe3bemp {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pipe3bemp::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Status for PIPE3"]
    #[inline(always)]
    pub const fn set_pipe3bemp(&mut self, val: super::vals::Pipe3bemp) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "BEMP Interrupt Status for PIPE4"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe4bemp(&self) -> super::vals::Pipe4bemp {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pipe4bemp::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Status for PIPE4"]
    #[inline(always)]
    pub const fn set_pipe4bemp(&mut self, val: super::vals::Pipe4bemp) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "BEMP Interrupt Status for PIPE5"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe5bemp(&self) -> super::vals::Pipe5bemp {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pipe5bemp::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Status for PIPE5"]
    #[inline(always)]
    pub const fn set_pipe5bemp(&mut self, val: super::vals::Pipe5bemp) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "BEMP Interrupt Status for PIPE6"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe6bemp(&self) -> super::vals::Pipe6bemp {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pipe6bemp::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Status for PIPE6"]
    #[inline(always)]
    pub const fn set_pipe6bemp(&mut self, val: super::vals::Pipe6bemp) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "BEMP Interrupt Status for PIPE7"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe7bemp(&self) -> super::vals::Pipe7bemp {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pipe7bemp::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Status for PIPE7"]
    #[inline(always)]
    pub const fn set_pipe7bemp(&mut self, val: super::vals::Pipe7bemp) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "BEMP Interrupt Status for PIPE8"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe8bemp(&self) -> super::vals::Pipe8bemp {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pipe8bemp::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Status for PIPE8"]
    #[inline(always)]
    pub const fn set_pipe8bemp(&mut self, val: super::vals::Pipe8bemp) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "BEMP Interrupt Status for PIPE9"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe9bemp(&self) -> super::vals::Pipe9bemp {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pipe9bemp::from_bits(val as u8)
    }
    #[doc = "BEMP Interrupt Status for PIPE9"]
    #[inline(always)]
    pub const fn set_pipe9bemp(&mut self, val: super::vals::Pipe9bemp) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
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
            .field("pipe0bemp", &self.pipe0bemp())
            .field("pipe1bemp", &self.pipe1bemp())
            .field("pipe2bemp", &self.pipe2bemp())
            .field("pipe3bemp", &self.pipe3bemp())
            .field("pipe4bemp", &self.pipe4bemp())
            .field("pipe5bemp", &self.pipe5bemp())
            .field("pipe6bemp", &self.pipe6bemp())
            .field("pipe7bemp", &self.pipe7bemp())
            .field("pipe8bemp", &self.pipe8bemp())
            .field("pipe9bemp", &self.pipe9bemp())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bempsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bempsts {{ pipe0bemp: {:?}, pipe1bemp: {:?}, pipe2bemp: {:?}, pipe3bemp: {:?}, pipe4bemp: {:?}, pipe5bemp: {:?}, pipe6bemp: {:?}, pipe7bemp: {:?}, pipe8bemp: {:?}, pipe9bemp: {:?}, reserved: {=u8:?} }}",
            self.pipe0bemp(),
            self.pipe1bemp(),
            self.pipe2bemp(),
            self.pipe3bemp(),
            self.pipe4bemp(),
            self.pipe5bemp(),
            self.pipe6bemp(),
            self.pipe7bemp(),
            self.pipe8bemp(),
            self.pipe9bemp(),
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
    pub const fn pipe0brdye(&self) -> super::vals::Pipe0brdye {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pipe0brdye::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Enable for PIPE0"]
    #[inline(always)]
    pub const fn set_pipe0brdye(&mut self, val: super::vals::Pipe0brdye) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE1"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe1brdye(&self) -> super::vals::Pipe1brdye {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pipe1brdye::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Enable for PIPE1"]
    #[inline(always)]
    pub const fn set_pipe1brdye(&mut self, val: super::vals::Pipe1brdye) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE2"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe2brdye(&self) -> super::vals::Pipe2brdye {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pipe2brdye::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Enable for PIPE2"]
    #[inline(always)]
    pub const fn set_pipe2brdye(&mut self, val: super::vals::Pipe2brdye) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE3"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe3brdye(&self) -> super::vals::Pipe3brdye {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pipe3brdye::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Enable for PIPE3"]
    #[inline(always)]
    pub const fn set_pipe3brdye(&mut self, val: super::vals::Pipe3brdye) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE4"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe4brdye(&self) -> super::vals::Pipe4brdye {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pipe4brdye::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Enable for PIPE4"]
    #[inline(always)]
    pub const fn set_pipe4brdye(&mut self, val: super::vals::Pipe4brdye) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE5"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe5brdye(&self) -> super::vals::Pipe5brdye {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pipe5brdye::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Enable for PIPE5"]
    #[inline(always)]
    pub const fn set_pipe5brdye(&mut self, val: super::vals::Pipe5brdye) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE6"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe6brdye(&self) -> super::vals::Pipe6brdye {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pipe6brdye::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Enable for PIPE6"]
    #[inline(always)]
    pub const fn set_pipe6brdye(&mut self, val: super::vals::Pipe6brdye) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE7"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe7brdye(&self) -> super::vals::Pipe7brdye {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pipe7brdye::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Enable for PIPE7"]
    #[inline(always)]
    pub const fn set_pipe7brdye(&mut self, val: super::vals::Pipe7brdye) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE8"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe8brdye(&self) -> super::vals::Pipe8brdye {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pipe8brdye::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Enable for PIPE8"]
    #[inline(always)]
    pub const fn set_pipe8brdye(&mut self, val: super::vals::Pipe8brdye) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "BRDY Interrupt Enable for PIPE9"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe9brdye(&self) -> super::vals::Pipe9brdye {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pipe9brdye::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Enable for PIPE9"]
    #[inline(always)]
    pub const fn set_pipe9brdye(&mut self, val: super::vals::Pipe9brdye) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
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
            "Brdyenb {{ pipe0brdye: {:?}, pipe1brdye: {:?}, pipe2brdye: {:?}, pipe3brdye: {:?}, pipe4brdye: {:?}, pipe5brdye: {:?}, pipe6brdye: {:?}, pipe7brdye: {:?}, pipe8brdye: {:?}, pipe9brdye: {:?}, reserved: {=u8:?} }}",
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
    pub const fn pipe0brdy(&self) -> super::vals::Pipe0brdy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pipe0brdy::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Status for PIPE0"]
    #[inline(always)]
    pub const fn set_pipe0brdy(&mut self, val: super::vals::Pipe0brdy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "BRDY Interrupt Status for PIPE1"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe1brdy(&self) -> super::vals::Pipe1brdy {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pipe1brdy::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Status for PIPE1"]
    #[inline(always)]
    pub const fn set_pipe1brdy(&mut self, val: super::vals::Pipe1brdy) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "BRDY Interrupt Status for PIPE2"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe2brdy(&self) -> super::vals::Pipe2brdy {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pipe2brdy::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Status for PIPE2"]
    #[inline(always)]
    pub const fn set_pipe2brdy(&mut self, val: super::vals::Pipe2brdy) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "BRDY Interrupt Status for PIPE3"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe3brdy(&self) -> super::vals::Pipe3brdy {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pipe3brdy::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Status for PIPE3"]
    #[inline(always)]
    pub const fn set_pipe3brdy(&mut self, val: super::vals::Pipe3brdy) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "BRDY Interrupt Status for PIPE4"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe4brdy(&self) -> super::vals::Pipe4brdy {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pipe4brdy::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Status for PIPE4"]
    #[inline(always)]
    pub const fn set_pipe4brdy(&mut self, val: super::vals::Pipe4brdy) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "BRDY Interrupt Status for PIPE5"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe5brdy(&self) -> super::vals::Pipe5brdy {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pipe5brdy::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Status for PIPE5"]
    #[inline(always)]
    pub const fn set_pipe5brdy(&mut self, val: super::vals::Pipe5brdy) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "BRDY Interrupt Status for PIPE6"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe6brdy(&self) -> super::vals::Pipe6brdy {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pipe6brdy::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Status for PIPE6"]
    #[inline(always)]
    pub const fn set_pipe6brdy(&mut self, val: super::vals::Pipe6brdy) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "BRDY Interrupt Status for PIPE7"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe7brdy(&self) -> super::vals::Pipe7brdy {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pipe7brdy::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Status for PIPE7"]
    #[inline(always)]
    pub const fn set_pipe7brdy(&mut self, val: super::vals::Pipe7brdy) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "BRDY Interrupt Status for PIPE8"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe8brdy(&self) -> super::vals::Pipe8brdy {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pipe8brdy::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Status for PIPE8"]
    #[inline(always)]
    pub const fn set_pipe8brdy(&mut self, val: super::vals::Pipe8brdy) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "BRDY Interrupt Status for PIPE9"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe9brdy(&self) -> super::vals::Pipe9brdy {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pipe9brdy::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Status for PIPE9"]
    #[inline(always)]
    pub const fn set_pipe9brdy(&mut self, val: super::vals::Pipe9brdy) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
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
            .field("pipe0brdy", &self.pipe0brdy())
            .field("pipe1brdy", &self.pipe1brdy())
            .field("pipe2brdy", &self.pipe2brdy())
            .field("pipe3brdy", &self.pipe3brdy())
            .field("pipe4brdy", &self.pipe4brdy())
            .field("pipe5brdy", &self.pipe5brdy())
            .field("pipe6brdy", &self.pipe6brdy())
            .field("pipe7brdy", &self.pipe7brdy())
            .field("pipe8brdy", &self.pipe8brdy())
            .field("pipe9brdy", &self.pipe9brdy())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Brdysts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Brdysts {{ pipe0brdy: {:?}, pipe1brdy: {:?}, pipe2brdy: {:?}, pipe3brdy: {:?}, pipe4brdy: {:?}, pipe5brdy: {:?}, pipe6brdy: {:?}, pipe7brdy: {:?}, pipe8brdy: {:?}, pipe9brdy: {:?}, reserved: {=u8:?} }}",
            self.pipe0brdy(),
            self.pipe1brdy(),
            self.pipe2brdy(),
            self.pipe3brdy(),
            self.pipe4brdy(),
            self.pipe5brdy(),
            self.pipe6brdy(),
            self.pipe7brdy(),
            self.pipe8brdy(),
            self.pipe9brdy(),
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
    pub const fn frdy(&self) -> super::vals::CfifoctrFrdy {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::CfifoctrFrdy::from_bits(val as u8)
    }
    #[doc = "FIFO Port Ready"]
    #[inline(always)]
    pub const fn set_frdy(&mut self, val: super::vals::CfifoctrFrdy) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "CPU Buffer Clear Note: Only 0 can be read."]
    #[must_use]
    #[inline(always)]
    pub const fn bclr(&self) -> super::vals::CfifoctrBclr {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::CfifoctrBclr::from_bits(val as u8)
    }
    #[doc = "CPU Buffer Clear Note: Only 0 can be read."]
    #[inline(always)]
    pub const fn set_bclr(&mut self, val: super::vals::CfifoctrBclr) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Buffer Memory Valid Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bval(&self) -> super::vals::CfifoctrBval {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::CfifoctrBval::from_bits(val as u8)
    }
    #[doc = "Buffer Memory Valid Flag"]
    #[inline(always)]
    pub const fn set_bval(&mut self, val: super::vals::CfifoctrBval) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "Cfifoctr {{ dtln: {=u16:?}, reserved: {=u8:?}, frdy: {:?}, bclr: {:?}, bval: {:?} }}",
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
    pub const fn isel(&self) -> super::vals::Isel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Isel::from_bits(val as u8)
    }
    #[doc = "CFIFO Port Access Direction When DCP is Selected"]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: super::vals::Isel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
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
    pub const fn bigend(&self) -> super::vals::CfifoselBigend {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::CfifoselBigend::from_bits(val as u8)
    }
    #[doc = "CFIFO Port Endian Control"]
    #[inline(always)]
    pub const fn set_bigend(&mut self, val: super::vals::CfifoselBigend) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
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
    pub const fn mbw(&self) -> super::vals::CfifoselMbw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::CfifoselMbw::from_bits(val as u8)
    }
    #[doc = "CFIFO Port Access Bit Width"]
    #[inline(always)]
    pub const fn set_mbw(&mut self, val: super::vals::CfifoselMbw) {
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
    #[doc = "Buffer Pointer Rewind"]
    #[must_use]
    #[inline(always)]
    pub const fn rew(&self) -> super::vals::CfifoselRew {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::CfifoselRew::from_bits(val as u8)
    }
    #[doc = "Buffer Pointer Rewind"]
    #[inline(always)]
    pub const fn set_rew(&mut self, val: super::vals::CfifoselRew) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Read Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rcnt(&self) -> super::vals::CfifoselRcnt {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::CfifoselRcnt::from_bits(val as u8)
    }
    #[doc = "Read Count Mode"]
    #[inline(always)]
    pub const fn set_rcnt(&mut self, val: super::vals::CfifoselRcnt) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "Cfifosel {{ curpipe: {:?}, reserved: {=bool:?}, isel: {:?}, reserved_2: {=u8:?}, bigend: {:?}, reserved_3: {=bool:?}, mbw: {:?}, reserved_4: {=u8:?}, rew: {:?}, rcnt: {:?} }}",
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
    pub const fn frdy(&self) -> super::vals::D0fifoctrFrdy {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::D0fifoctrFrdy::from_bits(val as u8)
    }
    #[doc = "FIFO Port Ready"]
    #[inline(always)]
    pub const fn set_frdy(&mut self, val: super::vals::D0fifoctrFrdy) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "CPU Buffer Clear Note: Only 0 can be read."]
    #[must_use]
    #[inline(always)]
    pub const fn bclr(&self) -> super::vals::D0fifoctrBclr {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::D0fifoctrBclr::from_bits(val as u8)
    }
    #[doc = "CPU Buffer Clear Note: Only 0 can be read."]
    #[inline(always)]
    pub const fn set_bclr(&mut self, val: super::vals::D0fifoctrBclr) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Buffer Memory Valid Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bval(&self) -> super::vals::D0fifoctrBval {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::D0fifoctrBval::from_bits(val as u8)
    }
    #[doc = "Buffer Memory Valid Flag"]
    #[inline(always)]
    pub const fn set_bval(&mut self, val: super::vals::D0fifoctrBval) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "D0fifoctr {{ dtln: {=u16:?}, reserved: {=u8:?}, frdy: {:?}, bclr: {:?}, bval: {:?} }}",
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
    pub const fn bigend(&self) -> super::vals::D0fifoselBigend {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::D0fifoselBigend::from_bits(val as u8)
    }
    #[doc = "FIFO Port Endian Control"]
    #[inline(always)]
    pub const fn set_bigend(&mut self, val: super::vals::D0fifoselBigend) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
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
    pub const fn mbw(&self) -> super::vals::D0fifoselMbw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::D0fifoselMbw::from_bits(val as u8)
    }
    #[doc = "FIFO Port Access Bit Width"]
    #[inline(always)]
    pub const fn set_mbw(&mut self, val: super::vals::D0fifoselMbw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
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
    pub const fn dreqe(&self) -> super::vals::D0fifoselDreqe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::D0fifoselDreqe::from_bits(val as u8)
    }
    #[doc = "DMA/DTC Transfer Request Enable"]
    #[inline(always)]
    pub const fn set_dreqe(&mut self, val: super::vals::D0fifoselDreqe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read"]
    #[must_use]
    #[inline(always)]
    pub const fn dclrm(&self) -> super::vals::D0fifoselDclrm {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::D0fifoselDclrm::from_bits(val as u8)
    }
    #[doc = "Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read"]
    #[inline(always)]
    pub const fn set_dclrm(&mut self, val: super::vals::D0fifoselDclrm) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Buffer Pointer Rewind Note: Only 0 can be read."]
    #[must_use]
    #[inline(always)]
    pub const fn rew(&self) -> super::vals::D0fifoselRew {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::D0fifoselRew::from_bits(val as u8)
    }
    #[doc = "Buffer Pointer Rewind Note: Only 0 can be read."]
    #[inline(always)]
    pub const fn set_rew(&mut self, val: super::vals::D0fifoselRew) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Read Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rcnt(&self) -> super::vals::D0fifoselRcnt {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::D0fifoselRcnt::from_bits(val as u8)
    }
    #[doc = "Read Count Mode"]
    #[inline(always)]
    pub const fn set_rcnt(&mut self, val: super::vals::D0fifoselRcnt) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "D0fifosel {{ curpipe: {:?}, reserved: {=u8:?}, bigend: {:?}, reserved_2: {=bool:?}, mbw: {:?}, reserved_3: {=bool:?}, dreqe: {:?}, dclrm: {:?}, rew: {:?}, rcnt: {:?} }}",
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
    pub const fn frdy(&self) -> super::vals::D1fifoctrFrdy {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::D1fifoctrFrdy::from_bits(val as u8)
    }
    #[doc = "FIFO Port Ready"]
    #[inline(always)]
    pub const fn set_frdy(&mut self, val: super::vals::D1fifoctrFrdy) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "CPU Buffer Clear Note: Only 0 can be read."]
    #[must_use]
    #[inline(always)]
    pub const fn bclr(&self) -> super::vals::D1fifoctrBclr {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::D1fifoctrBclr::from_bits(val as u8)
    }
    #[doc = "CPU Buffer Clear Note: Only 0 can be read."]
    #[inline(always)]
    pub const fn set_bclr(&mut self, val: super::vals::D1fifoctrBclr) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Buffer Memory Valid Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bval(&self) -> super::vals::D1fifoctrBval {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::D1fifoctrBval::from_bits(val as u8)
    }
    #[doc = "Buffer Memory Valid Flag"]
    #[inline(always)]
    pub const fn set_bval(&mut self, val: super::vals::D1fifoctrBval) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "D1fifoctr {{ dtln: {=u16:?}, reserved: {=u8:?}, frdy: {:?}, bclr: {:?}, bval: {:?} }}",
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
    pub const fn bigend(&self) -> super::vals::D1fifoselBigend {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::D1fifoselBigend::from_bits(val as u8)
    }
    #[doc = "FIFO Port Endian Control"]
    #[inline(always)]
    pub const fn set_bigend(&mut self, val: super::vals::D1fifoselBigend) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
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
    pub const fn mbw(&self) -> super::vals::D1fifoselMbw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::D1fifoselMbw::from_bits(val as u8)
    }
    #[doc = "FIFO Port Access Bit Width"]
    #[inline(always)]
    pub const fn set_mbw(&mut self, val: super::vals::D1fifoselMbw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
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
    pub const fn dreqe(&self) -> super::vals::D1fifoselDreqe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::D1fifoselDreqe::from_bits(val as u8)
    }
    #[doc = "DMA/DTC Transfer Request Enable"]
    #[inline(always)]
    pub const fn set_dreqe(&mut self, val: super::vals::D1fifoselDreqe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read"]
    #[must_use]
    #[inline(always)]
    pub const fn dclrm(&self) -> super::vals::D1fifoselDclrm {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::D1fifoselDclrm::from_bits(val as u8)
    }
    #[doc = "Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read"]
    #[inline(always)]
    pub const fn set_dclrm(&mut self, val: super::vals::D1fifoselDclrm) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Buffer Pointer Rewind"]
    #[must_use]
    #[inline(always)]
    pub const fn rew(&self) -> super::vals::D1fifoselRew {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::D1fifoselRew::from_bits(val as u8)
    }
    #[doc = "Buffer Pointer Rewind"]
    #[inline(always)]
    pub const fn set_rew(&mut self, val: super::vals::D1fifoselRew) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Read Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rcnt(&self) -> super::vals::D1fifoselRcnt {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::D1fifoselRcnt::from_bits(val as u8)
    }
    #[doc = "Read Count Mode"]
    #[inline(always)]
    pub const fn set_rcnt(&mut self, val: super::vals::D1fifoselRcnt) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "D1fifosel {{ curpipe: {:?}, reserved: {=u8:?}, bigend: {:?}, reserved_2: {=bool:?}, mbw: {:?}, reserved_3: {=bool:?}, dreqe: {:?}, dclrm: {:?}, rew: {:?}, rcnt: {:?} }}",
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
    pub const fn dir(&self) -> super::vals::DcpcfgDir {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DcpcfgDir::from_bits(val as u8)
    }
    #[doc = "Transfer Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::DcpcfgDir) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
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
    pub const fn shtnak(&self) -> super::vals::DcpcfgShtnak {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::DcpcfgShtnak::from_bits(val as u8)
    }
    #[doc = "Pipe Disabled at End of Transfer"]
    #[inline(always)]
    pub const fn set_shtnak(&mut self, val: super::vals::DcpcfgShtnak) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
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
            "Dcpcfg {{ reserved: {=u8:?}, dir: {:?}, reserved_2: {=u8:?}, shtnak: {:?}, reserved_3: {=u8:?} }}",
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
    pub const fn ccpl(&self) -> super::vals::Ccpl {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ccpl::from_bits(val as u8)
    }
    #[doc = "Control Transfer End Enable"]
    #[inline(always)]
    pub const fn set_ccpl(&mut self, val: super::vals::Ccpl) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
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
    pub const fn pbusy(&self) -> super::vals::DcpctrPbusy {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::DcpctrPbusy::from_bits(val as u8)
    }
    #[doc = "Pipe Busy"]
    #[inline(always)]
    pub const fn set_pbusy(&mut self, val: super::vals::DcpctrPbusy) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Sequence Toggle Bit Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn sqmon(&self) -> super::vals::DcpctrSqmon {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DcpctrSqmon::from_bits(val as u8)
    }
    #[doc = "Sequence Toggle Bit Monitor"]
    #[inline(always)]
    pub const fn set_sqmon(&mut self, val: super::vals::DcpctrSqmon) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Sequence Toggle Bit Set"]
    #[must_use]
    #[inline(always)]
    pub const fn sqset(&self) -> super::vals::DcpctrSqset {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::DcpctrSqset::from_bits(val as u8)
    }
    #[doc = "Sequence Toggle Bit Set"]
    #[inline(always)]
    pub const fn set_sqset(&mut self, val: super::vals::DcpctrSqset) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Sequence Toggle Bit Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sqclr(&self) -> super::vals::DcpctrSqclr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::DcpctrSqclr::from_bits(val as u8)
    }
    #[doc = "Sequence Toggle Bit Clear"]
    #[inline(always)]
    pub const fn set_sqclr(&mut self, val: super::vals::DcpctrSqclr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
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
    pub const fn sureqclr(&self) -> super::vals::Sureqclr {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Sureqclr::from_bits(val as u8)
    }
    #[doc = "SUREQ Bit Clear"]
    #[inline(always)]
    pub const fn set_sureqclr(&mut self, val: super::vals::Sureqclr) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
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
    pub const fn sureq(&self) -> super::vals::Sureq {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sureq::from_bits(val as u8)
    }
    #[doc = "Setup Token Transmission"]
    #[inline(always)]
    pub const fn set_sureq(&mut self, val: super::vals::Sureq) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Buffer Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bsts(&self) -> super::vals::DcpctrBsts {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::DcpctrBsts::from_bits(val as u8)
    }
    #[doc = "Buffer Status"]
    #[inline(always)]
    pub const fn set_bsts(&mut self, val: super::vals::DcpctrBsts) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "Dcpctr {{ pid: {:?}, ccpl: {:?}, reserved: {=u8:?}, pbusy: {:?}, sqmon: {:?}, sqset: {:?}, sqclr: {:?}, reserved_2: {=u8:?}, sureqclr: {:?}, reserved_3: {=u8:?}, sureq: {:?}, bsts: {:?} }}",
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
    pub const fn uact(&self) -> super::vals::Uact {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Uact::from_bits(val as u8)
    }
    #[doc = "USB Bus Enable"]
    #[inline(always)]
    pub const fn set_uact(&mut self, val: super::vals::Uact) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Resume Output"]
    #[must_use]
    #[inline(always)]
    pub const fn resume(&self) -> super::vals::Resume {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Resume::from_bits(val as u8)
    }
    #[doc = "Resume Output"]
    #[inline(always)]
    pub const fn set_resume(&mut self, val: super::vals::Resume) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "USB Bus Reset Output"]
    #[must_use]
    #[inline(always)]
    pub const fn usbrst(&self) -> super::vals::Usbrst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Usbrst::from_bits(val as u8)
    }
    #[doc = "USB Bus Reset Output"]
    #[inline(always)]
    pub const fn set_usbrst(&mut self, val: super::vals::Usbrst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Wakeup Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rwupe(&self) -> super::vals::Rwupe {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Rwupe::from_bits(val as u8)
    }
    #[doc = "Wakeup Detection Enable"]
    #[inline(always)]
    pub const fn set_rwupe(&mut self, val: super::vals::Rwupe) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Wakeup Output"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup(&self) -> super::vals::Wkup {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Wkup::from_bits(val as u8)
    }
    #[doc = "Wakeup Output"]
    #[inline(always)]
    pub const fn set_wkup(&mut self, val: super::vals::Wkup) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "USB_VBUSEN Output Pin Control"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusen(&self) -> super::vals::Vbusen {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Vbusen::from_bits(val as u8)
    }
    #[doc = "USB_VBUSEN Output Pin Control"]
    #[inline(always)]
    pub const fn set_vbusen(&mut self, val: super::vals::Vbusen) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "USB_EXICEN Output Pin Control"]
    #[must_use]
    #[inline(always)]
    pub const fn exicen(&self) -> super::vals::Exicen {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Exicen::from_bits(val as u8)
    }
    #[doc = "USB_EXICEN Output Pin Control"]
    #[inline(always)]
    pub const fn set_exicen(&mut self, val: super::vals::Exicen) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Host Negotiation Protocol (HNP) Control This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set."]
    #[must_use]
    #[inline(always)]
    pub const fn hnpbtoa(&self) -> super::vals::Hnpbtoa {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Hnpbtoa::from_bits(val as u8)
    }
    #[doc = "Host Negotiation Protocol (HNP) Control This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set."]
    #[inline(always)]
    pub const fn set_hnpbtoa(&mut self, val: super::vals::Hnpbtoa) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
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
            "Dvstctr0 {{ rhst: {:?}, reserved: {=bool:?}, uact: {:?}, resume: {:?}, usbrst: {:?}, rwupe: {:?}, wkup: {:?}, vbusen: {:?}, exicen: {:?}, hnpbtoa: {:?}, reserved_2: {=u8:?} }}",
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
    pub const fn crce(&self) -> super::vals::Crce {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Crce::from_bits(val as u8)
    }
    #[doc = "Receive Data Error"]
    #[inline(always)]
    pub const fn set_crce(&mut self, val: super::vals::Crce) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Overrun/Underrun Detection Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrn(&self) -> super::vals::Ovrn {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Ovrn::from_bits(val as u8)
    }
    #[doc = "Overrun/Underrun Detection Status"]
    #[inline(always)]
    pub const fn set_ovrn(&mut self, val: super::vals::Ovrn) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "Frmnum {{ frnm: {=u16:?}, reserved: {=u8:?}, crce: {:?}, ovrn: {:?} }}",
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
    pub const fn brdye(&self) -> super::vals::Brdye {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Brdye::from_bits(val as u8)
    }
    #[doc = "Buffer Ready Interrupt Enable"]
    #[inline(always)]
    pub const fn set_brdye(&mut self, val: super::vals::Brdye) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Buffer Not Ready Response Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nrdye(&self) -> super::vals::Nrdye {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Nrdye::from_bits(val as u8)
    }
    #[doc = "Buffer Not Ready Response Interrupt Enable"]
    #[inline(always)]
    pub const fn set_nrdye(&mut self, val: super::vals::Nrdye) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Buffer Empty Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bempe(&self) -> super::vals::Bempe {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Bempe::from_bits(val as u8)
    }
    #[doc = "Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub const fn set_bempe(&mut self, val: super::vals::Bempe) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Control Transfer Stage Transition Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ctre(&self) -> super::vals::Ctre {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Ctre::from_bits(val as u8)
    }
    #[doc = "Control Transfer Stage Transition Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ctre(&mut self, val: super::vals::Ctre) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Device State Transition Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dvse(&self) -> super::vals::Dvse {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dvse::from_bits(val as u8)
    }
    #[doc = "Device State Transition Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dvse(&mut self, val: super::vals::Dvse) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Frame Number Update Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sofe(&self) -> super::vals::Sofe {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sofe::from_bits(val as u8)
    }
    #[doc = "Frame Number Update Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sofe(&mut self, val: super::vals::Sofe) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Resume Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rsme(&self) -> super::vals::Rsme {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Rsme::from_bits(val as u8)
    }
    #[doc = "Resume Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rsme(&mut self, val: super::vals::Rsme) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "VBUS Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbse(&self) -> super::vals::Vbse {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Vbse::from_bits(val as u8)
    }
    #[doc = "VBUS Interrupt Enable"]
    #[inline(always)]
    pub const fn set_vbse(&mut self, val: super::vals::Vbse) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "Intenb0 {{ reserved: {=u8:?}, brdye: {:?}, nrdye: {:?}, bempe: {:?}, ctre: {:?}, dvse: {:?}, sofe: {:?}, rsme: {:?}, vbse: {:?} }}",
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
    pub const fn pddetinte0(&self) -> super::vals::Pddetinte0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pddetinte0::from_bits(val as u8)
    }
    #[doc = "PDDETINT0 Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_pddetinte0(&mut self, val: super::vals::Pddetinte0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
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
    pub const fn sacke(&self) -> super::vals::Sacke {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sacke::from_bits(val as u8)
    }
    #[doc = "Setup Transaction Normal Response Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sacke(&mut self, val: super::vals::Sacke) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Setup Transaction Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn signe(&self) -> super::vals::Signe {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Signe::from_bits(val as u8)
    }
    #[doc = "Setup Transaction Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_signe(&mut self, val: super::vals::Signe) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "EOF Error Detection Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eoferre(&self) -> super::vals::Eoferre {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Eoferre::from_bits(val as u8)
    }
    #[doc = "EOF Error Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_eoferre(&mut self, val: super::vals::Eoferre) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
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
    pub const fn attche(&self) -> super::vals::Attche {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Attche::from_bits(val as u8)
    }
    #[doc = "Connection Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_attche(&mut self, val: super::vals::Attche) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Disconnection Detection Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtche(&self) -> super::vals::Dtche {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dtche::from_bits(val as u8)
    }
    #[doc = "Disconnection Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dtche(&mut self, val: super::vals::Dtche) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
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
    pub const fn bchge(&self) -> super::vals::Bchge {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Bchge::from_bits(val as u8)
    }
    #[doc = "USB Bus Change Interrupt Enable"]
    #[inline(always)]
    pub const fn set_bchge(&mut self, val: super::vals::Bchge) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Overcurrent Input Change Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrcre(&self) -> super::vals::Ovrcre {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Ovrcre::from_bits(val as u8)
    }
    #[doc = "Overcurrent Input Change Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ovrcre(&mut self, val: super::vals::Ovrcre) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "Intenb1 {{ pddetinte0: {:?}, reserved: {=u8:?}, sacke: {:?}, signe: {:?}, eoferre: {:?}, reserved_2: {=u8:?}, attche: {:?}, dtche: {:?}, reserved_3: {=bool:?}, bchge: {:?}, ovrcre: {:?} }}",
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
    pub const fn valid(&self) -> super::vals::Valid {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Valid::from_bits(val as u8)
    }
    #[doc = "USB Request Reception"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: super::vals::Valid) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
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
    pub const fn vbsts(&self) -> super::vals::Vbsts {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Vbsts::from_bits(val as u8)
    }
    #[doc = "VBUS Input Status"]
    #[inline(always)]
    pub const fn set_vbsts(&mut self, val: super::vals::Vbsts) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Buffer Ready Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn brdy(&self) -> super::vals::Brdy {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Brdy::from_bits(val as u8)
    }
    #[doc = "Buffer Ready Interrupt Status"]
    #[inline(always)]
    pub const fn set_brdy(&mut self, val: super::vals::Brdy) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Buffer Not Ready Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn nrdy(&self) -> super::vals::Nrdy {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Nrdy::from_bits(val as u8)
    }
    #[doc = "Buffer Not Ready Interrupt Status"]
    #[inline(always)]
    pub const fn set_nrdy(&mut self, val: super::vals::Nrdy) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Buffer Empty Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bemp(&self) -> super::vals::Bemp {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Bemp::from_bits(val as u8)
    }
    #[doc = "Buffer Empty Interrupt Status"]
    #[inline(always)]
    pub const fn set_bemp(&mut self, val: super::vals::Bemp) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Control Transfer Stage Transition Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrt(&self) -> super::vals::Ctrt {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Ctrt::from_bits(val as u8)
    }
    #[doc = "Control Transfer Stage Transition Interrupt Status"]
    #[inline(always)]
    pub const fn set_ctrt(&mut self, val: super::vals::Ctrt) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Device State Transition Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn dvst(&self) -> super::vals::Dvst {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dvst::from_bits(val as u8)
    }
    #[doc = "Device State Transition Interrupt Status"]
    #[inline(always)]
    pub const fn set_dvst(&mut self, val: super::vals::Dvst) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Frame Number Refresh Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sofr(&self) -> super::vals::Sofr {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sofr::from_bits(val as u8)
    }
    #[doc = "Frame Number Refresh Interrupt Status"]
    #[inline(always)]
    pub const fn set_sofr(&mut self, val: super::vals::Sofr) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Resume Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn resm(&self) -> super::vals::Resm {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Resm::from_bits(val as u8)
    }
    #[doc = "Resume Interrupt Status"]
    #[inline(always)]
    pub const fn set_resm(&mut self, val: super::vals::Resm) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "VBUS Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn vbint(&self) -> super::vals::Vbint {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Vbint::from_bits(val as u8)
    }
    #[doc = "VBUS Interrupt Status"]
    #[inline(always)]
    pub const fn set_vbint(&mut self, val: super::vals::Vbint) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "Intsts0 {{ ctsq: {:?}, valid: {:?}, dvsq: {:?}, vbsts: {:?}, brdy: {:?}, nrdy: {:?}, bemp: {:?}, ctrt: {:?}, dvst: {:?}, sofr: {:?}, resm: {:?}, vbint: {:?} }}",
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
    pub const fn pddetint0(&self) -> super::vals::Pddetint0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pddetint0::from_bits(val as u8)
    }
    #[doc = "PDDET0 Detection Interrupt Status"]
    #[inline(always)]
    pub const fn set_pddetint0(&mut self, val: super::vals::Pddetint0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
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
    pub const fn sack(&self) -> super::vals::Sack {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sack::from_bits(val as u8)
    }
    #[doc = "Setup Transaction Normal Response Interrupt Status"]
    #[inline(always)]
    pub const fn set_sack(&mut self, val: super::vals::Sack) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Setup Transaction Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sign(&self) -> super::vals::Sign {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Sign::from_bits(val as u8)
    }
    #[doc = "Setup Transaction Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_sign(&mut self, val: super::vals::Sign) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "EOF Error Detection Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn eoferr(&self) -> super::vals::Eoferr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Eoferr::from_bits(val as u8)
    }
    #[doc = "EOF Error Detection Interrupt Status"]
    #[inline(always)]
    pub const fn set_eoferr(&mut self, val: super::vals::Eoferr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
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
    pub const fn attch(&self) -> super::vals::Attch {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Attch::from_bits(val as u8)
    }
    #[doc = "ATTCH Interrupt Status"]
    #[inline(always)]
    pub const fn set_attch(&mut self, val: super::vals::Attch) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "USB Disconnection Detection Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn dtch(&self) -> super::vals::Dtch {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dtch::from_bits(val as u8)
    }
    #[doc = "USB Disconnection Detection Interrupt Status"]
    #[inline(always)]
    pub const fn set_dtch(&mut self, val: super::vals::Dtch) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
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
    pub const fn bchg(&self) -> super::vals::Bchg {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Bchg::from_bits(val as u8)
    }
    #[doc = "USB Bus Change Interrupt Status"]
    #[inline(always)]
    pub const fn set_bchg(&mut self, val: super::vals::Bchg) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Overcurrent Input Change Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrcr(&self) -> super::vals::Ovrcr {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Ovrcr::from_bits(val as u8)
    }
    #[doc = "Overcurrent Input Change Interrupt Status"]
    #[inline(always)]
    pub const fn set_ovrcr(&mut self, val: super::vals::Ovrcr) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "Intsts1 {{ pddetint0: {:?}, reserved: {=u8:?}, sack: {:?}, sign: {:?}, eoferr: {:?}, reserved_2: {=u8:?}, attch: {:?}, dtch: {:?}, reserved_3: {=bool:?}, bchg: {:?}, ovrcr: {:?} }}",
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
    pub const fn pipe0nrdye(&self) -> super::vals::Pipe0nrdye {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pipe0nrdye::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Enable for PIPE0"]
    #[inline(always)]
    pub const fn set_pipe0nrdye(&mut self, val: super::vals::Pipe0nrdye) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE1"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe1nrdye(&self) -> super::vals::Pipe1nrdye {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pipe1nrdye::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Enable for PIPE1"]
    #[inline(always)]
    pub const fn set_pipe1nrdye(&mut self, val: super::vals::Pipe1nrdye) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE2"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe2nrdye(&self) -> super::vals::Pipe2nrdye {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pipe2nrdye::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Enable for PIPE2"]
    #[inline(always)]
    pub const fn set_pipe2nrdye(&mut self, val: super::vals::Pipe2nrdye) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE3"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe3nrdye(&self) -> super::vals::Pipe3nrdye {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pipe3nrdye::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Enable for PIPE3"]
    #[inline(always)]
    pub const fn set_pipe3nrdye(&mut self, val: super::vals::Pipe3nrdye) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE4"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe4nrdye(&self) -> super::vals::Pipe4nrdye {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pipe4nrdye::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Enable for PIPE4"]
    #[inline(always)]
    pub const fn set_pipe4nrdye(&mut self, val: super::vals::Pipe4nrdye) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE5"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe5nrdye(&self) -> super::vals::Pipe5nrdye {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pipe5nrdye::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Enable for PIPE5"]
    #[inline(always)]
    pub const fn set_pipe5nrdye(&mut self, val: super::vals::Pipe5nrdye) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE6"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe6nrdye(&self) -> super::vals::Pipe6nrdye {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pipe6nrdye::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Enable for PIPE6"]
    #[inline(always)]
    pub const fn set_pipe6nrdye(&mut self, val: super::vals::Pipe6nrdye) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE7"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe7nrdye(&self) -> super::vals::Pipe7nrdye {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pipe7nrdye::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Enable for PIPE7"]
    #[inline(always)]
    pub const fn set_pipe7nrdye(&mut self, val: super::vals::Pipe7nrdye) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE8"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe8nrdye(&self) -> super::vals::Pipe8nrdye {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pipe8nrdye::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Enable for PIPE8"]
    #[inline(always)]
    pub const fn set_pipe8nrdye(&mut self, val: super::vals::Pipe8nrdye) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "NRDY Interrupt Enable for PIPE9"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe9nrdye(&self) -> super::vals::Pipe9nrdye {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pipe9nrdye::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Enable for PIPE9"]
    #[inline(always)]
    pub const fn set_pipe9nrdye(&mut self, val: super::vals::Pipe9nrdye) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
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
            "Nrdyenb {{ pipe0nrdye: {:?}, pipe1nrdye: {:?}, pipe2nrdye: {:?}, pipe3nrdye: {:?}, pipe4nrdye: {:?}, pipe5nrdye: {:?}, pipe6nrdye: {:?}, pipe7nrdye: {:?}, pipe8nrdye: {:?}, pipe9nrdye: {:?}, reserved: {=u8:?} }}",
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
    pub const fn pipe0nrdy(&self) -> super::vals::Pipe0nrdy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pipe0nrdy::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Status for PIPE0"]
    #[inline(always)]
    pub const fn set_pipe0nrdy(&mut self, val: super::vals::Pipe0nrdy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "NRDY Interrupt Status for PIPE1"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe1nrdy(&self) -> super::vals::Pipe1nrdy {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pipe1nrdy::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Status for PIPE1"]
    #[inline(always)]
    pub const fn set_pipe1nrdy(&mut self, val: super::vals::Pipe1nrdy) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "NRDY Interrupt Status for PIPE2"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe2nrdy(&self) -> super::vals::Pipe2nrdy {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pipe2nrdy::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Status for PIPE2"]
    #[inline(always)]
    pub const fn set_pipe2nrdy(&mut self, val: super::vals::Pipe2nrdy) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "NRDY Interrupt Status for PIPE3"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe3nrdy(&self) -> super::vals::Pipe3nrdy {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pipe3nrdy::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Status for PIPE3"]
    #[inline(always)]
    pub const fn set_pipe3nrdy(&mut self, val: super::vals::Pipe3nrdy) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "NRDY Interrupt Status for PIPE4"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe4nrdy(&self) -> super::vals::Pipe4nrdy {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pipe4nrdy::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Status for PIPE4"]
    #[inline(always)]
    pub const fn set_pipe4nrdy(&mut self, val: super::vals::Pipe4nrdy) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "NRDY Interrupt Status for PIPE5"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe5nrdy(&self) -> super::vals::Pipe5nrdy {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pipe5nrdy::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Status for PIPE5"]
    #[inline(always)]
    pub const fn set_pipe5nrdy(&mut self, val: super::vals::Pipe5nrdy) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "NRDY Interrupt Status for PIPE6"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe6nrdy(&self) -> super::vals::Pipe6nrdy {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pipe6nrdy::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Status for PIPE6"]
    #[inline(always)]
    pub const fn set_pipe6nrdy(&mut self, val: super::vals::Pipe6nrdy) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "NRDY Interrupt Status for PIPE7"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe7nrdy(&self) -> super::vals::Pipe7nrdy {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pipe7nrdy::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Status for PIPE7"]
    #[inline(always)]
    pub const fn set_pipe7nrdy(&mut self, val: super::vals::Pipe7nrdy) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "NRDY Interrupt Status for PIPE8"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe8nrdy(&self) -> super::vals::Pipe8nrdy {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pipe8nrdy::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Status for PIPE8"]
    #[inline(always)]
    pub const fn set_pipe8nrdy(&mut self, val: super::vals::Pipe8nrdy) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "NRDY Interrupt Status for PIPE9"]
    #[must_use]
    #[inline(always)]
    pub const fn pipe9nrdy(&self) -> super::vals::Pipe9nrdy {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pipe9nrdy::from_bits(val as u8)
    }
    #[doc = "NRDY Interrupt Status for PIPE9"]
    #[inline(always)]
    pub const fn set_pipe9nrdy(&mut self, val: super::vals::Pipe9nrdy) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
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
            .field("pipe0nrdy", &self.pipe0nrdy())
            .field("pipe1nrdy", &self.pipe1nrdy())
            .field("pipe2nrdy", &self.pipe2nrdy())
            .field("pipe3nrdy", &self.pipe3nrdy())
            .field("pipe4nrdy", &self.pipe4nrdy())
            .field("pipe5nrdy", &self.pipe5nrdy())
            .field("pipe6nrdy", &self.pipe6nrdy())
            .field("pipe7nrdy", &self.pipe7nrdy())
            .field("pipe8nrdy", &self.pipe8nrdy())
            .field("pipe9nrdy", &self.pipe9nrdy())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nrdysts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nrdysts {{ pipe0nrdy: {:?}, pipe1nrdy: {:?}, pipe2nrdy: {:?}, pipe3nrdy: {:?}, pipe4nrdy: {:?}, pipe5nrdy: {:?}, pipe6nrdy: {:?}, pipe7nrdy: {:?}, pipe8nrdy: {:?}, pipe9nrdy: {:?}, reserved: {=u8:?} }}",
            self.pipe0nrdy(),
            self.pipe1nrdy(),
            self.pipe2nrdy(),
            self.pipe3nrdy(),
            self.pipe4nrdy(),
            self.pipe5nrdy(),
            self.pipe6nrdy(),
            self.pipe7nrdy(),
            self.pipe8nrdy(),
            self.pipe9nrdy(),
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
    pub const fn dir(&self) -> super::vals::PipecfgDir {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PipecfgDir::from_bits(val as u8)
    }
    #[doc = "Transfer Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::PipecfgDir) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
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
    pub const fn shtnak(&self) -> super::vals::PipecfgShtnak {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::PipecfgShtnak::from_bits(val as u8)
    }
    #[doc = "Pipe Disabled at End of Transfer"]
    #[inline(always)]
    pub const fn set_shtnak(&mut self, val: super::vals::PipecfgShtnak) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
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
    pub const fn dblb(&self) -> super::vals::Dblb {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dblb::from_bits(val as u8)
    }
    #[doc = "Double Buffer Mode"]
    #[inline(always)]
    pub const fn set_dblb(&mut self, val: super::vals::Dblb) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "BRDY Interrupt Operation Specification"]
    #[must_use]
    #[inline(always)]
    pub const fn bfre(&self) -> super::vals::Bfre {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Bfre::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Operation Specification"]
    #[inline(always)]
    pub const fn set_bfre(&mut self, val: super::vals::Bfre) {
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
            "Pipecfg {{ epnum: {=u8:?}, dir: {:?}, reserved: {=bool:?}, reserved_2: {=bool:?}, shtnak: {:?}, reserved_3: {=bool:?}, dblb: {:?}, bfre: {:?}, reserved_4: {=u8:?}, type_: {:?} }}",
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
    pub const fn pbusy(&self) -> super::vals::PipectrPbusy {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::PipectrPbusy::from_bits(val as u8)
    }
    #[doc = "Pipe Busy"]
    #[inline(always)]
    pub const fn set_pbusy(&mut self, val: super::vals::PipectrPbusy) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Sequence Toggle Bit Confirmation"]
    #[must_use]
    #[inline(always)]
    pub const fn sqmon(&self) -> super::vals::PipectrSqmon {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PipectrSqmon::from_bits(val as u8)
    }
    #[doc = "Sequence Toggle Bit Confirmation"]
    #[inline(always)]
    pub const fn set_sqmon(&mut self, val: super::vals::PipectrSqmon) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Sequence Toggle Bit Set"]
    #[must_use]
    #[inline(always)]
    pub const fn sqset(&self) -> super::vals::PipectrSqset {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::PipectrSqset::from_bits(val as u8)
    }
    #[doc = "Sequence Toggle Bit Set"]
    #[inline(always)]
    pub const fn set_sqset(&mut self, val: super::vals::PipectrSqset) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Sequence Toggle Bit Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sqclr(&self) -> super::vals::PipectrSqclr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PipectrSqclr::from_bits(val as u8)
    }
    #[doc = "Sequence Toggle Bit Clear"]
    #[inline(always)]
    pub const fn set_sqclr(&mut self, val: super::vals::PipectrSqclr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Auto Buffer Clear Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn aclrm(&self) -> super::vals::PipectrAclrm {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PipectrAclrm::from_bits(val as u8)
    }
    #[doc = "Auto Buffer Clear Mode"]
    #[inline(always)]
    pub const fn set_aclrm(&mut self, val: super::vals::PipectrAclrm) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Auto Response Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn atrepm(&self) -> super::vals::Atrepm {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Atrepm::from_bits(val as u8)
    }
    #[doc = "Auto Response Mode"]
    #[inline(always)]
    pub const fn set_atrepm(&mut self, val: super::vals::Atrepm) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
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
    pub const fn inbufm(&self) -> super::vals::Inbufm {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Inbufm::from_bits(val as u8)
    }
    #[doc = "Transmit Buffer Monitor"]
    #[inline(always)]
    pub const fn set_inbufm(&mut self, val: super::vals::Inbufm) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Buffer Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bsts(&self) -> super::vals::PipectrBsts {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PipectrBsts::from_bits(val as u8)
    }
    #[doc = "Buffer Status"]
    #[inline(always)]
    pub const fn set_bsts(&mut self, val: super::vals::PipectrBsts) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "Pipectr {{ pid: {:?}, reserved: {=u8:?}, pbusy: {:?}, sqmon: {:?}, sqset: {:?}, sqclr: {:?}, aclrm: {:?}, atrepm: {:?}, reserved_2: {=u8:?}, inbufm: {:?}, bsts: {:?} }}",
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
    pub const fn pbusy(&self) -> super::vals::Pipectr2Pbusy {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pipectr2Pbusy::from_bits(val as u8)
    }
    #[doc = "Pipe Busy"]
    #[inline(always)]
    pub const fn set_pbusy(&mut self, val: super::vals::Pipectr2Pbusy) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Sequence Toggle Bit Confirmation"]
    #[must_use]
    #[inline(always)]
    pub const fn sqmon(&self) -> super::vals::Pipectr2Sqmon {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pipectr2Sqmon::from_bits(val as u8)
    }
    #[doc = "Sequence Toggle Bit Confirmation"]
    #[inline(always)]
    pub const fn set_sqmon(&mut self, val: super::vals::Pipectr2Sqmon) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Sequence Toggle Bit Set"]
    #[must_use]
    #[inline(always)]
    pub const fn sqset(&self) -> super::vals::Pipectr2Sqset {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pipectr2Sqset::from_bits(val as u8)
    }
    #[doc = "Sequence Toggle Bit Set"]
    #[inline(always)]
    pub const fn set_sqset(&mut self, val: super::vals::Pipectr2Sqset) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Sequence Toggle Bit Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sqclr(&self) -> super::vals::Pipectr2Sqclr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pipectr2Sqclr::from_bits(val as u8)
    }
    #[doc = "Sequence Toggle Bit Clear"]
    #[inline(always)]
    pub const fn set_sqclr(&mut self, val: super::vals::Pipectr2Sqclr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Auto Buffer Clear Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn aclrm(&self) -> super::vals::Pipectr2Aclrm {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pipectr2Aclrm::from_bits(val as u8)
    }
    #[doc = "Auto Buffer Clear Mode"]
    #[inline(always)]
    pub const fn set_aclrm(&mut self, val: super::vals::Pipectr2Aclrm) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
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
    pub const fn bsts(&self) -> super::vals::Pipectr2Bsts {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pipectr2Bsts::from_bits(val as u8)
    }
    #[doc = "Buffer Status"]
    #[inline(always)]
    pub const fn set_bsts(&mut self, val: super::vals::Pipectr2Bsts) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            "Pipectr2 {{ pid: {:?}, reserved: {=u8:?}, pbusy: {:?}, sqmon: {:?}, sqset: {:?}, sqclr: {:?}, aclrm: {:?}, reserved_2: {=u8:?}, bsts: {:?} }}",
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
    pub const fn ifis(&self) -> super::vals::Ifis {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Ifis::from_bits(val as u8)
    }
    #[doc = "Isochronous IN Buffer Flush"]
    #[inline(always)]
    pub const fn set_ifis(&mut self, val: super::vals::Ifis) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
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
            "Pipeperi {{ iitv: {=u8:?}, reserved: {=u16:?}, ifis: {:?}, reserved_2: {=u8:?} }}",
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
    pub const fn trclr(&self) -> super::vals::Trclr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Trclr::from_bits(val as u8)
    }
    #[doc = "Transaction Counter Clear"]
    #[inline(always)]
    pub const fn set_trclr(&mut self, val: super::vals::Trclr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Transaction Counter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn trenb(&self) -> super::vals::Trenb {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Trenb::from_bits(val as u8)
    }
    #[doc = "Transaction Counter Enable"]
    #[inline(always)]
    pub const fn set_trenb(&mut self, val: super::vals::Trenb) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
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
            "Pipetre {{ reserved: {=u8:?}, trclr: {:?}, trenb: {:?}, reserved_2: {=u8:?} }}",
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
    pub const fn edgests(&self) -> super::vals::Edgests {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Edgests::from_bits(val as u8)
    }
    #[doc = "Edge Interrupt Output Status Monitor"]
    #[inline(always)]
    pub const fn set_edgests(&mut self, val: super::vals::Edgests) {
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
    #[doc = "BRDY Interrupt Status Clear Timing"]
    #[must_use]
    #[inline(always)]
    pub const fn brdym(&self) -> super::vals::Brdym {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Brdym::from_bits(val as u8)
    }
    #[doc = "BRDY Interrupt Status Clear Timing"]
    #[inline(always)]
    pub const fn set_brdym(&mut self, val: super::vals::Brdym) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
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
    pub const fn trnensel(&self) -> super::vals::Trnensel {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Trnensel::from_bits(val as u8)
    }
    #[doc = "Transaction-Enabled Time Select"]
    #[inline(always)]
    pub const fn set_trnensel(&mut self, val: super::vals::Trnensel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
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
            "Sofcfg {{ reserved: {=u8:?}, edgests: {:?}, reserved_2: {=bool:?}, brdym: {:?}, reserved_3: {=bool:?}, trnensel: {:?}, reserved_4: {=u8:?} }}",
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
    pub const fn usbe(&self) -> super::vals::Usbe {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usbe::from_bits(val as u8)
    }
    #[doc = "USB Operation Enable"]
    #[inline(always)]
    pub const fn set_usbe(&mut self, val: super::vals::Usbe) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "D- Line Resistor Control"]
    #[must_use]
    #[inline(always)]
    pub const fn dmrpu(&self) -> super::vals::Dmrpu {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmrpu::from_bits(val as u8)
    }
    #[doc = "D- Line Resistor Control"]
    #[inline(always)]
    pub const fn set_dmrpu(&mut self, val: super::vals::Dmrpu) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "D+ Line Resistor Control"]
    #[must_use]
    #[inline(always)]
    pub const fn dprpu(&self) -> super::vals::Dprpu {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dprpu::from_bits(val as u8)
    }
    #[doc = "D+ Line Resistor Control"]
    #[inline(always)]
    pub const fn set_dprpu(&mut self, val: super::vals::Dprpu) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "D+/D- Line Resistor Control"]
    #[must_use]
    #[inline(always)]
    pub const fn drpd(&self) -> super::vals::Drpd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Drpd::from_bits(val as u8)
    }
    #[doc = "D+/D- Line Resistor Control"]
    #[inline(always)]
    pub const fn set_drpd(&mut self, val: super::vals::Drpd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Controller Function Select"]
    #[must_use]
    #[inline(always)]
    pub const fn dcfm(&self) -> super::vals::Dcfm {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dcfm::from_bits(val as u8)
    }
    #[doc = "Controller Function Select"]
    #[inline(always)]
    pub const fn set_dcfm(&mut self, val: super::vals::Dcfm) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
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
    pub const fn cnen(&self) -> super::vals::Cnen {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cnen::from_bits(val as u8)
    }
    #[doc = "CNEN Single End Receiver Enable"]
    #[inline(always)]
    pub const fn set_cnen(&mut self, val: super::vals::Cnen) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
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
    pub const fn scke(&self) -> super::vals::Scke {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Scke::from_bits(val as u8)
    }
    #[doc = "USB Clock Enable"]
    #[inline(always)]
    pub const fn set_scke(&mut self, val: super::vals::Scke) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
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
            "Syscfg {{ usbe: {:?}, dmrpu: {:?}, dprpu: {:?}, drpd: {:?}, dcfm: {:?}, reserved: {=bool:?}, cnen: {:?}, reserved_2: {=bool:?}, scke: {:?}, reserved_3: {=u8:?} }}",
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
    pub const fn idmon(&self) -> super::vals::Idmon {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Idmon::from_bits(val as u8)
    }
    #[doc = "External ID0 Input Pin Monitor"]
    #[inline(always)]
    pub const fn set_idmon(&mut self, val: super::vals::Idmon) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
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
    pub const fn htact(&self) -> super::vals::Htact {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Htact::from_bits(val as u8)
    }
    #[doc = "USB Host Sequencer Status Monitor"]
    #[inline(always)]
    pub const fn set_htact(&mut self, val: super::vals::Htact) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
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
            "Syssts0 {{ lnst: {:?}, idmon: {:?}, reserved: {=u8:?}, htact: {:?}, reserved_2: {=u8:?}, ovcmon: {=u8:?} }}",
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
    pub const fn rpdme0(&self) -> super::vals::Rpdme0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rpdme0::from_bits(val as u8)
    }
    #[doc = "D- Pin Pull-Down Control"]
    #[inline(always)]
    pub const fn set_rpdme0(&mut self, val: super::vals::Rpdme0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "D+ Pin IDPSRC Output Control"]
    #[must_use]
    #[inline(always)]
    pub const fn idpsrce0(&self) -> super::vals::Idpsrce0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Idpsrce0::from_bits(val as u8)
    }
    #[doc = "D+ Pin IDPSRC Output Control"]
    #[inline(always)]
    pub const fn set_idpsrce0(&mut self, val: super::vals::Idpsrce0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "D- Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[must_use]
    #[inline(always)]
    pub const fn idmsinke0(&self) -> super::vals::Idmsinke0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Idmsinke0::from_bits(val as u8)
    }
    #[doc = "D- Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[inline(always)]
    pub const fn set_idmsinke0(&mut self, val: super::vals::Idmsinke0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "D+ Pin VDPSRC (0.6 V) Output Control"]
    #[must_use]
    #[inline(always)]
    pub const fn vdpsrce0(&self) -> super::vals::Vdpsrce0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Vdpsrce0::from_bits(val as u8)
    }
    #[doc = "D+ Pin VDPSRC (0.6 V) Output Control"]
    #[inline(always)]
    pub const fn set_vdpsrce0(&mut self, val: super::vals::Vdpsrce0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "D+ Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[must_use]
    #[inline(always)]
    pub const fn idpsinke0(&self) -> super::vals::Idpsinke0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Idpsinke0::from_bits(val as u8)
    }
    #[doc = "D+ Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[inline(always)]
    pub const fn set_idpsinke0(&mut self, val: super::vals::Idpsinke0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "D- Pin VDMSRC (0.6 V) Output Control"]
    #[must_use]
    #[inline(always)]
    pub const fn vdmsrce0(&self) -> super::vals::Vdmsrce0 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Vdmsrce0::from_bits(val as u8)
    }
    #[doc = "D- Pin VDMSRC (0.6 V) Output Control"]
    #[inline(always)]
    pub const fn set_vdmsrce0(&mut self, val: super::vals::Vdmsrce0) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
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
    pub const fn batchge0(&self) -> super::vals::Batchge0 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Batchge0::from_bits(val as u8)
    }
    #[doc = "BC (Battery Charger) Function Ch0 General Enable Control"]
    #[inline(always)]
    pub const fn set_batchge0(&mut self, val: super::vals::Batchge0) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "D- Pin 0.6 V Input Detection Status"]
    #[must_use]
    #[inline(always)]
    pub const fn chgdetsts0(&self) -> super::vals::Chgdetsts0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Chgdetsts0::from_bits(val as u8)
    }
    #[doc = "D- Pin 0.6 V Input Detection Status"]
    #[inline(always)]
    pub const fn set_chgdetsts0(&mut self, val: super::vals::Chgdetsts0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "D+ Pin 0.6 V Input Detection Status"]
    #[must_use]
    #[inline(always)]
    pub const fn pddetsts0(&self) -> super::vals::Pddetsts0 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pddetsts0::from_bits(val as u8)
    }
    #[doc = "D+ Pin 0.6 V Input Detection Status"]
    #[inline(always)]
    pub const fn set_pddetsts0(&mut self, val: super::vals::Pddetsts0) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
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
            "Usbbcctrl0 {{ rpdme0: {:?}, idpsrce0: {:?}, idmsinke0: {:?}, vdpsrce0: {:?}, idpsinke0: {:?}, vdmsrce0: {:?}, reserved: {=bool:?}, batchge0: {:?}, chgdetsts0: {:?}, pddetsts0: {:?}, reserved_2: {=u8:?} }}",
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
    pub const fn vddusbe(&self) -> super::vals::Vddusbe {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Vddusbe::from_bits(val as u8)
    }
    #[doc = "USB Reference Power Supply Circuit On/Off Control"]
    #[inline(always)]
    pub const fn set_vddusbe(&mut self, val: super::vals::Vddusbe) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
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
    pub const fn vdcen(&self) -> super::vals::Vdcen {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Vdcen::from_bits(val as u8)
    }
    #[doc = "USB Regulator On/Off Control"]
    #[inline(always)]
    pub const fn set_vdcen(&mut self, val: super::vals::Vdcen) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
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
            "Usbmc {{ vddusbe: {:?}, reserved: {=bool:?}, reserved_2: {=u8:?}, vdcen: {:?}, reserved_3: {=u8:?} }}",
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
