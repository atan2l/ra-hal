//! `i2c_async`

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_hal_async::i2c::I2c as _;
use panic_probe as _;
use ra_hal::{
    bind_interrupts,
    clock::ClockConfig,
    i2c::{I2c, I2cSpeed, RxInterruptHandler, TeInterruptHandler, TxInterruptHandler},
    peripherals::IIC1,
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

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

bind_interrupts!(struct Irqs {
    IEL2 => TxInterruptHandler<IIC1>;
    IEL3 => TeInterruptHandler<IIC1>;
    IEL4 => RxInterruptHandler<IIC1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());

    let scl = p.P100;
    let sda = p.P101;

    {
        let mut tx_buf = [0_u8; 16];
        let mut i2c = I2c::new_async(p.IIC1, scl, sda, I2cSpeed::Fast, &mut tx_buf, Irqs);

        let mut data = [0_u8; 1];

        i2c.write_read(I2C_ADDRESS, &[register::CHIPID], &mut data)
            .await
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
