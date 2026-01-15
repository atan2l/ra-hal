use core::marker::PhantomData;

use cortex_m::asm;
#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
use embassy_hal_internal::{Peri, PeripheralType};
use ra4m1_ctpac::sci0::{
    regs::{Smr, Tdr},
    vals::{SmrCks, SmrPm, Stop},
};

use crate::{pac, peripherals};

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

impl Instance for peripherals::SCI0 {}
impl SealedInstance for peripherals::SCI0 {
    fn regs() -> ra4m1_ctpac::sci0::Sci0 {
        crate::pac::SCI0
    }

    fn start() {
        pac::MSTP.mstpcrb().write(|w| {
            info!("Starting SCI0");
            w.set_mstpb31(false);
        });
    }

    fn stop() {
        warn!("Stopping SCI0");
        pac::MSTP.mstpcrb().write(|w| {
            w.set_mstpb31(true);
        });
    }
}

impl<'d, I: Instance> Uart<'d, I> {
    pub fn new(_peri: Peri<'d, I>) -> Self {
        I::start();

        let sci = I::regs();

        // not-smart card mode
        sci.scmr().write(|w| {
            w.set_smif(false);
        });

        // // 9 Data bits
        // sci.scmr().write(|w| w.set_chr1(false));
        // sci.smr().write(|w| w.set_chr(false));

        // 8 Data bits
        sci.scmr().write(|w| w.set_chr1(true));
        sci.smr().write(|w| w.set_chr(false));

        // // 7 Data bits
        // // Restrictions apply, page 704 note 3
        // sci.scmr().write(|w| w.set_chr1(true));
        // sci.smr().write(|w| w.set_chr(true));

        // Set it up for _N1
        sci.smr().write(|w| {
            w.set_pe(false);
            // w.set_pm(SmrPm::Even);
            w.set_stop(Stop::Stop1);
        });

        sci.smr().write(|w| {
            w.set_mp(false);
            w.set_cm(false);
        });

        sci.scr().write(|w| {
            w.set_re(false);
            w.set_te(false);
        });

        //Baud=9600 SmallN=0 BigN=156 Error=0.16%

        sci.brr().write(|w| {
            w.set_brr(156);
        });

        sci.smr().write(|w| {
            w.set_cks(SmrCks::DIV_1);
        });

        sci.semr().write(|w| {
            w.set_bgdm(false);
            w.set_abcs(false);
            w.set_abcse(false);
        });

        sci.fcr().write(|w| {
            // turn off FIFO for now
            w.set_fm(false);
        });

        sci.scr().write(|w| {
            w.set_re(true);
            w.set_te(true);
        });

        Self {
            _phantom: PhantomData,
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        let sci = I::regs();

        for byte in data.iter() {
            while !sci.ssr().read().tdre() {
                asm::nop();
            }
            sci.tdr().write_value(Tdr(*byte));
        }
    }
}

impl<'d, I: Instance> Drop for Uart<'d, I> {
    fn drop(&mut self) {
        I::stop();
    }
}
