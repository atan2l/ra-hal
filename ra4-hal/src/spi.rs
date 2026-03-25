#![allow(missing_docs)]

//! Serial Peripheral Interface (`SPI`).
//!
//! # TODO
//! * Error handling
//! * Implement `embedded-hal` traits (0.2 and 1.0)
//! * Subnode mode
//! * Hardware support for multiple chip select lines
//! * Confirm DMAC transfers actually work at 24 MHz.

use core::{future::poll_fn, marker::PhantomData, ops::RangeInclusive, task::Poll};

use embassy_futures::join::join;
use embassy_hal_internal::{Peri, PeripheralType, interrupt::InterruptExt as _};
use embassy_sync::waitqueue::AtomicWaker;
use embedded_hal_1::spi::{ErrorType, MODE_0, Mode};

#[cfg(feature = "_dmac")]
use crate::dmac::{Channel as DmacChannel, DmacInterruptHandler};
#[cfg(feature = "_dtc")]
use crate::dtc::{Channel as DtcChannel, DtcInterruptHandler, Instance as DtcInstance};
use crate::{
    event_link::IcuInterrupt as _,
    gpio::{Basic, Flex, Pin, PortFunction},
    interrupt::{
        self,
        typelevel::{Handler as InterruptHandler, Interrupt as InterruptType},
    },
    module_stop::ModuleStop,
    pac::{
        self,
        spi::vals::{Brdv, Bysw, Cpha, Cpol, Lsbf, Moifv, Spb, Spbyt, Splw, Spms},
    },
};

const VALID_CLOCK_DIVIDER: RangeInclusive<u32> = 2..=4096;

pub struct ErrorInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

pub struct RxInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

pub struct TeInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

pub struct TxInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

impl<I: Instance, ErrInt: InterruptType> InterruptHandler<ErrInt> for ErrorInterruptHandler<I> {
    unsafe fn on_interrupt() {
        error!("{}ErrInt", I::PERIPHERAL);
    }
}

impl<I: Instance, RxInt: InterruptType> InterruptHandler<RxInt> for RxInterruptHandler<I> {
    unsafe fn on_interrupt() {
        trace!("{}RxInt", I::PERIPHERAL);
        RxInt::IRQ.icu_unpend();
        I::rx_waker().wake();
    }
}

impl<I: Instance, TeInt: InterruptType> InterruptHandler<TeInt> for TeInterruptHandler<I> {
    unsafe fn on_interrupt() {
        trace!("{}TeInt", I::PERIPHERAL);
        TeInt::IRQ.icu_unpend();
        I::te_waker().wake();
    }
}

impl<I: Instance, TxInt: InterruptType> InterruptHandler<TxInt> for TxInterruptHandler<I> {
    unsafe fn on_interrupt() {
        trace!("{}TxInt", I::PERIPHERAL);
        TxInt::IRQ.icu_unpend();
        I::tx_waker().wake();
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
struct ClockConfig {
    pub rate: u32,
    pub error: u32,
    pub spbr: u8,
    pub brdv: u8,
}

#[allow(private_bounds)]
pub trait TransferMode: SealedTransferMode {}
trait SealedTransferMode {}

#[cfg(feature = "_dmac")]
pub struct Dma<'d> {
    rx_dma: DmacChannel<'d>,
    tx_dma: DmacChannel<'d>,
}

#[cfg(feature = "_dtc")]
pub struct Dtc<D1: DtcInstance, D2: DtcInstance> {
    rx_dtc: DtcChannel<D1>,
    tx_dtc: DtcChannel<D2>,
}

pub struct Blocking {}

/// SPI driver for the `SPI` peripheral.
#[allow(private_bounds)]
pub struct Spi<'d, I: Instance, W: Word, M: TransferMode> {
    _instance: PhantomData<&'d I>,
    _miso: Flex<'d, Basic>,
    _mosi: Flex<'d, Basic>,
    _sck: Flex<'d, Basic>,
    _cs: Option<Flex<'d, Basic>>,
    _word: PhantomData<W>,
    bit_order: BitOrder,
    io_mode: M,
}

