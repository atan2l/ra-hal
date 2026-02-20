//! `gpio_interrupt` GPIO input example (interrupt)
//!
//! Connect a button between D12 and ground on an Uno R4 and this will toggle the builtin LED
//! when the button is pressed.
//!
//! This version will wait for an input interrupt.
//! Only certain pins are able to trigger interrupts.
//! To see a version that polls the PORT peripheral (which works on all pins) see `gpio_poll`.

#![no_std]
#![no_main]
#![warn(missing_docs)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;
use ra4_hal::{
    bind_interrupts,
    gpio::{
        Debounce, DriveCapacity, GpioTrigger, InputInterruptHandler, InterruptFlex, Level, Output,
    },
};
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};

// Defines the pins we want on the R4 Minima
#[cfg(feature = "uno-r4-minima")]
macro_rules! pins {
    ($p:ident) => {
        ($p.P110, $p.P111)
    };
}

// Defines the pins we want on the R4 WiFi
#[cfg(feature = "uno-r4-wifi")]
macro_rules! pins {
    ($p:ident) => {
        ($p.P410, $p.P102)
    };
}

#[cfg(feature = "uno-r4-wifi")]
bind_interrupts!(struct Irqs {
    IEL2 => InputInterruptHandler<ra4_hal::peripherals::P410>;
});

#[cfg(feature = "uno-r4-minima")]
bind_interrupts!(struct Irqs {
    IEL2 => InputInterruptHandler<ra4_hal::peripherals::P110>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra4_hal::init();

    // Grab D12 and the LED pins
    let (button, led) = pins!(p);

    let mut button = InterruptFlex::new(button, p.GPIO_IRQ5, Irqs);
    button.set_pull_up(true);
    button.set_trigger(GpioTrigger::Both);
    button.set_debounce(Debounce::Min8);

    let mut led = Output::new_basic(led, Level::Low, DriveCapacity::Low);

    loop {
        let current_event = button.wait_for_event().await;

        debug!("Event = {}", current_event);
        led.toggle();
    }
}
