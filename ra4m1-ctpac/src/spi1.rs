#[doc = "Serial Peripheral Interface 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi1 {
    ptr: *mut u8,
}
unsafe impl Send for Spi1 {}
unsafe impl Sync for Spi1 {}
impl Spi1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SPI Control Register"]
    #[inline(always)]
    pub const fn spcr(self) -> crate::common::Reg<regs::Spcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SPI Slave Select Polarity Register"]
    #[inline(always)]
    pub const fn sslp(self) -> crate::common::Reg<regs::Sslp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "SPI Pin Control Register"]
    #[inline(always)]
    pub const fn sppcr(self) -> crate::common::Reg<regs::Sppcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "SPI Status Register"]
    #[inline(always)]
    pub const fn spsr(self) -> crate::common::Reg<regs::Spsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "SPI Data Register"]
    #[inline(always)]
    pub const fn spdr(self) -> crate::common::Reg<regs::Spdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SPI Data Register ( halfword access )"]
    #[inline(always)]
    pub const fn spdr_ha(self) -> crate::common::Reg<regs::SpdrHa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SPI Bit Rate Register"]
    #[inline(always)]
    pub const fn spbr(self) -> crate::common::Reg<regs::Spbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "SPI Data Control Register"]
    #[inline(always)]
    pub const fn spdcr(self) -> crate::common::Reg<regs::Spdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "SPI Clock Delay Register"]
    #[inline(always)]
    pub const fn spckd(self) -> crate::common::Reg<regs::Spckd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SPI Slave Select Negation Delay Register"]
    #[inline(always)]
    pub const fn sslnd(self) -> crate::common::Reg<regs::Sslnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "SPI Next-Access Delay Register"]
    #[inline(always)]
    pub const fn spnd(self) -> crate::common::Reg<regs::Spnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "SPI Control Register 2"]
    #[inline(always)]
    pub const fn spcr2(self) -> crate::common::Reg<regs::Spcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fusize) as _) }
    }
    #[doc = "SPI Command Register 0"]
    #[inline(always)]
    pub const fn spcmd0(self) -> crate::common::Reg<regs::Spcmd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
pub mod regs;
pub mod vals;
