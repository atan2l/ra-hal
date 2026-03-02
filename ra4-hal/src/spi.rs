//! Serial Peripheral Interface (`SPI`).
//!
//! # TODO
//! * Get the clock settings right for `HOCO` ≠ 48 MHz
//! * Error handling
//! * Implement `embedded-hal` traits (0.2 and 1.0)
//! * Slave mode
//! * Hardware support for multiple slave select lines

use core::marker::PhantomData;

use cortex_m::asm;
use embassy_hal_internal::{Peri, PeripheralType};
use embedded_hal_1::spi::{MODE_0, Mode};
use ra4m1_ctpac::spi::vals::{Brdv, Cpha, Cpol, Lsbf, Spb, Spbyt, Splw, Spms, Sprdtd};

use crate::{
    gpio::{Basic, Flex, Pin, PortFunction},
    pac,
};

/// SPI driver for the `SPI` peripheral.
#[allow(private_bounds)]
pub struct Spi<'d, I: Instance, W: Word> {
    _instance: PhantomData<&'d I>,
    _miso: Flex<'d, Basic>,
    _mosi: Flex<'d, Basic>,
    _sck: Flex<'d, Basic>,
    _ss: Option<Flex<'d, Basic>>,
    _word: PhantomData<W>,
    bit_order: BitOrder,
}

/// Whether the `SS` line is low or high when a slave is selected.
///
/// Only applicable when the `SPI` peripheral is controlling the `SS` line.
/// Leave this at the default `ActiveLow` for most use cases.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq)]
pub enum SlaveSelectPolarity {
    /// `SS` line is driven high for the selected slave device.
    ActiveHigh,

    /// `SS` line is driven low for the selected slave device.
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
    LsbFirst,
    #[allow(missing_docs)]
    MsbFirst,
}

/// SPI configuration.
#[non_exhaustive]
pub struct Config {
    #[allow(missing_docs)]
    pub bit_order: BitOrder,

    /// SPI mode.
    pub mode: Mode,

    /// SPI bit rate in bits per second.
    ///
    /// Currently supported:
    /// * 24,000,000
    /// * 12,000,000
    /// * 1,000,000 (*default*)
    /// * 500,000
    /// * 250,000
    /// * 150,000
    /// * 125,000
    /// * 11,000
    pub bit_rate: u32,

    #[allow(missing_docs)]
    pub ss_polarity: SlaveSelectPolarity,
}

/// Trait that defines the size of a "word".
///
/// Currently 8-, 16-, and 32- bit words are supported as they cleanly map to `u8`, `u16`, and `u32` types respectively.
/// The `RA4M1` supports a variety of intermediate word sizes but that's an implementation detail for another day.
#[allow(private_bounds)]
pub trait Word: SealedWord {}

pub(crate) trait SealedWord: Copy + Into<u32> {
    const BYTE_ACCESS: Spbyt;
    const WORD_LENGTH: Splw;
    const BIT_LENGTH: Spb;

    fn read(reg: crate::pac::common::Reg<u32, crate::pac::common::RW>, bit_order: BitOrder)
    -> Self;

    fn write(reg: crate::pac::common::Reg<u32, crate::pac::common::RW>, word: Self);
}

impl Word for u8 {}
impl SealedWord for u8 {
    const BYTE_ACCESS: Spbyt = Spbyt::Byte;
    const WORD_LENGTH: Splw = Splw::HalfWord;
    const BIT_LENGTH: Spb = Spb::_8bits;

    #[inline(always)]
    fn read(
        reg: crate::pac::common::Reg<u32, crate::pac::common::RW>,
        bit_order: BitOrder,
    ) -> Self {
        let ret = (reg.read() & 0xFF) as u8;
        if bit_order == BitOrder::LsbFirst {
            ret.reverse_bits()
        } else {
            ret
        }
    }

    #[inline(always)]
    fn write(reg: crate::pac::common::Reg<u32, crate::pac::common::RW>, word: u8) {
        reg.write_value(u32::from(word) << 24);
    }
}

impl Word for u16 {}
impl SealedWord for u16 {
    const BYTE_ACCESS: Spbyt = Spbyt::Word;
    const WORD_LENGTH: Splw = Splw::HalfWord;
    const BIT_LENGTH: Spb = Spb::_16bits;

