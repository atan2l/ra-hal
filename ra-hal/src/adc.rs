//!  Analog-to-Digital Converter (`ADC12`, `ADC14`, `ADC16`).
//!
//! # Supported Hardware
//! * `ADC12`: supports 12-bit precision (`RA4L1`) or 8-, 10-, 12-bit precision (`RA6M5`, soon).
//! * `ADC14`: supports 12- and 14-bit precision (`RA4M1`).
//! * `ADC16`: supports 16-bit precision (`RA2A1`).
//!
//! # Notes
//! * The driver will turn the module off when it is dropped.

use core::marker::PhantomData;
use core::task::Poll;

use cortex_m::asm;
use embassy_hal_internal::{Peri, PeripheralType};

use crate::{adc::channel::AdcChannel, module_stop::ModuleStop, pac};

#[cfg(adc12)]
use pac::adc12::{self as adc_pac, Adc12 as AdcPac};
#[cfg(adc14)]
use pac::adc14::{self as adc_pac, Adc14 as AdcPac};
#[cfg(adc16)]
use pac::adc16::{self as adc_pac, Adc16 as AdcPac};

use adc_pac::{
    regs::Adans,
    vals::{AdcCountSelect, Adcs},
};

#[cfg(adc12)]
use crate::peripherals::ADC12_0;

#[cfg(adc14)]
use crate::{pac::adc14::vals::Adprc, peripherals::ADC14_0};

#[cfg(adc16)]
use crate::peripherals::ADC16_0;

use crate::interrupt::typelevel::{
    Binding, Handler as InterruptHandler, Interrupt as InterruptType,
};
pub use channel::{AdcInputPin, AdcPin, AdcSequence, Temperature, Vref};

#[cfg(dmac)]
use crate::dmac::{Channel as DmacChannel, DmacInterruptHandler};

pub(crate) mod channel;

/// `ADC` driver.
#[allow(private_bounds)]
pub struct Adc<'d, I: Instance> {
    _phantom: PhantomData<&'d I>,
    average_mode: AverageMode,
    sample_time: u8,
}

/// Addition/Average method.
///
/// For a single reading:
/// * `ADC12` and `ADC14` can report the single sample or the sum or average of multiple samples.
/// * `ADC16` can report the single sample or the average of multiple samples.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Default, PartialEq)]
pub enum AverageMode {
    /// Sample the channel once.
    #[default]
    Off,

    /// Sample the channel 2x and place the average in the output register.
    Average2,

    /// Sample the channel 4x and place the average in the output register.
    Average4,

    /// Place the sum of 2 samples in the output register.
    #[cfg(any(adc12, adc14))]
    Add2,

    /// Place the sum of 3 samples in the output register.
    #[cfg(any(adc12, adc14))]
    Add3,

    /// Place the sum of 4 samples in the output register.
    #[cfg(any(adc12, adc14))]
    Add4,

    /// Place the sum of 16 samples in the output register.
    #[cfg(any(adc12, adc14))]
    Add16,
}

/// ADC resolution.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Default)]
pub enum Resolution {
    /// 12-bit resolution
    #[cfg(any(adc12, adc14))]
    #[default]
    _12bit,

    /// 14-bit resolution
    #[cfg(adc14)]
    _14bit,

    /// 16-bit resolution
    #[cfg(adc16)]
    #[default]
    _16bit,
}

/// Alignment of readings within a 16-bit variable.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Default)]
pub enum Alignment {
    /// Pad the value with trailing zeros.
    FlushLeft,

    /// Pad the value with leading zeros.
    #[default]
    FlushRight,
}

/// `ADC` configuration.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone)]
pub struct AdcConfig {
    #[allow(missing_docs)]
    pub resolution: Resolution,

    /// If true the data registers are zeroed after being read (whether by CPU or DMA).
    pub clear_on_read: bool,

    #[allow(missing_docs)]
    #[cfg(not(adc16))]
    pub alignment: Alignment,

    #[allow(missing_docs)]
    pub average_mode: AverageMode,

    /// Sample time in number of `ADCLK` cycles.
    ///
    /// # Notes
    /// Sample time can *never* be lower than 5 states.
    /// If the ratio of `PCLKB` to `PCLKC` is 1:2 or 1:4 the minimum is 6 states.
    pub sample_time: u8,
}