/// Whether the `CS` line is low or high when a subnode is selected.
///
/// Only applicable when the `SPI` peripheral is controlling the `CS` line.
/// Leave this at the default `ActiveLow` for most use cases.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq)]
pub enum ChipSelectPolarity {
    /// `CS` line is driven high for the selected subnode device.
    ActiveHigh,

    /// `CS` line is driven low for the selected subnode device.
    ActiveLow,
}

/// Order in which the bits are sent out over the wire.
///
/// # Notes
/// If `LsbFirst` is selected the bits are reversed upon reception.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq)]
pub enum BitOrder {
    #[allow(missing_docs)]
    Lsb0,
    #[allow(missing_docs)]
    Msb0,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq)]
pub enum Endian {
    #[allow(missing_docs)]
    Big,
    #[allow(missing_docs)]
    Little,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq)]
/// `MOSI` pin state at idle
pub enum MosiIdle {
    /// `MOSI` remains at last value
    Last,

    /// `MOSI` is set low when bus is idle
    Low,

    /// `MOSI` is set high when bus is idle
    High,
}

/// SPI configuration.
#[non_exhaustive]
#[derive(Copy, Clone)]
pub struct Config {
    #[allow(missing_docs)]
    pub bit_order: BitOrder,

    #[allow(missing_docs)]
    pub endian: Endian,

    /// SPI mode.
    pub mode: Mode,

    /// SPI bit rate in bits per second.
    pub bit_rate: u32,

    #[allow(missing_docs)]
    pub cs_polarity: ChipSelectPolarity,

    #[allow(missing_docs)]
    pub mosi_idle: MosiIdle,
}

/// Trait that defines the size of an SPI "word".
///
/// Currently 8-, 16-, and 32- bit words are supported as they cleanly map to `u8`, `u16`, and `u32` types respectively.
/// The `RA4M1` supports a variety of intermediate word sizes but that's an implementation detail for another day.
#[allow(private_bounds)]
#[cfg(feature = "defmt")]
pub trait Word: SealedWord + Copy + defmt::Format {}

/// Trait that defines the size of a "word".
///
/// Currently 8-, 16-, and 32- bit words are supported as they cleanly map to `u8`, `u16`, and `u32` types respectively.
/// The `RA4M1` supports a variety of intermediate word sizes but that's an implementation detail for another day.
#[allow(private_bounds)]
#[cfg(not(feature = "defmt"))]
pub trait Word: SealedWord + Copy {}

pub(crate) trait SealedWord {
    const BYTE_ACCESS: Spbyt;
    const WORD_LENGTH: Splw;
    const BIT_LENGTH: Spb;

    fn read(peri: &mut crate::pac::spi::Spi, bit_order: BitOrder) -> Self;
    fn write(peri: &mut crate::pac::spi::Spi, word: Self, bit_order: BitOrder);
}

impl Word for u8 {}
impl SealedWord for u8 {
    const BYTE_ACCESS: Spbyt = Spbyt::Byte;
    const WORD_LENGTH: Splw = Splw::HalfWord;
    const BIT_LENGTH: Spb = Spb::_8bits;

    #[inline(always)]
    fn read(peri: &mut crate::pac::spi::Spi, bit_order: BitOrder) -> Self {
        let by = peri.spdr_by().read();
        if bit_order == BitOrder::Lsb0 {
            by.reverse_bits()
        } else {
            by
        }
    }

    #[inline(always)]
    fn write(peri: &mut crate::pac::spi::Spi, word: Self, bit_order: BitOrder) {
        if bit_order == BitOrder::Lsb0 {
            peri.spdr_by().write_value(word.reverse_bits());
        } else {
            peri.spdr_by().write_value(word);
        }
    }
}

impl Word for u16 {}
impl SealedWord for u16 {
    const BYTE_ACCESS: Spbyt = Spbyt::Word;
    const WORD_LENGTH: Splw = Splw::HalfWord;
    const BIT_LENGTH: Spb = Spb::_16bits;

    #[inline(always)]
    fn read(peri: &mut crate::pac::spi::Spi, bit_order: BitOrder) -> Self {
        let ha = peri.spdr_ha().read();
        if bit_order == BitOrder::Lsb0 {
            ha.reverse_bits()
        } else {
            ha
        }
    }

