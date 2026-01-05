#[doc = "Clock Frequency Accuracy Measurement Circuit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cac {
    ptr: *mut u8,
}
unsafe impl Send for Cac {}
unsafe impl Sync for Cac {}
impl Cac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CAC Control Register 0"]
    #[inline(always)]
    pub const fn cacr0(self) -> crate::common::Reg<regs::Cacr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "CAC Control Register 1"]
    #[inline(always)]
    pub const fn cacr1(self) -> crate::common::Reg<regs::Cacr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "CAC Control Register 2"]
    #[inline(always)]
    pub const fn cacr2(self) -> crate::common::Reg<regs::Cacr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "CAC Interrupt Control Register"]
    #[inline(always)]
    pub const fn caicr(self) -> crate::common::Reg<regs::Caicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "CAC Status Register"]
    #[inline(always)]
    pub const fn castr(self) -> crate::common::Reg<regs::Castr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CAC Upper-Limit Value Setting Register"]
    #[inline(always)]
    pub const fn caulvr(self) -> crate::common::Reg<regs::Caulvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "CAC Lower-Limit Value Setting Register"]
    #[inline(always)]
    pub const fn callvr(self) -> crate::common::Reg<regs::Callvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CAC Counter Buffer Register"]
    #[inline(always)]
    pub const fn cacntbr(self) -> crate::common::Reg<regs::Cacntbr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
}
pub mod regs;
pub mod vals;