#[derive(Copy, Clone)]
struct AdcChannelConfig {
    pub average_mode: AverageMode,
    pub sample_time: u8,
}

/// `ADC` peripheral instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + ModuleStop + PeripheralType + 'static + Send {
    /// The interrupt event that is triggered when the ADC scan is complete.
    const RX_EVENT: crate::event_link::InterruptEvent;
}

impl Default for AdcConfig {
    fn default() -> Self {
        Self {
            resolution: Default::default(),
            clear_on_read: Default::default(),
            #[cfg(not(adc16))]
            alignment: Default::default(),
            average_mode: Default::default(),
            sample_time: 13,
        }
    }
}

trait SealedInstance: PeripheralType {
    fn regs() -> AdcPac;
}

#[cfg(adc12)]
impl SealedInstance for ADC12_0 {
    fn regs() -> AdcPac {
        pac::ADC12_0
    }
}

#[cfg(adc14)]
impl SealedInstance for ADC14_0 {
    fn regs() -> AdcPac {
        pac::ADC14_0
    }
}

#[cfg(adc16)]
impl SealedInstance for ADC16_0 {
    fn regs() -> AdcPac {
        pac::ADC16_0
    }
}

#[cfg(adc12)]
impl Instance for ADC12_0 {
    const RX_EVENT: crate::event_link::InterruptEvent =
        crate::event_link::InterruptEvent::Adc0ScanEnd;
}

#[cfg(adc14)]
impl Instance for ADC14_0 {
    const RX_EVENT: crate::event_link::InterruptEvent =
        crate::event_link::InterruptEvent::Adc0ScanEnd;
}

#[cfg(adc16)]
impl Instance for ADC16_0 {
    const RX_EVENT: crate::event_link::InterruptEvent =
        crate::event_link::InterruptEvent::Adc0ScanEnd;
}

impl<'d, I: Instance> Adc<'d, I> {
    /// Creates a new `ADC` driver.
    pub fn new(_adc: Peri<'d, I>, config: AdcConfig) -> Self {
        #[cfg(feature = "strict-assert")]
        assert!(config.sample_time >= 5);

        I::start_module();

        #[cfg(adc14)]
        let resolution = match config.resolution {
            Resolution::_12bit => Adprc::_12bit,
            Resolution::_14bit => Adprc::_14bit,
        };

        let clear_on_read = config.clear_on_read;

        #[cfg(not(adc16))]
        let alignment = match config.alignment {
            Alignment::FlushLeft => true,
            Alignment::FlushRight => false,
        };

        let adc = I::regs();

        // Turn everything off.
        adc.adansa(0).write_value(Adans(0));
        adc.adansa(1).write_value(Adans(0));

        adc.adexicr().modify(|w| {
            w.set_ocsa(false);
            w.set_tssa(false);
        });

        adc.adcer().write(|w| {
            #[cfg(adc14)]
            {
                w.set_adprc(resolution);
            }

            #[cfg(any(adc12, adc14))]
            {
                w.set_adrfmt(alignment);
            }

            w.set_ace(clear_on_read);
        });

        adc.adadc().write(|w| match config.average_mode {
            AverageMode::Off => {}
            AverageMode::Average2 => {
                #[cfg(any(adc12, adc14))]
                w.set_avee(true);
                w.set_adc(AdcCountSelect::Convert2);
            }
            AverageMode::Average4 => {
                #[cfg(any(adc12, adc14))]
                w.set_avee(true);
                w.set_adc(AdcCountSelect::Convert4);
            }
            #[cfg(any(adc12, adc14))]
            AverageMode::Add2 => {
                w.set_avee(false);
                w.set_adc(AdcCountSelect::Convert2);
            }
            #[cfg(any(adc12, adc14))]
            AverageMode::Add3 => {
                w.set_avee(false);
                w.set_adc(AdcCountSelect::Convert3);
            }
            #[cfg(any(adc12, adc14))]
            AverageMode::Add4 => {
                w.set_avee(false);
                w.set_adc(AdcCountSelect::Convert4);
            }
            #[cfg(any(adc12, adc14))]
            AverageMode::Add16 => {
                w.set_avee(false);
                w.set_adc(AdcCountSelect::Convert16);
            }
        });

        Self {
            _phantom: PhantomData,
            average_mode: config.average_mode,
            sample_time: config.sample_time,
        }
    }

