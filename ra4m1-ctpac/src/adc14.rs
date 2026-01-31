#[doc = "14bit A/D Converter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc14 {
    ptr: *mut u8,
}
unsafe impl Send for Adc14 {}
unsafe impl Sync for Adc14 {}
impl Adc14 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "A/D Control Register"]
    #[inline(always)]
    pub const fn adcsr(self) -> crate::common::Reg<regs::Adcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "A/D Channel Select Register A0"]
    #[inline(always)]
    pub const fn adansa0(self) -> crate::common::Reg<regs::Adansa0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "A/D Channel Select Register A1"]
    #[inline(always)]
    pub const fn adansa1(self) -> crate::common::Reg<regs::Adansa1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "A/D-Converted Value Addition/Average Channel Select Register 0"]
    #[inline(always)]
    pub const fn adads0(self) -> crate::common::Reg<regs::Adads0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "A/D-Converted Value Addition/Average Channel Select Register 1"]
    #[inline(always)]
    pub const fn adads1(self) -> crate::common::Reg<regs::Adads1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "A/D-Converted Value Addition/Average Count Select Register"]
    #[inline(always)]
    pub const fn adadc(self) -> crate::common::Reg<regs::Adadc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "A/D Control Extended Register"]
    #[inline(always)]
    pub const fn adcer(self) -> crate::common::Reg<regs::Adcer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "A/D Conversion Start Trigger Select Register"]
    #[inline(always)]
    pub const fn adstrgr(self) -> crate::common::Reg<regs::Adstrgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "A/D Conversion Extended Input Control Register"]
    #[inline(always)]
    pub const fn adexicr(self) -> crate::common::Reg<regs::Adexicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "A/D Channel Select Register B0"]
    #[inline(always)]
    pub const fn adansb0(self) -> crate::common::Reg<regs::Adansb0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "A/D Channel Select Register B1"]
    #[inline(always)]
    pub const fn adansb1(self) -> crate::common::Reg<regs::Adansb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "A/D Data Duplication Register"]
    #[inline(always)]
    pub const fn addbldr(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "A/D Temperature Sensor Data Register"]
    #[inline(always)]
    pub const fn adtsdr(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "A/D Internal Reference Voltage Data Register"]
    #[inline(always)]
    pub const fn adocdr(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "A/D Self-Diagnosis Data Register"]
    #[inline(always)]
    pub const fn adrd(self) -> crate::common::Reg<regs::Adrd, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "A/D Data Register %s"]
    #[inline(always)]
    pub const fn addr(self, n: usize) -> crate::common::Reg<u16, crate::common::R> {
        assert!(n < 15usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 2usize) as _) }
    }
    #[doc = "A/D Data Register %s"]
    #[inline(always)]
    pub const fn addr2(self, n: usize) -> crate::common::Reg<u16, crate::common::R> {
        assert!(n < 10usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 2usize) as _) }
    }
    #[doc = "A/D Disconnection Detection Control Register"]
    #[inline(always)]
    pub const fn addiscr(self) -> crate::common::Reg<regs::Addiscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7ausize) as _) }
    }
    #[doc = "A/D Group Scan Priority Control Register"]
    #[inline(always)]
    pub const fn adgspcr(self) -> crate::common::Reg<regs::Adgspcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "A/D Data Duplexing Register A"]
    #[inline(always)]
    pub const fn addbldra(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "A/D Data Duplexing Register B"]
    #[inline(always)]
    pub const fn addbldrb(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x86usize) as _) }
    }
    #[doc = "A/D High-Potential/Low-Potential Reference Voltage Control Register"]
    #[inline(always)]
    pub const fn adhvrefcnt(self) -> crate::common::Reg<regs::Adhvrefcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8ausize) as _) }
    }
    #[doc = "A/D Compare Function Window A/B Status Monitor Register"]
    #[inline(always)]
    pub const fn adwinmon(self) -> crate::common::Reg<regs::Adwinmon, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "A/D Compare Function Control Register"]
    #[inline(always)]
    pub const fn adcmpcr(self) -> crate::common::Reg<regs::Adcmpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "A/D Compare Function Window A Extended Input Select Register"]
    #[inline(always)]
    pub const fn adcmpanser(self) -> crate::common::Reg<regs::Adcmpanser, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x92usize) as _) }
    }
    #[doc = "A/D Compare Function Window A Extended Input Comparison Condition Setting Register"]
    #[inline(always)]
    pub const fn adcmpler(self) -> crate::common::Reg<regs::Adcmpler, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x93usize) as _) }
    }
    #[doc = "A/D Compare Function Window A Channel Select Register 0"]
    #[inline(always)]
    pub const fn adcmpansr0(self) -> crate::common::Reg<regs::Adcmpansr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "A/D Compare Function Window A Channel Select Register 1"]
    #[inline(always)]
    pub const fn adcmpansr1(self) -> crate::common::Reg<regs::Adcmpansr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x96usize) as _) }
    }
    #[doc = "A/D Compare Function Window A Comparison Condition Setting Register 0"]
    #[inline(always)]
    pub const fn adcmplr0(self) -> crate::common::Reg<regs::Adcmplr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "A/D Compare Function Window A Comparison Condition Setting Register 1"]
    #[inline(always)]
    pub const fn adcmplr1(self) -> crate::common::Reg<regs::Adcmplr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9ausize) as _) }
    }
    #[doc = "A/D Compare Function Window A Lower-Side Level Setting Register"]
    #[inline(always)]
    pub const fn adcmpdr0(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "A/D Compare Function Window A Upper-Side Level Setting Register"]
    #[inline(always)]
    pub const fn adcmpdr1(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9eusize) as _) }
    }
    #[doc = "A/D Compare Function Window A Channel Status Register 0"]
    #[inline(always)]
    pub const fn adcmpsr0(self) -> crate::common::Reg<regs::Adcmpsr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "A/D Compare Function Window A Channel Status Register 1"]
    #[inline(always)]
    pub const fn adcmpsr1(self) -> crate::common::Reg<regs::Adcmpsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa2usize) as _) }
    }
    #[doc = "A/D Compare Function Window A Extended Input Channel Status Register"]
    #[inline(always)]
    pub const fn adcmpser(self) -> crate::common::Reg<regs::Adcmpser, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "A/D Compare Function Window B Channel Selection Register"]
    #[inline(always)]
    pub const fn adcmpbnsr(self) -> crate::common::Reg<regs::Adcmpbnsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa6usize) as _) }
    }
    #[doc = "A/D Compare Function Window B Lower-Side Level Setting Register"]
    #[inline(always)]
    pub const fn adwinllb(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "A/D Compare Function Window B Upper-Side Level Setting Register"]
    #[inline(always)]
    pub const fn adwinulb(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xaausize) as _) }
    }
    #[doc = "A/D Compare Function Window B Status Register"]
    #[inline(always)]
    pub const fn adcmpbsr(self) -> crate::common::Reg<regs::Adcmpbsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "A/D Sampling State Register L"]
    #[inline(always)]
    pub const fn adsstrl(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xddusize) as _) }
    }
    #[doc = "A/D Sampling State Register T"]
    #[inline(always)]
    pub const fn adsstrt(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdeusize) as _) }
    }
    #[doc = "A/D Sampling State Register O"]
    #[inline(always)]
    pub const fn adsstro(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdfusize) as _) }
    }
    #[doc = "A/D Sampling State Register %s"]
    #[inline(always)]
    pub const fn adsstr(self, n: usize) -> crate::common::Reg<u8, crate::common::RW> {
        assert!(n < 15usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize + n * 1usize) as _) }
    }
}
pub mod regs;
pub mod vals;
