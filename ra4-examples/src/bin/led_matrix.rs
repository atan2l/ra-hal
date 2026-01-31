//! LedMatrix example
//!
//! Displays a sequence of numbers on the matrix

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant};
use panic_probe as _;
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};
use ra4_hal::{
    osm::{ofs0::Ofs0, ofs1::Ofs1, sec_mpu::SecurityMpu},
    print_clock_config,
};
use uno_r4wifi_bsc::led_matrix_init;

// Option Function Select Register 0 (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs0")]
static OFS0: Ofs0 = Ofs0::arduino_core();

// Option Function Select Register 1 (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs1")]
static OFS1: Ofs1 = Ofs1::arduino_core();

// Security MPU (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".sec_mpu")]
static SEC_MPU: SecurityMpu = SecurityMpu::disabled();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra4_hal::init();

    let matrix = led_matrix_init!(p);

    print_clock_config();

    let digits: [[u8; 84]; 4] = [
        [
            1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, //
            1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, //
            1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, //
            1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, //
            1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, //
            1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, //
            1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, //
        ],
        [
            0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, //
            0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, //
            1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, //
            1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, //
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
            1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, //
            1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
        ],
    ];

    let mut digit = 0;
    let mut down = false;

    loop {
        let now = Instant::now();
        while now.elapsed() < Duration::from_millis(500) {
            for pixel in 0..digits[digit].len() {
                let state = digits[digit][pixel];
                if state == 1 {
                    matrix.set_pixel(pixel as u8, true).await;
                }
            }

            for row in 0..(2 * digit) {
                for pixel in [8, 9, 10] {
                    let row = 6 - row;
                    matrix.set_pixel(pixel as u8 + (row as u8 * 12), true).await;
                }
            }
        }

        if (digit == 0 && down) || (digit == digits.len() - 1) {
            down = !down
        }
        if down {
            digit -= 1;
        } else {
            digit += 1
        }
    }
}
