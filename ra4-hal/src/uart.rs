//! `UART` Universal Asynchronous Receiver-Transmitter implemented using the `SCI` peripheral.

use core::marker::PhantomData;

use cortex_m::asm;
use embassy_hal_internal::{
    Peri, PeripheralType, atomic_ring_buffer::RingBuffer, interrupt::InterruptExt as _,
};
use paste::paste;
use ra4m1_ctpac::sci0::{
    regs::{Ftdrl, Scr},
    vals::{ScrCke, SmrCks, SmrPm, Stop, Ttrg},
};

use crate::interrupt::typelevel::{Handler as InterruptHandler, Interrupt};
use crate::{
    event_link::{IcuEventer, InterruptEvent},
    gpio::{AnyPin, Pin, PortFunction},
    interrupt, pac, peripherals,
};

/// UART driver.
#[allow(private_bounds)]
pub struct Uart<
    'd,
    I: Instance,
    RxInt: Interrupt + IcuEventer,
    TxInt: Interrupt + IcuEventer,
    TeInt: Interrupt + IcuEventer,
> {
    _phantom: PhantomData<&'d I>,
    _phantom_rx: PhantomData<&'d RxInt>,
    _phantom_tx: PhantomData<&'d TxInt>,
    _phantom_te: PhantomData<&'d TeInt>,
}

/// Interrupt handler that handles incoming data for an `SCI` instance.
pub struct RxInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

/// Interrupt handler that handles outgoing data (tx buffer empty) for an `SCI` instance.
pub struct TxInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

/// Interrupt handler that handles outgoing data (transmission end) for an `SCI` instance.
pub struct TeInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

trait SealedInstance {
    #[cfg(feature = "defmt")]
    const PERIPHERAL: &'static str;
    #[cfg(not(feature = "defmt"))]
    const PERIPHERAL: () = ();
    const RX_INTERRUPT_EVENT: InterruptEvent;
    const TX_INTERRUPT_EVENT: InterruptEvent;
    const TE_INTERRUPT_EVENT: InterruptEvent;

    /// `SCI0` and `SCI1` have 16-byte FIFO buffers for RX and TX ops per Table 28.1.
    const FIFO_DEPTH: u8 = 16;

    fn regs() -> pac::sci0::Sci0;
    fn start();
    fn stop();

    fn tx_buffer() -> &'static RingBuffer;
    fn rx_buffer() -> &'static RingBuffer;
}

trait TxPinSealed<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn pfunc(&self) -> PortFunction {
        Self::PERIPHERAL_FUNC
    }
}

trait RxPinSealed<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn pfunc(&self) -> PortFunction {
        Self::PERIPHERAL_FUNC
    }
}

/// An I/O pin being used for transmission by an `SCI` peripheral instance.
#[allow(private_bounds)]
pub struct TxPin<'d, I: SealedInstance> {
    // TODO: Should we remove this field?
    _pin: Peri<'d, AnyPin>,
    _phantom_i: PhantomData<I>,
}

/// An I/O pin being used for reception by an `SCI` peripheral instance.
#[allow(private_bounds)]
pub struct RxPin<'d, I: SealedInstance> {
    // TODO: Should we remove this field?
    _pin: Peri<'d, AnyPin>,
    _phantom_i: PhantomData<I>,
}

#[allow(private_bounds)]
impl<'d, I: SealedInstance> TxPin<'d, I> {
    /// Takes ownership of a pin and configures it to be used as an `SCI` TX line.
    pub fn new(pin: Peri<'d, impl TxPinSealed<I>>) -> Self {
        debug!("TX: {}/{}", pin._port(), pin._pin());
        pin.set_port_func(pin.pfunc());

        Self {
            _pin: pin.into(),
            _phantom_i: PhantomData,
        }
    }
}

#[allow(private_bounds)]
impl<'d, I: SealedInstance> RxPin<'d, I> {
    /// Takes ownership of a pin and configures it to be used as an `SCI` RX line.
    pub fn new(pin: Peri<'d, impl RxPinSealed<I>>) -> Self {
        debug!("RX: {}/{}", pin._port(), pin._pin());
        pin.set_port_func(pin.pfunc());

        Self {
            _pin: pin.into(),
            _phantom_i: PhantomData,
        }
    }
}

