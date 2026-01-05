//! Conveniences for registers that are gated by a write protect flag.

use ra4m1_ctpac::{
    common::{RW, Reg},
    iic::regs::Icmr3,
    pfs::regs::PmnPfs,
    system::vals::Prc0,
};

/// Manages write protection at the peripheral level.
pub trait WriteProtect {
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
}

/// Provides a function to disable write protection for a single write operation on a register.
pub trait ProtectedWrite<T: Copy> {
    /// Performs a write operation on a register, disabling WP before and enabling it after.
    fn protected_write(&self, f: impl FnOnce(&mut T));
}

impl ProtectedModify<Icmr3> for Reg<Icmr3, RW> {
    #[inline]
    fn protected_modify(&self, func: impl FnOnce(&mut Icmr3)) {
        self.modify(|w| w.set_ackwp(true));
        self.modify(func)
    }
}

impl ProtectedModify<PmnPfs> for Reg<PmnPfs, RW> {
    fn protected_modify(&self, func: impl FnOnce(&mut PmnPfs)) {
        // § 19.2.6

        let pmisc = crate::pac::PMISC;
        let protected = !pmisc.pwpr().read().pfswe();

        if protected {
            trace!("PFS WriteProt: {}", pmisc.pwpr().read());

            pmisc.pwpr().modify(|w| w.set_b0wi(false));
            pmisc.pwpr().modify(|w| w.set_pfswe(true));

            #[cfg(feature = "strict-assert")]
            assert!(pmisc.pwpr().read().pfswe());
        }

        self.modify(func);

        if protected {
            pmisc.pwpr().modify(|w| w.set_b0wi(false));
            pmisc.pwpr().modify(|w| w.set_pfswe(false));

            #[cfg(feature = "strict-assert")]
            assert!(!pmisc.pwpr().read().pfswe());
            trace!("PFS WriteProt: {}", pmisc.pwpr().read());
        }
    }
}

impl WriteProtect for crate::pac::system::System {
    fn protected_write<F>(&self, func: F)
    where
        F: Fn(),
    {
        use ra4m1_ctpac::system::vals::Prc0;

        let protected = self.is_protected();

        if protected {
            trace!("SYSTEM WriteProt: {}", self.prcr().read());
            self.prcr().write(|w| {
                w.set_prkey(crate::pac::system::vals::Prkey::PROTECT_KEY);
                w.set_prc0(Prc0::NotProtected);
            });
        }

        func();

        if protected {
            self.prcr().write(|w| {
                w.set_prkey(crate::pac::system::vals::Prkey::PROTECT_KEY);
                w.set_prc0(Prc0::Protected);
            });
            trace!("SYSTEM WriteProt: {}", self.prcr().read());
        }
    }

    #[inline]
    fn is_protected(&self) -> bool {
        self.prcr().read().prc0() == Prc0::Protected
    }
}

/// Note that write protection is disabled by default for timers.
impl WriteProtect for crate::pac::gpt::Gpt {
    fn protected_write<F>(&self, func: F)
    where
        F: Fn(),
    {
        use ra4m1_ctpac::gpt::vals::Prkey;

        let protected = self.is_protected();

        if protected {
            self.gtwp().write(|w| {
                w.set_wp(false);
                w.set_prkey(Prkey::_0X_A5);
            });
            trace!("WP: {}", self.gtwp().read());
        }

        func();

        if protected {
            self.gtwp().write(|w| {
                w.set_wp(true);
                w.set_prkey(Prkey::_0X_A5);
            });
            trace!("WP: {}", self.gtwp().read());
        }
    }

    #[inline]
    fn is_protected(&self) -> bool {
        self.gtwp().read().wp()
    }
}
