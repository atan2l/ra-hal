#[doc = "Output Phase Switching Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GptOps {
    ptr: *mut u8,
}
unsafe impl Send for GptOps {}
unsafe impl Sync for GptOps {}
impl GptOps {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Output Phase Switching Control Register"]
    #[inline(always)]
    pub const fn opscr(self) -> crate::common::Reg<regs::Opscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
