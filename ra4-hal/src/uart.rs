//! `UART` Universal Asynchronous Receiver-Transmitter implemented using the `SCI` peripheral.

use core::{future::poll_fn, marker::PhantomData, task::Poll};

use cortex_m::asm;
use embassy_hal_internal::{
    Peri, PeripheralType, atomic_ring_buffer::RingBuffer, interrupt::InterruptExt as _,
};
use embassy_sync::waitqueue::AtomicWaker;
use paste::paste;
use ra4m1_ctpac::sci::{
    regs::{Ftdrl, Scr},
    vals::{ScrCke, SmrCks, SmrPm, Stop, Ttrg},
};

use crate::interrupt::typelevel::{Handler as InterruptHandler, Interrupt as InterruptType};
use crate::{
    event_link::{IcuInterrupt, InterruptEvent},
    gpio::{Pin, PortFunction},
    interrupt,
    interrupt::Interrupt,
    pac, peripherals,
};

/// Baud rate generator configuration for fixed speeds, rates that use "baud rate modulation" may achieve more precise timing.
/// Derived from the formula listed in Table 28.19.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
struct SpeedEntry {
    baud_rate: u32,
    small_n: u8,
    big_n: u8,
    modulation: u8,
}

/// UART driver, backed by a [`RingBuffer`] and 16-byte on-device FIFO buffer.
#[allow(private_bounds)]
pub struct BufferedUart<'d, I: Instance> {
    _phantom: PhantomData<&'d I>,
    rx_int: Interrupt,
    tx_int: Interrupt,
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

/// UART error
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum UartError {
    /// Framing error
    Framing,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
}

/// Buffered UART instance.
///
/// Note: On the `RA4M1` only `SCI0` and `SCI1` have FIFOs.
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

    fn regs() -> pac::sci::Sci;

    /// Turns the `SCI` module on.
    /// In Renesas speak it turns off "module stop" for the `SCI` instance.
    /// See §10 of the reference manual.
    fn start();

    /// Turns the `SCI` module off.
    /// In Renesas speak it turns on "module stop" for the `SCI` instance.
    /// See §10 of the reference manual.
    fn stop();

    /// # Returns
    ///
    /// Static reference to the statically allocated [`RingBuffer`] for transmit operations.
    fn tx_buffer() -> &'static RingBuffer;

    /// # Returns
    ///
    /// Static reference to the statically allocated [`RingBuffer`] for receive operations.
    fn rx_buffer() -> &'static RingBuffer;

    /// Waker for "transmit end" events.
    fn te_waker() -> &'static AtomicWaker;

    /// Waker for transmit buffer empty events.
    fn tx_waker() -> &'static AtomicWaker;

    /// Waker for receive events.
    fn rx_waker() -> &'static AtomicWaker;
}

/// A pin that can be used for transmission.
#[allow(private_bounds)]
pub trait TxPin<I: Instance>: TxPinSealed<I> {}
// impl<I: Instance, T: TxPinSealed<I>> UartTxPin<I> for T {}

/// A pin that can be used for reception.
#[allow(private_bounds)]
pub trait RxPin<I: Instance>: RxPinSealed<I> {}

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

impl<'d, I: Instance> BufferedUart<'d, I> {
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

    fn configure_pins(tx: Peri<'d, impl TxPin<I>>, rx: Peri<'d, impl RxPin<I>>) {
        debug!("TX: {}/{}", tx._port(), tx._pin());
        tx.set_port_func(tx.pfunc());

        debug!("RX: {}/{}", rx._port(), rx._pin());
        rx.set_port_func(rx.pfunc());
    }