macro_rules! tx_pin_impl {
    ($sci:ident, $pin:ident, $pfunc:ident) => {
        impl TxPinSealed<crate::peripherals::$sci> for peripherals::$pin {
            const PERIPHERAL_FUNC: PortFunction = PortFunction::$pfunc;
        }
    };
}

macro_rules! rx_pin_impl {
    ($sci:ident, $pin:ident, $pfunc:ident) => {
        impl RxPinSealed<crate::peripherals::$sci> for peripherals::$pin {
            const PERIPHERAL_FUNC: PortFunction = PortFunction::$pfunc;
        }
    };
}

macro_rules! instance_impl {
    ($periph:ident, $rx_int:ident, $tx_int:ident, $te_int:ident, $stop:ident) => {
        impl Instance for peripherals::$periph {}

        paste! {
            impl SealedInstance for crate::peripherals::$periph {
                #[cfg(feature = "defmt")]
                const PERIPHERAL: &'static str = concat!(stringify!($periph), ": ");
                const RX_INTERRUPT_EVENT: InterruptEvent = InterruptEvent::$rx_int;
                const TX_INTERRUPT_EVENT: InterruptEvent = InterruptEvent::$tx_int;
                const TE_INTERRUPT_EVENT: InterruptEvent = InterruptEvent::$te_int;

                fn regs() -> ra4m1_ctpac::sci0::Sci0 {
                    crate::pac::$periph
                }

                fn start() {
                    debug!("{}: stop=false", stringify!($periph));

                    pac::MSTP.mstpcrb().write(|w| {
                        w.[< set_ $stop >](false);
                    });
                }

                fn stop() {
                    debug!("{}: stop=true", stringify!($periph));

                    pac::MSTP.mstpcrb().write(|w| {
                        w.[< set_ $stop >](true);
                    });
                }

                fn tx_buffer() -> &'static RingBuffer {
                    static TX_BUF: RingBuffer = RingBuffer::new();
                    &TX_BUF

                }

                fn rx_buffer() -> &'static RingBuffer {
                    static RX_BUF: RingBuffer = RingBuffer::new();
                    &RX_BUF
                }
            }
        }
    };
}

tx_pin_impl!(SCI0, P101, Sci1);
tx_pin_impl!(SCI0, P205, Sci1);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
tx_pin_impl!(SCI0, P411, Sci1);

tx_pin_impl!(SCI1, P213, Sci2);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
tx_pin_impl!(SCI1, P401, Sci2);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
tx_pin_impl!(SCI1, P501, Sci2);

rx_pin_impl!(SCI0, P100, Sci1);
#[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
rx_pin_impl!(SCI0, P104, Sci1);
#[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
rx_pin_impl!(SCI0, P206, Sci1);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
rx_pin_impl!(SCI0, P410, Sci1);

rx_pin_impl!(SCI1, P212, Sci2);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
rx_pin_impl!(SCI1, P402, Sci2);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
rx_pin_impl!(SCI1, P502, Sci2);
#[cfg(feature = "_100pin")]
rx_pin_impl!(SCI1, P708, Sci2);

instance_impl!(SCI0, Sci0Rxi, Sci0Txi, Sci0Tei, mstpb31);
instance_impl!(SCI1, Sci1Rxi, Sci1Txi, Sci1Tei, mstpb30);

/// Baud rate generator configuration for fixed speeds, rates that use "baud rate modulation" may achieve more precise timing.
/// Derived from the formula listed in Table 28.19.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
struct SpeedEntry {
    baud_rate: u32,
    small_n: u8,
    big_n: u8,
    modulation: u8,
}

