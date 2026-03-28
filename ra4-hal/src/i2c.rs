//! I2C Bus Interface (`IIC`).
//!
//! The `IIC` peripheral provides hardware support for NXP's I2C at both Normal (100 kbps) and Fast (400 kbps) rates.
//!
//! # Notes
//! * Is it really worth allocating a buffer and grabbing an interrupt to make up for the lack of built-in FIFO?
//! * The `RA4M1` does not provide any pull-up resistors for use with `I2C`.
//!   While there are internal pull-up resistors on the pins attached to the `IIC` peripherals they do *not* function in peripheral mode.
//! * The exact speeds you will see depend on the capacitive load and resistors used.
//! * If you need to set the clocks more precisely use `I2cSpeed::Manual`.
//!
//! # TODO
//! * Error checking and handling
//! * Clock config for arbitrary rates
//! * SMBus mode
//! * Subnode mode

use core::{future::poll_fn, marker::PhantomData, task::Poll};

use cortex_m::asm;
use embassy_hal_internal::{
    Peri, PeripheralType, atomic_ring_buffer::RingBuffer, interrupt::InterruptExt as _,
};
use embassy_sync::waitqueue::AtomicWaker;
use embedded_hal_1::i2c::SevenBitAddress;

use crate::{
    dtc::{Channel as DtcChannel, DtcInterruptHandler, Instance as DtcInstance},
    event_link::{IcuInterrupt as _, InterruptEvent},
    gpio::{Flex, Pin, PortFunction, WithOpenDrain},
    interrupt::{
        self,
        typelevel::{Handler as InterruptHandler, Interrupt as InterruptType},
    },
    module_stop::ModuleStop,
    pac::{self, iic::vals::Cks},
    write_protect::ProtectedModify,
};

/// Represents one of the possible modes of transfer e.g. spin-wait (blocking), interrupt, or DMA.
#[allow(private_bounds)]
pub trait TransferMode: SealedTransferMode {}
trait SealedTransferMode {}

/// Synchronous mode.
pub struct Blocking;

/// Interrupt driven asynchronous mode.
pub struct Async<RxInt: InterruptType, TeInt: InterruptType, TxInt: InterruptType> {
    _rx: PhantomData<RxInt>,
    _te: PhantomData<TeInt>,
    _tx: PhantomData<TxInt>,
}

/// DTC driven asynchronous mode.
pub struct Dtc<RxInt: InterruptType, TxDtc: DtcInstance> {
    rx_dtc: PhantomData<RxInt>,
    tx_dtc: DtcChannel<TxDtc>,
}

impl TransferMode for Blocking {}
impl SealedTransferMode for Blocking {}

impl<RxInt: InterruptType, TeInt: InterruptType, TxInt: InterruptType> TransferMode
    for Async<RxInt, TeInt, TxInt>
{
}

impl<RxInt: InterruptType, TeInt: InterruptType, TxInt: InterruptType> SealedTransferMode
    for Async<RxInt, TeInt, TxInt>
{
}

impl<RxInt: InterruptType, TxDtc: DtcInstance> TransferMode for Dtc<RxInt, TxDtc> {}
impl<RxInt: InterruptType, TxDtc: DtcInstance> SealedTransferMode for Dtc<RxInt, TxDtc> {}

/// I2C driver for the `IIC` peripheral.
#[allow(private_bounds)]
pub struct I2c<'d, I: Instance, M: TransferMode> {
    _instance: PhantomData<&'d I>,
    mode: M,
    // These are set to WithOpenDrain because on the RA4M1 all I2C pins have both capabilities
    _scl: Flex<'d, WithOpenDrain>,
    _sda: Flex<'d, WithOpenDrain>,
}

/// Clock configuration for an [`I2c`] instance.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClockConfig {
    /// Divider for the `IIC` reference clock (`IICϕ`). `PCKLB / divider = IICϕ`. §29.2.3.
    pub divider: Divider,

    /// `I2C` Low-level period (`t_low`). Units are `IICϕ` cycles? §29.2.15.
    pub low: u8,

    /// `I2C` High-level period (`t_high`). Units are `IICϕ` cycles? §29.2.16.
    pub high: u8,
}

