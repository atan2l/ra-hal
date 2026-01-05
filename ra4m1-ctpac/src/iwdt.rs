#[doc = "Independent Watchdog Timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdt {
    ptr: *mut u8,
}
unsafe impl Send for Iwdt {}
unsafe impl Sync for Iwdt {}
impl Iwdt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "IWDT Refresh Register"]
    #[inline(always)]
    pub const fn iwdtrr(self) -> crate::common::Reg<regs::Iwdtrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "IWDT Status Register"]
    #[inline(always)]
    pub const fn iwdtsr(self) -> crate::common::Reg<regs::Iwdtsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs;
pub mod vals;
