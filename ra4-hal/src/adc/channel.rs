pub struct Temperature;
pub struct Vref;

#[allow(private_bounds)]
pub trait AdcChannel: SealedAdcChannel {}

trait AdcInputPin {}

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

impl Temperature {
    /// Converts an ADC reading in millivolts to a temperature in degrees celsius.
    pub fn millivolt_to_celsius(&self, v_s: u16) -> f32 {
        // § 48.7 TSN Characteristics
        let slope = -3.65;
        let v_1 = 1050.0;
        let intercept = 25.0;

        ((f32::from(v_s) - v_1) / slope) - intercept
    }
}

impl AdcChannel for Temperature {}

impl AdcInputPin for Temperature {}

impl SealedAdcChannel for Temperature {
    const CHANNEL: u8 = 254;

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

impl AdcChannel for Vref {}

impl AdcInputPin for Vref {}

impl SealedAdcChannel for Vref {
    const CHANNEL: u8 = 255;

    fn enable<I: super::Instance>(&self) {
        let adc = I::regs();

        trace!("ADC14: enable_channel(VREF)");

        if adc.adcsr().read().adst() {
            warn!("ADC14: Trying to enable Vref with a conversion running");
        }

        adc.adexicr().modify(|w| {
            w.set_ocsa(true);
            w.set_tssa(false);
        });
    }

    fn disable<I: super::Instance>(&self) {
        let adc = I::regs();

        trace!("ADC14: disable_channel(VREF)");

        if adc.adcsr().read().adst() {
            warn!("ADC14: Trying to disable Vref with a conversion running");
        }

        adc.adexicr().modify(|w| {
            w.set_ocsa(false);
        });
    }

    fn read_one<I: super::Instance>(&self) -> u16 {
        let adc = I::regs();

        trace!("ADC14: read_one(VREF)");
        adc.adocdr().read().adocdr()
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
#[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
input_pin_impl!(P002);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
input_pin_impl!(P003);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
input_pin_impl!(P004);
input_pin_impl!(P010);
input_pin_impl!(P011);
input_pin_impl!(P012);
input_pin_impl!(P013);
input_pin_impl!(P014);
input_pin_impl!(P015);
#[cfg(feature = "_100pin")]
input_pin_impl!(P005);
#[cfg(feature = "_100pin")]
input_pin_impl!(P006);
#[cfg(feature = "_100pin")]
input_pin_impl!(P007);
#[cfg(feature = "_100pin")]
input_pin_impl!(P008);

input_pin_impl!(P500);
input_pin_impl!(P501);
input_pin_impl!(P502);

#[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
input_pin_impl!(P103);
input_pin_impl!(P102);
input_pin_impl!(P101);
input_pin_impl!(P100);

#[cfg(feature = "_100pin")]
input_pin_impl!(P503);
#[cfg(feature = "_100pin")]
input_pin_impl!(P504);
#[cfg(feature = "_100pin")]
input_pin_impl!(P505);

chan_impl!(0, P000);
chan_impl!(1, P001);
#[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
chan_impl!(2, P002);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
chan_impl!(3, P003);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
chan_impl!(4, P004);
chan_impl!(5, P010);
chan_impl!(6, P011);
chan_impl!(7, P012);
chan_impl!(8, P013);
chan_impl!(9, P014);
chan_impl!(10, P015);
#[cfg(feature = "_100pin")]
chan_impl!(11, P005);
#[cfg(feature = "_100pin")]
chan_impl!(12, P006);
#[cfg(feature = "_100pin")]
chan_impl!(13, P007);
#[cfg(feature = "_100pin")]
chan_impl!(14, P008);

chan_impl!(16, P500);
chan_impl!(17, P501);
chan_impl!(18, P502);

#[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
chan_impl!(19, P103);
chan_impl!(20, P102);
chan_impl!(21, P101);
chan_impl!(22, P100);

#[cfg(feature = "_100pin")]
chan_impl!(23, P503);
#[cfg(feature = "_100pin")]
chan_impl!(24, P504);
#[cfg(feature = "_100pin")]
chan_impl!(25, P505);