/// `IICϕ` divider
#[derive(Copy, Clone, Debug)]
#[allow(missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Divider {
    Div1 = 0x0,
    Div2 = 0x01,
    Div4 = 0x02,
    Div8 = 0x03,
    Div16 = 0x04,
    Div32 = 0x05,
    Div64 = 0x06,
    Div128 = 0x07,
}

/// Pre-baked clock configuration.
///
/// Due to the nature of I2C, timing depends on the bus capacitance.
/// If more precise control is needed, use the Manual variant.
/// Max supported speed is 400 kHz § 29.1
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cSpeed {
    /// 100 kHz
    #[default]
    Normal,

    /// 400 kHz
    Fast,

    /// Set the clock configuration explicitly, see § 29.2.16.
    Manual(ClockConfig),
}

/// Interrupt handler that handles incoming data for an [`I2c`] instance.
pub struct RxInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

/// Interrupt handler that handles outgoing data (transmission end) for an [`I2c`] instance.
pub struct TeInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

/// Interrupt handler that handles outgoing data (tx buffer empty) for an [`I2c`] instance.
pub struct TxInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

/// [`I2c`] driver instance.
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

    fn regs() -> pac::iic::Iic;

    /// Waker for receive data events.
    fn rx_waker() -> &'static AtomicWaker;

    /// Waker for transmit buffer empty events.
    fn tx_waker() -> &'static AtomicWaker;

    /// Waker for "transmit end" events.
    fn te_waker() -> &'static AtomicWaker;

    /// # Returns
    /// Static [`RingBuffer`] for outgoing data.
    fn tx_buffer() -> &'static RingBuffer;
}

/// GPIO pin connected to the `SCL` line of an [`I2c`] instance.
#[allow(private_bounds)]
pub trait SclPin<I: Instance>: SealedSclPin<I> {}

pub(crate) trait SealedSclPin<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline]
    fn set_as_scl(&self) {
        self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

/// GPIO pin connected to the `SDA` line of an [`I2c`] instance.
#[allow(private_bounds)]
pub trait SdaPin<I: Instance>: SealedSdaPin<I> {}

pub(crate) trait SealedSdaPin<I: SealedInstance>: Pin {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline]
    fn set_as_sda(&self) {
        self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

impl From<Divider> for Cks {
    fn from(value: Divider) -> Self {
        Self::from_bits(value as u8)
    }
}

impl Instance for crate::peripherals::IIC0 {}
impl Instance for crate::peripherals::IIC1 {}

macro_rules! instance_impl {
    ($instance:ident, $rx_int:ident, $te_int:ident, $tx_int:ident) => {
        paste::paste! {
            impl SealedInstance for crate::peripherals::$instance {
                #[cfg(feature = "defmt")]
                const PERIPHERAL: &'static str = concat!(stringify!($instance), ": ");

                const RX_INTERRUPT_EVENT: InterruptEvent = InterruptEvent::$rx_int;
                const TE_INTERRUPT_EVENT: InterruptEvent = InterruptEvent::$te_int;
                const TX_INTERRUPT_EVENT: InterruptEvent = InterruptEvent::$tx_int;

                #[inline(always)]
                fn regs() -> pac::iic::Iic {
                    crate::pac::$instance
                }

                /// Waker for incoming data events.
                fn rx_waker() -> &'static AtomicWaker{
                    static RX_WAKER: AtomicWaker = AtomicWaker::new();
                    &RX_WAKER
                }

                /// Waker for "transmit end" events.
                fn te_waker() -> &'static AtomicWaker{
                    static TE_WAKER: AtomicWaker = AtomicWaker::new();
                    &TE_WAKER
                }

                /// Waker for transmit buffer empty events.
                fn tx_waker() -> &'static AtomicWaker{
                    static TX_WAKER: AtomicWaker = AtomicWaker::new();
                    &TX_WAKER
                }

                fn tx_buffer() -> &'static RingBuffer {
                    static TX_BUF: RingBuffer = RingBuffer::new();
                    &TX_BUF
                }
            }
        }
    };
}

