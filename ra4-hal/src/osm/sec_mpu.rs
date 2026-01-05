//! Security MPU Registers.

/// §15.6: The secure program is executed in the memory space defined by the `SECMPUPCSn` and
/// `SECMPUPCEn` registers and can access the secure data specified in the `SECMPUSm` and `SECMPUEm`
/// (m = 0 to 3) registers.
/// Address space of greater than 12 bytes is required between the last instruction of a non-secure
/// program and the first instruction of a secure program.
#[repr(C)]
#[non_exhaustive]
pub struct SecurityMpu {
    /// Code segment 0 (`SECMPUCS0`, `SECMPUCE0`)
    pub pc0: [u32; 2],

    /// Code segment 1 (`SECMPUCS1`, `SECMPUCE1`)
    pub pc1: [u32; 2],

    /// Secure RAM region 0 (`SECMPUS0`, `SECMPUE0`)
    pub region0: [u32; 2],

    /// Secure RAM region 1 (`SECMPUS1`, `SECMPUE1`)
    pub region1: [u32; 2],

    /// Secure RAM region 2 (`SECMPUS2`, `SECMPUE2`)
    pub region2: [u32; 2],

    /// Secure RAM region 3 (`SECMPUS3`, `SECMPUE3`)
    pub region3: [u32; 2],

    /// Access Control register (`SECMPUAC`)
    ///
    /// If you zero this out you will have a bad time.
    /// Leave at `0xffff_ffff`.
    /// If you *must* change it, read §15.6, §15.6.1.11.
    pub access_control: u32,
}

impl SecurityMpu {
    /// Initializes the Security MPU to a disabled state.
    pub const fn disabled() -> Self {
        Self {
            pc0: [0x00fffffc, 0x00ffffff],
            pc1: [0x00fffffc, 0x00ffffff],
            region0: [0x00fffffc, 0x00ffffff],
            region1: [0x200ffffc, 0x200fffff],
            region2: [0x407ffffc, 0x407fffff],
            region3: [0x400dfffc, 0x400dffff],
            access_control: 0xffffffff,
        }
    }
}