    /// Configures a new UART and returns the driver.
    #[allow(private_bounds)]
    pub fn new<RxInt: InterruptType, TxInt: InterruptType, TeInt: InterruptType>(
        _peri: Peri<'d, I>,
        tx_pin: Peri<'d, impl TxPin<I>>,
        tx_buffer: &'d mut [u8],
        rx_pin: Peri<'d, impl RxPin<I>>,
        rx_buffer: &'d mut [u8],
        _irqs: impl interrupt::typelevel::Binding<RxInt, RxInterruptHandler<I>>
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
            // TODO: Give enum variants meaningful names.
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
        Self::configure_pins(tx_pin, rx_pin);

        let tx_len = tx_buffer.len();
        unsafe { I::tx_buffer().init(tx_buffer.as_mut_ptr(), tx_len) };

        let rx_len = rx_buffer.len();
        unsafe { I::rx_buffer().init(rx_buffer.as_mut_ptr(), rx_len) };

        // Enable interrupts in NVIC. We can largely ignore the NVIC after this as all of the
        // peripheral interrupts are going to be managed by the ICU and/or ELC.
        unsafe { RxInt::IRQ.enable() };
        unsafe { TxInt::IRQ.enable() };
        unsafe { TeInt::IRQ.enable() };

        // Enable in ICU
        RxInt::IRQ.icu_enable(I::RX_INTERRUPT_EVENT);
        TxInt::IRQ.icu_enable(I::TX_INTERRUPT_EVENT);
        TeInt::IRQ.icu_enable(I::TE_INTERRUPT_EVENT);

        // We can leave the receiver on, but not the transmitter as enabling the transmitter in
        // combination with the TX interrupt is what kicks off the whole transmit procedure.
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
            rx_int: RxInt::IRQ,
            tx_int: TxInt::IRQ,
        }
    }

    fn show_speed() {
        let sci = I::regs();

        // Baud rate divisor
        let brr = sci.brr().read().brr();
        // Modulation duty-cycle
        let mddr = sci.mddr().read().mddr();
        // Modulation en/disabled
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
    ///
    /// # Arguments
    /// * `baud_rate` - Desired baud rate.
    /// Currently only 300, 1200, 2400, 4800, 9600, 19200, 3840, and 115200 baud are supported.
    ///
    /// # TODO
    /// * Support arbitrary baud rates
    /// * Support arbitrary `PCLKA` rates
    pub fn set_speed(&mut self, baud_rate: u32) {
        let speed = SPEED_ENTRIES
            .iter()
            .find(|e| e.baud_rate == baud_rate)
            .unwrap();

        Self::set_speed_from_entry(speed);
    }

    /// Reads data until the buffer is full or `b"\r\n"` is read.
    ///
    /// # Arguments
    /// * `data` - Mutable slice to hold incoming data.
    ///
    /// # Returns
    ///
    /// Number of bytes read excluding the trailing newline which is not copied to `data`.
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

    fn read_ready(&mut self) -> Result<bool, UartError> {
        Ok(!I::rx_buffer().is_empty())
    }

    /// # Returns
    ///
    /// The amount of data in the RX [`RingBuffer`].
    pub fn depth(&self) -> usize {
        I::rx_buffer().available()
    }

    /// Clears out the contents of the RX [`RingBuffer`].
    pub fn drain(&mut self) {
        let mut reader = unsafe { I::rx_buffer().reader() };
        loop {
            let (_, len) = reader.pop_buf();
            reader.pop_done(len);
            if len == 0 {
                break;
            }
        }
    }

    /// Reads data from the UART.
    ///
    /// # Returns
    ///
    /// Returns when there is data in the RX [`RingBuffer`].
    /// `buf` is not guaranteed to be full and the length of its contents is returned.
    #[inline(always)]
    pub async fn read(&self, buf: &mut [u8]) -> Result<usize, UartError> {
        poll_fn(|cx| {
            let mut buf_pos = 0;
            let mut reader = unsafe { I::rx_buffer().reader() };
            let mut data = reader.pop_slice();

            while !data.is_empty() && buf_pos < buf.len() {
                let data_len = data.len().min(buf.len() - buf_pos);
                buf[buf_pos..buf_pos + data_len].copy_from_slice(&data[..data_len]);
                buf_pos += data_len;

                let pending = I::rx_buffer().is_full();
                reader.pop_done(data_len);

                if pending {
                    self.rx_int.icu_pend();
                }

                data = reader.pop_slice();
            }

            if buf_pos != 0 {
                Poll::Ready(Ok(buf_pos))
            } else {
                I::rx_waker().register(cx.waker());
                Poll::Pending
            }
        })
        .await
    }

    async fn write(&mut self, buf: &[u8]) -> Result<usize, UartError> {
        let sci = I::regs();

        let mut written: usize = 0;
        let mut writer = unsafe { I::tx_buffer().writer() };

        poll_fn(|cx| {
            if written < buf.len() {
                I::tx_waker().register(cx.waker());

                if I::tx_buffer().is_full() {
                    trace!("{}TX buffer full in async write", I::PERIPHERAL);
                    return Poll::Pending;
                }

                let out = writer.push_slice();
                let chunk_len = out.len().min(buf.len().saturating_sub(written));
                out[..chunk_len].copy_from_slice(&buf[written..(written + chunk_len)]);
                written += chunk_len;

                writer.push_done(chunk_len);

                if !sci.scr().read().te() {
                    sci.scr().modify(|w| {
                        w.set_te(true);
                        w.set_tie(true);
                    });
                }

                if written < buf.len() {
                    return Poll::Pending;
                }

                // If there's nothing else wait on the Transmit End interrupt
                I::te_waker().register(cx.waker());

                if !sci.ssr_fifo().read().tend() {
                    return Poll::Pending;
                }
            } else {
                I::te_waker().register(cx.waker());

                // If we're still waiting for the FIFO to write everything to the wire
                if !sci.ssr_fifo().read().tend() {
                    return Poll::Pending;
                }
            }

            return Poll::Ready(Ok(written));
        })
        .await
    }

    /// Reads data from the UART, blocks until `data` is full.
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

    /// Writes `data` to the UART, blocking until the last byte has been sent out on the wire.
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
            } else {
                self.tx_int.icu_pend();
            }

            if !sci.scr().read().te() {
                sci.scr().modify(|w| {
                    w.set_te(true);
                    w.set_tie(true);
                });
            }
        }

        while !sci.ssr_fifo().read().tend() {
            asm::nop();
        }
    }
}