instance_impl!(IIC0, Iic0Rxi, Iic0Tei, Iic0Txi);
instance_impl!(IIC1, Iic1Rxi, Iic1Tei, Iic1Txi);

impl<'d, I: Instance> I2c<'d, I, Blocking> {
    /// Creates a new blocking `I2c` driver.
    ///
    /// # Arguments
    /// * `iic` An [`IIC`](trait@Instance) peripheral.
    /// * `scl` A [`Pin`] that can be connected to the [clock line](trait@SclPin).
    /// * `sda` A [`Pin`] that can be connected to the [data line](trait@SdaPin).
    /// * `speed` Bitrate.
    ///
    #[inline]
    pub fn new_blocking<C: SclPin<I>, D: SdaPin<I>>(
        iic: Peri<'d, I>,
        scl: Peri<'d, C>,
        sda: Peri<'d, D>,
        speed: I2cSpeed,
    ) -> Self {
        Self::new_inner(iic, scl, sda, speed, Blocking {})
    }

    fn blocking_read(&self, address: u8, data: &mut [u8]) -> Result<(), I2cError> {
        let iic = I::regs();
        let r_addr = (address << 1) | 0x01;
        let data_len = data.len();

        while iic.iccr2().read().bbsy() {
            asm::nop();
        }

        iic.iccr2().modify(|r| r.set_st(true));

        while !iic.icsr2().read().tdre() {
            asm::nop();
        }

        iic.icdrt().write_value(r_addr);

        let mut status = iic.icsr2().read();
        while !status.rdrf() {
            if status.nackf() {
                iic.iccr2().modify(|r| r.set_sp(true));
                return Err(I2cError::Nack);
            }
            asm::nop();
            status = iic.icsr2().read();
        }

        // Required dummy read
        let _ = iic.icdrr().read();

        for (i, byte) in data.iter_mut().enumerate() {
            if i == (data_len - 1) {
                iic.icmr3().protected_modify(|r| r.set_ackbt(true));
            }
            while !iic.icsr2().read().rdrf() {
                asm::nop();
            }
            *byte = iic.icdrr().read();
        }

        while !iic.icsr2().read().rdrf() {
            asm::nop();
        }

        iic.iccr2().modify(|r| r.set_sp(true));

        // Required dummy read
        let _ = iic.icdrr().read();

        while !iic.icsr2().read().stop() {
            asm::nop()
        }

        iic.icmr3().modify(|r| r.set_wait(false));
        iic.icsr2().modify(|r| r.set_stop(false));

        Ok(())
    }

    fn blocking_write(&self, address: u8, data: &[u8]) -> Result<(), I2cError> {
        let iic = I::regs();

        // info!("write! addr={:02x}, len={}", address, data.len());

        let w_addr = (address << 1) | 0x00;

        while iic.iccr2().read().bbsy() {
            asm::nop();
        }
        iic.iccr2().modify(|r| r.set_st(true));

        while !iic.icsr2().read().tdre() {
            if iic.icsr2().read().nackf() {
                return Err(I2cError::Nack);
            }
            asm::nop();
        }

        iic.icdrt().write_value(w_addr);
        while !iic.icsr2().read().tdre() {
            asm::nop();
        }

        if iic.icsr2().read().nackf() {
            iic.iccr2().modify(|r| r.set_sp(true));
            return Err(I2cError::Nack);
        }

        for byte in data.iter() {
            while !iic.icsr2().read().tdre() {
                asm::nop();
            }
            iic.icdrt().write_value(*byte);
        }

        while !iic.icsr2().read().tend() {
            asm::nop();
        }

        iic.iccr2().modify(|r| r.set_sp(true));

        while !iic.icsr2().read().stop() {
            asm::nop();
        }

        iic.icsr2().modify(|r| {
            r.set_stop(false);
            r.set_nackf(false);
        });

        Ok(())
    }
}

