#[doc = "Module Stop Control B,C,D"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstp {
    ptr: *mut u8,
}
unsafe impl Send for Mstp {}
unsafe impl Sync for Mstp {}
impl Mstp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Module Stop Control Register B"]
    #[inline(always)]
    pub const fn mstpcrb(self) -> crate::common::Reg<regs::Mstpcrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Module Stop Control Register C"]
    #[inline(always)]
    pub const fn mstpcrc(self) -> crate::common::Reg<regs::Mstpcrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Module Stop Control Register D"]
    #[inline(always)]
    pub const fn mstpcrd(self) -> crate::common::Reg<regs::Mstpcrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs;
pub mod vals;
