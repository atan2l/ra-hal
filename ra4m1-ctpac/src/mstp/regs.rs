#[doc = "Module Stop Control Register B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrb(pub u32);
impl Mstpcrb {
    #[doc = "These bits are read as 11. The write value should be 11."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 11. The write value should be 11."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Controller Area Network Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Controller Area Network Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "These bits are read as 11111. The write value should be 11111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits are read as 11111. The write value should be 11111."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
    #[doc = "I2C Bus Interface 1 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Bus Interface 1 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "I2C Bus Interface 0 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Bus Interface 0 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Universal Serial Bus 2.0 FS Interface Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Serial Bus 2.0 FS Interface Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "These bits are read as 111111. The write value should be 111111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 111111. The write value should be 111111."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[doc = "Serial Peripheral Interface 1 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Peripheral Interface 1 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Serial Peripheral Interface 0 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Peripheral Interface 0 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "These bits are read as 11. The write value should be 11."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 11. The write value should be 11."]
    #[inline(always)]
    pub const fn set_reserved_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Serial Communication Interface 9 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Communication Interface 9 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "These bits are read as 111111. The write value should be 111111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_6(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits are read as 111111. The write value should be 111111."]
    #[inline(always)]
    pub const fn set_reserved_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 23usize)) | (((val as u32) & 0x3f) << 23usize);
    }
    #[doc = "Serial Communication Interface 2 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Communication Interface 2 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Serial Communication Interface 1 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Communication Interface 1 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Serial Communication Interface 0 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Communication Interface 0 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mstpcrb {
    #[inline(always)]
    fn default() -> Mstpcrb {
        Mstpcrb(0)
    }
}
impl core::fmt::Debug for Mstpcrb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mstpcrb")
            .field("reserved", &self.reserved())
            .field("mstpb2", &self.mstpb2())
            .field("reserved_2", &self.reserved_2())
            .field("mstpb8", &self.mstpb8())
            .field("mstpb9", &self.mstpb9())
            .field("reserved_3", &self.reserved_3())
            .field("mstpb11", &self.mstpb11())
            .field("reserved_4", &self.reserved_4())
            .field("mstpb18", &self.mstpb18())
            .field("mstpb19", &self.mstpb19())
            .field("reserved_5", &self.reserved_5())
            .field("mstpb22", &self.mstpb22())
            .field("reserved_6", &self.reserved_6())
            .field("mstpb29", &self.mstpb29())
            .field("mstpb30", &self.mstpb30())
            .field("mstpb31", &self.mstpb31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mstpcrb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mstpcrb {{ reserved: {=u8:?}, mstpb2: {=bool:?}, reserved_2: {=u8:?}, mstpb8: {=bool:?}, mstpb9: {=bool:?}, reserved_3: {=bool:?}, mstpb11: {=bool:?}, reserved_4: {=u8:?}, mstpb18: {=bool:?}, mstpb19: {=bool:?}, reserved_5: {=u8:?}, mstpb22: {=bool:?}, reserved_6: {=u8:?}, mstpb29: {=bool:?}, mstpb30: {=bool:?}, mstpb31: {=bool:?} }}",
            self.reserved(),
            self.mstpb2(),
            self.reserved_2(),
            self.mstpb8(),
            self.mstpb9(),
            self.reserved_3(),
            self.mstpb11(),
            self.reserved_4(),
            self.mstpb18(),
            self.mstpb19(),
            self.reserved_5(),
            self.mstpb22(),
            self.reserved_6(),
            self.mstpb29(),
            self.mstpb30(),
            self.mstpb31()
        )
    }
}
#[doc = "Module Stop Control Register C"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrc(pub u32);
impl Mstpcrc {
    #[doc = "Clock Frequency Accuracy Measurement Circuit Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Frequency Accuracy Measurement Circuit Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Cyclic Redundancy Check Calculator Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Cyclic Redundancy Check Calculator Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Capacitive Touch Sensing Unit Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capacitive Touch Sensing Unit Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Segment LCD Controller Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Segment LCD Controller Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "These bits are read as 111. The write value should be 111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 111. The write value should be 111."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Synchronous Serial Interface 0 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous Serial Interface 0 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "These bits are read as 1111. The write value should be 1111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 1111. The write value should be 1111."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
    }
    #[doc = "Data Operation Circuit Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Data Operation Circuit Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Event Link Controller Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Event Link Controller Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "These bits are read as 1111111111111111. The write value should be 1111111111111111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> u16 {
        let val = (self.0 >> 15usize) & 0xffff;
        val as u16
    }
    #[doc = "These bits are read as 1111111111111111. The write value should be 1111111111111111."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 15usize)) | (((val as u32) & 0xffff) << 15usize);
    }
    #[doc = "SCE5 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SCE5 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mstpcrc {
    #[inline(always)]
    fn default() -> Mstpcrc {
        Mstpcrc(0)
    }
}
impl core::fmt::Debug for Mstpcrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mstpcrc")
            .field("mstpc0", &self.mstpc0())
            .field("mstpc1", &self.mstpc1())
            .field("reserved", &self.reserved())
            .field("mstpc3", &self.mstpc3())
            .field("mstpc4", &self.mstpc4())
            .field("reserved_2", &self.reserved_2())
            .field("mstpc8", &self.mstpc8())
            .field("reserved_3", &self.reserved_3())
            .field("mstpc13", &self.mstpc13())
            .field("mstpc14", &self.mstpc14())
            .field("reserved_4", &self.reserved_4())
            .field("mstpc31", &self.mstpc31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mstpcrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mstpcrc {{ mstpc0: {=bool:?}, mstpc1: {=bool:?}, reserved: {=bool:?}, mstpc3: {=bool:?}, mstpc4: {=bool:?}, reserved_2: {=u8:?}, mstpc8: {=bool:?}, reserved_3: {=u8:?}, mstpc13: {=bool:?}, mstpc14: {=bool:?}, reserved_4: {=u16:?}, mstpc31: {=bool:?} }}",
            self.mstpc0(),
            self.mstpc1(),
            self.reserved(),
            self.mstpc3(),
            self.mstpc4(),
            self.reserved_2(),
            self.mstpc8(),
            self.reserved_3(),
            self.mstpc13(),
            self.mstpc14(),
            self.reserved_4(),
            self.mstpc31()
        )
    }
}
#[doc = "Module Stop Control Register D"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrd(pub u32);
impl Mstpcrd {
    #[doc = "These bits are read as 11. The write value should be 11."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 11. The write value should be 11."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Asynchronous General Purpose Timer 1 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous General Purpose Timer 1 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Asynchronous General Purpose Timer 0 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous General Purpose Timer 0 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "General PWM Timer 323 to 320 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "General PWM Timer 323 to 320 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "General PWM Timer 169 to 164 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "General PWM Timer 169 to 164 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "These bits are read as 1111111. The write value should be 1111111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 1111111. The write value should be 1111111."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[doc = "Port Output Enable for GPT Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port Output Enable for GPT Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "14-Bit A/D Converter Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "14-Bit A/D Converter Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits are read as 11. The write value should be 11."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_5(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 11. The write value should be 11."]
    #[inline(always)]
    pub const fn set_reserved_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
    }
    #[doc = "8-bit D/A Converter Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "8-bit D/A Converter Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "12-Bit D/A Converter Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "12-Bit D/A Converter Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "These bits are read as 11111111. The write value should be 11111111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_6(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are read as 11111111. The write value should be 11111111."]
    #[inline(always)]
    pub const fn set_reserved_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 21usize)) | (((val as u32) & 0xff) << 21usize);
    }
    #[doc = "Low-Power Analog Comparator Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Low-Power Analog Comparator Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_7(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[inline(always)]
    pub const fn set_reserved_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Operational Amplifier Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Operational Amplifier Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mstpcrd {
    #[inline(always)]
    fn default() -> Mstpcrd {
        Mstpcrd(0)
    }
}
impl core::fmt::Debug for Mstpcrd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mstpcrd")
            .field("reserved", &self.reserved())
            .field("mstpd2", &self.mstpd2())
            .field("mstpd3", &self.mstpd3())
            .field("reserved_2", &self.reserved_2())
            .field("mstpd5", &self.mstpd5())
            .field("mstpd6", &self.mstpd6())
            .field("reserved_3", &self.reserved_3())
            .field("mstpd14", &self.mstpd14())
            .field("reserved_4", &self.reserved_4())
            .field("mstpd16", &self.mstpd16())
            .field("reserved_5", &self.reserved_5())
            .field("mstpd19", &self.mstpd19())
            .field("mstpd20", &self.mstpd20())
            .field("reserved_6", &self.reserved_6())
            .field("mstpd29", &self.mstpd29())
            .field("reserved_7", &self.reserved_7())
            .field("mstpd31", &self.mstpd31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mstpcrd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mstpcrd {{ reserved: {=u8:?}, mstpd2: {=bool:?}, mstpd3: {=bool:?}, reserved_2: {=bool:?}, mstpd5: {=bool:?}, mstpd6: {=bool:?}, reserved_3: {=u8:?}, mstpd14: {=bool:?}, reserved_4: {=bool:?}, mstpd16: {=bool:?}, reserved_5: {=u8:?}, mstpd19: {=bool:?}, mstpd20: {=bool:?}, reserved_6: {=u8:?}, mstpd29: {=bool:?}, reserved_7: {=bool:?}, mstpd31: {=bool:?} }}",
            self.reserved(),
            self.mstpd2(),
            self.mstpd3(),
            self.reserved_2(),
            self.mstpd5(),
            self.mstpd6(),
            self.reserved_3(),
            self.mstpd14(),
            self.reserved_4(),
            self.mstpd16(),
            self.reserved_5(),
            self.mstpd19(),
            self.mstpd20(),
            self.reserved_6(),
            self.mstpd29(),
            self.reserved_7(),
            self.mstpd31()
        )
    }
}
