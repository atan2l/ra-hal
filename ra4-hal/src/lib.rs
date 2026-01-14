#![no_std]

pub mod crc;
pub mod ofs0;
pub mod ofs1;
#[cfg(feature = "time-driver")]
pub mod time_driver;
pub mod write_protect;

#[cfg(feature = "unstable-pac")]
pub use ra4m1_ctpac as pac;
#[cfg(not(feature = "unstable-pac"))]
pub(crate) use ra4m1_ctpac as pac;

#[rustfmt::skip]
embassy_hal_internal::peripherals_definition!(
    CRC,
    ICU,
    GPT320,
    GPT321,
);

#[rustfmt::skip]
embassy_hal_internal::peripherals_struct!(
    CRC,
    ICU,
    GPT320,
    GPT321,
);

#[rustfmt::skip]
embassy_hal_internal::interrupt_mod!(
    IEL0,
    IEL1,
    IEL2,
);

pub fn init() -> Peripherals {
    critical_section::with(|cs| {
        let p = Peripherals::take_with_cs(cs);

        #[cfg(feature = "time-driver")]
        time_driver::init(crate::interrupt::Priority::P2);

        p
    })
}
