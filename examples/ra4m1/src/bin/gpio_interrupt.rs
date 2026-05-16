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

use assign_resources::assign_resources;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;
use ra_hal::{
    Peri, bind_interrupts,
    clock::ClockConfig,
    gpio::{
        Debounce, DriveCapacity, GpioTrigger, InputInterruptHandler, InterruptFlex, Level, Output,
    },
    peripherals,
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};

cfg_select! {
    feature = "uno-r4-minima" => {
        // Define the pins we want on the R4 Minima
        assign_resources! {
            gpio: GpioResources {
                button: P110,
                led: P111,
                irq: GPIO_IRQ5,
            }
        }
    },
    feature = "uno-r4-wifi" => {
        // Define the pins we want on the R4 WiFi
        assign_resources! {
            gpio: GpioResources {
                button: P410,
                led: P102,
                irq: GPIO_IRQ5,
            }
        }
    }
    _ => {
        compile_error!(
            "Ensure the pin and timer assignments are correct for your board before continuing."
        );
    }
}

#[cfg(feature = "uno-r4-wifi")]
bind_interrupts!(struct Irqs {
    IEL2 => InputInterruptHandler<ra_hal::peripherals::P410>;
});

#[cfg(feature = "uno-r4-minima")]
bind_interrupts!(struct Irqs {
    IEL2 => InputInterruptHandler<ra_hal::peripherals::P110>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());
    let r = split_resources!(p);

    let mut button = InterruptFlex::new(r.gpio.button, r.gpio.irq, Irqs);
    button.set_pull_up(true);
    button.set_trigger(GpioTrigger::Both);
    button.set_debounce(Debounce::Min8);

    let mut led = Output::new_basic(r.gpio.led, Level::Low, DriveCapacity::Low);

    loop {
        let current_event = button.wait_for_event().await;

        debug!("Event = {}", current_event);
        led.toggle();
    }
}
