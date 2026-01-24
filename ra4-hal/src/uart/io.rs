//! `embedded-io`, `embedded-io-async`, and `embedded-serial` trait implementations for [`Uart`]

use crate::uart::{Instance, Uart, UartError};

impl<'d, I: Instance> embedded_io_async::ErrorType for Uart<'d, I> {
    type Error = UartError;
}

impl<'d, I: Instance> embedded_io_async::Read for Uart<'d, I> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        Self::read(self, buf).await
    }
}

impl<'d, I: Instance> embedded_io_async::Write for Uart<'d, I> {
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

impl<'d, I: Instance> embedded_io_async::ReadReady for Uart<'d, I> {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Self::read_ready(self)
    }
}

impl<'d, I: Instance> embedded_serial::MutBlockingTx for Uart<'d, I> {
    type Error = ();

    // TODO: Change… "optimize" this so we only wait for data to leave the ring buffer.
    fn putc(&mut self, ch: u8) -> Result<(), Self::Error> {
        Self::blocking_write(self, &[ch]);
        Ok(())
    }
}

impl<'d, I: Instance> embedded_serial::MutBlockingRx for Uart<'d, I> {
    type Error = ();

    fn getc(&mut self) -> Result<u8, Self::Error> {
        let mut ch = [0_u8];

        Self::blocking_read(self, &mut ch);

        Ok(ch[0])
    }
}
