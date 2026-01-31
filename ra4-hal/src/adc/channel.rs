/// Pseudo-channel for the on-die temperature sensor.
pub struct Temperature;

/// Pseudo-channel for `Vref` measurement.
pub struct Vref;

/// `ADC14` channel instance.
#[allow(private_bounds)]
pub trait AdcChannel: SealedAdcChannel {}

pub(crate) trait AdcInputPin {}

#[allow(private_bounds)]
pub(crate) trait SealedAdcChannel: AdcInputPin {
    const CHANNEL: u8;

    fn enable<I: super::Instance>(&self) {
        let adc = I::regs();

        trace!("ADC14: enable_channel({})", Self::CHANNEL);

        #[cfg(feature = "strict-assert")]
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

        #[cfg(feature = "strict-assert")]
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

        #[cfg(feature = "strict-assert")]
        assert!(Self::CHANNEL <= 14 || (Self::CHANNEL >= 16 && Self::CHANNEL < 25));

        // TODO: Merge these in the chiptool IR as it's continuous memory with channel 15 being a gap
        if Self::CHANNEL <= 14 {
            adc.addr(Self::CHANNEL as _).read()
        } else if Self::CHANNEL >= 16 && Self::CHANNEL < 25 {
            adc.addr2((Self::CHANNEL - 16) as _).read()
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
        adc.adtsdr().read()
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
        adc.adocdr().read()
    }
}

macro_rules! input_pin_impl {
    ($pin:ident) => {
        impl crate::adc::channel::AdcInputPin for crate::peripherals::$pin {}
    };
}
pub(crate) use input_pin_impl;

macro_rules! chan_impl {
    ($chan:literal, $pin:ident) => {
        impl crate::adc::channel::AdcChannel for crate::peripherals::$pin {}
        impl crate::adc::channel::SealedAdcChannel for crate::peripherals::$pin {
            const CHANNEL: u8 = $chan;
        }
    };
}
pub(crate) use chan_impl;
