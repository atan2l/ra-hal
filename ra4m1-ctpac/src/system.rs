#[doc = "System Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct System {
    ptr: *mut u8,
}
unsafe impl Send for System {}
unsafe impl Sync for System {}
impl System {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Standby Control Register"]
    #[inline(always)]
    pub const fn sbycr(self) -> crate::common::Reg<regs::Sbycr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Module Stop Control Register A"]
    #[inline(always)]
    pub const fn mstpcra(self) -> crate::common::Reg<regs::Mstpcra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "System Clock Division Control Register"]
    #[inline(always)]
    pub const fn sckdivcr(self) -> crate::common::Reg<regs::Sckdivcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "System Clock Source Control Register"]
    #[inline(always)]
    pub const fn sckscr(self) -> crate::common::Reg<regs::Sckscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "PLL Control Register"]
    #[inline(always)]
    pub const fn pllcr(self) -> crate::common::Reg<regs::Pllcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "PLL Clock Control Register2"]
    #[inline(always)]
    pub const fn pllccr2(self) -> crate::common::Reg<regs::Pllccr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2busize) as _) }
    }
    #[doc = "Memory Wait Cycle Control Register"]
    #[inline(always)]
    pub const fn memwait(self) -> crate::common::Reg<regs::Memwait, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31usize) as _) }
    }
    #[doc = "Main Clock Oscillator Control Register"]
    #[inline(always)]
    pub const fn mosccr(self) -> crate::common::Reg<regs::Mosccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "High-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn hococr(self) -> crate::common::Reg<regs::Hococr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "Middle-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn mococr(self) -> crate::common::Reg<regs::Mococr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Oscillation Stabilization Flag Register"]
    #[inline(always)]
    pub const fn oscsf(self) -> crate::common::Reg<regs::Oscsf, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Clock Out Control Register"]
    #[inline(always)]
    pub const fn ckocr(self) -> crate::common::Reg<regs::Ckocr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize) as _) }
    }
    #[doc = "Trace Clock Control Register"]
    #[inline(always)]
    pub const fn trckcr(self) -> crate::common::Reg<regs::Trckcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3fusize) as _) }
    }
    #[doc = "Oscillation Stop Detection Control Register"]
    #[inline(always)]
    pub const fn ostdcr(self) -> crate::common::Reg<regs::Ostdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Oscillation Stop Detection Status Register"]
    #[inline(always)]
    pub const fn ostdsr(self) -> crate::common::Reg<regs::Ostdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x41usize) as _) }
    }
    #[doc = "Segment LCD Source Clock Control Register"]
    #[inline(always)]
    pub const fn slcdsckcr(self) -> crate::common::Reg<regs::Slcdsckcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "MOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn mocoutcr(self) -> crate::common::Reg<regs::Mocoutcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x61usize) as _) }
    }
    #[doc = "HOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn hocoutcr(self) -> crate::common::Reg<regs::Hocoutcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x62usize) as _) }
    }
    #[doc = "Snooze Control Register"]
    #[inline(always)]
    pub const fn snzcr(self) -> crate::common::Reg<regs::Snzcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x92usize) as _) }
    }
    #[doc = "Snooze End Control Register"]
    #[inline(always)]
    pub const fn snzedcr(self) -> crate::common::Reg<regs::Snzedcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Snooze Request Control Register"]
    #[inline(always)]
    pub const fn snzreqcr(self) -> crate::common::Reg<regs::Snzreqcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Flash Operation Control Register"]
    #[inline(always)]
    pub const fn flstop(self) -> crate::common::Reg<regs::Flstop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9eusize) as _) }
    }
    #[doc = "Operating Power Control Register"]
    #[inline(always)]
    pub const fn opccr(self) -> crate::common::Reg<regs::Opccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Main Clock Oscillator Wait Control Register"]
    #[inline(always)]
    pub const fn moscwtcr(self) -> crate::common::Reg<regs::Moscwtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa2usize) as _) }
    }
    #[doc = "High-Speed On-Chip Oscillator Wait Control Register"]
    #[inline(always)]
    pub const fn hocowtcr(self) -> crate::common::Reg<regs::Hocowtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa5usize) as _) }
    }
    #[doc = "Sub Operating Power Control Register"]
    #[inline(always)]
    pub const fn sopccr(self) -> crate::common::Reg<regs::Sopccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xaausize) as _) }
    }
    #[doc = "Reset Status Register 1"]
    #[inline(always)]
    pub const fn rstsr1(self) -> crate::common::Reg<regs::Rstsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Backup Register Access Control Register"]
    #[inline(always)]
    pub const fn bkracr(self) -> crate::common::Reg<regs::Bkracr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc6usize) as _) }
    }
    #[doc = "USB Clock Control register"]
    #[inline(always)]
    pub const fn usbckcr(self) -> crate::common::Reg<regs::Usbckcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Voltage Monitor %s Circuit Control Register 1"]
    #[inline(always)]
    pub const fn lvdcr1(self, n: usize) -> crate::common::Reg<regs::Lvdcr1, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize + n * 2usize) as _) }
    }
    #[doc = "Voltage Monitor %s Circuit Status Register"]
    #[inline(always)]
    pub const fn lvdsr(self, n: usize) -> crate::common::Reg<regs::Lvdsr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe1usize + n * 2usize) as _) }
    }
    #[doc = "Protect Register"]
    #[inline(always)]
    pub const fn prcr(self) -> crate::common::Reg<regs::Prcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03feusize) as _) }
    }
    #[doc = "System Control OCD Control Register"]
    #[inline(always)]
    pub const fn syocdcr(self) -> crate::common::Reg<regs::Syocdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040eusize) as _) }
    }
    #[doc = "Reset Status Register 0"]
    #[inline(always)]
    pub const fn rstsr0(self) -> crate::common::Reg<regs::Rstsr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "Reset Status Register 2"]
    #[inline(always)]
    pub const fn rstsr2(self) -> crate::common::Reg<regs::Rstsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0411usize) as _) }
    }
    #[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
    #[inline(always)]
    pub const fn momcr(self) -> crate::common::Reg<regs::Momcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0413usize) as _) }
    }
    #[doc = "Voltage Monitor Circuit Control Register"]
    #[inline(always)]
    pub const fn lvcmpcr(self) -> crate::common::Reg<regs::Lvcmpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0417usize) as _) }
    }
    #[doc = "Voltage Detection Level Select Register"]
    #[inline(always)]
    pub const fn lvdlvlr(self) -> crate::common::Reg<regs::Lvdlvlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "Voltage Monitor %s Circuit Control Register 0"]
    #[inline(always)]
    pub const fn lvdcr0(self, n: usize) -> crate::common::Reg<regs::Lvdcr0, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041ausize + n * 1usize) as _)
        }
    }
    #[doc = "VBATT Control Register1"]
    #[inline(always)]
    pub const fn vbtcr1(self) -> crate::common::Reg<regs::Vbtcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041fusize) as _) }
    }
    #[doc = "Sub-Clock Oscillator Control Register"]
    #[inline(always)]
    pub const fn sosccr(self) -> crate::common::Reg<regs::Sosccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "Sub Clock Oscillator Mode Control Register"]
    #[inline(always)]
    pub const fn somcr(self) -> crate::common::Reg<regs::Somcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0481usize) as _) }
    }
    #[doc = "Low-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn lococr(self) -> crate::common::Reg<regs::Lococr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0490usize) as _) }
    }
    #[doc = "LOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn locoutcr(self) -> crate::common::Reg<regs::Locoutcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0492usize) as _) }
    }
    #[doc = "VBATT Control Register2"]
    #[inline(always)]
    pub const fn vbtcr2(self) -> crate::common::Reg<regs::Vbtcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b0usize) as _) }
    }
    #[doc = "VBATT Status Register"]
    #[inline(always)]
    pub const fn vbtsr(self) -> crate::common::Reg<regs::Vbtsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b1usize) as _) }
    }
    #[doc = "VBATT Comparator Control Register"]
    #[inline(always)]
    pub const fn vbtcmpcr(self) -> crate::common::Reg<regs::Vbtcmpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b2usize) as _) }
    }
    #[doc = "VBATT Pin Low Voltage Detect Interrupt Control Register"]
    #[inline(always)]
    pub const fn vbtlvdicr(self) -> crate::common::Reg<regs::Vbtlvdicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b4usize) as _) }
    }
    #[doc = "VBATT Wakeup function Control Register"]
    #[inline(always)]
    pub const fn vbtwctlr(self) -> crate::common::Reg<regs::Vbtwctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b6usize) as _) }
    }
    #[doc = "VBATT Wakeup I/O 0 Output Trigger Select Register"]
    #[inline(always)]
    pub const fn vbtwch0otsr(self) -> crate::common::Reg<regs::Vbtwch0otsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b8usize) as _) }
    }
    #[doc = "VBATT Wakeup I/O 1 Output Trigger Select Register"]
    #[inline(always)]
    pub const fn vbtwch1otsr(self) -> crate::common::Reg<regs::Vbtwch1otsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b9usize) as _) }
    }
    #[doc = "VBATT Wakeup I/O 2 Output Trigger Select Register"]
    #[inline(always)]
    pub const fn vbtwch2otsr(self) -> crate::common::Reg<regs::Vbtwch2otsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04bausize) as _) }
    }
    #[doc = "VBATT Input Control Register"]
    #[inline(always)]
    pub const fn vbtictlr(self) -> crate::common::Reg<regs::Vbtictlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04bbusize) as _) }
    }
    #[doc = "VBATT Output Control Register"]
    #[inline(always)]
    pub const fn vbtoctlr(self) -> crate::common::Reg<regs::Vbtoctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04bcusize) as _) }
    }
    #[doc = "VBATT Wakeup Trigger source Enable Register"]
    #[inline(always)]
    pub const fn vbtwter(self) -> crate::common::Reg<regs::Vbtwter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04bdusize) as _) }
    }
    #[doc = "VBATT Wakeup Trigger source Edge Register"]
    #[inline(always)]
    pub const fn vbtwegr(self) -> crate::common::Reg<regs::Vbtwegr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04beusize) as _) }
    }
    #[doc = "VBATT Wakeup trigger source Flag Register"]
    #[inline(always)]
    pub const fn vbtwfr(self) -> crate::common::Reg<regs::Vbtwfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04bfusize) as _) }
    }
    #[doc = "VBATT Backup Register \\[%s\\]"]
    #[inline(always)]
    pub const fn vbtbkr(self, n: usize) -> crate::common::Reg<regs::Vbtbkr, crate::common::RW> {
        assert!(n < 512usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 1usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
