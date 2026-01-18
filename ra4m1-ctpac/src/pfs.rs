#[doc = "Pmn Pin Function Control Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfs {
    ptr: *mut u8,
}
unsafe impl Send for Pfs {}
unsafe impl Sync for Pfs {}
impl Pfs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PORT0 Pin Function Control Register"]
    #[inline(always)]
    pub const fn port0pfs(self, n: usize) -> crate::common::Reg<regs::Port0Pfs, crate::common::RW> {
        assert!(n < 15usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "PORT1 Pin Function Control Register"]
    #[inline(always)]
    pub const fn port1pfs(self, n: usize) -> crate::common::Reg<regs::Port0Pfs, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs(self) -> crate::common::Reg<regs::P108pfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs_ha(self) -> crate::common::Reg<regs::P108pfsHa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x62usize) as _) }
    }
    #[doc = "P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs_by(self) -> crate::common::Reg<regs::P108pfsBy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x63usize) as _) }
    }
    #[doc = "P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs(self) -> crate::common::Reg<regs::P109pfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs_ha(self) -> crate::common::Reg<regs::P109pfsHa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x66usize) as _) }
    }
    #[doc = "P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs_by(self) -> crate::common::Reg<regs::P109pfsBy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x67usize) as _) }
    }
    #[doc = "PORT2 Pin Function Control Register"]
    #[inline(always)]
    pub const fn port2pfs(self, n: usize) -> crate::common::Reg<regs::Port0Pfs, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs(self) -> crate::common::Reg<regs::P201pfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs_ha(self) -> crate::common::Reg<regs::P201pfsHa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x86usize) as _) }
    }
    #[doc = "P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs_by(self) -> crate::common::Reg<regs::P201pfsBy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x87usize) as _) }
    }
    #[doc = "PORT3 Pin Function Control Register"]
    #[inline(always)]
    pub const fn port3pfs(self, n: usize) -> crate::common::Reg<regs::Port0Pfs, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "PORT4 Pin Function Control Register"]
    #[inline(always)]
    pub const fn port4pfs(self, n: usize) -> crate::common::Reg<regs::Port0Pfs, crate::common::RW> {
        assert!(n < 9usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "P408 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs(self) -> crate::common::Reg<regs::P408pfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "P408 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs_ha(self) -> crate::common::Reg<regs::P408pfsHa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0122usize) as _) }
    }
    #[doc = "P408 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs_by(self) -> crate::common::Reg<regs::P408pfsBy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0123usize) as _) }
    }
    #[doc = "PORT5 Pin Function Control Register"]
    #[inline(always)]
    pub const fn port5pfs(self, n: usize) -> crate::common::Reg<regs::Port0Pfs, crate::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
        }
    }
    #[doc = "PORT6 Pin Function Control Register"]
    #[inline(always)]
    pub const fn port6pfs(self, n: usize) -> crate::common::Reg<regs::Port0Pfs, crate::common::RW> {
        assert!(n < 11usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize + n * 4usize) as _)
        }
    }
    #[doc = "PORT7 Pin Function Control Register"]
    #[inline(always)]
    pub const fn port7pfs(self, n: usize) -> crate::common::Reg<regs::Port0Pfs, crate::common::RW> {
        assert!(n < 9usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize + n * 4usize) as _)
        }
    }
    #[doc = "PORT8 Pin Function Control Register"]
    #[inline(always)]
    pub const fn port8pfs(self, n: usize) -> crate::common::Reg<regs::Port0Pfs, crate::common::RW> {
        assert!(n < 10usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
