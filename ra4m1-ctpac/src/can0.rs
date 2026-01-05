#[doc = "CAN0 Module"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can0 {
    ptr: *mut u8,
}
unsafe impl Send for Can0 {}
unsafe impl Sync for Can0 {}
impl Can0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Mailbox Register"]
    #[inline(always)]
    pub const fn mb_id(self, n: usize) -> crate::common::Reg<regs::MbId, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 16usize) as _)
        }
    }
    #[doc = "Mailbox Register"]
    #[inline(always)]
    pub const fn mb_dl(self, n: usize) -> crate::common::Reg<regs::MbDl, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize + n * 16usize) as _)
        }
    }
    #[doc = "Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d0(self, n: usize) -> crate::common::Reg<regs::MbD0, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0206usize + n * 16usize) as _)
        }
    }
    #[doc = "Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d1(self, n: usize) -> crate::common::Reg<regs::MbD1, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0207usize + n * 16usize) as _)
        }
    }
    #[doc = "Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d2(self, n: usize) -> crate::common::Reg<regs::MbD2, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize + n * 16usize) as _)
        }
    }
    #[doc = "Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d3(self, n: usize) -> crate::common::Reg<regs::MbD3, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0209usize + n * 16usize) as _)
        }
    }
    #[doc = "Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d4(self, n: usize) -> crate::common::Reg<regs::MbD4, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020ausize + n * 16usize) as _)
        }
    }
    #[doc = "Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d5(self, n: usize) -> crate::common::Reg<regs::MbD5, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020busize + n * 16usize) as _)
        }
    }
    #[doc = "Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d6(self, n: usize) -> crate::common::Reg<regs::MbD6, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize + n * 16usize) as _)
        }
    }
    #[doc = "Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d7(self, n: usize) -> crate::common::Reg<regs::MbD7, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020dusize + n * 16usize) as _)
        }
    }
    #[doc = "Mailbox Register"]
    #[inline(always)]
    pub const fn mb_ts(self, n: usize) -> crate::common::Reg<regs::MbTs, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020eusize + n * 16usize) as _)
        }
    }
    #[doc = "Mask Register"]
    #[inline(always)]
    pub const fn mkr(self, n: usize) -> crate::common::Reg<regs::Mkr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "FIFO Received ID Compare Registers"]
    #[inline(always)]
    pub const fn fidcr(self, n: usize) -> crate::common::Reg<regs::Fidcr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize + n * 4usize) as _)
        }
    }
    #[doc = "Mask Invalid Register"]
    #[inline(always)]
    pub const fn mkivlr(self) -> crate::common::Reg<regs::Mkivlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "Mailbox Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mier(self) -> crate::common::Reg<regs::Mier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "Mailbox Interrupt Enable Register for FIFO Mailbox Mode"]
    #[inline(always)]
    pub const fn mier_fifo(self) -> crate::common::Reg<regs::MierFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "Message Control Register for Receive"]
    #[inline(always)]
    pub const fn mctl_rx(self, n: usize) -> crate::common::Reg<regs::MctlRx, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0820usize + n * 1usize) as _)
        }
    }
    #[doc = "Message Control Register for Transmit"]
    #[inline(always)]
    pub const fn mctl_tx(self, n: usize) -> crate::common::Reg<regs::MctlTx, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0820usize + n * 1usize) as _)
        }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0840usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn str(self) -> crate::common::Reg<regs::Str, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0842usize) as _) }
    }
    #[doc = "Bit Configuration Register"]
    #[inline(always)]
    pub const fn bcr(self) -> crate::common::Reg<regs::Bcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0844usize) as _) }
    }
    #[doc = "Receive FIFO Control Register"]
    #[inline(always)]
    pub const fn rfcr(self) -> crate::common::Reg<regs::Rfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0848usize) as _) }
    }
    #[doc = "Receive FIFO Pointer Control Register"]
    #[inline(always)]
    pub const fn rfpcr(self) -> crate::common::Reg<regs::Rfpcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0849usize) as _) }
    }
    #[doc = "Transmit FIFO Control Register"]
    #[inline(always)]
    pub const fn tfcr(self) -> crate::common::Reg<regs::Tfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x084ausize) as _) }
    }
    #[doc = "Transmit FIFO Pointer Control Register"]
    #[inline(always)]
    pub const fn tfpcr(self) -> crate::common::Reg<regs::Tfpcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x084busize) as _) }
    }
    #[doc = "Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn eier(self) -> crate::common::Reg<regs::Eier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x084cusize) as _) }
    }
    #[doc = "Error Interrupt Factor Judge Register"]
    #[inline(always)]
    pub const fn eifr(self) -> crate::common::Reg<regs::Eifr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x084dusize) as _) }
    }
    #[doc = "Receive Error Count Register"]
    #[inline(always)]
    pub const fn recr(self) -> crate::common::Reg<regs::Recr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x084eusize) as _) }
    }
    #[doc = "Transmit Error Count Register"]
    #[inline(always)]
    pub const fn tecr(self) -> crate::common::Reg<regs::Tecr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x084fusize) as _) }
    }
    #[doc = "Error Code Store Register"]
    #[inline(always)]
    pub const fn ecsr(self) -> crate::common::Reg<regs::Ecsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0850usize) as _) }
    }
    #[doc = "Channel Search Support Register"]
    #[inline(always)]
    pub const fn cssr(self) -> crate::common::Reg<regs::Cssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0851usize) as _) }
    }
    #[doc = "Mailbox Search Status Register"]
    #[inline(always)]
    pub const fn mssr(self) -> crate::common::Reg<regs::Mssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0852usize) as _) }
    }
    #[doc = "Mailbox Search Mode Register"]
    #[inline(always)]
    pub const fn msmr(self) -> crate::common::Reg<regs::Msmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0853usize) as _) }
    }
    #[doc = "Time Stamp Register"]
    #[inline(always)]
    pub const fn tsr(self) -> crate::common::Reg<regs::Tsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0854usize) as _) }
    }
    #[doc = "Acceptance Filter Support Register"]
    #[inline(always)]
    pub const fn afsr(self) -> crate::common::Reg<regs::Afsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0856usize) as _) }
    }
    #[doc = "Test Control Register"]
    #[inline(always)]
    pub const fn tcr(self) -> crate::common::Reg<regs::Tcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0858usize) as _) }
    }
}
pub mod regs;
pub mod vals;