    #[inline(always)]
    fn write(peri: &mut crate::pac::spi::Spi, word: Self, bit_order: BitOrder) {
        if bit_order == BitOrder::Lsb0 {
            let word = ((word & 0x00FF) as u8).reverse_bits() as u16
                | ((((word & 0xFF00) >> 8) as u8).reverse_bits() as u16) << 8;
            peri.spdr_ha().write_value(word);
        } else {
            peri.spdr().write_value(word as u32);
        }
    }
}

impl Word for u32 {}
impl SealedWord for u32 {
    const BYTE_ACCESS: Spbyt = Spbyt::Word;
    const WORD_LENGTH: Splw = Splw::Word;
    const BIT_LENGTH: Spb = Spb::_32bits;

    #[inline(always)]
    fn read(peri: &mut crate::pac::spi::Spi, bit_order: BitOrder) -> Self {
        let word = peri.spdr().read();
        if bit_order == BitOrder::Lsb0 {
            word.reverse_bits()
        } else {
            word
        }
    }

    #[inline(always)]
    fn write(peri: &mut crate::pac::spi::Spi, word: Self, bit_order: BitOrder) {
        if bit_order == BitOrder::Lsb0 {
            peri.spdr().write_value(word.reverse_bits());
        } else {
            peri.spdr().write_value(word);
        }
    }
}

/// [`Spi`] driver instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + ModuleStop + PeripheralType + 'static + Send {
    const RX_EVENT: crate::event_link::InterruptEvent;
    const TX_EVENT: crate::event_link::InterruptEvent;
    const TE_EVENT: crate::event_link::InterruptEvent;
    const ERROR_EVENT: crate::event_link::InterruptEvent;

    /// Waker for transmit buffer empty events.
    fn tx_waker() -> &'static AtomicWaker;
    fn te_waker() -> &'static AtomicWaker;
    fn rx_waker() -> &'static AtomicWaker;
}

pub(crate) trait SealedInstance {
    #[cfg(feature = "defmt")]
    const PERIPHERAL: &'static str;
    #[cfg(not(feature = "defmt"))]
    const PERIPHERAL: () = ();

    fn regs() -> pac::spi::Spi;
}

/// GPIO pin connected to the `MISO` line of an [`Spi`] instance.
#[allow(private_bounds)]
pub trait MisoPin<I: Instance>: SealedMisoPin<I> {}

pub(crate) trait SealedMisoPin<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_pfunc(&self) {
        self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

/// GPIO pin connected to the `MOSI` line of an [`Spi`] instance.
#[allow(private_bounds)]
pub trait MosiPin<I: Instance>: SealedMosiPin<I> {}

pub(crate) trait SealedMosiPin<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_pfunc(&self) {
        self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

/// GPIO pin connected to the `SCK` line of an [`Spi`] instance.
#[allow(private_bounds)]
pub trait SckPin<I: Instance>: SealedSckPin<I> {}

pub(crate) trait SealedSckPin<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_pfunc(&self) {
        self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

/// GPIO pin connected to the `SS0` line of an [`Spi`] instance.
#[allow(private_bounds)]
pub trait CsPin<I: Instance>: SealedCsPin<I> {}

pub(crate) trait SealedCsPin<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_pfunc(&self) {
        self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

macro_rules! miso_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::spi::MisoPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::spi::SealedMisoPin<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use miso_pin;

macro_rules! mosi_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::spi::MosiPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::spi::SealedMosiPin<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use mosi_pin;

macro_rules! sck_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::spi::SckPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::spi::SealedSckPin<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use sck_pin;

macro_rules! cs_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::spi::CsPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::spi::SealedCsPin<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use cs_pin;

macro_rules! instance_impl {
    ($instance:ident, $error_int:ident, $te_int:ident, $tx_int:ident, $rx_int:ident) => {
        paste::paste! {
            impl Instance for crate::peripherals::$instance {
                const RX_EVENT: crate::event_link::InterruptEvent = crate::event_link::InterruptEvent::$rx_int;
                const TX_EVENT: crate::event_link::InterruptEvent = crate::event_link::InterruptEvent::$tx_int;
                const TE_EVENT: crate::event_link::InterruptEvent = crate::event_link::InterruptEvent::$te_int;
                const ERROR_EVENT: crate::event_link::InterruptEvent = crate::event_link::InterruptEvent::$error_int;

                /// Waker for transmit buffer empty events.
                #[inline(always)]
                fn tx_waker() -> &'static AtomicWaker{
                    static TX_WAKER: AtomicWaker = AtomicWaker::new();
                    &TX_WAKER
                }

                /// Waker for transmit end events.
                #[inline(always)]
                fn te_waker() -> &'static AtomicWaker{
                    static TE_WAKER: AtomicWaker = AtomicWaker::new();
                    &TE_WAKER
                }

                #[inline(always)]
                fn rx_waker() -> &'static AtomicWaker{
                    static RX_WAKER: AtomicWaker = AtomicWaker::new();
                    &RX_WAKER
                }
            }

