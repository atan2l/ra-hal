//! Conveniences for registers that are gated by a write protect flag.

#[cfg(gpt)]
use crate::pac::gpt;
#[cfg(iic)]
use crate::pac::iic;
use crate::pac::{
    common::{RW, Reg},
    pfs::regs::Pin,
    system::vals::{Prc0, Prc1},
};

/// Manages write protection at the peripheral level.
pub trait ProtectedPeripheral {
    /// # Returns
    /// * `true` if this peripheral is currently write protected.
    fn is_protected(&self) -> bool;

    /// Disables write protection for a peripheral for the duration of the closure.
    fn protected_write<F>(&self, func: F)
    where
        F: Fn();
}

/// Provides a function to disable write protection for a single modify operation on a register.
pub trait ProtectedModify<T: Copy> {
    /// Performs a modify operation on a register, disabling WP before and enabling it after.
    fn protected_modify(&self, f: impl FnOnce(&mut T));

    /// # Returns
    /// * `true` if this register is currently write protected.
    fn is_protected(&self) -> bool {
        true
    }
}

/// Provides a function to disable write protection for a single write operation on a register.
pub trait ProtectedWrite<T: Copy> {
    /// Performs a write operation on a register, disabling WP before and enabling it after.
    fn protected_write(&self, f: impl FnOnce(&mut T));
}

#[cfg(iic)]
impl ProtectedModify<iic::regs::Icmr3> for Reg<iic::regs::Icmr3, RW> {
    #[inline]
    fn is_protected(&self) -> bool {
        !self.read().ackwp()
    }

    #[inline]
    fn protected_modify(&self, func: impl FnOnce(&mut iic::regs::Icmr3)) {
        let protected = self.is_protected();

        if protected {
            self.modify(|w| w.set_ackwp(true));
        }

        self.modify(func);

        if protected {
            self.modify(|w| w.set_ackwp(false));
        }
    }
}

impl ProtectedModify<Pin> for Reg<Pin, RW> {
    #[inline]
    fn is_protected(&self) -> bool {
        let pfs = crate::pac::PFS;
        !pfs.pwpr().read().pfswe()
    }

    fn protected_modify(&self, func: impl FnOnce(&mut Pin)) {
        // § 19.2.6

        let pfs = crate::pac::PFS;
        let protected = self.is_protected();

        if protected {
            trace!("PFS WriteProt: {}", pfs.pwpr().read());

            pfs.pwpr().modify(|w| w.set_b0wi(false));
            pfs.pwpr().modify(|w| w.set_pfswe(true));

            #[cfg(feature = "strict-assert")]
            assert!(pfs.pwpr().read().pfswe());
        }

        self.modify(func);

        if protected {
            pfs.pwpr().modify(|w| w.set_b0wi(false));
            pfs.pwpr().modify(|w| w.set_pfswe(false));

            #[cfg(feature = "strict-assert")]
            assert!(!pfs.pwpr().read().pfswe());
            trace!("PFS WriteProt: {}", pfs.pwpr().read());
        }
    }
}

impl ProtectedPeripheral for crate::pac::system::System {
    fn protected_write<F>(&self, func: F)
    where
        F: Fn(),
    {
        use crate::pac::system::vals::Prc0;

        let prcr = cfg_select! {
            all(trust_zone_v2, secure) => self.prcr_s(),
            _ => self.prcr()
        };

        let protected = self.is_protected();

        if protected {
            trace!("SYSTEM WriteProt: {}", prcr.read());
            prcr.write(|w| {
                w.set_prkey(crate::pac::system::vals::Prkey::ProtectKey);
                w.set_prc0(Prc0::NotProtected);
                w.set_prc1(Prc1::NotProtected);
            });
        }

        func();

        if protected {
            prcr.write(|w| {
                w.set_prkey(crate::pac::system::vals::Prkey::ProtectKey);
                w.set_prc0(Prc0::Protected);
                w.set_prc1(Prc1::Protected);
            });
            trace!("SYSTEM WriteProt: {}", prcr.read());
        }
    }

    #[inline]
    fn is_protected(&self) -> bool {
        let prcr = cfg_select! {
            all(trust_zone_v2, secure) => self.prcr_s(),
            _ => self.prcr()
        };

        let status = prcr.read();
        status.prc0() == Prc0::Protected || status.prc1() == Prc1::Protected
    }
}

/// Note that write protection is disabled by default for timers.
#[cfg(gpt)]
impl ProtectedPeripheral for gpt::Gpt {
    fn protected_write<F>(&self, func: F)
    where
        F: Fn(),
    {
        use gpt::vals::Prkey;

        let protected = self.is_protected();

        if protected {
            self.gtwp().write(|w| {
                w.set_wp(false);
                w.set_prkey(Prkey::ProtectKey);
            });
            trace!("WP: {}", self.gtwp().read());
        }

        func();

        if protected {
            self.gtwp().write(|w| {
                w.set_wp(true);
                w.set_prkey(Prkey::ProtectKey);
            });
            trace!("WP: {}", self.gtwp().read());
        }
    }

    #[inline]
    fn is_protected(&self) -> bool {
        self.gtwp().read().wp()
    }
}
