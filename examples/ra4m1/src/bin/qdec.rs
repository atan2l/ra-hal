//! `qdec` quadrature decoding example
//!
//! This example will use a `GPT` instance to decode the output of a quadrature encoder.  It will
//! asynchronously wait for an event and log the direction of rotation for each change.

#![no_std]
#![no_main]

use assign_resources::assign_resources;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_hal_internal::Peri;
use panic_probe as _;
use ra_hal::{
    bind_interrupts,
    clock::ClockConfig,
    peripherals::{self, GPT16_2},
    qdec::{self, Qdec, QdecInterruptHandler},
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

cfg_select! {
    any(feature = "uno-r4-minima", feature = "uno-r4-wifi") => {
        assign_resources! {
            qdec: DecoderResources {
                chan_a: P103,
                chan_b: P102,
                timer: GPT16_2,
            }
        }
    }
    _ => {
        compile_error!(
            "Ensure the pin and timer assignments are correct for your board before continuing."
        );
    }
}

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
    let r = split_resources!(p);

    let qdec = Qdec::new(
        r.qdec.timer,
        r.qdec.chan_a,
        r.qdec.chan_b,
        qdec::Config::default(),
        Irqs,
    );

    loop {
        info!("{}", qdec.read().await);
    }
}
