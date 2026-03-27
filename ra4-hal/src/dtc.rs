//! Data Transfer Controller (`DTC`).
//!
//! `DTC` is one of two DMA engines on the `RA4M1`.
//!
//! # TODO
//! - Store channels as type erased structs like `DMAC`?
//!
//! # Notes
//! - There are no fixed channels and `DTC` transfers can can occupy any interrupt (`IEL0..=IEL31`).
//! - Transfer configuration is stored in SRAM and copied to internal registers by the `DTC` peripheral at the start of each transfer.

use core::{marker::PhantomData, task::Poll};

use bitbybit::{bitenum, bitfield};
use cortex_m::interrupt::InterruptNumber;
use embassy_hal_internal::{Peri, PeripheralType, interrupt::InterruptExt as _};
use embassy_sync::waitqueue::AtomicWaker;

use crate::{
    event_link::{IcuInterrupt as _, InterruptEvent},
    interrupt::{
        self,
        typelevel::{Handler as InterruptHandler, Interrupt as InterruptType},
    },
    module_stop::ModuleStop as _,
    pac,
};

// Access to this is generally rationalized as safe as we're only ever indexing into it
// via the const associated with the instance.
static mut DTC_VECTOR_TABLE: VectorTable = VectorTable::new();

cfg_if::cfg_if! {
    if #[cfg(feature = "defmt")] {
        /// Unit of measure for a DTC transfer.
        #[allow(private_bounds)]
        pub trait Word: SealedWord + defmt::Format {}
    } else {
        /// Unit of measure for a DTC transfer.
        #[allow(private_bounds)]
        pub trait Word: SealedWord  {}
    }
}

trait SealedWord: Copy {
    const WORD_SIZE: WordSize;
}

/// 8-bit (byte) DMA word.
impl Word for u8 {}
impl SealedWord for u8 {
    const WORD_SIZE: WordSize = WordSize::Byte;
}

/// 16-bit (half-word) DMA word.
impl Word for u16 {}
impl SealedWord for u16 {
    const WORD_SIZE: WordSize = WordSize::HalfWord;
}

/// 32-bit (word) DMA word.
impl Word for u32 {}
impl SealedWord for u32 {
    const WORD_SIZE: WordSize = WordSize::Word;
}

/// Interrupt handler for a `DTC` transfer.
///
/// The interrupt number determines the priority of the transfer. §17.7.
pub struct DtcInterruptHandler<C: Instance> {
    phantom: PhantomData<C>,
}

/// `DTC` transfer channel.
///
/// `DTC` has no channels per se, instead transfers are bound to interrupts which are then bound to specific events.
pub struct Channel<C: Instance> {
    entry: DtcEntry,
    phantom: PhantomData<C>,
}

/// How to modify a source or target address upon completion of a `DTC` transfer.
#[bitenum(u2)]
#[allow(dead_code)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum AddressMode {
    /// No write-back to the source or target address register is performed.
    Fixed = 0b00,

    /// The address (source or target) is incremented by the size of the [`Word`] upon completion of a transfer.
    Increment = 0b10,

    /// The address (source or target) is decremented by the size of the [`Word`] upon completion of a transfer.
    Decrement = 0b11,
}

/// Which address is modified upon completion of a transfer.
#[bitenum(u1, exhaustive = true)]
#[allow(dead_code)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum RepeatMode {
    /// The destination address is updated after a transfer.
    Destination = 0,

    /// The source address is updated after a transfer.
    Source = 1,
}

/// When to pass interrupt through to the CPU.
///
/// # Notes
/// The flowchart in Figure 17.4 explains how things ought to work.
#[bitenum(u1, exhaustive = true)]
#[allow(dead_code)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum InterruptMode {
    OnCompletion = 0,
    PerTransfer = 1,
}

/// CHNE + CHNS
#[bitenum(u2)]
#[allow(dead_code)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum ChainMode {
    Disabled = 0b00,
    Continuous = 0b01,
    Penultimate = 0b11,
}

/// SZ
#[bitenum(u2)]
#[allow(dead_code)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum WordSize {
    /// 8 bits
    Byte = 0b00,

    /// 16 bits
    HalfWord = 0b01,

    /// 32 bits
    Word = 0b10,
}

/// MD
#[bitenum(u2)]
#[allow(dead_code)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum TransferMode {
    Normal = 0b00,
    Repeat = 0b01,
    Block = 0b10,
}