impl<'d, I: Instance, RxInt: InterruptType, TxDtc: DtcInstance> I2c<'d, I, Dtc<RxInt, TxDtc>> {
    /// Creates a new asynchronous, [`DTC`](module@crate::dtc) driven `I2c` driver.
    ///
    /// # Arguments
    /// * `iic` An [`IIC`](trait@Instance) peripheral.
    /// * `scl` A [`Pin`] that can be connected to the [clock line](trait@SclPin).
    /// * `sda` A [`Pin`] that can be connected to the [data line](trait@SdaPin).
    /// * `speed` Bitrate.
    /// * `tx_dtc` DTC peripheral for transmission.
    /// * `irqs` interrupts bound with the [`bind_interrupts!`](crate::bind_interrupts) macro.
    ///
    /// # Notes
    /// This uses `DTC` for transmission and interrupts for reception.
    /// Due to limitations with the `IIC` peripheral DMA/DTC reception offers little, if any, benefit.
    pub fn new_dtc<C: SclPin<I>, D: SdaPin<I>>(
        iic: Peri<'d, I>,
        scl: Peri<'d, C>,
        sda: Peri<'d, D>,
        speed: I2cSpeed,
        tx_dtc: Peri<'d, TxDtc>,
        irqs: impl interrupt::typelevel::Binding<RxInt, RxInterruptHandler<I>>
        + interrupt::typelevel::Binding<TxDtc::Int, DtcInterruptHandler<TxDtc>>
        + Clone
        + 'd,
    ) -> Self {
        let tx_dtc = DtcChannel::new(tx_dtc, irqs);
        let dtc = Dtc {
            tx_dtc,
            rx_dtc: PhantomData,
        };

        Self::new_inner(iic, scl, sda, speed, dtc)
    }

    async fn dtc_write(&mut self, address: u8, data: &[u8]) -> Result<usize, I2cError> {
        let iic = I::regs();

        if data.is_empty() {
            return Ok(0);
        }

        let w_addr = (address << 1) | 0x00;

        while iic.iccr2().read().bbsy() {}

        iic.icsr2().modify(|r| {
            r.set_start(false);
            r.set_stop(false);
        });

        iic.icier().write(|r| {
            r.set_stie(true);
            r.set_spie(true);
            r.set_tie(true);
        });

        let addr = [w_addr];

        let tx1 = self
            .mode
            .tx_dtc
            .prepare_write(&addr, iic.icdrt().as_ptr() as *mut _, false);
        let tx2 = self
            .mode
            .tx_dtc
            .prepare_write(data, iic.icdrt().as_ptr() as *mut _, true);
        let chain = [tx1, tx2];

        // Safety: The address buffer, data buffer, and the register should outlive the transfer.
        let tx = unsafe { self.mode.tx_dtc.write_chain(&chain, I::TX_INTERRUPT_EVENT) };

        iic.iccr2().modify(|r| r.set_st(true));

        tx.await;

        // This shouldn't take long and isn't worth the overhead of an interrupt
        while !iic.icsr2().read().tend() {}

        // Reading `tend` will not clear it, but a stop condition will.
        iic.iccr2().modify(|r| r.set_sp(true));

        while !iic.icsr2().read().stop() {}

        iic.icsr2().modify(|r| {
            r.set_stop(false);
            r.set_nackf(false);
        });

        Ok(addr.len())
    }

