use core::marker::PhantomData;

use cortex_m::asm;
use embassy_hal_internal::{Peri, PeripheralType, interrupt::InterruptExt as _};
use paste::paste;
use ra4m1_ctpac::sci0::{
    regs::{Scr, Tdr},
    vals::{ScrCke, SmrCks, SmrPm, Stop},
};

use crate::{
    IcuEventer, InterruptEvent,
    gpio::{AnyPin, Pin},
    interrupt,
    interrupt::typelevel::Interrupt,
    pac,
};

#[allow(private_bounds)]
pub struct Uart<'d, I: Instance> {
    _phantom: PhantomData<&'d I>,
}

#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

trait SealedInstance {
    fn regs() -> pac::sci0::Sci0;
    fn start();
    fn stop();
}

trait TxPinSealed<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: crate::gpio::PortFunction;

    #[inline(always)]
    fn pfunc(&self) -> crate::gpio::PortFunction {
        Self::PERIPHERAL_FUNC
    }
}

trait RxPinSealed<I: SealedInstance>: Pin + PeripheralType {
    const PERIPHERAL_FUNC: crate::gpio::PortFunction;

    #[inline(always)]
    fn pfunc(&self) -> crate::gpio::PortFunction {
        Self::PERIPHERAL_FUNC
    }
}

#[allow(private_bounds)]
pub struct TxPin<'d, I: SealedInstance> {
    // TODO: Should we remove this field?
    _pin: Peri<'d, AnyPin>,
    _phantom_i: PhantomData<I>,
}

#[allow(private_bounds)]
pub struct RxPin<'d, I: SealedInstance> {
    // TODO: Should we remove this field?
    _pin: Peri<'d, AnyPin>,
    _phantom_i: PhantomData<I>,
}

#[allow(private_bounds)]
impl<'d, I: SealedInstance> TxPin<'d, I> {
    /// Takes a pin and configures it to be used as an SCI TX line
    pub fn new(pin: Peri<'d, impl TxPinSealed<I>>) -> Self {
        debug!("TX: {}/{}", pin._port(), pin._pin());
        pin.set_port_func(pin.pfunc());

        Self {
            _pin: pin.into(),
            _phantom_i: PhantomData,
        }
    }
}

#[allow(private_bounds)]
impl<'d, I: SealedInstance> RxPin<'d, I> {
    /// Takes a pin and configures it to be used as an SCI RX line
    pub fn new(pin: Peri<'d, impl RxPinSealed<I>>) -> Self {
        debug!("RX: {}/{}", pin._port(), pin._pin());
        pin.set_port_func(pin.pfunc());

        Self {
            _pin: pin.into(),
            _phantom_i: PhantomData,
        }
    }
}

