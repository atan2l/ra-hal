#[doc = "Debug Function"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbg {
    ptr: *mut u8,
}
unsafe impl Send for Dbg {}
unsafe impl Sync for Dbg {}
impl Dbg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Debug Status Register"]
    #[inline(always)]
    pub const fn dbgstr(self) -> crate::common::Reg<regs::Dbgstr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Debug Stop Control Register"]
    #[inline(always)]
    pub const fn dbgstopcr(self) -> crate::common::Reg<regs::Dbgstopcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Trace Control Register"]
    #[inline(always)]
    pub const fn tracectr(self) -> crate::common::Reg<regs::Tracectr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs;
pub mod vals;
