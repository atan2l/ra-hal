#[doc = "Low-Power Analog Comparator"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acmplp {
    ptr: *mut u8,
}
unsafe impl Send for Acmplp {}
unsafe impl Sync for Acmplp {}
impl Acmplp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ACMPLP Mode Setting Register"]
    #[inline(always)]
    pub const fn compmdr(self) -> crate::common::Reg<regs::Compmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "ACMPLP Filter Control Register"]
    #[inline(always)]
    pub const fn compfir(self) -> crate::common::Reg<regs::Compfir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "ACMPLP Output Control Register"]
    #[inline(always)]
    pub const fn compocr(self) -> crate::common::Reg<regs::Compocr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Comparator Input Select Register"]
    #[inline(always)]
    pub const fn compsel0(self) -> crate::common::Reg<regs::Compsel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Comparator Reference Voltage Select Register"]
    #[inline(always)]
    pub const fn compsel1(self) -> crate::common::Reg<regs::Compsel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
}
pub mod regs;
pub mod vals;
