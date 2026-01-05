#[doc = "Asynchronous General purpose Timer 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agt0 {
    ptr: *mut u8,
}
unsafe impl Send for Agt0 {}
unsafe impl Sync for Agt0 {}
impl Agt0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "AGT Counter Register"]
    #[inline(always)]
    pub const fn agt(self) -> crate::common::Reg<regs::Agt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "AGT Compare Match A Register"]
    #[inline(always)]
    pub const fn agtcma(self) -> crate::common::Reg<regs::Agtcma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "AGT Compare Match B Register"]
    #[inline(always)]
    pub const fn agtcmb(self) -> crate::common::Reg<regs::Agtcmb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "AGT Control Register"]
    #[inline(always)]
    pub const fn agtcr(self) -> crate::common::Reg<regs::Agtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "AGT Mode Register 1"]
    #[inline(always)]
    pub const fn agtmr1(self) -> crate::common::Reg<regs::Agtmr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "AGT Mode Register 2"]
    #[inline(always)]
    pub const fn agtmr2(self) -> crate::common::Reg<regs::Agtmr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "AGT I/O Control Register"]
    #[inline(always)]
    pub const fn agtioc(self) -> crate::common::Reg<regs::Agtioc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "AGT Event Pin Select Register"]
    #[inline(always)]
    pub const fn agtisr(self) -> crate::common::Reg<regs::Agtisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "AGT Compare Match Function Select Register"]
    #[inline(always)]
    pub const fn agtcmsr(self) -> crate::common::Reg<regs::Agtcmsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "AGT Pin Select Register"]
    #[inline(always)]
    pub const fn agtiosel(self) -> crate::common::Reg<regs::Agtiosel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fusize) as _) }
    }
}
pub mod regs;
pub mod vals;