    #[inline(always)]
    fn read(
        reg: crate::pac::common::Reg<u32, crate::pac::common::RW>,
        bit_order: BitOrder,
    ) -> Self {
        let ret = (reg.read() & 0xFFFF) as u16;
        if bit_order == BitOrder::LsbFirst {
            ret.reverse_bits()
        } else {
            ret
        }
    }

    #[inline(always)]
    fn write(reg: crate::pac::common::Reg<u32, crate::pac::common::RW>, word: u16) {
        reg.write_value(u32::from(word) << 16);
    }
}

impl Word for u32 {}
impl SealedWord for u32 {
    const BYTE_ACCESS: Spbyt = Spbyt::Word;
    const WORD_LENGTH: Splw = Splw::Word;
    const BIT_LENGTH: Spb = Spb::_32bits;

    #[inline(always)]
    fn read(
        reg: crate::pac::common::Reg<u32, crate::pac::common::RW>,
        bit_order: BitOrder,
    ) -> Self {
        let ret = reg.read();
        if bit_order == BitOrder::LsbFirst {
            ret.reverse_bits()
        } else {
            ret
        }
    }

    #[inline(always)]
    fn write(reg: crate::pac::common::Reg<u32, crate::pac::common::RW>, word: u32) {
        reg.write_value(word);
    }
}

/// [`Spi`] driver instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

pub(crate) trait SealedInstance {
    fn regs() -> pac::spi::Spi;
    fn module_stop();
    fn module_start();
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
pub trait SsPin<I: Instance>: SealedSsPin<I> {}

pub(crate) trait SealedSsPin<I: SealedInstance>: Pin + PeripheralType {
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

macro_rules! ss_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::spi::SsPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::spi::SealedSsPin<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use ss_pin;

macro_rules! instance_impl {
    ($instance:ident, $mstp:ident, $te_int:ident, $tx_int:ident, $rx_int:ident) => {
        paste::paste! {
            impl Instance for crate::peripherals::$instance {}
            impl SealedInstance for crate::peripherals::$instance {

                #[inline(always)]
                fn regs() -> crate::pac::spi::Spi {
                    crate::pac::$instance
                }

                #[inline(always)]
                fn module_stop() {
                    debug!("{}: stop=true", stringify!($instance));
                    let mstp = crate::pac::MSTP;
                    mstp.mstpcrb().modify(|w| w.[< set_ $mstp >](true));
                }

                #[inline(always)]
                fn module_start() {
                    debug!("{}: stop=false", stringify!($instance));
                    let mstp = crate::pac::MSTP;
                    mstp.mstpcrb().modify(|w| w.[< set_ $mstp >](false));
                }
            }
        }
    };
}

instance_impl!(SPI0, mstpb19, Spi0SpTend, Spi0SpTi, Spi0SpRi);
instance_impl!(SPI1, mstpb18, Spi1SpTend, Spi1SpTi, Spi1SpRi);

impl<'d, I: Instance, W: Word> Spi<'d, I, W> {
    /// Creates a new `Spi` instance with hardware control of the slave select line.
    pub fn new_with_ss(
        spi: Peri<'d, I>,
        sck: Peri<'d, impl SckPin<I>>,
        mosi: Peri<'d, impl MosiPin<I>>,
        miso: Peri<'d, impl MisoPin<I>>,
        ss: Peri<'d, impl SsPin<I>>,
        config: Config,
    ) -> Self {
        ss.set_pfunc();
        Self::new_inner(spi, sck, mosi, miso, Some(Flex::new(ss)), config)
    }

    /// Creates a new `Spi` instance with end-user control of the slave select line.
    pub fn new(
        spi: Peri<'d, I>,
        sck: Peri<'d, impl SckPin<I>>,
        mosi: Peri<'d, impl MosiPin<I>>,
        miso: Peri<'d, impl MisoPin<I>>,
        config: Config,
    ) -> Self {
        Self::new_inner(spi, sck, mosi, miso, None, config)
    }

