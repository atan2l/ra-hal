// port0 indexes: "0,1,2,3,4,5,6,7,8 10,11,12,13,14,15"
// port1 indexes: "0-15"
// P108, P109, P100 have event link
// port2 indexes: "0,1,2,3,4,5,6,12,13,14,15"
// P201 has event link
// port3 indexes: "0-7"
// P300 has event link
// port4 indexes: "0-8"
// P408 has event link
// port5 indexes: "0-5"
// port6 indexes: "0,1,2,3,8,9,10"
// port7 indexes: "8"
// port8 indexes: "8,9"
// port9 indexes: "14,15"
impl Pfs {
    #[doc = "Pin Function Control Register"]
    #[inline(always)]
    pub const fn pin(
        self,
        port: usize,
        pin: usize,
    ) -> crate::common::Reg<regs::PmnPfs, crate::common::RW> {
        assert!(port < 10);
        assert!(pin < 16);

        #[cfg(feature = "strict-assert")]
        match port {
            0 => assert!(pin <= 8 || (pin >= 10 && pin <= 15)),
            1 => {}
            2 => assert!(pin <= 6 || (pin >= 12 && pin <= 15)),
            3 => assert!(pin <= 7),
            4 => assert!(pin <= 15),
            5 => assert!(pin <= 5),
            6 => assert!(pin <= 10),
            7 => assert!(pin == 8),
            8 => assert!(pin == 8 || pin == 9),
            9 => assert!(pin == 14 || pin == 15),
            _ => unreachable!(),
        }

        self.pin_raw((port * 16) + pin)
    }
}
