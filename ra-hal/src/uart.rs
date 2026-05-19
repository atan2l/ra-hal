//! Universal Asynchronous Receiver-Transmitter (`SCI`).
//!
//! # TODO
//! * Get the timing calculation working, and don't base the peripheral clock speed on ƒHOCO.
//! * Get clock stuff sorted for arbitrary baud rates and PCLKA frequencies
//! * Get non-FIFO instances working
//! # Notes
//! * The implementations for `SCI2` and `SCI9` are currently non-operational pending support for non-FIFO SCI instances.
//! * The driver will stop the `SCI` instance (`MSTPBnn=1`) when it is dropped.

use core::{future::poll_fn, marker::PhantomData, task::Poll};

use cortex_m::asm;
use embassy_hal_internal::{
    Peri, PeripheralType, atomic_ring_buffer::RingBuffer, interrupt::InterruptExt as _,
};
use embassy_sync::waitqueue::AtomicWaker;
use fugit::HertzU32;
use micromath::F32Ext as _;

use crate::{
    event_link::{IcuInterrupt, InterruptEvent},
    gpio::{Pin, PortFunction},
    interrupt,
    interrupt::{
        Interrupt,
        typelevel::{Handler as InterruptHandler, Interrupt as InterruptType},
    },
    module_stop::ModuleStop,
    pac::{
        self,
        sci_fifo::{
            regs::Scr,
            vals::{ScrCke, SmrCks, SmrPm, Stop},
        },
    },
};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
struct BaudGenConfig {
    small_n: u32,
    big_n: u32,
    err: f32,
}

/// UART configuration
#[non_exhaustive]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config {
    /// Baud rate
    pub baud_rate: u32,

    /// Number of data bits.
    /// Note: 9 bit data is not yet supported.
    #[allow(missing_docs)]
    pub data_bits: DataBits,

    /// Parity type
    #[allow(missing_docs)]
    pub parity: Parity,

    /// Number of stop bits.
    #[allow(missing_docs)]
    pub stop_bits: StopBits,
}

/// Number of stop bits
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StopBits {
    /// 1 stop bit
    Stop1,

    /// 2 stop bits
    Stop2,
}

/// Word length.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DataBits {
    /// 7 bits.
    DataBits7,

    /// 8 bits.
    DataBits8,

    /// 9 bits.
    DataBits9,
}

