#[doc = "USB 2.0 FS Module"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbfs {
    ptr: *mut u8,
}
unsafe impl Send for Usbfs {}
unsafe impl Sync for Usbfs {}
impl Usbfs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "System Configuration Control Register"]
    #[inline(always)]
    pub const fn syscfg(self) -> crate::common::Reg<regs::Syscfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "System Configuration Status Register 0"]
    #[inline(always)]
    pub const fn syssts0(self) -> crate::common::Reg<regs::Syssts0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Device State Control Register 0"]
    #[inline(always)]
    pub const fn dvstctr0(self) -> crate::common::Reg<regs::Dvstctr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CFIFO Port Register"]
    #[inline(always)]
    pub const fn cfifo(self) -> crate::common::Reg<regs::Cfifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "CFIFO Port Register L"]
    #[inline(always)]
    pub const fn cfifol(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "D0FIFO Port Register"]
    #[inline(always)]
    pub const fn d0fifo(self) -> crate::common::Reg<regs::D0fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "D0FIFO Port Register L"]
    #[inline(always)]
    pub const fn d0fifol(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "D1FIFO Port Register"]
    #[inline(always)]
    pub const fn d1fifo(self) -> crate::common::Reg<regs::D1fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "D1FIFO Port Register L"]
    #[inline(always)]
    pub const fn d1fifol(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "CFIFO Port Select Register"]
    #[inline(always)]
    pub const fn cfifosel(self) -> crate::common::Reg<regs::Cfifosel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "CFIFO Port Control Register"]
    #[inline(always)]
    pub const fn cfifoctr(self) -> crate::common::Reg<regs::Cfifoctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "D0FIFO Port Select Register"]
    #[inline(always)]
    pub const fn d0fifosel(self) -> crate::common::Reg<regs::D0fifosel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "D0FIFO Port Control Register"]
    #[inline(always)]
    pub const fn d0fifoctr(self) -> crate::common::Reg<regs::D0fifoctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "D1FIFO Port Select Register"]
    #[inline(always)]
    pub const fn d1fifosel(self) -> crate::common::Reg<regs::D1fifosel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "D1FIFO Port Control Register"]
    #[inline(always)]
    pub const fn d1fifoctr(self) -> crate::common::Reg<regs::D1fifoctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2eusize) as _) }
    }
    #[doc = "Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn intenb0(self) -> crate::common::Reg<regs::Intenb0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn intenb1(self) -> crate::common::Reg<regs::Intenb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "BRDY Interrupt Enable Register"]
    #[inline(always)]
    pub const fn brdyenb(self) -> crate::common::Reg<regs::Brdyenb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "NRDY Interrupt Enable Register"]
    #[inline(always)]
    pub const fn nrdyenb(self) -> crate::common::Reg<regs::Nrdyenb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "BEMP Interrupt Enable Register"]
    #[inline(always)]
    pub const fn bempenb(self) -> crate::common::Reg<regs::Bempenb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3ausize) as _) }
    }
    #[doc = "SOF Output Configuration Register"]
    #[inline(always)]
    pub const fn sofcfg(self) -> crate::common::Reg<regs::Sofcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn intsts0(self) -> crate::common::Reg<regs::Intsts0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn intsts1(self) -> crate::common::Reg<regs::Intsts1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
    #[doc = "BRDY Interrupt Status Register"]
    #[inline(always)]
    pub const fn brdysts(self) -> crate::common::Reg<regs::Brdysts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x46usize) as _) }
    }
    #[doc = "NRDY Interrupt Status Register"]
    #[inline(always)]
    pub const fn nrdysts(self) -> crate::common::Reg<regs::Nrdysts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "BEMP Interrupt Status Register"]
    #[inline(always)]
    pub const fn bempsts(self) -> crate::common::Reg<regs::Bempsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4ausize) as _) }
    }
    #[doc = "Frame Number Register"]
    #[inline(always)]
    pub const fn frmnum(self) -> crate::common::Reg<regs::Frmnum, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "USB Request Type Register"]
    #[inline(always)]
    pub const fn usbreq(self) -> crate::common::Reg<regs::Usbreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "USB Request Value Register"]
    #[inline(always)]
    pub const fn usbval(self) -> crate::common::Reg<regs::Usbval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x56usize) as _) }
    }
    #[doc = "USB Request Index Register"]
    #[inline(always)]
    pub const fn usbindx(self) -> crate::common::Reg<regs::Usbindx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "USB Request Length Register"]
    #[inline(always)]
    pub const fn usbleng(self) -> crate::common::Reg<regs::Usbleng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5ausize) as _) }
    }
    #[doc = "DCP Configuration Register"]
    #[inline(always)]
    pub const fn dcpcfg(self) -> crate::common::Reg<regs::Dcpcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "DCP Maximum Packet Size Register"]
    #[inline(always)]
    pub const fn dcpmaxp(self) -> crate::common::Reg<regs::Dcpmaxp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5eusize) as _) }
    }
    #[doc = "DCP Control Register"]
    #[inline(always)]
    pub const fn dcpctr(self) -> crate::common::Reg<regs::Dcpctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Pipe Window Select Register"]
    #[inline(always)]
    pub const fn pipesel(self) -> crate::common::Reg<regs::Pipesel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Pipe Configuration Register"]
    #[inline(always)]
    pub const fn pipecfg(self) -> crate::common::Reg<regs::Pipecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Pipe Maximum Packet Size Register"]
    #[inline(always)]
    pub const fn pipemaxp(self) -> crate::common::Reg<regs::Pipemaxp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Pipe Cycle Control Register"]
    #[inline(always)]
    pub const fn pipeperi(self) -> crate::common::Reg<regs::Pipeperi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6eusize) as _) }
    }
    #[doc = "Pipe %s Control Register"]
    #[inline(always)]
    pub const fn pipectr(self, n: usize) -> crate::common::Reg<regs::Pipectr, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize + n * 2usize) as _) }
    }
    #[doc = "Pipe %s Control Register"]
    #[inline(always)]
    pub const fn pipectr2(self, n: usize) -> crate::common::Reg<regs::Pipectr2, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7ausize + n * 2usize) as _) }
    }
    #[doc = "Pipe %s Transaction Counter Enable Register"]
    #[inline(always)]
    pub const fn pipetre(self, n: usize) -> crate::common::Reg<regs::Pipetre, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize + n * 4usize) as _) }
    }
    #[doc = "Pipe %s Transaction Counter Register"]
    #[inline(always)]
    pub const fn pipetrn(self, n: usize) -> crate::common::Reg<regs::Pipetrn, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x92usize + n * 4usize) as _) }
    }
    #[doc = "BC Control Register 0"]
    #[inline(always)]
    pub const fn usbbcctrl0(self) -> crate::common::Reg<regs::Usbbcctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "USB Module Control Register"]
    #[inline(always)]
    pub const fn usbmc(self) -> crate::common::Reg<regs::Usbmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Device Address %s Configuration Register"]
    #[inline(always)]
    pub const fn devadd(self, n: usize) -> crate::common::Reg<regs::Devadd, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize + n * 2usize) as _) }
    }
}
pub mod regs;
pub mod vals;
