#[doc = "CRC Calculator"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crc {
    ptr: *mut u8,
}
unsafe impl Send for Crc {}
unsafe impl Sync for Crc {}
impl Crc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CRC Control Register0"]
    #[inline(always)]
    pub const fn crccr0(self) -> crate::common::Reg<regs::Crccr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "CRC Control Register1"]
    #[inline(always)]
    pub const fn crccr1(self) -> crate::common::Reg<regs::Crccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "CRC Data Input Register"]
    #[inline(always)]
    pub const fn crcdir(self) -> crate::common::Reg<regs::Crcdir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CRC Data Input Register (byte access)"]
    #[inline(always)]
    pub const fn crcdir_by(self) -> crate::common::Reg<regs::CrcdirBy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CRC Data Output Register"]
    #[inline(always)]
    pub const fn crcdor(self) -> crate::common::Reg<regs::Crcdor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CRC Data Output Register(byte access)"]
    #[inline(always)]
    pub const fn crcdor_by(self) -> crate::common::Reg<regs::CrcdorBy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CRC Data Output Register (halfword access)"]
    #[inline(always)]
    pub const fn crcdor_ha(self) -> crate::common::Reg<regs::CrcdorHa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Snoop Address Register"]
    #[inline(always)]
    pub const fn crcsar(self) -> crate::common::Reg<regs::Crcsar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