#[bitfield(u128, default = 0)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct DtcEntry {
    #[bits(0..=15)]
    reserved: u16,

    // MRB
    #[bits(16..=17)]
    reserved: u2,
    /// DM
    #[bits(18..=19, rw)]
    dest_address_mode: Option<AddressMode>,
    /// DTS
    #[bit(20, rw)]
    repeat_mode: RepeatMode,
    /// DISEL
    #[bit(21, rw)]
    interrupt_mode: InterruptMode,
    /// CHNS+CHNE
    #[bits(22..=23, rw)]
    chain_mode: Option<ChainMode>,

    // MRA
    #[bits(24..=25)]
    reserved: u2,
    /// SM
    #[bits(26..=27, rw)]
    source_address_mode: Option<AddressMode>,
    /// SZ
    #[bits(28..=29, rw)]
    word_size: Option<WordSize>,
    /// MD
    #[bits(30..=31, rw)]
    transfer_mode: Option<TransferMode>,

    // SAR
    #[bits(32..=63, rw)]
    source_address: u32,

    // DAR
    #[bits(64..=95, rw)]
    dest_address: u32,

    // CRB
    #[bits(96..=111, rw)]
    count_b: u16,

    // CRA
    #[bits(112..=119, rw)]
    count_a_low: u8,
    #[bits(120..=127, rw)]
    count_a_high: u8,
}

#[repr(C, align(1024))]
#[derive(Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
struct VectorTable {
    vectors: [u32; 32],
}

/// `DTC` instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance {
    /// Interrupt associated with the `DTC` instance.  `DTC_CHANn` = `IELn`.
    type Int: interrupt::typelevel::Interrupt;
}

trait SealedInstance: PeripheralType {}

impl VectorTable {
    const fn new() -> Self {
        Self { vectors: [0; 32] }
    }
}

/// Represents a DTC transfer, implements `Future` for use in async code.
pub struct Transfer<'d, C: Instance> {
    event: InterruptEvent,
    phantom: PhantomData<&'d C>,
}

impl<'d, C: Instance> Transfer<'d, C> {
    fn new(event: InterruptEvent) -> Self {
        Self {
            event,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    /// Links the associated interrupt to the DTC.
    /// The transfer itself won't start until the associated interrupt is triggered.
    pub fn start(&mut self) {
        trace!("DTC{}: Enable for: {}", C::Int::IRQ.number(), self.event);
        C::Int::IRQ.dtc_enable(self.event);
    }

    /// Unlink an interrupt from the DTC.
    /// This has the practical effect of stopping a repeating transfer but may not actually cancel a transfer currently in progress.
    #[inline(always)]
    pub fn stop(&mut self) {
        C::Int::IRQ.icu_disable();
    }
}

pub(crate) fn init() {
    let dtc = pac::DTC;

    // Turn on DMAC and DTC
    crate::peripherals::DTC::start_module();

    let vector_base = unsafe { core::ptr::addr_of!(DTC_VECTOR_TABLE.vectors) as u32 };
    dtc.dtcvbr().write(|r| r.set_dtcvbr(vector_base));

    dtc.dtccr().modify(|r| r.set_rrs(false));
    dtc.dtcst().modify(|r| r.set_dtcst(true));
}

impl<C: Instance> Channel<C> {
    fn waker() -> &'static AtomicWaker {
        static WAKER: AtomicWaker = AtomicWaker::new();
        &WAKER
    }

    /// Constructs a new Dtc driver bound to an interrupt.
    #[allow(private_bounds)]
    pub fn new<'d>(
        peri: Peri<'d, C>,
        irqs: impl interrupt::typelevel::Binding<C::Int, DtcInterruptHandler<C>>,
    ) -> Self {
        let _ = peri;
        let _ = irqs;

        let this = Self {
            entry: Default::default(),
            phantom: PhantomData,
        };

        unsafe { C::Int::IRQ.enable() };
        C::Int::IRQ.icu_disable();

        this
    }

