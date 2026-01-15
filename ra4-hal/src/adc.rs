use core::marker::PhantomData;

#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
use embassy_hal_internal::{Peri, PeripheralType};

use crate::{pac, peripherals};

#[allow(private_bounds)]
pub struct Adc<'d, I: Instance> {
    _phantom: PhantomData<&'d I>,
}

#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

trait SealedInstance: PeripheralType {
    // fn regs() -> pac::adc::Adc;
}

impl SealedInstance for peripherals::ADC14 {}

impl Instance for peripherals::ADC14 {}

impl<'d, I: Instance> Adc<'d, I> {
    pub fn new(
        _adc: Peri<'d, I>,
        // config: AdcConfig,
        // _irq: impl interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>> + 'd,
    ) -> Self {
        trace!("ADC14: Powering up");
        let mstp = pac::MSTP;
        mstp.mstpcrd().write(|w| {
            w.set_mstpd16(false);
        });

        todo!()
    }
}

impl<'d, I: Instance> Drop for Adc<'d, I> {
    fn drop(&mut self) {
        trace!("ADC14: Powering down");

        let mstp = pac::MSTP;
        mstp.mstpcrd().write(|w| {
            w.set_mstpd16(true);
        });
    }
}

mod channel {
    #[allow(private_bounds)]
    pub trait AdcChannel: SealedAdcChannel {}

    #[allow(private_bounds)]
    pub(crate) trait SealedAdcChannel: AdcInputPin {
        const CHANNEL: u8;

        fn channel(&self) -> u8 {
            Self::CHANNEL
        }
    }

    trait AdcInputPin {}

    macro_rules! input_pin_impl {
        ($pin:ident) => {
            impl AdcInputPin for crate::peripherals::$pin {}
        };
    }

    macro_rules! chan_impl {
        ($chan:literal, $pin:ident) => {
            impl AdcChannel for crate::peripherals::$pin {}
            impl SealedAdcChannel for crate::peripherals::$pin {
                const CHANNEL: u8 = $chan;
            }
        };
    }

    input_pin_impl!(P000);
    input_pin_impl!(P001);
    input_pin_impl!(P002);
    input_pin_impl!(P003);
    input_pin_impl!(P004);
    input_pin_impl!(P010);
    input_pin_impl!(P011);
    input_pin_impl!(P012);
    input_pin_impl!(P013);
    input_pin_impl!(P014);
    input_pin_impl!(P015);
    #[cfg(feature = "100pin")]
    input_pin_impl!(P005);
    #[cfg(feature = "100pin")]
    input_pin_impl!(P006);
    #[cfg(feature = "100pin")]
    input_pin_impl!(P007);
    #[cfg(feature = "100pin")]
    input_pin_impl!(P008);

    input_pin_impl!(P500);
    input_pin_impl!(P501);
    input_pin_impl!(P502);

    input_pin_impl!(P103);
    input_pin_impl!(P102);
    input_pin_impl!(P101);
    input_pin_impl!(P100);

    #[cfg(feature = "100pin")]
    input_pin_impl!(P503);
    #[cfg(feature = "100pin")]
    input_pin_impl!(P504);
    #[cfg(feature = "100pin")]
    input_pin_impl!(P505);

    chan_impl!(0, P000);
    chan_impl!(1, P001);
    chan_impl!(2, P002);
    chan_impl!(3, P003);
    chan_impl!(4, P004);
    chan_impl!(5, P010);
    chan_impl!(6, P011);
    chan_impl!(7, P012);
    chan_impl!(8, P013);
    chan_impl!(9, P014);
    chan_impl!(10, P015);
    #[cfg(feature = "100pin")]
    chan_impl!(11, P005);
    #[cfg(feature = "100pin")]
    chan_impl!(12, P006);
    #[cfg(feature = "100pin")]
    chan_impl!(13, P007);
    #[cfg(feature = "100pin")]
    chan_impl!(14, P008);

    chan_impl!(16, P500);
    chan_impl!(17, P501);
    chan_impl!(18, P502);

    chan_impl!(19, P103);
    chan_impl!(20, P102);
    chan_impl!(21, P101);
    chan_impl!(22, P100);

    #[cfg(feature = "100pin")]
    chan_impl!(23, P503);
    #[cfg(feature = "100pin")]
    chan_impl!(24, P504);
    #[cfg(feature = "100pin")]
    chan_impl!(25, P505);
}
