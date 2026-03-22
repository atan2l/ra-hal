#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ofs0(pub u32);
impl Ofs0 {
    #[doc = "IWDT Start Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iwdtstrt(&self) -> super::vals::StartMode {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::StartMode::from_bits(val as u8)
    }
    #[doc = "IWDT Start Mode Select"]
    #[inline(always)]
    pub const fn set_iwdtstrt(&mut self, val: super::vals::StartMode) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IWDT Timeout Period Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iwdttops(&self) -> super::vals::IwdtTops {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::IwdtTops::from_bits(val as u8)
    }
    #[doc = "IWDT Timeout Period Select"]
    #[inline(always)]
    pub const fn set_iwdttops(&mut self, val: super::vals::IwdtTops) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "IWDT-Dedicated Clock Frequency Division Ratio Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iwdtcks(&self) -> super::vals::IwdtCks {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::IwdtCks::from_bits(val as u8)
    }
    #[doc = "IWDT-Dedicated Clock Frequency Division Ratio Select"]
    #[inline(always)]
    pub const fn set_iwdtcks(&mut self, val: super::vals::IwdtCks) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "IWDT Window End Position Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iwdtrpes(&self) -> super::vals::Rpes {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Rpes::from_bits(val as u8)
    }
    #[doc = "IWDT Window End Position Select"]
    #[inline(always)]
    pub const fn set_iwdtrpes(&mut self, val: super::vals::Rpes) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "IWDT Window Start Position Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iwdtrpss(&self) -> super::vals::Rpss {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Rpss::from_bits(val as u8)
    }
    #[doc = "IWDT Window Start Position Select"]
    #[inline(always)]
    pub const fn set_iwdtrpss(&mut self, val: super::vals::Rpss) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "IWDT Reset Interrupt Request Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iwdtrstirqs(&self) -> super::vals::UnderflowAction {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::UnderflowAction::from_bits(val as u8)
    }
    #[doc = "IWDT Reset Interrupt Request Select"]
    #[inline(always)]
    pub const fn set_iwdtrstirqs(&mut self, val: super::vals::UnderflowAction) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "IWDT Stop Control"]
    #[must_use]
    #[inline(always)]
    pub const fn iwdtstpctl(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "IWDT Stop Control"]
    #[inline(always)]
    pub const fn set_iwdtstpctl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "WDT Start Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn wdtstrt(&self) -> super::vals::StartMode {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::StartMode::from_bits(val as u8)
    }
    #[doc = "WDT Start Mode Select"]
    #[inline(always)]
    pub const fn set_wdtstrt(&mut self, val: super::vals::StartMode) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "WDT Timeout Period Select"]
    #[must_use]
    #[inline(always)]
    pub const fn wdttops(&self) -> super::vals::WdtTops {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::WdtTops::from_bits(val as u8)
    }
    #[doc = "WDT Timeout Period Select"]
    #[inline(always)]
    pub const fn set_wdttops(&mut self, val: super::vals::WdtTops) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "WDT Clock Frequency Division Ratio Select"]
    #[must_use]
    #[inline(always)]
    pub const fn wdtcks(&self) -> super::vals::WdtCks {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::WdtCks::from_bits(val as u8)
    }
    #[doc = "WDT Clock Frequency Division Ratio Select"]
    #[inline(always)]
    pub const fn set_wdtcks(&mut self, val: super::vals::WdtCks) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "WDT Window End Position Select"]
    #[must_use]
    #[inline(always)]
    pub const fn wdtrpes(&self) -> super::vals::Rpes {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Rpes::from_bits(val as u8)
    }
    #[doc = "WDT Window End Position Select"]
    #[inline(always)]
    pub const fn set_wdtrpes(&mut self, val: super::vals::Rpes) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "WDT Window Start Position Select"]
    #[must_use]
    #[inline(always)]
    pub const fn wdtrpss(&self) -> super::vals::Rpss {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Rpss::from_bits(val as u8)
    }
    #[doc = "WDT Window Start Position Select"]
    #[inline(always)]
    pub const fn set_wdtrpss(&mut self, val: super::vals::Rpss) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "WDT Reset Interrupt Request Select"]
    #[must_use]
    #[inline(always)]
    pub const fn wdtrstirqs(&self) -> super::vals::UnderflowAction {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::UnderflowAction::from_bits(val as u8)
    }
    #[doc = "WDT Reset Interrupt Request Select"]
    #[inline(always)]
    pub const fn set_wdtrstirqs(&mut self, val: super::vals::UnderflowAction) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "WDT Stop Control"]
    #[must_use]
    #[inline(always)]
    pub const fn wdtstpctl(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "WDT Stop Control"]
    #[inline(always)]
    pub const fn set_wdtstpctl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ofs0 {
    #[inline(always)]
    fn default() -> Ofs0 {
        Ofs0(0)
    }
}
impl core::fmt::Debug for Ofs0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ofs0")
            .field("iwdtstrt", &self.iwdtstrt())
            .field("iwdttops", &self.iwdttops())
            .field("iwdtcks", &self.iwdtcks())
            .field("iwdtrpes", &self.iwdtrpes())
            .field("iwdtrpss", &self.iwdtrpss())
            .field("iwdtrstirqs", &self.iwdtrstirqs())
            .field("iwdtstpctl", &self.iwdtstpctl())
            .field("wdtstrt", &self.wdtstrt())
            .field("wdttops", &self.wdttops())
            .field("wdtcks", &self.wdtcks())
            .field("wdtrpes", &self.wdtrpes())
            .field("wdtrpss", &self.wdtrpss())
            .field("wdtrstirqs", &self.wdtrstirqs())
            .field("wdtstpctl", &self.wdtstpctl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ofs0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ofs0 {{ iwdtstrt: {:?}, iwdttops: {:?}, iwdtcks: {:?}, iwdtrpes: {:?}, iwdtrpss: {:?}, iwdtrstirqs: {:?}, iwdtstpctl: {=bool:?}, wdtstrt: {:?}, wdttops: {:?}, wdtcks: {:?}, wdtrpes: {:?}, wdtrpss: {:?}, wdtrstirqs: {:?}, wdtstpctl: {=bool:?} }}",
            self.iwdtstrt(),
            self.iwdttops(),
            self.iwdtcks(),
            self.iwdtrpes(),
            self.iwdtrpss(),
            self.iwdtrstirqs(),
            self.iwdtstpctl(),
            self.wdtstrt(),
            self.wdttops(),
            self.wdtcks(),
            self.wdtrpes(),
            self.wdtrpss(),
            self.wdtrstirqs(),
            self.wdtstpctl()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ofs1(pub u32);
impl Ofs1 {
    #[doc = "Voltage Detection 0 Circuit Start"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdas(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Detection 0 Circuit Start"]
    #[inline(always)]
    pub const fn set_lvdas(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Voltage Detection 0 Level Select"]
    #[must_use]
    #[inline(always)]
    pub const fn vdsel1(&self) -> super::vals::Vdsel1 {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Vdsel1::from_bits(val as u8)
    }
    #[doc = "Voltage Detection 0 Level Select"]
    #[inline(always)]
    pub const fn set_vdsel1(&mut self, val: super::vals::Vdsel1) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "HOCO Oscillation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hocoen(&self) -> super::vals::HocoEnable {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::HocoEnable::from_bits(val as u8)
    }
    #[doc = "HOCO Oscillation Enable"]
    #[inline(always)]
    pub const fn set_hocoen(&mut self, val: super::vals::HocoEnable) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "HOCO Frequency Setting 1"]
    #[must_use]
    #[inline(always)]
    pub const fn hocofrq(&self) -> super::vals::HocoFrq {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::HocoFrq::from_bits(val as u8)
    }
    #[doc = "HOCO Frequency Setting 1"]
    #[inline(always)]
    pub const fn set_hocofrq(&mut self, val: super::vals::HocoFrq) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
}
impl Default for Ofs1 {
    #[inline(always)]
    fn default() -> Ofs1 {
        Ofs1(0)
    }
}
impl core::fmt::Debug for Ofs1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ofs1")
            .field("lvdas", &self.lvdas())
            .field("vdsel1", &self.vdsel1())
            .field("hocoen", &self.hocoen())
            .field("hocofrq", &self.hocofrq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ofs1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ofs1 {{ lvdas: {=bool:?}, vdsel1: {:?}, hocoen: {:?}, hocofrq: {:?} }}",
            self.lvdas(),
            self.vdsel1(),
            self.hocoen(),
            self.hocofrq()
        )
    }
}
