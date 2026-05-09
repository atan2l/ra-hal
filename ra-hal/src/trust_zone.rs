use crate::pac;

cfg_select! {
    ra8 => {
        /// IDAU regions per the reference manual.
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        #[derive(Copy, Clone, PartialEq)]
        #[repr(u8)]
        enum IdauRegion {
            NonSecureCallableCodeFlash = 0x01,
            NonSecureCodeFlash = 0x02,
            NonSecureCallableSram = 0x03,
            NonSecureSram = 0x04,
            SecurePeripheral = 0x05,
            NonSecureExternalOrPeripheral= 0x06,
            Exempt = 0x00,
        }

        impl From<u8> for IdauRegion {
            fn from(value: u8) -> Self {
                match value {
                    0x01 => Self::NonSecureCallableCodeFlash,
                    0x02 => Self::NonSecureCodeFlash,
                    0x03 => Self::NonSecureCallableSram,
                    0x04 => Self::NonSecureSram,
                    0x05 => Self::SecurePeripheral,
                    0x06 => Self::NonSecureExternalOrPeripheral,
                    0x00 => Self::Exempt,
                    _ => unreachable!(),
                }
            }
        }
    }
    _ => {
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
    }
}

pub fn init() {
    // 4L1 RM: After reset, all of address space is marked as Secure by SAU default setting.
    // SAU_CTRL register should be set to 0x2 to enable the IDAU security attribution. That
    // is, after setting SAU_CTRL register to 0x2, the address space security attribution
    // becomes as shown in Table 48.6.
    // TODO: Handle the SAU in RA8

    #[cfg(secure)]
    use pac::{
        cpscu::{regs::Icusar, vals::SecurityAttribution},
        system::vals::Prc4,
    };

    let pscu = pac::PSCU;
    let cpscu = pac::CPSCU;
    let system = pac::SYSTEM;
    let prcr = cfg_select! {
        all(trust_zone_v2, secure) => system.prcr_s(),
        _ => system.prcr()
    };

    // Enable SAU.  Might as well?
    unsafe {
        (*cortex_m::peripheral::SAU::PTR)
            .ctrl
            .write(cortex_m::peripheral::sau::Ctrl(0x02))
    };

    let idau_region = {
        let mut i = [1];
        let tt =
            cortex_m::cmse::TestTarget::check(i.as_mut_ptr(), cortex_m::cmse::AccessType::Current);
        tt.idau_region().map(IdauRegion::from)
    };
    let life_cycle = pscu.dlmmon().read().dlmmon();

    info!(
        "TrustZone: region={} life_cycle={}",
        idau_region, life_cycle
    );

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
