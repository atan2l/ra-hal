//! USB Control Pipe, for operations that work on the control endpoint (EP0).

use core::{marker::PhantomData, sync::atomic::Ordering, task::Poll};

use embassy_usb_driver::{ControlPipe as DriverControlPipe, EndpointError};

use crate::{
    pac::usbfs::vals::{CfifoselCurpipe, DcpctrPid},
    usb::{Instance, STATE},
};

/// USB control endpoint IN+OUT.
pub struct ControlPipe<'a, I: Instance> {
    pub(crate) _phantom: PhantomData<&'a I>,
    pub(crate) max_packet_size: u16,
}

impl<'a, I: Instance> DriverControlPipe for ControlPipe<'a, I> {
    fn max_packet_size(&self) -> usize {
        self.max_packet_size as _
    }

    async fn setup(&mut self) -> [u8; 8] {
        let pkt = core::future::poll_fn(|cx| {
            STATE.ep_wakers[0].register(cx.waker());

            if STATE.setup_ready.load(Ordering::Acquire) {
                STATE.setup_ready.store(false, Ordering::Release);
                let lo = STATE.setup_lo.load(Ordering::Acquire);
                let hi = STATE.setup_hi.load(Ordering::Acquire);
                let pkt = [
                    lo as _,
                    (lo >> 8) as _,
                    (lo >> 16) as _,
                    (lo >> 24) as _,
                    hi as _,
                    (hi >> 8) as _,
                    (hi >> 16) as _,
                    (hi >> 24) as _,
                ];
                Poll::Ready(pkt)
            } else {
                Poll::Pending
            }
        })
        .await;

        debug!(
            "USB: setup [{:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}]",
            pkt[0], pkt[1], pkt[2], pkt[3], pkt[4], pkt[5], pkt[6], pkt[7]
        );
        pkt
    }

    async fn data_out(
        &mut self,
        buf: &mut [u8],
        _first: bool,
        _last: bool,
    ) -> Result<usize, EndpointError> {
        trace!("USB: ctrl data_out waiting (buf={})", buf.len());
        let r = I::regs();

        // Point CFIFO at the DCP (default control pipe) receive buffer
        r.cfifosel().modify(|r| {
            r.set_curpipe(CfifoselCurpipe::Default);
            // OUT direction (rx)
            r.set_isel(false);
        });

        // After the SETUP stage, DCPCTR.PID is NAK. Set BUF so the hardware
        // accepts the host's DATA OUT token; without this BRDY never fires.
        r.dcpctr().modify(|r| r.set_pid(DcpctrPid::Buffer));

        // Wait for DCP BRDY (host sent OUT data)
        core::future::poll_fn(|cx| {
            STATE.ep_wakers[0].register(cx.waker());
            if STATE.pipe_brdy.load(Ordering::Acquire) & 1 != 0 {
                STATE.pipe_brdy.fetch_and(!1, Ordering::AcqRel);
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        // Wait for FIFO to be ready
        while !r.cfifoctr().read().frdy() {
            cortex_m::asm::nop();
        }

        let data_len = r.cfifoctr().read().dtln() as usize;
        let read_len = data_len.min(buf.len());
        if data_len > read_len {
            // Overflow, clear buffer and signal
            r.cfifoctr().modify(|r| r.set_bclr(true));
            return Err(EndpointError::BufferOverflow);
        }

        for byte in buf[..read_len].iter_mut() {
            *byte = r.cfifol().read().fifoport();
        }

        // Discard remaining
        if data_len > read_len {
            r.cfifoctr().modify(|r| r.set_bclr(true));
        }

        trace!("USB: ctrl data_out read {} bytes", read_len);
        Ok(read_len)
    }

    async fn data_in(
        &mut self,
        data: &[u8],
        _first: bool,
        last: bool,
    ) -> Result<(), EndpointError> {
        trace!("USB: ctrl data_in {} bytes, last={}", data.len(), last);
        let r = I::regs();

        if data.is_empty() && last {
            // Zero-length status response: PID=Buffer + CCPL
            r.dcpctr().modify(|r| {
                r.set_pid(DcpctrPid::Buffer);
                r.set_ccpl(true);
            });
            return Ok(());
        }

        /*
         * Discard any stale BEMP that may have been set while the DCP was idle (e.g. the inital
         * empty buffer notification from `start()`).
         */
        STATE.pipe_bemp.fetch_and(!1, Ordering::AcqRel);

        // Select DCP TX direction on CFIFO
        r.cfifosel().modify(|r| {
            r.set_curpipe(CfifoselCurpipe::Default);
            r.set_isel(true);
        });

        // Wait for the TX FIFO to be ready
        while !r.cfifoctr().read().frdy() {
            cortex_m::asm::nop();
        }

        for &byte in data {
            r.cfifol().write(|w| w.set_fifoport(byte));
        }

        // Commit the buffer
        r.cfifoctr().modify(|r| r.set_bval(true));

        // Enable auto-ACK of the host STATUS stage OUT before starting TX
        if last {
            r.dcpctr().modify(|r| r.set_ccpl(true));
        }

        // Start transmission: PID=BUF lets hardware respond to IN tokens.
        r.dcpctr().modify(|r| r.set_pid(DcpctrPid::Buffer));

        core::future::poll_fn(|cx| {
            STATE.ep_wakers[0].register(cx.waker());
            if STATE.pipe_bemp.load(Ordering::Acquire) & 1 != 0 {
                STATE.pipe_bemp.fetch_and(!1, Ordering::AcqRel);
                return Poll::Ready(());
            } else {
                Poll::Pending
            }
        })
        .await;

        Ok(())
    }

    async fn accept(&mut self) {
        debug!("USB: ctrl accept");
        let r = I::regs();
        r.dcpctr().modify(|r| {
            r.set_pid(DcpctrPid::Buffer);
            /*
             * Page 634: When CCPL is set to 1 while the associated PID is set to BUF, the USBFS
             * completes the control transfer status stage.
             */
            r.set_ccpl(true);
        })
    }

    async fn reject(&mut self) {
        debug!("USB: ctrl reject");
        let r = I::regs();
        r.dcpctr().modify(|r| r.set_pid(DcpctrPid::Stall2));
    }

    async fn accept_set_address(&mut self, addr: u8) {
        debug!("USB: ctrl accept_set_address={}", addr);
        /*
         * Page 634: On detecting a SET_ADDRESS request, the USBFS operates in auto-response mode
         * from the setup stage up to status stage completion regardless of the CCPL bit.
         */
        self.accept().await;
    }
}
