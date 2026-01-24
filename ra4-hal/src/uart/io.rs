//! Embedded I/O trait implementations for [`Uart`]

use crate::{
    event_link::IcuEventer,
    interrupt::typelevel::Interrupt,
    uart::{Instance, Uart, UartError},
};

impl<
    'd,
    I: Instance,
    RxInt: Interrupt + IcuEventer,
    TxInt: Interrupt + IcuEventer,
    TeInt: Interrupt + IcuEventer,
> embedded_io_async::ErrorType for Uart<'d, I, RxInt, TxInt, TeInt>
{
    type Error = UartError;
}

impl<
    'd,
    I: Instance,
    RxInt: Interrupt + IcuEventer,
    TxInt: Interrupt + IcuEventer,
    TeInt: Interrupt + IcuEventer,
> embedded_io_async::Read for Uart<'d, I, RxInt, TxInt, TeInt>
{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        Self::read(self, buf).await
    }
}

impl<
    'd,
    I: Instance,
    RxInt: Interrupt + IcuEventer,
    TxInt: Interrupt + IcuEventer,
    TeInt: Interrupt + IcuEventer,
> embedded_io_async::Write for Uart<'d, I, RxInt, TxInt, TeInt>
{
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

impl<
    'd,
    I: Instance,
    RxInt: Interrupt + IcuEventer,
    TxInt: Interrupt + IcuEventer,
    TeInt: Interrupt + IcuEventer,
> embedded_io_async::ReadReady for Uart<'d, I, RxInt, TxInt, TeInt>
{
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Self::read_ready(self)
    }
}
