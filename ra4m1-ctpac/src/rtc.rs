#[doc = "Realtime Clock"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "64-Hz Counter"]
    #[inline(always)]
    pub const fn r64cnt(self) -> crate::common::Reg<regs::R64cnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Binary Counter 0"]
    #[inline(always)]
    pub const fn bcnt0(self) -> crate::common::Reg<regs::Bcnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Second Counter"]
    #[inline(always)]
    pub const fn rseccnt(self) -> crate::common::Reg<regs::Rseccnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Binary Counter 1"]
    #[inline(always)]
    pub const fn bcnt1(self) -> crate::common::Reg<regs::Bcnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Minute Counter"]
    #[inline(always)]
    pub const fn rmincnt(self) -> crate::common::Reg<regs::Rmincnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Binary Counter 2"]
    #[inline(always)]
    pub const fn bcnt2(self) -> crate::common::Reg<regs::Bcnt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Hour Counter"]
    #[inline(always)]
    pub const fn rhrcnt(self) -> crate::common::Reg<regs::Rhrcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Binary Counter 3"]
    #[inline(always)]
    pub const fn bcnt3(self) -> crate::common::Reg<regs::Bcnt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Day-of-Week Counter"]
    #[inline(always)]
    pub const fn rwkcnt(self) -> crate::common::Reg<regs::Rwkcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Day Counter"]
    #[inline(always)]
    pub const fn rdaycnt(self) -> crate::common::Reg<regs::Rdaycnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Month Counter"]
    #[inline(always)]
    pub const fn rmoncnt(self) -> crate::common::Reg<regs::Rmoncnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Year Counter"]
    #[inline(always)]
    pub const fn ryrcnt(self) -> crate::common::Reg<regs::Ryrcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Binary Counter 0 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt0ar(self) -> crate::common::Reg<regs::Bcnt0ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Second Alarm Register"]
    #[inline(always)]
    pub const fn rsecar(self) -> crate::common::Reg<regs::Rsecar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Binary Counter 1 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt1ar(self) -> crate::common::Reg<regs::Bcnt1ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Minute Alarm Register"]
    #[inline(always)]
    pub const fn rminar(self) -> crate::common::Reg<regs::Rminar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Binary Counter 2 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt2ar(self) -> crate::common::Reg<regs::Bcnt2ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Hour Alarm Register"]
    #[inline(always)]
    pub const fn rhrar(self) -> crate::common::Reg<regs::Rhrar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Binary Counter 3 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt3ar(self) -> crate::common::Reg<regs::Bcnt3ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Day-of-Week Alarm Register"]
    #[inline(always)]
    pub const fn rwkar(self) -> crate::common::Reg<regs::Rwkar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Binary Counter 0 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt0aer(self) -> crate::common::Reg<regs::Bcnt0aer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Date Alarm Register"]
    #[inline(always)]
    pub const fn rdayar(self) -> crate::common::Reg<regs::Rdayar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Binary Counter 1 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt1aer(self) -> crate::common::Reg<regs::Bcnt1aer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Month Alarm Register"]
    #[inline(always)]
    pub const fn rmonar(self) -> crate::common::Reg<regs::Rmonar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Binary Counter 2 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt2aer(self) -> crate::common::Reg<regs::Bcnt2aer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Year Alarm Register"]
    #[inline(always)]
    pub const fn ryrar(self) -> crate::common::Reg<regs::Ryrar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Binary Counter 3 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt3aer(self) -> crate::common::Reg<regs::Bcnt3aer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "Year Alarm Enable Register"]
    #[inline(always)]
    pub const fn ryraren(self) -> crate::common::Reg<regs::Ryraren, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "RTC Control Register 1"]
    #[inline(always)]
    pub const fn rcr1(self) -> crate::common::Reg<regs::Rcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "RTC Control Register 2"]
    #[inline(always)]
    pub const fn rcr2(self) -> crate::common::Reg<regs::Rcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "RTC Control Register 4"]
    #[inline(always)]
    pub const fn rcr4(self) -> crate::common::Reg<regs::Rcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Frequency Register H"]
    #[inline(always)]
    pub const fn rfrh(self) -> crate::common::Reg<regs::Rfrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "Frequency Register L"]
    #[inline(always)]
    pub const fn rfrl(self) -> crate::common::Reg<regs::Rfrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Time Error Adjustment Register"]
    #[inline(always)]
    pub const fn radj(self) -> crate::common::Reg<regs::Radj, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2eusize) as _) }
    }
    #[doc = "Time Capture Control Register %s"]
    #[inline(always)]
    pub const fn rtccr(self, n: usize) -> crate::common::Reg<regs::Rtccr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 2usize) as _) }
    }
    #[doc = "BCNT0 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt0cp(self, n: usize) -> crate::common::Reg<regs::Bcnt0cp, crate::common::R> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x52usize + n * 16usize) as _) }
    }
    #[doc = "Second Capture Register %s"]
    #[inline(always)]
    pub const fn rseccp(self, n: usize) -> crate::common::Reg<regs::Rseccp, crate::common::R> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x52usize + n * 16usize) as _) }
    }
    #[doc = "BCNT1 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt1cp(self, n: usize) -> crate::common::Reg<regs::Bcnt1cp, crate::common::R> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize + n * 16usize) as _) }
    }
    #[doc = "Minute Capture Register %s"]
    #[inline(always)]
    pub const fn rmincp(self, n: usize) -> crate::common::Reg<regs::Rmincp, crate::common::R> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize + n * 16usize) as _) }
    }
    #[doc = "BCNT2 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt2cp(self, n: usize) -> crate::common::Reg<regs::Bcnt2cp, crate::common::R> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x56usize + n * 16usize) as _) }
    }
    #[doc = "Hour Capture Register %s"]
    #[inline(always)]
    pub const fn rhrcp(self, n: usize) -> crate::common::Reg<regs::Rhrcp, crate::common::R> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x56usize + n * 16usize) as _) }
    }
    #[doc = "BCNT3 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt3cp(self, n: usize) -> crate::common::Reg<regs::Bcnt3cp, crate::common::R> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5ausize + n * 16usize) as _) }
    }
    #[doc = "Date Capture Register %s"]
    #[inline(always)]
    pub const fn rdaycp(self, n: usize) -> crate::common::Reg<regs::Rdaycp, crate::common::R> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5ausize + n * 16usize) as _) }
    }
    #[doc = "Month Capture Register %s"]
    #[inline(always)]
    pub const fn rmoncp(self, n: usize) -> crate::common::Reg<regs::Rmoncp, crate::common::R> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize + n * 16usize) as _) }
    }
}
pub mod regs;
pub mod vals;
