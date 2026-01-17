//! Blink, hello world example

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
use ra4_hal::{ofs0, ofs1, print_clock_config};
use uno_r4wifi_bsc::led_matrix_init;

/// Option Function Select Register 0
/// Accepts either:
/// - a series of configuration values
/// - the literal `ArduinoCore` which emulates the Arduino defaults
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs0")]
pub static OFS0: u32 = ofs0!(ArduinoCore);

/// Option Function Select Register 1
/// Accepts either:
/// - a series of configuration values
/// - the literal `ArduinoCore` which emulates the Arduino defaults
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs1")]
pub static OFS1: u32 = ofs1!(ArduinoCore);

/// Configures the Security MPU.  See the reference manual for more details.
/// Setting all bits to 1 would also work.  Setting all bits to 0 is a good way to brick your board.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".sec_mpu")]
pub static SEC_MPU: [u32; 13] = [
    0x00fffffc, 0x00ffffff, 0x00fffffc, 0x00ffffff, 0x00fffffc, 0x00ffffff, 0x200ffffc, 0x200fffff,
    0x407ffffc, 0x407fffff, 0x400dfffc, 0x400dffff, 0xffffffff,
];

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
