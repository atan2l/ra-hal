//! I2C Bus Interface (`IIC`).
//!
//! # Notes
//! Is it really worth allocating a buffer and grabbing an interrupt to make up for the lack of built-in FIFO?
//! # TODO
//! * Get clock stuff sorted for not 48 MHz
//! * Error checking and handling

use core::{future::poll_fn, marker::PhantomData, task::Poll};

use crate::{
    event_link::{IcuInterrupt as _, InterruptEvent},
    gpio::{Flex, Pin, PortFunction, WithOpenDrain},
    interrupt::{
        self,
        typelevel::{Handler as InterruptHandler, Interrupt as InterruptType},
    },
    mode::{Async, Blocking, Mode},
    pac,
    write_protect::ProtectedModify,
};

use cortex_m::asm;
use embassy_hal_internal::{
    Peri, PeripheralType, atomic_ring_buffer::RingBuffer, interrupt::InterruptExt as _,
};
use embassy_sync::waitqueue::AtomicWaker;
use embedded_hal_1::i2c::SevenBitAddress;
use ra4m1_ctpac::iic::vals::Cks;

/// I2C driver for the `IIC` peripheral.
///
/// # Notes
///
/// While there are internal pull-up resistors on the pins attached to the `IIC` peripherals they do *not* function in peripheral mode.
#[allow(private_bounds)]
pub struct I2c<'d, M: Mode, I: Instance> {
    _instance: PhantomData<&'d I>,
    _mode: PhantomData<M>,
    // These are set to WithOpenDrain because on the RA4M1 all I2C pins have both capabilities
    _scl: Flex<'d, WithOpenDrain>,
    _sda: Flex<'d, WithOpenDrain>,
}

/// Max supported speed is 400 kHz § 29.1
#[derive(Debug, Default)]
pub enum I2cSpeed {
    /// 100 kHz
    #[default]
    Normal,

    /// 400 kHz
    Fast,
}

/// Interrupt handler that handles incoming data for an [`I2c`] instance.
pub struct RxInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

/// Interrupt handler that handles outgoing data (tx buffer empty) for an [`I2c`] instance.
pub struct TxInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

/// Interrupt handler that handles outgoing data (transmission end) for an [`I2c`] instance.
pub struct TeInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

/// [`I2c`] driver instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

pub(crate) trait SealedInstance {
    #[cfg(feature = "defmt")]
    const PERIPHERAL: &'static str;
    #[cfg(not(feature = "defmt"))]
    const PERIPHERAL: () = ();

    const RX_INTERRUPT_EVENT: InterruptEvent;
    const TE_INTERRUPT_EVENT: InterruptEvent;
    const TX_INTERRUPT_EVENT: InterruptEvent;

