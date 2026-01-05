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
