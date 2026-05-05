#![no_std]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

//! ## Feature flags
#![doc = document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#)]

// This needs to come first so the macros are visible everywhere else.
#[doc(hidden)]
pub mod fmt;

#[cfg(any(adc12, adc14, adc16))]
pub mod adc;

pub mod clock;
pub mod crc;
#[cfg(dac12)]
pub mod dac;
#[cfg(dmac)]
pub mod dmac;
pub mod dtc;
pub mod event_link;
pub mod gpio;
#[cfg(iic)]
pub mod i2c;
pub mod mcu_info;
pub mod module_stop;
#[cfg(not(feature = "skip-osm"))]
pub mod osm;
#[cfg(gpt)]
pub mod pwm;
#[cfg(gpt)]
pub mod qdec;
// #[cfg(feature = "_enable-rtc-beware-of-dragons")]
// pub mod rtc;
#[cfg(any(agt, agtw))]
pub mod timer_agt;
#[cfg(gpt)]
pub mod timer_gpt;
#[cfg(ulpt)]
pub mod timer_ulpt;
pub mod watchdog;
// pub mod sce5;
#[cfg(ra4m1)]
pub mod spi;
#[cfg(trust_zone)]
pub mod trust_zone;

// This uses cfg_select explicitly so we can avoid defining a time_driver module if the feature
// isn't enabled, but still fail if we don't enable a specific time driver.
cfg_select! {
    feature = "time-driver" => {
        #[cfg_attr(feature = "time-driver-agt", path = "time_driver_agt.rs")]
        #[cfg_attr(any(feature = "time-driver-gpt0", feature = "time-driver-gpt1"), path = "time_driver_gpt.rs")]
        #[cfg_attr(feature = "time-driver-ulpt", path = "time_driver_ulpt.rs")]
        pub mod time_driver;
    }
    _ => {}
}

#[cfg(not(sci_b))]
pub mod uart;
pub mod write_protect;

// Re-exports
#[cfg(feature = "chrono")]
pub use chrono;

cfg_select! {
    feature = "unstable-pac" => {
        pub use ra_metapac as pac;
    }
    _ => {
        pub(crate) use ra_metapac as pac;
    }
}

use crate::mcu_info::McuInfo;

/// Coarse indication of why the processor reset.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(unused)]
pub enum ResetCause {
    /// Power was turned on.
    PowerOn,

    /// Low voltage monitor 0, 1, or 2 tripped.
    LowVoltage,

    /// Watchdog or independent watchdog.
    Watchdog,

    /// Bus error, parity error, or ECC error.
    HadwareError,

    /// Stack pointer error.
    StackPointer,

    /// Software reset requested.
    SoftwareReset,

    /// Should never be here.
    Unknown,
}

/// Initializes the MCU.
///
/// # Returns
///
/// The available peripherals as a [`Peripherals`] struct.
pub fn init(clocks: clock::ClockConfig) -> Peripherals {
    critical_section::with(|cs| {
        debug!("Starting board init");

        let mcu_info = McuInfo::info();

        #[cfg(feature = "defmt")]
        mcu_info.print_info();

        // Check if the crate was configured correctly
        mcu_info.validate_package();

        {
            let osm = pac::OSM;
            debug!("OFS0: {}", osm.ofs0().read());
            debug!("OFS1: {}", osm.ofs1().read());
        }

        #[cfg(trust_zone)]
        trust_zone::init();

        if clock::init(clocks).is_err() {
            panic!("Clocks were already initialized?");
        }
        info!("{}", clock::clock_status());

        #[cfg(feature = "time-driver")]
        time_driver::init();

        event_link::init();

        dtc::init();

        #[cfg(dmac)]
        dmac::init();

        Peripherals::take_with_cs(cs)
    })
}

// NOTE: this macro can't be in `embassy-hal-internal` due to the use of `$crate`.
/// Macro to bind interrupts to interrupt handlers.
///
/// For example:
///
/// ```rust,ignore
/// use ra_hal::{bind_interrupts, peripherals::SCI1, uart};
///
/// bind_interrupts!(struct Irqs {
///     IEL2 => uart::RxInterruptHandler<SCI1>;
///     IEL3 => uart::TxInterruptHandler<SCI1>;
///     IEL4 => uart::TeInterruptHandler<SCI1>;
/// });
///```
///
/// Any interrupt `IEL2..=IEL31` can be assigned to any one handler.
/// Note that `IEL0` and `IEL1` are used by the [time driver](crate::time_driver) and are unavailable for general use.
#[macro_export]
macro_rules! bind_interrupts {
    ($(#[$outer:meta])* $vis:vis struct $name:ident {
        $(
            $(#[doc = $doc:literal])*
            $(#[cfg($cond_irq:meta)])?
            $irq:ident => $(
                $(#[cfg($cond_handler:meta)])?
                $handler:ty
            ),*;
        )*
    }) => {
        #[derive(Copy, Clone)]
        $(#[$outer])*
        $vis struct $name;

        $(
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            $(#[cfg($cond_irq)])?
            $(#[doc = $doc])*
            unsafe extern "C" fn $irq() {
                unsafe {
                    $(
                        $(#[cfg($cond_handler)])?
                        <$handler as $crate::interrupt::typelevel::Handler<$crate::interrupt::typelevel::$irq>>::on_interrupt();

                    )*
                }
            }

            $(#[cfg($cond_irq)])?
            $crate::bind_interrupts!(@inner
                $(
                    $(#[cfg($cond_handler)])?
                    unsafe impl $crate::interrupt::typelevel::Binding<$crate::interrupt::typelevel::$irq, $handler> for $name {}
                )*
            );
        )*
    };
    (@inner $($t:tt)*) => {
        $($t)*
    }
}

include!(concat!(env!("OUT_DIR"), "/interrupts.rs"));
include!(concat!(env!("OUT_DIR"), "/peripherals.rs"));
include!(concat!(env!("OUT_DIR"), "/module_stops.rs"));

#[allow(missing_docs)]
pub mod constants {
    include!(concat!(env!("OUT_DIR"), "/constants.rs"));
}

#[cfg(not(feature = "skip-osm"))]
mod _osm_config {
    use crate::osm::{ofs0::Ofs0, ofs1::Ofs1, sec_mpu::SecurityMpu};

    // Option Function Select Register 0
    #[unsafe(no_mangle)]
    #[unsafe(link_section = ".ofs0")]
    static OFS0: Ofs0 = Ofs0::default();

    // Option Function Select Register 1
    #[unsafe(no_mangle)]
    #[unsafe(link_section = ".ofs1")]
    static OFS1: Ofs1 = Ofs1::default();

    // Security MPU
    #[unsafe(no_mangle)]
    #[unsafe(link_section = ".sec_mpu")]
    static SEC_MPU: SecurityMpu = SecurityMpu::disabled();
}

/// Calls the semihosting exit hook.
/// Will cause an exception without a semihosting implementation on the host side.
#[macro_export]
macro_rules! exit {
    () => {
        unsafe {
            cortex_m::asm::semihosting_syscall(0x18, 0x20026);
        };
    };
}
