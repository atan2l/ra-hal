#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmifrt {
    ptr: *mut u8,
}
unsafe impl Send for Fmifrt {}
unsafe impl Sync for Fmifrt {}
impl Fmifrt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn uidr(self, n: usize) -> crate::common::Reg<regs::Uid, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn pnr(self, n: usize) -> crate::common::Reg<regs::PartNumber, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn mcuver(self) -> crate::common::Reg<regs::McuVersion, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
}
pub mod regs;