/// These are valid for 48 MHz `PCLKA` only.
const SPEED_ENTRIES: [SpeedEntry; 9] = [
    SpeedEntry {
        baud_rate: 300,
        small_n: 3,
        big_n: 77,
        modulation: 0,
    },
    SpeedEntry {
        baud_rate: 1200,
        small_n: 2,
        big_n: 77,
        modulation: 0,
    },
    SpeedEntry {
        baud_rate: 2400,
        small_n: 2,
        big_n: 38,
        modulation: 0,
    },
    SpeedEntry {
        baud_rate: 4800,
        small_n: 1,
        big_n: 77,
        modulation: 0,
    },
    SpeedEntry {
        baud_rate: 9600,
        small_n: 1,
        big_n: 38,
        modulation: 0,
    },
    SpeedEntry {
        baud_rate: 9600,
        small_n: 0,
        big_n: 140,
        modulation: 231,
    },
    SpeedEntry {
        baud_rate: 19200,
        small_n: 0,
        big_n: 77,
        modulation: 0,
    },
    SpeedEntry {
        baud_rate: 38400,
        small_n: 0,
        big_n: 38,
        modulation: 0,
    },
    SpeedEntry {
        baud_rate: 115200,
        small_n: 0,
        big_n: 12,
        modulation: 0,
    },
];

impl<
    'd,
    I: Instance,
    RxInt: Interrupt + IcuEventer,
    TxInt: Interrupt + IcuEventer,
    TeInt: Interrupt + IcuEventer,
