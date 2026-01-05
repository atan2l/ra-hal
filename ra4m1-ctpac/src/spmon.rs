#[doc = "CPU Stack Pointer Monitor"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spmon {
    ptr: *mut u8,
}
unsafe impl Send for Spmon {}
unsafe impl Sync for Spmon {}
impl Spmon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Stack Pointer Monitor Operation After Detection Register"]
    #[inline(always)]
    pub const fn mspmpuoad(self) -> crate::common::Reg<regs::Mspmpuoad, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Stack Pointer Monitor Access Control Register"]
    #[inline(always)]
    pub const fn mspmpuctl(self) -> crate::common::Reg<regs::Mspmpuctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Stack Pointer Monitor Protection Register"]
    #[inline(always)]
    pub const fn mspmpupt(self) -> crate::common::Reg<regs::Mspmpupt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Main Stack Pointer (MSP) Monitor Start Address Register"]
    #[inline(always)]
    pub const fn mspmpusa(self) -> crate::common::Reg<regs::Mspmpusa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Main Stack Pointer (MSP) Monitor End Address Register"]
    #[inline(always)]
    pub const fn mspmpuea(self) -> crate::common::Reg<regs::Mspmpuea, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Stack Pointer Monitor Operation After Detection Register"]
    #[inline(always)]
    pub const fn pspmpuoad(self) -> crate::common::Reg<regs::Pspmpuoad, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Stack Pointer Monitor Access Control Register"]
    #[inline(always)]
    pub const fn pspmpuctl(self) -> crate::common::Reg<regs::Pspmpuctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Stack Pointer Monitor Protection Register"]
    #[inline(always)]
    pub const fn pspmpupt(self) -> crate::common::Reg<regs::Pspmpupt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Process Stack Pointer (PSP) Monitor Start Address Register"]
    #[inline(always)]
    pub const fn pspmpusa(self) -> crate::common::Reg<regs::Pspmpusa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Process Stack Pointer (PSP) Monitor End Address Register"]
    #[inline(always)]
    pub const fn pspmpuea(self) -> crate::common::Reg<regs::Pspmpuea, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
