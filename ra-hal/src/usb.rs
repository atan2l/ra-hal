//! USB Full-Speed 2.0 peripheral.

use core::{
    marker::PhantomData,
    sync::atomic::{AtomicBool, AtomicU8, AtomicU32, Ordering},
};

use embassy_hal_internal::{Peri, PeripheralType, interrupt::InterruptExt};
use embassy_sync::waitqueue::AtomicWaker;
use embassy_usb_driver::{
    Direction, Driver, EndpointAddress, EndpointAllocError, EndpointInfo, EndpointType,
};

use crate::{
    event_link::{IcuInterrupt, InterruptEvent},
    gpio::{Pin, PortFunction},
    interrupt::typelevel::{Binding, Handler as InterruptHandler, Interrupt as InterruptType},
    module_stop::ModuleStop,
    pac::{
        self,
        usbfs::vals::{Dvsq, Pipesel, Type},
    },
    peripherals::USBFS,
    usb::{
        bus::{Bus, BusEvent},
        control_pipe::ControlPipe,
        endpoint::{EndpointIn, EndpointOut},
    },
};

pub mod bus;
pub mod control_pipe;
pub mod endpoint;

#[derive(Clone, Copy)]
struct PipeConfig {
    ep_addr: EndpointAddress,
    ep_type: EndpointType,
    max_packet: u16,
    interval_ms: u8,
}
struct State {
    /// The bus waker
    bus_waker: AtomicWaker,
    /// The various endpoint wakers, indexed by pipe number (0 = control, 1-9 = data pipes)
    ep_wakers: [AtomicWaker; 10],
    /// Bus-level events set by ISR, consumed by Bus::poll.
    pending_bus: AtomicU32,
    /// BRDY flags set by ISR per pipe (bit N = pipe N has data ready).
    pipe_brdy: AtomicU32,
    /// BEMP flags set by ISR per pipe (bit N = pipe N transmit buffer empty).
    pipe_bemp: AtomicU32,
    /// Whether a valid SETUP packet is waiting to be read.
    setup_ready: AtomicBool,
    /// Cached setup packet set by ISR (8 bytes stored as two u32s, LE).
    setup_lo: AtomicU32,
    setup_hi: AtomicU32,
    /// Bit mask of pipes that have been enabled via endpoint_set_enabled.
    pipe_enabled: AtomicU32,
}

/// Interrupt handler for `USBFS` interrupts.
pub struct UsbInterruptHandler<I: Instance> {
    _phantom: PhantomData<I>,
}

/// `embassy-usb-driver` implementation for the USBFS peripheral.
pub struct Usbfs<'a, I: Instance> {
    _phantom: PhantomData<&'a I>,
    pipes: [Option<PipeConfig>; 9],
    next_ep_in: u8,
    next_ep_out: u8,
}

impl State {
    const fn new() -> Self {
        Self {
            bus_waker: AtomicWaker::new(),
            ep_wakers: [const { AtomicWaker::new() }; 10],
            pending_bus: AtomicU32::new(0),
            pipe_brdy: AtomicU32::new(0),
            pipe_bemp: AtomicU32::new(0),
            setup_ready: AtomicBool::new(false),
            setup_lo: AtomicU32::new(0),
            setup_hi: AtomicU32::new(0),
            pipe_enabled: AtomicU32::new(0),
        }
    }
}

static STATE: State = State::new();

/// Per-pipe endpoint address table populated during `start()`.
/// Index 0 = pipe 1, index 8 = pipe 9. 0xFF means unallocated.
static EP_ADDR: [AtomicU8; 9] = [const { AtomicU8::new(0xFF) }; 9];

/// USB device instance
#[allow(private_bounds)]
pub trait Instance: SealedInstance + ModuleStop + PeripheralType + 'static + Send {}

pub(crate) trait SealedInstance {
    fn regs() -> pac::usbfs::Usbfs;
}

