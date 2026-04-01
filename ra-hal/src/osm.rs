//! Option-Setting Memory (`OFS0`, `OFS1`, `SECMPU`).
//!
//! This module contains functions to define the option-setting memory (OSM) on the `RA2A1` and
//! `RA4M1` at compile time.
//! The OSM contains settings for the clocks, security MPU, and both watchdog timers that are
//! applied by the MCU after reset.
//! On these chips OSM is located in flash between the ARM vector table and program code+data and
//! so is written at the same time your program is flashed to the MCU.
//! OSM registers are defined as static values assigned to a special section that the linker script
//! will pick up and do very basic sanity checking of.
//! On other chips OSM lives in a separate space and the clocks are configured by the HAL at runtime.
//!
//! For more information see §6 of the `RA4M1` reference manual.
//!
//! Typically `ra-hal` will set sensible defaults.
//! However by enabling the `skip-osm` feature you can override the defaults in your application.
//! Typical  usage:
//!
//! ```rust,ignore
//! // Option Function Select Register 0
//! #[unsafe(no_mangle)]
//! #[unsafe(link_section = ".ofs0")]
//! static OFS0: Ofs0 = Ofs0::default();
//!
//! // Option Function Select Register 1
//! #[unsafe(no_mangle)]
//! #[unsafe(link_section = ".ofs1")]
//! static OFS1: Ofs1 = Ofs1::default();
//!
//! // Configures the Security MPU.
//! // Accepts only a value of `Disabled`.
//! #[unsafe(no_mangle)]
//! #[unsafe(link_section = ".sec_mpu")]
//! static SEC_MPU: SecurityMpu = SecurityMpu::disabled();
//! ```
//!
//! Missing OSM settings will generate linker errors like this:
//! ```ignore
//!  = note: rust-lld: error: .sec_mpu must be 52 bytes
//! ```

pub mod ofs0;
pub mod ofs1;
pub mod sec_mpu;
