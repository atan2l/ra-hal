#[doc = "Interrupt Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icu {
    ptr: *mut u8,
}
unsafe impl Send for Icu {}
unsafe impl Sync for Icu {}
impl Icu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "IRQ Control Register %s"]
    #[inline(always)]
    pub const fn irqcr(self, n: usize) -> crate::common::Reg<regs::Irqcr, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 1usize) as _) }
    }
    #[doc = "NMI Pin Interrupt Control Register"]
    #[inline(always)]
    pub const fn nmicr(self) -> crate::common::Reg<regs::Nmicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Non-Maskable Interrupt Enable Register"]
    #[inline(always)]
    pub const fn nmier(self) -> crate::common::Reg<regs::Nmier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Non-Maskable Interrupt Status Clear Register"]
    #[inline(always)]
    pub const fn nmiclr(self) -> crate::common::Reg<regs::Nmiclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Non-Maskable Interrupt Status Register"]
    #[inline(always)]
    pub const fn nmisr(self) -> crate::common::Reg<regs::Nmisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Wake Up Interrupt Enable Register"]
    #[inline(always)]
    pub const fn wupen(self) -> crate::common::Reg<regs::Wupen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "SYS Event Link Setting Register"]
    #[inline(always)]
    pub const fn selsr0(self) -> crate::common::Reg<regs::Selsr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "DMAC Event Link Setting Register"]
    #[inline(always)]
    pub const fn delsr(self, n: usize) -> crate::common::Reg<regs::Delsr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 4usize) as _)
        }
    }
    #[doc = "ICU Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn ielsr(self, n: usize) -> crate::common::Reg<regs::Ielsr, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
