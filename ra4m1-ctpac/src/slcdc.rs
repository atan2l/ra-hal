#[doc = "Segment LCD Controller/Driver"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slcdc {
    ptr: *mut u8,
}
unsafe impl Send for Slcdc {}
unsafe impl Sync for Slcdc {}
impl Slcdc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "LCD Mode Register 0"]
    #[inline(always)]
    pub const fn lcdm0(self) -> crate::common::Reg<regs::Lcdm0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "LCD Mode Register 1"]
    #[inline(always)]
    pub const fn lcdm1(self) -> crate::common::Reg<regs::Lcdm1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "LCD Clock Control Register 0"]
    #[inline(always)]
    pub const fn lcdc0(self) -> crate::common::Reg<regs::Lcdc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "LCD Boost Level Control Register"]
    #[inline(always)]
    pub const fn vlcd(self) -> crate::common::Reg<regs::Vlcd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "LCD Display Data Register %s"]
    #[inline(always)]
    pub const fn seg(self, n: usize) -> crate::common::Reg<regs::Seg, crate::common::RW> {
        assert!(n < 38usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 1usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
