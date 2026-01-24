//! Conveniences for registers that are gated by a write protect flag.

use ra4m1_ctpac::system::vals::Prc0;

// TODO: Implement this at the register level
/// Encapsulates access so that write protection is always re-enabled after a write operation.
pub trait WriteProtect {
    /// # Returns
    /// * `true` if this peripheral is currently write protected.
    fn is_protected(&self) -> bool;

    /// Disables write protection for a peripheral for the duration of the closure.
    fn protected_write<F>(&self, func: F)
    where
        F: Fn();
}

impl WriteProtect for crate::pac::pfs::Pfs {
    fn is_protected(&self) -> bool {
        let pmisc = crate::pac::PMISC;
        !pmisc.pwpr().read().pfswe()
    }

    fn protected_write<F>(&self, func: F)
    where
        F: Fn(),
    {
        // § 19.2.6

        let protected = self.is_protected();
        let pmisc = crate::pac::PMISC;

        if protected {
            trace!("PFS WriteProt: {}", pmisc.pwpr().read());

            pmisc.pwpr().write(|w| {
                w.set_b0wi(false);
                w.set_pfswe(true);
            });
        }

        func();

        if protected {
            pmisc.pwpr().write(|w| {
                w.set_b0wi(false);
                w.set_pfswe(false);
            });

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
impl WriteProtect for crate::pac::gpt32::Gpt32 {
    fn protected_write<F>(&self, func: F)
    where
        F: Fn(),
    {
        use ra4m1_ctpac::gpt32::vals::Prkey;

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