> Uart<'d, I, RxInt, TxInt, TeInt>
{
    #[inline]
    fn set_data_bits(n: u8) {
        let sci = I::regs();

        match n {
            7 => {
                // // 7 Data bits
                // // Restrictions apply, page 704 note 3
                sci.scmr().modify(|w| w.set_chr1(true));
                sci.smr().modify(|w| w.set_chr(true));
            }
            8 => {
                // 8 Data bits
                info!("{}Setting 8 data bits", I::PERIPHERAL);
                sci.scmr().modify(|w| w.set_chr1(true));
                sci.smr().modify(|w| w.set_chr(false));
            }
            9 => {
                // // 9 Data bits
                // sci.scmr().write(|w| w.set_chr1(false));
                // sci.smr().write(|w| w.set_chr(false));
                todo!()
            }
            _ => unimplemented!(),
        }
    }

    pub fn init_buffers(&self, tx_buffer: &'d mut [u8], rx_buffer: &'d mut [u8]) {
        let tx_len = tx_buffer.len();
        unsafe { I::tx_buffer().init(tx_buffer.as_mut_ptr(), tx_len) };

        let rx_len = rx_buffer.len();
        unsafe { I::rx_buffer().init(rx_buffer.as_mut_ptr(), rx_len) };
    }

    #[allow(private_bounds)]
    pub fn new(
        _peri: Peri<'d, I>,
        tx: Peri<'d, impl TxPinSealed<I>>,
        rx: Peri<'d, impl RxPinSealed<I>>,
        _irq: impl interrupt::typelevel::Binding<RxInt, RxInterruptHandler<I>>
        + interrupt::typelevel::Binding<TxInt, TxInterruptHandler<I>>
        + interrupt::typelevel::Binding<TeInt, TeInterruptHandler<I>>
        + 'd,
    ) -> Self {
        I::start();

        let sci = I::regs();

        sci.scr().write_value(Scr(0));

        sci.scr().modify(|w| {
            w.set_tie(false);
            w.set_rie(false);
            w.set_te(false);
            w.set_re(false);
            w.set_teie(false);
        });

        sci.fcr().modify(|w| {
            // Enable FIFO
            w.set_fm(true);
            // TODO: Is this the value we want?
            w.set_ttrg(Ttrg::from_bits(I::FIFO_DEPTH));
        });

        sci.scr().modify(|w| {
            w.set_cke(ScrCke::_00);
        });

        sci.simr1().modify(|w| {
            w.set_iicm(false);
        });

        sci.spmr().modify(|w| {
            w.set_ckph(false);
            w.set_ckpol(false);
        });

        // not-smart card mode
        sci.scmr().modify(|w| {
            w.set_smif(false);
        });

        Self::set_data_bits(8);

        // Set it up for No Parity, 1 stop bit
        sci.smr().modify(|w| {
            w.set_pe(false);
            w.set_pm(SmrPm::Even);
            w.set_stop(Stop::Stop1);
        });

        sci.smr().modify(|w| {
            w.set_mp(false);
            w.set_cm(false);
        });

        sci.semr().modify(|w| {
            w.set_brme(false);
            w.set_bgdm(false);
            w.set_abcs(false);
            w.set_abcse(false);
            w.set_rxdesel(false);
        });

        sci.sptr().write(|w| {
            w.set_spb2dt(false);
            w.set_spb2io(false);
        });

        let speed = &SPEED_ENTRIES[0];
        Self::set_speed_from_entry(speed);

        // Move pins over to SCI
        let tx = TxPin::new(tx);
        let rx = RxPin::new(rx);
        let _ = tx;
        let _ = rx;

        // Enable in NVIC. We can largely ignore the NVIC after this as
        // all of the peripheral interrupts are going to be managed by
        // the ICU and/or ELC.
        unsafe { <RxInt as Interrupt>::IRQ.enable() };
        unsafe { <TxInt as Interrupt>::IRQ.enable() };
        unsafe { <TeInt as Interrupt>::IRQ.enable() };
        // Enable in ICU
        RxInt::iel_enable(I::RX_INTERRUPT_EVENT);
        TxInt::iel_enable(I::TX_INTERRUPT_EVENT);
        TeInt::iel_enable(I::TE_INTERRUPT_EVENT);

        // We can leave the receiver on, but not the transmitter as enabling
        // the transmitter in combination with the TX interrupt is what kicks
        // off the whole transmit procedure.
        sci.scr().modify(|w| {
            w.set_re(true);
            w.set_rie(true);
        });

        trace!("SMR: {}", sci.smr().read());
        trace!("SCR: {}", sci.scr().read());
        trace!("SSR: {}", sci.ssr().read());
        trace!("SEMR: {}", sci.semr().read());
        trace!("BRR: {}", sci.brr().read());
        trace!("FCR: {}", sci.fcr().read());

        Self {
            _phantom: PhantomData,
            _phantom_rx: PhantomData,
            _phantom_tx: PhantomData,
            _phantom_te: PhantomData,
        }
    }

    fn show_speed() {
        let sci = I::regs();
        let brr = sci.brr().read().brr();
        let mddr = sci.mddr().read().mddr();
        let brme = sci.semr().read().brme();
        debug!(
            "{}Speed: BRR: {}, MDDR: {}, BRME: {}",
            I::PERIPHERAL,
            brr,
            mddr,
            brme
        );
    }

    fn set_speed_from_entry(speed: &SpeedEntry) {
        let sci = I::regs();

        sci.scr().modify(|w| {
            w.set_re(false);
            w.set_te(false);
        });
        debug!("{}Applying {}", I::PERIPHERAL, speed);
        sci.brr().write(|w| {
            w.set_brr(speed.big_n);
        });
        if speed.modulation != 0 {
            sci.mddr().write(|w| {
                w.set_mddr(speed.modulation);
            });
            sci.semr().modify(|w| {
                w.set_brme(true);
            });
        } else {
            sci.mddr().write(|w| {
                w.set_mddr(0);
            });
            sci.semr().modify(|w| w.set_brme(false));
        }
        sci.smr().modify(|w| {
            w.set_cks(SmrCks::from_bits(speed.small_n));
        });
        sci.scr().modify(|w| {
            w.set_re(true);
        });
        Self::show_speed();
    }

    /// Configures the `SCI` instance for a given baud rate.  Currently only works with `PCLKA` set to 48 MHz.
    /// # Arguments
    /// * `baud_rate` - Desired baud rate.
    /// Currently only 300, 1200, 2400, 4800, 9600, 19200, 3840, and 115200 baud are supported.
    ///
    /// # TODO
    /// * Support arbitrary baud rates
    /// * Support arbitrary `PCLKA` rates
    ///
    pub fn set_speed(&mut self, baud_rate: u32) {
        let speed = SPEED_ENTRIES
            .iter()
            .find(|e| e.baud_rate == baud_rate)
            .unwrap();
        Self::set_speed_from_entry(speed);
    }

    pub fn read_line(&self, data: &mut [u8]) -> usize {
        let mut crlf = false;
        let mut count = 0;
        for byte in data.iter_mut() {
            let mut data = [0_u8; 1];
            self.blocking_read(&mut data);
            match data[0] {
                0x0d => {
                    crlf = true;
                }
                0x0a => {
                    if crlf {
                        break;
                    }
                }
                valid => {
                    *byte = valid;
                    count += 1;
                }
            }
        }
        count
    }

    #[inline(always)]
    pub fn blocking_read(&self, data: &mut [u8]) {
        let mut reader = unsafe { I::rx_buffer().reader() };

        for byte in data.iter_mut() {
            loop {
                match reader.pop_one() {
                    Some(rx) => {
                        *byte = rx;
                        break;
                    }
                    None => {
                        asm::nop();
                    }
                }
            }
        }
    }

    pub fn blocking_write(&mut self, data: &[u8]) {
        let sci = I::regs();

        let mut written: usize = 0;
        let mut tx_writer = unsafe { I::tx_buffer().writer() };

        while written != data.len() {
            let out_slice = tx_writer.push_slice();
            if !out_slice.is_empty() {
                let n = out_slice.len().min(data.len() - written);
                out_slice[..n].copy_from_slice(&data[written..written + n]);
                written += n;
                tx_writer.push_done(n);
            }
        }

        if !sci.scr().read().te() {
            sci.scr().modify(|w| {
                w.set_te(true);
                w.set_tie(true);
            });
        }

        while sci.scr().read().te() {
            asm::nop()
        }
    }
}

