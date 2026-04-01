//! Watchdog drivers.
//!
//! The `RA4M1` has two watchdog timers available for use: Independent Watchdog Timer ([`IWDT`](iwdt)) and Watchdog Timer ([`WDT`](wdt)).
//! `IWDT` is driven by a wholly independent clock and can only be configured at reset-time via [`OSM`](module@crate::osm).
//! `WDT` is driven by [`PCLKB`](struct@crate::ClockConfig) and can be configured either at reset-time via `OSM` or at runtime via its registers.

pub mod iwdt;
pub mod wdt;

/// Which action to take when the watchdog counter underflows.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Action {
    /// Trigger an NMI on watchdog underflow.
    Interrupt,

    /// Trigger a software reset on watchdog underflow.
    Reset,

    /// Trigger a software interrupt on watchdog underflow, use with `new_handler`.
    Handler,
}