    /// Returns the currently configured resolution (12 or 14-bits).
    pub fn resolution(&self) -> Resolution {
        #[cfg(adc12)]
        {
            Resolution::_12bit
        }

        #[cfg(adc14)]
        {
            let adc = I::regs();

            match adc.adcer().read().adprc() {
                Adprc::_12bit => Resolution::_12bit,
                Adprc::_RESERVED_1 => unimplemented!(),
                Adprc::_RESERVED_2 => unimplemented!(),
                Adprc::_14bit => Resolution::_14bit,
            }
        }

        #[cfg(adc16)]
        {
            Resolution::_16bit
        }
    }

    /// Returns the currently configured alignment. §35.2.1, §35.2.11
    #[cfg(adc14)]
    pub fn alignment(&self) -> Alignment {
        let adc = I::regs();

        match adc.adcer().read().adrfmt() {
            true => Alignment::FlushLeft,
            false => Alignment::FlushRight,
        }
    }

    /// Returns the on-die [temperature measurement pseudo-channel](Temperature) without performing configuration.
    pub fn temperature_channel(&self) -> Temperature {
        Temperature {}
    }

    /// Returns the [`Vref`] pseudo channel.
    pub fn vref_channel(&self) -> Vref {
        Vref {}
    }

    /// Returns a single reading from an ADC channel.
    /// Generic over `R` so we can return individual values or sequences as needed.
    pub fn blocking_read<R, C: AdcChannel<R>>(&self, channel: &C) -> R {
        let adc = I::regs();

        channel.enable::<I>(AdcChannelConfig {
            average_mode: self.average_mode,
            sample_time: self.sample_time,
        });

        // TODO: read ADST first to ensure we're stopped?

        adc.adcsr().modify(|w| w.set_adcs(Adcs::Single));

        adc.adcsr().modify(|w| w.set_adst(true));

        // When the conversion is finished an interrupt is fired (without modifying the registers) and the ADST bit is cleared
        while adc.adcsr().read().adst() {
            asm::nop()
        }

        channel.disable::<I>();

        channel.read_one::<I>()
    }

    /// Returns a single reading from an ADC channel via DMA.
    ///
    /// The channel selection registers are re-programmed on every call. Use
    /// [`configured_sequence`](Adc::configured_sequence) if you need repeated reads
    /// from the same channel without that per-call overhead.
    #[cfg(dmac)]
    pub async fn read<
        'a,
        R: crate::dmac::Word,
        RxDmaInstance: crate::dmac::Instance,
        C: AdcChannel<R>,
        DmaInt: InterruptType,
    >(
        &'a self,
        rx_dma: Peri<'a, RxDmaInstance>,
        channel: &C,
        reading: &mut R,
        irqs: impl Binding<DmaInt, DmacInterruptHandler<RxDmaInstance>>,
    ) {
        let adc = I::regs();

        let mut rx_dma = DmacChannel::new(rx_dma, irqs);

        channel.enable::<I>(AdcChannelConfig {
            average_mode: self.average_mode,
            sample_time: self.sample_time,
        });

        adc.adcsr().modify(|w| w.set_adcs(Adcs::Single));

        let rx = rx_dma.read::<R>(
            channel.address::<I>(),
            core::slice::from_mut(reading),
            I::RX_EVENT,
        );

        adc.adcsr().modify(|w| w.set_adst(true));
        rx.await;

        channel.disable::<I>();
    }

    /// Configure an ADC channel once and return a [`ConfiguredSequence`] for repeated DMA reads.
    ///
    /// Unlike [`read`](Adc::read), the channel selection registers are programmed only at
    /// construction. Each call to [`ConfiguredSequence::read`] skips that setup, which is useful
    /// when sampling the same channel at high rates.
    #[cfg(dmac)]
    pub fn configured_sequence<
        'a,
        R: crate::dmac::Word,
        RxDmaInstance: crate::dmac::Instance,
        C: AdcChannel<R>,
        DmaInt: InterruptType,
    >(
        &'a self,
        rx_dma: Peri<'a, RxDmaInstance>,
        channel: &C,
        irqs: impl Binding<DmaInt, DmacInterruptHandler<RxDmaInstance>> + 'a,
    ) -> ConfiguredSequence<'a, I, R> {
        let adc = I::regs();

