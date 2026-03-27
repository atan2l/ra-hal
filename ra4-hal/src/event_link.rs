//! Event Link Controller (`ELC`) and Interrupt Controller Unit (`ICU`) related shenanigans.
//!
//! On the `RA4M1` peripheral interrupts are not directly exposed by the `NVIC`.
//! Instead the `NVIC` exposes 32 programmable interrupts via the `ICU`.
//! Each `ICU` interrupt can be mapped to one of ≈100 interrupt sources as defined in [`InterruptEvent`].
//! Each `ICU` interrupt can also be used to trigger a variety of different events including DMA transfers and power state changes.
//! See §13, §18 of the reference manual for more information.

use cortex_m::interrupt::InterruptNumber;

use crate::{
    module_stop::ModuleStop as _,
    pac::{self, icu::regs::Ielsr},
    peripherals::ELC,
};

/// Trait that implements functions allowing inspection and manipulation of the interrupt's `ELC`/`ICU` status.
///
/// # Safety
///
/// Conveniently the `NVIC` only surfaces `ICU` interrupts so each IRQ to IELSR mapping is 1:1.
pub unsafe trait IcuInterrupt: InterruptNumber + Copy {
    /// Disables the interrupt in the `ICU`.  Does not modify its status in the `NVIC`.
    #[inline(always)]
    fn icu_disable(&self) {
        let icu = pac::ICU;

        icu.ielsr(self.number() as _).modify(|w| w.set_iels(0));
        trace!("IEL{}: disable", self.number());
    }

    /// Enables the interrupt in the `ICU`.  Does not modify its status in the `NVIC`.
    ///
    /// # Safety
    ///
    /// Safe so long as there's an interrupt handler in place.
    #[inline(always)]
    unsafe fn icu_enable(&self, mask: InterruptEvent) {
        let icu = pac::ICU;

        trace!("IEL{}: enable={}", self.number(), mask);

        icu.ielsr(self.number() as _).modify(|w| {
            w.set_iels(mask as _);
        });
    }

    /// Returns `true` if the interrupt has been linked to an [`InterruptEvent`].
    #[inline(always)]
    fn icu_enabled(&self) -> bool {
        self.icu_event() != InterruptEvent::None
    }

    /// Returns the [`InterruptEvent`] that the interrupt is linked to or `InterruptEvent::None` if none is configured.
    #[inline(always)]
    fn icu_event(&self) -> InterruptEvent {
        let icu = pac::ICU;

        let event = icu.ielsr(self.number() as _).read().iels();

        // Yeah we're trusting there won't be gibberish in the register.
        // TODO: Find a better way to map a bunch of variants to/from primitives.
        let event: InterruptEvent = unsafe { core::mem::transmute(event) };
        trace!("IEL{}: event={}", self.number(), event);

        event
    }

    /// Disables the interrupt in the `ICU` and disables it as a `DTC` activation source.  Does not modify its status in the `NVIC`.
    ///
    /// See §17 of the reference manual for more information.
    #[inline(always)]
    fn dtc_disable(&self) {
        let icu = pac::ICU;

        icu.ielsr(self.number() as _).write_value(Ielsr(0));
    }

    /// Links an interrupt to an event and configures `DTC` vector `n` for activation on interrupt where `n` is the index of this interrupt.
    ///
    /// See §17 of the reference manual for more information.
    #[inline(always)]
    fn dtc_enable(&self, mask: InterruptEvent) {
        let icu = pac::ICU;

        trace!("IEL{}: enable={}, dtc=true", self.number(), mask);

        icu.ielsr(self.number() as _).write(|r| {
            r.set_dtce(true);
            r.set_iels(mask as _);
        });
    }

    /// # Returns
    ///
    /// - `true` if the interrupt is configured for `DTC` activation.
    /// - `false` otherwise
    #[inline(always)]
    fn is_dtc(&self) -> bool {
        let icu = pac::ICU;

        icu.ielsr(self.number() as _).read().dtce()
    }

    /// Clears the interrupt request flag in the `ICU`.
    /// Does not modify the `NVIC`.
    ///
    /// Similar to [`InterruptExt::unpend`](trait@crate::interrupt::InterruptExt).
    /// See also §13.2.6.
    #[inline(always)]
    fn icu_unpend(&self) {
        let icu = pac::ICU;

        icu.ielsr(self.number() as _).modify(|w| w.set_ir(false));
    }

