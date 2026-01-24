//! Event Link Controller (`ELC`) and Interrupt Controller Unit (`ICU`) related shenanigans.
//!
//! On the `RA4M1` peripheral interrupts are not directly exposed by the `NVIC`.
//! Instead the `NVIC` exposes 32 programmable interrupts via the `ICU`.
//! Each `ICU` interrupt can be mapped to one of ≈100 interrupt sources as defined in [`InterruptEvent`].
//! Each `ICU` interrupt can also be used to trigger a variety of different events including DMA transfers and power state changes.
//! See §13, §18 of the reference manual for more information.

use crate::pac;

use cortex_m::interrupt::InterruptNumber;

/// Trait that implements functions allowing inspection and manipulation of the interrupt's `ELC`/`ICU` status.
/// Conveniently the `NVIC` only surfaces `ICU` interrupts so each IRQ to IELSR mapping is 1:1.
pub unsafe trait IcuInterrupt: InterruptNumber + Copy {
    /// Disables the interrupt in the `ICU`.  Does not modify its status in the `NVIC`.
    #[inline(always)]
    fn icu_disable(&self) {
        let icu = pac::ICU;

        icu.ielsr(self.number() as _).modify(|w| {
            w.set_iels(ra4m1_ctpac::icu::vals::Iels::_0X000);
        });
    }

    /// Enables the interrupt in the `ICU`.  Does not modify its status in the `NVIC`.
    #[inline(always)]
    fn icu_enable(&self, mask: InterruptEvent) {
        let icu = pac::ICU;

        icu.ielsr(self.number() as _).modify(|w| {
            w.set_iels(ra4m1_ctpac::icu::vals::Iels::from_bits(mask as u8));
        });
    }

    /// Configures the Data Transfer Controller (`DTC`) activation bit.
    ///
    /// See §17 of the reference manual for more information.
    ///
    /// # Arguments
    /// * `true` interrupt will trigger `DTC` activation for vector `n` where `n` is the index of this interrupt.
    /// * `false` interrupt will not trigger `DTC` activation.
    #[inline(always)]
    fn set_dtc(&self, enabled: bool) {
        let icu = pac::ICU;

        icu.ielsr(self.number() as _).modify(|w| {
            w.set_dtce(enabled);
        });
    }

    /// # Returns
    ///
    /// - `true` if the interrupt is configured for `DTC` activation (`IELSRn.DTCE` bit is set).
    /// - `false` otherwise
    #[inline(always)]
    fn is_dtc(&self) -> bool {
        let icu = pac::ICU;

        icu.ielsr(self.number() as _).read().dtce()
    }

    #[inline(always)]
    fn icu_unpend(&self) {
        let icu = pac::ICU;

        icu.ielsr(self.number() as _).modify(|w| {
            w.set_ir(false);
        });
    }

    #[inline(always)]
    fn icu_pend(&self) {
        let icu = pac::ICU;

        icu.ielsr(self.number() as _).modify(|w| {
            w.set_ir(true);
        });
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

/// `ELC` event signal numbers.  These correspond to Table 18.3 in the reference manual. Used in `ELC.ELSRn.ELS`.
#[allow(unused)]
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
}

/// `ICU` event numbers.  These correspond to Table 13.4 in the reference manual. Used in `IELSRn` to generate `NVIC` interrupts and start `DTC` transfers and in `DELSRn` to trigger `DMAC` transfers.
#[allow(unused)]
#[repr(u8)]
pub enum InterruptEvent {
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

    Iic0Rxi = 0x35,
    Iic0Txi = 0x36,
    Iic0Tei = 0x37,
    Iic0Eei = 0x38,
    Iic0Wui = 0x39,

    Iic1Rxi = 0x3A,
    Iic1Txi = 0x3B,
    Iic1Tei = 0x3C,
    Iic1Eei = 0x3D,

    Kint = 0x45,

    CacFerri = 0x47,
    CacMendi = 0x48,
    CacOvfi = 0x49,

    Can0Ers = 0x4A,
    Can0Rxf = 0x4B,
    Can0Txf = 0x4C,
    Can0Rxm = 0x4D,
    Can0Txm = 0x4E,

    Gpt0CcmpA = 0x57,
    Gpt0CcmpB = 0x58,
    Gpt0CmpC = 0x59,
    Gpt0CmpD = 0x5A,
    Gpt0CmpE = 0x5B,
    Gpt0CmpF = 0x5C,
    Gpt0Ovf = 0x5D,
    Gpt0Udf = 0x5E,

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