        channel.enable::<I>(AdcChannelConfig {
            average_mode: self.average_mode,
            sample_time: self.sample_time,
        });

        adc.adcsr().modify(|w| w.set_adcs(Adcs::Single));

        ConfiguredSequence {
            dma: DmacChannel::new(rx_dma, irqs),
            source: channel.address::<I>(),
            _phantom: PhantomData,
        }
    }

    /// Configure an ADC channel for continuous DMA sampling into a ping-pong buffer.
    ///
    /// The ADC is placed in continuous scan mode: it runs without software re-triggering
    /// and each scan-end event fires the DMA to write one sample.
    ///
    /// Because the RA4M1 DMAC has no half-transfer interrupt, the driver implements a
    /// software ping-pong: the DMA fills half of `dma_buf` at a time. When that half is
    /// full, [`RingBufferedAdc::read`] re-arms the DMA for the other half before returning
    /// to the caller, keeping the gap where samples might be lost as short as possible.
    ///
    /// `dma_buf` must have an even length >= 2. Each call to [`RingBufferedAdc::read`]
    /// waits for `dma_buf.len() / 2` samples and copies them into the provided output.
    #[cfg(dmac)]
    pub fn into_ring_buffered<
        'a,
        R: crate::dmac::Word,
        RxDmaInstance: crate::dmac::Instance,
        C: AdcChannel<u16>,
        DmaInt: InterruptType,
    >(
        self,
        rx_dma: Peri<'a, RxDmaInstance>,
        channel: &C,
        dma_buf: &'a mut [R],
        irqs: impl Binding<DmaInt, DmacInterruptHandler<RxDmaInstance>> + 'a,
    ) -> RingBufferedAdc<'a, I, R>
    where
        'd: 'a,
    {
        assert!(
            dma_buf.len() >= 2 && dma_buf.len() % 2 == 0,
            "DMA buffer must have an even length >= 2"
        );

        let source = channel.address::<I>();

        channel.enable::<I>(AdcChannelConfig {
            average_mode: self.average_mode,
            sample_time: self.sample_time,
        });

        I::regs().adcsr().modify(|w| w.set_adcs(Adcs::Continuous));

        let mut dma = DmacChannel::new(rx_dma, irqs);
        dma.configure_peripheral_read::<R>(source as _, I::RX_EVENT);

        // Prevent the Adc drop from stopping the module; RingBufferedAdc::drop handles it.
        core::mem::forget(self);

        RingBufferedAdc {
            dma,
            source: source as _,
            buf: dma_buf.as_mut_ptr(),
            half_len: dma_buf.len() / 2,
            fill_half: false,
            started: false,
            _phantom: PhantomData,
        }
    }
}

/// An ADC continuously sampling one channel into a ping-pong DMA buffer.
///
/// Because the RA DMAC has no half-transfer interrupt, this driver implements a software
/// ping-pong: the DMA fills one half of the internal buffer (`dma_buf.len() / 2` samples),
/// signals completion via DTIF, and [`read`](RingBufferedAdc::read) immediately re-arms the
/// DMA for the other half before copying the completed half to the caller. The window where
/// incoming scan-end events are not forwarded to memory is kept to a few register writes.
///
/// Obtain via [`Adc::into_ring_buffered`].
#[cfg(dmac)]
#[allow(private_bounds)]
pub struct RingBufferedAdc<'d, I: Instance, R: crate::dmac::Word> {
    dma: DmacChannel<'d>,
    #[allow(dead_code)]
    source: *const R,
    buf: *mut R,
    half_len: usize,
    /// The half the DMA is currently filling (false = first, true = second).
    /// This is also the half that will be ready to read on the next DTIF.
    fill_half: bool,
    started: bool,
    _phantom: PhantomData<(&'d I, *mut R)>,
}