    /// Sets the interrupt request flag in the `ICU`.
    /// Does not modify the `NVIC`.
    ///
    /// Similar to [`InterruptExt::pend`](trait@crate::interrupt::InterruptExt).
    /// See also §13.2.6.
    #[inline(always)]
    fn icu_pend(&self) {
        let icu = pac::ICU;

        icu.ielsr(self.number() as _).modify(|w| w.set_ir(true));
    }

    /// Returns true if the interrupt is pending in the `ICU`.
    fn icu_is_pending(&self) -> bool {
        let icu = pac::ICU;

        icu.ielsr(self.number() as _).read().ir()
    }

    /// Logs information about how this interrupt is configured in the `ICU` at the `trace` level.
    fn icu_status(&self) {
        let icu = pac::ICU;
        let number = self.number();
        let ielsr = icu.ielsr(number as _);
        let status = ielsr.read();
        trace!("IEL{}: ir={}, dtce={}", number, status.ir(), status.dtce());
    }
}

unsafe impl<T: InterruptNumber + Copy> IcuInterrupt for T {}

/// Trait that restricts the software event generator to one of the two available events.
#[allow(private_bounds)]
pub trait SoftwareEventGenerator<const N: u8>: SealedSoftwareEventGenerator<N> {
    /// Returns the [`InterruptEvent`] associated with this software generated event.
    fn event(&self) -> InterruptEvent;

    /// Trigger a software generated event.
    #[inline(always)]
    fn fire(&self) {
        let elc = crate::pac::ELC;
        elc.elsegr(self.index() as _).write(|r| r.set_wi(false));
        elc.elsegr(self.index() as _).write(|r| r.set_we(true));
        elc.elsegr(self.index() as _).write(|r| r.set_seg(true));
    }
}

trait SealedSoftwareEventGenerator<const N: u8> {
    fn index(&self) -> u8;
}

/// A software event generator.
/// Use this to trigger a software event for the ICU.
/// On the `RA4M1` valid values for `N` are 0 or 1.
pub struct SoftwareEvent<const N: u8> {}

impl<const N: u8> SoftwareEvent<N> {
    /// Constructs a new software event generator.
    pub const fn new() -> Self {
        Self {}
    }
}

impl SoftwareEventGenerator<0> for SoftwareEvent<0> {
    #[inline(always)]
    fn event(&self) -> InterruptEvent {
        InterruptEvent::ElcSwEvt0
    }
}

impl SealedSoftwareEventGenerator<0> for SoftwareEvent<0> {
    #[inline(always)]
    fn index(&self) -> u8 {
        0
    }
}

impl SoftwareEventGenerator<1> for SoftwareEvent<1> {
    #[inline(always)]
    fn event(&self) -> InterruptEvent {
        InterruptEvent::ElcSwEvt1
    }
}
impl SealedSoftwareEventGenerator<1> for SoftwareEvent<1> {
    #[inline(always)]
    fn index(&self) -> u8 {
        1
    }
}

