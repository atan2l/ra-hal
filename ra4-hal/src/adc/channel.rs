use embassy_hal_internal::{Peri, PeripheralType};
use ra4m1_ctpac::adc14::regs::Adans;

use crate::{
    adc::{AdcChannelConfig, AverageMode},
    gpio::{Flex, Pin},
};

/// Pseudo-channel for the on-die temperature sensor.
///
/// # Notes
///
/// This is mutually exclusive with all other channels.
/// §35.2.13 `TSSA` bit.
pub struct Temperature;

/// Pseudo-channel for `Vref` measurement.
///
/// # Notes
///
/// This is mutually exclusive with all other channels.
/// §35.2.13 `OCSA` bit.
pub struct Vref;

/// GPIO pin that can be used as an `ADC14` channel.
///
/// This struct is used because there is a bit of setup needed to configure the pin for ADC usage.
/// In theory, at some point in the future, we could reset this to GPIO use on drop.
#[allow(private_bounds)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcPin {
    adc_channel: u8,
}

/// Multiple GPIO pins that can be used as `ADC14` channels.
///
/// # Notes
///
/// `ADC14` doesn't support arbitrary sequences but does support reading from arbitrary channels in order by channel index.
/// See the [docs](crate::adc::AdcInputPin) for each pin to reveal its associated ADC channel.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcSequence<const N: usize> {
    channels: [u8; N],
}

/// For the physical GPIO pins connected to `ADC14` channels.
#[allow(private_bounds)]
pub trait AdcInputPin: SealedAdcInputPin {
    /// ADC channel index `ANnnn`
    const CHANNEL: u8;
}

/// For the physical GPIO pins connected to `ADC14` channels.
pub(crate) trait SealedAdcInputPin: Pin + PeripheralType {}

/// `ADC14` channel instance.
///
/// An `AdcChannel` may be:
/// * An individual ADC channel (`ANnnn` in Renesas speak)
/// * A pseudo-channel such as the on-die temperature sensor
/// * A collection of individual channels to be read in sequence
///
/// So R may be either `u16` or `[u16; N]`.
#[allow(private_bounds)]
pub trait AdcChannel<R>: SealedAdcChannel<R> {}

pub(super) trait SealedAdcChannel<R> {
    fn enable<I: super::Instance>(&self, config: AdcChannelConfig);
    fn disable<I: super::Instance>(&self);
    fn read_one<I: super::Instance>(&self) -> R;
}

#[allow(private_bounds)]
impl AdcPin {
    /// Takes ownership of a pin for ADC use.
    #[inline]
    pub fn new<'d, P: AdcInputPin>(pin: Peri<'d, P>) -> Self {
        let mut flex = Flex::new_basic(pin);
        flex.set_as_analog();

        #[cfg(feature = "strict-assert")]
        assert!(P::CHANNEL <= 14 || (P::CHANNEL >= 16 && P::CHANNEL < 25));

        let adc_channel = P::CHANNEL;

        Self { adc_channel }
    }
}

impl<const N: usize> AdcSequence<N> {
    /// Takes ownership of an array of pins for ADC use.
    pub fn new(pins: [AdcPin; N]) -> Self {
        // There are only 26 ADC channels (0..=14, 16..=26).
        #[cfg(feature = "strict-assert")]
        assert!(N <= 26);

        // Taking an array of AdcPin is nice here because they're type erased and properly configured
        // for ADC input.  Will have to revisit this if we ever reset things on AdcPin::drop.  An
        // added benefit is that this won't accept the temp or vref pseudo-channels.

        let channels: [u8; N] = pins.map(|pin| pin.adc_channel);

        Self { channels }
    }

    /// Returns a slice containing the indices of the channels to be read.
    #[inline]
    pub fn channels(&self) -> &[u8] {
        &self.channels
    }
}

impl<const N: usize> AdcChannel<[u16; N]> for AdcSequence<N> {}

impl<const N: usize> SealedAdcChannel<[u16; N]> for AdcSequence<N> {
    #[inline]
    fn enable<I: super::Instance>(&self, config: AdcChannelConfig) {
        // ADC14 lets us independently adjust the sample time for some, but not all channels.
        // For now, let's just assign the same sample time.

        trace!("ADC14: enable_channel(sequence)");

        let adc = I::regs();

        adc.adexicr().modify(|w| {
            w.set_ocsa(false);
            w.set_tssa(false);
        });

        for channel in self.channels.iter() {
            trace!("  {} enable", channel);

            let index = *channel as usize / 16;
            let offset = *channel as usize % 16;

            adc.adansa(index).write(|w| w.set_ans(offset, true));

            if *channel <= 14 {
                adc.adsstr(*channel as _).write_value(config.sample_time);
            } else {
                adc.adsstrl().write_value(config.sample_time);
            }
        }

        if config.average_mode != AverageMode::Off {
            for channel in self.channels.iter() {
                trace!("  {} add/avg", channel);

                let index = *channel as usize / 16;
                let offset = *channel as usize % 16;

                adc.adads(index).write(|w| w.set_ads(offset, true));
            }
        }
    }

