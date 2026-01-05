#[doc = "Inter-Integrated Circuit 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iic0 {
    ptr: *mut u8,
}
unsafe impl Send for Iic0 {}
unsafe impl Sync for Iic0 {}
impl Iic0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "I2C Bus Control Register 1"]
    #[inline(always)]
    pub const fn iccr1(self) -> crate::common::Reg<regs::Iccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "I2C Bus Control Register 2"]
    #[inline(always)]
    pub const fn iccr2(self) -> crate::common::Reg<regs::Iccr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "I2C Bus Mode Register 1"]
    #[inline(always)]
    pub const fn icmr1(self) -> crate::common::Reg<regs::Icmr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "I2C Bus Mode Register 2"]
    #[inline(always)]
    pub const fn icmr2(self) -> crate::common::Reg<regs::Icmr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "I2C Bus Mode Register 3"]
    #[inline(always)]
    pub const fn icmr3(self) -> crate::common::Reg<regs::Icmr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "I2C Bus Function Enable Register"]
    #[inline(always)]
    pub const fn icfer(self) -> crate::common::Reg<regs::Icfer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "I2C Bus Status Enable Register"]
    #[inline(always)]
    pub const fn icser(self) -> crate::common::Reg<regs::Icser, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "I2C Bus Interrupt Enable Register"]
    #[inline(always)]
    pub const fn icier(self) -> crate::common::Reg<regs::Icier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "I2C Bus Status Register 1"]
    #[inline(always)]
    pub const fn icsr1(self) -> crate::common::Reg<regs::Icsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "I2C Bus Status Register 2"]
    #[inline(always)]
    pub const fn icsr2(self) -> crate::common::Reg<regs::Icsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "Slave Address Register L%s"]
    #[inline(always)]
    pub const fn sarl(self, n: usize) -> crate::common::Reg<regs::Sarl, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize + n * 2usize) as _) }
    }
    #[doc = "Slave Address Register U%s"]
    #[inline(always)]
    pub const fn saru(self, n: usize) -> crate::common::Reg<regs::Saru, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize + n * 2usize) as _) }
    }
    #[doc = "I2C Bus Bit Rate Low-Level Register"]
    #[inline(always)]
    pub const fn icbrl(self) -> crate::common::Reg<regs::Icbrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "I2C Bus Bit Rate High-Level Register"]
    #[inline(always)]
    pub const fn icbrh(self) -> crate::common::Reg<regs::Icbrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11usize) as _) }
    }
    #[doc = "I2C Bus Transmit Data Register"]
    #[inline(always)]
    pub const fn icdrt(self) -> crate::common::Reg<regs::Icdrt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "I2C Bus Receive Data Register"]
    #[inline(always)]
    pub const fn icdrr(self) -> crate::common::Reg<regs::Icdrr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13usize) as _) }
    }
    #[doc = "I2C Bus Wake Up Unit Register"]
    #[inline(always)]
    pub const fn icwur(self) -> crate::common::Reg<regs::Icwur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "I2C Bus Wake up Unit Register 2"]
    #[inline(always)]
    pub const fn icwur2(self) -> crate::common::Reg<regs::Icwur2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x17usize) as _) }
    }
}
pub mod regs;
pub mod vals;