/// `ELC` event signal numbers.
/// These correspond to Table 18.3 in the reference manual.
/// Used in `ELC.ELSRn.ELS`.
#[allow(unused, missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum EventSignal {
    PortIrq0 = 0x01,
    PortIrq1 = 0x02,
    PortIrq2 = 0x03,
    PortIrq3 = 0x04,
    PortIrq4 = 0x05,
    PortIrq5 = 0x06,
    PortIrq6 = 0x07,
    PortIrq7 = 0x08,
    PortIrq8 = 0x09,
    PortIrq9 = 0x0a,
    PortIrq10 = 0x0b,
    PortIrq11 = 0x0c,
    PortIrq12 = 0x0d,
    PortIrq13 = 0x0e,
    PortIrq14 = 0x0f,
    PortIrq15 = 0x10,

    Dmac0Int = 0x11,
    Dmac1Int = 0x12,
    Dmac2Int = 0x13,
    Dmac3Int = 0x14,

    DtcEnd = 0x16,

    Lvd1 = 0x19,
    Lvd2 = 0x1A,

    MoscStop = 0x1C,

    SnoozeRequest = 0x1D,

    Agt0Agti = 0x1E,
    Agt0AgtCmAi = 0x1F,
    Agt0AgtCmBi = 0x20,

    Agt1Agti = 0x21,
    Agt1AgtCmAi = 0x22,
    Agt1AgtCmBi = 0x23,

    IwdtNmi = 0x24,
    WdtNmi = 0x25,

    RtcPrd = 0x27,

    AdcAdi = 0x29,
    AdcWcmpM = 0x2D,
    AdcWcmpUm = 0x2E,

    AcmpLp0 = 0x2F,
    AcmpLp1 = 0x30,

    Iic0Rxi = 0x35,
    Iic0Txi = 0x36,
    Iic0Tei = 0x37,
    Iic0Eei = 0x38,

    Iic1Rxi = 0x3A,
    Iic1Txi = 0x3B,
    Iic1Tei = 0x3C,
    Iic1Eei = 0x3D,

    DocDopci = 0x46,

    IoportGroup1 = 0x4F,
    IoportGroup2 = 0x50,
    IoportGroup3 = 0x51,
    IoportGroup4 = 0x52,

    ElcSwEvt0 = 0x53,
    ElcSwEvt1 = 0x54,

    Gpt0CcmpA = 0x57,
    Gpt0CcmpB = 0x58,
    Gpt0CmpC = 0x59,
    Gpt0CmpD = 0x5A,
    Gpt0CmpE = 0x5B,
    Gpt0CmpF = 0x5C,
    Gpt0Ovf = 0x5D,
    Gpt0Udf = 0x5E,

    Gpt1CcmpA = 0x5F,
    Gpt1CcmpB = 0x60,
    Gpt1CmpC = 0x61,
    Gpt1CmpD = 0x62,
    Gpt1CmpE = 0x63,
    Gpt1CmpF = 0x64,
    Gpt1Ovf = 0x65,
    Gpt1Udf = 0x66,

    Gpt2CcmpA = 0x67,
    Gpt2CcmpB = 0x68,
    Gpt2CmpC = 0x69,
    Gpt2CmpD = 0x6A,
    Gpt2CmpE = 0x6B,
    Gpt2CmpF = 0x6C,
    Gpt2Ovf = 0x6D,
    Gpt2Udf = 0x6E,

    Gpt3CcmpA = 0x6F,
    Gpt3CcmpB = 0x70,
    Gpt3CmpC = 0x71,
    Gpt3CmpD = 0x72,
    Gpt3CmpE = 0x73,
    Gpt3CmpF = 0x74,
    Gpt3Ovf = 0x75,
    Gpt3Udf = 0x76,

    Gpt4CcmpA = 0x77,
    Gpt4CcmpB = 0x78,
    Gpt4CmpC = 0x79,
    Gpt4CmpD = 0x7A,
    Gpt4CmpE = 0x7B,
    Gpt4CmpF = 0x7C,
    Gpt4Ovf = 0x7D,
    Gpt4Udf = 0x7E,

    Gpt5CcmpA = 0x7F,
    Gpt5CcmpB = 0x80,
    Gpt5CmpC = 0x81,
    Gpt5CmpD = 0x82,
    Gpt5CmpE = 0x83,
    Gpt5CmpF = 0x84,
    Gpt5Ovf = 0x85,
    Gpt5Udf = 0x86,

    Gpt6CcmpA = 0x87,
    Gpt6CcmpB = 0x88,
    Gpt6CmpC = 0x89,
    Gpt6CmpD = 0x8A,
    Gpt6CmpE = 0x8B,
    Gpt6CmpF = 0x8C,
    Gpt6Ovf = 0x8D,
    Gpt6Udf = 0x8E,

    Gpt7CcmpA = 0x8F,
    Gpt7CcmpB = 0x90,
    Gpt7CmpC = 0x91,
    Gpt7CmpD = 0x92,
    Gpt7CmpE = 0x93,
    Gpt7CmpF = 0x94,
    Gpt7Ovf = 0x95,
    Gpt7Udf = 0x96,

    GptUvwEdge = 0x97,

    Sci0Rxi = 0x98,
    Sci0Txi = 0x99,
    Sci0Tei = 0x9A,
    Sci0Eri = 0x9B,
    Sci0Am = 0x9C,

    Sci1Rxi = 0x9E,
    Sci1Txi = 0x9F,
    Sci1Tei = 0xA0,
    Sci1Eri = 0xA1,
    Sci1Am = 0xA2,

    Sci2Rxi = 0xA3,
    Sci2Txi = 0xA4,
    Sci2Tei = 0xA5,
    Sci2Eri = 0xA6,
    Sci2Am = 0xA7,

    Sci9Rxi = 0xA8,
    Sci9Txi = 0xA9,
    Sci9Tei = 0xAA,
    Sci9Eri = 0xAB,
    Sci9Am = 0xAC,

    Spi0SpRi = 0xAD,
    Spi0SpTi = 0xAE,
    Spi0SpIi = 0xAF,
    Spi0SpEi = 0xB0,
    Spi0SpTend = 0xB1,

    Spi1SpRi = 0xB2,
    Spi1SpTi = 0xB3,
    Spi1SpIi = 0xB4,
    Spi1SpEi = 0xB5,
    Spi1SpTend = 0xB6,
}

