//! Blink, hello world example

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
use defmt_rtt as _;
use embassy_executor::Spawner;
// use embassy_time::Timer;
use panic_probe as _;
use ra4_hal::{ofs0, ofs1};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs0")]
/// Option Function Select Register 0
/// Accepts either:
/// - a series of configuration values
/// - the literal `ArduinoCore` which emulates the Arduino defaults
pub static OFS0: u32 = ofs0!(ArduinoCore);

#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs1")]
/// Option Function Select Register 1
/// Accepts either:
/// - a series of configuration values
/// - the literal `ArduinoCore` which emulates the Arduino defaults
pub static OFS1: u32 = ofs1!(ArduinoCore);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // #define BSP_CLOCK_CFG_MAIN_OSC_WAIT (9)
    // #define BSP_LOCO_HZ                 (32768)
    // #define BSP_MOCO_HZ                 (8000000)
    // #define BSP_SUB_CLOCK_HZ            (32768)
    // #define BSP_MCU_VBATT_SUPPORT       (1)
}
