//! Module stop (`MSTP`).
//!
//! The `RA4M1` supports clock gating via four (4) "module stop" registers.
//! After reset only the `DMAC`, `DTC`, and `SRAM` peripherals are enabled.
//! To enable additional peripherals use the `start_module` function.
//!
//! # Notes
//! - Some peripheral instances (e.g. `GPT`) share the same gate.
//! - See §10.4.

/// A trait that provides a consistent interface to "module stop" functionality.
#[allow(private_bounds)]
pub trait ModuleStop: SealedModuleStop {
    /// Returns `true` if the peripheral's clock is enabled.
    fn active() -> bool;

    /// Enables the peripheral clock.
    fn start_module();

    /// Disables the peripheral clock.
    fn stop_module();
}

pub(crate) trait SealedModuleStop {}
