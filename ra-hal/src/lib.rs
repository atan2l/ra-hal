#![no_std]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

//! ## Feature flags
#![doc = document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#)]

// This needs to come first so the macros are visible everywhere else.
#[doc(hidden)]
pub mod fmt;

#[cfg(any(adc12, adc14, adc16))]
pub mod adc;

pub mod clock;
pub mod crc;
#[cfg(dac12)]
pub mod dac;
#[cfg(dmac)]
pub mod dmac;
pub mod dtc;
pub mod event_link;
pub mod gpio;
#[cfg(iic)]
pub mod i2c;
pub mod mcu_info;
pub mod module_stop;
#[cfg(not(feature = "skip-osm"))]
pub mod osm;
#[cfg(gpt)]
pub mod pwm;
#[cfg(gpt)]
pub mod qdec;
// #[cfg(feature = "_enable-rtc-beware-of-dragons")]
// pub mod rtc;
#[cfg(any(agt, agtw))]
pub mod timer_agt;
#[cfg(gpt)]
pub mod timer_gpt;
pub mod watchdog;
// pub mod sce5;
#[cfg(ra4m1)]
pub mod spi;

// This uses cfg_select explicitly so we can avoid defining a time_driver module if the feature
// isn't enabled, but still fail if we don't enable a specific time driver.
cfg_select! {
    feature = "time-driver" => {
        #[cfg_attr(feature = "time-driver-agt", path = "time_driver_agt.rs")]
        #[cfg_attr(feature = "time-driver-gpt", path = "time_driver_gpt.rs")]
        #[cfg_attr(feature = "time-driver-ulpt", path = "time_driver_ulpt.rs")]
        pub mod time_driver;
    }
    _ => {}
}

#[cfg(not(sci_b))]
pub mod uart;
pub mod write_protect;

// Re-exports
#[cfg(feature = "chrono")]
pub use chrono;

cfg_select! {
    feature = "unstable-pac" => {
        pub use ra_metapac as pac;
    }
    _ => {
        pub(crate) use ra_metapac as pac;
    }
}

use crate::mcu_info::McuInfo;

/// Coarse indication of why the processor reset.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(unused)]
pub enum ResetCause {
    /// Power was turned on.
    PowerOn,

    /// Low voltage monitor 0, 1, or 2 tripped.
    LowVoltage,

    /// Watchdog or independent watchdog.
    Watchdog,

    /// Bus error, parity error, or ECC error.
    HadwareError,

    /// Stack pointer error.
    StackPointer,

    /// Software reset requested.
    SoftwareReset,

    /// Should never be here.
    Unknown,
}

/// IDAU regions per the reference manual.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
enum IdauRegion {
    NonSecureSram = 0x0D,
    NonSecureCallableSram = 0x0E,
    SecureSram = 0x0F,
    NonSecureDataFlash = 0x09,
    SecureDataFlash = 0x0B,
    NonSecureCodeFlash = 0x05,
    NonSecureCallableCodeFlash = 0x06,
    SecureCodeFlash = 0x07,
}

impl From<u8> for IdauRegion {
    fn from(value: u8) -> Self {
        match value {
            0x05 => Self::NonSecureCodeFlash,
            0x06 => Self::NonSecureCallableCodeFlash,
            0x07 => Self::SecureCodeFlash,
            0x09 => Self::NonSecureDataFlash,
            0x0B => Self::SecureDataFlash,
            0x0D => Self::NonSecureSram,
            0x0E => Self::NonSecureCallableSram,
            0x0F => Self::SecureSram,
            _ => unreachable!(),
        }
    }
}

