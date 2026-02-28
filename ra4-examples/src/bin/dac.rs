//! `dac_output` Uses DTC to send different waveforms to the DAC.

#![no_std]
#![no_main]
#![warn(missing_docs)]

use core::f32::consts::PI;

use cortex_m::asm;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};
use libm::{acosf, asinf, cosf, roundf, sinf};
use panic_probe as _;
use ra4_hal::{
    bind_interrupts,
    dac::{Align, Dac},
    dtc::{Channel, DtcInterruptHandler},
    event_link::{SoftwareEvent, SoftwareEventGenerator as _},
    peripherals::DTC_CHAN2,
    timer::InterruptTimer,
};
#[allow(unused)]
use ra4_hal::{debug, error, info, trace, warn};

bind_interrupts!(struct Irqs {
    IEL2 => DtcInterruptHandler<DTC_CHAN2>;
    // IEL3 => DtcInterruptHandler<DTC_CHAN3>;
});

/// Max value for our 12-bit DAC
const DAC_MAX: f32 = ((1 << 12) - 1) as f32;

const SPACING: u64 = 2000;

/// Number of samples per period, max is 255.
const SAMPLE_COUNT: usize = 48 * 5;

/// Associated functions generate and copy waveforms to a buffer.
struct Waveform {}

impl Waveform {
    /// Populates `buffer` with a square waveform, 50% duty cycle.
    pub fn square(buffer: &mut [u16]) {
        let on = DAC_MAX as u16;
        let off = 0;

        let len = buffer.len();

        for (i, byte) in buffer.iter_mut().enumerate() {
            if i < len / 2 {
                *byte = off;
            } else {
                *byte = on;
            }
        }
    }

    /// Populates `buffer` with a sine waveform.
    pub fn sine(buffer: &mut [u16]) {
        let buffer_len = buffer.len() as f32;

        for (i, word) in buffer.iter_mut().enumerate() {
            let phase = buffer_len / -2.0;
            let x = (i as f32 * ((2.0 * PI) / buffer_len)) + phase;
            let y = (DAC_MAX * (sinf(x) + 1.0)) / 2.0;

            *word = roundf(y) as u16;
        }
    }

    /// Populates `buffer` with a trapezoidal waveform.
    pub fn trapezoid(buffer: &mut [u16]) {
        let a = DAC_MAX;
        let m = buffer.len() as f32 / 2.0;
        let l = a / 2.0;
        let c = 0.0;

        for (x, word) in buffer.iter_mut().enumerate() {
            let y = (a / PI)
                * ((asinf(sinf((PI / m) * x as f32 + l))) + acosf(cosf((PI / m) * x as f32 + l)))
                - a / 2.0
                + c;

            let y = roundf(y + l - c);

            // Make sure we're generating sane values.
            assert!(y <= DAC_MAX);

            *word = y as u16;
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra4_hal::init();

    let mut buffer = [0_u16; SAMPLE_COUNT];

    // Generate "frequency" Hz waves.
    let frequency = 10 * buffer.len();

    let mut dtc_channel2 = Channel::new(p.DTC_CHAN2, Irqs);
    let mut timer = InterruptTimer::new(p.GPT16_2);
    timer.set_frequency(frequency as u32);

    let mut dac = Dac::new(p.DAC12, p.P014);
    dac.set_align(Align::Right);

    // In repeat mode we're limited to 8 bits because the counter serves dual duty §17.2.5.
    assert!(buffer.len() <= usize::from(u8::MAX));

    // Asynchronous DTC
    info!("Single word DTC transfers");

    let trigger = SoftwareEvent::<0>::new();

    Waveform::sine(&mut buffer);

    for _ in 0..50 {
        for word in buffer.iter() {
            // DTC transfers are always initiated by interrupts.  So to avoid a race
            // we build up the transfer and link the event to the DTC controller and
            // manually fire the interrupt separately.

            let now = Instant::now();

            // Gotta make sure that the source (and destination) outlive the transfer.
            let buf = [*word];

            let transfer =
                unsafe { dtc_channel2.write(&buf, dac.buffer(), trigger.event(), false) };

            // Kick off software generated interrupt
            trigger.fire();

            // Wait for transfer to finish
            transfer.await;

            // Wait a bit to stretch out the waveform
            while now.elapsed() < Duration::from_nanos(250_000) {
                asm::nop()
            }
        }
    }

    info!("On to circular transfers");
    let event = timer.start();

    loop {
        // Start a circular transfer. In this case we're not treating the transfer as a future
        // we're just letting it loop for a few seconds.
        {
            Waveform::sine(&mut buffer);
            let _transfer = unsafe { dtc_channel2.write(&buffer, dac.buffer(), event, true) };
            Timer::after_millis(SPACING).await;
        }

        {
            Waveform::square(&mut buffer);
            let _transfer = unsafe { dtc_channel2.write(&buffer, dac.buffer(), event, true) };
            Timer::after_millis(SPACING).await;
        }

        {
            Waveform::trapezoid(&mut buffer);
            let _transfer = unsafe { dtc_channel2.write(&buffer, dac.buffer(), event, true) };
            Timer::after_millis(SPACING).await;
        }
    }
}
