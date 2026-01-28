//! Option-Setting Memory
//!
//! This module contains macros to define the contents of the option-setting memory on the `RA4M1`.
//! Option-setting memory (OSM) contains settings for the watchdog timers, clocks, and security MPU
//! that are applied by the MCU after reset.
//! OFM is located in flash between the ARM vector table and program code+data and so is written
//! at the same time your program is flashed to the MCU.
//! OSM registers are defined as static values assigned to a special section that the linker script
//! will pick up and do very basic sanity checking of.
//!
//! For more information see §6 of the reference manual.
//!
//! ```rust,ignore
//! // Option Function Select Register 0
//! // Accepts either:
//! // - a series of configuration values
//! // - the literal `ArduinoCore` which emulates the Arduino defaults
//! #[unsafe(no_mangle)]
//! #[unsafe(link_section = ".ofs0")]
//! pub static OFS0: u32 = ofs0!(ArduinoCore);
//!
//! // Option Function Select Register 1
//! // Accepts either:
//! // - a series of configuration values
//! // - the literal `ArduinoCore` which emulates the Arduino defaults
//! #[unsafe(no_mangle)]
//! #[unsafe(link_section = ".ofs1")]
//! pub static OFS1: u32 = ofs1!(ArduinoCore);
//!
//! // Configures the Security MPU.
//! // Accepts only a value of `Disabled`.
//! #[unsafe(no_mangle)]
//! #[unsafe(link_section = ".sec_mpu")]
//! pub static SEC_MPU: SecurityMpu = sec_mpu!(Disabled);
//! ```

pub mod ofs0;
pub mod ofs1;
pub mod sec_mpu;