            impl SealedInstance for crate::peripherals::$instance {

                #[cfg(feature = "defmt")]
                const PERIPHERAL: &'static str = concat!(stringify!($instance), ": ");

                #[inline(always)]
                fn regs() -> crate::pac::spi::Spi {
                    crate::pac::$instance
                }
            }
        }
    };
}

instance_impl!(SPI0, Spi0SpEi, Spi0SpTend, Spi0SpTi, Spi0SpRi);
instance_impl!(SPI1, Spi1SpEi, Spi1SpTend, Spi1SpTi, Spi1SpRi);

fn calc_dividers(rate: u32) -> Result<ClockConfig, ()> {
    let clocks = crate::clock_config();
    let clk = clocks.peripheral_a;

    let target = rate;

    let mut divider = clk / target;

    if !VALID_CLOCK_DIVIDER.contains(&divider) {
        return Err(());
    }

    divider -= 1;

    let mut options: [Option<(u32, u32, u32, u32)>; 4] = [None; 4];

    for n in 0..=3 {
        let spbr = divider / (2 << n);
        let act = clk / (2 * (spbr + 1) * (2_u32.pow(n)));
        let diff = target.abs_diff(act);
        if spbr <= 255 {
            options[n as usize] = Some((n, spbr, diff, act));
        }
    }

    if !options.iter().any(|o| o.is_some()) {
        return Err(());
    }

    let best = options[0].unwrap();

    Ok(ClockConfig {
        error: best.2,
        spbr: best.1 as u8,
        brdv: best.0 as u8,
        rate,
    })
}

impl TransferMode for Blocking {}
impl SealedTransferMode for Blocking {}

#[cfg(feature = "_dmac")]
impl<'d> TransferMode for Dma<'d> {}
#[cfg(feature = "_dmac")]
impl<'d> SealedTransferMode for Dma<'d> {}

#[cfg(feature = "_dtc")]
impl<Rx: DtcInstance, Tx: DtcInstance> TransferMode for Dtc<Rx, Tx> {}
#[cfg(feature = "_dtc")]
impl<Rx: DtcInstance, Tx: DtcInstance> SealedTransferMode for Dtc<Rx, Tx> {}

impl<'d, I: Instance, W: Word> Spi<'d, I, W, Blocking> {
    /// Creates a new `Spi` instance with hardware control of the chip select line.
    pub fn new_blocking(
        spi: Peri<'d, I>,
        sck: Peri<'d, impl SckPin<I>>,
        mosi: Peri<'d, impl MosiPin<I>>,
        miso: Peri<'d, impl MisoPin<I>>,
        cs: Peri<'d, impl CsPin<I>>,
        config: Config,
    ) -> Self {
        cs.set_pfunc();
        Self::new_inner(
            spi,
            sck,
            mosi,
            miso,
            Some(Flex::new(cs)),
            config,
            Blocking {},
        )
    }

    /// Writes one or more words to the SPI bus.
    ///
    /// # Notes
    /// Calls to this function should be wrapped with [`blocking_setup`](Self::blocking_setup) and [`blocking_shutdown`](Self::blocking_shutdown) as required, the `SpiBus` impl does this automatically.
    #[inline(always)]
    pub fn blocking_write(&mut self, bytes: &[W]) -> Result<(), SpiError> {
        let mut spi = I::regs();

        for byte in bytes.iter() {
            while !spi.spsr().read().sptef() {}
            W::write(&mut spi, *byte, self.bit_order);
        }

        Ok(())
    }

