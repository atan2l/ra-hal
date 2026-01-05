#[doc = "BUS Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bus {
    ptr: *mut u8,
}
unsafe impl Send for Bus {}
unsafe impl Sync for Bus {}
impl Bus {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Master Bus Control Register m4i"]
    #[inline(always)]
    pub const fn busmcnt_m4i(self) -> crate::common::Reg<regs::Busmcnt, crate::common::RW> {
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize + 0usize * 4usize) as _)
        }
    }
    #[doc = "Master Bus Control Register m4d"]
    #[inline(always)]
    pub const fn busmcnt_m4d(self) -> crate::common::Reg<regs::Busmcnt, crate::common::RW> {
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize + 1usize * 4usize) as _)
        }
    }
    #[doc = "Master Bus Control Register sys"]
    #[inline(always)]
    pub const fn busmcnt_sys(self) -> crate::common::Reg<regs::Busmcnt, crate::common::RW> {
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize + 2usize * 4usize) as _)
        }
    }
    #[doc = "Master Bus Control Register dma"]
    #[inline(always)]
    pub const fn busmcnt_dma(self) -> crate::common::Reg<regs::Busmcnt, crate::common::RW> {
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize + 3usize * 4usize) as _)
        }
    }
    #[doc = "Slave Bus Control Register FLI"]
    #[inline(always)]
    pub const fn busscntfli(self) -> crate::common::Reg<regs::Busscntfli, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1100usize) as _) }
    }
    #[doc = "Slave Bus Control Register mbiu"]
    #[inline(always)]
    pub const fn busscnt_mbiu(self) -> crate::common::Reg<regs::Busscnt, crate::common::RW> {
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize + 0usize * 4usize) as _)
        }
    }
    #[doc = "Slave Bus Control Register ram0"]
    #[inline(always)]
    pub const fn busscnt_ram0(self) -> crate::common::Reg<regs::Busscnt, crate::common::RW> {
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize + 1usize * 4usize) as _)
        }
    }
    #[doc = "Slave Bus Control Register p0b"]
    #[inline(always)]
    pub const fn busscnt2_p0b(self) -> crate::common::Reg<regs::Busscnt2, crate::common::RW> {
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1114usize + 0usize * 4usize) as _)
        }
    }
    #[doc = "Slave Bus Control Register p2b"]
    #[inline(always)]
    pub const fn busscnt2_p2b(self) -> crate::common::Reg<regs::Busscnt2, crate::common::RW> {
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1114usize + 1usize * 4usize) as _)
        }
    }
    #[doc = "Slave Bus Control Register p3b"]
    #[inline(always)]
    pub const fn busscnt2_p3b(self) -> crate::common::Reg<regs::Busscnt2, crate::common::RW> {
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1114usize + 2usize * 4usize) as _)
        }
    }
    #[doc = "Slave Bus Control Register p4b"]
    #[inline(always)]
    pub const fn busscnt2_p4b(self) -> crate::common::Reg<regs::Busscnt2, crate::common::RW> {
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1114usize + 3usize * 4usize) as _)
        }
    }
    #[doc = "Slave Bus Control Register P6B"]
    #[inline(always)]
    pub const fn busscntp6b(self) -> crate::common::Reg<regs::Busscntp6b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "Slave Bus Control Register FBU"]
    #[inline(always)]
    pub const fn busscntfbu(self) -> crate::common::Reg<regs::Busscntfbu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1130usize) as _) }
    }
    #[doc = "Bus Error Address Register %s"]
    #[inline(always)]
    pub const fn buserradd(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Buserradd, crate::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1800usize + n * 16usize) as _)
        }
    }
    #[doc = "Bus Error Status Register %s"]
    #[inline(always)]
    pub const fn buserrstat(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Buserrstat, crate::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1804usize + n * 16usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