impl<'d, I: Instance> Drop for BufferedUart<'d, I> {
    fn drop(&mut self) {
        I::stop();
    }
}

impl<I: Instance, Int: InterruptType> InterruptHandler<Int> for RxInterruptHandler<I> {
    unsafe fn on_interrupt() {
        trace!("RxI");

        let sci = I::regs();
        let mut writer = unsafe { I::rx_buffer().writer() };
        let buf = writer.push_slice();

        match buf.is_empty() {
            false => {
                let fifo_len = sci.fdr().read().r() as _;
                let read_len = buf.len().min(fifo_len);

                for i in 0..read_len {
                    buf[i] = sci.frdrl().read().rdatl();
                }
                writer.push_done(read_len);

                sci.ssr_fifo().modify(|w| {
                    w.set_rdf(false);
                    // If there isn't enough space in the static buffer are we dropping it on the floor when we reset dr?
                    w.set_dr(false);
                });

                I::rx_waker().wake();

                if read_len != fifo_len {
                    trace!("{}RX Buffer full, FIFO drain={}", I::PERIPHERAL, read_len);
                } else {
                    Int::IRQ.icu_unpend();
                }
            }
            true => {
                let fifo_free = I::FIFO_DEPTH - sci.fdr().read().r() as u8;

                warn!("{}RX Buffer full, FIFO cap={}", I::PERIPHERAL, fifo_free);

                Int::IRQ.icu_unpend();

                I::rx_waker().wake();

                if sci.ssr_fifo().read().orer() {
                    error!("{}Overrun, dropping 1", I::PERIPHERAL);
                    sci.ssr_fifo().modify(|w| w.set_orer(false));
                    Int::IRQ.icu_unpend();
                }
            }
        }
    }
}

