#[doc = "12-bit D/A converter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac12 {
    ptr: *mut u8,
}
unsafe impl Send for Dac12 {}
unsafe impl Sync for Dac12 {}
impl Dac12 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "D/A Data Register 0"]
    #[inline(always)]
    pub const fn dadr0(self) -> crate::common::Reg<regs::Dadr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "D/A Control Register"]
    #[inline(always)]
    pub const fn dacr(self) -> crate::common::Reg<regs::Dacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DADR0 Format Select Register"]
    #[inline(always)]
    pub const fn dadpr(self) -> crate::common::Reg<regs::Dadpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "D/A-A/D Synchronous Start Control Register"]
    #[inline(always)]
    pub const fn daadscr(self) -> crate::common::Reg<regs::Daadscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "D/A VREF Control Register"]
    #[inline(always)]
    pub const fn davrefcr(self) -> crate::common::Reg<regs::Davrefcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
}
pub mod regs;
pub mod vals;
