#[doc = "Temperature Sensor"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsn {
    ptr: *mut u8,
}
unsafe impl Send for Tsn {}
unsafe impl Sync for Tsn {}
impl Tsn {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Temperature Sensor Calibration Data Register L"]
    #[inline(always)]
    pub const fn tscdrl(self) -> crate::common::Reg<regs::Tscdrl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Temperature Sensor Calibration Data Register H"]
    #[inline(always)]
    pub const fn tscdrh(self) -> crate::common::Reg<regs::Tscdrh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0229usize) as _) }
    }
}
pub mod regs;