    async fn dtc_read(&self, address: u8, data: &mut [u8]) -> Result<(), I2cError> {
        let iic = I::regs();
        let r_addr = (address << 1) | 0x01;
        let data_len = data.len();

        iic.icier().write(|r| r.set_rie(true));

        while iic.iccr2().read().bbsy() {
            asm::nop();
        }

        iic.iccr2().modify(|r| r.set_st(true));

        while !iic.icsr2().read().tdre() {
            asm::nop();
        }

        iic.icdrt().write_value(r_addr);

        let mut status = iic.icsr2().read();
        while !status.rdrf() {
            if status.nackf() {
                iic.iccr2().modify(|r| r.set_sp(true));
                return Err(I2cError::Nack);
            }
            asm::nop();
            status = iic.icsr2().read();
        }

        // Required dummy read
        let _ = iic.icdrr().read();

        for (i, byte) in data.iter_mut().enumerate() {
            if i == (data_len - 1) {
                iic.icmr3().protected_modify(|r| r.set_ackbt(true));
            }

            *byte = poll_fn(|cx| {
                if iic.icsr2().read().rdrf() {
                    let byte = iic.icdrr().read();
                    Poll::Ready(byte)
                } else {
                    I::rx_waker().register(cx.waker());
                    Poll::Pending
                }
            })
            .await;
        }

        poll_fn(|cx| {
            if iic.icsr2().read().rdrf() {
                Poll::Ready(())
            } else {
                I::rx_waker().register(cx.waker());
                Poll::Pending
            }
        })
        .await;

        iic.iccr2().modify(|r| r.set_sp(true));

        // Required dummy read
        let _ = iic.icdrr().read();

        while !iic.icsr2().read().stop() {
            asm::nop()
        }

        iic.icmr3().modify(|r| r.set_wait(false));
        iic.icsr2().modify(|r| r.set_stop(false));

        iic.icier().write(|r| r.set_rie(false));

        Ok(())
    }
}

