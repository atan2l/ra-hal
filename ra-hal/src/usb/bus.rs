use crate::usb::{EP_ADDR, Instance as UsbInstance, STATE};
use core::{
    marker::PhantomData,
    ops::{BitAnd, Not},
    sync::atomic::Ordering,
    task::Poll,
};
use embassy_time::Timer;
use embassy_usb_driver::{Bus as DriverBus, EndpointAddress, Event, Unsupported};
use ra_metapac::usbfs::vals::{DcpctrPid, PipectrPid};

// Bus event flags
pub(crate) enum BusEvent {
    PowerDetected = 1 << 0,
    PowerRemoved = 1 << 1,
    Reset = 1 << 2,
    Suspend = 1 << 3,
    Resume = 1 << 4,
}

pub struct Bus<'a, I: UsbInstance> {
    pub(crate) _phantom: PhantomData<&'a I>,
}

impl BitAnd<BusEvent> for u32 {
    type Output = bool;

    fn bitand(self, rhs: BusEvent) -> Self::Output {
        self & rhs as u32 != 0
    }
}

impl Not for BusEvent {
    type Output = u32;

    fn not(self) -> Self::Output {
        !(self as u32)
    }
}

impl<'a, I: UsbInstance> DriverBus for Bus<'a, I> {
    async fn enable(&mut self) {
        let r = I::regs();
        info!("USB: bus enable D+ pullup");
        r.syscfg().modify(|r| r.set_dprpu(true));
        // Per the manual `LNST[1:0]` bits must be read after connection processing SYSCFG.DPRPU = 1
        let lnst = r.syssts0().read().lnst();
        let syscfg = r.syscfg().read();
        info!("USB: SYSCFG={:?}, LNST={:02b}", syscfg, lnst);
    }

    async fn disable(&mut self) {
        let r = I::regs();
        info!("USB: disable");
        r.syscfg().modify(|r| r.set_dprpu(false));
        r.syscfg().modify(|r| r.set_usbe(false));
        // Call I::stop_module()? Maybe?
    }

    async fn poll(&mut self) -> Event {
        core::future::poll_fn(|cx| {
            STATE.bus_waker.register(cx.waker());

            let bits = STATE.pending_bus.load(Ordering::Acquire);
            // No bus events to process
            if bits == 0 {
                return Poll::Pending;
            }

            /*
             * Note that PowerDetected must come before Reset, so embassy-usb sees them in the
             * right order. The bus won't be enabled until PowerDetected is set.
             */
            if bits & BusEvent::PowerDetected {
                STATE
                    .pending_bus
                    .fetch_and(!BusEvent::PowerDetected, Ordering::AcqRel);
                debug!("USB: poll -> power detected");
                return Poll::Ready(Event::PowerDetected);
            }

            if bits & BusEvent::PowerRemoved {
                STATE
                    .pending_bus
                    .fetch_and(!BusEvent::PowerRemoved, Ordering::AcqRel);
                debug!("USB: poll -> power removed");
                return Poll::Ready(Event::PowerRemoved);
            }

            if bits & BusEvent::Reset {
                STATE
                    .pending_bus
                    .fetch_and(!BusEvent::Reset, Ordering::AcqRel);
                debug!("USB: poll -> reset");
                return Poll::Ready(Event::Reset);
            }

            if bits & BusEvent::Suspend {
                STATE
                    .pending_bus
                    .fetch_and(!BusEvent::Suspend, Ordering::AcqRel);
                debug!("USB: poll -> suspend");
                return Poll::Ready(Event::Suspend);
            }

            if bits & BusEvent::Resume {
                STATE
                    .pending_bus
                    .fetch_and(!BusEvent::Resume, Ordering::AcqRel);
                debug!("USB: poll -> resume");
                return Poll::Ready(Event::Resume);
            }

            Poll::Pending
        })
        .await
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        // The control endpoint is handled by the control pipe
        if ep_addr.index() == 0 {
            return;
        }

        let r = I::regs();
        if let Some(pipe) = pipe_for_ep(ep_addr) {
            /*
             * IMPORTANT: The PAC exposes the PIPEnCTR registers as 0-index, while the manual lists
             * them as 1-index. Much time has been lost here because the assertion wasn't hit :(
             */
            let pipenctr_idx = pipe - 1;
            if enabled {
                // Enable per-pipe interrupt sources
                r.brdyenb().modify(|r| r.set_brdy(pipe, true));
                r.bempenb().modify(|r| r.set_bemp(pipe, true));
                // Enable, so set to BUF
                r.pipectr(pipenctr_idx)
                    .modify(|r| r.set_pid(PipectrPid::Buffer));
                STATE.pipe_enabled.fetch_or(1 << pipe, Ordering::Release);
                // For IN pipes: seed BEMP so the first write() doesn't block
                // waiting for a "previous packet sent" signal that never arrived.
                if ep_addr.is_in() {
                    STATE.pipe_bemp.fetch_or(1 << pipe, Ordering::Release);
                }
            } else {
                r.pipectr(pipenctr_idx)
                    .modify(|r| r.set_pid(PipectrPid::Nak));
                STATE
                    .pipe_enabled
                    .fetch_and(!(1 << pipe), Ordering::Release);
                r.brdyenb().modify(|r| r.set_brdy(pipe, false));
                r.bempenb().modify(|r| r.set_bemp(pipe, false));
            }
            STATE.ep_wakers[pipe].wake();
        }
    }

