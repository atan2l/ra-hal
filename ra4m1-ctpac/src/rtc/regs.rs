#[doc = "Binary Counter 0 Alarm Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt0aer(pub u8);
impl Bcnt0aer {
    #[doc = "The BCNT0AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b7 to b0."]
    #[must_use]
    #[inline(always)]
    pub const fn enb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The BCNT0AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b7 to b0."]
    #[inline(always)]
    pub const fn set_enb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bcnt0aer {
    #[inline(always)]
    fn default() -> Bcnt0aer {
        Bcnt0aer(0)
    }
}
impl core::fmt::Debug for Bcnt0aer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bcnt0aer")
            .field("enb", &self.enb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bcnt0aer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bcnt0aer {{ enb: {=u8:?} }}", self.enb())
    }
}
#[doc = "Binary Counter 0 Alarm Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt0ar(pub u8);
impl Bcnt0ar {
    #[doc = "he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0."]
    #[must_use]
    #[inline(always)]
    pub const fn bcnt0ar(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0."]
    #[inline(always)]
    pub const fn set_bcnt0ar(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bcnt0ar {
    #[inline(always)]
    fn default() -> Bcnt0ar {
        Bcnt0ar(0)
    }
}
impl core::fmt::Debug for Bcnt0ar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bcnt0ar")
            .field("bcnt0ar", &self.bcnt0ar())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bcnt0ar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bcnt0ar {{ bcnt0ar: {=u8:?} }}", self.bcnt0ar())
    }
}
#[doc = "BCNT0 Capture Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt0cp(pub u8);
impl Bcnt0cp {
    #[doc = "BCNT0CP is a read-only register that captures the BCNT0 value when a time capture event is detected."]
    #[must_use]
    #[inline(always)]
    pub const fn bcnt0cp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "BCNT0CP is a read-only register that captures the BCNT0 value when a time capture event is detected."]
    #[inline(always)]
    pub const fn set_bcnt0cp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bcnt0cp {
    #[inline(always)]
    fn default() -> Bcnt0cp {
        Bcnt0cp(0)
    }
}
impl core::fmt::Debug for Bcnt0cp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bcnt0cp")
            .field("bcnt0cp", &self.bcnt0cp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bcnt0cp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bcnt0cp {{ bcnt0cp: {=u8:?} }}", self.bcnt0cp())
    }
}
#[doc = "Binary Counter 1 Alarm Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt1aer(pub u8);
impl Bcnt1aer {
    #[doc = "The BCNT1AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b15 to b8."]
    #[must_use]
    #[inline(always)]
    pub const fn enb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The BCNT1AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b15 to b8."]
    #[inline(always)]
    pub const fn set_enb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bcnt1aer {
    #[inline(always)]
    fn default() -> Bcnt1aer {
        Bcnt1aer(0)
    }
}
impl core::fmt::Debug for Bcnt1aer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bcnt1aer")
            .field("enb", &self.enb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bcnt1aer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bcnt1aer {{ enb: {=u8:?} }}", self.enb())
    }
}
#[doc = "Binary Counter 1 Alarm Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt1ar(pub u8);
impl Bcnt1ar {
    #[doc = "he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
    #[must_use]
    #[inline(always)]
    pub const fn bcnt1ar(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
    #[inline(always)]
    pub const fn set_bcnt1ar(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bcnt1ar {
    #[inline(always)]
    fn default() -> Bcnt1ar {
        Bcnt1ar(0)
    }
}
impl core::fmt::Debug for Bcnt1ar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bcnt1ar")
            .field("bcnt1ar", &self.bcnt1ar())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bcnt1ar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bcnt1ar {{ bcnt1ar: {=u8:?} }}", self.bcnt1ar())
    }
}
#[doc = "BCNT1 Capture Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt1cp(pub u8);
impl Bcnt1cp {
    #[doc = "BCNT1CP is a read-only register that captures the BCNT1 value when a time capture event is detected."]
    #[must_use]
    #[inline(always)]
    pub const fn bcnt1cp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "BCNT1CP is a read-only register that captures the BCNT1 value when a time capture event is detected."]
    #[inline(always)]
    pub const fn set_bcnt1cp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bcnt1cp {
    #[inline(always)]
    fn default() -> Bcnt1cp {
        Bcnt1cp(0)
    }
}
impl core::fmt::Debug for Bcnt1cp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bcnt1cp")
            .field("bcnt1cp", &self.bcnt1cp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bcnt1cp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bcnt1cp {{ bcnt1cp: {=u8:?} }}", self.bcnt1cp())
    }
}
#[doc = "Binary Counter 2 Alarm Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt2aer(pub u16);
impl Bcnt2aer {
    #[doc = "The BCNT2AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b23 to b16."]
    #[must_use]
    #[inline(always)]
    pub const fn enb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The BCNT2AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b23 to b16."]
    #[inline(always)]
    pub const fn set_enb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Bcnt2aer {
    #[inline(always)]
    fn default() -> Bcnt2aer {
        Bcnt2aer(0)
    }
}
impl core::fmt::Debug for Bcnt2aer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bcnt2aer")
            .field("enb", &self.enb())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bcnt2aer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bcnt2aer {{ enb: {=u8:?}, reserved: {=u8:?} }}",
            self.enb(),
            self.reserved()
        )
    }
}
#[doc = "Binary Counter 2 Alarm Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt2ar(pub u8);
impl Bcnt2ar {
    #[doc = "The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
    #[must_use]
    #[inline(always)]
    pub const fn bcnt2ar(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
    #[inline(always)]
    pub const fn set_bcnt2ar(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bcnt2ar {
    #[inline(always)]
    fn default() -> Bcnt2ar {
        Bcnt2ar(0)
    }
}
impl core::fmt::Debug for Bcnt2ar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bcnt2ar")
            .field("bcnt2ar", &self.bcnt2ar())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bcnt2ar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bcnt2ar {{ bcnt2ar: {=u8:?} }}", self.bcnt2ar())
    }
}
#[doc = "BCNT2 Capture Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt2cp(pub u8);
impl Bcnt2cp {
    #[doc = "BCNT2CP is a read-only register that captures the BCNT2 value when a time capture event is detected."]
    #[must_use]
    #[inline(always)]
    pub const fn bcnt2cp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "BCNT2CP is a read-only register that captures the BCNT2 value when a time capture event is detected."]
    #[inline(always)]
    pub const fn set_bcnt2cp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bcnt2cp {
    #[inline(always)]
    fn default() -> Bcnt2cp {
        Bcnt2cp(0)
    }
}
impl core::fmt::Debug for Bcnt2cp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bcnt2cp")
            .field("bcnt2cp", &self.bcnt2cp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bcnt2cp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bcnt2cp {{ bcnt2cp: {=u8:?} }}", self.bcnt2cp())
    }
}
#[doc = "Binary Counter 3 Alarm Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt3aer(pub u8);
impl Bcnt3aer {
    #[doc = "The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24."]
    #[must_use]
    #[inline(always)]
    pub const fn enb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24."]
    #[inline(always)]
    pub const fn set_enb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bcnt3aer {
    #[inline(always)]
    fn default() -> Bcnt3aer {
        Bcnt3aer(0)
    }
}
impl core::fmt::Debug for Bcnt3aer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bcnt3aer")
            .field("enb", &self.enb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bcnt3aer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bcnt3aer {{ enb: {=u8:?} }}", self.enb())
    }
}
#[doc = "Binary Counter 3 Alarm Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt3ar(pub u8);
impl Bcnt3ar {
    #[doc = "The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
    #[must_use]
    #[inline(always)]
    pub const fn bcnt3ar(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
    #[inline(always)]
    pub const fn set_bcnt3ar(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bcnt3ar {
    #[inline(always)]
    fn default() -> Bcnt3ar {
        Bcnt3ar(0)
    }
}
impl core::fmt::Debug for Bcnt3ar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bcnt3ar")
            .field("bcnt3ar", &self.bcnt3ar())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bcnt3ar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bcnt3ar {{ bcnt3ar: {=u8:?} }}", self.bcnt3ar())
    }
}
#[doc = "BCNT3 Capture Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt3cp(pub u8);
impl Bcnt3cp {
    #[doc = "BCNT3CP is a read-only register that captures the BCNT3 value when a time capture event is detected."]
    #[must_use]
    #[inline(always)]
    pub const fn bcnt3cp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "BCNT3CP is a read-only register that captures the BCNT3 value when a time capture event is detected."]
    #[inline(always)]
    pub const fn set_bcnt3cp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bcnt3cp {
    #[inline(always)]
    fn default() -> Bcnt3cp {
        Bcnt3cp(0)
    }
}
impl core::fmt::Debug for Bcnt3cp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bcnt3cp")
            .field("bcnt3cp", &self.bcnt3cp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bcnt3cp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bcnt3cp {{ bcnt3cp: {=u8:?} }}", self.bcnt3cp())
    }
}
#[doc = "64-Hz Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct R64cnt(pub u8);
impl R64cnt {
    #[doc = "64Hz"]
    #[must_use]
    #[inline(always)]
    pub const fn f64hz(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "64Hz"]
    #[inline(always)]
    pub const fn set_f64hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "32Hz"]
    #[must_use]
    #[inline(always)]
    pub const fn f32hz(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "32Hz"]
    #[inline(always)]
    pub const fn set_f32hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "16Hz"]
    #[must_use]
    #[inline(always)]
    pub const fn f16hz(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "16Hz"]
    #[inline(always)]
    pub const fn set_f16hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "8Hz"]
    #[must_use]
    #[inline(always)]
    pub const fn f8hz(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "8Hz"]
    #[inline(always)]
    pub const fn set_f8hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "4Hz"]
    #[must_use]
    #[inline(always)]
    pub const fn f4hz(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4Hz"]
    #[inline(always)]
    pub const fn set_f4hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "2Hz"]
    #[must_use]
    #[inline(always)]
    pub const fn f2hz(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "2Hz"]
    #[inline(always)]
    pub const fn set_f2hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "1Hz"]
    #[must_use]
    #[inline(always)]
    pub const fn f1hz(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1Hz"]
    #[inline(always)]
    pub const fn set_f1hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
}
impl Default for R64cnt {
    #[inline(always)]
    fn default() -> R64cnt {
        R64cnt(0)
    }
}
impl core::fmt::Debug for R64cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R64cnt")
            .field("f64hz", &self.f64hz())
            .field("f32hz", &self.f32hz())
            .field("f16hz", &self.f16hz())
            .field("f8hz", &self.f8hz())
            .field("f4hz", &self.f4hz())
            .field("f2hz", &self.f2hz())
            .field("f1hz", &self.f1hz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for R64cnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "R64cnt {{ f64hz: {=bool:?}, f32hz: {=bool:?}, f16hz: {=bool:?}, f8hz: {=bool:?}, f4hz: {=bool:?}, f2hz: {=bool:?}, f1hz: {=bool:?} }}",
            self.f64hz(),
            self.f32hz(),
            self.f16hz(),
            self.f8hz(),
            self.f4hz(),
            self.f2hz(),
            self.f1hz()
        )
    }
}
#[doc = "Time Error Adjustment Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Radj(pub u8);
impl Radj {
    #[doc = "Adjustment Value These bits specify the adjustment value from the prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn adj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Adjustment Value These bits specify the adjustment value from the prescaler."]
    #[inline(always)]
    pub const fn set_adj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u8) & 0x3f) << 0usize);
    }
    #[doc = "Plus-Minus"]
    #[must_use]
    #[inline(always)]
    pub const fn pmadj(&self) -> super::vals::Pmadj {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Pmadj::from_bits(val as u8)
    }
    #[doc = "Plus-Minus"]
    #[inline(always)]
    pub const fn set_pmadj(&mut self, val: super::vals::Pmadj) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u8) & 0x03) << 6usize);
    }
}
impl Default for Radj {
    #[inline(always)]
    fn default() -> Radj {
        Radj(0)
    }
}
impl core::fmt::Debug for Radj {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Radj")
            .field("adj", &self.adj())
            .field("pmadj", &self.pmadj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Radj {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Radj {{ adj: {=u8:?}, pmadj: {:?} }}",
            self.adj(),
            self.pmadj()
        )
    }
}
#[doc = "RTC Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr1(pub u8);
impl Rcr1 {
    #[doc = "Alarm Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm Interrupt Enable"]
    #[inline(always)]
    pub const fn set_aie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Carry Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Carry Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Periodic Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Periodic Interrupt Enable"]
    #[inline(always)]
    pub const fn set_pie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "RTCOUT Output Select"]
    #[must_use]
    #[inline(always)]
    pub const fn rtcos(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RTCOUT Output Select"]
    #[inline(always)]
    pub const fn set_rtcos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Periodic Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pes(&self) -> super::vals::Pes {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Pes::from_bits(val as u8)
    }
    #[doc = "Periodic Interrupt Select"]
    #[inline(always)]
    pub const fn set_pes(&mut self, val: super::vals::Pes) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u8) & 0x0f) << 4usize);
    }
}
impl Default for Rcr1 {
    #[inline(always)]
    fn default() -> Rcr1 {
        Rcr1(0)
    }
}
impl core::fmt::Debug for Rcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr1")
            .field("aie", &self.aie())
            .field("cie", &self.cie())
            .field("pie", &self.pie())
            .field("rtcos", &self.rtcos())
            .field("pes", &self.pes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr1 {{ aie: {=bool:?}, cie: {=bool:?}, pie: {=bool:?}, rtcos: {=bool:?}, pes: {:?} }}",
            self.aie(),
            self.cie(),
            self.pie(),
            self.rtcos(),
            self.pes()
        )
    }
}
#[doc = "RTC Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr2(pub u8);
impl Rcr2 {
    #[doc = "Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "RTC Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RTC Software Reset"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "30-Second Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn adj30(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "30-Second Adjustment"]
    #[inline(always)]
    pub const fn set_adj30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "RTCOUT Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rtcoe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RTCOUT Output Enable"]
    #[inline(always)]
    pub const fn set_rtcoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)"]
    #[must_use]
    #[inline(always)]
    pub const fn aadje(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)"]
    #[inline(always)]
    pub const fn set_aadje(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)"]
    #[must_use]
    #[inline(always)]
    pub const fn aadjp(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)"]
    #[inline(always)]
    pub const fn set_aadjp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Hours Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hr24(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Hours Mode"]
    #[inline(always)]
    pub const fn set_hr24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Count Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cntmd(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Count Mode Select"]
    #[inline(always)]
    pub const fn set_cntmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Rcr2 {
    #[inline(always)]
    fn default() -> Rcr2 {
        Rcr2(0)
    }
}
impl core::fmt::Debug for Rcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr2")
            .field("start", &self.start())
            .field("reset", &self.reset())
            .field("adj30", &self.adj30())
            .field("rtcoe", &self.rtcoe())
            .field("aadje", &self.aadje())
            .field("aadjp", &self.aadjp())
            .field("hr24", &self.hr24())
            .field("cntmd", &self.cntmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr2 {{ start: {=bool:?}, reset: {=bool:?}, adj30: {=bool:?}, rtcoe: {=bool:?}, aadje: {=bool:?}, aadjp: {=bool:?}, hr24: {=bool:?}, cntmd: {=bool:?} }}",
            self.start(),
            self.reset(),
            self.adj30(),
            self.rtcoe(),
            self.aadje(),
            self.aadjp(),
            self.hr24(),
            self.cntmd()
        )
    }
}
#[doc = "RTC Control Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr4(pub u8);
impl Rcr4 {
    #[doc = "Count Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn rcksel(&self) -> super::vals::Rcksel {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rcksel::from_bits(val as u8)
    }
    #[doc = "Count Source Select"]
    #[inline(always)]
    pub const fn set_rcksel(&mut self, val: super::vals::Rcksel) {
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
impl Default for Rcr4 {
    #[inline(always)]
    fn default() -> Rcr4 {
        Rcr4(0)
    }
}
impl core::fmt::Debug for Rcr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr4")
            .field("rcksel", &self.rcksel())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr4 {{ rcksel: {:?}, reserved: {=u8:?} }}",
            self.rcksel(),
            self.reserved()
        )
    }
}
#[doc = "Date Alarm Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdayar(pub u8);
impl Rdayar {
    #[doc = "1 Day Value for the ones place of days"]
    #[must_use]
    #[inline(always)]
    pub const fn date1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1 Day Value for the ones place of days"]
    #[inline(always)]
    pub const fn set_date1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10 Days Value for the tens place of days"]
    #[must_use]
    #[inline(always)]
    pub const fn date10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "10 Days Value for the tens place of days"]
    #[inline(always)]
    pub const fn set_date10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
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
    #[doc = "Compare enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enb(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Compare enable"]
    #[inline(always)]
    pub const fn set_enb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Rdayar {
    #[inline(always)]
    fn default() -> Rdayar {
        Rdayar(0)
    }
}
impl core::fmt::Debug for Rdayar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdayar")
            .field("date1", &self.date1())
            .field("date10", &self.date10())
            .field("reserved", &self.reserved())
            .field("enb", &self.enb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdayar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rdayar {{ date1: {=u8:?}, date10: {=u8:?}, reserved: {=bool:?}, enb: {=bool:?} }}",
            self.date1(),
            self.date10(),
            self.reserved(),
            self.enb()
        )
    }
}
#[doc = "Day Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdaycnt(pub u8);
impl Rdaycnt {
    #[doc = "1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place."]
    #[must_use]
    #[inline(always)]
    pub const fn date1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub const fn set_date1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10-Day Count Counts from 0 to 3 once per carry from the ones place."]
    #[must_use]
    #[inline(always)]
    pub const fn date10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "10-Day Count Counts from 0 to 3 once per carry from the ones place."]
    #[inline(always)]
    pub const fn set_date10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
}
impl Default for Rdaycnt {
    #[inline(always)]
    fn default() -> Rdaycnt {
        Rdaycnt(0)
    }
}
impl core::fmt::Debug for Rdaycnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdaycnt")
            .field("date1", &self.date1())
            .field("date10", &self.date10())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdaycnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rdaycnt {{ date1: {=u8:?}, date10: {=u8:?} }}",
            self.date1(),
            self.date10()
        )
    }
}
#[doc = "Date Capture Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdaycp(pub u8);
impl Rdaycp {
    #[doc = "1-Day Capture Capture value for the ones place of minutes"]
    #[must_use]
    #[inline(always)]
    pub const fn date1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1-Day Capture Capture value for the ones place of minutes"]
    #[inline(always)]
    pub const fn set_date1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10-Day Capture Capture value for the tens place of minutes"]
    #[must_use]
    #[inline(always)]
    pub const fn date10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "10-Day Capture Capture value for the tens place of minutes"]
    #[inline(always)]
    pub const fn set_date10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "These bits are read as 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
    }
}
impl Default for Rdaycp {
    #[inline(always)]
    fn default() -> Rdaycp {
        Rdaycp(0)
    }
}
impl core::fmt::Debug for Rdaycp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdaycp")
            .field("date1", &self.date1())
            .field("date10", &self.date10())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdaycp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rdaycp {{ date1: {=u8:?}, date10: {=u8:?}, reserved: {=u8:?} }}",
            self.date1(),
            self.date10(),
            self.reserved()
        )
    }
}
#[doc = "Frequency Register H"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfrh(pub u16);
impl Rfrh {
    #[doc = "Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn rfc16(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
    #[inline(always)]
    pub const fn set_rfc16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x7fff;
        val as u16
    }
    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 1usize)) | (((val as u16) & 0x7fff) << 1usize);
    }
}
impl Default for Rfrh {
    #[inline(always)]
    fn default() -> Rfrh {
        Rfrh(0)
    }
}
impl core::fmt::Debug for Rfrh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rfrh")
            .field("rfc16", &self.rfc16())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rfrh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rfrh {{ rfc16: {=bool:?}, reserved: {=u16:?} }}",
            self.rfc16(),
            self.reserved()
        )
    }
}
#[doc = "Hour Alarm Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rhrar(pub u8);
impl Rhrar {
    #[doc = "1-Hour Count Value for the ones place of hours"]
    #[must_use]
    #[inline(always)]
    pub const fn hr1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1-Hour Count Value for the ones place of hours"]
    #[inline(always)]
    pub const fn set_hr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10-Hour Count Value for the tens place of hours"]
    #[must_use]
    #[inline(always)]
    pub const fn hr10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "10-Hour Count Value for the tens place of hours"]
    #[inline(always)]
    pub const fn set_hr10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Time Counter Setting for a.m./p.m."]
    #[must_use]
    #[inline(always)]
    pub const fn pm(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Time Counter Setting for a.m./p.m."]
    #[inline(always)]
    pub const fn set_pm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Compare enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enb(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Compare enable"]
    #[inline(always)]
    pub const fn set_enb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Rhrar {
    #[inline(always)]
    fn default() -> Rhrar {
        Rhrar(0)
    }
}
impl core::fmt::Debug for Rhrar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rhrar")
            .field("hr1", &self.hr1())
            .field("hr10", &self.hr10())
            .field("pm", &self.pm())
            .field("enb", &self.enb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rhrar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rhrar {{ hr1: {=u8:?}, hr10: {=u8:?}, pm: {=bool:?}, enb: {=bool:?} }}",
            self.hr1(),
            self.hr10(),
            self.pm(),
            self.enb()
        )
    }
}
#[doc = "Hour Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rhrcnt(pub u8);
impl Rhrcnt {
    #[doc = "1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place."]
    #[must_use]
    #[inline(always)]
    pub const fn hr1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub const fn set_hr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10-Hour Count Counts from 0 to 2 once per carry from the ones place."]
    #[must_use]
    #[inline(always)]
    pub const fn hr10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "10-Hour Count Counts from 0 to 2 once per carry from the ones place."]
    #[inline(always)]
    pub const fn set_hr10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Time Counter Setting for a.m./p.m."]
    #[must_use]
    #[inline(always)]
    pub const fn pm(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Time Counter Setting for a.m./p.m."]
    #[inline(always)]
    pub const fn set_pm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
}
impl Default for Rhrcnt {
    #[inline(always)]
    fn default() -> Rhrcnt {
        Rhrcnt(0)
    }
}
impl core::fmt::Debug for Rhrcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rhrcnt")
            .field("hr1", &self.hr1())
            .field("hr10", &self.hr10())
            .field("pm", &self.pm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rhrcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rhrcnt {{ hr1: {=u8:?}, hr10: {=u8:?}, pm: {=bool:?} }}",
            self.hr1(),
            self.hr10(),
            self.pm()
        )
    }
}
#[doc = "Hour Capture Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rhrcp(pub u8);
impl Rhrcp {
    #[doc = "1-Minute Capture Capture value for the ones place of minutes"]
    #[must_use]
    #[inline(always)]
    pub const fn hr1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1-Minute Capture Capture value for the ones place of minutes"]
    #[inline(always)]
    pub const fn set_hr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10-Minute Capture Capture value for the tens place of minutes"]
    #[must_use]
    #[inline(always)]
    pub const fn hr10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "10-Minute Capture Capture value for the tens place of minutes"]
    #[inline(always)]
    pub const fn set_hr10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "A.m./p.m. select for time counter setting."]
    #[must_use]
    #[inline(always)]
    pub const fn pm(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "A.m./p.m. select for time counter setting."]
    #[inline(always)]
    pub const fn set_pm(&mut self, val: bool) {
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
impl Default for Rhrcp {
    #[inline(always)]
    fn default() -> Rhrcp {
        Rhrcp(0)
    }
}
impl core::fmt::Debug for Rhrcp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rhrcp")
            .field("hr1", &self.hr1())
            .field("hr10", &self.hr10())
            .field("pm", &self.pm())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rhrcp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rhrcp {{ hr1: {=u8:?}, hr10: {=u8:?}, pm: {=bool:?}, reserved: {=bool:?} }}",
            self.hr1(),
            self.hr10(),
            self.pm(),
            self.reserved()
        )
    }
}
#[doc = "Minute Alarm Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rminar(pub u8);
impl Rminar {
    #[doc = "1-Minute Count Value for the ones place of minutes"]
    #[must_use]
    #[inline(always)]
    pub const fn min1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1-Minute Count Value for the ones place of minutes"]
    #[inline(always)]
    pub const fn set_min1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10-Minute Count Value for the tens place of minutes"]
    #[must_use]
    #[inline(always)]
    pub const fn min10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "10-Minute Count Value for the tens place of minutes"]
    #[inline(always)]
    pub const fn set_min10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
    }
    #[doc = "Compare enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enb(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Compare enable"]
    #[inline(always)]
    pub const fn set_enb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Rminar {
    #[inline(always)]
    fn default() -> Rminar {
        Rminar(0)
    }
}
impl core::fmt::Debug for Rminar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rminar")
            .field("min1", &self.min1())
            .field("min10", &self.min10())
            .field("enb", &self.enb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rminar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rminar {{ min1: {=u8:?}, min10: {=u8:?}, enb: {=bool:?} }}",
            self.min1(),
            self.min10(),
            self.enb()
        )
    }
}
#[doc = "Minute Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmincnt(pub u8);
impl Rmincnt {
    #[doc = "1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place."]
    #[must_use]
    #[inline(always)]
    pub const fn min1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub const fn set_min1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10-Minute Count Counts from 0 to 5 for 60-minute counting."]
    #[must_use]
    #[inline(always)]
    pub const fn min10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "10-Minute Count Counts from 0 to 5 for 60-minute counting."]
    #[inline(always)]
    pub const fn set_min10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
    }
}
impl Default for Rmincnt {
    #[inline(always)]
    fn default() -> Rmincnt {
        Rmincnt(0)
    }
}
impl core::fmt::Debug for Rmincnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rmincnt")
            .field("min1", &self.min1())
            .field("min10", &self.min10())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rmincnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rmincnt {{ min1: {=u8:?}, min10: {=u8:?} }}",
            self.min1(),
            self.min10()
        )
    }
}
#[doc = "Minute Capture Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmincp(pub u8);
impl Rmincp {
    #[doc = "1-Minute Capture Capture value for the ones place of minutes"]
    #[must_use]
    #[inline(always)]
    pub const fn min1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1-Minute Capture Capture value for the ones place of minutes"]
    #[inline(always)]
    pub const fn set_min1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10-Minute Capture Capture value for the tens place of minutes"]
    #[must_use]
    #[inline(always)]
    pub const fn min10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "10-Minute Capture Capture value for the tens place of minutes"]
    #[inline(always)]
    pub const fn set_min10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
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
impl Default for Rmincp {
    #[inline(always)]
    fn default() -> Rmincp {
        Rmincp(0)
    }
}
impl core::fmt::Debug for Rmincp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rmincp")
            .field("min1", &self.min1())
            .field("min10", &self.min10())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rmincp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rmincp {{ min1: {=u8:?}, min10: {=u8:?}, reserved: {=bool:?} }}",
            self.min1(),
            self.min10(),
            self.reserved()
        )
    }
}
#[doc = "Month Alarm Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmonar(pub u8);
impl Rmonar {
    #[doc = "1 Month Value for the ones place of months"]
    #[must_use]
    #[inline(always)]
    pub const fn mon1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1 Month Value for the ones place of months"]
    #[inline(always)]
    pub const fn set_mon1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10 Months Value for the tens place of months"]
    #[must_use]
    #[inline(always)]
    pub const fn mon10(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "10 Months Value for the tens place of months"]
    #[inline(always)]
    pub const fn set_mon10(&mut self, val: bool) {
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
    #[doc = "Compare enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enb(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Compare enable"]
    #[inline(always)]
    pub const fn set_enb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Rmonar {
    #[inline(always)]
    fn default() -> Rmonar {
        Rmonar(0)
    }
}
impl core::fmt::Debug for Rmonar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rmonar")
            .field("mon1", &self.mon1())
            .field("mon10", &self.mon10())
            .field("reserved", &self.reserved())
            .field("enb", &self.enb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rmonar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rmonar {{ mon1: {=u8:?}, mon10: {=bool:?}, reserved: {=u8:?}, enb: {=bool:?} }}",
            self.mon1(),
            self.mon10(),
            self.reserved(),
            self.enb()
        )
    }
}
#[doc = "Month Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmoncnt(pub u8);
impl Rmoncnt {
    #[doc = "1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
    #[must_use]
    #[inline(always)]
    pub const fn mon1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub const fn set_mon1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10-Month Count Counts from 0 to 1 once per carry from the ones place."]
    #[must_use]
    #[inline(always)]
    pub const fn mon10(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "10-Month Count Counts from 0 to 1 once per carry from the ones place."]
    #[inline(always)]
    pub const fn set_mon10(&mut self, val: bool) {
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
impl Default for Rmoncnt {
    #[inline(always)]
    fn default() -> Rmoncnt {
        Rmoncnt(0)
    }
}
impl core::fmt::Debug for Rmoncnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rmoncnt")
            .field("mon1", &self.mon1())
            .field("mon10", &self.mon10())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rmoncnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rmoncnt {{ mon1: {=u8:?}, mon10: {=bool:?}, reserved: {=u8:?} }}",
            self.mon1(),
            self.mon10(),
            self.reserved()
        )
    }
}
#[doc = "Month Capture Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmoncp(pub u8);
impl Rmoncp {
    #[doc = "1-Month Capture Capture value for the ones place of months"]
    #[must_use]
    #[inline(always)]
    pub const fn mon1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1-Month Capture Capture value for the ones place of months"]
    #[inline(always)]
    pub const fn set_mon1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10-Month Capture Capture value for the tens place of months"]
    #[must_use]
    #[inline(always)]
    pub const fn mon10(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "10-Month Capture Capture value for the tens place of months"]
    #[inline(always)]
    pub const fn set_mon10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
}
impl Default for Rmoncp {
    #[inline(always)]
    fn default() -> Rmoncp {
        Rmoncp(0)
    }
}
impl core::fmt::Debug for Rmoncp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rmoncp")
            .field("mon1", &self.mon1())
            .field("mon10", &self.mon10())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rmoncp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rmoncp {{ mon1: {=u8:?}, mon10: {=bool:?} }}",
            self.mon1(),
            self.mon10()
        )
    }
}
#[doc = "Second Alarm Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsecar(pub u8);
impl Rsecar {
    #[doc = "1-Second Value for the ones place of seconds"]
    #[must_use]
    #[inline(always)]
    pub const fn sec1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1-Second Value for the ones place of seconds"]
    #[inline(always)]
    pub const fn set_sec1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10-Seconds Value for the tens place of seconds"]
    #[must_use]
    #[inline(always)]
    pub const fn sec10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "10-Seconds Value for the tens place of seconds"]
    #[inline(always)]
    pub const fn set_sec10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
    }
    #[doc = "Compare enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enb(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Compare enable"]
    #[inline(always)]
    pub const fn set_enb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Rsecar {
    #[inline(always)]
    fn default() -> Rsecar {
        Rsecar(0)
    }
}
impl core::fmt::Debug for Rsecar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsecar")
            .field("sec1", &self.sec1())
            .field("sec10", &self.sec10())
            .field("enb", &self.enb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsecar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rsecar {{ sec1: {=u8:?}, sec10: {=u8:?}, enb: {=bool:?} }}",
            self.sec1(),
            self.sec10(),
            self.enb()
        )
    }
}
#[doc = "Second Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rseccnt(pub u8);
impl Rseccnt {
    #[doc = "1-Second Count Counts from 0 to 9 every second. When a carry is generated, 1 is added to the tens place."]
    #[must_use]
    #[inline(always)]
    pub const fn sec1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1-Second Count Counts from 0 to 9 every second. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub const fn set_sec1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10-Second Count Counts from 0 to 5 for 60-second counting."]
    #[must_use]
    #[inline(always)]
    pub const fn sec10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "10-Second Count Counts from 0 to 5 for 60-second counting."]
    #[inline(always)]
    pub const fn set_sec10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
    }
}
impl Default for Rseccnt {
    #[inline(always)]
    fn default() -> Rseccnt {
        Rseccnt(0)
    }
}
impl core::fmt::Debug for Rseccnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rseccnt")
            .field("sec1", &self.sec1())
            .field("sec10", &self.sec10())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rseccnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rseccnt {{ sec1: {=u8:?}, sec10: {=u8:?} }}",
            self.sec1(),
            self.sec10()
        )
    }
}
#[doc = "Second Capture Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rseccp(pub u8);
impl Rseccp {
    #[doc = "1-Second Capture Capture value for the ones place of seconds"]
    #[must_use]
    #[inline(always)]
    pub const fn sec1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1-Second Capture Capture value for the ones place of seconds"]
    #[inline(always)]
    pub const fn set_sec1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "10-Second Capture Capture value for the tens place of seconds"]
    #[must_use]
    #[inline(always)]
    pub const fn sec10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "10-Second Capture Capture value for the tens place of seconds"]
    #[inline(always)]
    pub const fn set_sec10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
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
impl Default for Rseccp {
    #[inline(always)]
    fn default() -> Rseccp {
        Rseccp(0)
    }
}
impl core::fmt::Debug for Rseccp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rseccp")
            .field("sec1", &self.sec1())
            .field("sec10", &self.sec10())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rseccp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rseccp {{ sec1: {=u8:?}, sec10: {=u8:?}, reserved: {=bool:?} }}",
            self.sec1(),
            self.sec10(),
            self.reserved()
        )
    }
}
#[doc = "Time Capture Control Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtccr(pub u8);
impl Rtccr {
    #[doc = "Time Capture Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tcct(&self) -> super::vals::Tcct {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Tcct::from_bits(val as u8)
    }
    #[doc = "Time Capture Control"]
    #[inline(always)]
    pub const fn set_tcct(&mut self, val: super::vals::Tcct) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "Time Capture Status"]
    #[must_use]
    #[inline(always)]
    pub const fn tcst(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Time Capture Status"]
    #[inline(always)]
    pub const fn set_tcst(&mut self, val: bool) {
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
    #[doc = "Time Capture Noise Filter Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tcnf(&self) -> super::vals::Tcnf {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Tcnf::from_bits(val as u8)
    }
    #[doc = "Time Capture Noise Filter Control"]
    #[inline(always)]
    pub const fn set_tcnf(&mut self, val: super::vals::Tcnf) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
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
impl Default for Rtccr {
    #[inline(always)]
    fn default() -> Rtccr {
        Rtccr(0)
    }
}
impl core::fmt::Debug for Rtccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rtccr")
            .field("tcct", &self.tcct())
            .field("tcst", &self.tcst())
            .field("reserved", &self.reserved())
            .field("tcnf", &self.tcnf())
            .field("reserved_2", &self.reserved_2())
            .field("reserved_3", &self.reserved_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rtccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rtccr {{ tcct: {:?}, tcst: {=bool:?}, reserved: {=bool:?}, tcnf: {:?}, reserved_2: {=bool:?}, reserved_3: {=bool:?} }}",
            self.tcct(),
            self.tcst(),
            self.reserved(),
            self.tcnf(),
            self.reserved_2(),
            self.reserved_3()
        )
    }
}
#[doc = "Day-of-Week Alarm Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rwkar(pub u8);
impl Rwkar {
    #[doc = "Day-of-Week Counting"]
    #[must_use]
    #[inline(always)]
    pub const fn dayw(&self) -> super::vals::RwkarDayw {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::RwkarDayw::from_bits(val as u8)
    }
    #[doc = "Day-of-Week Counting"]
    #[inline(always)]
    pub const fn set_dayw(&mut self, val: super::vals::RwkarDayw) {
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
    #[doc = "Compare enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enb(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Compare enable"]
    #[inline(always)]
    pub const fn set_enb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Rwkar {
    #[inline(always)]
    fn default() -> Rwkar {
        Rwkar(0)
    }
}
impl core::fmt::Debug for Rwkar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rwkar")
            .field("dayw", &self.dayw())
            .field("reserved", &self.reserved())
            .field("enb", &self.enb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rwkar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rwkar {{ dayw: {:?}, reserved: {=u8:?}, enb: {=bool:?} }}",
            self.dayw(),
            self.reserved(),
            self.enb()
        )
    }
}
#[doc = "Day-of-Week Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rwkcnt(pub u8);
impl Rwkcnt {
    #[doc = "Day-of-Week Counting"]
    #[must_use]
    #[inline(always)]
    pub const fn dayw(&self) -> super::vals::RwkcntDayw {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::RwkcntDayw::from_bits(val as u8)
    }
    #[doc = "Day-of-Week Counting"]
    #[inline(always)]
    pub const fn set_dayw(&mut self, val: super::vals::RwkcntDayw) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u8) & 0x07) << 0usize);
    }
}
impl Default for Rwkcnt {
    #[inline(always)]
    fn default() -> Rwkcnt {
        Rwkcnt(0)
    }
}
impl core::fmt::Debug for Rwkcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rwkcnt")
            .field("dayw", &self.dayw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rwkcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rwkcnt {{ dayw: {:?} }}", self.dayw())
    }
}
#[doc = "Year Alarm Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ryrar(pub u16);
impl Ryrar {
    #[doc = "1 Year Value for the ones place of years"]
    #[must_use]
    #[inline(always)]
    pub const fn yr1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1 Year Value for the ones place of years"]
    #[inline(always)]
    pub const fn set_yr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "10 Years Value for the tens place of years"]
    #[must_use]
    #[inline(always)]
    pub const fn yr10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "10 Years Value for the tens place of years"]
    #[inline(always)]
    pub const fn set_yr10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Ryrar {
    #[inline(always)]
    fn default() -> Ryrar {
        Ryrar(0)
    }
}
impl core::fmt::Debug for Ryrar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ryrar")
            .field("yr1", &self.yr1())
            .field("yr10", &self.yr10())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ryrar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ryrar {{ yr1: {=u8:?}, yr10: {=u8:?}, reserved: {=u8:?} }}",
            self.yr1(),
            self.yr10(),
            self.reserved()
        )
    }
}
#[doc = "Year Alarm Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ryraren(pub u8);
impl Ryraren {
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
    #[doc = "Compare enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enb(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Compare enable"]
    #[inline(always)]
    pub const fn set_enb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Ryraren {
    #[inline(always)]
    fn default() -> Ryraren {
        Ryraren(0)
    }
}
impl core::fmt::Debug for Ryraren {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ryraren")
            .field("reserved", &self.reserved())
            .field("enb", &self.enb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ryraren {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ryraren {{ reserved: {=u8:?}, enb: {=bool:?} }}",
            self.reserved(),
            self.enb()
        )
    }
}
#[doc = "Year Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ryrcnt(pub u16);
impl Ryrcnt {
    #[doc = "1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place."]
    #[must_use]
    #[inline(always)]
    pub const fn yr1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub const fn set_yr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place."]
    #[must_use]
    #[inline(always)]
    pub const fn yr10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place."]
    #[inline(always)]
    pub const fn set_yr10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Ryrcnt {
    #[inline(always)]
    fn default() -> Ryrcnt {
        Ryrcnt(0)
    }
}
impl core::fmt::Debug for Ryrcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ryrcnt")
            .field("yr1", &self.yr1())
            .field("yr10", &self.yr10())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ryrcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ryrcnt {{ yr1: {=u8:?}, yr10: {=u8:?}, reserved: {=u8:?} }}",
            self.yr1(),
            self.yr10(),
            self.reserved()
        )
    }
}
