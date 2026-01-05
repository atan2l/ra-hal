#[doc = "8-bit D/A converter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac8 {
    ptr: *mut u8,
}
unsafe impl Send for Dac8 {}
unsafe impl Sync for Dac8 {}
impl Dac8 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "D/A Conversion Value Setting Register %s"]
    #[inline(always)]
    pub const fn dacs(self, n: usize) -> crate::common::Reg<regs::Dacs, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 1usize) as _) }
    }
    #[doc = "D/A Converter Mode Register"]
    #[inline(always)]
    pub const fn dam(self) -> crate::common::Reg<regs::Dam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
}
pub mod regs;
pub mod vals;
