#[doc = "Watchdog Timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt {
    ptr: *mut u8,
}
unsafe impl Send for Wdt {}
unsafe impl Sync for Wdt {}
impl Wdt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "WDT Refresh Register"]
    #[inline(always)]
    pub const fn wdtrr(self) -> crate::common::Reg<regs::Wdtrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "WDT Control Register"]
    #[inline(always)]
    pub const fn wdtcr(self) -> crate::common::Reg<regs::Wdtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "WDT Status Register"]
    #[inline(always)]
    pub const fn wdtsr(self) -> crate::common::Reg<regs::Wdtsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "WDT Reset Control Register"]
    #[inline(always)]
    pub const fn wdtrcr(self) -> crate::common::Reg<regs::Wdtrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "WDT Count Stop Control Register"]
    #[inline(always)]
    pub const fn wdtcstpr(self) -> crate::common::Reg<regs::Wdtcstpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs;
pub mod vals;
