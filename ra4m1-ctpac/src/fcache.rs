#[doc = "Flash Cache"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcache {
    ptr: *mut u8,
}
unsafe impl Send for Fcache {}
unsafe impl Sync for Fcache {}
impl Fcache {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash Cache Enable Register"]
    #[inline(always)]
    pub const fn fcachee(self) -> crate::common::Reg<regs::Fcachee, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Flash Cache Invalidate Register"]
    #[inline(always)]
    pub const fn fcacheiv(self) -> crate::common::Reg<regs::Fcacheiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Flash Wait Cycle Register"]
    #[inline(always)]
    pub const fn flwt(self) -> crate::common::Reg<regs::Flwt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
