#[doc = "Key Interrupt Function"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kint {
    ptr: *mut u8,
}
unsafe impl Send for Kint {}
unsafe impl Sync for Kint {}
impl Kint {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "KEY Return Control Register"]
    #[inline(always)]
    pub const fn krctl(self) -> crate::common::Reg<regs::Krctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "KEY Return Flag Register"]
    #[inline(always)]
    pub const fn krf(self) -> crate::common::Reg<regs::Krf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "KEY Return Mode Register"]
    #[inline(always)]
    pub const fn krm(self) -> crate::common::Reg<regs::Krm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs;
pub mod vals;
