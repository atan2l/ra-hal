#[allow(unused)]
use defmt::{debug, error, info, trace, warn};
use embassy_hal_internal::Peri;

use crate::{pac, peripherals::SCE5};

pub struct Sce<'d> {
    _peri: Peri<'d, SCE5>,
    // config: Config,
}

impl<'d> Sce<'d> {
    pub fn new(peri: Peri<'d, SCE5>) -> Self {
        debug!("SCE5: stop=false");
        warn!("Expect increased idle consumption!");

        let mstp = pac::MSTP;

        mstp.mstpcrc().write(|w| {
            w.set_mstpc31(false);
        });

        Self { _peri: peri }
    }

    pub fn uid(&self) -> u32 {
        todo!()
    }
}
