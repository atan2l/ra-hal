#[doc = "Serial Communication Interface 2"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sci2 {
    ptr: *mut u8,
}
unsafe impl Send for Sci2 {}
unsafe impl Sync for Sci2 {}
impl Sci2 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Serial Mode Register (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn smr(self) -> crate::common::Reg<regs::Smr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Serial mode register (SCMR.SMIF = 1)"]
    #[inline(always)]
    pub const fn smr_smci(self) -> crate::common::Reg<regs::SmrSmci, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Bit Rate Register"]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "Serial Control Register (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Serial Control Register (SCMR.SMIF =1)"]
    #[inline(always)]
    pub const fn scr_smci(self) -> crate::common::Reg<regs::ScrSmci, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Transmit Data Register"]
    #[inline(always)]
    pub const fn tdr(self) -> crate::common::Reg<regs::Tdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)"]
    #[inline(always)]
    pub const fn ssr(self) -> crate::common::Reg<regs::Ssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Serial Status Register(SCMR.SMIF = 1)"]
    #[inline(always)]
    pub const fn ssr_smci(self) -> crate::common::Reg<regs::SsrSmci, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Receive Data Register"]
    #[inline(always)]
    pub const fn rdr(self) -> crate::common::Reg<regs::Rdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "Smart Card Mode Register"]
    #[inline(always)]
    pub const fn scmr(self) -> crate::common::Reg<regs::Scmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Serial Extended Mode Register"]
    #[inline(always)]
    pub const fn semr(self) -> crate::common::Reg<regs::Semr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "Noise Filter Setting Register"]
    #[inline(always)]
    pub const fn snfr(self) -> crate::common::Reg<regs::Snfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "I2C Mode Register 1"]
    #[inline(always)]
    pub const fn simr1(self) -> crate::common::Reg<regs::Simr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "I2C Mode Register 2"]
    #[inline(always)]
    pub const fn simr2(self) -> crate::common::Reg<regs::Simr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "I2C Mode Register 3"]
    #[inline(always)]
    pub const fn simr3(self) -> crate::common::Reg<regs::Simr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "I2C Status Register"]
    #[inline(always)]
    pub const fn sisr(self) -> crate::common::Reg<regs::Sisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SPI Mode Register"]
    #[inline(always)]
    pub const fn spmr(self) -> crate::common::Reg<regs::Spmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "Transmit 9-bit Data Register"]
    #[inline(always)]
    pub const fn tdrhl(self) -> crate::common::Reg<regs::Tdrhl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Receive 9-bit Data Register"]
    #[inline(always)]
    pub const fn rdrhl(self) -> crate::common::Reg<regs::Rdrhl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Modulation Duty Register"]
    #[inline(always)]
    pub const fn mddr(self) -> crate::common::Reg<regs::Mddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Data Compare Match Control Register"]
    #[inline(always)]
    pub const fn dccr(self) -> crate::common::Reg<regs::Dccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13usize) as _) }
    }
    #[doc = "Compare Match Data Register"]
    #[inline(always)]
    pub const fn cdr(self) -> crate::common::Reg<regs::Cdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Serial Port Register"]
    #[inline(always)]
    pub const fn sptr(self) -> crate::common::Reg<regs::Sptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