impl<I: Instance, Int: InterruptType> InterruptHandler<Int> for TxInterruptHandler<I> {
    unsafe fn on_interrupt() {
        trace!("TxI");
        Int::IRQ.icu_unpend();

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

        I::tx_waker().wake();
    }
}
impl<I: Instance, Int: InterruptType> InterruptHandler<Int> for TeInterruptHandler<I> {
    unsafe fn on_interrupt() {
        trace!("TeI");
        Int::IRQ.icu_unpend();

        let sci = I::regs();

        if I::tx_buffer().is_empty() {
            while !sci.ssr_fifo().read().tend() {
                asm::nop();
            }

            sci.scr().modify(|w| {
                w.set_te(false);
                w.set_tie(false);
                w.set_teie(false);
            });

            I::te_waker().wake();
        } else {
            sci.scr().modify(|w| {
                w.set_te(true);
                w.set_tie(true);
                w.set_teie(false);
            });
        }
    }
}

impl<'d, I: Instance> embedded_io_async::ErrorType for BufferedUart<'d, I> {
    type Error = UartError;
}

impl<'d, I: Instance> embedded_io_async::Read for BufferedUart<'d, I> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        Self::read(self, buf).await
    }
}

impl<'d, I: Instance> embedded_io_async::Write for BufferedUart<'d, I> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Self::write(self, buf).await
    }
}

impl embedded_io::Error for UartError {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl core::fmt::Display for UartError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let message = match self {
            Self::Framing => "Framing Error",
            Self::Overrun => "RX Buffer Overrun",
            Self::Parity => "Parity Check Error",
        };

        write!(f, "{}", message)
    }
}

impl core::error::Error for UartError {}

impl<'d, I: Instance> embedded_io_async::ReadReady for BufferedUart<'d, I> {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Self::read_ready(self)
    }
}

impl<'d, I: Instance> embedded_serial::MutBlockingTx for BufferedUart<'d, I> {
    type Error = ();

    // TODO: Change… "optimize" this so we only wait for data to leave the ring buffer.
    fn putc(&mut self, ch: u8) -> Result<(), Self::Error> {
        Self::blocking_write(self, &[ch]);
        Ok(())
    }
}

impl<'d, I: Instance> embedded_serial::MutBlockingRx for BufferedUart<'d, I> {
    type Error = ();

    fn getc(&mut self) -> Result<u8, Self::Error> {
        let mut ch = [0_u8];

        Self::blocking_read(self, &mut ch);

        Ok(ch[0])
    }
}

macro_rules! tx_pin_impl {
    ($sci:ident, $pin:ident, $pfunc:ident) => {
        impl TxPin<crate::peripherals::$sci> for peripherals::$pin {}
        impl TxPinSealed<crate::peripherals::$sci> for peripherals::$pin {
            const PERIPHERAL_FUNC: PortFunction = PortFunction::$pfunc;
        }
    };
}

macro_rules! rx_pin_impl {
    ($sci:ident, $pin:ident, $pfunc:ident) => {
        impl RxPin<crate::peripherals::$sci> for peripherals::$pin {}
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

                fn regs() -> ra4m1_ctpac::sci::Sci {
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

                fn te_waker() -> &'static AtomicWaker{
                    static TE_WAKER: AtomicWaker = AtomicWaker::new();
                    &TE_WAKER
                }

                fn tx_waker() -> &'static AtomicWaker{
                    static TX_WAKER: AtomicWaker = AtomicWaker::new();
                    &TX_WAKER
                }

                fn rx_waker() -> &'static AtomicWaker{
                    static RX_WAKER: AtomicWaker = AtomicWaker::new();
                    &RX_WAKER

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