    #[inline]
    fn disable<I: super::Instance>(&self) {
        trace!("ADC14: disable_channel(sequence)");

        let adc = I::regs();

        if adc.adcsr().read().adst() {
            warn!("ADC14: Trying to disable sequence with a conversion running");
        }

        // Turn everything off.
        adc.adansa(0).write_value(Adans(0));
        adc.adansa(1).write_value(Adans(0));
    }

    #[inline]
    fn read_one<I: super::Instance>(&self) -> [u16; N] {
        let adc = I::regs();

        self.channels.map(|chan| adc.addr(chan as _).read())
    }
}

impl AdcChannel<u16> for AdcPin {}

impl SealedAdcChannel<u16> for AdcPin {
    #[inline]
    fn enable<I: super::Instance>(&self, config: AdcChannelConfig) {
        let adc = I::regs();

        trace!("ADC14: enable_channel({})", self.adc_channel);

        adc.adexicr().modify(|w| {
            w.set_ocsa(false);
            w.set_tssa(false);
        });

        let index = self.adc_channel as usize / 16;
        let offset = self.adc_channel as usize % 16;

        adc.adansa(index).write(|w| w.set_ans(offset, true));

        if config.average_mode != AverageMode::Off {
            adc.adads(index).write(|w| w.set_ads(offset, true))
        }

        if self.adc_channel <= 14 {
            adc.adsstr(self.adc_channel as _)
                .write_value(config.sample_time);
        } else {
            adc.adsstrl().write_value(config.sample_time);
        }
    }

    #[inline]
    fn disable<I: super::Instance>(&self) {
        let adc = I::regs();

        trace!("ADC14: disable_channel({})", self.adc_channel);

        if adc.adcsr().read().adst() {
            warn!(
                "ADC14: Trying to disable chan={} with a conversion running",
                self.adc_channel
            );
        }

        let index = self.adc_channel as usize / 16;
        let offset = self.adc_channel as usize % 16;

        adc.adansa(index).modify(|w| w.set_ans(offset, false));
    }

    #[inline]
    fn read_one<I: super::Instance>(&self) -> u16 {
        let adc = I::regs();

        trace!("ADC14: read_one({})", self.adc_channel);

        adc.addr(self.adc_channel as _).read()
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

impl AdcChannel<u16> for Temperature {}

impl SealedAdcChannel<u16> for Temperature {
    #[inline]
    fn enable<I: super::Instance>(&self, config: AdcChannelConfig) {
        let adc = I::regs();

        trace!("ADC14: enable_channel(TEMPERATURE)");

        if adc.adcsr().read().adst() {
            warn!("ADC14: Trying to enable temp with a conversion running");
        }

        adc.adsstrt().write_value(config.sample_time);

        adc.adexicr().modify(|w| w.set_tssa(true));

        if config.average_mode != AverageMode::Off {
            todo!()
        }
    }

    #[inline]
    fn disable<I: super::Instance>(&self) {
        let adc = I::regs();

        trace!("ADC14: disable_channel(TEMPERATURE)");

        if adc.adcsr().read().adst() {
            warn!("ADC14: Trying to disable temp with a conversion running");
        }

        adc.adexicr().modify(|w| w.set_tssa(false));
    }

    #[inline]
    fn read_one<I: super::Instance>(&self) -> u16 {
        let adc = I::regs();

        trace!("ADC14: read_one(TEMPERATURE)");
        adc.adtsdr().read()
    }
}

impl AdcChannel<u16> for Vref {}

impl SealedAdcChannel<u16> for Vref {
    #[inline]
    fn enable<I: super::Instance>(&self, config: AdcChannelConfig) {
        let adc = I::regs();

        trace!("ADC14: enable_channel(VREF)");

        if adc.adcsr().read().adst() {
            warn!("ADC14: Trying to enable Vref with a conversion running");
        }

        adc.adsstro().write_value(config.sample_time);

        adc.adexicr().modify(|w| w.set_ocsa(true));

        if config.average_mode != AverageMode::Off {
            todo!()
        }
    }

    #[inline]
    fn disable<I: super::Instance>(&self) {
        let adc = I::regs();

        trace!("ADC14: disable_channel(VREF)");

        if adc.adcsr().read().adst() {
            warn!("ADC14: Trying to disable Vref with a conversion running");
        }

        adc.adexicr().modify(|w| w.set_ocsa(false));
    }

    #[inline]
    fn read_one<I: super::Instance>(&self) -> u16 {
        let adc = I::regs();

        trace!("ADC14: read_one(VREF)");
        adc.adocdr().read()
    }
}

macro_rules! adc_pin {
    ($channel:literal, $pin:ident) => {
        impl crate::adc::channel::AdcInputPin for crate::peripherals::$pin {
            const CHANNEL: u8 = $channel;
        }
        impl crate::adc::channel::SealedAdcInputPin for crate::peripherals::$pin {}
    };
}
pub(crate) use adc_pin;
