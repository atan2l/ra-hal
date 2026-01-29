//! Option-Setting Memory
//!
//! This module contains functions to define the option-setting memory (OSM) on the `RA4M1`.
//! The OSM contains settings for the clocks, security MPU, and both watchdog timers that are
//! applied by the MCU after reset.
//! The OSM is located in flash between the ARM vector table and program code+data and so is written
//! at the same time your program is flashed to the MCU.
//! OSM registers are defined as static values assigned to a special section that the linker script
//! will pick up and do very basic sanity checking of.
//!
//! For more information see §6 of the reference manual.
//! Typical usage:
//!
//! ```rust,ignore
//! // Option Function Select Register 0
//! #[unsafe(no_mangle)]
//! #[unsafe(link_section = ".ofs0")]
//! pub static OFS0: Ofs0 = Ofs0::ArduinoCore();
//!
//! // Option Function Select Register 1
//! #[unsafe(no_mangle)]
//! #[unsafe(link_section = ".ofs1")]
//! pub static OFS1: Ofs1 = Ofs1::ArduinoCore();
//!
//! // Configures the Security MPU.
//! // Accepts only a value of `Disabled`.
//! #[unsafe(no_mangle)]
//! #[unsafe(link_section = ".sec_mpu")]
//! pub static SEC_MPU: SecurityMpu = sec_mpu!(Disabled);
//! ```
//!
//! Missing OSM settings will generate linker errors like this:
//! ```
//!  = note: rust-lld: error: .sec_mpu must be 52 bytes
//! ```

pub mod ofs0;
pub mod ofs1;
pub mod sec_mpu;