#[cfg(dmac)]
impl<'d, I: Instance, R: crate::dmac::Word> RingBufferedAdc<'d, I, R> {
    /// Arms the DMA to fill the half indicated by `self.fill_half`.
    #[inline]
    fn arm(&mut self) {
        let dest = unsafe {
            if self.fill_half {
                self.buf.add(self.half_len)
            } else {
                self.buf
            }
        };
        self.dma.rearm::<R>(dest, self.half_len);
    }

    /// Start continuous ADC sampling. Called automatically by [`read`](Self::read).
    pub fn start(&mut self) {
        if self.started {
            return;
        }
        self.started = true;
        self.fill_half = false;
        self.arm();
        I::regs().adcsr().modify(|w| w.set_adst(true));
    }

    /// Stop continuous ADC sampling.
    pub fn stop(&mut self) {
        I::regs().adcsr().modify(|w| w.set_adst(false));
        self.dma.disable_dte();
        self.started = false;
    }

    /// Read one half-buffer of ADC samples.
    ///
    /// Waits until the DMA has finished filling the current half, then immediately re-arms
    /// the DMA for the other half and copies the completed data into `buf`.
    ///
    /// `buf.len()` must equal `dma_buf.len() / 2` (i.e. exactly one half of the buffer
    /// passed to [`into_ring_buffered`](Adc::into_ring_buffered)).
    pub async fn read(&mut self, buf: &mut [R]) {
        #[cfg(feature = "strict-assert")]
        assert_eq!(
            buf.len(),
            self.half_len,
            "Buffer must be half the DMA buffer length"
        );

        if !self.started {
            self.start();
        }

        let fill_half = self.fill_half;

        core::future::poll_fn(|cx| {
            self.dma.register_waker(cx.waker());

            if self.dma.take_dtif() {
                // Switch halves and immediately re-arm so the gap is minimal.
                self.fill_half = !fill_half;
                self.arm();
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        // Copy the completed half (the one DMA just finished) to the caller's buffer.
        let src_offset = if fill_half { self.half_len } else { 0 };
        let src = unsafe { core::slice::from_raw_parts(self.buf.add(src_offset), self.half_len) };
        buf.copy_from_slice(src);
    }
}

#[cfg(dmac)]
impl<I: Instance, R: crate::dmac::Word> Drop for RingBufferedAdc<'_, I, R> {
    fn drop(&mut self) {
        if self.started {
            self.stop();
        }
        let adc = I::regs();
        adc.adansa(0).write_value(Adans(0));
        adc.adansa(1).write_value(Adans(0));
        adc.adexicr().modify(|w| {
            w.set_ocsa(false);
            w.set_tssa(false);
        });
        I::stop_module();
    }
}

/// An ADC with a pre-configured channel for repeated DMA reads.
///
/// Unlike [`Adc::read`], this type programs the ADC channel selection registers only once at
/// construction. Each call to [`read`](ConfiguredSequence::read) reuses the existing hardware
/// configuration, skipping the per-call overhead of reprogramming those registers.
///
/// Obtain via [`Adc::configured_sequence`].
#[cfg(dmac)]
#[allow(private_bounds)]
pub struct ConfiguredSequence<'d, I: Instance, R: crate::dmac::Word> {
    dma: DmacChannel<'d>,
    source: *const R,
    _phantom: PhantomData<(&'d I, R)>,
}

#[cfg(dmac)]
impl<'d, I: Instance, R: crate::dmac::Word> ConfiguredSequence<'d, I, R> {
    /// Trigger one DMA conversion of the pre-configured channel and wait for it to complete.
    pub async fn read(&mut self, reading: &mut R) {
        let adc = I::regs();
        let rx = self
            .dma
            .read::<R>(self.source, core::slice::from_mut(reading), I::RX_EVENT);
        adc.adcsr().modify(|w| w.set_adst(true));
        rx.await;
    }
}

#[cfg(dmac)]
impl<I: Instance, R: crate::dmac::Word> Drop for ConfiguredSequence<'_, I, R> {
    fn drop(&mut self) {
        let adc = I::regs();
        adc.adansa(0).write_value(Adans(0));
        adc.adansa(1).write_value(Adans(0));
        adc.adexicr().modify(|w| {
            w.set_ocsa(false);
            w.set_tssa(false);
        });
    }
}

impl<'d, I: Instance> Drop for Adc<'d, I> {
    fn drop(&mut self) {
        I::stop_module();
    }
}
