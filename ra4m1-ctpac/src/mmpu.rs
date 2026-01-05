#[doc = "Bus Master MPU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpu {
    ptr: *mut u8,
}
unsafe impl Send for Mmpu {}
unsafe impl Sync for Mmpu {}
impl Mmpu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Bus Master MPU Control Register A"]
    #[inline(always)]
    pub const fn mmpuctla(self) -> crate::common::Reg<regs::Mmpuctla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Group A Protection of Register"]
    #[inline(always)]
    pub const fn mmpupta(self) -> crate::common::Reg<regs::Mmpupta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "Group A Region %s Access Control Register"]
    #[inline(always)]
    pub const fn mmpuaca(self, n: usize) -> crate::common::Reg<regs::Mmpuaca, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 16usize) as _)
        }
    }
    #[doc = "Group A Region %s Start Address Register"]
    #[inline(always)]
    pub const fn mmpusa(self, n: usize) -> crate::common::Reg<regs::Mmpusa, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize + n * 16usize) as _)
        }
    }
    #[doc = "Group A Region %s End Address Register"]
    #[inline(always)]
    pub const fn mmpuea(self, n: usize) -> crate::common::Reg<regs::Mmpuea, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize + n * 16usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