impl<'d, I: Instance, RxInt: InterruptType, TeInt: InterruptType, TxInt: InterruptType>
    I2c<'d, I, Async<RxInt, TeInt, TxInt>>
{
    /// Creates a new asynchronous, interrupt driven `I2c` driver.
    ///
    /// # Arguments
    /// * `iic` An [`IIC`](trait@Instance) peripheral.
    /// * `scl` A [`Pin`] that can be connected to the [clock line](trait@SclPin).
    /// * `sda` A [`Pin`] that can be connected to the [data line](trait@SdaPin).
    /// * `speed` Bitrate.
    /// * `tx_buffer` Transmit buffer.
    /// * `irqs` interrupts bound with the [`bind_interrupts!`](crate::bind_interrupts) macro.
    pub fn new_async<C: SclPin<I>, D: SdaPin<I>>(
        iic: Peri<'d, I>,
        scl: Peri<'d, C>,
        sda: Peri<'d, D>,
        speed: I2cSpeed,
        tx_buffer: &'d mut [u8],
        irqs: impl interrupt::typelevel::Binding<RxInt, RxInterruptHandler<I>>
        + interrupt::typelevel::Binding<TeInt, TeInterruptHandler<I>>
        + interrupt::typelevel::Binding<TxInt, TxInterruptHandler<I>>
        + 'd,
    ) -> Self {
        let _ = irqs;
        let tx_len = tx_buffer.len();

        // Safety: Caller defines the backing store for the ring buffer and the borrow checker
        //         should ensure that it remains in place as long as we need.
        unsafe { I::tx_buffer().init(tx_buffer.as_mut_ptr(), tx_len) };

        // Safety: Interrupt handlers are defined by the irqs argument and thus the interrupts are safe to enable.
        unsafe {
            // Enable in NVIC
            RxInt::IRQ.enable();
            TeInt::IRQ.enable();
            TxInt::IRQ.enable();

            // Enable in ICU
            RxInt::IRQ.icu_enable(I::RX_INTERRUPT_EVENT);
            TeInt::IRQ.icu_enable(I::TE_INTERRUPT_EVENT);
            TxInt::IRQ.icu_enable(I::TX_INTERRUPT_EVENT);
        }

        let iic_regs = I::regs();

        iic_regs.icier().modify(|r| {
            r.set_tie(true);
            r.set_teie(false);
        });

        TeInt::IRQ.icu_unpend();
        TxInt::IRQ.icu_unpend();

        Self::new_inner(
            iic,
            scl,
            sda,
            speed,
            Async {
                _rx: PhantomData,
                _te: PhantomData,
                _tx: PhantomData,
            },
        )
    }

    async fn write(&self, address: u8, data: &[u8]) -> Result<usize, I2cError> {
        let iic = I::regs();

        if data.is_empty() {
            return Ok(0);
        }

        let w_addr = (address << 1) | 0x00;

        while iic.iccr2().read().bbsy() {}

        iic.icsr2().modify(|r| {
            r.set_start(false);
            r.set_stop(false);
        });

        iic.icier().write(|r| {
            r.set_stie(true);
            r.set_spie(true);
            r.set_tie(true);
        });

        let mut written: usize = 0;
        let mut writer = unsafe { I::tx_buffer().writer() };

        let out = writer.push_slice();
        if out.is_empty() {
            error!("{}TX buffer is full", I::PERIPHERAL);
        }
        out[0] = w_addr;
        writer.push_done(1);
        let mut started = false;

        let ret = poll_fn(|cx| {
            if written < data.len() {
                I::tx_waker().register(cx.waker());

                if I::tx_buffer().is_full() {
                    debug!("{}TX buffer full in async write", I::PERIPHERAL);
                    return Poll::Pending;
                }

                let out = writer.push_slice();
                let chunk_len = out.len().min(data.len().saturating_sub(written));
                out[..chunk_len].copy_from_slice(&data[written..(written + chunk_len)]);

                writer.push_done(chunk_len);

                written += chunk_len;

                if !started {
                    // TODO: § 29.17.2
                    // if TxInt::IRQ.icu_is_pending() {
                    //     iic.iccr1().modify(r| r.set_ice(false));
                    //     iic.icier().modify(r| r.set_tie(false));
                    //     while iic.icier().read().tie() {
                    //         asm::nop();
                    //     }
                    //     TxInt::IRQ.unpend();
                    //     TxInt::IRQ.icu_unpend();
                    // }

                    started = true;
                    iic.iccr2().modify(|r| r.set_st(true));
                }

                if written < data.len() {
                    return Poll::Pending;
                }

                // If there's nothing else wait on the Transmit End interrupt
                I::te_waker().register(cx.waker());

                if !iic.icsr2().read().tend() {
                    return Poll::Pending;
                }
            } else {
                I::te_waker().register(cx.waker());

                if !iic.icsr2().read().tend() {
                    return Poll::Pending;
                }
            }

            Poll::Ready(Ok(written))
        })
        .await?;

        // Reading `tend` will not clear it, but a stop condition will.
        iic.iccr2().modify(|r| r.set_sp(true));

        while !iic.icsr2().read().stop() {}

        iic.icsr2().modify(|r| {
            r.set_stop(false);
            r.set_nackf(false);
        });

        Ok(ret)
    }

    async fn read_byte(&self) -> u8 {
        let iic = I::regs();

        poll_fn(|cx| {
            if iic.icsr2().read().rdrf() {
                let byte = iic.icdrr().read();
                Poll::Ready(byte)
            } else {
                I::rx_waker().register(cx.waker());
                Poll::Pending
            }
        })
        .await
    }

    async fn read(&self, address: u8, data: &mut [u8]) -> Result<(), I2cError> {
        let iic = I::regs();
        let r_addr = (address << 1) | 0x01;
        let data_len = data.len();

        iic.icier().write(|r| r.set_rie(true));

        while iic.iccr2().read().bbsy() {
            asm::nop();
        }

        iic.iccr2().modify(|r| r.set_st(true));

        while !iic.icsr2().read().tdre() {
            asm::nop();
        }

        iic.icdrt().write_value(r_addr);

        let mut status = iic.icsr2().read();
        while !status.rdrf() {
            if status.nackf() {
                iic.iccr2().modify(|r| r.set_sp(true));
                return Err(I2cError::Nack);
            }
            asm::nop();
            status = iic.icsr2().read();
        }

        // Required dummy read
        let _ = iic.icdrr().read();

        for (i, byte) in data.iter_mut().enumerate() {
            if i == (data_len - 1) {
                iic.icmr3().protected_modify(|r| r.set_ackbt(true));
            }

            *byte = self.read_byte().await;
        }

        poll_fn(|cx| {
            if iic.icsr2().read().rdrf() {
                Poll::Ready(())
            } else {
                I::rx_waker().register(cx.waker());
                Poll::Pending
            }
        })
        .await;

        iic.iccr2().modify(|r| r.set_sp(true));

        // Required dummy read
        let _ = iic.icdrr().read();

        while !iic.icsr2().read().stop() {
            asm::nop()
        }

        iic.icmr3().modify(|r| r.set_wait(false));
        iic.icsr2().modify(|r| r.set_stop(false));

        iic.icier().write(|r| r.set_rie(false));

        Ok(())
    }
}

