//! `qdec` quadrature decoding example
//!
//! This example will use a `GPT` instance to decode the output of a quadrature encoder.  It will
//! asynchronously wait for an event and log the direction of rotation for each change.

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;
use ra_hal::{
    bind_interrupts,
    clock::ClockConfig,
    peripherals::GPT16_2,
    qdec::{self, Qdec, QdecInterruptHandler},
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

#[cfg(any(feature = "uno-r4-wifi", feature = "uno-r4-minima"))]
macro_rules! pins {
    ($p:ident) => {
        ($p.GPT16_2, $p.P103, $p.P102)
    };
}

bind_interrupts!(struct Irqs {
    IEL2 => QdecInterruptHandler<GPT16_2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());
    let (timer, chan_a, chan_b) = pins!(p);

    let qdec = Qdec::new(timer, chan_a, chan_b, qdec::Config::default(), Irqs);

    loop {
        info!("{}", qdec.read().await);
    }
}
