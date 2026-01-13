#![no_std]

pub mod ofs0;
pub mod ofs1;
pub mod time_driver;

use ra4m1_ctpac as pac;

#[rustfmt::skip]
embassy_hal_internal::peripherals_definition!(
    ICU,
    GPT320,
    GPT321,
);

#[rustfmt::skip]
embassy_hal_internal::peripherals_struct!(
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