    fn endpoint_set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        let r = I::regs();

        if ep_addr.index() == 0 {
            // Control pipe
            // Note that technically we should be checking DCPCTR.PBUSY
            r.dcpctr().modify(|r| {
                r.set_pid(if stalled {
                    DcpctrPid::Stall2
                } else {
                    DcpctrPid::Nak
                });
                if !stalled {
                    r.set_sqclr(true);
                }
            });
            return;
        }

        if let Some(pipe) = pipe_for_ep(ep_addr) {
            let pipectr_idx = pipe - 1;
            r.pipectr(pipectr_idx).modify(|r| {
                r.set_pid(if stalled {
                    PipectrPid::Stall2
                } else {
                    PipectrPid::Nak
                });
                if !stalled {
                    r.set_sqclr(true);
                }
            });
        }
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        let r = I::regs();

        if ep_addr.index() == 0 {
            let pid = r.dcpctr().read().pid();
            return matches!(pid, DcpctrPid::Stall2 | DcpctrPid::Stall3);
        }

        if let Some(pipe) = pipe_for_ep(ep_addr) {
            let pipectr_idx = pipe - 1;
            let pid = r.pipectr(pipectr_idx).read().pid();
            return matches!(pid, PipectrPid::Stall2 | PipectrPid::Stall3);
        }
        false
    }

    #[cfg(feature = "time-driver")]
    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        let r = I::regs();
        r.dvstctr0().modify(|r| {
            r.set_rwupe(true);
            r.set_wkup(true);
        });
        // Hold WKUP for >= 1 ms as per USB spec
        Timer::after_millis(2).await;
        r.dvstctr0().modify(|r| r.set_wkup(false));
        Ok(())
    }

    #[cfg(not(feature = "time-driver"))]
    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        Err(Unsupported)
    }
}

/// Find the pipe number for a given endpoint address by scanning the static
/// mapping table.  Returns `None` if not found.
fn pipe_for_ep(ep_addr: EndpointAddress) -> Option<usize> {
    let target: u8 = ep_addr.into();
    for (i, ep) in EP_ADDR.iter().enumerate() {
        if ep.load(Ordering::Acquire) == target {
            /*
             * i + 1 because EP_ADDR doesn't store the address of EP0 (control).
             * So i = 0, pipe = 1; i = 1, pipe = 2; etc.
             */
            return Some(i + 1);
        }
    }
    None
}
