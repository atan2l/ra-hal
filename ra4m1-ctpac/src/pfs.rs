#[doc = "Pmn Pin Function Control Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfs {
    ptr: *mut u8,
}
unsafe impl Send for Pfs {}
unsafe impl Sync for Pfs {}
impl Pfs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PORT0 Pin Function Control Register"]
    #[inline(always)]
    pub const fn pin_inner(self, n: usize) -> crate::common::Reg<regs::PmnPfs, crate::common::RW> {
        assert!(n < 144usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;

impl Pfs {
    #[doc = "Pin Function Control Register"]
    #[inline(always)]
    pub const fn pin(
        self,
        port: usize,
        pin: usize,
    ) -> crate::common::Reg<regs::PmnPfs, crate::common::RW> {
        assert!(port < 9);
        assert!(pin < 16);

        // port0 indexes: "0,1,2,3,4,5,6,7,8 10,11,12,13,14,15"
        // port1 indexes: "0-15"
        // P108, P109, P100 have event link
        // port2 indexes: "0,1,2,3,4,5,6, 12,13,14,15"
        // P201 has event link
        // port3 indexes: "0-7"
        // P300 has event link
        // port4 indexes: "0-8"
        // P408 has event link
        // port6 indexes: "0,1,2,3,8,9,10"
        // port7 indexes: "8"
        // port8 indexes: "8, 9"

        #[cfg(feature = "strict-assert")]
        match port {
            0 => assert!(pin <= 8 || (pin >= 10 && pin <= 15)),
            1 => {}
            2 => assert!(pin <= 6 || (pin >= 12 && pin <= 15)),
            3 => assert!(pin <= 7),
            4 => assert!(pin <= 8),
            5 => {}
            6 => assert!(pin <= 10),
            7 => assert!(pin == 8),
            8 => assert!(pin == 8 || pin == 9),
            _ => unreachable!(),
        }

        self.pin_inner((port * 16) + pin)
    }
}
