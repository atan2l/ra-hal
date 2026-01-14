//! Conveniences for registers that are gated by a write protect flag.

#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
use ra4m1_ctpac::system::vals::Prc0;

/// Encapsulates access so that write protection is always re-enabled after a write operation.
pub trait WriteProtect {
    fn is_protected(&self) -> bool;

    fn protected_write<F>(&self, func: F)
    where
        F: Fn();
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
