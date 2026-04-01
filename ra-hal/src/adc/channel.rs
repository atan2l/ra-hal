use embassy_hal_internal::{Peri, PeripheralType};
use embassy_time::{Duration, block_for};

use crate::{
    adc::{AdcChannelConfig, AverageMode, Resolution},
    constants::{TSN_INTERCEPT, TSN_SLOPE},
    gpio::{Flex, Pin},
    pac,
};

#[cfg(adc12)]
use pac::adc12::{self as adc_pac};
#[cfg(adc14)]
use pac::adc14::{self as adc_pac};
#[cfg(adc16)]
use pac::adc16::{self as adc_pac};

use adc_pac::regs::Adans;

#[cfg(adc16)]
use pac::adc16::{regs::Addiscr, vals::Vrefadcg};

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

/// GPIO pin that can be used as an `ADC` input channel.
///
/// This struct is used because there is a bit of setup needed to configure the pin for ADC usage.
/// In theory, at some point in the future, we could reset this to GPIO use on drop.
#[allow(private_bounds)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcPin {
    adc_channel: u8,
}

/// Multiple GPIO pins that can be used as `ADC` input channels.
///
/// # Notes
///
/// `ADC14` doesn't support arbitrary sequences but does support reading from arbitrary channels in order by channel index.
/// See the [docs](crate::adc::AdcInputPin) for each pin to reveal its associated ADC channel.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcSequence<const N: usize> {
    channels: [u8; N],
}

/// For the physical GPIO pins connected to `ADC` input channels.
#[allow(private_bounds)]
pub trait AdcInputPin: SealedAdcInputPin {
    /// ADC channel index `ANnnn`
    const CHANNEL: u8;
}

/// For the physical GPIO pins connected to `ADC` input channels.
pub(crate) trait SealedAdcInputPin: Pin + PeripheralType {}

/// `ADC` channel instance.
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
    /// Converts an ADC reading to a temperature in degrees celsius.
    pub fn raw_to_celsius(&self, v_s: u16, vref_mv: u16, width: Resolution) -> i32 {
        // RA4M1: § 48.7 TSN Characteristics

        let tsn = pac::TSN;
        let v_1 = tsn.tscdr().read();

        let calibration_width = cfg_select! {
          adc12 => 4096, // 12-bit
          adc14 => 4096, // 12-bit
          adc16 => 32768, // 15-bit
          _ => compile_error!()
        };

        let measurement_width = match width {
            #[cfg(any(adc12, adc14))]
            Resolution::_12bit => 4_096,
            #[cfg(adc14)]
            Resolution::_14bit => 16_384,
            #[cfg(adc16)]
            Resolution::_16bit => 32_768,
        };

        let v_1 = (u64::from(v_1) * 3_300_000) / calibration_width;
        let v_s = (u64::from(v_s) * u64::from(vref_mv) * 1_000) / measurement_width;

        i32::try_from(((v_s - v_1) as i64 / i64::from(TSN_SLOPE)) + i64::from(TSN_INTERCEPT))
            .expect("Temperature reading overflowed the bounds of an i32")
    }

    /// Converts an ADC reading to a temperature in degrees celsius.
    pub fn raw_to_celsius_float(&self, v_s: u16, vref_mv: u16, width: Resolution) -> f32 {
        // RA4M1: § 48.7 TSN Characteristics

        let tsn = pac::TSN;
        let v_1 = tsn.tscdr().read();

        let calibration_width = cfg_select! {
          adc12 => 4096, // 12-bit
          adc14 => 4096, // 12-bit
          adc16 => 32768, // 15-bit
          _ => compile_error!()
        };

        let measurement_width = match width {
            #[cfg(any(adc12, adc14))]
            Resolution::_12bit => 4_096,
            #[cfg(adc14)]
            Resolution::_14bit => 16_384,
            #[cfg(adc16)]
            Resolution::_16bit => 32_768,
        };

        let v_1 = (u64::from(v_1) * 3_300_000) / calibration_width;
        let v_s = (u64::from(v_s) * u64::from(vref_mv) * 1_000) / measurement_width;

        let v_1 = v_1 * 100;
        let v_s = v_s * 100;
        let intercept = TSN_INTERCEPT * 100;

        (((v_s - v_1) as i64 / i64::from(TSN_SLOPE)) + i64::from(intercept)) as f32 / 100.0
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

        #[cfg(any(adc12, adc14))]
        {
            adc.adhvrefcnt().modify(|r| {
                use adc_pac::vals::Hvsel;

                r.set_hvsel(Hvsel::Avcc0);
            });
        }

        #[cfg(adc16)]
        {
            adc.addiscr().write_value(Addiscr(0xF));

            adc.vrefampcnt().modify(|r| {
                r.set_vrefadcg(Vrefadcg::_11);
                r.set_bgren(true);
            });

            // Table 47.45 Internal reference voltage for 16-bit ADC (VREFADC) characteristics
            block_for(Duration::from_micros(2000));

            adc.vrefampcnt().modify(|r| {
                r.set_oldeten(true);
                r.set_vrefadcen(true);
                r.set_adslp(false);
            });
            block_for(Duration::from_micros(2000));
        }

        adc.adexicr().write(|w| w.set_tssa(true));
        block_for(Duration::from_micros(2000));

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
