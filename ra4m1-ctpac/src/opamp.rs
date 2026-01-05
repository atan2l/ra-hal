#[doc = "OperationalAmplifier"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp {
    ptr: *mut u8,
}
unsafe impl Send for Opamp {}
unsafe impl Sync for Opamp {}
impl Opamp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Operational amplifier mode control register"]
    #[inline(always)]
    pub const fn ampmc(self) -> crate::common::Reg<regs::Ampmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Operational amplifier trigger mode control register"]
    #[inline(always)]
    pub const fn amptrm(self) -> crate::common::Reg<regs::Amptrm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "Operational Amplifier Activation Trigger Select Register"]
    #[inline(always)]
    pub const fn amptrs(self) -> crate::common::Reg<regs::Amptrs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Operational amplifier control register"]
    #[inline(always)]
    pub const fn ampc(self) -> crate::common::Reg<regs::Ampc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "Operational amplifier monitor register"]
    #[inline(always)]
    pub const fn ampmon(self) -> crate::common::Reg<regs::Ampmon, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