pub(crate) trait SealedDpPin<I: SealedInstance>: Pin + PeripheralType {
    // const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_pfunc(&self) {
        /*
         * Technically, we should probably set the peripheral function for the pin; however,
         * Table 19.17 shows that the pins used for DP and DM don't care about the `PSEL` settings.
         * Furthermore, if we set a value for pins P914 and P915, the readback fails as they read
         * as HiZ (0x00).
         *
         * Leaving this as a vestigial remnant for reference in other chips.
         */
        //self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

/// A pin that can be used for the differential negative input.
#[allow(private_bounds)]
pub trait DpPin<I: Instance>: SealedDpPin<I> {}

#[allow(unused)]
macro_rules! dp_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::usb::DpPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::usb::SealedDpPin<crate::peripherals::$instance> for crate::peripherals::$pin {
            // const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}

#[cfg(not(ra6m5))]
pub(crate) use dp_pin;

pub(crate) trait SealedDmPin<I: SealedInstance>: Pin + PeripheralType {
    // const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_pfunc(&self) {
        /*
         * Technically, we should probably set the peripheral function for the pin; however,
         * Table 19.17 shows that the pins used for DP and DM don't care about the `PSEL` settings.
         * Furthermore, if we set a value for pins P914 and P915, the readback fails as they read
         * as HiZ (0x00).
         *
         * Leaving this as a vestigial remnant for reference in other chips.
         */
        //self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

/// A pin that can be used for the differential positive input.
#[allow(private_bounds)]
pub trait DmPin<I: Instance>: SealedDmPin<I> {}

#[allow(unused)]
macro_rules! dm_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::usb::DmPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::usb::SealedDmPin<crate::peripherals::$instance> for crate::peripherals::$pin {
            // const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}

#[cfg(not(ra6m5))]
pub(crate) use dm_pin;

pub(crate) trait SealedVbusPin<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: PortFunction;

    #[inline(always)]
    fn set_pfunc(&self) {
        self.set_as_pf(Self::PERIPHERAL_FUNC);
    }
}

/// A pin that can be used for VBUS detection.
#[allow(private_bounds)]
pub trait VbusPin<I: Instance>: SealedVbusPin<I> {}

macro_rules! vbus_pin {
    ($instance:ident, $pin:ident, $pfunc:ident) => {
        impl crate::usb::VbusPin<crate::peripherals::$instance> for crate::peripherals::$pin {}
        impl crate::usb::SealedVbusPin<crate::peripherals::$instance> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}

pub(crate) use vbus_pin;

impl SealedInstance for USBFS {
    fn regs() -> pac::usbfs::Usbfs {
        pac::USBFS
    }
}

impl Instance for USBFS {}

impl<I: Instance, Int: InterruptType> InterruptHandler<Int> for UsbInterruptHandler<I> {
    unsafe fn on_interrupt() {
        let r = I::regs();
        Int::IRQ.icu_unpend();

        let sts = r.intsts0().read();
        trace!(
            "USBFS IRQ: vbint={} dvst={} resm={} ctrt={} valid={} brdy={} bemp={}",
            sts.vbint(),
            sts.dvst(),
            sts.resm(),
            sts.ctrt(),
            sts.valid(),
            sts.brdy(),
            sts.bemp(),
        );

        // VBUS change
        if sts.vbint() {
            r.intsts0().modify(|r| r.set_vbint(false));
            if sts.vbsts() {
                debug!("USBFS IRQ: VBUS detected");
                STATE
                    .pending_bus
                    .fetch_or(BusEvent::PowerDetected as _, Ordering::Release);
            } else {
                debug!("USBFS IRQ: VBUS removed");
                STATE
                    .pending_bus
                    .fetch_or(BusEvent::PowerRemoved as _, Ordering::Release);
            }
            STATE.bus_waker.wake();
        }

        // Device state transition (reset / suspend / configured)
        if sts.dvst() {
            r.intsts0().modify(|r| r.set_dvst(false));
            match sts.dvsq() {
                Dvsq::Default => {
                    debug!("USBFS IRQ: bus reset (DVSQ=Default)");
                    STATE
                        .pending_bus
                        .fetch_or(BusEvent::Reset as _, Ordering::Release);
                    STATE.bus_waker.wake();
                }
                Dvsq::Suspend4 | Dvsq::Suspend5 | Dvsq::Suspend6 | Dvsq::Suspend7 => {
                    debug!("USBFS IRQ: suspend (DVSQ={:04b})", sts.dvsq().to_bits());
                    STATE
                        .pending_bus
                        .fetch_or(BusEvent::Suspend as _, Ordering::Release);
                    STATE.bus_waker.wake();
                }
                other => {
                    trace!("USBFS IRQ: DVST dvsq={:04b}", other.to_bits());
                }
            }
        }

        // Resume
        if sts.resm() {
            r.intsts0().modify(|r| r.set_resm(false));
            debug!("USBFS IRQ: resume");
            STATE
                .pending_bus
                .fetch_or(BusEvent::Resume as _, Ordering::Release);
            STATE.bus_waker.wake();
        }

        // Control transfer stage transition
        if sts.ctrt() {
            r.intsts0().modify(|r| r.set_ctrt(false));
            if sts.valid() {
                // Latch setup packet before clearing VALID
                let req = r.usbreq().read();
                let val = r.usbval().read();
                let idx = r.usbindx().read();
                let len = r.usbleng().read();

                let lo = u32::from(req.bmrequesttype())
                    | (u32::from(req.brequest()) << 8)
                    | ((val.wvalue() as u32) << 16);
                let hi = idx.windex() as u32 | ((len.wlentuh() as u32) << 16);

                debug!(
                    "USBFS IRQ: SETUP bmrt={:08b} req={:#04x} val={:#06x} idx={:#06x} len={}",
                    req.bmrequesttype(),
                    req.brequest(),
                    val.wvalue(),
                    idx.windex(),
                    len.wlentuh(),
                );

                STATE.setup_lo.store(lo, Ordering::Relaxed);
                STATE.setup_hi.store(hi, Ordering::Relaxed);
                STATE.setup_ready.store(true, Ordering::Release);
                // Clear VALID so hardware can accept the next SETUP
                r.intsts0().modify(|r| r.set_valid(false));
                STATE.ep_wakers[0].wake();
            } else {
                trace!("USBFS IRQ: CTRT without VALID (status stage)");
            }
        }

        // Buffer ready (data received into pipe FIFO)
        if sts.brdy() {
            let brdy = r.brdysts().read();
            let mut mask = 0u32;
            for i in 0..10usize {
                if brdy.brdy(i) {
                    // Clear hardware flag to stop interrupt from re-firing
                    r.brdysts().modify(|r| r.set_brdy(i, false));
                    mask |= 1 << i;
                }
            }
            trace!("USBFS IRQ: BRDY pipes={:010b}", mask);
            STATE.pipe_brdy.fetch_or(mask, Ordering::Release);
            for i in 0..10usize {
                if mask & (1 << i) != 0 {
                    STATE.ep_wakers[i].wake();
                }
            }
        }

        // Buffer empty (pipe transmit FIFO drained / data sent)
        if sts.bemp() {
            let bemp = r.bempsts().read();
            let mut mask = 0u32;
            for i in 0..10usize {
                if bemp.bemp(i) {
                    r.bempsts().modify(|r| r.set_bemp(i, false));
                    mask |= 1 << i;
                }
            }
            trace!("USBFS IRQ: BEMP pipes={:010b}", mask);
            STATE.pipe_bemp.fetch_or(mask, Ordering::Release);
            for i in 0..10usize {
                if mask & (1 << i) != 0 {
                    STATE.ep_wakers[i].wake();
                }
            }
        }
    }
}

// TODO: Come up with a better way to gate the dedicated dp/dm pins.
impl<'a, I: Instance> Usbfs<'a, I> {
    /// Creates a new USB driver.
    pub fn new<
        Int: InterruptType,
        #[cfg(not(ra6m5))] P: DpPin<I>,
        #[cfg(not(ra6m5))] M: DmPin<I>,
        V: VbusPin<I>,
    >(
        _usb: Peri<'a, I>,
        #[cfg(not(ra6m5))] dp: Peri<'a, P>,
        #[cfg(not(ra6m5))] dm: Peri<'a, M>,
        vbus: Peri<'a, V>,
        irqs: impl Binding<Int, UsbInterruptHandler<I>> + 'a,
    ) -> Self {
        let _ = irqs;

        // Manual section 27.4.1: Releasing the module-stop state enables access to the registers.
        I::start_module();

        #[cfg(not(ra6m5))]
        {
            dp.set_pfunc();
            dm.set_pfunc();
        }
        vbus.set_pfunc();

        // Safety: interrupt handler is bound above.
        unsafe {
            Int::IRQ.enable();
            Int::IRQ.icu_enable(InterruptEvent::UsbfsInt);
        }

        info!("USBFS: driver created, interrupt enabled");

        Self {
            _phantom: PhantomData,
            pipes: [None; 9],
            next_ep_in: 1,
            next_ep_out: 1,
        }
    }
}

impl<'a, I: Instance> Driver<'a> for Usbfs<'a, I> {
    type EndpointOut = EndpointOut<'a, I>;
    type EndpointIn = EndpointIn<'a, I>;
    type ControlPipe = ControlPipe<'a, I>;
    type Bus = Bus<'a, I>;

    fn alloc_endpoint_out(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointOut, EndpointAllocError> {
        let pipe = find_free_pipe(&self.pipes, ep_type).ok_or(EndpointAllocError)?;

        let ep_num = if let Some(addr) = ep_addr {
            if addr.is_in() {
                return Err(EndpointAllocError);
            }
            addr.index() as u8
        } else {
            let n = self.next_ep_out;
            self.next_ep_out += 1;
            n
        };

        let addr = EndpointAddress::from_parts(ep_num as usize, Direction::Out);
        self.pipes[pipe - 1] = Some(PipeConfig {
            ep_addr: addr,
            ep_type,
            max_packet: max_packet_size,
            interval_ms,
        });

        Ok(EndpointOut {
            _phantom: PhantomData,
            info: EndpointInfo {
                addr,
                ep_type,
                max_packet_size,
                interval_ms,
            },
            pipe: pipe as u8,
        })
    }

    fn alloc_endpoint_in(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointIn, EndpointAllocError> {
        let pipe = find_free_pipe(&self.pipes, ep_type).ok_or(EndpointAllocError)?;

        let ep_num = if let Some(addr) = ep_addr {
            if addr.is_out() {
                return Err(EndpointAllocError);
            }
            addr.index() as u8
        } else {
            let n = self.next_ep_in;
            self.next_ep_in += 1;
            n
        };

        let addr = EndpointAddress::from_parts(ep_num as usize, Direction::In);
        self.pipes[pipe - 1] = Some(PipeConfig {
            ep_addr: addr,
            ep_type,
            max_packet: max_packet_size,
            interval_ms,
        });

        Ok(EndpointIn {
            _phantom: PhantomData,
            info: EndpointInfo {
                addr,
                ep_type,
                max_packet_size,
                interval_ms,
            },
            pipe: pipe as u8,
        })
    }

    fn start(self, control_max_packet_size: u16) -> (Self::Bus, Self::ControlPipe) {
        let r = I::regs();

        info!("USBFS: start (ctrl max_packet={})", control_max_packet_size);
        /*
         * Section 27.3.1.1: Enable the USB clock gate first (SCKE=1). When SCKE=0, only SYSCFG
         * may be written (Section 27.2.1, `SCKE` bit).
         */
        r.syscfg().modify(|r| r.set_scke(true));

        // Brief spin to let the clock stabilise before touching other USB regs.
        for _ in 0..200 {
            cortex_m::asm::nop();
        }

        // Section 27.2.1 Note 2: Read and confirm SCKE=1
        if !r.syscfg().read().scke() {
            panic!("USBFS: SCKE=0 after SCKE=1 write");
        }

        /*
         * Section 27.2.34: Enable the internal USB LDO regulator (VDCEN=1) before starting the
         * transceiver. Without this, the USB transceiver has no power supply (Section 27.3.1.4,
         * Figure 27.4), and DPRPU has no physical effect (DP stays at SE0 (LNST=00) regardless of
         * SYSCFG settings).
         *
         * Note: This depends on the hardware design (see Figure 27.2). `VCC_USB` and `VCC_USB_LDO`
         * can be connected together to `VCC` when `VCC` is between 3.0V and 3.6V.
         */
        #[cfg(usbfs_4m1)]
        {
            r.usbmc().modify(|r| {
                r.set_vdcen(true);
            });
            debug!("USBFS: USBMC written (VDCEN=1)");
        }

        // Errata note: the PAC exposes `USBFS.UCKSEL`. This register doesn't appear in the manual.

        // Allow the LDO output to stabilise before enabling the transceiver.
        for _ in 0..1000 {
            cortex_m::asm::nop();
        }

        r.syscfg().modify(|r| r.set_usbe(true));
        // Device mode, not host
        r.syscfg().modify(|r| r.set_dcfm(false));
        debug!("USBFS: SYSCFG after init={:?}", r.syscfg().read());

        // Default control pipe max packet size
        r.dcpmaxp()
            .modify(|r| r.set_mxps(control_max_packet_size as u8));

        // Configure allocated data pipes
        for (idx, slot) in self.pipes.iter().enumerate() {
            let Some(cfg) = slot else { continue };
            let pipe = (idx + 1) as u8;

            info!(
                "USBFS: pipe {} epnum={} dir={} type={:02b} mxps={}",
                pipe,
                cfg.ep_addr.index(),
                cfg.ep_addr.is_in(),
                pipe_type(cfg.ep_type).to_bits(),
                cfg.max_packet,
            );

            // Select the pipe to configure via the windowed registers
            r.pipesel().modify(|r| {
                r.set_pipesel(Pipesel::from_bits(pipe));
            });

            let is_in = cfg.ep_addr.is_in();
            let is_bulk = cfg.ep_type == EndpointType::Bulk;
            let is_double_buffered = is_bulk && pipe <= 5; // double-buffer for bulk pipes

            r.pipecfg().modify(|r| {
                r.set_epnum(cfg.ep_addr.index() as u8);
                r.set_dir(is_in); // true = IN (device -> host)
                r.set_type_(pipe_type(cfg.ep_type));
                r.set_dblb(is_double_buffered);
                // SHTNAK: auto-NAK after transfer for OUT bulk pipes
                r.set_shtnak(!is_in && is_bulk);
            });

            r.pipemaxp().modify(|r| r.set_mxps(cfg.max_packet));

            if cfg.ep_type == EndpointType::Interrupt {
                // interval_ms is a power-of-two exponent for the RA4M1 interval field
                let iitv = cfg.interval_ms.trailing_zeros().min(7) as u8;
                r.pipeperi().modify(|r| r.set_iitv(iitv));
            }

            // Reset sequence toggle and clear FIFO
            r.pipectr(idx).modify(|r| {
                r.set_sqclr(true);
                r.set_aclrm(true);
            });
            r.pipectr(idx).modify(|r| r.set_aclrm(false));

            // Store ep_addr to pipe mapping for Bus methods
            EP_ADDR[idx].store(u8::from(cfg.ep_addr), Ordering::Release);
        }

        // Enable global interrupt sources
        r.intenb0().modify(|r| {
            r.set_brdye(true);
            r.set_bempe(true);
            r.set_ctre(true);
            r.set_dvse(true);
            r.set_rsme(true);
            r.set_vbse(true);
        });

        // Enable BEMP for pipe 0 (DCP) so the control pipe write path works
        r.bempenb().modify(|r| r.set_bemp(0, true));
        // Enable BRDY for pipe 0 so the control pipe read path works
        r.brdyenb().modify(|r| r.set_brdy(0, true));

        /*
         * Section 27.4.3: clear INTSTS0 and INTSTS1 after port setup to avoid spurious interrupts
         * from pin state changes during initialisation.
         */
        r.intsts0().write_value(Default::default());
        r.intsts1().write_value(Default::default());

        /*
         * Section 27.3.3.7 If the USB peripheral is already powered, the VBUS signal will be present.
         * Pretend that a PowerDetected event has occurred since VBINT only fires on transitions.
         * Read VBSTS three times and confirm a consistent value.
         */
        let vbsts = {
            let a = r.intsts0().read().vbsts();
            let b = r.intsts0().read().vbsts();
            let c = r.intsts0().read().vbsts();
            // Use majority vote; if two of the three agree, we have a stable reading.
            (c || a) && b || (a && c)
        };
        debug!("USBFS: VBSTS={}", vbsts);
        if vbsts {
            info!("USBFS: VBUS already present, synthesizing PowerDetected");
            STATE
                .pending_bus
                .fetch_or(BusEvent::PowerDetected as _, Ordering::Release);
        }

        let max_packet_size = control_max_packet_size;
        (
            Bus {
                _phantom: PhantomData,
            },
            ControlPipe {
                _phantom: PhantomData,
                max_packet_size,
            },
        )
    }
}

/// Return the pipe number (1-9) suitable for the requested transfer type, or
/// `None` if no compatible free pipe is available.
fn find_free_pipe(pipes: &[Option<PipeConfig>; 9], ep_type: EndpointType) -> Option<usize> {
    for (i, cfg) in pipes.iter().enumerate() {
        if cfg.is_some() {
            continue;
        }
        let pipe = i + 1; // pipe numbers 1-9
        let ok = match ep_type {
            EndpointType::Bulk => pipe <= 5,
            EndpointType::Interrupt => pipe >= 6,
            EndpointType::Isochronous => pipe <= 2,
            EndpointType::Control => false,
        };
        if ok {
            return Some(pipe);
        }
    }
    None
}

// Because of orphan rules
fn pipe_type(ep_type: EndpointType) -> Type {
    match ep_type {
        EndpointType::Bulk => Type::_01,
        EndpointType::Interrupt => Type::_10,
        EndpointType::Isochronous => Type::_11,
        EndpointType::Control => Type::_00,
    }
}