#[allow(private_bounds)]
impl<'d, I: Instance, M: TransferMode> I2c<'d, I, M> {
    #[inline]
    fn new_inner<C: SclPin<I>, D: SdaPin<I>>(
        iic: Peri<'d, I>,
        scl: Peri<'d, C>,
        sda: Peri<'d, D>,
        speed: I2cSpeed,
        mode: M,
    ) -> Self {
        #[cfg(feature = "hoco_48mhz")]
        #[rustfmt::skip]
        let clk_config = [
            ClockConfig { divider: Divider::Div4, low: 25, high: 24 },
            ClockConfig { divider: Divider::Div1, low: 24, high: 16 },
        ];
        #[cfg(any(feature = "hoco_32mhz", feature = "hoco_64mhz"))]
        #[rustfmt::skip]
        let clk_config = [
            ClockConfig { divider: Divider::Div8, low: 15, high: 13 },
            ClockConfig { divider: Divider::Div2, low: 18, high: 9 },
        ];

        let clocks = match speed {
            // Tlow ≥ 4.7 µs, Thigh ≥ 4.0 µs, UM10204 Table 11
            I2cSpeed::Normal => clk_config[0],
            // Tlow ≥ 1.3 µs, Thigh ≥ 0.6 µs, UM10204 Table 11
            I2cSpeed::Fast => clk_config[1],
            I2cSpeed::Manual(clk_config) => clk_config,
        };

        let _ = iic;
        I::start_module();

        let iic = I::regs();

        iic.iccr1().write(|r| r.set_ice(false));
        iic.iccr1().modify(|r| r.set_iicrst(true));
        iic.iccr1().modify(|r| r.set_ice(true));

        iic.icmr1().modify(|r| r.set_cks(clocks.divider.into()));

        // Use modify here because chiptool doesn't respect the reset values
        // Lifted these from the arduino library.
        iic.icbrl().modify(|r| r.set_brl(clocks.low));
        iic.icbrh().modify(|r| r.set_brh(clocks.high));

        iic.iccr1().modify(|r| r.set_iicrst(false));

        // p826
        // Do not assign the SCLn or SDAn pin to the IIC when setting up the pin function control.
        // Slave address comparison is performed if the pins are assigned to the IIC.
        scl.set_as_scl();
        sda.set_as_sda();

        Self {
            _instance: PhantomData,
            mode,
            _scl: Flex::new(scl),
            _sda: Flex::new(sda),
        }
    }
}

impl<RxInt: InterruptType, TeInt: InterruptType, TxInt: InterruptType> Drop
    for Async<RxInt, TeInt, TxInt>
{
    fn drop(&mut self) {
        RxInt::IRQ.icu_disable();
        TeInt::IRQ.icu_disable();
        TxInt::IRQ.icu_disable();
    }
}

impl<RxInt: InterruptType, TxDtc: DtcInstance> Drop for Dtc<RxInt, TxDtc> {
    fn drop(&mut self) {
        RxInt::IRQ.icu_disable();
        TxDtc::Int::IRQ.dtc_disable();
    }
}

impl<'d, I: Instance, M: TransferMode> Drop for I2c<'d, I, M> {
    fn drop(&mut self) {
        I::stop_module();
    }
}

macro_rules! scl_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::i2c::SclPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::i2c::SealedSclPin<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use scl_pin;

macro_rules! sda_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::i2c::SdaPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::i2c::SealedSdaPin<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use sda_pin;

/// I2C error.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cError {
    /// Arbitration lost
    Arbitration,

    /// ACK not received (either to the address or to a data byte)
    Nack,

    /// Overrun error
    Overrun,
}

impl core::fmt::Display for I2cError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let message = match self {
            Self::Arbitration => "Arbitration Lost",
            Self::Nack => "ACK Not Received",
            Self::Overrun => "Buffer Overrun",
        };

        write!(f, "{}", message)
    }
}

impl core::error::Error for I2cError {}