    /// Sets up the peripheral for a read/write/transfer operation.
    pub fn blocking_setup(&mut self) {
        let spi = I::regs();
        spi.spcr2().write(|r| r.set_spiie(false));
        spi.spcr().modify(|r| {
            r.set_sptie(false);
            r.set_sprie(false);
            r.set_speie(false);
            r.set_spe(true);
        });
    }

    /// Disables the peripheral and related interrupts.
    pub fn blocking_shutdown(&mut self) {
        let spi = I::regs();

        // Okay that last part was a lie but we're not enabling any interrupts.
        spi.spcr().modify(|r| r.set_spe(false));
    }

    /// Performs a concurrent read and write.
    ///
    /// # Notes
    /// Calls to this function should be wrapped with [`blocking_setup`](Self::blocking_setup) and [`blocking_shutdown`](Self::blocking_shutdown) as required, the `SpiBus` impl does this automatically.
    pub fn blocking_transfer(&mut self, input: &mut [W], output: &[W]) -> Result<(), SpiError> {
        #[cfg(feature = "strict-assert")]
        assert_eq!(input.len(), output.len());

        let mut spi = I::regs();

        for (obyte, ibyte) in output.iter().zip(input.iter_mut()) {
            while !spi.spsr().read().sptef() {}
            W::write(&mut spi, *obyte, self.bit_order);

            while !spi.spsr().read().sprf() {}
            let word = W::read(&mut spi, self.bit_order);
            *ibyte = word;

            if spi.spsr().read().ovrf() {
                return Err(SpiError::Overrun);
            }
        }

        Ok(())
    }

    /// Reads one or more words from the SPI bus.
    ///
    /// # Notes
    /// Calls to this function should be wrapped with [`blocking_setup`](Self::blocking_setup) and [`blocking_shutdown`](Self::blocking_shutdown) as required, the `SpiBus` impl does this automatically.
    #[inline(always)]
    pub fn blocking_read(&mut self, buf: &mut [W]) -> Result<(), SpiError> {
        let mut spi = I::regs();

        let mut ret = Ok(());

        for byte in buf.iter_mut() {
            while !spi.spsr().read().sprf() {}
            let datum = W::read(&mut spi, self.bit_order);
            *byte = datum;
            if spi.spsr().read().ovrf() {
                ret = Err(SpiError::Overrun);
                break;
            }
        }

        ret
    }
}

#[cfg(feature = "_dmac")]
impl<'d, I: Instance, W: Word + crate::dmac::Word> Spi<'d, I, W, Dma<'d>> {
    /// Creates a new `Spi` instance with hardware control of the chip select line.
    #[allow(clippy::too_many_arguments)]
    pub fn new_dma<
        TeInt: InterruptType,
        RxInt: InterruptType,
        TxInt: InterruptType,
        RxDmaInstance: crate::dmac::Instance,
        TxDmaInstance: crate::dmac::Instance,
    >(
        spi: Peri<'d, I>,
        sck: Peri<'d, impl SckPin<I>>,
        mosi: Peri<'d, impl MosiPin<I>>,
        miso: Peri<'d, impl MisoPin<I>>,
        cs: Peri<'d, impl CsPin<I>>,
        config: Config,
        rx_dmac: Peri<'d, RxDmaInstance>,
        tx_dmac: Peri<'d, TxDmaInstance>,
        irqs: impl interrupt::typelevel::Binding<TeInt, TeInterruptHandler<I>>
        + interrupt::typelevel::Binding<TxInt, DmacInterruptHandler<TxDmaInstance>>
        + interrupt::typelevel::Binding<RxInt, DmacInterruptHandler<RxDmaInstance>>
        + Clone
        + 'd,
    ) -> Self {
        unsafe { TeInt::IRQ.enable() };

        // Enable in ICU
        TeInt::IRQ.icu_enable(I::TE_EVENT);

        let rx_dma = DmacChannel::new(rx_dmac, irqs.clone());
        let tx_dma = DmacChannel::new(tx_dmac, irqs.clone());
        let dma = Dma { rx_dma, tx_dma };

        cs.set_pfunc();
        Self::new_inner(spi, sck, mosi, miso, Some(Flex::new(cs)), config, dma)
    }