    /// Copies the contents of `source` to `dest`.
    ///
    /// # Arguments
    /// * `source` Slice to copy from.
    /// * `dest` Slice to write to.
    /// * `increment_on` An [`InterruptEvent`] used to start the transfer.  Can be used with
    ///   [`SoftwareEvent`](crate::event_link::SoftwareEvent).
    pub unsafe fn copy_slice<'d, W: Word>(
        &mut self,
        source: &'d [W],
        dest: &'d mut [W],
        increment_on: InterruptEvent,
    ) -> Transfer<'d, C> {
        let len = source.len() as u8;
        let transfer_entry = DtcEntry::builder()
            .with_chain_mode(ChainMode::Disabled)
            .with_source_address_mode(AddressMode::Increment)
            .with_dest_address_mode(AddressMode::Increment)
            .with_word_size(W::WORD_SIZE)
            .with_transfer_mode(TransferMode::Block)
            .with_repeat_mode(RepeatMode::Destination)
            .with_interrupt_mode(InterruptMode::OnCompletion)
            .with_source_address(source.as_ptr() as u32)
            .with_dest_address(dest.as_ptr() as u32)
            .with_count_b(1)
            .with_count_a_low(len)
            .with_count_a_high(len)
            .build();
        self.update_entry(transfer_entry);

        let mut transfer = Transfer::new(increment_on);
        transfer.start();

        transfer
    }

    /// Configures a DTC transfer
    pub unsafe fn write<'d, W: Word>(
        &mut self,
        source: &'d [W],
        dest: *mut W,
        increment_on: InterruptEvent,
        repeat: bool,
    ) -> Transfer<'d, C> {
        // In normal mode count is a 16-bit counter
        // In repeat mode count is an 8-bit counter with an 8-bit reload value
        let (transfer_mode, low_count, high_count) = match repeat {
            false => (TransferMode::Normal, source.len() as u8, 0),
            true => (TransferMode::Repeat, source.len() as u8, source.len() as u8),
        };

        let transfer_entry = DtcEntry::builder()
            .with_chain_mode(ChainMode::Disabled)
            .with_source_address_mode(AddressMode::Increment)
            .with_dest_address_mode(AddressMode::Fixed)
            .with_word_size(W::WORD_SIZE)
            .with_transfer_mode(transfer_mode)
            .with_repeat_mode(RepeatMode::Source)
            .with_interrupt_mode(InterruptMode::OnCompletion)
            .with_source_address(source.as_ptr() as u32)
            .with_dest_address(dest as u32)
            .with_count_b(0)
            .with_count_a_low(low_count)
            .with_count_a_high(high_count)
            .build();
        self.update_entry(transfer_entry);

        let mut transfer = Transfer::new(increment_on);
        transfer.start();

        transfer
    }

    pub(crate) fn prepare_write<'d, W: Word>(
        &'d mut self,
        source: &'d [W],
        dest: *mut W,
        last: bool,
    ) -> DtcEntry {
        let chain_mode = match last {
            true => ChainMode::Disabled,
            false => ChainMode::Penultimate,
        };

        DtcEntry::builder()
            .with_chain_mode(chain_mode)
            .with_source_address_mode(AddressMode::Increment)
            .with_dest_address_mode(AddressMode::Fixed)
            .with_word_size(W::WORD_SIZE)
            .with_transfer_mode(TransferMode::Normal)
            .with_repeat_mode(RepeatMode::Source)
            .with_interrupt_mode(InterruptMode::OnCompletion)
            .with_source_address(source.as_ptr() as u32)
            .with_dest_address(dest as u32)
            .with_count_b(0)
            .with_count_a_low(source.len() as u8)
            .with_count_a_high(0)
            .build()
    }

    /// Configures a DTC transfer
    pub(crate) unsafe fn write_chain<'d>(
        &mut self,
        chain: &'d [DtcEntry],
        increment_on: InterruptEvent,
    ) -> Transfer<'d, C> {
        unsafe { DTC_VECTOR_TABLE.vectors[C::Int::IRQ.number() as usize] = chain.as_ptr() as u32 };

        // Ensure the DTC module re-reads our vector. §17.4.1.
        let dtc = crate::pac::DTC;
        dtc.dtccr().modify(|r| r.set_rrs(false));
        dtc.dtccr().modify(|r| r.set_rrs(true));

        let mut transfer = Transfer::new(increment_on);
        transfer.start();

        transfer
    }

    /// Configures a DTC transfer
    pub unsafe fn read<'d, W: Word>(
        &mut self,
        source: *const W,
        dest: &'d mut [W],
        increment_on: InterruptEvent,
        repeat: bool,
    ) -> Transfer<'d, C> {
        // In normal mode count is a 16-bit counter
        // In repeat mode count is an 8-bit counter with an 8-bit reload value
        let (transfer_mode, low_count, high_count) = match repeat {
            false => (TransferMode::Normal, dest.len() as u8, 0),
            true => (TransferMode::Repeat, dest.len() as u8, dest.len() as u8),
        };

        let transfer_entry = DtcEntry::builder()
            .with_chain_mode(ChainMode::Disabled)
            .with_source_address_mode(AddressMode::Fixed)
            .with_dest_address_mode(AddressMode::Increment)
            .with_word_size(W::WORD_SIZE)
            .with_transfer_mode(transfer_mode)
            .with_repeat_mode(RepeatMode::Destination)
            .with_interrupt_mode(InterruptMode::OnCompletion)
            .with_source_address(source as u32)
            .with_dest_address(dest.as_ptr() as u32)
            .with_count_b(0)
            .with_count_a_low(low_count)
            .with_count_a_high(high_count)
            .build();
        self.update_entry(transfer_entry);

        let mut transfer = Transfer::new(increment_on);
        transfer.start();

        transfer
    }

    #[inline(always)]
    fn update_entry(&mut self, entry: DtcEntry) {
        self.entry = entry;
        let entry_addr = &self.entry as *const _;
        unsafe { DTC_VECTOR_TABLE.vectors[C::Int::IRQ.number() as usize] = entry_addr as u32 };

        // Ensure the DTC module re-reads our vector. §17.4.1.
        let dtc = crate::pac::DTC;
        dtc.dtccr().modify(|r| r.set_rrs(false));
        dtc.dtccr().modify(|r| r.set_rrs(true));
    }
}