// impl<'d, I: Instance> Drop for Uart<'d, I> {
//     fn drop(&mut self) {
//         I::stop();
//     }
// }

impl<I: Instance, Int: Interrupt + IcuEventer> InterruptHandler<Int> for RxInterruptHandler<I> {
    unsafe fn on_interrupt() {
        trace!("RxI");

        let sci = I::regs();
        let mut writer = unsafe { I::rx_buffer().writer() };
        let buf = writer.push_slice();

        match buf.is_empty() {
            false => {
                let read_len = buf.len().min(sci.fdr().read().r() as _);
                for i in 0..read_len {
                    buf[i] = sci.frdrl().read().rdatl();
                }
                sci.ssr_fifo().modify(|w| {
                    w.set_rdf(false);
                    // If there isn't enough space in the static buffer are we dropping it on the floor when we reset dr?
                    w.set_dr(false);
                });
                writer.push_done(read_len);
            }
            true => {
                error!("RX Buffer is full");
            }
        }

        Int::iel_unpend();
    }
}

impl<I: Instance, Int: Interrupt + IcuEventer> InterruptHandler<Int> for TxInterruptHandler<I> {
    unsafe fn on_interrupt() {
        trace!("TxI");
        Int::iel_unpend();

        let sci = I::regs();
        let mut tx_reader = unsafe { I::tx_buffer().reader() };

        let out_buf = tx_reader.pop_slice();

        if out_buf.is_empty() {
            sci.scr().modify(|w| {
                w.set_tie(false);
                w.set_teie(true);
            });
            return;
        }

        let out_len = out_buf.len();
        let fifo_available = usize::from(I::FIFO_DEPTH - sci.fdr().read().t());

        if out_len > fifo_available {
            for byte in out_buf[0..fifo_available].iter() {
                sci.ftdrl().write_value(Ftdrl(*byte));
            }

            tx_reader.pop_done(fifo_available);
        } else {
            for byte in out_buf[0..out_len - 1].iter() {
                sci.ftdrl().write_value(Ftdrl(*byte));
                // Should we clear TDFE per Fig 28.14?
            }

            sci.ftdrl().write_value(Ftdrl(out_buf[out_len - 1]));

            sci.scr().modify(|w| {
                w.set_tie(false);
                w.set_teie(true);
            });

            tx_reader.pop_done(out_len);
        }
    }
}
impl<I: Instance, Int: Interrupt + IcuEventer> InterruptHandler<Int> for TeInterruptHandler<I> {
    unsafe fn on_interrupt() {
        trace!("TeI");
        Int::iel_unpend();
        let sci = I::regs();
        sci.scr().modify(|w| {
            w.set_te(false);
            w.set_tie(false);
            w.set_teie(false);
        });
    }
}