#[cfg(trust_zone)]
fn trust_zone_init() {
    // 4L1 RM: After reset, all of address space is marked as Secure by SAU default setting.
    // SAU_CTRL register should be set to 0x2 to enable the IDAU security attribution. That
    // is, after setting SAU_CTRL register to 0x2, the address space security attribution
    // becomes as shown in Table 48.6.
    // TODO: Handle the SAU in RA8
    use pac::system::vals::Prc4;

    #[cfg(secure)]
    use pac::cpscu::{regs::Icusar, vals::SecurityAttribution};

    unsafe {
        (*cortex_m::peripheral::SAU::PTR)
            .ctrl
            .write(cortex_m::peripheral::sau::Ctrl(0x02))
    };

    // let idau_region = {
    //     let mut i = [1];
    //     let tt =
    //         cortex_m::cmse::TestTarget::check(i.as_mut_ptr(), cortex_m::cmse::AccessType::Current);
    //     tt.idau_region().map(IdauRegion::from)
    // };
    // let pscu = pac::PSCU;
    // let life_cycle = pscu.dlmmon().read().dlmmon();
    // let secure_mode = idau_region == Some(IdauRegion::SecureSram);

    // info!(
    //     "TrustZone: secure_mode={}, region={} life_cycle={}",
    //     secure_mode, idau_region, life_cycle
    // );

    let cpscu = pac::CPSCU;
    let system = pac::SYSTEM;
    let prcr = cfg_select! {
        all(trust_zone_v2, secure) => system.prcr_s(),
        _ => system.prcr()
    };

    #[cfg(secure)]
    {
        prcr.modify(|r| {
            r.set_prkey(crate::pac::system::vals::Prkey::ProtectKey);
            r.set_prc4(Prc4::NotProtected);
        });

        cpscu
            .dtcsar()
            .write(|r| r.set_dtcstsa(SecurityAttribution::Secure));

        // RA4L1 § 12.2.7 The Secure Attribute managed within the Arm CPU NVIC must match the security
        // attribution of the IELSEn (0..=31). NVIC internal registers are in NVIC_ITNSn[31::0].
        // The initial values of NVIC_ITNSn and ICUSARn are different.  NVIC_ITNSn is secure and ICUSARn
        // is non-secure. Polarity has the same meaning so program these to match.
        //
        // The most helpful tidbit is conspicuously missing from the RA8M1 manual…
        //
        // Until we move off of cortex-m 0.7 we can't even access NVIC_ITNS sooooooo.
        cpscu.icusarg().write_value(Icusar(0x0000_0000));

        #[cfg(any(ra6, ra8))]
        {
            cpscu.icusarh().write_value(Icusar(0x0000_0000));
            cpscu.icusari().write_value(Icusar(0x0000_0000));
        }

        prcr.modify(|r| {
            r.set_prkey(crate::pac::system::vals::Prkey::ProtectKey);
            r.set_prc4(Prc4::Protected);
        });
    }

    {
        //         let cpscu = pac::CPSCU;
        //         debug!(
        //             r#"
        // ===== Current security attributions:
        //   DMAC: {}
        //   DTC: {}
        //   BUS: {}, {}
        //   SRAM: {}
        //   CACHE: {}
        //   BMPU: {}, {}
        //   TZ_FILTER: {}
        // =====
        // "#,
        //             cpscu.dmacsar().read(),
        //             cpscu.dtcsar().read(),
        //             cpscu.bussara().read(),
        //             cpscu.bussarb().read(),
        //             cpscu.sramsar().read(),
        //             cpscu.csar().read(),
        //             cpscu.mmpusara().read(),
        //             cpscu.mmpusarb().read(),
        //             cpscu.tzfsar().read(),
        //         );

        // debug!(r#"ICU: {}"#, cpscu.icusarg().read());
        // debug!(r#"ICU: {}"#, cpscu.icusarh().read());
        // debug!(r#"ICU: {}"#, cpscu.icusari().read());

        // let rmpu = pac::RMPU;
        // for i in 0..7 {
        //     info!(" AccessControl: {}", rmpu.mmpuacdmac(i).read());
        //     info!("         Start: {:x}", rmpu.mmpusdmac(i).read());
        //     info!("           End: {:x}", rmpu.mmpuedmac(i).read());
        // }
    }
}

