#[doc = "Direct memory access controller 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac0 {
    ptr: *mut u8,
}
unsafe impl Send for Dmac0 {}
unsafe impl Sync for Dmac0 {}
impl Dmac0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA Source Address Register"]
    #[inline(always)]
    pub const fn dmsar(self) -> crate::common::Reg<regs::Dmsar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DMA Destination Address Register"]
    #[inline(always)]
    pub const fn dmdar(self) -> crate::common::Reg<regs::Dmdar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DMA Transfer Count Register"]
    #[inline(always)]
    pub const fn dmcra(self) -> crate::common::Reg<regs::Dmcra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DMA Block Transfer Count Register"]
    #[inline(always)]
    pub const fn dmcrb(self) -> crate::common::Reg<regs::Dmcrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "DMA Transfer Mode Register"]
    #[inline(always)]
    pub const fn dmtmd(self) -> crate::common::Reg<regs::Dmtmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DMA Interrupt Setting Register"]
    #[inline(always)]
    pub const fn dmint(self) -> crate::common::Reg<regs::Dmint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13usize) as _) }
    }
    #[doc = "DMA Address Mode Register"]
    #[inline(always)]
    pub const fn dmamd(self) -> crate::common::Reg<regs::Dmamd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "DMA Offset Register"]
    #[inline(always)]
    pub const fn dmofr(self) -> crate::common::Reg<regs::Dmofr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "DMA Transfer Enable Register"]
    #[inline(always)]
    pub const fn dmcnt(self) -> crate::common::Reg<regs::Dmcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "DMA Software Start Register"]
    #[inline(always)]
    pub const fn dmreq(self) -> crate::common::Reg<regs::Dmreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1dusize) as _) }
    }
    #[doc = "DMA Status Register"]
    #[inline(always)]
    pub const fn dmsts(self) -> crate::common::Reg<regs::Dmsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
}
pub mod regs;
pub mod vals;
