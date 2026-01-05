#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmifrtBase {
    ptr: *mut u8,
}
unsafe impl Send for FmifrtBase {}
unsafe impl Sync for FmifrtBase {}
impl FmifrtBase {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn base(self) -> crate::common::Reg<regs::Base, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
