//! `i2c_benchmark`

#![no_std]
#![no_main]

use cortex_m::asm;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Instant;
use embedded_hal_1::i2c::I2c as _;
use embedded_hal_async::i2c::I2c as _;
use panic_probe as _;
use ra_hal::{
    bind_interrupts,
    clock::ClockConfig,
    dtc::DtcInterruptHandler,
    i2c::{self, I2c, I2cSpeed},
    peripherals::{DTC_CHAN5, DTC_CHAN6, IIC1},
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

const SAMPLES: usize = 1000;
const ADDRESS: u8 = 0x69;
const BYTES: [u8; 1] = [0x00];

bind_interrupts!(struct Irqs {
    IEL2 => i2c::TxInterruptHandler<IIC1>;
    IEL3 => i2c::TeInterruptHandler<IIC1>;
    IEL4 => i2c::RxInterruptHandler<IIC1>;
    // Rx DMAC channel
    IEL5 => DtcInterruptHandler<DTC_CHAN5>;
    // Tx DMAC channel
    IEL6 => DtcInterruptHandler<DTC_CHAN6>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut p = ra_hal::init(ClockConfig::default());

    for speed in [I2cSpeed::Normal, I2cSpeed::Fast] {
        warn!("I2C: {}", speed);

        {
            let iic = p.IIC1.reborrow();
            let scl = p.P100.reborrow();
            let sda = p.P101.reborrow();
            let mut i2c = I2c::new_blocking(iic, scl, sda, speed);

            let mut samples: [f32; SAMPLES] = [0.0; SAMPLES];
            let clocks = ra_hal::clock::clock_status();

            for sample in samples.iter_mut() {
                let now = Instant::now();
                i2c.write(ADDRESS, &BYTES).ok();
                let elapsed = now.elapsed().as_ticks() as f64;
                *sample = ((elapsed / clocks.peripheral_d.to_Hz() as f64) * 1_000_000.0) as f32;
            }
            let avg = samples.iter().fold(0.0, |acc, val| acc + val) / SAMPLES as f32;
            info!("Blocking: {} µs", avg);
        }

        {
            let iic = p.IIC1.reborrow();
            let scl = p.P100.reborrow();
            let sda = p.P101.reborrow();
            let mut tx_buf = [0_u8; 16];
            let mut i2c = I2c::new_async(iic, scl, sda, speed, &mut tx_buf, Irqs);

            let mut samples: [f32; SAMPLES] = [0.0; SAMPLES];
            let clocks = ra_hal::clock::clock_status();

            for sample in samples.iter_mut() {
                let now = Instant::now();
                i2c.write(ADDRESS, &BYTES).await.ok();
                let elapsed = now.elapsed().as_ticks() as f64;
                *sample = ((elapsed / clocks.peripheral_d.to_Hz() as f64) * 1_000_000.0) as f32;
            }
            let avg = samples.iter().fold(0.0, |acc, val| acc + val) / SAMPLES as f32;
            info!("Async (INT): {} µs", avg);
        }

        {
            let iic = p.IIC1.reborrow();
            let scl = p.P100.reborrow();
            let sda = p.P101.reborrow();

            let mut i2c = I2c::new_dtc(iic, scl, sda, speed, p.DTC_CHAN5.reborrow(), Irqs);

            let mut samples: [f32; SAMPLES] = [0.0; SAMPLES];
            let clocks = ra_hal::clock::clock_status();

            for sample in samples.iter_mut() {
                let now = Instant::now();
                i2c.write(ADDRESS, &BYTES).await.ok();
                let elapsed = now.elapsed().as_ticks() as f64;
                *sample = ((elapsed / clocks.peripheral_d.to_Hz() as f64) * 1_000_000.0) as f32;
            }
            let avg = samples.iter().fold(0.0, |acc, val| acc + val) / SAMPLES as f32;
            info!("Async (DTC): {} µs", avg);
        }
    }

    loop {
        asm::nop();
    }
}