/// `ICU` event numbers.
/// These correspond to Table 13.4 in the reference manual.
/// Used in `IELSRn` to generate `NVIC` interrupts and start `DTC` transfers and in `DELSRn` to trigger `DMAC` transfers.
#[allow(unused, missing_docs)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InterruptEvent {
    None = 0x00,

    PortIrq0 = 0x01,
    PortIrq1 = 0x02,
    PortIrq2 = 0x03,
    PortIrq3 = 0x04,
    PortIrq4 = 0x05,
    PortIrq5 = 0x06,
    PortIrq6 = 0x07,
    PortIrq7 = 0x08,
    PortIrq8 = 0x09,
    PortIrq9 = 0x0a,
    PortIrq10 = 0x0b,
    PortIrq11 = 0x0c,
    PortIrq12 = 0x0d,
    PortIrq13 = 0x0e,
    PortIrq14 = 0x0f,
    PortIrq15 = 0x10,

    Dmac0Int = 0x11,
    Dmac1Int = 0x12,
    Dmac2Int = 0x13,
    Dmac3Int = 0x14,

    DtcComplete = 0x15,

    SnoozeCancel = 0x17,

    FcuFrdyi = 0x18,

    Lvd1 = 0x19,
    Lvd2 = 0x1A,

    VbattLvd = 0x1B,

    MoscStop = 0x1C,

    SnoozeRequest = 0x1D,

    Agt0Agti = 0x1E,
    Agt0AgtCmAi = 0x1F,
    Agt0AgtCmBi = 0x20,

    Agt1Agti = 0x21,
    Agt1AgtCmAi = 0x22,
    Agt1AgtCmBi = 0x23,

    IwdtNmi = 0x24,
    WdtNmi = 0x25,

    RtcAlarm = 0x26,
    RtcPrd = 0x27,
    RtcCup = 0x28,

    AdcAdi = 0x29,
    AdcGbadi = 0x2A,
    AdcCmpAi = 0x2B,
    AdcCmpBi = 0x2C,
    AdcWcmpM = 0x2D,
    AdcWcmpUm = 0x2E,

    AcmpLp0 = 0x2F,
    AcmpLp1 = 0x30,

    UsbfsD0Fifo = 0x31,
    UsbfsD1Fifo = 0x32,
    UsbfsUsbi = 0x33,
    UsbfsUsbr = 0x34,

    Iic0Rxi = 0x35,
    Iic0Txi = 0x36,
    Iic0Tei = 0x37,
    Iic0Eei = 0x38,
    Iic0Wui = 0x39,

    Iic1Rxi = 0x3A,
    Iic1Txi = 0x3B,
    Iic1Tei = 0x3C,
    Iic1Eei = 0x3D,

    SseiSsiTxi = 0x3E,
    SseiSsiRxi = 0x3F,
    SseiSsif = 0x41,

    CtsuWr = 0x42,
    CtsuRd = 0x43,
    CtsuFn = 0x44,

    Kint = 0x45,

    DocDopci = 0x46,

    CacFerri = 0x47,
    CacMendi = 0x48,
    CacOvfi = 0x49,

    Can0Ers = 0x4A,
    Can0Rxf = 0x4B,
    Can0Txf = 0x4C,
    Can0Rxm = 0x4D,
    Can0Txm = 0x4E,

    IoportGroup1 = 0x4F,
    IoportGroup2 = 0x50,
    IoportGroup3 = 0x51,
    IoportGroup4 = 0x52,

    ElcSwEvt0 = 0x53,
    ElcSwEvt1 = 0x54,

    PoegGroup0 = 0x55,
    PoegGroup1 = 0x56,

    Gpt0CcmpA = 0x57,
    Gpt0CcmpB = 0x58,
    Gpt0CmpC = 0x59,
    Gpt0CmpD = 0x5A,
    Gpt0CmpE = 0x5B,
    Gpt0CmpF = 0x5C,
    Gpt0Ovf = 0x5D,
    Gpt0Udf = 0x5E,

    Gpt1CcmpA = 0x5F,
    Gpt1CcmpB = 0x60,
    Gpt1CmpC = 0x61,
    Gpt1CmpD = 0x62,
    Gpt1CmpE = 0x63,
    Gpt1CmpF = 0x64,
    Gpt1Ovf = 0x65,
    Gpt1Udf = 0x66,

    Gpt2CcmpA = 0x67,
    Gpt2CcmpB = 0x68,
    Gpt2CmpC = 0x69,
    Gpt2CmpD = 0x6A,
    Gpt2CmpE = 0x6B,
    Gpt2CmpF = 0x6C,
    Gpt2Ovf = 0x6D,
    Gpt2Udf = 0x6E,

    Gpt3CcmpA = 0x6F,
    Gpt3CcmpB = 0x70,
    Gpt3CmpC = 0x71,
    Gpt3CmpD = 0x72,
    Gpt3CmpE = 0x73,
    Gpt3CmpF = 0x74,
    Gpt3Ovf = 0x75,
    Gpt3Udf = 0x76,

    Gpt4CcmpA = 0x77,
    Gpt4CcmpB = 0x78,
    Gpt4CmpC = 0x79,
    Gpt4CmpD = 0x7A,
    Gpt4CmpE = 0x7B,
    Gpt4CmpF = 0x7C,
    Gpt4Ovf = 0x7D,
    Gpt4Udf = 0x7E,

    Gpt5CcmpA = 0x7F,
    Gpt5CcmpB = 0x80,
    Gpt5CmpC = 0x81,
    Gpt5CmpD = 0x82,
    Gpt5CmpE = 0x83,
    Gpt5CmpF = 0x84,
    Gpt5Ovf = 0x85,
    Gpt5Udf = 0x86,

    Gpt6CcmpA = 0x87,
    Gpt6CcmpB = 0x88,
    Gpt6CmpC = 0x89,
    Gpt6CmpD = 0x8A,
    Gpt6CmpE = 0x8B,
    Gpt6CmpF = 0x8C,
    Gpt6Ovf = 0x8D,
    Gpt6Udf = 0x8E,

    Gpt7CcmpA = 0x8F,
    Gpt7CcmpB = 0x90,
    Gpt7CmpC = 0x91,
    Gpt7CmpD = 0x92,
    Gpt7CmpE = 0x93,
    Gpt7CmpF = 0x94,
    Gpt7Ovf = 0x95,
    Gpt7Udf = 0x96,

    GptUvwEdge = 0x97,

    Sci0Rxi = 0x98,
    Sci0Txi = 0x99,
    Sci0Tei = 0x9A,
    Sci0Eri = 0x9B,
    Sci0Am = 0x9C,
    Sci0RxiOrEri = 0x9D,

    Sci1Rxi = 0x9E,
    Sci1Txi = 0x9F,
    Sci1Tei = 0xA0,
    Sci1Eri = 0xA1,
    Sci1Am = 0xA2,

    Sci2Rxi = 0xA3,
    Sci2Txi = 0xA4,
    Sci2Tei = 0xA5,
    Sci2Eri = 0xA6,
    Sci2Am = 0xA7,

    Sci9Rxi = 0xA8,
    Sci9Txi = 0xA9,
    Sci9Tei = 0xAA,
    Sci9Eri = 0xAB,
    Sci9Am = 0xAC,

    Spi0SpRi = 0xAD,
    Spi0SpTi = 0xAE,
    Spi0SpIi = 0xAF,
    Spi0SpEi = 0xB0,
    Spi0SpTend = 0xB1,

    Spi1SpRi = 0xB2,
    Spi1SpTi = 0xB3,
    Spi1SpIi = 0xB4,
    Spi1SpEi = 0xB5,
    Spi1SpTend = 0xB6,
}

/// Turn on the `ELC` clock.
pub(crate) fn init() {
    ELC::start_module();
}
