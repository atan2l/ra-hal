#[doc = "Serial Sound Interface Ver.2.0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssie0 {
    ptr: *mut u8,
}
unsafe impl Send for Ssie0 {}
unsafe impl Sync for Ssie0 {}
impl Ssie0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn ssicr(self) -> crate::common::Reg<regs::Ssicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn ssisr(self) -> crate::common::Reg<regs::Ssisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn ssifcr(self) -> crate::common::Reg<regs::Ssifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "FIFO Status Register"]
    #[inline(always)]
    pub const fn ssifsr(self) -> crate::common::Reg<regs::Ssifsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Transmit FIFO Data Register"]
    #[inline(always)]
    pub const fn ssiftdr(self) -> crate::common::Reg<regs::Ssiftdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Receive FIFO Data Register"]
    #[inline(always)]
    pub const fn ssifrdr(self) -> crate::common::Reg<regs::Ssifrdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "TDM Mode Register"]
    #[inline(always)]
    pub const fn ssitdmr(self) -> crate::common::Reg<regs::Ssitdmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Status Control Register"]
    #[inline(always)]
    pub const fn ssiscr(self) -> crate::common::Reg<regs::Ssiscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
}
pub mod regs;
pub mod vals;
