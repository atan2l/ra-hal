#[doc = "Data Operation Circuit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doc {
    ptr: *mut u8,
}
unsafe impl Send for Doc {}
unsafe impl Sync for Doc {}
impl Doc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DOC Control Register"]
    #[inline(always)]
    pub const fn docr(self) -> crate::common::Reg<regs::Docr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DOC Data Input Register"]
    #[inline(always)]
    pub const fn dodir(self) -> crate::common::Reg<regs::Dodir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "DOC Data Setting Register"]
    #[inline(always)]
    pub const fn dodsr(self) -> crate::common::Reg<regs::Dodsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs;
pub mod vals;
