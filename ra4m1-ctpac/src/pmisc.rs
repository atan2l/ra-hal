#[doc = "Miscellaneous Port Control Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmisc {
    ptr: *mut u8,
}
unsafe impl Send for Pmisc {}
unsafe impl Sync for Pmisc {}
impl Pmisc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Write-Protect Register"]
    #[inline(always)]
    pub const fn pwpr(self) -> crate::common::Reg<regs::Pwpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
}
pub mod regs;
pub mod vals;
