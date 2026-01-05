#[doc = "Capacitive Touch Sensing Unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsu {
    ptr: *mut u8,
}
unsafe impl Send for Ctsu {}
unsafe impl Sync for Ctsu {}
impl Ctsu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CTSU Control Register 0"]
    #[inline(always)]
    pub const fn ctsucr0(self) -> crate::common::Reg<regs::Ctsucr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "CTSU Control Register 1"]
    #[inline(always)]
    pub const fn ctsucr1(self) -> crate::common::Reg<regs::Ctsucr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "CTSU Synchronous Noise Reduction Setting Register"]
    #[inline(always)]
    pub const fn ctsusdprs(self) -> crate::common::Reg<regs::Ctsusdprs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "CTSU Sensor Stabilization Wait Control Register"]
    #[inline(always)]
    pub const fn ctsusst(self) -> crate::common::Reg<regs::Ctsusst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "CTSU Measurement Channel Register 0"]
    #[inline(always)]
    pub const fn ctsumch0(self) -> crate::common::Reg<regs::Ctsumch0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CTSU Measurement Channel Register 1"]
    #[inline(always)]
    pub const fn ctsumch1(self) -> crate::common::Reg<regs::Ctsumch1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "CTSU Channel Enable Control Register 0"]
    #[inline(always)]
    pub const fn ctsuchac0(self) -> crate::common::Reg<regs::Ctsuchac0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "CTSU Channel Enable Control Register 1"]
    #[inline(always)]
    pub const fn ctsuchac1(self) -> crate::common::Reg<regs::Ctsuchac1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "CTSU Channel Enable Control Register 2"]
    #[inline(always)]
    pub const fn ctsuchac2(self) -> crate::common::Reg<regs::Ctsuchac2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CTSU Channel Enable Control Register 3"]
    #[inline(always)]
    pub const fn ctsuchac3(self) -> crate::common::Reg<regs::Ctsuchac3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "CTSU Channel Enable Control Register 4"]
    #[inline(always)]
    pub const fn ctsuchac4(self) -> crate::common::Reg<regs::Ctsuchac4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "CTSU Channel Transmit/Receive Control Register 0"]
    #[inline(always)]
    pub const fn ctsuchtrc0(self) -> crate::common::Reg<regs::Ctsuchtrc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "CTSU Channel Transmit/Receive Control Register 1"]
    #[inline(always)]
    pub const fn ctsuchtrc1(self) -> crate::common::Reg<regs::Ctsuchtrc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "CTSU Channel Transmit/Receive Control Register 3"]
    #[inline(always)]
    pub const fn ctsuchtrc2(self) -> crate::common::Reg<regs::Ctsuchtrc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "CTSU Channel Transmit/Receive Control Register 3"]
    #[inline(always)]
    pub const fn ctsuchtrc3(self) -> crate::common::Reg<regs::Ctsuchtrc3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "CTSU Channel Transmit/Receive Control Register 4"]
    #[inline(always)]
    pub const fn ctsuchtrc4(self) -> crate::common::Reg<regs::Ctsuchtrc4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fusize) as _) }
    }
    #[doc = "CTSU High-Pass Noise Reduction Control Register"]
    #[inline(always)]
    pub const fn ctsudclkc(self) -> crate::common::Reg<regs::Ctsudclkc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "CTSU Status Register"]
    #[inline(always)]
    pub const fn ctsust(self) -> crate::common::Reg<regs::Ctsust, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11usize) as _) }
    }
    #[doc = "CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register"]
    #[inline(always)]
    pub const fn ctsussc(self) -> crate::common::Reg<regs::Ctsussc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "CTSU Sensor Offset Register 0"]
    #[inline(always)]
    pub const fn ctsuso0(self) -> crate::common::Reg<regs::Ctsuso0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "CTSU Sensor Offset Register 1"]
    #[inline(always)]
    pub const fn ctsuso1(self) -> crate::common::Reg<regs::Ctsuso1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "CTSU Sensor Counter"]
    #[inline(always)]
    pub const fn ctsusc(self) -> crate::common::Reg<regs::Ctsusc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "CTSU Reference Counter"]
    #[inline(always)]
    pub const fn ctsurc(self) -> crate::common::Reg<regs::Ctsurc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "CTSU Error Status Register"]
    #[inline(always)]
    pub const fn ctsuerrs(self) -> crate::common::Reg<regs::Ctsuerrs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
