//! USB Endpoint, for operations that work with a single endpoint.

use core::{marker::PhantomData, sync::atomic::Ordering, task::Poll};

use embassy_usb_driver::{
    Endpoint as DriverEndpoint, EndpointError, EndpointIn as DriverEndpointIn, EndpointInfo,
    EndpointOut as DriverEndpointOut,
};

use crate::{
    pac::usbfs::vals::{CfifoselCurpipe, PipectrPid},
    usb::{Instance as UsbInstance, STATE},
};

/// A USB endpoint for IN transfers that implements `embassy_usb_driver::EndpointIn`.
pub struct EndpointIn<'a, I: UsbInstance> {
    pub(crate) _phantom: PhantomData<&'a I>,
    pub(crate) info: EndpointInfo,
    pub(crate) pipe: u8,
}

/// A USB endpoint for OUT transfers that implements `embassy_usb_driver::EndpointOut`.
pub struct EndpointOut<'a, I: UsbInstance> {
    pub(crate) _phantom: PhantomData<&'a I>,
    pub(crate) info: EndpointInfo,
    pub(crate) pipe: u8,
}

impl<'a, I: UsbInstance> DriverEndpoint for EndpointIn<'a, I> {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        let pipe = self.pipe as usize;
        core::future::poll_fn(|cx| {
            STATE.ep_wakers[pipe].register(cx.waker());
            if STATE.pipe_enabled.load(Ordering::Acquire) & (1 << pipe) != 0 {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
    }
}

impl<'a, I: UsbInstance> DriverEndpointIn for EndpointIn<'a, I> {
    async fn write(&mut self, buf: &[u8]) -> Result<(), EndpointError> {
        trace!(
            "USB: EP{}IN write {} bytes",
            self.info.addr.index(),
            buf.len()
        );
        let r = I::regs();
        let pipe = self.pipe as usize;
        let pipectr_idx = pipe - 1;

        if STATE.pipe_enabled.load(Ordering::Acquire) & (1 << pipe) == 0 {
            return Err(EndpointError::Disabled);
        }

        if buf.len() > self.info.max_packet_size as _ {
            return Err(EndpointError::BufferOverflow);
        }

        core::future::poll_fn(|cx| {
            STATE.ep_wakers[pipe].register(cx.waker());

            if STATE.pipe_enabled.load(Ordering::Acquire) & (1 << pipe) == 0 {
                return Poll::Ready(Err(EndpointError::Disabled));
            }

            if STATE.pipe_bemp.load(Ordering::Acquire) & (1 << pipe) != 0 {
                STATE.pipe_bemp.fetch_and(!(1 << pipe), Ordering::AcqRel);
                Poll::Ready(Ok(()))
            } else {
                // Ensure PID is BUF so BEMP can fire
                r.pipectr(pipectr_idx)
                    .modify(|r| r.set_pid(PipectrPid::Buffer));
                Poll::Pending
            }
        })
        .await?;

        // Point the CFIFO at this pipe
        r.cfifosel()
            .modify(|r| r.set_curpipe(CfifoselCurpipe::from_bits(self.pipe)));

        // Wait for FIFO to be ready
        while !r.cfifoctr().read().frdy() {
            cortex_m::asm::nop();
        }

        for &byte in buf {
            r.cfifol().write(|w| w.set_fifoport(byte));
        }

        // Commit and arm for transmission
        r.cfifoctr().modify(|r| r.set_bval(true));
        r.pipectr(pipectr_idx)
            .modify(|r| r.set_pid(PipectrPid::Buffer));

        Ok(())
    }
}

impl<'a, I: UsbInstance> DriverEndpoint for EndpointOut<'a, I> {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        let pipe = self.pipe as usize;
        core::future::poll_fn(|cx| {
            STATE.ep_wakers[pipe].register(cx.waker());
            if STATE.pipe_enabled.load(Ordering::Acquire) & (1 << pipe) != 0 {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
    }
}

impl<'a, I: UsbInstance> DriverEndpointOut for EndpointOut<'a, I> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        trace!("USB: EP{}OUT read waiting", self.info.addr.index());
        let r = I::regs();
        let pipe = self.pipe as usize;
        let pipectr_idx = pipe - 1;

        if STATE.pipe_enabled.load(Ordering::Acquire) & (1 << pipe) == 0 {
            return Err(EndpointError::Disabled);
        }

        // Wait for data to arrive in the FIFO (BRDY)
        core::future::poll_fn(|cx| {
            STATE.ep_wakers[pipe].register(cx.waker());
            if STATE.pipe_enabled.load(Ordering::Acquire) & (1 << pipe) == 0 {
                return Poll::Ready(Err(EndpointError::Disabled));
            }

            if STATE.pipe_brdy.load(Ordering::Acquire) & (1 << pipe) != 0 {
                STATE.pipe_brdy.fetch_and(!(1 << pipe), Ordering::AcqRel);
                Poll::Ready(Ok(()))
            } else {
                Poll::Pending
            }
        })
        .await?;

        // Point the CFIFO at this pipe
        r.cfifosel()
            .modify(|r| r.set_curpipe(CfifoselCurpipe::from_bits(self.pipe)));

        // Wait for FIFO to be ready
        while !r.cfifoctr().read().frdy() {
            cortex_m::asm::nop();
        }

        let data_len = r.cfifoctr().read().dtln() as usize;
        if data_len > buf.len() {
            r.cfifoctr().modify(|r| r.set_bclr(true));
            r.pipectr(pipectr_idx)
                .modify(|r| r.set_pid(PipectrPid::Buffer));
            return Err(EndpointError::BufferOverflow);
        }

        for byte in buf[..data_len].iter_mut() {
            *byte = r.cfifol().read().fifoport();
        }

        // Clear buffer pointer and re-arm
        r.cfifoctr().modify(|r| r.set_bclr(true));
        r.pipectr(pipectr_idx)
            .modify(|r| r.set_pid(PipectrPid::Buffer));

        trace!(
            "USB: EP{}OUT read {} bytes",
            self.info.addr.index(),
            data_len
        );

        Ok(data_len)
    }
}
