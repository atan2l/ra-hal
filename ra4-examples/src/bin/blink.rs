#![no_std]
#![no_main]

#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
use defmt_rtt as _;
use embassy_executor::Spawner;
// use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs0")]
pub static OFS0: [u8; 4] = *b"DEAD";

#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs1")]
pub static OFS1: [u8; 4] = *b"MAUS";
