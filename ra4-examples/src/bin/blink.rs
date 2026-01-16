//! Blink, hello world example

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};
use ra4_hal::{gpio::Flex, ofs0, ofs1, print_clock_config};

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

    print_clock_config();

    // static const int pin_zero_index = 28;
    // static const uint8_t pins[][2] = {

    //   { 7, 3 }, // 0
    // { BSP_IO_PORT_02_PIN_05,    P205   }, /* (35) D35  */
    // { BSP_IO_PORT_00_PIN_11,    P011   }, /* (30) D30  */
    // { BSP_IO_PORT_00_PIN_12,    P012   }, /* (31) D31  */
    // { BSP_IO_PORT_00_PIN_13,    P013   }, /* (32) D32  */
    // let pfs = pac::PFS;
    // let port0 = pac::PORT0;
    // let port2 = pac::PORT2;

    // port2.pcntr1().write(|w| {
    //     w.set_pdr(5, true);
    //     w.set_podr(5, true);
    // });
    // port0.pcntr1().write(|w| {
    //     w.set_pdr(12, true);
    //     w.set_pdr(13, true);
    //     // w.set_pdr(pac::port0::vals::Pcntr1Pdr::from_bits(1 << 12 | 1 << 13));
    //     // w.set_pdr(pac::port0::vals::Pcntr1Pdr::from_bits(1 << 11));
    //     // w.set_podr(pac::port0::vals::Pcntr1Podr::from_bits(1 << 12));
    // });

    let mut pin0 = Flex::new(p.P205);
    let mut pin1 = Flex::new(p.P012);
    let mut pin2 = Flex::new(p.P013);

    pin0.set_as_output();
    pin1.set_as_output();
    pin2.set_as_output();

    pin0.set_high();
    pin1.set_low();
    pin2.set_low();

    let mut state = false;

    loop {
        if state {
            pin1.set_as_output();
            pin2.set_as_input();
        } else {
            pin2.set_as_output();
            pin1.set_as_input();
        }

        state = !state;
        Timer::after_millis(333).await;
    }
}