/// Parity bit
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Parity {
    /// Even parity
    Even,

    /// Odd parity
    Odd,

    /// No parity bit
    None,
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

/// Interrupt handler that handles outgoing data (transmission end) for an `SCI` instance.
pub struct TeInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

/// Interrupt handler that handles outgoing data (tx buffer empty) for an `SCI` instance.
pub struct TxInterruptHandler<I: Instance> {
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
pub trait Instance: SealedInstance + ModuleStop + PeripheralType + 'static + Send {}

pub(crate) trait SealedInstance {
    #[cfg(feature = "defmt")]
    const PERIPHERAL: &'static str;
    #[cfg(not(feature = "defmt"))]
    const PERIPHERAL: () = ();

    const RX_INTERRUPT_EVENT: InterruptEvent;
    const TE_INTERRUPT_EVENT: InterruptEvent;
    const TX_INTERRUPT_EVENT: InterruptEvent;

    /// `SCI0` and `SCI1` have 16-byte FIFO buffers for RX and TX ops per Table 28.1.
    const FIFO_DEPTH: u8 = 16;

    fn regs() -> pac::sci_fifo::SciFifo;

    /// # Returns
    ///
    /// Static reference to the statically allocated [`RingBuffer`] for receive operations.
    fn rx_buffer() -> &'static RingBuffer;

    /// # Returns
    ///
    /// Static reference to the statically allocated [`RingBuffer`] for transmit operations.
    fn tx_buffer() -> &'static RingBuffer;

    /// Waker for receive events.
    fn rx_waker() -> &'static AtomicWaker;

    /// Waker for "transmit end" events.
    fn te_waker() -> &'static AtomicWaker;

    /// Waker for transmit buffer empty events.
    fn tx_waker() -> &'static AtomicWaker;
}

/// A pin that can be used for reception.
#[allow(private_bounds)]
pub trait RxPin<I: Instance>: SealedRxPin<I> {}

/// A pin that can be used for transmission.
#[allow(private_bounds)]
pub trait TxPin<I: Instance>: SealedTxPin<I> {}
// impl<I: Instance, T: SealedTxPin<I>> UartTxPin<I> for T {}

pub(crate) trait SealedRxPin<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_pfunc(&self) {
        self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

pub(crate) trait SealedTxPin<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_pfunc(&self) {
        self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            baud_rate: 9600,
            data_bits: DataBits::DataBits8,
            parity: Parity::None,
            stop_bits: StopBits::Stop1,
        }
    }
}

impl<'d, I: Instance> BufferedUart<'d, I> {
    fn calc_baud_gen(baud: u32, clock: HertzU32) -> Option<BaudGenConfig> {
        let semr = 64.0;
        let mut big_n = 0_u32;
        let mut small_n = 3_u32;

        let mut best: Option<BaudGenConfig> = None;

        loop {
            let mut last_config: Option<BaudGenConfig> = None;

            for _ in 0..256 {
                let new_error = (clock.to_Hz() as f32
                    / (baud as f32
                        * semr
                        * (2_f32.powi(2 * small_n as i32 - 1))
                        * (big_n as f32 + 1.0)))
                    - 1.0;

                if let Some(last_config) = last_config {
                    if new_error.abs() > last_config.err.abs() {
                        break;
                    }
                }

                last_config = Some(BaudGenConfig {
                    small_n,
                    big_n,
                    err: new_error,
                });
                big_n += 1;
            }

            if let Some(ref mut best) = best {
                if last_config.unwrap().err.abs() < best.err.abs() {
                    *best = last_config.unwrap();
                }
            } else {
                best = last_config;
            }

            if small_n == 0 {
                break;
            } else {
                small_n -= 1;
                big_n = 0;
            }
        }

        best
    }

    /// Sets the number of bits in a byte.
    ///
    /// Note:
    /// * 9 bit data is supported by the underlying hardware but not yet by this driver.
    /// * This will disable the transmitter and temporarily disable the receiver (RA4M1 §28.2.9 note 4).
    #[inline]
    pub fn set_data_bits(&mut self, n: DataBits) {
        let sci = I::regs();

        sci.scr().modify(|r| {
            r.set_re(false);
            r.set_te(false);
        });

        self.set_data_bits_inner(n);

        sci.scr().modify(|r| r.set_re(true));
    }

    fn set_data_bits_inner(&mut self, n: DataBits) {
        let sci = I::regs();

        match n {
            DataBits::DataBits7 => {
                // Restrictions apply, page 704 note 3
                sci.scmr().modify(|r| r.set_chr1(true));
                sci.smr().modify(|r| r.set_chr(true));
            }
            DataBits::DataBits8 => {
                sci.scmr().modify(|r| r.set_chr1(true));
                sci.smr().modify(|r| r.set_chr(false));
            }
            DataBits::DataBits9 => {
                // // 9 Data bits
                // sci.scmr().write(|r| r.set_chr1(false));
                // sci.smr().write(|r| r.set_chr(false));
                todo!()
            }
        }
    }

    /// Sets baud rate for the `SCI` instance.
    ///
    /// Note: This will disable the transmitter and temporarily disable the receiver (§28.2.9 note 4).
    pub fn set_parity(&mut self, parity: Parity) {
        let sci = I::regs();

        sci.scr().modify(|r| {
            r.set_re(false);
            r.set_te(false);
        });

        self.set_parity_inner(parity);

        sci.scr().modify(|r| r.set_re(true));
    }

    fn set_parity_inner(&mut self, parity: Parity) {
        let sci = I::regs();

        let (pe, pm) = match parity {
            Parity::Even => (true, SmrPm::Even),
            Parity::Odd => (true, SmrPm::Odd),
            Parity::None => (false, SmrPm::Even),
        };

        sci.smr().modify(|r| {
            r.set_pe(pe);
            r.set_pm(pm);
        });
    }

    /// Set stop bit length.
    ///
    /// Note: This will disable the transmitter and temporarily disable the receiver (§28.2.9 note 4).
    pub fn set_stop_bits(&mut self, stop_bits: StopBits) {
        let sci = I::regs();

        sci.scr().modify(|r| {
            r.set_re(false);
            r.set_te(false);
        });

        self.set_stop_bits_inner(stop_bits);

        sci.scr().modify(|r| r.set_re(true));
    }

    fn set_stop_bits_inner(&mut self, stop_bits: StopBits) {
        let sci = I::regs();

        // Set it up for No Parity, 1 stop bit
        sci.smr().modify(|r| {
            match stop_bits {
                StopBits::Stop1 => r.set_stop(Stop::Stop1),
                StopBits::Stop2 => r.set_stop(Stop::Stop2),
            };
        });
    }

    /// Sets baud rate for the `SCI` instance.
    ///
    /// Note:
    /// * This will disable the transmitter and temporarily disable the receiver (§28.2.9 note 4).
    /// * Currently depends on micromath. Pending <https://github.com/rust-lang/rust/issues/137578>.
    ///
    /// # Arguments
    /// * `baud_rate` - Desired baud rate.
    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        let clock_config = crate::clock::clock_status();

        #[cfg(ra2)]
        let brr_config = Self::calc_baud_gen(baud_rate, clock_config.peripheral_b).unwrap();
        #[cfg(not(ra2))]
        let brr_config = Self::calc_baud_gen(baud_rate, clock_config.peripheral_a).unwrap();

        let sci = I::regs();

        sci.scr().modify(|r| {
            r.set_re(false);
            r.set_te(false);
        });

        debug!(
            "{}Baud rate: {}, Config: {}",
            I::PERIPHERAL,
            baud_rate,
            brr_config
        );

        Self::set_baud_from_config(&brr_config);

        sci.scr().modify(|r| r.set_re(true));
    }

    fn set_baud_from_config(brr_config: &BaudGenConfig) {
        let sci = I::regs();

        sci.brr()
            .write_value(u8::try_from(brr_config.big_n).expect("SCI big_n > 255"));

        sci.mddr().write_value(0);
        sci.semr().modify(|r| r.set_brme(false));

        sci.smr().modify(|r| {
            r.set_cks(SmrCks::from_bits(
                u8::try_from(brr_config.small_n).expect("SCI small_n > 255"),
            ))
        });

        sci.scr().modify(|r| r.set_re(true));
    }

    /// Configures a new UART and returns the driver.
    #[allow(private_bounds)]
    pub fn new<RxInt: InterruptType, TeInt: InterruptType, TxInt: InterruptType>(
        _peri: Peri<'d, I>,
        tx_pin: Peri<'d, impl TxPin<I>>,
        tx_buffer: &'d mut [u8],
        rx_pin: Peri<'d, impl RxPin<I>>,
        rx_buffer: &'d mut [u8],
        irqs: impl interrupt::typelevel::Binding<RxInt, RxInterruptHandler<I>>
        + interrupt::typelevel::Binding<TeInt, TeInterruptHandler<I>>
        + interrupt::typelevel::Binding<TxInt, TxInterruptHandler<I>>
        + 'd,
        config: Config,
    ) -> Self {
        info!("{}{}", I::PERIPHERAL, config);

        let clock_config = crate::clock::clock_status();
        let mut this = Self::new_inner(tx_pin, tx_buffer, rx_pin, rx_buffer, irqs);

        this.set_data_bits_inner(config.data_bits);
        this.set_parity_inner(config.parity);
        this.set_stop_bits_inner(config.stop_bits);

        #[cfg(ra2)]
        let brr_config = Self::calc_baud_gen(config.baud_rate, clock_config.peripheral_b).unwrap();
        #[cfg(not(ra2))]
        let brr_config = Self::calc_baud_gen(config.baud_rate, clock_config.peripheral_a).unwrap();

        info!("{}{}", I::PERIPHERAL, brr_config);
        Self::set_baud_from_config(&brr_config);

        let sci = I::regs();

        // We can leave the receiver on, but not the transmitter as enabling the transmitter in
        // combination with the TX interrupt is what kicks off the whole transmit procedure.
        sci.scr().modify(|r| {
            r.set_re(true);
            r.set_rie(true);
        });

        this
    }

    fn new_inner<RxInt: InterruptType, TxInt: InterruptType, TeInt: InterruptType>(
        tx_pin: Peri<'d, impl TxPin<I>>,
        tx_buffer: &'d mut [u8],
        rx_pin: Peri<'d, impl RxPin<I>>,
        rx_buffer: &'d mut [u8],
        irqs: impl interrupt::typelevel::Binding<RxInt, RxInterruptHandler<I>>
        + interrupt::typelevel::Binding<TxInt, TxInterruptHandler<I>>
        + interrupt::typelevel::Binding<TeInt, TeInterruptHandler<I>>
        + 'd,
    ) -> Self {
        let _ = irqs;

        I::start_module();

        let sci = I::regs();

        sci.scr().write_value(Scr(0));

        sci.scr().modify(|r| {
            r.set_re(false);
            r.set_rie(false);
            r.set_te(false);
            r.set_teie(false);
            r.set_tie(false);
        });

        sci.fcr().modify(|r| {
            // Enable FIFO
            r.set_fm(true);
            // TODO: Is this the value we want?
            r.set_ttrg(I::FIFO_DEPTH);
        });

        // TODO: Give enum variants meaningful names.
        sci.scr().modify(|r| r.set_cke(ScrCke::_00));

        sci.simr1().modify(|r| r.set_iicm(false));

        sci.spmr().modify(|r| {
            r.set_ckph(false);
            r.set_ckpol(false);
        });

        // not-smart card mode
        sci.scmr().modify(|r| r.set_smif(false));

        sci.smr().modify(|r| {
            r.set_cm(false);
            r.set_mp(false);
        });

        sci.semr().modify(|r| {
            r.set_abcs(false);
            r.set_abcse(false);
            r.set_bgdm(false);
            r.set_brme(false);
            r.set_rxdesel(false);
        });

        sci.sptr().write(|r| {
            r.set_spb2dt(false);
            r.set_spb2io(false);
        });

        // Move pins over to SCI
        trace!("P{}RX=p{}/{}", I::PERIPHERAL, rx_pin._port(), rx_pin._pin());
        rx_pin.set_pfunc();

        trace!("{}TX=p{}/{}", I::PERIPHERAL, tx_pin._port(), tx_pin._pin());
        tx_pin.set_pfunc();

        let rx_len = rx_buffer.len();
        unsafe { I::rx_buffer().init(rx_buffer.as_mut_ptr(), rx_len) };

        let tx_len = tx_buffer.len();
        unsafe { I::tx_buffer().init(tx_buffer.as_mut_ptr(), tx_len) };

        // Safety: Interrupt handlers are defined by the irqs argument and thus the interrupts are safe to enable.
        unsafe {
            // Enable interrupts in NVIC. We can largely ignore the NVIC after this as all of the
            // peripheral interrupts are going to be managed by the ICU and/or ELC.
            RxInt::IRQ.enable();
            TeInt::IRQ.enable();
            TxInt::IRQ.enable();

            // Enable in ICU
            RxInt::IRQ.icu_enable(I::RX_INTERRUPT_EVENT);
            TeInt::IRQ.icu_enable(I::TE_INTERRUPT_EVENT);
            TxInt::IRQ.icu_enable(I::TX_INTERRUPT_EVENT);
        }

        Self {
            _phantom: PhantomData,
            rx_int: RxInt::IRQ,
            tx_int: TxInt::IRQ,
        }
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
                    sci.scr().modify(|r| {
                        r.set_te(true);
                        r.set_tie(true);
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

            Poll::Ready(Ok(written))
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
                sci.scr().modify(|r| {
                    r.set_te(true);
                    r.set_tie(true);
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
        I::stop_module();
    }
}

impl<I: Instance, RxInt: InterruptType> InterruptHandler<RxInt> for RxInterruptHandler<I> {
    unsafe fn on_interrupt() {
        trace!("RxI");

        let sci = I::regs();
        let mut writer = unsafe { I::rx_buffer().writer() };
        let buf = writer.push_slice();

        match buf.is_empty() {
            false => {
                let fifo_len = sci.fdr().read().r() as _;
                let read_len = buf.len().min(fifo_len);

                for out_byte in buf.iter_mut().take(read_len) {
                    *out_byte = sci.frdrl().read();
                }
                writer.push_done(read_len);

                sci.ssr_fifo().modify(|r| {
                    // If there isn't enough space in the static buffer are we dropping it on the floor when we reset dr?
                    r.set_dr(false);
                    r.set_rdf(false);
                });

                I::rx_waker().wake();

                if read_len != fifo_len {
                    trace!("{}RX Buffer full, FIFO drain={}", I::PERIPHERAL, read_len);
                } else {
                    RxInt::IRQ.icu_unpend();
                }
            }
            true => {
                let fifo_free = I::FIFO_DEPTH - sci.fdr().read().r();

                warn!("{}RX Buffer full, FIFO cap={}", I::PERIPHERAL, fifo_free);

                RxInt::IRQ.icu_unpend();

                I::rx_waker().wake();

                if sci.ssr_fifo().read().orer() {
                    error!("{}Overrun, dropping 1", I::PERIPHERAL);
                    sci.ssr_fifo().modify(|r| r.set_orer(false));
                    RxInt::IRQ.icu_unpend();
                }
            }
        }
    }
}

impl<I: Instance, TeInt: InterruptType> InterruptHandler<TeInt> for TeInterruptHandler<I> {
    unsafe fn on_interrupt() {
        trace!("TeI");
        TeInt::IRQ.icu_unpend();

        let sci = I::regs();

        if I::tx_buffer().is_empty() {
            while !sci.ssr_fifo().read().tend() {
                asm::nop();
            }

            sci.scr().modify(|r| {
                r.set_te(false);
                r.set_teie(false);
                r.set_tie(false);
            });

            I::te_waker().wake();
        } else {
            sci.scr().modify(|r| {
                r.set_te(true);
                r.set_teie(false);
                r.set_tie(true);
            });
        }
    }
}

impl<I: Instance, TxInt: InterruptType> InterruptHandler<TxInt> for TxInterruptHandler<I> {
    unsafe fn on_interrupt() {
        trace!("TxI");
        TxInt::IRQ.icu_unpend();

        let sci = I::regs();
        let mut tx_reader = unsafe { I::tx_buffer().reader() };

        let out_buf = tx_reader.pop_slice();

        if out_buf.is_empty() {
            sci.scr().modify(|r| {
                r.set_teie(true);
                r.set_tie(false);
            });

            return;
        }

        let out_len = out_buf.len();
        let fifo_available = usize::from(I::FIFO_DEPTH - sci.fdr().read().t());

        if out_len > fifo_available {
            for byte in out_buf[0..fifo_available].iter() {
                sci.ftdrl().write_value(*byte);
            }

            tx_reader.pop_done(fifo_available);
        } else {
            for byte in out_buf[0..out_len].iter() {
                sci.ftdrl().write_value(*byte);
                // Should we clear TDFE per Fig 28.14? (RA4M1)
            }

            sci.scr().modify(|r| {
                r.set_teie(true);
                r.set_tie(false);
            });

            tx_reader.pop_done(out_len);
        }

        I::tx_waker().wake();
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

macro_rules! tx_pin {
    ($sci:ident, $pin:ident, $pfunc:ident) => {
        impl crate::uart::TxPin<crate::peripherals::$sci> for crate::peripherals::$pin {}
        impl crate::uart::SealedTxPin<crate::peripherals::$sci> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use tx_pin;

macro_rules! rx_pin {
    ($sci:ident, $pin:ident, $pfunc:ident) => {
        impl crate::uart::RxPin<crate::peripherals::$sci> for crate::peripherals::$pin {}
        impl crate::uart::SealedRxPin<crate::peripherals::$sci> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use rx_pin;

macro_rules! instance_impl {
    ($instance:ident, $rx_int:ident, $te_int:ident, $tx_int:ident) => {
        impl crate::uart::Instance for peripherals::$instance {}

        impl crate::uart::SealedInstance for crate::peripherals::$instance {
            #[cfg(feature = "defmt")]
            const PERIPHERAL: &'static str = concat!(stringify!($instance), ": ");

            const RX_INTERRUPT_EVENT: crate::event_link::InterruptEvent =
                crate::event_link::InterruptEvent::$rx_int;
            const TE_INTERRUPT_EVENT: crate::event_link::InterruptEvent =
                crate::event_link::InterruptEvent::$te_int;
            const TX_INTERRUPT_EVENT: crate::event_link::InterruptEvent =
                crate::event_link::InterruptEvent::$tx_int;

            #[inline]
            fn regs() -> crate::pac::sci_fifo::SciFifo {
                crate::pac::$instance
            }

            fn rx_buffer() -> &'static embassy_hal_internal::atomic_ring_buffer::RingBuffer {
                static RX_BUF: embassy_hal_internal::atomic_ring_buffer::RingBuffer =
                    embassy_hal_internal::atomic_ring_buffer::RingBuffer::new();
                &RX_BUF
            }

            fn tx_buffer() -> &'static embassy_hal_internal::atomic_ring_buffer::RingBuffer {
                static TX_BUF: embassy_hal_internal::atomic_ring_buffer::RingBuffer =
                    embassy_hal_internal::atomic_ring_buffer::RingBuffer::new();
                &TX_BUF
            }

            fn rx_waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
                static RX_WAKER: embassy_sync::waitqueue::AtomicWaker =
                    embassy_sync::waitqueue::AtomicWaker::new();
                &RX_WAKER
            }

            fn te_waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
                static TE_WAKER: embassy_sync::waitqueue::AtomicWaker =
                    embassy_sync::waitqueue::AtomicWaker::new();
                &TE_WAKER
            }

            fn tx_waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
                static TX_WAKER: embassy_sync::waitqueue::AtomicWaker =
                    embassy_sync::waitqueue::AtomicWaker::new();
                &TX_WAKER
            }
        }
    };
}
pub(crate) use instance_impl;
