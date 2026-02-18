//! `i2c`

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_hal_1::i2c::I2c as _;
use panic_probe as _;
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};
use ra4_hal::{
    i2c::{I2c, I2cSpeed},
    mode::Blocking,
    osm::{ofs0::Ofs0, ofs1::Ofs1, sec_mpu::SecurityMpu},
};

// Option Function Select Register 0 (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs0")]
static OFS0: Ofs0 = Ofs0::default();

// Option Function Select Register 1 (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ofs1")]
static OFS1: Ofs1 = Ofs1::default();

// Security MPU (required)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".sec_mpu")]
static SEC_MPU: SecurityMpu = SecurityMpu::disabled();

const CHIP_ID: u8 = 0xD1;
const I2C_ADDRESS: u8 = 0x69;

mod register {
    #![allow(unused)]

    pub(super) const CHIPID: u8 = 0x00;
    pub(super) const PMU_STATUS: u8 = 0x03;
    pub(super) const DATA: u8 = 0x04;
    pub(super) const SENSOR_TIME: u8 = 0x18;
    pub(super) const INTERRUPT_STATUS: u8 = 0x1C;
    pub(super) const TEMPERATURE: u8 = 0x20;
    pub(super) const INTERRUPT_LATCH: u8 = 0x54;
    pub(super) const INTERRUPT_INPUT_FILTER: u8 = 0x58;
    pub(super) const FIFO_LENGTH: u8 = 0x22;
    pub(super) const PMU_TRIGGER: u8 = 0x6C;
    pub(super) const NVM_CONFIG: u8 = 0x70;
    pub(super) const CMD: u8 = 0x7E;
    pub(super) const INTERRUPT_MASK: u8 = 0x50;
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra4_hal::init();

    let scl = p.P100;
    let sda = p.P101;

    // This doesn't need its own block, but this demonstrates
    // that the pins will be "released" when i2c goes out of scope.

    {
        let mut i2c = I2c::<Blocking, _>::new(p.IIC1, scl, sda, I2cSpeed::Normal);

        let mut data = [0_u8; 1];

        i2c.write_read(I2C_ADDRESS, &[register::CHIPID], &mut data)
            .ok();

        if data[0] == CHIP_ID {
            info!("Bosch BMI");
        } else {
            error!("Unknown");
        }
    }

    loop {
        Timer::after_millis(333).await;
    }
}
