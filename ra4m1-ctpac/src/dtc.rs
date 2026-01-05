#[doc = "Data Transfer Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtc {
    ptr: *mut u8,
}
unsafe impl Send for Dtc {}
unsafe impl Sync for Dtc {}
impl Dtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DTC Control Register"]
    #[inline(always)]
    pub const fn dtccr(self) -> crate::common::Reg<regs::Dtccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DTC Vector Base Register"]
    #[inline(always)]
    pub const fn dtcvbr(self) -> crate::common::Reg<regs::Dtcvbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DTC Module Start Register"]
    #[inline(always)]
    pub const fn dtcst(self) -> crate::common::Reg<regs::Dtcst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "DTC Status Register"]
    #[inline(always)]
    pub const fn dtcsts(self) -> crate::common::Reg<regs::Dtcsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
}
pub mod regs;
pub mod vals;
