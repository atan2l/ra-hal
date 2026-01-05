#[doc = "General PWM Timer 0 (32-bit)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt320 {
    ptr: *mut u8,
}
unsafe impl Send for Gpt320 {}
unsafe impl Sync for Gpt320 {}
impl Gpt320 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "General PWM Timer Write-Protection Register"]
    #[inline(always)]
    pub const fn gtwp(self) -> crate::common::Reg<regs::Gtwp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "General PWM Timer Software Start Register"]
    #[inline(always)]
    pub const fn gtstr(self) -> crate::common::Reg<regs::Gtstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "General PWM Timer Software Stop Register"]
    #[inline(always)]
    pub const fn gtstp(self) -> crate::common::Reg<regs::Gtstp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "General PWM Timer Software Clear Register"]
    #[inline(always)]
    pub const fn gtclr(self) -> crate::common::Reg<regs::Gtclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "General PWM Timer Start Source Select Register"]
    #[inline(always)]
    pub const fn gtssr(self) -> crate::common::Reg<regs::Gtssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "General PWM Timer Stop Source Select Register"]
    #[inline(always)]
    pub const fn gtpsr(self) -> crate::common::Reg<regs::Gtpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "General PWM Timer Clear Source Select Register"]
    #[inline(always)]
    pub const fn gtcsr(self) -> crate::common::Reg<regs::Gtcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "General PWM Timer Up Count Source Select Register"]
    #[inline(always)]
    pub const fn gtupsr(self) -> crate::common::Reg<regs::Gtupsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "General PWM Timer Down Count Source Select Register"]
    #[inline(always)]
    pub const fn gtdnsr(self) -> crate::common::Reg<regs::Gtdnsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "General PWM Timer Input Capture Source Select Register A"]
    #[inline(always)]
    pub const fn gticasr(self) -> crate::common::Reg<regs::Gticasr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "General PWM Timer Input Capture Source Select Register B"]
    #[inline(always)]
    pub const fn gticbsr(self) -> crate::common::Reg<regs::Gticbsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "General PWM Timer Control Register"]
    #[inline(always)]
    pub const fn gtcr(self) -> crate::common::Reg<regs::Gtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "General PWM Timer Count Direction and Duty Setting Register"]
    #[inline(always)]
    pub const fn gtuddtyc(self) -> crate::common::Reg<regs::Gtuddtyc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "General PWM Timer I/O Control Register"]
    #[inline(always)]
    pub const fn gtior(self) -> crate::common::Reg<regs::Gtior, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "General PWM Timer Interrupt Output Setting Register"]
    #[inline(always)]
    pub const fn gtintad(self) -> crate::common::Reg<regs::Gtintad, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "General PWM Timer Status Register"]
    #[inline(always)]
    pub const fn gtst(self) -> crate::common::Reg<regs::Gtst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "General PWM Timer Buffer Enable Register"]
    #[inline(always)]
    pub const fn gtber(self) -> crate::common::Reg<regs::Gtber, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "General PWM Timer Counter"]
    #[inline(always)]
    pub const fn gtcnt(self) -> crate::common::Reg<regs::Gtcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "General PWM Timer Compare Capture Register A"]
    #[inline(always)]
    pub const fn gtccra(self) -> crate::common::Reg<regs::Gtccra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "General PWM Timer Compare Capture Register B"]
    #[inline(always)]
    pub const fn gtccrb(self) -> crate::common::Reg<regs::Gtccrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "General PWM Timer Compare Capture Register C"]
    #[inline(always)]
    pub const fn gtccrc(self) -> crate::common::Reg<regs::Gtccrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "General PWM Timer Compare Capture Register E"]
    #[inline(always)]
    pub const fn gtccre(self) -> crate::common::Reg<regs::Gtccre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "General PWM Timer Compare Capture Register D"]
    #[inline(always)]
    pub const fn gtccrd(self) -> crate::common::Reg<regs::Gtccrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "General PWM Timer Compare Capture Register F"]
    #[inline(always)]
    pub const fn gtccrf(self) -> crate::common::Reg<regs::Gtccrf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "General PWM Timer Cycle Setting Register"]
    #[inline(always)]
    pub const fn gtpr(self) -> crate::common::Reg<regs::Gtpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "General PWM Timer Cycle Setting Buffer Register"]
    #[inline(always)]
    pub const fn gtpbr(self) -> crate::common::Reg<regs::Gtpbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "General PWM Timer Dead Time Control Register"]
    #[inline(always)]
    pub const fn gtdtcr(self) -> crate::common::Reg<regs::Gtdtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "General PWM Timer Dead Time Value Register U"]
    #[inline(always)]
    pub const fn gtdvu(self) -> crate::common::Reg<regs::Gtdvu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
