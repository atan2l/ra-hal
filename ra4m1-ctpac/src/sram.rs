#[doc = "SRAM Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sram {
    ptr: *mut u8,
}
unsafe impl Send for Sram {}
unsafe impl Sync for Sram {}
impl Sram {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SRAM Parity Error Operation After Detection Register"]
    #[inline(always)]
    pub const fn parioad(self) -> crate::common::Reg<regs::Parioad, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SRAM Protection Register"]
    #[inline(always)]
    pub const fn sramprcr(self) -> crate::common::Reg<regs::Sramprcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "ECC Operating Mode Control Register"]
    #[inline(always)]
    pub const fn eccmode(self) -> crate::common::Reg<regs::Eccmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "ECC 2-Bit Error Status Register"]
    #[inline(always)]
    pub const fn ecc2sts(self) -> crate::common::Reg<regs::Ecc2sts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc1usize) as _) }
    }
    #[doc = "ECC 1-Bit Error Information Update Enable Register"]
    #[inline(always)]
    pub const fn ecc1stsen(self) -> crate::common::Reg<regs::Ecc1stsen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc2usize) as _) }
    }
    #[doc = "ECC 1-Bit Error Status Register"]
    #[inline(always)]
    pub const fn ecc1sts(self) -> crate::common::Reg<regs::Ecc1sts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc3usize) as _) }
    }
    #[doc = "ECC Protection Register"]
    #[inline(always)]
    pub const fn eccprcr(self) -> crate::common::Reg<regs::Eccprcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "ECC Protection Register 2"]
    #[inline(always)]
    pub const fn eccprcr2(self) -> crate::common::Reg<regs::Eccprcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "ECC Test Control Register"]
    #[inline(always)]
    pub const fn eccetst(self) -> crate::common::Reg<regs::Eccetst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "SRAM ECC Error Operation After Detection Register"]
    #[inline(always)]
    pub const fn eccoad(self) -> crate::common::Reg<regs::Eccoad, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