    fn new_inner(
        spi: Peri<'d, I>,
        sck: Peri<'d, impl SckPin<I>>,
        mosi: Peri<'d, impl MosiPin<I>>,
        miso: Peri<'d, impl MisoPin<I>>,
        ss: Option<Flex<'d, Basic>>,
        config: Config,
    ) -> Self {
        let _ = spi;

        I::module_start();

        sck.set_pfunc();
        mosi.set_pfunc();
        miso.set_pfunc();

        let spi = I::regs();

        spi.spcr().modify(|r| r.set_spe(false));

        spi.spcr().modify(|r| {
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
            BitOrder::LsbFirst => Lsbf::Lsb,
            BitOrder::MsbFirst => Lsbf::Msb,
        };

        info!("Polarity: {}, Phase: {}", polarity, phase);

        spi.spcmd0().modify(|r| {
            r.set_cpol(polarity);
            r.set_cpha(phase);
            r.set_lsbf(bit_order);
        });

        // bps = PCLKA / (2 * (SPBR+1) * (2**BRDV))
        // BRDV = 0,1,2, or 3
        // At 48 MHz:
        //   The slowest clock is about 11 kHz
        //   The fastest clock is about 24 MHz
        #[cfg(feature = "hoco_48mhz")]
        let (spbr, brdv) = match config.bit_rate {
            24_000_000 => (0, Brdv::_00),
            12_000_000 => (1, Brdv::_00),
            1_000_000 => (11, Brdv::_01),
            500_000 => (11, Brdv::_10),
            250_000 => (11, Brdv::_11),
            150_000 => (39, Brdv::_10),
            125_000 => (47, Brdv::_10),
            11_000 => (255, Brdv::_11),
            _ => unimplemented!(),
        };

        spi.spbr().write_value(spbr);
        spi.spcmd0().modify(|r| r.set_brdv(brdv));

        spi.spdcr().modify(|r| r.set_spbyt(W::BYTE_ACCESS));
        spi.spdcr().modify(|r| r.set_splw(W::WORD_LENGTH));
        spi.spcmd0().modify(|r| r.set_spb(W::BIT_LENGTH));

        spi.spcr().modify(|r| r.set_spe(true));

        Self {
            _instance: PhantomData,
            _ss: ss,
            _miso: Flex::new(miso),
            _mosi: Flex::new(mosi),
            _sck: Flex::new(sck),
            _word: PhantomData,
            bit_order: config.bit_order,
        }
    }

    /// Writes one or more words to the SPI bus.
    #[inline(always)]
    pub fn write(&mut self, bytes: &[W]) {
        let spi = I::regs();

        spi.spdcr().modify(|r| r.set_sprdtd(Sprdtd::TxBuf));

        spi.spcr().modify(|r| {
            r.set_sptie(false);
            r.set_sprie(false);
            r.set_speie(false);
        });
        spi.spcr2().modify(|r| r.set_spiie(false));

        for byte in bytes.iter() {
            while !spi.spsr().read().sptef() {
                asm::nop();
            }
            W::write(spi.spdr(), *byte);
        }

        spi.spcr().modify(|r| r.set_sptie(false));
        spi.spcr2().modify(|r| r.set_spiie(false));

        while !spi.spsr().read().idlnf() {
            asm::nop();
        }
    }

    /// Reads one or more words from the SPI bus.
    #[inline(always)]
    pub fn read(&mut self, buf: &mut [W]) {
        let spi = I::regs();
        spi.spdcr().modify(|r| r.set_sprdtd(Sprdtd::RxBuf));

        spi.spcr().modify(|r| {
            r.set_sptie(false);
            r.set_sprie(false);
            r.set_speie(false);
        });
        spi.spcr2().modify(|r| r.set_spiie(false));

        for byte in buf.iter_mut() {
            while !spi.spsr().read().sprf() {
                asm::nop();
            }
            let datum = W::read(spi.spdr(), self.bit_order);
            *byte = datum;
        }
    }
}

impl<'d, I: Instance, W: Word> Drop for Spi<'d, I, W> {
    fn drop(&mut self) {
        I::module_stop();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bit_order: BitOrder::LsbFirst,
            mode: MODE_0,
            bit_rate: 1_000_000,
            ss_polarity: SlaveSelectPolarity::ActiveLow,
        }
    }
}
