#[doc = "General PWM Timer Buffer Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtber(pub u32);
impl Gtber {
    #[doc = "BD\\[1\\]: GTPR Buffer Operation Disable BD\\[0\\]: GTCCR Buffer Operation Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn bd(&self) -> super::vals::Bd {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bd::from_bits(val as u8)
    }
    #[doc = "BD\\[1\\]: GTPR Buffer Operation Disable BD\\[0\\]: GTCCR Buffer Operation Disable"]
    #[inline(always)]
    pub const fn set_bd(&mut self, val: super::vals::Bd) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GTCCRA Buffer Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn ccra(&self) -> super::vals::Ccra {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ccra::from_bits(val as u8)
    }
    #[doc = "GTCCRA Buffer Operation"]
    #[inline(always)]
    pub const fn set_ccra(&mut self, val: super::vals::Ccra) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "GTCCRB Buffer Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn ccrb(&self) -> super::vals::Ccrb {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Ccrb::from_bits(val as u8)
    }
    #[doc = "GTCCRB Buffer Operation"]
    #[inline(always)]
    pub const fn set_ccrb(&mut self, val: super::vals::Ccrb) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "GTPR Buffer Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn pr(&self) -> super::vals::Pr {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Pr::from_bits(val as u8)
    }
    #[doc = "GTPR Buffer Operation"]
    #[inline(always)]
    pub const fn set_pr(&mut self, val: super::vals::Pr) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "GTCCRA and GTCCRB Forcible Buffer Operation This bit is read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ccrswt(&self) -> super::vals::Ccrswt {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Ccrswt::from_bits(val as u8)
    }
    #[doc = "GTCCRA and GTCCRB Forcible Buffer Operation This bit is read as 0."]
    #[inline(always)]
    pub const fn set_ccrswt(&mut self, val: super::vals::Ccrswt) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
}
impl Default for Gtber {
    #[inline(always)]
    fn default() -> Gtber {
        Gtber(0)
    }
}
impl core::fmt::Debug for Gtber {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtber")
            .field("bd", &self.bd())
            .field("ccra", &self.ccra())
            .field("ccrb", &self.ccrb())
            .field("pr", &self.pr())
            .field("ccrswt", &self.ccrswt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtber {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtber {{ bd: {:?}, ccra: {:?}, ccrb: {:?}, pr: {:?}, ccrswt: {:?} }}",
            self.bd(),
            self.ccra(),
            self.ccrb(),
            self.pr(),
            self.ccrswt()
        )
    }
}
#[doc = "General PWM Timer Compare Capture Register A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccra(pub u32);
impl Gtccra {
    #[doc = "Compare Capture Register A"]
    #[must_use]
    #[inline(always)]
    pub const fn gtccra(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compare Capture Register A"]
    #[inline(always)]
    pub const fn set_gtccra(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gtccra {
    #[inline(always)]
    fn default() -> Gtccra {
        Gtccra(0)
    }
}
impl core::fmt::Debug for Gtccra {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtccra")
            .field("gtccra", &self.gtccra())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtccra {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gtccra {{ gtccra: {=u32:?} }}", self.gtccra())
    }
}
#[doc = "General PWM Timer Compare Capture Register B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrb(pub u32);
impl Gtccrb {
    #[doc = "Compare Capture Register B"]
    #[must_use]
    #[inline(always)]
    pub const fn gtccrb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compare Capture Register B"]
    #[inline(always)]
    pub const fn set_gtccrb(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gtccrb {
    #[inline(always)]
    fn default() -> Gtccrb {
        Gtccrb(0)
    }
}
impl core::fmt::Debug for Gtccrb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtccrb")
            .field("gtccrb", &self.gtccrb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtccrb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gtccrb {{ gtccrb: {=u32:?} }}", self.gtccrb())
    }
}
#[doc = "General PWM Timer Compare Capture Register C"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrc(pub u32);
impl Gtccrc {
    #[doc = "Compare Capture Register C"]
    #[must_use]
    #[inline(always)]
    pub const fn gtccrc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compare Capture Register C"]
    #[inline(always)]
    pub const fn set_gtccrc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gtccrc {
    #[inline(always)]
    fn default() -> Gtccrc {
        Gtccrc(0)
    }
}
impl core::fmt::Debug for Gtccrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtccrc")
            .field("gtccrc", &self.gtccrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtccrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gtccrc {{ gtccrc: {=u32:?} }}", self.gtccrc())
    }
}
#[doc = "General PWM Timer Compare Capture Register D"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrd(pub u32);
impl Gtccrd {
    #[doc = "Compare Capture Register D"]
    #[must_use]
    #[inline(always)]
    pub const fn gtccrd(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compare Capture Register D"]
    #[inline(always)]
    pub const fn set_gtccrd(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gtccrd {
    #[inline(always)]
    fn default() -> Gtccrd {
        Gtccrd(0)
    }
}
impl core::fmt::Debug for Gtccrd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtccrd")
            .field("gtccrd", &self.gtccrd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtccrd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gtccrd {{ gtccrd: {=u32:?} }}", self.gtccrd())
    }
}
#[doc = "General PWM Timer Compare Capture Register E"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccre(pub u32);
impl Gtccre {
    #[doc = "Compare Capture Register E"]
    #[must_use]
    #[inline(always)]
    pub const fn gtccre(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compare Capture Register E"]
    #[inline(always)]
    pub const fn set_gtccre(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gtccre {
    #[inline(always)]
    fn default() -> Gtccre {
        Gtccre(0)
    }
}
impl core::fmt::Debug for Gtccre {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtccre")
            .field("gtccre", &self.gtccre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtccre {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gtccre {{ gtccre: {=u32:?} }}", self.gtccre())
    }
}
#[doc = "General PWM Timer Compare Capture Register F"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrf(pub u32);
impl Gtccrf {
    #[doc = "Compare Capture Register F"]
    #[must_use]
    #[inline(always)]
    pub const fn gtccrf(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compare Capture Register F"]
    #[inline(always)]
    pub const fn set_gtccrf(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gtccrf {
    #[inline(always)]
    fn default() -> Gtccrf {
        Gtccrf(0)
    }
}
impl core::fmt::Debug for Gtccrf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtccrf")
            .field("gtccrf", &self.gtccrf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtccrf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gtccrf {{ gtccrf: {=u32:?} }}", self.gtccrf())
    }
}
#[doc = "General PWM Timer Software Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtclr(pub u32);
impl Gtclr {
    #[doc = "Channel 0 GTCNT Count Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cclr0(&self) -> super::vals::Cclr0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cclr0::from_bits(val as u8)
    }
    #[doc = "Channel 0 GTCNT Count Clear"]
    #[inline(always)]
    pub const fn set_cclr0(&mut self, val: super::vals::Cclr0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 GTCNT Count Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cclr1(&self) -> super::vals::Cclr1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cclr1::from_bits(val as u8)
    }
    #[doc = "Channel 1 GTCNT Count Clear"]
    #[inline(always)]
    pub const fn set_cclr1(&mut self, val: super::vals::Cclr1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 GTCNT Count Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cclr2(&self) -> super::vals::Cclr2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Cclr2::from_bits(val as u8)
    }
    #[doc = "Channel 2 GTCNT Count Clear"]
    #[inline(always)]
    pub const fn set_cclr2(&mut self, val: super::vals::Cclr2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 GTCNT Count Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cclr3(&self) -> super::vals::Cclr3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cclr3::from_bits(val as u8)
    }
    #[doc = "Channel 3 GTCNT Count Clear"]
    #[inline(always)]
    pub const fn set_cclr3(&mut self, val: super::vals::Cclr3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 GTCNT Count Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cclr4(&self) -> super::vals::Cclr4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Cclr4::from_bits(val as u8)
    }
    #[doc = "Channel 4 GTCNT Count Clear"]
    #[inline(always)]
    pub const fn set_cclr4(&mut self, val: super::vals::Cclr4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 GTCNT Count Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cclr5(&self) -> super::vals::Cclr5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cclr5::from_bits(val as u8)
    }
    #[doc = "Channel 5 GTCNT Count Clear"]
    #[inline(always)]
    pub const fn set_cclr5(&mut self, val: super::vals::Cclr5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 GTCNT Count Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cclr6(&self) -> super::vals::Cclr6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cclr6::from_bits(val as u8)
    }
    #[doc = "Channel 6 GTCNT Count Clear"]
    #[inline(always)]
    pub const fn set_cclr6(&mut self, val: super::vals::Cclr6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 GTCNT Count Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cclr7(&self) -> super::vals::Cclr7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cclr7::from_bits(val as u8)
    }
    #[doc = "Channel 7 GTCNT Count Clear"]
    #[inline(always)]
    pub const fn set_cclr7(&mut self, val: super::vals::Cclr7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "The write value should be 000000000000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "The write value should be 000000000000000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Gtclr {
    #[inline(always)]
    fn default() -> Gtclr {
        Gtclr(0)
    }
}
impl core::fmt::Debug for Gtclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtclr")
            .field("cclr0", &self.cclr0())
            .field("cclr1", &self.cclr1())
            .field("cclr2", &self.cclr2())
            .field("cclr3", &self.cclr3())
            .field("cclr4", &self.cclr4())
            .field("cclr5", &self.cclr5())
            .field("cclr6", &self.cclr6())
            .field("cclr7", &self.cclr7())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtclr {{ cclr0: {:?}, cclr1: {:?}, cclr2: {:?}, cclr3: {:?}, cclr4: {:?}, cclr5: {:?}, cclr6: {:?}, cclr7: {:?}, reserved: {=u32:?} }}",
            self.cclr0(),
            self.cclr1(),
            self.cclr2(),
            self.cclr3(),
            self.cclr4(),
            self.cclr5(),
            self.cclr6(),
            self.cclr7(),
            self.reserved()
        )
    }
}
#[doc = "General PWM Timer Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtcnt(pub u32);
impl Gtcnt {
    #[doc = "Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn gtcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Counter"]
    #[inline(always)]
    pub const fn set_gtcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gtcnt {
    #[inline(always)]
    fn default() -> Gtcnt {
        Gtcnt(0)
    }
}
impl core::fmt::Debug for Gtcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtcnt")
            .field("gtcnt", &self.gtcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gtcnt {{ gtcnt: {=u32:?} }}", self.gtcnt())
    }
}
#[doc = "General PWM Timer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtcr(pub u32);
impl Gtcr {
    #[doc = "Count Start"]
    #[must_use]
    #[inline(always)]
    pub const fn cst(&self) -> super::vals::Cst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cst::from_bits(val as u8)
    }
    #[doc = "Count Start"]
    #[inline(always)]
    pub const fn set_cst(&mut self, val: super::vals::Cst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
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
        self.0 = (self.0 & !(0x7fff << 1usize)) | (((val as u32) & 0x7fff) << 1usize);
    }
    #[doc = "Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn md(&self) -> super::vals::Mode {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Mode Select"]
    #[inline(always)]
    pub const fn set_md(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
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
    #[doc = "Timer Prescaler Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tpcs(&self) -> super::vals::Tpcs {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Tpcs::from_bits(val as u8)
    }
    #[doc = "Timer Prescaler Select"]
    #[inline(always)]
    pub const fn set_tpcs(&mut self, val: super::vals::Tpcs) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Gtcr {
    #[inline(always)]
    fn default() -> Gtcr {
        Gtcr(0)
    }
}
impl core::fmt::Debug for Gtcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtcr")
            .field("cst", &self.cst())
            .field("reserved", &self.reserved())
            .field("md", &self.md())
            .field("reserved_2", &self.reserved_2())
            .field("tpcs", &self.tpcs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtcr {{ cst: {:?}, reserved: {=u16:?}, md: {:?}, reserved_2: {=u8:?}, tpcs: {:?} }}",
            self.cst(),
            self.reserved(),
            self.md(),
            self.reserved_2(),
            self.tpcs()
        )
    }
}
#[doc = "General PWM Timer Clear Source Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtcsr(pub u32);
impl Gtcsr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn csgtrgar(&self) -> super::vals::Csgtrgar {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Csgtrgar::from_bits(val as u8)
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_csgtrgar(&mut self, val: super::vals::Csgtrgar) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn csgtrgaf(&self) -> super::vals::Csgtrgaf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Csgtrgaf::from_bits(val as u8)
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_csgtrgaf(&mut self, val: super::vals::Csgtrgaf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn csgtrgbr(&self) -> super::vals::Csgtrgbr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Csgtrgbr::from_bits(val as u8)
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_csgtrgbr(&mut self, val: super::vals::Csgtrgbr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn csgtrgbf(&self) -> super::vals::Csgtrgbf {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Csgtrgbf::from_bits(val as u8)
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_csgtrgbf(&mut self, val: super::vals::Csgtrgbf) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
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
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscarbl(&self) -> super::vals::Cscarbl {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cscarbl::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscarbl(&mut self, val: super::vals::Cscarbl) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscarbh(&self) -> super::vals::Cscarbh {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Cscarbh::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscarbh(&mut self, val: super::vals::Cscarbh) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscafbl(&self) -> super::vals::Cscafbl {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Cscafbl::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscafbl(&mut self, val: super::vals::Cscafbl) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscafbh(&self) -> super::vals::Cscafbh {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Cscafbh::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscafbh(&mut self, val: super::vals::Cscafbh) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscbral(&self) -> super::vals::Cscbral {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Cscbral::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscbral(&mut self, val: super::vals::Cscbral) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscbrah(&self) -> super::vals::Cscbrah {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cscbrah::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscbrah(&mut self, val: super::vals::Cscbrah) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscbfal(&self) -> super::vals::Cscbfal {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Cscbfal::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscbfal(&mut self, val: super::vals::Cscbfal) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cscbfah(&self) -> super::vals::Cscbfah {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Cscbfah::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cscbfah(&mut self, val: super::vals::Cscbfah) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ELC_GPTA Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselca(&self) -> super::vals::Cselca {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Cselca::from_bits(val as u8)
    }
    #[doc = "ELC_GPTA Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselca(&mut self, val: super::vals::Cselca) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "ELC_GPTB Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselcb(&self) -> super::vals::Cselcb {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Cselcb::from_bits(val as u8)
    }
    #[doc = "ELC_GPTB Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselcb(&mut self, val: super::vals::Cselcb) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "ELC_GPTC Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselcc(&self) -> super::vals::Cselcc {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Cselcc::from_bits(val as u8)
    }
    #[doc = "ELC_GPTC Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselcc(&mut self, val: super::vals::Cselcc) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "ELC_GPTD Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselcd(&self) -> super::vals::Cselcd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Cselcd::from_bits(val as u8)
    }
    #[doc = "ELC_GPTD Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselcd(&mut self, val: super::vals::Cselcd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "ELC_GPTE Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselce(&self) -> super::vals::Cselce {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Cselce::from_bits(val as u8)
    }
    #[doc = "ELC_GPTE Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselce(&mut self, val: super::vals::Cselce) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "ELC_GPTF Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselcf(&self) -> super::vals::Cselcf {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Cselcf::from_bits(val as u8)
    }
    #[doc = "ELC_GPTF Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselcf(&mut self, val: super::vals::Cselcf) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "ELC_GPTG Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselcg(&self) -> super::vals::Cselcg {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Cselcg::from_bits(val as u8)
    }
    #[doc = "ELC_GPTG Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselcg(&mut self, val: super::vals::Cselcg) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "ELCH Event Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cselch(&self) -> super::vals::Cselch {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Cselch::from_bits(val as u8)
    }
    #[doc = "ELCH Event Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cselch(&mut self, val: super::vals::Cselch) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "Software Source Counter Clear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cclr(&self) -> super::vals::Cclr {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Cclr::from_bits(val as u8)
    }
    #[doc = "Software Source Counter Clear Enable"]
    #[inline(always)]
    pub const fn set_cclr(&mut self, val: super::vals::Cclr) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gtcsr {
    #[inline(always)]
    fn default() -> Gtcsr {
        Gtcsr(0)
    }
}
impl core::fmt::Debug for Gtcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtcsr")
            .field("csgtrgar", &self.csgtrgar())
            .field("csgtrgaf", &self.csgtrgaf())
            .field("csgtrgbr", &self.csgtrgbr())
            .field("csgtrgbf", &self.csgtrgbf())
            .field("reserved", &self.reserved())
            .field("cscarbl", &self.cscarbl())
            .field("cscarbh", &self.cscarbh())
            .field("cscafbl", &self.cscafbl())
            .field("cscafbh", &self.cscafbh())
            .field("cscbral", &self.cscbral())
            .field("cscbrah", &self.cscbrah())
            .field("cscbfal", &self.cscbfal())
            .field("cscbfah", &self.cscbfah())
            .field("cselca", &self.cselca())
            .field("cselcb", &self.cselcb())
            .field("cselcc", &self.cselcc())
            .field("cselcd", &self.cselcd())
            .field("cselce", &self.cselce())
            .field("cselcf", &self.cselcf())
            .field("cselcg", &self.cselcg())
            .field("cselch", &self.cselch())
            .field("reserved_2", &self.reserved_2())
            .field("cclr", &self.cclr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtcsr {{ csgtrgar: {:?}, csgtrgaf: {:?}, csgtrgbr: {:?}, csgtrgbf: {:?}, reserved: {=u8:?}, cscarbl: {:?}, cscarbh: {:?}, cscafbl: {:?}, cscafbh: {:?}, cscbral: {:?}, cscbrah: {:?}, cscbfal: {:?}, cscbfah: {:?}, cselca: {:?}, cselcb: {:?}, cselcc: {:?}, cselcd: {:?}, cselce: {:?}, cselcf: {:?}, cselcg: {:?}, cselch: {:?}, reserved_2: {=u8:?}, cclr: {:?} }}",
            self.csgtrgar(),
            self.csgtrgaf(),
            self.csgtrgbr(),
            self.csgtrgbf(),
            self.reserved(),
            self.cscarbl(),
            self.cscarbh(),
            self.cscafbl(),
            self.cscafbh(),
            self.cscbral(),
            self.cscbrah(),
            self.cscbfal(),
            self.cscbfah(),
            self.cselca(),
            self.cselcb(),
            self.cselcc(),
            self.cselcd(),
            self.cselce(),
            self.cselcf(),
            self.cselcg(),
            self.cselch(),
            self.reserved_2(),
            self.cclr()
        )
    }
}
#[doc = "General PWM Timer Down Count Source Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdnsr(pub u32);
impl Gtdnsr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dsgtrgar(&self) -> super::vals::Dsgtrgar {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dsgtrgar::from_bits(val as u8)
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dsgtrgar(&mut self, val: super::vals::Dsgtrgar) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dsgtrgaf(&self) -> super::vals::Dsgtrgaf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dsgtrgaf::from_bits(val as u8)
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dsgtrgaf(&mut self, val: super::vals::Dsgtrgaf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dsgtrgbr(&self) -> super::vals::Dsgtrgbr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dsgtrgbr::from_bits(val as u8)
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dsgtrgbr(&mut self, val: super::vals::Dsgtrgbr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dsgtrgbf(&self) -> super::vals::Dsgtrgbf {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dsgtrgbf::from_bits(val as u8)
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dsgtrgbf(&mut self, val: super::vals::Dsgtrgbf) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscarbl(&self) -> super::vals::Dscarbl {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dscarbl::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscarbl(&mut self, val: super::vals::Dscarbl) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscarbh(&self) -> super::vals::Dscarbh {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dscarbh::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscarbh(&mut self, val: super::vals::Dscarbh) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscafbl(&self) -> super::vals::Dscafbl {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dscafbl::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscafbl(&mut self, val: super::vals::Dscafbl) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscafbh(&self) -> super::vals::Dscafbh {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dscafbh::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscafbh(&mut self, val: super::vals::Dscafbh) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscbral(&self) -> super::vals::Dscbral {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dscbral::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscbral(&mut self, val: super::vals::Dscbral) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscbrah(&self) -> super::vals::Dscbrah {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dscbrah::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscbrah(&mut self, val: super::vals::Dscbrah) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscbfal(&self) -> super::vals::Dscbfal {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Dscbfal::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscbfal(&mut self, val: super::vals::Dscbfal) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dscbfah(&self) -> super::vals::Dscbfah {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dscbfah::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dscbfah(&mut self, val: super::vals::Dscbfah) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ELC_GPTA Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselca(&self) -> super::vals::Dselca {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dselca::from_bits(val as u8)
    }
    #[doc = "ELC_GPTA Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselca(&mut self, val: super::vals::Dselca) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "ELC_GPTB Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselcb(&self) -> super::vals::Dselcb {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dselcb::from_bits(val as u8)
    }
    #[doc = "ELC_GPTB Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselcb(&mut self, val: super::vals::Dselcb) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "ELC_GPTC Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselcc(&self) -> super::vals::Dselcc {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dselcc::from_bits(val as u8)
    }
    #[doc = "ELC_GPTC Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselcc(&mut self, val: super::vals::Dselcc) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "ELC_GPTD Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselcd(&self) -> super::vals::Dselcd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dselcd::from_bits(val as u8)
    }
    #[doc = "ELC_GPTD Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselcd(&mut self, val: super::vals::Dselcd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "ELC_GPTE Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselce(&self) -> super::vals::Dselce {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dselce::from_bits(val as u8)
    }
    #[doc = "ELC_GPTE Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselce(&mut self, val: super::vals::Dselce) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "ELC_GPTF Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselcf(&self) -> super::vals::Dselcf {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dselcf::from_bits(val as u8)
    }
    #[doc = "ELC_GPTF Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselcf(&mut self, val: super::vals::Dselcf) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "ELC_GPTG Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselcg(&self) -> super::vals::Dselcg {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dselcg::from_bits(val as u8)
    }
    #[doc = "ELC_GPTG Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselcg(&mut self, val: super::vals::Dselcg) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "ELCH Event Source Counter Count Down Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dselch(&self) -> super::vals::Dselch {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dselch::from_bits(val as u8)
    }
    #[doc = "ELCH Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub const fn set_dselch(&mut self, val: super::vals::Dselch) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Gtdnsr {
    #[inline(always)]
    fn default() -> Gtdnsr {
        Gtdnsr(0)
    }
}
impl core::fmt::Debug for Gtdnsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtdnsr")
            .field("dsgtrgar", &self.dsgtrgar())
            .field("dsgtrgaf", &self.dsgtrgaf())
            .field("dsgtrgbr", &self.dsgtrgbr())
            .field("dsgtrgbf", &self.dsgtrgbf())
            .field("dscarbl", &self.dscarbl())
            .field("dscarbh", &self.dscarbh())
            .field("dscafbl", &self.dscafbl())
            .field("dscafbh", &self.dscafbh())
            .field("dscbral", &self.dscbral())
            .field("dscbrah", &self.dscbrah())
            .field("dscbfal", &self.dscbfal())
            .field("dscbfah", &self.dscbfah())
            .field("dselca", &self.dselca())
            .field("dselcb", &self.dselcb())
            .field("dselcc", &self.dselcc())
            .field("dselcd", &self.dselcd())
            .field("dselce", &self.dselce())
            .field("dselcf", &self.dselcf())
            .field("dselcg", &self.dselcg())
            .field("dselch", &self.dselch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtdnsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtdnsr {{ dsgtrgar: {:?}, dsgtrgaf: {:?}, dsgtrgbr: {:?}, dsgtrgbf: {:?}, dscarbl: {:?}, dscarbh: {:?}, dscafbl: {:?}, dscafbh: {:?}, dscbral: {:?}, dscbrah: {:?}, dscbfal: {:?}, dscbfah: {:?}, dselca: {:?}, dselcb: {:?}, dselcc: {:?}, dselcd: {:?}, dselce: {:?}, dselcf: {:?}, dselcg: {:?}, dselch: {:?} }}",
            self.dsgtrgar(),
            self.dsgtrgaf(),
            self.dsgtrgbr(),
            self.dsgtrgbf(),
            self.dscarbl(),
            self.dscarbh(),
            self.dscafbl(),
            self.dscafbh(),
            self.dscbral(),
            self.dscbrah(),
            self.dscbfal(),
            self.dscbfah(),
            self.dselca(),
            self.dselcb(),
            self.dselcc(),
            self.dselcd(),
            self.dselce(),
            self.dselcf(),
            self.dselcg(),
            self.dselch()
        )
    }
}
#[doc = "General PWM Timer Dead Time Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdtcr(pub u32);
impl Gtdtcr {
    #[doc = "Negative-Phase Waveform Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn tde(&self) -> super::vals::Tde {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tde::from_bits(val as u8)
    }
    #[doc = "Negative-Phase Waveform Setting"]
    #[inline(always)]
    pub const fn set_tde(&mut self, val: super::vals::Tde) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
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
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
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
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_5(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "These bits are read as 00000000000000000000000. The write value should be 00000000000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_6(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "These bits are read as 00000000000000000000000. The write value should be 00000000000000000000000."]
    #[inline(always)]
    pub const fn set_reserved_6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for Gtdtcr {
    #[inline(always)]
    fn default() -> Gtdtcr {
        Gtdtcr(0)
    }
}
impl core::fmt::Debug for Gtdtcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtdtcr")
            .field("tde", &self.tde())
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
impl defmt::Format for Gtdtcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtdtcr {{ tde: {:?}, reserved: {=u8:?}, reserved_2: {=bool:?}, reserved_3: {=bool:?}, reserved_4: {=u8:?}, reserved_5: {=bool:?}, reserved_6: {=u32:?} }}",
            self.tde(),
            self.reserved(),
            self.reserved_2(),
            self.reserved_3(),
            self.reserved_4(),
            self.reserved_5(),
            self.reserved_6()
        )
    }
}
#[doc = "General PWM Timer Dead Time Value Register U"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdvu(pub u32);
impl Gtdvu {
    #[doc = "Dead Time Value Register U"]
    #[must_use]
    #[inline(always)]
    pub const fn gtdvu(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Dead Time Value Register U"]
    #[inline(always)]
    pub const fn set_gtdvu(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gtdvu {
    #[inline(always)]
    fn default() -> Gtdvu {
        Gtdvu(0)
    }
}
impl core::fmt::Debug for Gtdvu {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtdvu")
            .field("gtdvu", &self.gtdvu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtdvu {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gtdvu {{ gtdvu: {=u32:?} }}", self.gtdvu())
    }
}
#[doc = "General PWM Timer Input Capture Source Select Register A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gticasr(pub u32);
impl Gticasr {
    #[doc = "GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asgtrgar(&self) -> super::vals::Asgtrgar {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Asgtrgar::from_bits(val as u8)
    }
    #[doc = "GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_asgtrgar(&mut self, val: super::vals::Asgtrgar) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asgtrgaf(&self) -> super::vals::Asgtrgaf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Asgtrgaf::from_bits(val as u8)
    }
    #[doc = "GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_asgtrgaf(&mut self, val: super::vals::Asgtrgaf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asgtrgbr(&self) -> super::vals::Asgtrgbr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Asgtrgbr::from_bits(val as u8)
    }
    #[doc = "GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_asgtrgbr(&mut self, val: super::vals::Asgtrgbr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asgtrgbf(&self) -> super::vals::Asgtrgbf {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Asgtrgbf::from_bits(val as u8)
    }
    #[doc = "GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_asgtrgbf(&mut self, val: super::vals::Asgtrgbf) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascarbl(&self) -> super::vals::Ascarbl {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ascarbl::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascarbl(&mut self, val: super::vals::Ascarbl) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascarbh(&self) -> super::vals::Ascarbh {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ascarbh::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascarbh(&mut self, val: super::vals::Ascarbh) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascafbl(&self) -> super::vals::Ascafbl {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Ascafbl::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascafbl(&mut self, val: super::vals::Ascafbl) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascafbh(&self) -> super::vals::Ascafbh {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Ascafbh::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascafbh(&mut self, val: super::vals::Ascafbh) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascbral(&self) -> super::vals::Ascbral {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Ascbral::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascbral(&mut self, val: super::vals::Ascbral) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascbrah(&self) -> super::vals::Ascbrah {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Ascbrah::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascbrah(&mut self, val: super::vals::Ascbrah) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascbfal(&self) -> super::vals::Ascbfal {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Ascbfal::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascbfal(&mut self, val: super::vals::Ascbfal) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ascbfah(&self) -> super::vals::Ascbfah {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Ascbfah::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_ascbfah(&mut self, val: super::vals::Ascbfah) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ELC_GPTA Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselca(&self) -> super::vals::Aselca {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Aselca::from_bits(val as u8)
    }
    #[doc = "ELC_GPTA Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselca(&mut self, val: super::vals::Aselca) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "ELC_GPTB Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselcb(&self) -> super::vals::Aselcb {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Aselcb::from_bits(val as u8)
    }
    #[doc = "ELC_GPTB Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselcb(&mut self, val: super::vals::Aselcb) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "ELC_GPTC Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselcc(&self) -> super::vals::Aselcc {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Aselcc::from_bits(val as u8)
    }
    #[doc = "ELC_GPTC Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselcc(&mut self, val: super::vals::Aselcc) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "ELC_GPTD Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselcd(&self) -> super::vals::Aselcd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Aselcd::from_bits(val as u8)
    }
    #[doc = "ELC_GPTD Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselcd(&mut self, val: super::vals::Aselcd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "ELC_GPTE Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselce(&self) -> super::vals::Aselce {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Aselce::from_bits(val as u8)
    }
    #[doc = "ELC_GPTE Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselce(&mut self, val: super::vals::Aselce) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "ELC_GPTF Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselcf(&self) -> super::vals::Aselcf {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Aselcf::from_bits(val as u8)
    }
    #[doc = "ELC_GPTF Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselcf(&mut self, val: super::vals::Aselcf) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "ELC_GPTG Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselcg(&self) -> super::vals::Aselcg {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Aselcg::from_bits(val as u8)
    }
    #[doc = "ELC_GPTG Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselcg(&mut self, val: super::vals::Aselcg) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "ELCH Event Source GTCCRA Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aselch(&self) -> super::vals::Aselch {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Aselch::from_bits(val as u8)
    }
    #[doc = "ELCH Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub const fn set_aselch(&mut self, val: super::vals::Aselch) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Gticasr {
    #[inline(always)]
    fn default() -> Gticasr {
        Gticasr(0)
    }
}
impl core::fmt::Debug for Gticasr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gticasr")
            .field("asgtrgar", &self.asgtrgar())
            .field("asgtrgaf", &self.asgtrgaf())
            .field("asgtrgbr", &self.asgtrgbr())
            .field("asgtrgbf", &self.asgtrgbf())
            .field("ascarbl", &self.ascarbl())
            .field("ascarbh", &self.ascarbh())
            .field("ascafbl", &self.ascafbl())
            .field("ascafbh", &self.ascafbh())
            .field("ascbral", &self.ascbral())
            .field("ascbrah", &self.ascbrah())
            .field("ascbfal", &self.ascbfal())
            .field("ascbfah", &self.ascbfah())
            .field("aselca", &self.aselca())
            .field("aselcb", &self.aselcb())
            .field("aselcc", &self.aselcc())
            .field("aselcd", &self.aselcd())
            .field("aselce", &self.aselce())
            .field("aselcf", &self.aselcf())
            .field("aselcg", &self.aselcg())
            .field("aselch", &self.aselch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gticasr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gticasr {{ asgtrgar: {:?}, asgtrgaf: {:?}, asgtrgbr: {:?}, asgtrgbf: {:?}, ascarbl: {:?}, ascarbh: {:?}, ascafbl: {:?}, ascafbh: {:?}, ascbral: {:?}, ascbrah: {:?}, ascbfal: {:?}, ascbfah: {:?}, aselca: {:?}, aselcb: {:?}, aselcc: {:?}, aselcd: {:?}, aselce: {:?}, aselcf: {:?}, aselcg: {:?}, aselch: {:?} }}",
            self.asgtrgar(),
            self.asgtrgaf(),
            self.asgtrgbr(),
            self.asgtrgbf(),
            self.ascarbl(),
            self.ascarbh(),
            self.ascafbl(),
            self.ascafbh(),
            self.ascbral(),
            self.ascbrah(),
            self.ascbfal(),
            self.ascbfah(),
            self.aselca(),
            self.aselcb(),
            self.aselcc(),
            self.aselcd(),
            self.aselce(),
            self.aselcf(),
            self.aselcg(),
            self.aselch()
        )
    }
}
#[doc = "General PWM Timer Input Capture Source Select Register B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gticbsr(pub u32);
impl Gticbsr {
    #[doc = "GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bsgtrgar(&self) -> super::vals::Bsgtrgar {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Bsgtrgar::from_bits(val as u8)
    }
    #[doc = "GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bsgtrgar(&mut self, val: super::vals::Bsgtrgar) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bsgtrgaf(&self) -> super::vals::Bsgtrgaf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Bsgtrgaf::from_bits(val as u8)
    }
    #[doc = "GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bsgtrgaf(&mut self, val: super::vals::Bsgtrgaf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bsgtrgbr(&self) -> super::vals::Bsgtrgbr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Bsgtrgbr::from_bits(val as u8)
    }
    #[doc = "GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bsgtrgbr(&mut self, val: super::vals::Bsgtrgbr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bsgtrgbf(&self) -> super::vals::Bsgtrgbf {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Bsgtrgbf::from_bits(val as u8)
    }
    #[doc = "GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bsgtrgbf(&mut self, val: super::vals::Bsgtrgbf) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscarbl(&self) -> super::vals::Bscarbl {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Bscarbl::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscarbl(&mut self, val: super::vals::Bscarbl) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscarbh(&self) -> super::vals::Bscarbh {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Bscarbh::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscarbh(&mut self, val: super::vals::Bscarbh) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscafbl(&self) -> super::vals::Bscafbl {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Bscafbl::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscafbl(&mut self, val: super::vals::Bscafbl) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscafbh(&self) -> super::vals::Bscafbh {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Bscafbh::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscafbh(&mut self, val: super::vals::Bscafbh) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscbral(&self) -> super::vals::Bscbral {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Bscbral::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscbral(&mut self, val: super::vals::Bscbral) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscbrah(&self) -> super::vals::Bscbrah {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Bscbrah::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscbrah(&mut self, val: super::vals::Bscbrah) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscbfal(&self) -> super::vals::Bscbfal {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Bscbfal::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscbfal(&mut self, val: super::vals::Bscbfal) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bscbfah(&self) -> super::vals::Bscbfah {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Bscbfah::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bscbfah(&mut self, val: super::vals::Bscbfah) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ELC_GPTA Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselca(&self) -> super::vals::Bselca {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Bselca::from_bits(val as u8)
    }
    #[doc = "ELC_GPTA Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselca(&mut self, val: super::vals::Bselca) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "ELC_GPTB Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselcb(&self) -> super::vals::Bselcb {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Bselcb::from_bits(val as u8)
    }
    #[doc = "ELC_GPTB Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselcb(&mut self, val: super::vals::Bselcb) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "ELC_GPTC Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselcc(&self) -> super::vals::Bselcc {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Bselcc::from_bits(val as u8)
    }
    #[doc = "ELC_GPTC Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselcc(&mut self, val: super::vals::Bselcc) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "ELC_GPTD Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselcd(&self) -> super::vals::Bselcd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Bselcd::from_bits(val as u8)
    }
    #[doc = "ELC_GPTD Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselcd(&mut self, val: super::vals::Bselcd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "ELC_GPTE Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselce(&self) -> super::vals::Bselce {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Bselce::from_bits(val as u8)
    }
    #[doc = "ELC_GPTE Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselce(&mut self, val: super::vals::Bselce) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "ELC_GPTF Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselcf(&self) -> super::vals::Bselcf {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Bselcf::from_bits(val as u8)
    }
    #[doc = "ELC_GPTF Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselcf(&mut self, val: super::vals::Bselcf) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "ELC_GPTG Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselcg(&self) -> super::vals::Bselcg {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Bselcg::from_bits(val as u8)
    }
    #[doc = "ELC_GPTG Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselcg(&mut self, val: super::vals::Bselcg) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "ELCH Event Source GTCCRB Input Capture Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bselch(&self) -> super::vals::Bselch {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Bselch::from_bits(val as u8)
    }
    #[doc = "ELCH Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub const fn set_bselch(&mut self, val: super::vals::Bselch) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Gticbsr {
    #[inline(always)]
    fn default() -> Gticbsr {
        Gticbsr(0)
    }
}
impl core::fmt::Debug for Gticbsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gticbsr")
            .field("bsgtrgar", &self.bsgtrgar())
            .field("bsgtrgaf", &self.bsgtrgaf())
            .field("bsgtrgbr", &self.bsgtrgbr())
            .field("bsgtrgbf", &self.bsgtrgbf())
            .field("bscarbl", &self.bscarbl())
            .field("bscarbh", &self.bscarbh())
            .field("bscafbl", &self.bscafbl())
            .field("bscafbh", &self.bscafbh())
            .field("bscbral", &self.bscbral())
            .field("bscbrah", &self.bscbrah())
            .field("bscbfal", &self.bscbfal())
            .field("bscbfah", &self.bscbfah())
            .field("bselca", &self.bselca())
            .field("bselcb", &self.bselcb())
            .field("bselcc", &self.bselcc())
            .field("bselcd", &self.bselcd())
            .field("bselce", &self.bselce())
            .field("bselcf", &self.bselcf())
            .field("bselcg", &self.bselcg())
            .field("bselch", &self.bselch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gticbsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gticbsr {{ bsgtrgar: {:?}, bsgtrgaf: {:?}, bsgtrgbr: {:?}, bsgtrgbf: {:?}, bscarbl: {:?}, bscarbh: {:?}, bscafbl: {:?}, bscafbh: {:?}, bscbral: {:?}, bscbrah: {:?}, bscbfal: {:?}, bscbfah: {:?}, bselca: {:?}, bselcb: {:?}, bselcc: {:?}, bselcd: {:?}, bselce: {:?}, bselcf: {:?}, bselcg: {:?}, bselch: {:?} }}",
            self.bsgtrgar(),
            self.bsgtrgaf(),
            self.bsgtrgbr(),
            self.bsgtrgbf(),
            self.bscarbl(),
            self.bscarbh(),
            self.bscafbl(),
            self.bscafbh(),
            self.bscbral(),
            self.bscbrah(),
            self.bscbfal(),
            self.bscbfah(),
            self.bselca(),
            self.bselcb(),
            self.bselcc(),
            self.bselcd(),
            self.bselce(),
            self.bselcf(),
            self.bselcg(),
            self.bselch()
        )
    }
}
#[doc = "General PWM Timer Interrupt Output Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtintad(pub u32);
impl Gtintad {
    #[doc = "These bits are read as 000000000000000000000000. The write value should be 000000000000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "These bits are read as 000000000000000000000000. The write value should be 000000000000000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Output Disable Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn grp(&self) -> super::vals::Grp {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Grp::from_bits(val as u8)
    }
    #[doc = "Output Disable Source Select"]
    #[inline(always)]
    pub const fn set_grp(&mut self, val: super::vals::Grp) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "Same Time Output Level High Disable Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn grpabh(&self) -> super::vals::Grpabh {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Grpabh::from_bits(val as u8)
    }
    #[doc = "Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub const fn set_grpabh(&mut self, val: super::vals::Grpabh) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Same Time Output Level Low Disable Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn grpabl(&self) -> super::vals::Grpabl {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Grpabl::from_bits(val as u8)
    }
    #[doc = "Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub const fn set_grpabl(&mut self, val: super::vals::Grpabl) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Gtintad {
    #[inline(always)]
    fn default() -> Gtintad {
        Gtintad(0)
    }
}
impl core::fmt::Debug for Gtintad {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtintad")
            .field("reserved", &self.reserved())
            .field("grp", &self.grp())
            .field("reserved_2", &self.reserved_2())
            .field("grpabh", &self.grpabh())
            .field("grpabl", &self.grpabl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtintad {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtintad {{ reserved: {=u32:?}, grp: {:?}, reserved_2: {=u8:?}, grpabh: {:?}, grpabl: {:?} }}",
            self.reserved(),
            self.grp(),
            self.reserved_2(),
            self.grpabh(),
            self.grpabl()
        )
    }
}
#[doc = "General PWM Timer I/O Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtior(pub u32);
impl Gtior {
    #[doc = "GTIOCA Pin Function Select"]
    #[must_use]
    #[inline(always)]
    pub const fn gtioa(&self) -> super::vals::Gtioa {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Gtioa::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Function Select"]
    #[inline(always)]
    pub const fn set_gtioa(&mut self, val: super::vals::Gtioa) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
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
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GTIOCA Pin Output Value Setting at the Count Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn oadflt(&self) -> super::vals::Oadflt {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Oadflt::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub const fn set_oadflt(&mut self, val: super::vals::Oadflt) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "GTIOCA Pin Output Setting at the Start/Stop Count"]
    #[must_use]
    #[inline(always)]
    pub const fn oahld(&self) -> super::vals::Oahld {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Oahld::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub const fn set_oahld(&mut self, val: super::vals::Oahld) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "GTIOCA Pin Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oae(&self) -> super::vals::Oae {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Oae::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Output Enable"]
    #[inline(always)]
    pub const fn set_oae(&mut self, val: super::vals::Oae) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Disable Value Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn oadf(&self) -> super::vals::Oadf {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Oadf::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Disable Value Setting"]
    #[inline(always)]
    pub const fn set_oadf(&mut self, val: super::vals::Oadf) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "Noise Filter A Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nfaen(&self) -> super::vals::Nfaen {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Nfaen::from_bits(val as u8)
    }
    #[doc = "Noise Filter A Enable"]
    #[inline(always)]
    pub const fn set_nfaen(&mut self, val: super::vals::Nfaen) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Noise Filter A Sampling Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn nfcsa(&self) -> super::vals::Nfcsa {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Nfcsa::from_bits(val as u8)
    }
    #[doc = "Noise Filter A Sampling Clock Select"]
    #[inline(always)]
    pub const fn set_nfcsa(&mut self, val: super::vals::Nfcsa) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "GTIOCB Pin Function Select"]
    #[must_use]
    #[inline(always)]
    pub const fn gtiob(&self) -> super::vals::Gtiob {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Gtiob::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Function Select"]
    #[inline(always)]
    pub const fn set_gtiob(&mut self, val: super::vals::Gtiob) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "GTIOCB Pin Output Value Setting at the Count Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn obdflt(&self) -> super::vals::Obdflt {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Obdflt::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub const fn set_obdflt(&mut self, val: super::vals::Obdflt) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "GTIOCB Pin Output Setting at the Start/Stop Count"]
    #[must_use]
    #[inline(always)]
    pub const fn obhld(&self) -> super::vals::Obhld {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Obhld::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub const fn set_obhld(&mut self, val: super::vals::Obhld) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "GTIOCB Pin Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn obe(&self) -> super::vals::Obe {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Obe::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Output Enable"]
    #[inline(always)]
    pub const fn set_obe(&mut self, val: super::vals::Obe) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "GTIOCB Pin Disable Value Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn obdf(&self) -> super::vals::Obdf {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::Obdf::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Disable Value Setting"]
    #[inline(always)]
    pub const fn set_obdf(&mut self, val: super::vals::Obdf) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_4(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub const fn set_reserved_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "Noise Filter B Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nfben(&self) -> super::vals::Nfben {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Nfben::from_bits(val as u8)
    }
    #[doc = "Noise Filter B Enable"]
    #[inline(always)]
    pub const fn set_nfben(&mut self, val: super::vals::Nfben) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Noise Filter B Sampling Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn nfcsb(&self) -> super::vals::Nfcsb {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Nfcsb::from_bits(val as u8)
    }
    #[doc = "Noise Filter B Sampling Clock Select"]
    #[inline(always)]
    pub const fn set_nfcsb(&mut self, val: super::vals::Nfcsb) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Gtior {
    #[inline(always)]
    fn default() -> Gtior {
        Gtior(0)
    }
}
impl core::fmt::Debug for Gtior {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtior")
            .field("gtioa", &self.gtioa())
            .field("reserved", &self.reserved())
            .field("oadflt", &self.oadflt())
            .field("oahld", &self.oahld())
            .field("oae", &self.oae())
            .field("oadf", &self.oadf())
            .field("reserved_2", &self.reserved_2())
            .field("nfaen", &self.nfaen())
            .field("nfcsa", &self.nfcsa())
            .field("gtiob", &self.gtiob())
            .field("reserved_3", &self.reserved_3())
            .field("obdflt", &self.obdflt())
            .field("obhld", &self.obhld())
            .field("obe", &self.obe())
            .field("obdf", &self.obdf())
            .field("reserved_4", &self.reserved_4())
            .field("nfben", &self.nfben())
            .field("nfcsb", &self.nfcsb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtior {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtior {{ gtioa: {:?}, reserved: {=bool:?}, oadflt: {:?}, oahld: {:?}, oae: {:?}, oadf: {:?}, reserved_2: {=u8:?}, nfaen: {:?}, nfcsa: {:?}, gtiob: {:?}, reserved_3: {=bool:?}, obdflt: {:?}, obhld: {:?}, obe: {:?}, obdf: {:?}, reserved_4: {=u8:?}, nfben: {:?}, nfcsb: {:?} }}",
            self.gtioa(),
            self.reserved(),
            self.oadflt(),
            self.oahld(),
            self.oae(),
            self.oadf(),
            self.reserved_2(),
            self.nfaen(),
            self.nfcsa(),
            self.gtiob(),
            self.reserved_3(),
            self.obdflt(),
            self.obhld(),
            self.obe(),
            self.obdf(),
            self.reserved_4(),
            self.nfben(),
            self.nfcsb()
        )
    }
}
#[doc = "General PWM Timer Cycle Setting Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpbr(pub u32);
impl Gtpbr {
    #[doc = "Cycle Setting Buffer Register"]
    #[must_use]
    #[inline(always)]
    pub const fn gtpbr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cycle Setting Buffer Register"]
    #[inline(always)]
    pub const fn set_gtpbr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gtpbr {
    #[inline(always)]
    fn default() -> Gtpbr {
        Gtpbr(0)
    }
}
impl core::fmt::Debug for Gtpbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtpbr")
            .field("gtpbr", &self.gtpbr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtpbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gtpbr {{ gtpbr: {=u32:?} }}", self.gtpbr())
    }
}
#[doc = "General PWM Timer Cycle Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpr(pub u32);
impl Gtpr {
    #[doc = "Cycle Setting Register"]
    #[must_use]
    #[inline(always)]
    pub const fn gtpr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cycle Setting Register"]
    #[inline(always)]
    pub const fn set_gtpr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gtpr {
    #[inline(always)]
    fn default() -> Gtpr {
        Gtpr(0)
    }
}
impl core::fmt::Debug for Gtpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtpr").field("gtpr", &self.gtpr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gtpr {{ gtpr: {=u32:?} }}", self.gtpr())
    }
}
#[doc = "General PWM Timer Stop Source Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpsr(pub u32);
impl Gtpsr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn psgtrgar(&self) -> super::vals::Psgtrgar {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Psgtrgar::from_bits(val as u8)
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_psgtrgar(&mut self, val: super::vals::Psgtrgar) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn psgtrgaf(&self) -> super::vals::Psgtrgaf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Psgtrgaf::from_bits(val as u8)
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_psgtrgaf(&mut self, val: super::vals::Psgtrgaf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn psgtrgbr(&self) -> super::vals::Psgtrgbr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Psgtrgbr::from_bits(val as u8)
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_psgtrgbr(&mut self, val: super::vals::Psgtrgbr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn psgtrgbf(&self) -> super::vals::Psgtrgbf {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Psgtrgbf::from_bits(val as u8)
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_psgtrgbf(&mut self, val: super::vals::Psgtrgbf) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
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
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscarbl(&self) -> super::vals::Pscarbl {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pscarbl::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscarbl(&mut self, val: super::vals::Pscarbl) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscarbh(&self) -> super::vals::Pscarbh {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pscarbh::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscarbh(&mut self, val: super::vals::Pscarbh) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscafbl(&self) -> super::vals::Pscafbl {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pscafbl::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscafbl(&mut self, val: super::vals::Pscafbl) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscafbh(&self) -> super::vals::Pscafbh {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pscafbh::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscafbh(&mut self, val: super::vals::Pscafbh) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscbral(&self) -> super::vals::Pscbral {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pscbral::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscbral(&mut self, val: super::vals::Pscbral) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscbrah(&self) -> super::vals::Pscbrah {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pscbrah::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscbrah(&mut self, val: super::vals::Pscbrah) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscbfal(&self) -> super::vals::Pscbfal {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pscbfal::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscbfal(&mut self, val: super::vals::Pscbfal) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pscbfah(&self) -> super::vals::Pscbfah {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pscbfah::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pscbfah(&mut self, val: super::vals::Pscbfah) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ELC_GPTA Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselca(&self) -> super::vals::Pselca {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pselca::from_bits(val as u8)
    }
    #[doc = "ELC_GPTA Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselca(&mut self, val: super::vals::Pselca) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "ELC_GPTB Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselcb(&self) -> super::vals::Pselcb {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pselcb::from_bits(val as u8)
    }
    #[doc = "ELC_GPTB Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselcb(&mut self, val: super::vals::Pselcb) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "ELC_GPTC Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselcc(&self) -> super::vals::Pselcc {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pselcc::from_bits(val as u8)
    }
    #[doc = "ELC_GPTC Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselcc(&mut self, val: super::vals::Pselcc) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "ELC_GPTD Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselcd(&self) -> super::vals::Pselcd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pselcd::from_bits(val as u8)
    }
    #[doc = "ELC_GPTD Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselcd(&mut self, val: super::vals::Pselcd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "ELC_GPTE Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselce(&self) -> super::vals::Pselce {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pselce::from_bits(val as u8)
    }
    #[doc = "ELC_GPTE Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselce(&mut self, val: super::vals::Pselce) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "ELC_GPTF Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselcf(&self) -> super::vals::Pselcf {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pselcf::from_bits(val as u8)
    }
    #[doc = "ELC_GPTF Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselcf(&mut self, val: super::vals::Pselcf) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "ELC_GPTG Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselcg(&self) -> super::vals::Pselcg {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pselcg::from_bits(val as u8)
    }
    #[doc = "ELC_GPTG Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselcg(&mut self, val: super::vals::Pselcg) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "ELCH Event Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pselch(&self) -> super::vals::Pselch {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pselch::from_bits(val as u8)
    }
    #[doc = "ELCH Event Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_pselch(&mut self, val: super::vals::Pselch) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "Software Source Counter Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cstop(&self) -> super::vals::Cstop {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Cstop::from_bits(val as u8)
    }
    #[doc = "Software Source Counter Stop Enable"]
    #[inline(always)]
    pub const fn set_cstop(&mut self, val: super::vals::Cstop) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gtpsr {
    #[inline(always)]
    fn default() -> Gtpsr {
        Gtpsr(0)
    }
}
impl core::fmt::Debug for Gtpsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtpsr")
            .field("psgtrgar", &self.psgtrgar())
            .field("psgtrgaf", &self.psgtrgaf())
            .field("psgtrgbr", &self.psgtrgbr())
            .field("psgtrgbf", &self.psgtrgbf())
            .field("reserved", &self.reserved())
            .field("pscarbl", &self.pscarbl())
            .field("pscarbh", &self.pscarbh())
            .field("pscafbl", &self.pscafbl())
            .field("pscafbh", &self.pscafbh())
            .field("pscbral", &self.pscbral())
            .field("pscbrah", &self.pscbrah())
            .field("pscbfal", &self.pscbfal())
            .field("pscbfah", &self.pscbfah())
            .field("pselca", &self.pselca())
            .field("pselcb", &self.pselcb())
            .field("pselcc", &self.pselcc())
            .field("pselcd", &self.pselcd())
            .field("pselce", &self.pselce())
            .field("pselcf", &self.pselcf())
            .field("pselcg", &self.pselcg())
            .field("pselch", &self.pselch())
            .field("reserved_2", &self.reserved_2())
            .field("cstop", &self.cstop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtpsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtpsr {{ psgtrgar: {:?}, psgtrgaf: {:?}, psgtrgbr: {:?}, psgtrgbf: {:?}, reserved: {=u8:?}, pscarbl: {:?}, pscarbh: {:?}, pscafbl: {:?}, pscafbh: {:?}, pscbral: {:?}, pscbrah: {:?}, pscbfal: {:?}, pscbfah: {:?}, pselca: {:?}, pselcb: {:?}, pselcc: {:?}, pselcd: {:?}, pselce: {:?}, pselcf: {:?}, pselcg: {:?}, pselch: {:?}, reserved_2: {=u8:?}, cstop: {:?} }}",
            self.psgtrgar(),
            self.psgtrgaf(),
            self.psgtrgbr(),
            self.psgtrgbf(),
            self.reserved(),
            self.pscarbl(),
            self.pscarbh(),
            self.pscafbl(),
            self.pscafbh(),
            self.pscbral(),
            self.pscbrah(),
            self.pscbfal(),
            self.pscbfah(),
            self.pselca(),
            self.pselcb(),
            self.pselcc(),
            self.pselcd(),
            self.pselce(),
            self.pselcf(),
            self.pselcg(),
            self.pselch(),
            self.reserved_2(),
            self.cstop()
        )
    }
}
#[doc = "General PWM Timer Start Source Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtssr(pub u32);
impl Gtssr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ssgtrgar(&self) -> super::vals::Ssgtrgar {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ssgtrgar::from_bits(val as u8)
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_ssgtrgar(&mut self, val: super::vals::Ssgtrgar) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ssgtrgaf(&self) -> super::vals::Ssgtrgaf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ssgtrgaf::from_bits(val as u8)
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_ssgtrgaf(&mut self, val: super::vals::Ssgtrgaf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ssgtrgbr(&self) -> super::vals::Ssgtrgbr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ssgtrgbr::from_bits(val as u8)
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_ssgtrgbr(&mut self, val: super::vals::Ssgtrgbr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ssgtrgbf(&self) -> super::vals::Ssgtrgbf {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ssgtrgbf::from_bits(val as u8)
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_ssgtrgbf(&mut self, val: super::vals::Ssgtrgbf) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
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
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscarbl(&self) -> super::vals::Sscarbl {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sscarbl::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscarbl(&mut self, val: super::vals::Sscarbl) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscarbh(&self) -> super::vals::Sscarbh {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Sscarbh::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscarbh(&mut self, val: super::vals::Sscarbh) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscafbl(&self) -> super::vals::Sscafbl {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Sscafbl::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscafbl(&mut self, val: super::vals::Sscafbl) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscafbh(&self) -> super::vals::Sscafbh {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Sscafbh::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscafbh(&mut self, val: super::vals::Sscafbh) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscbral(&self) -> super::vals::Sscbral {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sscbral::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscbral(&mut self, val: super::vals::Sscbral) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscbrah(&self) -> super::vals::Sscbrah {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sscbrah::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscbrah(&mut self, val: super::vals::Sscbrah) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscbfal(&self) -> super::vals::Sscbfal {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sscbfal::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscbfal(&mut self, val: super::vals::Sscbfal) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sscbfah(&self) -> super::vals::Sscbfah {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Sscbfah::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sscbfah(&mut self, val: super::vals::Sscbfah) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ELC_GPTA Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselca(&self) -> super::vals::Sselca {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Sselca::from_bits(val as u8)
    }
    #[doc = "ELC_GPTA Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselca(&mut self, val: super::vals::Sselca) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "ELC_GPTB Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselcb(&self) -> super::vals::Sselcb {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Sselcb::from_bits(val as u8)
    }
    #[doc = "ELC_GPTB Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselcb(&mut self, val: super::vals::Sselcb) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "ELC_GPTC Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselcc(&self) -> super::vals::Sselcc {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Sselcc::from_bits(val as u8)
    }
    #[doc = "ELC_GPTC Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselcc(&mut self, val: super::vals::Sselcc) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "ELC_GPTD Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselcd(&self) -> super::vals::Sselcd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Sselcd::from_bits(val as u8)
    }
    #[doc = "ELC_GPTD Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselcd(&mut self, val: super::vals::Sselcd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "ELC_GPTE Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselce(&self) -> super::vals::Sselce {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Sselce::from_bits(val as u8)
    }
    #[doc = "ELC_GPTE Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselce(&mut self, val: super::vals::Sselce) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "ELC_GPTF Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselcf(&self) -> super::vals::Sselcf {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Sselcf::from_bits(val as u8)
    }
    #[doc = "ELC_GPTF Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselcf(&mut self, val: super::vals::Sselcf) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "ELC_GPTG Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselcg(&self) -> super::vals::Sselcg {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Sselcg::from_bits(val as u8)
    }
    #[doc = "ELC_GPTG Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselcg(&mut self, val: super::vals::Sselcg) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "ELCH Event Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sselch(&self) -> super::vals::Sselch {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Sselch::from_bits(val as u8)
    }
    #[doc = "ELCH Event Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_sselch(&mut self, val: super::vals::Sselch) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "Software Source Counter Start Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cstrt(&self) -> super::vals::Cstrt {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Cstrt::from_bits(val as u8)
    }
    #[doc = "Software Source Counter Start Enable"]
    #[inline(always)]
    pub const fn set_cstrt(&mut self, val: super::vals::Cstrt) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gtssr {
    #[inline(always)]
    fn default() -> Gtssr {
        Gtssr(0)
    }
}
impl core::fmt::Debug for Gtssr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtssr")
            .field("ssgtrgar", &self.ssgtrgar())
            .field("ssgtrgaf", &self.ssgtrgaf())
            .field("ssgtrgbr", &self.ssgtrgbr())
            .field("ssgtrgbf", &self.ssgtrgbf())
            .field("reserved", &self.reserved())
            .field("sscarbl", &self.sscarbl())
            .field("sscarbh", &self.sscarbh())
            .field("sscafbl", &self.sscafbl())
            .field("sscafbh", &self.sscafbh())
            .field("sscbral", &self.sscbral())
            .field("sscbrah", &self.sscbrah())
            .field("sscbfal", &self.sscbfal())
            .field("sscbfah", &self.sscbfah())
            .field("sselca", &self.sselca())
            .field("sselcb", &self.sselcb())
            .field("sselcc", &self.sselcc())
            .field("sselcd", &self.sselcd())
            .field("sselce", &self.sselce())
            .field("sselcf", &self.sselcf())
            .field("sselcg", &self.sselcg())
            .field("sselch", &self.sselch())
            .field("reserved_2", &self.reserved_2())
            .field("cstrt", &self.cstrt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtssr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtssr {{ ssgtrgar: {:?}, ssgtrgaf: {:?}, ssgtrgbr: {:?}, ssgtrgbf: {:?}, reserved: {=u8:?}, sscarbl: {:?}, sscarbh: {:?}, sscafbl: {:?}, sscafbh: {:?}, sscbral: {:?}, sscbrah: {:?}, sscbfal: {:?}, sscbfah: {:?}, sselca: {:?}, sselcb: {:?}, sselcc: {:?}, sselcd: {:?}, sselce: {:?}, sselcf: {:?}, sselcg: {:?}, sselch: {:?}, reserved_2: {=u8:?}, cstrt: {:?} }}",
            self.ssgtrgar(),
            self.ssgtrgaf(),
            self.ssgtrgbr(),
            self.ssgtrgbf(),
            self.reserved(),
            self.sscarbl(),
            self.sscarbh(),
            self.sscafbl(),
            self.sscafbh(),
            self.sscbral(),
            self.sscbrah(),
            self.sscbfal(),
            self.sscbfah(),
            self.sselca(),
            self.sselcb(),
            self.sselcc(),
            self.sselcd(),
            self.sselce(),
            self.sselcf(),
            self.sselcg(),
            self.sselch(),
            self.reserved_2(),
            self.cstrt()
        )
    }
}
#[doc = "General PWM Timer Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtst(pub u32);
impl Gtst {
    #[doc = "Input Capture/Compare Match Flag A"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfa(&self) -> super::vals::Tcfa {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tcfa::from_bits(val as u8)
    }
    #[doc = "Input Capture/Compare Match Flag A"]
    #[inline(always)]
    pub const fn set_tcfa(&mut self, val: super::vals::Tcfa) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Input Capture/Compare Match Flag B"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfb(&self) -> super::vals::Tcfb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tcfb::from_bits(val as u8)
    }
    #[doc = "Input Capture/Compare Match Flag B"]
    #[inline(always)]
    pub const fn set_tcfb(&mut self, val: super::vals::Tcfb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Input Compare Match Flag C"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfc(&self) -> super::vals::Tcfc {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Tcfc::from_bits(val as u8)
    }
    #[doc = "Input Compare Match Flag C"]
    #[inline(always)]
    pub const fn set_tcfc(&mut self, val: super::vals::Tcfc) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Input Compare Match Flag D"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfd(&self) -> super::vals::Tcfd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tcfd::from_bits(val as u8)
    }
    #[doc = "Input Compare Match Flag D"]
    #[inline(always)]
    pub const fn set_tcfd(&mut self, val: super::vals::Tcfd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Input Compare Match Flag E"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfe(&self) -> super::vals::Tcfe {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcfe::from_bits(val as u8)
    }
    #[doc = "Input Compare Match Flag E"]
    #[inline(always)]
    pub const fn set_tcfe(&mut self, val: super::vals::Tcfe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Input Compare Match Flag F"]
    #[must_use]
    #[inline(always)]
    pub const fn tcff(&self) -> super::vals::Tcff {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Tcff::from_bits(val as u8)
    }
    #[doc = "Input Compare Match Flag F"]
    #[inline(always)]
    pub const fn set_tcff(&mut self, val: super::vals::Tcff) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfpo(&self) -> super::vals::Tcfpo {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Tcfpo::from_bits(val as u8)
    }
    #[doc = "Overflow Flag"]
    #[inline(always)]
    pub const fn set_tcfpo(&mut self, val: super::vals::Tcfpo) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Underflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfpu(&self) -> super::vals::Tcfpu {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Tcfpu::from_bits(val as u8)
    }
    #[doc = "Underflow Flag"]
    #[inline(always)]
    pub const fn set_tcfpu(&mut self, val: super::vals::Tcfpu) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Count Direction Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tucf(&self) -> super::vals::Tucf {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Tucf::from_bits(val as u8)
    }
    #[doc = "Count Direction Flag"]
    #[inline(always)]
    pub const fn set_tucf(&mut self, val: super::vals::Tucf) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Output Disable Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn odf(&self) -> super::vals::Odf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Odf::from_bits(val as u8)
    }
    #[doc = "Output Disable Flag"]
    #[inline(always)]
    pub const fn set_odf(&mut self, val: super::vals::Odf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_3(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
    }
    #[doc = "Same Time Output Level High Disable Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oabhf(&self) -> super::vals::Oabhf {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Oabhf::from_bits(val as u8)
    }
    #[doc = "Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub const fn set_oabhf(&mut self, val: super::vals::Oabhf) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Same Time Output Level Low Disable Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oablf(&self) -> super::vals::Oablf {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Oablf::from_bits(val as u8)
    }
    #[doc = "Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub const fn set_oablf(&mut self, val: super::vals::Oablf) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Gtst {
    #[inline(always)]
    fn default() -> Gtst {
        Gtst(0)
    }
}
impl core::fmt::Debug for Gtst {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtst")
            .field("tcfa", &self.tcfa())
            .field("tcfb", &self.tcfb())
            .field("tcfc", &self.tcfc())
            .field("tcfd", &self.tcfd())
            .field("tcfe", &self.tcfe())
            .field("tcff", &self.tcff())
            .field("tcfpo", &self.tcfpo())
            .field("tcfpu", &self.tcfpu())
            .field("reserved", &self.reserved())
            .field("tucf", &self.tucf())
            .field("reserved_2", &self.reserved_2())
            .field("odf", &self.odf())
            .field("reserved_3", &self.reserved_3())
            .field("oabhf", &self.oabhf())
            .field("oablf", &self.oablf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtst {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtst {{ tcfa: {:?}, tcfb: {:?}, tcfc: {:?}, tcfd: {:?}, tcfe: {:?}, tcff: {:?}, tcfpo: {:?}, tcfpu: {:?}, reserved: {=u8:?}, tucf: {:?}, reserved_2: {=u8:?}, odf: {:?}, reserved_3: {=u8:?}, oabhf: {:?}, oablf: {:?} }}",
            self.tcfa(),
            self.tcfb(),
            self.tcfc(),
            self.tcfd(),
            self.tcfe(),
            self.tcff(),
            self.tcfpo(),
            self.tcfpu(),
            self.reserved(),
            self.tucf(),
            self.reserved_2(),
            self.odf(),
            self.reserved_3(),
            self.oabhf(),
            self.oablf()
        )
    }
}
#[doc = "General PWM Timer Software Stop Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtstp(pub u32);
impl Gtstp {
    #[doc = "Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[must_use]
    #[inline(always)]
    pub const fn cstop0(&self) -> super::vals::Cstop0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cstop0::from_bits(val as u8)
    }
    #[doc = "Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub const fn set_cstop0(&mut self, val: super::vals::Cstop0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[must_use]
    #[inline(always)]
    pub const fn cstop1(&self) -> super::vals::Cstop1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cstop1::from_bits(val as u8)
    }
    #[doc = "Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub const fn set_cstop1(&mut self, val: super::vals::Cstop1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[must_use]
    #[inline(always)]
    pub const fn cstop2(&self) -> super::vals::Cstop2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Cstop2::from_bits(val as u8)
    }
    #[doc = "Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub const fn set_cstop2(&mut self, val: super::vals::Cstop2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[must_use]
    #[inline(always)]
    pub const fn cstop3(&self) -> super::vals::Cstop3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cstop3::from_bits(val as u8)
    }
    #[doc = "Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub const fn set_cstop3(&mut self, val: super::vals::Cstop3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[must_use]
    #[inline(always)]
    pub const fn cstop4(&self) -> super::vals::Cstop4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Cstop4::from_bits(val as u8)
    }
    #[doc = "Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub const fn set_cstop4(&mut self, val: super::vals::Cstop4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[must_use]
    #[inline(always)]
    pub const fn cstop5(&self) -> super::vals::Cstop5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cstop5::from_bits(val as u8)
    }
    #[doc = "Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub const fn set_cstop5(&mut self, val: super::vals::Cstop5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[must_use]
    #[inline(always)]
    pub const fn cstop6(&self) -> super::vals::Cstop6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cstop6::from_bits(val as u8)
    }
    #[doc = "Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub const fn set_cstop6(&mut self, val: super::vals::Cstop6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[must_use]
    #[inline(always)]
    pub const fn cstop7(&self) -> super::vals::Cstop7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cstop7::from_bits(val as u8)
    }
    #[doc = "Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub const fn set_cstop7(&mut self, val: super::vals::Cstop7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "These bits are read as 111111111111111111111111. The write value should be 111111111111111111111111."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "These bits are read as 111111111111111111111111. The write value should be 111111111111111111111111."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Gtstp {
    #[inline(always)]
    fn default() -> Gtstp {
        Gtstp(0)
    }
}
impl core::fmt::Debug for Gtstp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtstp")
            .field("cstop0", &self.cstop0())
            .field("cstop1", &self.cstop1())
            .field("cstop2", &self.cstop2())
            .field("cstop3", &self.cstop3())
            .field("cstop4", &self.cstop4())
            .field("cstop5", &self.cstop5())
            .field("cstop6", &self.cstop6())
            .field("cstop7", &self.cstop7())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtstp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtstp {{ cstop0: {:?}, cstop1: {:?}, cstop2: {:?}, cstop3: {:?}, cstop4: {:?}, cstop5: {:?}, cstop6: {:?}, cstop7: {:?}, reserved: {=u32:?} }}",
            self.cstop0(),
            self.cstop1(),
            self.cstop2(),
            self.cstop3(),
            self.cstop4(),
            self.cstop5(),
            self.cstop6(),
            self.cstop7(),
            self.reserved()
        )
    }
}
#[doc = "General PWM Timer Software Start Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtstr(pub u32);
impl Gtstr {
    #[doc = "Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[must_use]
    #[inline(always)]
    pub const fn cstrt0(&self) -> super::vals::Cstrt0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cstrt0::from_bits(val as u8)
    }
    #[doc = "Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub const fn set_cstrt0(&mut self, val: super::vals::Cstrt0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[must_use]
    #[inline(always)]
    pub const fn cstrt1(&self) -> super::vals::Cstrt1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cstrt1::from_bits(val as u8)
    }
    #[doc = "Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub const fn set_cstrt1(&mut self, val: super::vals::Cstrt1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[must_use]
    #[inline(always)]
    pub const fn cstrt2(&self) -> super::vals::Cstrt2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Cstrt2::from_bits(val as u8)
    }
    #[doc = "Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub const fn set_cstrt2(&mut self, val: super::vals::Cstrt2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[must_use]
    #[inline(always)]
    pub const fn cstrt3(&self) -> super::vals::Cstrt3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cstrt3::from_bits(val as u8)
    }
    #[doc = "Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub const fn set_cstrt3(&mut self, val: super::vals::Cstrt3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[must_use]
    #[inline(always)]
    pub const fn cstrt4(&self) -> super::vals::Cstrt4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Cstrt4::from_bits(val as u8)
    }
    #[doc = "Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub const fn set_cstrt4(&mut self, val: super::vals::Cstrt4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[must_use]
    #[inline(always)]
    pub const fn cstrt5(&self) -> super::vals::Cstrt5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cstrt5::from_bits(val as u8)
    }
    #[doc = "Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub const fn set_cstrt5(&mut self, val: super::vals::Cstrt5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[must_use]
    #[inline(always)]
    pub const fn cstrt6(&self) -> super::vals::Cstrt6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cstrt6::from_bits(val as u8)
    }
    #[doc = "Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub const fn set_cstrt6(&mut self, val: super::vals::Cstrt6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[must_use]
    #[inline(always)]
    pub const fn cstrt7(&self) -> super::vals::Cstrt7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cstrt7::from_bits(val as u8)
    }
    #[doc = "Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub const fn set_cstrt7(&mut self, val: super::vals::Cstrt7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "These bits are read as 000000000000000000000000. The write value should be 000000000000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "These bits are read as 000000000000000000000000. The write value should be 000000000000000000000000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Gtstr {
    #[inline(always)]
    fn default() -> Gtstr {
        Gtstr(0)
    }
}
impl core::fmt::Debug for Gtstr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtstr")
            .field("cstrt0", &self.cstrt0())
            .field("cstrt1", &self.cstrt1())
            .field("cstrt2", &self.cstrt2())
            .field("cstrt3", &self.cstrt3())
            .field("cstrt4", &self.cstrt4())
            .field("cstrt5", &self.cstrt5())
            .field("cstrt6", &self.cstrt6())
            .field("cstrt7", &self.cstrt7())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtstr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtstr {{ cstrt0: {:?}, cstrt1: {:?}, cstrt2: {:?}, cstrt3: {:?}, cstrt4: {:?}, cstrt5: {:?}, cstrt6: {:?}, cstrt7: {:?}, reserved: {=u32:?} }}",
            self.cstrt0(),
            self.cstrt1(),
            self.cstrt2(),
            self.cstrt3(),
            self.cstrt4(),
            self.cstrt5(),
            self.cstrt6(),
            self.cstrt7(),
            self.reserved()
        )
    }
}
#[doc = "General PWM Timer Count Direction and Duty Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuddtyc(pub u32);
impl Gtuddtyc {
    #[doc = "Count Direction Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn ud(&self) -> super::vals::Ud {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ud::from_bits(val as u8)
    }
    #[doc = "Count Direction Setting"]
    #[inline(always)]
    pub const fn set_ud(&mut self, val: super::vals::Ud) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Forcible Count Direction Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn udf(&self) -> super::vals::Udf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Udf::from_bits(val as u8)
    }
    #[doc = "Forcible Count Direction Setting"]
    #[inline(always)]
    pub const fn set_udf(&mut self, val: super::vals::Udf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "GTIOCA Output Duty Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn oadty(&self) -> super::vals::Oadty {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Oadty::from_bits(val as u8)
    }
    #[doc = "GTIOCA Output Duty Setting"]
    #[inline(always)]
    pub const fn set_oadty(&mut self, val: super::vals::Oadty) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Forcible GTIOCA Output Duty Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn oadtyf(&self) -> super::vals::Oadtyf {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Oadtyf::from_bits(val as u8)
    }
    #[doc = "Forcible GTIOCA Output Duty Setting"]
    #[inline(always)]
    pub const fn set_oadtyf(&mut self, val: super::vals::Oadtyf) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "GTIOCA Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn oadtyr(&self) -> super::vals::Oadtyr {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Oadtyr::from_bits(val as u8)
    }
    #[doc = "GTIOCA Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
    #[inline(always)]
    pub const fn set_oadtyr(&mut self, val: super::vals::Oadtyr) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "GTIOCB Output Duty Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn obdty(&self) -> super::vals::Obdty {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Obdty::from_bits(val as u8)
    }
    #[doc = "GTIOCB Output Duty Setting"]
    #[inline(always)]
    pub const fn set_obdty(&mut self, val: super::vals::Obdty) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Forcible GTIOCB Output Duty Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn obdtyf(&self) -> super::vals::Obdtyf {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Obdtyf::from_bits(val as u8)
    }
    #[doc = "Forcible GTIOCB Output Duty Setting"]
    #[inline(always)]
    pub const fn set_obdtyf(&mut self, val: super::vals::Obdtyf) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "GTIOCB Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn obdtyr(&self) -> super::vals::Obdtyr {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Obdtyr::from_bits(val as u8)
    }
    #[doc = "GTIOCB Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
    #[inline(always)]
    pub const fn set_obdtyr(&mut self, val: super::vals::Obdtyr) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Gtuddtyc {
    #[inline(always)]
    fn default() -> Gtuddtyc {
        Gtuddtyc(0)
    }
}
impl core::fmt::Debug for Gtuddtyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtuddtyc")
            .field("ud", &self.ud())
            .field("udf", &self.udf())
            .field("oadty", &self.oadty())
            .field("oadtyf", &self.oadtyf())
            .field("oadtyr", &self.oadtyr())
            .field("reserved", &self.reserved())
            .field("obdty", &self.obdty())
            .field("obdtyf", &self.obdtyf())
            .field("obdtyr", &self.obdtyr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtuddtyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtuddtyc {{ ud: {:?}, udf: {:?}, oadty: {:?}, oadtyf: {:?}, oadtyr: {:?}, reserved: {=u8:?}, obdty: {:?}, obdtyf: {:?}, obdtyr: {:?} }}",
            self.ud(),
            self.udf(),
            self.oadty(),
            self.oadtyf(),
            self.oadtyr(),
            self.reserved(),
            self.obdty(),
            self.obdtyf(),
            self.obdtyr()
        )
    }
}
#[doc = "General PWM Timer Up Count Source Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtupsr(pub u32);
impl Gtupsr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usgtrgar(&self) -> super::vals::Usgtrgar {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usgtrgar::from_bits(val as u8)
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_usgtrgar(&mut self, val: super::vals::Usgtrgar) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usgtrgaf(&self) -> super::vals::Usgtrgaf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Usgtrgaf::from_bits(val as u8)
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_usgtrgaf(&mut self, val: super::vals::Usgtrgaf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usgtrgbr(&self) -> super::vals::Usgtrgbr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Usgtrgbr::from_bits(val as u8)
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_usgtrgbr(&mut self, val: super::vals::Usgtrgbr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usgtrgbf(&self) -> super::vals::Usgtrgbf {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Usgtrgbf::from_bits(val as u8)
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_usgtrgbf(&mut self, val: super::vals::Usgtrgbf) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscarbl(&self) -> super::vals::Uscarbl {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Uscarbl::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscarbl(&mut self, val: super::vals::Uscarbl) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscarbh(&self) -> super::vals::Uscarbh {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Uscarbh::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscarbh(&mut self, val: super::vals::Uscarbh) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscafbl(&self) -> super::vals::Uscafbl {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Uscafbl::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscafbl(&mut self, val: super::vals::Uscafbl) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscafbh(&self) -> super::vals::Uscafbh {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Uscafbh::from_bits(val as u8)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscafbh(&mut self, val: super::vals::Uscafbh) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscbral(&self) -> super::vals::Uscbral {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Uscbral::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscbral(&mut self, val: super::vals::Uscbral) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscbrah(&self) -> super::vals::Uscbrah {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Uscbrah::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscbrah(&mut self, val: super::vals::Uscbrah) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscbfal(&self) -> super::vals::Uscbfal {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Uscbfal::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscbfal(&mut self, val: super::vals::Uscbfal) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uscbfah(&self) -> super::vals::Uscbfah {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Uscbfah::from_bits(val as u8)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uscbfah(&mut self, val: super::vals::Uscbfah) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ELC_GPTA Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselca(&self) -> super::vals::Uselca {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Uselca::from_bits(val as u8)
    }
    #[doc = "ELC_GPTA Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselca(&mut self, val: super::vals::Uselca) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "ELC_GPTB Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselcb(&self) -> super::vals::Uselcb {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Uselcb::from_bits(val as u8)
    }
    #[doc = "ELC_GPTB Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselcb(&mut self, val: super::vals::Uselcb) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "ELC_GPTC Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselcc(&self) -> super::vals::Uselcc {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Uselcc::from_bits(val as u8)
    }
    #[doc = "ELC_GPTC Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselcc(&mut self, val: super::vals::Uselcc) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "ELC_GPTD Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselcd(&self) -> super::vals::Uselcd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Uselcd::from_bits(val as u8)
    }
    #[doc = "ELC_GPTD Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselcd(&mut self, val: super::vals::Uselcd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "ELC_GPTE Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselce(&self) -> super::vals::Uselce {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Uselce::from_bits(val as u8)
    }
    #[doc = "ELC_GPTE Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselce(&mut self, val: super::vals::Uselce) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "ELC_GPTF Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselcf(&self) -> super::vals::Uselcf {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Uselcf::from_bits(val as u8)
    }
    #[doc = "ELC_GPTF Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselcf(&mut self, val: super::vals::Uselcf) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "ELC_GPTG Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselcg(&self) -> super::vals::Uselcg {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Uselcg::from_bits(val as u8)
    }
    #[doc = "ELC_GPTG Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselcg(&mut self, val: super::vals::Uselcg) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "ELCH Event Source Counter Count Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uselch(&self) -> super::vals::Uselch {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Uselch::from_bits(val as u8)
    }
    #[doc = "ELCH Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub const fn set_uselch(&mut self, val: super::vals::Uselch) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Gtupsr {
    #[inline(always)]
    fn default() -> Gtupsr {
        Gtupsr(0)
    }
}
impl core::fmt::Debug for Gtupsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtupsr")
            .field("usgtrgar", &self.usgtrgar())
            .field("usgtrgaf", &self.usgtrgaf())
            .field("usgtrgbr", &self.usgtrgbr())
            .field("usgtrgbf", &self.usgtrgbf())
            .field("uscarbl", &self.uscarbl())
            .field("uscarbh", &self.uscarbh())
            .field("uscafbl", &self.uscafbl())
            .field("uscafbh", &self.uscafbh())
            .field("uscbral", &self.uscbral())
            .field("uscbrah", &self.uscbrah())
            .field("uscbfal", &self.uscbfal())
            .field("uscbfah", &self.uscbfah())
            .field("uselca", &self.uselca())
            .field("uselcb", &self.uselcb())
            .field("uselcc", &self.uselcc())
            .field("uselcd", &self.uselcd())
            .field("uselce", &self.uselce())
            .field("uselcf", &self.uselcf())
            .field("uselcg", &self.uselcg())
            .field("uselch", &self.uselch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtupsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtupsr {{ usgtrgar: {:?}, usgtrgaf: {:?}, usgtrgbr: {:?}, usgtrgbf: {:?}, uscarbl: {:?}, uscarbh: {:?}, uscafbl: {:?}, uscafbh: {:?}, uscbral: {:?}, uscbrah: {:?}, uscbfal: {:?}, uscbfah: {:?}, uselca: {:?}, uselcb: {:?}, uselcc: {:?}, uselcd: {:?}, uselce: {:?}, uselcf: {:?}, uselcg: {:?}, uselch: {:?} }}",
            self.usgtrgar(),
            self.usgtrgaf(),
            self.usgtrgbr(),
            self.usgtrgbf(),
            self.uscarbl(),
            self.uscarbh(),
            self.uscafbl(),
            self.uscafbh(),
            self.uscbral(),
            self.uscbrah(),
            self.uscbfal(),
            self.uscbfah(),
            self.uselca(),
            self.uselcb(),
            self.uselcc(),
            self.uselcd(),
            self.uselce(),
            self.uselcf(),
            self.uselcg(),
            self.uselch()
        )
    }
}
#[doc = "General PWM Timer Write-Protection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtwp(pub u32);
impl Gtwp {
    #[doc = "Register Write Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn wp(&self) -> super::vals::Wp {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Wp::from_bits(val as u8)
    }
    #[doc = "Register Write Disable"]
    #[inline(always)]
    pub const fn set_wp(&mut self, val: super::vals::Wp) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
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
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "GTWP Key Code"]
    #[must_use]
    #[inline(always)]
    pub const fn prkey(&self) -> super::vals::Prkey {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Prkey::from_bits(val as u8)
    }
    #[doc = "GTWP Key Code"]
    #[inline(always)]
    pub const fn set_prkey(&mut self, val: super::vals::Prkey) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Gtwp {
    #[inline(always)]
    fn default() -> Gtwp {
        Gtwp(0)
    }
}
impl core::fmt::Debug for Gtwp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtwp")
            .field("wp", &self.wp())
            .field("reserved", &self.reserved())
            .field("prkey", &self.prkey())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtwp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gtwp {{ wp: {:?}, reserved: {=u8:?}, prkey: {:?}, reserved_2: {=u16:?} }}",
            self.wp(),
            self.reserved(),
            self.prkey(),
            self.reserved_2()
        )
    }
}
