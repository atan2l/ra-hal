//! `qdec` quadrature decoding example

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;
use ra4_hal::{
    bind_interrupts,
    peripherals::GPT16_2,
    qdec::{self, Qdec, QdecInterruptHandler},
};
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};

// Define the pins we want on the R4 Minima
#[cfg(feature = "uno-r4-minima")]
macro_rules! pins {
    ($p:ident) => {
        $p.P111
    };
}

// Define the pins we want on the R4 WiFi
#[cfg(feature = "uno-r4-wifi")]
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
    let p = ra4_hal::init();
    let (timer, chan_a, chan_b) = pins!(p);

    let qdec = Qdec::new(timer, chan_a, chan_b, qdec::Config::default(), Irqs);

    loop {
        info!("{}", qdec.read().await);
    }
}
