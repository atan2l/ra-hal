use core::marker::PhantomData;

use cortex_m::asm;
#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
use embassy_hal_internal::{Peri, PeripheralType};
use ra4m1_ctpac::adc14::vals::Adcs;

use crate::{
    adc::channel::{AdcChannel, Temperature},
    pac, peripherals,
};

#[allow(private_bounds)]
pub struct Adc<'d, I: Instance> {
    _phantom: PhantomData<&'d I>,
}

/// `ADC14` peripheral instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

trait SealedInstance: PeripheralType {
    fn regs() -> pac::adc14::Adc14;
}

impl SealedInstance for peripherals::ADC14 {
    fn regs() -> ra4m1_ctpac::adc14::Adc14 {
        pac::ADC14
    }
}

impl Instance for peripherals::ADC14 {}

impl<'d, I: Instance> Adc<'d, I> {
    pub fn new(
        _adc: Peri<'d, I>,
        // config: AdcConfig,
        // _irq: impl interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>> + 'd,
    ) -> Self {
        warn!("ADC14: Powering up");
        let mstp = pac::MSTP;
        mstp.mstpcrd().write(|w| {
            w.set_mstpd16(false);
        });

        Self {
            _phantom: PhantomData,
        }
    }

    fn enable_channel(&self, channel: usize) {}

    fn disable_channel(&self, channel: usize) {
        let adc = I::regs();

        trace!("ADC14: disable_channel({})", channel);

        if channel <= 14 {
            adc.adansa0().modify(|w| {
                w.set_ansa(channel as _, false);
            });
        } else if channel >= 16 && channel < 25 {
            adc.adansa1().modify(|w| {
                w.set_ansa((channel - 16) as _, false);
            });
        } else if channel == 15 {
            if adc.adcsr().read().adst() {
                warn!("ADC14: Trying to disable temp with a conversion running");
            }

            adc.adexicr().modify(|w| {
                w.set_tssa(false);
            });
        } else {
            panic!("Invalid ADC channel");
        }
    }

    /// Return the pseudo-channel struct for temprature measurement.  Nothing is configured here.
    pub fn temperature_channel(&self) -> Temperature {
        Temperature {}
    }

    pub fn blocking_read(&self, channel: &impl AdcChannel) -> u16 {
        let adc = I::regs();

        channel.enable::<I>();

        // TODO: read adst first to ensure we're stopped?

        adc.adcsr().modify(|w| {
            w.set_adcs(Adcs::Single);
        });

        adc.adcsr().modify(|w| {
            w.set_adst(true);
        });

        // When the conversion is finished an interrupt is fired (without modifying the registers) and the ADST bit is cleared
        while adc.adcsr().read().adst() {
            asm::nop()
        }

        channel.disable::<I>();

        channel.read_one::<I>()
    }
}

impl<'d, I: Instance> Drop for Adc<'d, I> {
    fn drop(&mut self) {
        warn!("ADC14: Powering down");

        let mstp = pac::MSTP;
        mstp.mstpcrd().write(|w| {
            w.set_mstpd16(true);
        });
    }
}

mod channel {
    #[allow(unused)]
    use defmt::{debug, error, info, trace, warn};

    #[allow(private_bounds)]
    pub trait AdcChannel: SealedAdcChannel {}

    #[allow(private_bounds)]
    pub(crate) trait SealedAdcChannel: AdcInputPin {
        const CHANNEL: u8;

        fn enable<I: super::Instance>(&self) {
            let adc = I::regs();

            trace!("ADC14: enable_channel({})", Self::CHANNEL);

            assert!(Self::CHANNEL <= 14 || (Self::CHANNEL >= 16 && Self::CHANNEL < 25));

            if Self::CHANNEL <= 14 {
                adc.adexicr().modify(|w| {
                    w.set_ocsa(false);
                    w.set_tssa(false);
                });

                adc.adansa0().modify(|w| {
                    w.set_ansa(Self::CHANNEL as _, true);
                });
            } else if Self::CHANNEL >= 16 && Self::CHANNEL < 25 {
                adc.adexicr().modify(|w| {
                    w.set_ocsa(false);
                    w.set_tssa(false);
                });

                adc.adansa1().modify(|w| {
                    w.set_ansa((Self::CHANNEL - 16) as _, true);
                });
            }
        }

        fn disable<I: super::Instance>(&self) {
            let adc = I::regs();

            trace!("ADC14: disable_channel({})", Self::CHANNEL);

            assert!(Self::CHANNEL <= 14 || (Self::CHANNEL >= 16 && Self::CHANNEL < 25));

            if Self::CHANNEL <= 14 {
                adc.adansa0().modify(|w| {
                    w.set_ansa(Self::CHANNEL as _, false);
                });
            } else if Self::CHANNEL >= 16 && Self::CHANNEL < 25 {
                adc.adansa1().modify(|w| {
                    w.set_ansa((Self::CHANNEL - 16) as _, false);
                });
            }
        }

        fn read_one<I: super::Instance>(&self) -> u16 {
            let adc = I::regs();

            trace!("ADC14: read_one({})", Self::CHANNEL);

            assert!(Self::CHANNEL <= 14 || (Self::CHANNEL >= 16 && Self::CHANNEL < 25));

            // TODO: Merge these in the chiptool IR as it's continuous memory with channel 15 being a gap
            if Self::CHANNEL <= 14 {
                adc.addr(Self::CHANNEL as _).read().addr()
            } else if Self::CHANNEL >= 16 && Self::CHANNEL < 25 {
                adc.addr2((Self::CHANNEL - 16) as _).read().addr()
            } else {
                unimplemented!("Invalid ADC channel");
            }
        }
    }

    trait AdcInputPin {}

    pub struct Temperature;
    impl AdcInputPin for Temperature {}
    impl AdcChannel for Temperature {}

    impl SealedAdcChannel for Temperature {
        const CHANNEL: u8 = 255;

        fn enable<I: super::Instance>(&self) {
            let adc = I::regs();

            trace!("ADC14: enable_channel(TEMPERATURE)");

            if adc.adcsr().read().adst() {
                warn!("ADC14: Trying to enable temp with a conversion running");
            }

            adc.adexicr().modify(|w| {
                w.set_ocsa(false);
                w.set_tssa(true);
            });
        }

        fn disable<I: super::Instance>(&self) {
            let adc = I::regs();

            trace!("ADC14: disable_channel(TEMPERATURE)");

            if adc.adcsr().read().adst() {
                warn!("ADC14: Trying to disable temp with a conversion running");
            }

            adc.adexicr().modify(|w| {
                w.set_tssa(false);
            });
        }

        fn read_one<I: super::Instance>(&self) -> u16 {
            let adc = I::regs();

            trace!("ADC14: read_one(TEMPERATURE)");
            adc.adtsdr().read().adtsdr()
        }
    }

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
