#[doc = "CAC Counter Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacntbr(pub u16);
impl Cacntbr {
    #[doc = "CACNTBR is a 16-bit read-only register that retains the counter value at the time a valid reference signal edge is input"]
    #[must_use]
    #[inline(always)]
    pub const fn cacntbr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CACNTBR is a 16-bit read-only register that retains the counter value at the time a valid reference signal edge is input"]
    #[inline(always)]
    pub const fn set_cacntbr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Cacntbr {
    #[inline(always)]
    fn default() -> Cacntbr {
        Cacntbr(0)
    }
}
impl core::fmt::Debug for Cacntbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cacntbr")
            .field("cacntbr", &self.cacntbr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cacntbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cacntbr {{ cacntbr: {=u16:?} }}", self.cacntbr())
    }
}
#[doc = "CAC Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacr0(pub u8);
impl Cacr0 {
    #[doc = "Clock Frequency Measurement Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cfme(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Frequency Measurement Enable."]
    #[inline(always)]
    pub const fn set_cfme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
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
impl Default for Cacr0 {
    #[inline(always)]
    fn default() -> Cacr0 {
        Cacr0(0)
    }
}
impl core::fmt::Debug for Cacr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cacr0")
            .field("cfme", &self.cfme())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cacr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cacr0 {{ cfme: {=bool:?}, reserved: {=u8:?} }}",
            self.cfme(),
            self.reserved()
        )
    }
}
#[doc = "CAC Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacr1(pub u8);
impl Cacr1 {
    #[doc = "CACREF Pin Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cacrefe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CACREF Pin Input Enable"]
    #[inline(always)]
    pub const fn set_cacrefe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Measurement Target Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn fmcs(&self) -> super::vals::Fmcs {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Fmcs::from_bits(val as u8)
    }
    #[doc = "Measurement Target Clock Select"]
    #[inline(always)]
    pub const fn set_fmcs(&mut self, val: super::vals::Fmcs) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u8) & 0x07) << 1usize);
    }
    #[doc = "Measurement Target Clock Frequency Division Ratio Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tcss(&self) -> super::vals::Tcss {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Tcss::from_bits(val as u8)
    }
    #[doc = "Measurement Target Clock Frequency Division Ratio Select"]
    #[inline(always)]
    pub const fn set_tcss(&mut self, val: super::vals::Tcss) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
    }
    #[doc = "Valid Edge Select"]
    #[must_use]
    #[inline(always)]
    pub const fn edges(&self) -> super::vals::Edges {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Edges::from_bits(val as u8)
    }
    #[doc = "Valid Edge Select"]
    #[inline(always)]
    pub const fn set_edges(&mut self, val: super::vals::Edges) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u8) & 0x03) << 6usize);
    }
}
impl Default for Cacr1 {
    #[inline(always)]
    fn default() -> Cacr1 {
        Cacr1(0)
    }
}
impl core::fmt::Debug for Cacr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cacr1")
            .field("cacrefe", &self.cacrefe())
            .field("fmcs", &self.fmcs())
            .field("tcss", &self.tcss())
            .field("edges", &self.edges())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cacr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cacr1 {{ cacrefe: {=bool:?}, fmcs: {:?}, tcss: {:?}, edges: {:?} }}",
            self.cacrefe(),
            self.fmcs(),
            self.tcss(),
            self.edges()
        )
    }
}
#[doc = "CAC Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacr2(pub u8);
impl Cacr2 {
    #[doc = "Reference Signal Select"]
    #[must_use]
    #[inline(always)]
    pub const fn rps(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reference Signal Select"]
    #[inline(always)]
    pub const fn set_rps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Measurement Reference Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn rscs(&self) -> super::vals::Rscs {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Rscs::from_bits(val as u8)
    }
    #[doc = "Measurement Reference Clock Select"]
    #[inline(always)]
    pub const fn set_rscs(&mut self, val: super::vals::Rscs) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u8) & 0x07) << 1usize);
    }
    #[doc = "Measurement Reference Clock Frequency Division Ratio Select"]
    #[must_use]
    #[inline(always)]
    pub const fn rcds(&self) -> super::vals::Rcds {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Rcds::from_bits(val as u8)
    }
    #[doc = "Measurement Reference Clock Frequency Division Ratio Select"]
    #[inline(always)]
    pub const fn set_rcds(&mut self, val: super::vals::Rcds) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
    }
    #[doc = "Digital Filter Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn dfs(&self) -> super::vals::Dfs {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Dfs::from_bits(val as u8)
    }
    #[doc = "Digital Filter Selection"]
    #[inline(always)]
    pub const fn set_dfs(&mut self, val: super::vals::Dfs) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u8) & 0x03) << 6usize);
    }
}
impl Default for Cacr2 {
    #[inline(always)]
    fn default() -> Cacr2 {
        Cacr2(0)
    }
}
impl core::fmt::Debug for Cacr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cacr2")
            .field("rps", &self.rps())
            .field("rscs", &self.rscs())
            .field("rcds", &self.rcds())
            .field("dfs", &self.dfs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cacr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cacr2 {{ rps: {=bool:?}, rscs: {:?}, rcds: {:?}, dfs: {:?} }}",
            self.rps(),
            self.rscs(),
            self.rcds(),
            self.dfs()
        )
    }
}
#[doc = "CAC Interrupt Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caicr(pub u8);
impl Caicr {
    #[doc = "Frequency Error Interrupt Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ferrie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Frequency Error Interrupt Request Enable"]
    #[inline(always)]
    pub const fn set_ferrie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Measurement End Interrupt Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mendie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Measurement End Interrupt Request Enable"]
    #[inline(always)]
    pub const fn set_mendie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Overflow Interrupt Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovfie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Interrupt Request Enable"]
    #[inline(always)]
    pub const fn set_ovfie(&mut self, val: bool) {
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
    #[doc = "FERRF Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ferrfcl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "FERRF Clear"]
    #[inline(always)]
    pub const fn set_ferrfcl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "MENDF Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn mendfcl(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "MENDF Clear"]
    #[inline(always)]
    pub const fn set_mendfcl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "OVFF Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ovffcl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OVFF Clear"]
    #[inline(always)]
    pub const fn set_ovffcl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub const fn set_reserved_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Caicr {
    #[inline(always)]
    fn default() -> Caicr {
        Caicr(0)
    }
}
impl core::fmt::Debug for Caicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Caicr")
            .field("ferrie", &self.ferrie())
            .field("mendie", &self.mendie())
            .field("ovfie", &self.ovfie())
            .field("reserved", &self.reserved())
            .field("ferrfcl", &self.ferrfcl())
            .field("mendfcl", &self.mendfcl())
            .field("ovffcl", &self.ovffcl())
            .field("reserved_2", &self.reserved_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Caicr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Caicr {{ ferrie: {=bool:?}, mendie: {=bool:?}, ovfie: {=bool:?}, reserved: {=bool:?}, ferrfcl: {=bool:?}, mendfcl: {=bool:?}, ovffcl: {=bool:?}, reserved_2: {=bool:?} }}",
            self.ferrie(),
            self.mendie(),
            self.ovfie(),
            self.reserved(),
            self.ferrfcl(),
            self.mendfcl(),
            self.ovffcl(),
            self.reserved_2()
        )
    }
}
#[doc = "CAC Lower-Limit Value Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Callvr(pub u16);
impl Callvr {
    #[doc = "CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn callvr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency."]
    #[inline(always)]
    pub const fn set_callvr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Callvr {
    #[inline(always)]
    fn default() -> Callvr {
        Callvr(0)
    }
}
impl core::fmt::Debug for Callvr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Callvr")
            .field("callvr", &self.callvr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Callvr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Callvr {{ callvr: {=u16:?} }}", self.callvr())
    }
}
#[doc = "CAC Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Castr(pub u8);
impl Castr {
    #[doc = "Frequency Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ferrf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Frequency Error Flag"]
    #[inline(always)]
    pub const fn set_ferrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Measurement End Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn mendf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Measurement End Flag"]
    #[inline(always)]
    pub const fn set_mendf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Counter Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ovff(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Counter Overflow Flag"]
    #[inline(always)]
    pub const fn set_ovff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
}
impl Default for Castr {
    #[inline(always)]
    fn default() -> Castr {
        Castr(0)
    }
}
impl core::fmt::Debug for Castr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Castr")
            .field("ferrf", &self.ferrf())
            .field("mendf", &self.mendf())
            .field("ovff", &self.ovff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Castr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Castr {{ ferrf: {=bool:?}, mendf: {=bool:?}, ovff: {=bool:?} }}",
            self.ferrf(),
            self.mendf(),
            self.ovff()
        )
    }
}
#[doc = "CAC Upper-Limit Value Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caulvr(pub u16);
impl Caulvr {
    #[doc = "CAULVR is a 16-bit readable/writable register that stores the upper-limit value of the frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn caulvr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CAULVR is a 16-bit readable/writable register that stores the upper-limit value of the frequency."]
    #[inline(always)]
    pub const fn set_caulvr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Caulvr {
    #[inline(always)]
    fn default() -> Caulvr {
        Caulvr(0)
    }
}
impl core::fmt::Debug for Caulvr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Caulvr")
            .field("caulvr", &self.caulvr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Caulvr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Caulvr {{ caulvr: {=u16:?} }}", self.caulvr())
    }
}
