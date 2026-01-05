#[doc = "Port 0 Control Registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port0 {
    ptr: *mut u8,
}
unsafe impl Send for Port0 {}
unsafe impl Sync for Port0 {}
impl Port0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port Control Register 1"]
    #[inline(always)]
    pub const fn pcntr1(self) -> crate::common::Reg<regs::Pcntr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Output data register"]
    #[inline(always)]
    pub const fn podr(self) -> crate::common::Reg<regs::Podr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Data direction register"]
    #[inline(always)]
    pub const fn pdr(self) -> crate::common::Reg<regs::Pdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Port Control Register 2"]
    #[inline(always)]
    pub const fn pcntr2(self) -> crate::common::Reg<regs::Pcntr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Input data register"]
    #[inline(always)]
    pub const fn pidr(self) -> crate::common::Reg<regs::Pidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Port Control Register 3"]
    #[inline(always)]
    pub const fn pcntr3(self) -> crate::common::Reg<regs::Pcntr3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Output reset register"]
    #[inline(always)]
    pub const fn porr(self) -> crate::common::Reg<regs::Porr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Output set register"]
    #[inline(always)]
    pub const fn posr(self) -> crate::common::Reg<regs::Posr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
}
pub mod regs;
pub mod vals;
