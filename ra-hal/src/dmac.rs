//! Direct Memory Access Controller (`DMAC`).
//!
//! `DMAC` is one of two DMA engines on the `RA4M1`.
//!
//! # Notes
//! - There are four (4) channels, and as the API currently stands one interrupt is required to signal the end of a transfer.
//! - Configuration is performed directly on `DMAC` registers and typically results in lower latency than with `DTC`.
//! - Operation may be less predictable than `DTC`.

use core::{marker::PhantomData, task::Poll};

use embassy_hal_internal::{Peri, PeripheralType, interrupt::InterruptExt as _};
use embassy_sync::waitqueue::AtomicWaker;

use crate::{
    event_link::{IcuInterrupt as _, InterruptEvent},
    interrupt::typelevel::{Handler as InterruptHandler, Interrupt as InterruptType},
    pac::{
        self,
        dmac::vals::{Dctg, Dm, Dts, Md, Sm, Sz},
    },
};

/// Interrupt handler that handles `DMAC` interrupts.
pub struct DmacInterruptHandler<C: Instance> {
    phantom: PhantomData<C>,
}

/// `DMAC` instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + Into<AnyChannel> {}

pub(crate) trait SealedInstance: PeripheralType {
    const INTERRUPT_EVENT: InterruptEvent;
    const DMAC_INDEX: u8;
    fn regs() -> pac::dmac::Dmac;
}

/// Trait that defines the size of a `DMAC` "word".
///
/// `DMAC` supports 8-, 16-, and 32- bit words.
#[allow(private_bounds)]
pub trait Word: SealedWord {}
trait SealedWord: Copy {
    const DMA_WIDTH: Sz;
}

/// Type erased `DMAC` channel.
#[derive(Debug, Clone, Copy)]
pub struct AnyChannel {
    pub(crate) dmac_index: u8,
    pub(crate) interrupt_event: InterruptEvent,
}

impl<C: Instance, Int: InterruptType> InterruptHandler<Int> for DmacInterruptHandler<C> {
    unsafe fn on_interrupt() {
        Int::IRQ.icu_unpend();
        DMAC_WAKERS[usize::from(C::DMAC_INDEX)].wake();
    }
}

impl PeripheralType for AnyChannel {}

const DMAC_COUNT: usize = 4;
static DMAC_WAKERS: [AtomicWaker; DMAC_COUNT] = [const { AtomicWaker::new() }; DMAC_COUNT];

/// `DMAC` channel.
pub struct Channel<'d> {
    pub(crate) channel: Peri<'d, AnyChannel>,
}

impl AnyChannel {
    #[inline(always)]
    fn regs(&self) -> pac::dmac::Dmac {
        let dmac_ptr = pac::DMAC0.as_ptr() as usize;
        let offset = 0x40 * usize::from(self.dmac_index);
        unsafe { crate::pac::dmac::Dmac::from_ptr(dmac_ptr.wrapping_add(offset) as _) }
    }
}

/// Future representing a `DMAC` transfer.
pub struct Transfer<'d> {
    channel: Channel<'d>,
}

impl<'d> Channel<'d> {
    /// Creates a new `DMAC` driver for a specific channel.
    pub fn new<C: Instance, Int: InterruptType>(
        channel: Peri<'d, C>,
        irq: impl crate::interrupt::typelevel::Binding<Int, DmacInterruptHandler<C>> + 'd,
    ) -> Self {
        let _ = irq;

        // Safety: Interrupt handlers are defined by the irqs argument and thus the interrupts are safe to enable.
        unsafe {
            Int::IRQ.enable();
            Int::IRQ.icu_enable(C::INTERRUPT_EVENT);
        }

        trace!("DMAC{}, enable={}", C::DMAC_INDEX, C::INTERRUPT_EVENT);

        let dmac = C::regs();
        dmac.dmint().modify(|r| {
            r.set_rptie(true);
            r.set_dtie(true);
        });

        Self {
            channel: channel.into(),
        }
    }