    fn regs() -> pac::iic::Iic;
    fn module_stop();
    fn module_start();

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
pub trait SclPin<I: Instance>: SclPinSealed<I> {}

pub(crate) trait SclPinSealed<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_as_scl(&self) {
        trace!("P{}{:02}: SclPin::new", self.port(), self.pin());
        self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

/// GPIO pin connected to the `SDA` line of an [`I2c`] instance.
#[allow(private_bounds)]
pub trait SdaPin<I: Instance>: SdaPinSealed<I> {}

pub(crate) trait SdaPinSealed<I: SealedInstance>: Pin {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_as_sda(&self) {
        trace!("P{}{:02}: SdaPin::new", self.port(), self.pin());
        self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

impl Instance for crate::peripherals::IIC0 {}
impl Instance for crate::peripherals::IIC1 {}

macro_rules! instance_impl {
    ($instance:ident, $mstp:ident, $rx_int:ident, $te_int:ident, $tx_int:ident) => {
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

                #[inline(always)]
                fn module_stop() {
                    debug!("{}: stop=true", stringify!($instance));
                    let mstp = pac::MSTP;
                    mstp.mstpcrb().modify(|w| w.[< set_ $mstp >](true));
                }

                #[inline(always)]
                fn module_start() {
                    debug!("{}: stop=false", stringify!($instance));
                    let mstp = pac::MSTP;
                    mstp.mstpcrb().modify(|w| w.[< set_ $mstp >](false));
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

instance_impl!(IIC0, mstpb9, Iic0Rxi, Iic0Tei, Iic0Txi);
instance_impl!(IIC1, mstpb8, Iic1Rxi, Iic1Tei, Iic1Txi);

#[allow(private_bounds)]
impl<'d, M: Mode, I: Instance> I2c<'d, M, I> {
    async fn write(&self, address: u8, data: &[u8]) -> Result<usize, I2cError> {
        let iic = I::regs();

        if data.is_empty() {
            return Ok(0);
        }

        let w_addr = (address << 1) | 0x00;

        while iic.iccr2().read().bbsy() {
            asm::nop();
        }

        iic.icsr2().modify(|w| {
            w.set_start(false);
            w.set_stop(false);
        });

        iic.icier().write(|w| {
            w.set_stie(true);
            w.set_spie(true);
            w.set_tie(true);
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
                    debug!("{}TX buffer full in async write", "IIC");
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
                    //     iic.iccr1().modify(|w| w.set_ice(false));
                    //     iic.icier().modify(|w| w.set_tie(false));
                    //     while iic.icier().read().tie() {
                    //         asm::nop();
                    //     }
                    //     TxInt::IRQ.unpend();
                    //     TxInt::IRQ.icu_unpend();
                    // }

                    started = true;
                    iic.iccr2().modify(|w| w.set_st(true));
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
        iic.iccr2().modify(|w| w.set_sp(true));

        while !iic.icsr2().read().stop() {
            asm::nop();
        }

        iic.icsr2().modify(|w| {
            w.set_stop(false);
            w.set_nackf(false);
        });

        Ok(ret)
    }

    fn blocking_write(&self, address: u8, data: &[u8]) -> Result<(), I2cError> {
        let iic = I::regs();

        // info!("write! addr={:02x}, len={}", address, data.len());

        let w_addr = (address << 1) | 0x00;

        while iic.iccr2().read().bbsy() {
            asm::nop();
        }
        iic.iccr2().modify(|w| w.set_st(true));

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
            iic.iccr2().modify(|w| w.set_sp(true));
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

        iic.iccr2().modify(|w| w.set_sp(true));

        while !iic.icsr2().read().stop() {
            asm::nop();
        }

        iic.icsr2().modify(|w| {
            w.set_stop(false);
            w.set_nackf(false);
        });

        Ok(())
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

        iic.icier().write(|w| w.set_rie(true));

        while iic.iccr2().read().bbsy() {
            asm::nop();
        }

        iic.iccr2().modify(|w| w.set_st(true));

        while !iic.icsr2().read().tdre() {
            asm::nop();
        }

        iic.icdrt().write_value(r_addr);

        let mut status = iic.icsr2().read();
        while !status.rdrf() {
            if status.nackf() {
                iic.iccr2().modify(|w| w.set_sp(true));
                return Err(I2cError::Nack);
            }
            asm::nop();
            status = iic.icsr2().read();
        }

        // Required dummy read
        let _ = iic.icdrr().read();

        for (i, byte) in data.iter_mut().enumerate() {
            if i == (data_len - 1) {
                iic.icmr3().protected_modify(|w| w.set_ackbt(true));
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

        iic.iccr2().modify(|w| w.set_sp(true));

        // Required dummy read
        let _ = iic.icdrr().read();

        while !iic.icsr2().read().stop() {
            asm::nop()
        }

        iic.icmr3().modify(|w| w.set_wait(false));
        iic.icsr2().modify(|w| w.set_stop(false));

        iic.icier().write(|w| w.set_rie(false));

        Ok(())
    }

    fn blocking_read(&self, address: u8, data: &mut [u8]) -> Result<(), I2cError> {
        let iic = I::regs();
        let r_addr = (address << 1) | 0x01;
        let data_len = data.len();

        while iic.iccr2().read().bbsy() {
            asm::nop();
        }

        iic.iccr2().modify(|w| w.set_st(true));

        while !iic.icsr2().read().tdre() {
            asm::nop();
        }

        iic.icdrt().write_value(r_addr);

        let mut status = iic.icsr2().read();
        while !status.rdrf() {
            if status.nackf() {
                iic.iccr2().modify(|w| w.set_sp(true));
                return Err(I2cError::Nack);
            }
            asm::nop();
            status = iic.icsr2().read();
        }

        // Required dummy read
        let _ = iic.icdrr().read();

        for (i, byte) in data.iter_mut().enumerate() {
            if i == (data_len - 1) {
                iic.icmr3().protected_modify(|w| w.set_ackbt(true));
            }
            while !iic.icsr2().read().rdrf() {
                asm::nop();
            }
            *byte = iic.icdrr().read();
        }

        while !iic.icsr2().read().rdrf() {
            asm::nop();
        }

        iic.iccr2().modify(|w| w.set_sp(true));

        // Required dummy read
        let _ = iic.icdrr().read();

        while !iic.icsr2().read().stop() {
            asm::nop()
        }

        iic.icmr3().modify(|w| w.set_wait(false));
        iic.icsr2().modify(|w| w.set_stop(false));

        Ok(())
    }

    /// Creates a new asynchronous `I2c` driver.
    pub fn new_async<
        C: SclPinSealed<I>,
        D: SdaPinSealed<I>,
        RxInt: InterruptType,
        TeInt: InterruptType,
        TxInt: InterruptType,
    >(
        peri: Peri<'d, I>,
        scl: Peri<'d, C>,
        sda: Peri<'d, D>,
        speed: I2cSpeed,
        tx_buffer: &'d mut [u8],
        _irqs: impl interrupt::typelevel::Binding<RxInt, RxInterruptHandler<I>>
        + interrupt::typelevel::Binding<TeInt, TeInterruptHandler<I>>
        + interrupt::typelevel::Binding<TxInt, TxInterruptHandler<I>>
        + 'd,
    ) -> Self {
        let tx_len = tx_buffer.len();
        unsafe { I::tx_buffer().init(tx_buffer.as_mut_ptr(), tx_len) };

        // Enable in NVIC
        unsafe { RxInt::IRQ.enable() };
        unsafe { TeInt::IRQ.enable() };
        unsafe { TxInt::IRQ.enable() };

        // Enable in ICU
        RxInt::IRQ.icu_enable(I::RX_INTERRUPT_EVENT);
        TeInt::IRQ.icu_enable(I::TE_INTERRUPT_EVENT);
        TxInt::IRQ.icu_enable(I::TX_INTERRUPT_EVENT);

        let iic = I::regs();

        iic.icier().modify(|w| {
            w.set_tie(true);
            w.set_teie(false);
        });

        TeInt::IRQ.icu_unpend();
        TxInt::IRQ.icu_unpend();

        Self::new(peri, scl, sda, speed)
    }

    /// Creates a new blocking `I2c` driver.
    pub fn new<C: SclPinSealed<I>, D: SdaPinSealed<I>>(
        _iic: Peri<'d, I>,
        scl: Peri<'d, C>,
        sda: Peri<'d, D>,
        speed: I2cSpeed,
    ) -> Self {
        I::module_start();

        let iic = I::regs();

        iic.iccr1().write(|w| w.set_ice(false));
        iic.iccr1().modify(|w| w.set_iicrst(true));
        iic.iccr1().modify(|w| w.set_ice(true));

        #[cfg(feature = "hoco_48mhz")]
        match speed {
            // Tlow ≥ 4.7 µs, Thigh ≥ 4.0 µs, UM10204 Table 11
            I2cSpeed::Normal => {
                iic.icmr1().modify(|w| w.set_cks(Cks::PCLKB_4));

                // Use modify here because chiptool doesn't respect the reset values
                // Lifted these from the arduino library.
                iic.icbrl().modify(|w| w.set_brl(26));
                iic.icbrh().modify(|w| w.set_brh(25));
            }
            // Tlow ≥ 1.3 µs, Thigh ≥ 0.6 µs, UM10204 Table 11
            I2cSpeed::Fast => {
                iic.icmr1().modify(|w| w.set_cks(Cks::PCLKB_1));
                iic.icbrl().modify(|w| w.set_brl(24));
                iic.icbrh().modify(|w| w.set_brh(15));
            }
        }

        #[cfg(not(feature = "hoco_48mhz"))]
        compile_error!("Not yet");

        iic.iccr1().modify(|w| w.set_iicrst(false));

        // p826
        // Do not assign the SCLn or SDAn pin to the IIC when setting up the pin function control.
        // Slave address comparison is performed if the pins are assigned to the IIC.
        scl.set_as_scl();
        sda.set_as_sda();

        Self {
            _instance: PhantomData,
            _mode: PhantomData,
            _scl: Flex::new(scl),
            _sda: Flex::new(sda),
        }
    }
}

impl<'d, M: Mode, I: Instance> Drop for I2c<'d, M, I> {
    fn drop(&mut self) {
        I::module_stop();
    }
}

macro_rules! scl_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::i2c::SclPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::i2c::SclPinSealed<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}
pub(crate) use scl_pin;

macro_rules! sda_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::i2c::SdaPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::i2c::SdaPinSealed<crate::peripherals::$instance> for crate::peripherals::$pin {
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

impl<'d, M: Mode, I: Instance> embedded_hal_1::i2c::ErrorType for I2c<'d, M, I> {
    type Error = I2cError;
}

impl<'d, I: Instance> embedded_hal_1::i2c::I2c<SevenBitAddress> for I2c<'d, Blocking, I> {
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

impl<'d, I: Instance> embedded_hal_async::i2c::I2c for I2c<'d, Async, I> {
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

impl<I: Instance, TeInt: InterruptType> InterruptHandler<TeInt> for TeInterruptHandler<I> {
    // Table 29.10, note 4 admonishes us to clear `tend` here, but since we set a stop
    // condition after the waker is awoken this should be okay.
    unsafe fn on_interrupt() {
        trace!("{}TeInt", I::PERIPHERAL);
        TeInt::IRQ.icu_unpend();
        let iic = I::regs();
        iic.icier().modify(|w| w.set_teie(false));
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
            iic.icier().modify(|w| {
                w.set_tie(false);
                w.set_teie(true);
            });

            return;
        }

        iic.icdrt().write_value(out_buf[0]);

        tx_reader.pop_done(1);

        if out_len == 1 {
            iic.icier().modify(|w| {
                w.set_tie(false);
                w.set_teie(true);
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
