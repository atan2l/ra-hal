//! `spi_benchmark` SPI example that writes a arrays with different word sizes out in a loop and reports the mean time taken.

#![no_std]
#![no_main]

use assign_resources::assign_resources;
use cortex_m::asm;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Instant;
use panic_probe as _;
use ra_hal::{
    Peri, bind_interrupts,
    clock::ClockConfig,
    dmac::{self, DmacInterruptHandler},
    dtc::{self, DtcInterruptHandler},
    peripherals::{self, DMAC0, DMAC1, DTC_CHAN5, DTC_CHAN6, SPI0},
    spi::{self, Spi, TeInterruptHandler},
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

const SAMPLES: usize = 1000;
const SIZE: usize = 228;

cfg_select! {
    feature = "uno-r4-minima" => {
        // Define the pins we want on the R4 Minima
        // Note: SPI1 will not work without the "swd-as-gpio" feature.
        // Note: SPI0 will work with alternative pins that are not at the locations Arduino labels as "SPI".
        assign_resources! {
            spi: SpiResources {
                peri: SPI0,
                sck: P102,
                mosi: P101,
                miso: P100,
                cs: P103,
            }
        }
    },
    feature = "uno-r4-wifi" => {
        // Define the pins we want on the R4 WiFi
        assign_resources! {
            spi: SpiResources {
                peri: SPI0,
                sck: P102,
                mosi: P411,
                miso: P410,
                cs: P103,
            }
        }
    }
    _ => {
        compile_error!(
            "Ensure the pin and timer assignments are correct for your board before continuing."
        );
    }
}

bind_interrupts!(struct Irqs {
    // Tx DMAC channel
    IEL3 => DmacInterruptHandler<DMAC0>;
    // Rx DMAC channel
    IEL4 => DmacInterruptHandler<DMAC1>;
    // Tx DTC channel
    IEL5 => DtcInterruptHandler<DTC_CHAN5>;
    // Rx DTC channel
    IEL6 => DtcInterruptHandler<DTC_CHAN6>;
    // SPI transfer finished
    IEL7 => TeInterruptHandler<SPI0>;
});

async fn benchmark_dma<
    'd,
    const SIZE: usize,
    const SAMPLES: usize,
    W: spi::Word + dmac::Word,
    I: spi::Instance,
>(
    bus: &mut Spi<'d, I, W, spi::Dma<'d>>,
    o_val: W,
    i_val: W,
) -> f32 {
    let clocks = ra_hal::clock::clock_status();
    let mut durations: [f32; SAMPLES] = [0.0; SAMPLES];
    let output = [o_val; SIZE];
    let mut input = [i_val; SIZE];

    for sample in durations.iter_mut() {
        let now = Instant::now();
        bus.dma_transfer(&mut input, &output).await.unwrap();
        let elapsed = now.elapsed().as_ticks() as f64;
        *sample = ((elapsed / clocks.peripheral_d.to_Hz() as f64) * 1_000_000.0) as f32;
    }

    durations.iter().fold(0.0, |acc, val| acc + val) / SAMPLES as f32
}

async fn benchmark_dtc<
    'd,
    const SIZE: usize,
    const SAMPLES: usize,
    W: spi::Word + dtc::Word,
    I: spi::Instance,
    D1: dtc::Instance,
    D2: dtc::Instance,
>(
    bus: &mut Spi<'d, I, W, spi::Dtc<D1, D2>>,
    o_val: W,
    i_val: W,
) -> f32 {
    let clocks = ra_hal::clock::clock_status();
    let mut durations: [f32; SAMPLES] = [0.0; SAMPLES];
    let output = [o_val; SIZE];
    let mut input = [i_val; SIZE];

    for sample in durations.iter_mut() {
        let now = Instant::now();
        bus.dtc_transfer(&mut input, &output).await.unwrap();
        let elapsed = now.elapsed().as_ticks() as f64;
        *sample = ((elapsed / clocks.peripheral_d.to_Hz() as f64) * 1_000_000.0) as f32;
    }

    durations.iter().fold(0.0, |acc, val| acc + val) / SAMPLES as f32
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut p = ra_hal::init(ClockConfig::default());
    let r = split_resources!(p);

    let mut config = spi::Config::default();
    config.bit_rate = 24_000_000;

    let SpiResources {
        peri: mut spi,
        mut sck,
        mut mosi,
        mut miso,
        mut cs,
    } = r.spi;

    info!(
        "n={}, bytes={}, bit_rate={}",
        SAMPLES, SIZE, config.bit_rate
    );

    info!("DMAC:");

    {
        let mut bus = Spi::new_dma(
            spi.reborrow(),
            sck.reborrow(),
            mosi.reborrow(),
            miso.reborrow(),
            cs.reborrow(),
            config,
            p.DMAC0.reborrow(),
            p.DMAC1.reborrow(),
            Irqs,
        );
        const SIZE: usize = crate::SIZE / 1;
        let avg = benchmark_dma::<SIZE, SAMPLES, u8, _>(&mut bus, 0xA5, 0x5A).await;
        info!("8-bit: {} µs", avg);
    }

    {
        let mut bus = Spi::new_dma(
            spi.reborrow(),
            sck.reborrow(),
            mosi.reborrow(),
            miso.reborrow(),
            cs.reborrow(),
            config,
            p.DMAC0.reborrow(),
            p.DMAC1.reborrow(),
            Irqs,
        );
        const SIZE: usize = crate::SIZE / 2;
        let avg = benchmark_dma::<SIZE, SAMPLES, u16, _>(&mut bus, 0xA5A5, 0x5A5A).await;
        info!("16-bit: {} µs", avg);
    }

    {
        let mut bus = Spi::new_dma(
            spi.reborrow(),
            sck.reborrow(),
            mosi.reborrow(),
            miso.reborrow(),
            cs.reborrow(),
            config,
            p.DMAC0.reborrow(),
            p.DMAC1.reborrow(),
            Irqs,
        );
        const SIZE: usize = crate::SIZE / 4;
        let avg = benchmark_dma::<SIZE, SAMPLES, u32, _>(&mut bus, 0xA5A5A5A5, 0x5A5A5A5A).await;
        info!("32-bit: {} µs", avg);
    }

    info!("DTC:");

    {
        let mut bus = Spi::new_dtc(
            spi.reborrow(),
            sck.reborrow(),
            mosi.reborrow(),
            miso.reborrow(),
            cs.reborrow(),
            config,
            p.DTC_CHAN5.reborrow(),
            p.DTC_CHAN6.reborrow(),
            Irqs,
        );
        const SIZE: usize = crate::SIZE / 1;
        let avg = benchmark_dtc::<SIZE, SAMPLES, u8, _, _, _>(&mut bus, 0xA5, 0x5A).await;
        info!("8-bit: {} µs", avg);
    }

    {
        let mut bus = Spi::new_dtc(
            spi.reborrow(),
            sck.reborrow(),
            mosi.reborrow(),
            miso.reborrow(),
            cs.reborrow(),
            config,
            p.DTC_CHAN5.reborrow(),
            p.DTC_CHAN6.reborrow(),
            Irqs,
        );
        const SIZE: usize = crate::SIZE / 2;
        let avg = benchmark_dtc::<SIZE, SAMPLES, u16, _, _, _>(&mut bus, 0xA5A5, 0x5A5A).await;
        info!("16-bit: {} µs", avg);
    }

    {
        let mut bus = Spi::new_dtc(
            spi.reborrow(),
            sck.reborrow(),
            mosi.reborrow(),
            miso.reborrow(),
            cs.reborrow(),
            config,
            p.DTC_CHAN5.reborrow(),
            p.DTC_CHAN6.reborrow(),
            Irqs,
        );
        const SIZE: usize = crate::SIZE / 4;
        let avg =
            benchmark_dtc::<SIZE, SAMPLES, u32, _, _, _>(&mut bus, 0xA5A5A5A5, 0x5A5A5A5A).await;
        info!("32-bit: {} µs", avg);
    }

    loop {
        asm::nop()
    }
}