    /// Reborrow the DMA Channel with a shorter lifetime.
    #[inline]
    pub fn reborrow(&mut self) -> Channel<'_> {
        Channel {
            channel: self.channel.reborrow(),
        }
    }

    /// Configures a `DMAC` transfer that reads from an address incremented by the specified [`InterruptEvent`] and writes to a fixed address.
    /// e.g. memory-to-peripheral.
    ///
    /// # Arguments
    /// - `src`
    /// - `dest`
    /// - `event`
    ///
    /// # Returns
    /// A future that will finish upon completion of the transfer.
    pub fn write<W: Word>(
        &mut self,
        src: &[W],
        dest: *mut W,
        event: InterruptEvent,
    ) -> Transfer<'_> {
        let dmac = self.channel.regs();
        let icu = pac::ICU;

        dmac.dmamd().write(|r| {
            r.set_sm(Sm::Increment);
            r.set_dm(Dm::Fixed);
        });
        dmac.dmtmd().write(|r| {
            r.set_sz(W::DMA_WIDTH);
            r.set_md(Md::Normal);
            r.set_dts(Dts::RepeatSource);
            r.set_dctg(Dctg::Interrupts);
        });

        icu.delsr(self.channel.dmac_index as _).write(|r| {
            r.set_dels(event as u16);
        });
        dmac.dmsar().write_value(src.as_ptr() as u32);
        dmac.dmdar().write_value(dest as u32);
        assert!(src.len() < usize::from(u16::MAX));
        dmac.dmcra().write(|r| {
            r.set_dmcrah(src.len() as u16);
            r.set_dmcral(src.len() as u16);
        });

        dmac.dmcnt().modify(|r| r.set_dte(true));

        Transfer {
            channel: self.reborrow(),
        }
    }

    /// Configures a `DMAC` transfer that reads from a fixed address and writes to an address incremented by the specified [`InterruptEvent`].
    /// e.g. peripheral-to-memory.
    ///
    /// # Arguments
    /// - `src`
    /// - `dest`
    /// - `event`
    ///
    /// # Returns
    /// A future that will finish upon completion of the transfer.
    pub fn read<W: Word>(
        &mut self,
        src: *const W,
        dest: &mut [W],
        event: InterruptEvent,
    ) -> Transfer<'_> {
        let dmac = self.channel.regs();
        let icu = pac::ICU;

        dmac.dmamd().write(|r| {
            r.set_sm(Sm::Fixed);
            r.set_dm(Dm::Increment);
        });
        dmac.dmtmd().write(|r| {
            r.set_sz(W::DMA_WIDTH);
            r.set_md(Md::Normal);
            r.set_dts(Dts::RepeatDestination);
            r.set_dctg(Dctg::Interrupts);
        });

        icu.delsr(self.channel.dmac_index as _).write(|r| {
            r.set_dels(event as u16);
        });
        dmac.dmsar().write_value(src as u32);
        dmac.dmdar().write_value(dest.as_mut_ptr() as u32);
        assert!(dest.len() < usize::from(u16::MAX));
        dmac.dmcra().write(|r| {
            r.set_dmcrah(dest.len() as u16);
            r.set_dmcral(dest.len() as u16);
        });
        dmac.dmcnt().write(|r| r.set_dte(true));

        Transfer {
            channel: self.reborrow(),
        }
    }
}

impl<'d> Future for Transfer<'d> {
    type Output = ();

    fn poll(
        self: core::pin::Pin<&mut Self>,
        ctx: &mut core::task::Context<'_>,
    ) -> Poll<Self::Output> {
        DMAC_WAKERS[usize::from(self.channel.channel.dmac_index)].register(ctx.waker());
        let dmac = self.channel.channel.regs();
        if dmac.dmsts().read().dtif() {
            dmac.dmcnt().write(|r| r.set_dte(false));

            #[cfg(feature = "cache")]
            {
                let fcache = pac::FCACHE;
                fcache.fcacheiv().write(|r| r.set_fcacheiv(true));

                while fcache.fcacheiv().read().fcacheiv() {}
            }

            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl<'d> Drop for Channel<'d> {
    fn drop(&mut self) {
        let icu = pac::ICU;
        let index = self.channel.dmac_index;
        icu.delsr(index as _).write(|r| r.set_dels(0));
    }
}

pub(crate) fn init() {
    let dma = pac::DMA;
    dma.dmast().write(|r| r.set_dmst(true));
}

macro_rules! dmac_chan {
    ($index:literal) => {
        paste::paste! {
            impl crate::dmac::Instance for crate::peripherals::[< DMAC $index >] {}

            impl crate::dmac::SealedInstance for crate::peripherals::[< DMAC $index >] {
                const INTERRUPT_EVENT: InterruptEvent = crate::event_link::InterruptEvent::[< Dmac $index Int >];
                const DMAC_INDEX: u8 = $index;

                #[inline(always)]
                fn regs() -> crate::pac::dmac::Dmac {
                    crate::pac::[< DMAC $index >]
                }
            }

            impl From<crate::peripherals::[< DMAC $index >]> for AnyChannel {
                fn from(_: crate::peripherals::[< DMAC $index >]) -> Self {
                    AnyChannel {
                        dmac_index: $index,
                        interrupt_event: crate::event_link::InterruptEvent::[< Dmac $index Int >],
                    }
                }
            }

        }
    };
}

dmac_chan!(0);
dmac_chan!(1);
dmac_chan!(2);
dmac_chan!(3);

impl Word for u8 {}
impl SealedWord for u8 {
    const DMA_WIDTH: Sz = Sz::_8bits;
}

impl Word for u16 {}
impl SealedWord for u16 {
    const DMA_WIDTH: Sz = Sz::_16bits;
}

impl Word for u32 {}
impl SealedWord for u32 {
    const DMA_WIDTH: Sz = Sz::_32bits;
}
