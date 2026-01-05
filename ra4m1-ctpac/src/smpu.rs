#[doc = "Bus Slave MPU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpu {
    ptr: *mut u8,
}
unsafe impl Send for Smpu {}
unsafe impl Sync for Smpu {}
impl Smpu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Slave MPU Control Register"]
    #[inline(always)]
    pub const fn smpuctl(self) -> crate::common::Reg<regs::Smpuctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Access Control Register for MBIU"]
    #[inline(always)]
    pub const fn smpumbiu(self) -> crate::common::Reg<regs::Smpumbiu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Access Control Register for FBIU"]
    #[inline(always)]
    pub const fn smpufbiu(self) -> crate::common::Reg<regs::Smpufbiu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Access Control Register for SRAM0"]
    #[inline(always)]
    pub const fn smpusram0(self) -> crate::common::Reg<regs::Smpusram0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Access Control Register for P%sBIU"]
    #[inline(always)]
    pub const fn smpupbiu(self, n: usize) -> crate::common::Reg<regs::Smpupbiu, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