    pub async fn dma_write(&mut self, words: &[W]) -> Result<(), SpiError> {
        let spi = I::regs();

        spi.spcr().modify(|r| {
            r.set_sptie(true);
            r.set_sprie(false);
            r.set_speie(false);
        });
        spi.spcr2().modify(|r| r.set_spiie(false));

        let tx = self
            .io_mode
            .tx_dma
            .write::<W>(words, spi.spdr().as_ptr() as *mut _, I::TX_EVENT);

        spi.spcr().modify(|r| r.set_spe(true));

        tx.await;

        spi.spcr2().modify(|r| r.set_spiie(true));
        poll_fn(|ctx| {
            I::te_waker().register(ctx.waker());
            if spi.spsr().read().sptef() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        spi.spcr().modify(|r| {
            r.set_sptie(false);
            r.set_spe(false);
        });
        spi.spcr2().modify(|r| r.set_spiie(false));

        Ok(())
    }

    pub async fn dma_transfer(&mut self, input: &mut [W], output: &[W]) -> Result<(), SpiError> {
        #[cfg(feature = "strict-assert")]
        assert_eq!(input.len(), output.len());

        let spi = I::regs();

        spi.spcr().modify(|r| {
            r.set_sptie(true);
            r.set_sprie(true);
            r.set_speie(false);
        });
        spi.spcr2().modify(|r| r.set_spiie(false));

        let tx = self
            .io_mode
            .tx_dma
            .write::<W>(output, spi.spdr().as_ptr() as *mut _, I::TX_EVENT);
        let rx = self
            .io_mode
            .rx_dma
            .read::<W>(spi.spdr().as_ptr() as *const _, input, I::RX_EVENT);

        spi.spcr().modify(|r| r.set_spe(true));

        join(tx, rx).await;

        spi.spcr2().modify(|r| r.set_spiie(true));
        poll_fn(|ctx| {
            I::te_waker().register(ctx.waker());
            if spi.spsr().read().sptef() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        spi.spcr().modify(|r| {
            r.set_sptie(false);
            r.set_sprie(false);
            r.set_speie(false);
            r.set_spe(false);
        });
        spi.spcr2().modify(|r| r.set_spiie(false));

        spi.spcr().modify(|r| r.set_spe(false));

        Ok(())
    }
}

#[cfg(feature = "_dtc")]
impl<'d, I: Instance, W: Word + crate::dtc::Word, Rx: DtcInstance, Tx: DtcInstance>
    Spi<'d, I, W, Dtc<Rx, Tx>>
{
    #[allow(clippy::too_many_arguments)]
    pub fn new_dtc<TeInt: InterruptType>(
        spi: Peri<'d, I>,
        sck: Peri<'d, impl SckPin<I>>,
        mosi: Peri<'d, impl MosiPin<I>>,
        miso: Peri<'d, impl MisoPin<I>>,
        cs: Peri<'d, impl CsPin<I>>,
        config: Config,
        rx_dtc: Peri<'d, Rx>,
        tx_dtc: Peri<'d, Tx>,
        irqs: impl interrupt::typelevel::Binding<TeInt, TeInterruptHandler<I>>
        + interrupt::typelevel::Binding<Rx::Int, DtcInterruptHandler<Rx>>
        + interrupt::typelevel::Binding<Tx::Int, DtcInterruptHandler<Tx>>
        + Clone
        + 'd,
    ) -> Self {
        unsafe { TeInt::IRQ.enable() };

        // Enable in ICU
        TeInt::IRQ.icu_enable(I::TE_EVENT);

        let rx_dtc = DtcChannel::new(rx_dtc, irqs.clone());
        let tx_dtc = DtcChannel::new(tx_dtc, irqs.clone());
        let dtc = Dtc { rx_dtc, tx_dtc };

        cs.set_pfunc();
        Self::new_inner(spi, sck, mosi, miso, Some(Flex::new(cs)), config, dtc)
    }

    pub async fn dtc_write(&mut self, words: &[W]) -> Result<(), SpiError> {
        let spi = I::regs();

        spi.spcr().modify(|r| {
            r.set_sptie(true);
            r.set_sprie(false);
            r.set_speie(false);
        });
        spi.spcr2().modify(|r| r.set_spiie(false));

        let tx = unsafe {
            self.io_mode
                .tx_dtc
                .write(words, spi.spdr().as_ptr() as *mut _, I::TX_EVENT, false)
        };

        spi.spcr().modify(|r| r.set_spe(true));

        tx.await;

        spi.spcr2().modify(|r| r.set_spiie(true));
        poll_fn(|ctx| {
            I::te_waker().register(ctx.waker());
            if spi.spsr().read().sptef() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        spi.spcr().modify(|r| {
            r.set_sptie(false);
            r.set_spe(false);
        });
        spi.spcr2().modify(|r| r.set_spiie(false));

        Ok(())
    }

    pub async fn dtc_transfer(&mut self, input: &mut [W], output: &[W]) -> Result<(), SpiError> {
        #[cfg(feature = "strict-assert")]
        assert_eq!(input.len(), output.len());

        let spi = I::regs();

        spi.spcr().modify(|r| {
            r.set_sptie(true);
            r.set_sprie(true);
            r.set_speie(false);
        });
        spi.spcr2().modify(|r| r.set_spiie(false));

        let tx = unsafe {
            self.io_mode
                .tx_dtc
                .write(output, spi.spdr().as_ptr() as *mut _, I::TX_EVENT, false)
        };
        let rx = unsafe {
            self.io_mode
                .rx_dtc
                .read(spi.spdr().as_ptr() as *mut _, input, I::RX_EVENT, false)
        };

        spi.spcr().modify(|r| r.set_spe(true));

        join(tx, rx).await;

        spi.spcr2().modify(|r| r.set_spiie(true));
        poll_fn(|ctx| {
            I::te_waker().register(ctx.waker());
            if spi.spsr().read().sptef() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        spi.spcr().modify(|r| {
            r.set_sptie(false);
            r.set_sprie(false);
            r.set_speie(false);
            r.set_spe(false);
        });
        spi.spcr2().modify(|r| r.set_spiie(false));

        spi.spcr().modify(|r| r.set_spe(false));

        Ok(())
    }
}

impl<'d, I: Instance, W: Word, M: TransferMode> Spi<'d, I, W, M> {
    fn new_inner(
        spi: Peri<'d, I>,
        sck: Peri<'d, impl SckPin<I>>,
        mosi: Peri<'d, impl MosiPin<I>>,
        miso: Peri<'d, impl MisoPin<I>>,
        cs: Option<Flex<'d, Basic>>,
        config: Config,
        io_mode: M,
    ) -> Self {
        let _ = spi;

        I::start_module();

        sck.set_pfunc();
        mosi.set_pfunc();
        miso.set_pfunc();

        let spi = I::regs();

        spi.spcr().modify(|r| r.set_spe(false));

        spi.spcr().modify(|r| {
            r.set_txmd(false);
            r.set_mstr(true);
            r.set_modfen(false);
            r.set_spms(Spms::Spi);
        });

        let polarity = match config.mode.polarity {
            embedded_hal_1::spi::Polarity::IdleLow => Cpol::Low,
            embedded_hal_1::spi::Polarity::IdleHigh => Cpol::High,
        };

        let phase = match config.mode.phase {
            embedded_hal_1::spi::Phase::CaptureOnFirstTransition => Cpha::SampleShift,
            embedded_hal_1::spi::Phase::CaptureOnSecondTransition => Cpha::ShiftSample,
        };

        let bit_order = match config.bit_order {
            BitOrder::Lsb0 => Lsbf::Lsb0,
            BitOrder::Msb0 => Lsbf::Msb0,
        };

        let endian = match config.endian {
            Endian::Big => Bysw::Big,
            Endian::Little => Bysw::Little,
        };

        let (moifv, moife) = match config.mosi_idle {
            MosiIdle::Last => (Moifv::Low, true),
            MosiIdle::Low => (Moifv::Low, false),
            MosiIdle::High => (Moifv::High, false),
        };

        debug!(
            "{}polarity={}, phase={}, bit_order={}, endian={}",
            I::PERIPHERAL,
            polarity,
            phase,
            bit_order,
            endian,
        );

        spi.spcmd0().modify(|r| {
            r.set_cpol(polarity);
            r.set_cpha(phase);
            r.set_lsbf(bit_order);
        });

        // We can get away with write because we're not enabling loopback
        spi.sppcr().write(|r| {
            r.set_moifv(moifv);
            r.set_moife(moife);
        });

        // At 48 MHz:
        //   The slowest clock is about 12 kHz
        //   The fastest clock is about 24 MHz
        let dividers = calc_dividers(config.bit_rate).unwrap();
        debug!("{}{}", I::PERIPHERAL, dividers);
        let spbr = dividers.spbr;
        let brdv = Brdv::from_bits(dividers.brdv);

        spi.spbr().write_value(spbr);
        spi.spdcr().modify(|r| r.set_spbyt(W::BYTE_ACCESS));
        spi.spdcr2().modify(|r| r.set_bysw(endian));
        spi.spdcr().modify(|r| r.set_splw(W::WORD_LENGTH));
        spi.spcmd0().modify(|r| {
            r.set_brdv(brdv);
            r.set_spb(W::BIT_LENGTH);
        });

        Self {
            _instance: PhantomData,
            _cs: cs,
            _miso: Flex::new(miso),
            _mosi: Flex::new(mosi),
            _sck: Flex::new(sck),
            _word: PhantomData,
            bit_order: config.bit_order,
            io_mode,
        }
    }
}

impl<'d, I: Instance, W: Word, M: TransferMode> Drop for Spi<'d, I, W, M> {
    fn drop(&mut self) {
        I::stop_module();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bit_order: BitOrder::Msb0,
            mode: MODE_0,
            bit_rate: 1_000_000,
            cs_polarity: ChipSelectPolarity::ActiveLow,
            endian: Endian::Big,
            mosi_idle: MosiIdle::High,
        }
    }
}

/// SPI error.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpiError {
    Unknown,
    Overrun,
}

impl core::fmt::Display for SpiError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SpiError::Unknown => write!(f, "Unknown"),
            SpiError::Overrun => write!(f, "Overrun"),
        }
    }
}

impl core::error::Error for SpiError {}

impl embedded_hal_1::spi::Error for SpiError {
    fn kind(&self) -> embedded_hal_1::spi::ErrorKind {
        match *self {
            SpiError::Unknown => embedded_hal_1::spi::ErrorKind::Other,
            SpiError::Overrun => embedded_hal_1::spi::ErrorKind::Overrun,
        }
    }
}

impl<'d, I: Instance, W: Word + 'static, M: TransferMode> ErrorType for Spi<'d, I, W, M> {
    type Error = SpiError;
}

impl<'d, I: Instance, W: Word + 'static> embedded_hal_1::spi::SpiBus<W>
    for Spi<'d, I, W, Blocking>
{
    #[inline]
    fn read(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
        self.blocking_setup();
        self.blocking_read(words)?;
        self.blocking_shutdown();
        Ok(())
    }

    #[inline]
    fn write(&mut self, words: &[W]) -> Result<(), Self::Error> {
        self.blocking_setup();
        self.blocking_write(words)?;
        self.blocking_shutdown();
        Ok(())
    }

    #[inline]
    fn transfer(&mut self, read: &mut [W], write: &[W]) -> Result<(), Self::Error> {
        self.blocking_setup();
        self.blocking_transfer(read, write)?;
        self.blocking_shutdown();
        Ok(())
    }

    fn transfer_in_place(&mut self, _words: &mut [W]) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(feature = "_dmac")]
impl<'d, I: Instance, W: Word + crate::dmac::Word + 'static> embedded_hal_async::spi::SpiBus<W>
    for Spi<'d, I, W, Dma<'d>>
{
    async fn read(&mut self, _words: &mut [W]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn write(&mut self, words: &[W]) -> Result<(), Self::Error> {
        self.dma_write(words).await
    }

    async fn transfer(&mut self, input: &mut [W], output: &[W]) -> Result<(), Self::Error> {
        self.dma_transfer(input, output).await
    }

    async fn transfer_in_place(&mut self, _words: &mut [W]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