macro_rules! tx_pin_impl {
    ($sci:ident, $pin:ident, $pfunc:ident) => {
        impl crate::uart::TxPinSealed<crate::peripherals::$sci> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}

macro_rules! rx_pin_impl {
    ($sci:ident, $pin:ident, $pfunc:ident) => {
        impl crate::uart::RxPinSealed<crate::peripherals::$sci> for crate::peripherals::$pin {
            const PERIPHERAL_FUNC: crate::gpio::PortFunction = crate::gpio::PortFunction::$pfunc;
        }
    };
}

macro_rules! instance_impl {
    ($periph:ident, $stop:ident) => {
        impl Instance for crate::peripherals::$periph {}
        paste! {
            impl SealedInstance for crate::peripherals::$periph {
                fn regs() -> ra4m1_ctpac::sci0::Sci0 {
                    crate::pac::$periph
                }

                fn start() {
                    debug!("{}: stop=false", stringify!($periph));

                    pac::MSTP.mstpcrb().write(|w| {
                        w.[< set_ $stop >](false);
                    });
                }

                fn stop() {
                    debug!("{}: stop=true", stringify!($periph));

                    pac::MSTP.mstpcrb().write(|w| {
                        w.[< set_ $stop >](true);
                    });
                }
            }
        }
    };
}

tx_pin_impl!(SCI0, P101, Sci1);
tx_pin_impl!(SCI0, P205, Sci1);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
tx_pin_impl!(SCI0, P411, Sci1);

tx_pin_impl!(SCI1, P213, Sci2);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
tx_pin_impl!(SCI1, P401, Sci2);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
tx_pin_impl!(SCI1, P501, Sci2);

rx_pin_impl!(SCI0, P100, Sci1);
#[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
rx_pin_impl!(SCI0, P104, Sci1);
#[cfg(any(feature = "_48pin", feature = "_64pin", feature = "_100pin"))]
rx_pin_impl!(SCI0, P206, Sci1);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
rx_pin_impl!(SCI0, P410, Sci1);

rx_pin_impl!(SCI1, P212, Sci2);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
rx_pin_impl!(SCI1, P402, Sci2);
#[cfg(any(feature = "_64pin", feature = "_100pin"))]
rx_pin_impl!(SCI1, P502, Sci2);
#[cfg(feature = "_100pin")]
rx_pin_impl!(SCI1, P708, Sci2);

instance_impl!(SCI0, mstpb31);
instance_impl!(SCI1, mstpb30);

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
struct SpeedEntry {
    baud_rate: u32,
    small_n: u8,
    big_n: u8,
    modulation: u8,
}

const SPEED_ENTRIES: [SpeedEntry; 4] = [
    //
    SpeedEntry {
        baud_rate: 300,
        small_n: 3,
        big_n: 77,
        modulation: 0,
    },
    SpeedEntry {
        baud_rate: 9600,
        small_n: 1,
        big_n: 38,
        modulation: 0,
    },
    SpeedEntry {
        baud_rate: 9600,
        small_n: 0,
        big_n: 140,
        modulation: 231,
    },
    SpeedEntry {
        baud_rate: 115200,
        small_n: 0,
        big_n: 12,
        modulation: 0,
    },
];

impl<'d, I: Instance> Uart<'d, I> {
    #[inline]
    fn set_data_bits(n: u8) {
        let sci = I::regs();

        match n {
            7 => {
                // // 7 Data bits
                // // Restrictions apply, page 704 note 3
                sci.scmr().modify(|w| w.set_chr1(true));
                sci.smr().modify(|w| w.set_chr(true));
            }
            8 => {
                // 8 Data bits
                info!("Setting 8 data bits");
                sci.scmr().modify(|w| w.set_chr1(true));
                sci.smr().modify(|w| w.set_chr(false));
            }
            9 => {
                // // 9 Data bits
                // sci.scmr().write(|w| w.set_chr1(false));
                // sci.smr().write(|w| w.set_chr(false));
                todo!()
            }
            _ => unimplemented!(),
        }
    }

    #[allow(private_bounds)]
    pub fn new(
        _peri: Peri<'d, I>,
        tx: Peri<'d, impl TxPinSealed<I>>,
        rx: Peri<'d, impl RxPinSealed<I>>,
    ) -> Self {
        I::start();

        let sci = I::regs();

        sci.scr().write_value(Scr(0));

        sci.scr().modify(|w| {
            w.set_tie(false);
            w.set_rie(false);
            w.set_te(false);
            w.set_re(false);
            w.set_teie(false);
        });

        sci.fcr().modify(|w| {
            // turn off FIFO for now
            w.set_fm(false);
        });

        sci.scr().modify(|w| {
            w.set_cke(ScrCke::_00);
        });

        sci.simr1().modify(|w| {
            w.set_iicm(false);
        });

        sci.spmr().modify(|w| {
            w.set_ckph(false);
            w.set_ckpol(false);
        });

        // not-smart card mode
        sci.scmr().modify(|w| {
            w.set_smif(false);
        });

        Self::set_data_bits(8);

        // Set it up for No Parity, 1 stop bit
        sci.smr().modify(|w| {
            w.set_pe(false);
            w.set_pm(SmrPm::Even);
            w.set_stop(Stop::Stop1);
        });

        sci.smr().modify(|w| {
            w.set_mp(false);
            w.set_cm(false);
        });

        sci.semr().modify(|w| {
            w.set_brme(false);
            w.set_bgdm(false);
            w.set_abcs(false);
            w.set_abcse(false);
            w.set_rxdesel(false);
        });

        sci.sptr().write(|w| {
            w.set_spb2dt(false);
            w.set_spb2io(false);
        });

        let speed = &SPEED_ENTRIES[0];
        Self::set_speed_from_entry(speed);

        // Move pins over to SCI
        let tx = TxPin::new(tx);
        let rx = RxPin::new(rx);
        let _ = tx;
        let _ = rx;

        unsafe { Sci1RxInterrupt::IRQ.enable() };
        Sci1RxInterrupt::iel_enable();

        sci.scr().modify(|w| {
            w.set_re(true);
            w.set_te(true);
            w.set_rie(true);
        });

        warn!("SMR: {}", sci.smr().read());
        warn!("SCR: {}", sci.scr().read());
        warn!("SSR: {}", sci.ssr().read());
        warn!("SEMR: {}", sci.semr().read());
        warn!("BRR: {}", sci.brr().read());
        warn!("FCR: {}", sci.fcr().read());

        Self {
            _phantom: PhantomData,
        }
    }

    fn show_speed() {
        let sci = I::regs();
        let brr = sci.brr().read().brr();
        let mddr = sci.mddr().read().mddr();
        let brme = sci.semr().read().brme();
        info!("Speed: BRR: {}, MDDR: {}, BRME: {}", brr, mddr, brme);
    }

    fn set_speed_from_entry(speed: &SpeedEntry) {
        let sci = I::regs();

        sci.scr().modify(|w| {
            w.set_re(false);
            w.set_te(false);
        });
        info!("{}", speed);
        sci.brr().write(|w| {
            w.set_brr(speed.big_n);
        });
        if speed.modulation != 0 {
            sci.mddr().write(|w| {
                w.set_mddr(speed.modulation);
            });
            sci.semr().modify(|w| {
                w.set_brme(true);
            });
        } else {
            sci.mddr().write(|w| {
                w.set_mddr(0);
            });
            sci.semr().modify(|w| w.set_brme(false));
        }
        sci.smr().modify(|w| {
            w.set_cks(SmrCks::from_bits(speed.small_n));
        });
        sci.scr().modify(|w| {
            w.set_re(true);
            w.set_te(true);
        });
        Self::show_speed();
    }

    pub fn set_speed(&mut self, baud_rate: u32) {
        let speed = SPEED_ENTRIES
            .iter()
            .find(|e| e.baud_rate == baud_rate)
            .unwrap();
        Self::set_speed_from_entry(speed);
    }

    #[inline(always)]
    pub fn blocking_read(&self, data: &mut [u8]) {
        let sci = I::regs();

        warn!("Uhh? {:02x}", sci.ssr().read());

        for byte in data.iter_mut() {
            while !sci.ssr().read().rdrf() {
                asm::nop();
            }
            *byte = sci.rdr().read().rdr();
            // if *byte == 0x0a {
            //     return;
            // }
        }
    }

    pub fn blocking_write(&mut self, data: &[u8]) {
        let sci = I::regs();

        for byte in data.iter() {
            while !sci.ssr().read().tdre() {
                asm::nop();
            }
            info!("Writing: {:02x}", *byte as char);
            sci.tdr().write_value(Tdr(*byte));
            while !sci.ssr().read().tdre() {
                asm::nop();
            }
        }
    }
}

impl<'d, I: Instance> Drop for Uart<'d, I> {
    fn drop(&mut self) {
        I::stop();
    }
}

type Sci1RxInterrupt = crate::interrupt::typelevel::IEL2;

impl IcuEventer for crate::interrupt::typelevel::IEL2 {
    const ICU_INDEX: u8 = 2;
    const ICU_MASK: InterruptEvent = InterruptEvent::Sci1Rxi;
}

#[interrupt]
fn IEL2() {
    let icu = pac::ICU;
    warn!("RXD");

    icu.ielsr(2).modify(|w| {
        w.set_ir(false);
    });
}
