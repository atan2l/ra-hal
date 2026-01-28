//! Option Function Select Register 1 (`OFS1`)
//!
//! `OFS1` configures the after-reset behavior of the Low Voltage Detection (`LVDAS`) and High-Speed On-Chip Oscillator (`HOCO`).
//! See §5.3.2, §6.2.2, §7, §8.2.9, and §8.2.10 in the reference manual for more details.

// Reserved bits are to be written as 1.
#[doc(hidden)]
pub const OFS1_H: u32 = 0xFFFF0000;
#[doc(hidden)]
pub const OFS1_L: u32 = 0x00008EC3;

trait Lvdas {
    const OFS1: u32;
}

/// `LVDAS` Enable voltage monitor 0 reset after a reset. (§ 6.2.2).
pub enum LvdasOn {}
impl Lvdas for LvdasOn {
    const OFS1: u32 = 0 << 2;
}

/// `LVDAS` Disable voltage monitor 0 reset after a reset. (§ 6.2.2).
pub enum LvdasOff {}
impl Lvdas for LvdasOff {
    const OFS1: u32 = 1 << 2;
}

trait Vdsel {
    const OFS1: u32;
}

/// `VDSEL` Voltage detection 0 level select 3.84 V.
pub enum Vdsel384 {}
impl Vdsel for Vdsel384 {
    const OFS1: u32 = 0b000 << 3;
}

/// `VDSEL` Voltage detection 0 level select 2.82 V.
pub enum Vdsel282 {}
impl Vdsel for Vdsel282 {
    const OFS1: u32 = 0b001 << 3;
}

/// `VDSEL` Voltage detection 0 level select 2.51 V.
pub enum Vdsel251 {}
impl Vdsel for Vdsel251 {
    const OFS1: u32 = 0b010 << 3;
}

/// `VDSEL` Voltage detection 0 level select 1.90 V.
pub enum Vdsel190 {}
impl Vdsel for Vdsel190 {
    const OFS1: u32 = 0b011 << 3;
}

/// `VDSEL` Voltage detection 0 level select 1.70 V.
pub enum Vdsel170 {}
impl Vdsel for Vdsel170 {
    const OFS1: u32 = 0b100 << 3;
}

trait HocoEnable {
    const OFS1: u32;
}

/// `HOCOEN` Enable High-Speed On-Chip Oscillator (`HOCO`) after a reset. (§ 6.2.2).
pub enum HocoOn {}
impl HocoEnable for HocoOn {
    const OFS1: u32 = 0 << 8;
}

/// `HOCOEN` Disable High-Speed On-Chip Oscillator (`HOCO`) after a reset. (§ 6.2.2).
pub enum HocoOff {}
impl HocoEnable for HocoOff {
    const OFS1: u32 = 1 << 8;
}

trait HocoFrequency {
    const OFS1: u32;
}

/// `HOCOFRQ1` HOCO Frequency 24 MHz. (§ 6.2.2).
pub enum Hoco24Mhz {}
impl HocoFrequency for Hoco24Mhz {
    const OFS1: u32 = 0b000 << 12;
}

/// `HOCOFRQ1` HOCO Frequency 32 MHz. (§ 6.2.2).
pub enum Hoco32Mhz {}
impl HocoFrequency for Hoco32Mhz {
    const OFS1: u32 = 0b010 << 12;
}

/// `HOOFRQ1` HOCO Frequency 48 MHz. (§ 6.2.2).
pub enum Hoco48Mhz {}
impl HocoFrequency for Hoco48Mhz {
    const OFS1: u32 = 0b100 << 12;
}

/// `HOCOFRQ1` HOCO Frequency 64 MHz. (§ 6.2.2).
pub enum Hoco64Mhz {}
impl HocoFrequency for Hoco64Mhz {
    const OFS1: u32 = 0b101 << 12;
}

#[doc(hidden)]
#[allow(private_bounds)]
pub const fn ofs1<LVDAS: Lvdas, VDSEL: Vdsel, HOCOEN: HocoEnable, HOCOFR: HocoFrequency>() -> u32 {
    OFS1_H | OFS1_L | LVDAS::OFS1 | VDSEL::OFS1 | HOCOEN::OFS1 | HOCOFR::OFS1
}

/// Generates a value to configure `OFS1`.
///
/// This macro, along with [`ofs0!`](crate::ofs0!), needs to be invoked once per application.
/// Typical usage:
/// ```rust,ignore
/// #[unsafe(no_mangle)]
/// #[unsafe(link_section = ".ofs1")]
/// pub static OFS1: u32 = ofs1!(ArduinoCore);
/// ```
///
/// It can also be invoked with each configuration item explicitly specified, e.g.:
///
/// ```rust,ignore
/// #[unsafe(no_mangle)]
/// #[unsafe(link_section = ".ofs1")]
/// pub static OFS1: u32 = ofs1!(LvdasOff, Vdsel190, HocoOn, Hoco48Mhz);
/// ```
///
/// See the [module](module@crate::osm::ofs1) docs for the available values.
#[macro_export]
macro_rules! ofs1 {
    (ArduinoCore) => {
        ofs1!(LvdasOff, Vdsel190, HocoOn, Hoco48Mhz);
    };
    ($arg0:ident, $arg1:ident, $arg2:ident, $arg3:ident) => {
        $crate::osm::ofs1::ofs1::<
            $crate::osm::ofs1::$arg0,
            $crate::osm::ofs1::$arg1,
            $crate::osm::ofs1::$arg2,
            $crate::osm::ofs1::$arg3,
        >()
    };
}