/// Initializes the MCU.
///
/// # Returns
///
/// The available peripherals as a [`Peripherals`] struct.
pub fn init(clocks: clock::ClockConfig) -> Peripherals {
    critical_section::with(|cs| {
        debug!("Starting board init");

        let mcu_info = McuInfo::info();

        #[cfg(feature = "defmt")]
        mcu_info.print_info();

        // Check if the crate was configured correctly
        mcu_info.validate_package();

        {
            let osm = pac::OSM;
            debug!("OFS0: {}", osm.ofs0().read());
            debug!("OFS1: {}", osm.ofs1().read());
        }

        if clock::init(clocks).is_err() {
            panic!("Clocks were already initialized?");
        }
        info!("{}", clock::clock_status());

        let p = Peripherals::take_with_cs(cs);

        #[cfg(feature = "time-driver")]
        time_driver::init();

        event_link::init();

        dtc::init();

        #[cfg(dmac)]
        dmac::init();

        // This should be conditional on the presence of TZ not a specific peripheral.
        #[cfg(pscu)]
        trust_zone_init();

        p
    })
}

// NOTE: this macro can't be in `embassy-hal-internal` due to the use of `$crate`.
/// Macro to bind interrupts to interrupt handlers.
///
/// For example:
///
/// ```rust,ignore
/// use ra_hal::{bind_interrupts, peripherals::SCI1, uart};
///
/// bind_interrupts!(struct Irqs {
///     IEL2 => uart::RxInterruptHandler<SCI1>;
///     IEL3 => uart::TxInterruptHandler<SCI1>;
///     IEL4 => uart::TeInterruptHandler<SCI1>;
/// });
///```
///
/// Any interrupt `IEL2..=IEL31` can be assigned to any one handler.
/// Note that `IEL0` and `IEL1` are used by the [time driver](crate::time_driver) and are unavailable for general use.
#[macro_export]
macro_rules! bind_interrupts {
    ($(#[$outer:meta])* $vis:vis struct $name:ident {
        $(
            $(#[doc = $doc:literal])*
            $(#[cfg($cond_irq:meta)])?
            $irq:ident => $(
                $(#[cfg($cond_handler:meta)])?
                $handler:ty
            ),*;
        )*
    }) => {
        #[derive(Copy, Clone)]
        $(#[$outer])*
        $vis struct $name;

        $(
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            $(#[cfg($cond_irq)])?
            $(#[doc = $doc])*
            unsafe extern "C" fn $irq() {
                unsafe {
                    $(
                        $(#[cfg($cond_handler)])?
                        <$handler as $crate::interrupt::typelevel::Handler<$crate::interrupt::typelevel::$irq>>::on_interrupt();

                    )*
                }
            }

            $(#[cfg($cond_irq)])?
            $crate::bind_interrupts!(@inner
                $(
                    $(#[cfg($cond_handler)])?
                    unsafe impl $crate::interrupt::typelevel::Binding<$crate::interrupt::typelevel::$irq, $handler> for $name {}
                )*
            );
        )*
    };
    (@inner $($t:tt)*) => {
        $($t)*
    }
}

// include!(concat!(env!("OUT_DIR"), "/pin_traits.rs"));
include!(concat!(env!("OUT_DIR"), "/interrupts.rs"));
include!(concat!(env!("OUT_DIR"), "/peripherals.rs"));
include!(concat!(env!("OUT_DIR"), "/module_stops.rs"));

#[allow(missing_docs)]
pub mod constants {
    include!(concat!(env!("OUT_DIR"), "/constants.rs"));
}

#[cfg(not(feature = "skip-osm"))]
mod _osm_config {
    use crate::osm::{ofs0::Ofs0, ofs1::Ofs1, sec_mpu::SecurityMpu};

    // Option Function Select Register 0
    #[unsafe(no_mangle)]
    #[unsafe(link_section = ".ofs0")]
    static OFS0: Ofs0 = Ofs0::default();

    // Option Function Select Register 1
    #[unsafe(no_mangle)]
    #[unsafe(link_section = ".ofs1")]
    static OFS1: Ofs1 = Ofs1::default();

    // Security MPU
    #[unsafe(no_mangle)]
    #[unsafe(link_section = ".sec_mpu")]
    static SEC_MPU: SecurityMpu = SecurityMpu::disabled();
}

/// Calls the semihosting exit hook.
/// Will cause an exception without a semihosting implementation on the host side.
#[macro_export]
macro_rules! exit {
    () => {
        unsafe {
            cortex_m::asm::semihosting_syscall(0x18, 0x20026);
        };
    };
}
