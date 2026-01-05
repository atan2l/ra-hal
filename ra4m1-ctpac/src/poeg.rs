#[doc = "Port Output Enable Module for GPT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Poeg {
    ptr: *mut u8,
}
unsafe impl Send for Poeg {}
unsafe impl Sync for Poeg {}
impl Poeg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "POEG Group a Setting Register"]
    #[inline(always)]
    pub const fn poegg_a(self) -> crate::common::Reg<regs::Poegg, crate::common::RW> {
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + 0usize * 256usize) as _)
        }
    }
    #[doc = "POEG Group b Setting Register"]
    #[inline(always)]
    pub const fn poegg_b(self) -> crate::common::Reg<regs::Poegg, crate::common::RW> {
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + 1usize * 256usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