impl<'d, C: Instance> Drop for Transfer<'d, C> {
    fn drop(&mut self) {
        trace!(
            "DTC{}: Dropping transfer status={}",
            C::Int::IRQ.number(),
            pac::DTC.dtcsts().read()
        );
    }
}

impl<C: Instance> Drop for Channel<C> {
    fn drop(&mut self) {
        trace!("DTC{}: Drop", C::Int::IRQ.number());
        unsafe { DTC_VECTOR_TABLE.vectors[C::Int::IRQ.number() as usize] = 0 };
        C::Int::IRQ.dtc_disable();
    }
}

impl<C: Instance> InterruptHandler<C::Int> for DtcInterruptHandler<C> {
    unsafe fn on_interrupt() {
        C::Int::IRQ.icu_unpend();

        Channel::<C>::waker().wake();
    }
}

impl<'d, C: Instance> Future for Transfer<'d, C> {
    type Output = ();

    fn poll(
        self: core::pin::Pin<&mut Self>,
        ctx: &mut core::task::Context<'_>,
    ) -> Poll<Self::Output> {
        Channel::<C>::waker().register(ctx.waker());

        // §17.3 On completion of a specified round of data transfer, the ICU.IELSRn.DTCE bit is
        // set to 0 and an interrupt request is sent to the CPU.
        if !C::Int::IRQ.is_dtc() {
            C::Int::IRQ.icu_disable();
            return Poll::Ready(());
        }

        Poll::Pending
    }
}

macro_rules! dtc_link {
    ($index:literal) => {
        paste::paste! {
            impl crate::dtc::Instance for crate::peripherals::[< DTC_CHAN $index >] {
                type Int = interrupt::typelevel::[< IEL $index >];
            }

            impl crate::dtc::SealedInstance for crate::peripherals::[< DTC_CHAN $index >] {
            }
        }
    };
}

dtc_link!(0);
dtc_link!(1);
dtc_link!(2);
dtc_link!(3);
dtc_link!(4);
dtc_link!(5);
dtc_link!(6);
dtc_link!(7);
dtc_link!(8);
dtc_link!(9);
dtc_link!(10);
dtc_link!(11);
dtc_link!(12);
dtc_link!(13);
dtc_link!(14);
dtc_link!(15);
dtc_link!(16);
dtc_link!(17);
dtc_link!(18);
dtc_link!(19);
dtc_link!(20);
dtc_link!(21);
dtc_link!(22);
dtc_link!(23);
dtc_link!(24);
dtc_link!(25);
dtc_link!(26);
dtc_link!(27);
dtc_link!(28);
dtc_link!(29);
dtc_link!(30);
dtc_link!(31);
