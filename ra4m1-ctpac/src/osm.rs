#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osm {
    ptr: *mut u8,
}
unsafe impl Send for Osm {}
unsafe impl Sync for Osm {}
impl Osm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Option Function Select Register 0 (OFS0)"]
    #[inline(always)]
    pub const fn ofs0(self) -> crate::common::Reg<regs::Ofs0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Option Function Select Register 1 (OFS1)"]
    #[inline(always)]
    pub const fn ofs1(self) -> crate::common::Reg<regs::Ofs1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs;
pub mod vals;
