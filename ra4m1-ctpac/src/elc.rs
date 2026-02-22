#[doc = "Event Link Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elc {
    ptr: *mut u8,
}
unsafe impl Send for Elc {}
unsafe impl Sync for Elc {}
impl Elc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Event Link Controller Register"]
    #[inline(always)]
    pub const fn elcr(self) -> crate::common::Reg<regs::Elcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Event Link Software Event Generation Register %s"]
    #[inline(always)]
    pub const fn elsegr(self, n: usize) -> crate::common::Reg<regs::Elsegr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize + n * 2usize) as _) }
    }
    #[doc = "Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn elsr(self, n: usize) -> crate::common::Reg<regs::Elsr, crate::common::RW> {
        assert!(n < 19usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