impl embedded_hal_1::i2c::Error for I2cError {
    fn kind(&self) -> embedded_hal_1::i2c::ErrorKind {
        match *self {
            Self::Arbitration => embedded_hal_1::i2c::ErrorKind::ArbitrationLoss,
            Self::Nack => embedded_hal_1::i2c::ErrorKind::NoAcknowledge(
                embedded_hal_1::i2c::NoAcknowledgeSource::Unknown,
            ),
            Self::Overrun => embedded_hal_1::i2c::ErrorKind::Overrun,
        }
    }
}

impl<'d, I: Instance, M: TransferMode> embedded_hal_1::i2c::ErrorType for I2c<'d, I, M> {
    type Error = I2cError;
}

impl<'d, I: Instance> embedded_hal_1::i2c::I2c<SevenBitAddress> for I2c<'d, I, Blocking> {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal_1::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        for op in operations.iter_mut() {
            match op {
                embedded_hal_1::i2c::Operation::Read(buffer) => {
                    self.blocking_read(address, buffer)?;
                }
                embedded_hal_1::i2c::Operation::Write(data) => {
                    self.blocking_write(address, data)?;
                }
            }
        }

        Ok(())
    }
}

impl<'d, I: Instance, RxInt: InterruptType, TeInt: InterruptType, TxInt: InterruptType>
    embedded_hal_async::i2c::I2c for I2c<'d, I, Async<RxInt, TeInt, TxInt>>
{
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal_1::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        for op in operations.iter_mut() {
            match op {
                embedded_hal_1::i2c::Operation::Read(buffer) => {
                    I2c::read(self, address, buffer).await?;
                }
                embedded_hal_1::i2c::Operation::Write(data) => {
                    I2c::write(self, address, data).await?;
                }
            }
        }

        Ok(())
    }
}

impl<'d, I: Instance, RxInt: InterruptType, TxDtc: DtcInstance> embedded_hal_async::i2c::I2c
    for I2c<'d, I, Dtc<RxInt, TxDtc>>
{
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal_1::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        for op in operations.iter_mut() {
            match op {
                embedded_hal_1::i2c::Operation::Read(buffer) => {
                    I2c::dtc_read(self, address, buffer).await?;
                }
                embedded_hal_1::i2c::Operation::Write(data) => {
                    I2c::dtc_write(self, address, data).await?;
                }
            }
        }

        Ok(())
    }
}

impl<I: Instance, TeInt: InterruptType> InterruptHandler<TeInt> for TeInterruptHandler<I> {
    // Table 29.10, note 4 admonishes us to clear `tend` here, but since we set a stop
    // condition after the waker is awoken this should be okay.
    unsafe fn on_interrupt() {
        trace!("{}TeInt", I::PERIPHERAL);
        TeInt::IRQ.icu_unpend();
        let iic = I::regs();
        iic.icier().modify(|r| r.set_teie(false));
        I::te_waker().wake();
    }
}

impl<I: Instance, TxInt: InterruptType> InterruptHandler<TxInt> for TxInterruptHandler<I> {
    unsafe fn on_interrupt() {
        trace!("{}TxInt", I::PERIPHERAL);
        TxInt::IRQ.icu_unpend();

        let iic = I::regs();
        let mut tx_reader = unsafe { I::tx_buffer().reader() };
        let out_buf = tx_reader.pop_slice();
        let out_len = out_buf.len();

        if out_buf.is_empty() {
            iic.icier().modify(|r| {
                r.set_tie(false);
                r.set_teie(true);
            });

            return;
        }

        iic.icdrt().write_value(out_buf[0]);

        tx_reader.pop_done(1);

        if out_len == 1 {
            iic.icier().modify(|r| {
                r.set_tie(false);
                r.set_teie(true);
            });
            return;
        }

        I::tx_waker().wake();
    }
}

impl<I: Instance, RxInt: InterruptType> InterruptHandler<RxInt> for RxInterruptHandler<I> {
    unsafe fn on_interrupt() {
        trace!("{}RxI", I::PERIPHERAL);

        RxInt::IRQ.icu_unpend();

        I::rx_waker().wake();
    }
}
