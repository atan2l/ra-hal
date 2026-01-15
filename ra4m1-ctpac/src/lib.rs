#![doc = include_str!("../README.md")]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (887077d 2026-01-05))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - ICU Interrupt 0"]
    IEL0 = 0,
    #[doc = "1 - ICU Interrupt 1"]
    IEL1 = 1,
    #[doc = "2 - ICU Interrupt 2"]
    IEL2 = 2,
    #[doc = "3 - ICU Interrupt 3"]
    IEL3 = 3,
    #[doc = "4 - ICU Interrupt 4"]
    IEL4 = 4,
    #[doc = "5 - ICU Interrupt 5"]
    IEL5 = 5,
    #[doc = "6 - ICU Interrupt 6"]
    IEL6 = 6,
    #[doc = "7 - ICU Interrupt 7"]
    IEL7 = 7,
    #[doc = "8 - ICU Interrupt 8"]
    IEL8 = 8,
    #[doc = "9 - ICU Interrupt 9"]
    IEL9 = 9,
    #[doc = "10 - ICU Interrupt 10"]
    IEL10 = 10,
    #[doc = "11 - ICU Interrupt 11"]
    IEL11 = 11,
    #[doc = "12 - ICU Interrupt 12"]
    IEL12 = 12,
    #[doc = "13 - ICU Interrupt 13"]
    IEL13 = 13,
    #[doc = "14 - ICU Interrupt 14"]
    IEL14 = 14,
    #[doc = "15 - ICU Interrupt 15"]
    IEL15 = 15,
    #[doc = "16 - ICU Interrupt 16"]
    IEL16 = 16,
    #[doc = "17 - ICU Interrupt 17"]
    IEL17 = 17,
    #[doc = "18 - ICU Interrupt 18"]
    IEL18 = 18,
    #[doc = "19 - ICU Interrupt 19"]
    IEL19 = 19,
    #[doc = "20 - ICU Interrupt 20"]
    IEL20 = 20,
    #[doc = "21 - ICU Interrupt 21"]
    IEL21 = 21,
    #[doc = "22 - ICU Interrupt 22"]
    IEL22 = 22,
    #[doc = "23 - ICU Interrupt 23"]
    IEL23 = 23,
    #[doc = "24 - ICU Interrupt 24"]
    IEL24 = 24,
    #[doc = "25 - ICU Interrupt 25"]
    IEL25 = 25,
    #[doc = "26 - ICU Interrupt 26"]
    IEL26 = 26,
    #[doc = "27 - ICU Interrupt 27"]
    IEL27 = 27,
    #[doc = "28 - ICU Interrupt 28"]
    IEL28 = 28,
    #[doc = "29 - ICU Interrupt 29"]
    IEL29 = 29,
    #[doc = "30 - ICU Interrupt 30"]
    IEL30 = 30,
    #[doc = "31 - ICU Interrupt 31"]
    IEL31 = 31,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "Factory MCU Information Flash Root Table (FMIFRT)"]
pub const FMIFRT: fmifrt::Fmifrt = unsafe { fmifrt::Fmifrt::from_ptr(0x0100_3c00usize as _) };
#[doc = "Bus Master MPU"]
pub const MMPU: mmpu::Mmpu = unsafe { mmpu::Mmpu::from_ptr(0x4000_0000usize as _) };
#[doc = "Bus Slave MPU"]
pub const SMPU: smpu::Smpu = unsafe { smpu::Smpu::from_ptr(0x4000_0c00usize as _) };
#[doc = "CPU Stack Pointer Monitor"]
pub const SPMON: spmon::Spmon = unsafe { spmon::Spmon::from_ptr(0x4000_0d00usize as _) };
#[doc = "SRAM Control"]
pub const SRAM: sram::Sram = unsafe { sram::Sram::from_ptr(0x4000_2000usize as _) };
#[doc = "BUS Control"]
pub const BUS: bus::Bus = unsafe { bus::Bus::from_ptr(0x4000_3000usize as _) };
#[doc = "Direct memory access controller 0"]
pub const DMAC0: dmac::Dmac = unsafe { dmac::Dmac::from_ptr(0x4000_5000usize as _) };
#[doc = "Direct memory access controller 1"]
pub const DMAC1: dmac::Dmac = unsafe { dmac::Dmac::from_ptr(0x4000_5040usize as _) };
#[doc = "Direct memory access controller 2"]
pub const DMAC2: dmac::Dmac = unsafe { dmac::Dmac::from_ptr(0x4000_5080usize as _) };
#[doc = "Direct memory access controller 3"]
pub const DMAC3: dmac::Dmac = unsafe { dmac::Dmac::from_ptr(0x4000_50c0usize as _) };
#[doc = "DMAC Module Activation"]
pub const DMA: dma::Dma = unsafe { dma::Dma::from_ptr(0x4000_5200usize as _) };
#[doc = "Data Transfer Controller"]
pub const DTC: dtc::Dtc = unsafe { dtc::Dtc::from_ptr(0x4000_5400usize as _) };
#[doc = "Interrupt Controller"]
pub const ICU: icu::Icu = unsafe { icu::Icu::from_ptr(0x4000_6000usize as _) };
#[doc = "Debug Function"]
pub const DBG: dbg::Dbg = unsafe { dbg::Dbg::from_ptr(0x4001_b000usize as _) };
#[doc = "Flash Cache"]
pub const FCACHE: fcache::Fcache = unsafe { fcache::Fcache::from_ptr(0x4001_c000usize as _) };
#[doc = "System Control"]
pub const SYSTEM: system::System = unsafe { system::System::from_ptr(0x4001_e000usize as _) };
#[doc = "Port 0 Control Registers"]
pub const PORT0: port0::Port0 = unsafe { port0::Port0::from_ptr(0x4004_0000usize as _) };
#[doc = "Port 1 Control Registers"]
pub const PORT1: port1::Port1 = unsafe { port1::Port1::from_ptr(0x4004_0020usize as _) };
#[doc = "Port 2 Control Registers"]
pub const PORT2: port1::Port1 = unsafe { port1::Port1::from_ptr(0x4004_0040usize as _) };
#[doc = "Port 3 Control Registers"]
pub const PORT3: port1::Port1 = unsafe { port1::Port1::from_ptr(0x4004_0060usize as _) };
#[doc = "Port 4 Control Registers"]
pub const PORT4: port1::Port1 = unsafe { port1::Port1::from_ptr(0x4004_0080usize as _) };
#[doc = "Port 5 Control Registers"]
pub const PORT5: port0::Port0 = unsafe { port0::Port0::from_ptr(0x4004_00a0usize as _) };
#[doc = "Port 6 Control Registers"]
pub const PORT6: port0::Port0 = unsafe { port0::Port0::from_ptr(0x4004_00c0usize as _) };
#[doc = "Port 7 Control Registers"]
pub const PORT7: port0::Port0 = unsafe { port0::Port0::from_ptr(0x4004_00e0usize as _) };
#[doc = "Port 8 Control Registers"]
pub const PORT8: port0::Port0 = unsafe { port0::Port0::from_ptr(0x4004_0100usize as _) };
#[doc = "Port 9 Control Registers"]
pub const PORT9: port0::Port0 = unsafe { port0::Port0::from_ptr(0x4004_0120usize as _) };
#[doc = "Pmn Pin Function Control Register"]
pub const PFS: pfs::Pfs = unsafe { pfs::Pfs::from_ptr(0x4004_0800usize as _) };
#[doc = "Miscellaneous Port Control Register"]
pub const PMISC: pmisc::Pmisc = unsafe { pmisc::Pmisc::from_ptr(0x4004_0d00usize as _) };
#[doc = "Event Link Controller"]
pub const ELC: elc::Elc = unsafe { elc::Elc::from_ptr(0x4004_1000usize as _) };
#[doc = "Port Output Enable Module for GPT"]
pub const POEG: poeg::Poeg = unsafe { poeg::Poeg::from_ptr(0x4004_2000usize as _) };
#[doc = "Realtime Clock"]
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4004_4000usize as _) };
#[doc = "Watchdog Timer"]
pub const WDT: wdt::Wdt = unsafe { wdt::Wdt::from_ptr(0x4004_4200usize as _) };
#[doc = "Independent Watchdog Timer"]
pub const IWDT: iwdt::Iwdt = unsafe { iwdt::Iwdt::from_ptr(0x4004_4400usize as _) };
#[doc = "Clock Frequency Accuracy Measurement Circuit"]
pub const CAC: cac::Cac = unsafe { cac::Cac::from_ptr(0x4004_4600usize as _) };
#[doc = "Module Stop Control B,C,D"]
pub const MSTP: mstp::Mstp = unsafe { mstp::Mstp::from_ptr(0x4004_7000usize as _) };
#[doc = "Serial Sound Interface Ver.2.0"]
pub const SSIE0: ssie0::Ssie0 = unsafe { ssie0::Ssie0::from_ptr(0x4004_e000usize as _) };
#[doc = "CAN0 Module"]
pub const CAN0: can0::Can0 = unsafe { can0::Can0::from_ptr(0x4005_0000usize as _) };
#[doc = "Inter-Integrated Circuit 0"]
pub const IIC0: iic0::Iic0 = unsafe { iic0::Iic0::from_ptr(0x4005_3000usize as _) };
#[doc = "Inter-Integrated Circuit 1"]
pub const IIC1: iic1::Iic1 = unsafe { iic1::Iic1::from_ptr(0x4005_3100usize as _) };
#[doc = "Data Operation Circuit"]
pub const DOC: doc::Doc = unsafe { doc::Doc::from_ptr(0x4005_4100usize as _) };
#[doc = "14bit A/D Converter"]
pub const ADC14: adc14::Adc14 = unsafe { adc14::Adc14::from_ptr(0x4005_c000usize as _) };
#[doc = "12-bit D/A converter"]
pub const DAC12: dac12::Dac12 = unsafe { dac12::Dac12::from_ptr(0x4005_e000usize as _) };
#[doc = "Serial Communication Interface 0"]
pub const SCI0: sci0::Sci0 = unsafe { sci0::Sci0::from_ptr(0x4007_0000usize as _) };
#[doc = "Serial Communication Interface 1"]
pub const SCI1: sci0::Sci0 = unsafe { sci0::Sci0::from_ptr(0x4007_0020usize as _) };
#[doc = "Serial Communication Interface 2"]
pub const SCI2: sci2::Sci2 = unsafe { sci2::Sci2::from_ptr(0x4007_0040usize as _) };
#[doc = "Serial Communication Interface 9"]
pub const SCI9: sci2::Sci2 = unsafe { sci2::Sci2::from_ptr(0x4007_0120usize as _) };
#[doc = "Serial Peripheral Interface 0"]
pub const SPI0: spi0::Spi0 = unsafe { spi0::Spi0::from_ptr(0x4007_2000usize as _) };
#[doc = "Serial Peripheral Interface 1"]
pub const SPI1: spi1::Spi1 = unsafe { spi1::Spi1::from_ptr(0x4007_2100usize as _) };
#[doc = "CRC Calculator"]
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4007_4000usize as _) };
#[doc = "General PWM Timer 0 (32-bit)"]
pub const GPT32_0: gpt32::Gpt32 = unsafe { gpt32::Gpt32::from_ptr(0x4007_8000usize as _) };
#[doc = "General PWM Timer 1 (32-bit)"]
pub const GPT32_1: gpt32::Gpt32 = unsafe { gpt32::Gpt32::from_ptr(0x4007_8100usize as _) };
#[doc = "General PWM Timer 2 (16-bit)"]
pub const GPT16_2: gpt16::Gpt16 = unsafe { gpt16::Gpt16::from_ptr(0x4007_8200usize as _) };
#[doc = "General PWM Timer 3 (16-bit)"]
pub const GPT16_3: gpt16::Gpt16 = unsafe { gpt16::Gpt16::from_ptr(0x4007_8300usize as _) };
#[doc = "General PWM Timer 4 (16-bit)"]
pub const GPT16_4: gpt16::Gpt16 = unsafe { gpt16::Gpt16::from_ptr(0x4007_8400usize as _) };
#[doc = "General PWM Timer 5 (16-bit)"]
pub const GPT16_5: gpt16::Gpt16 = unsafe { gpt16::Gpt16::from_ptr(0x4007_8500usize as _) };
#[doc = "General PWM Timer 6 (16-bit)"]
pub const GPT16_6: gpt16::Gpt16 = unsafe { gpt16::Gpt16::from_ptr(0x4007_8600usize as _) };
#[doc = "General PWM Timer 7 (16-bit)"]
pub const GPT16_7: gpt16::Gpt16 = unsafe { gpt16::Gpt16::from_ptr(0x4007_8700usize as _) };
#[doc = "Output Phase Switching Controller"]
pub const GPT_OPS: gpt_ops::GptOps = unsafe { gpt_ops::GptOps::from_ptr(0x4007_8ff0usize as _) };
#[doc = "Key Interrupt Function"]
pub const KINT: kint::Kint = unsafe { kint::Kint::from_ptr(0x4008_0000usize as _) };
#[doc = "Capacitive Touch Sensing Unit"]
pub const CTSU: ctsu::Ctsu = unsafe { ctsu::Ctsu::from_ptr(0x4008_1000usize as _) };
#[doc = "Segment LCD Controller/Driver"]
pub const SLCDC: slcdc::Slcdc = unsafe { slcdc::Slcdc::from_ptr(0x4008_2000usize as _) };
#[doc = "Asynchronous General purpose Timer 0"]
pub const AGT0: agt::Agt = unsafe { agt::Agt::from_ptr(0x4008_4000usize as _) };
#[doc = "Asynchronous General purpose Timer 1"]
pub const AGT1: agt::Agt = unsafe { agt::Agt::from_ptr(0x4008_4100usize as _) };
#[doc = "Low-Power Analog Comparator"]
pub const ACMPLP: acmplp::Acmplp = unsafe { acmplp::Acmplp::from_ptr(0x4008_5e00usize as _) };
#[doc = "OperationalAmplifier"]
pub const OPAMP: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x4008_6000usize as _) };
#[doc = "USB 2.0 FS Module"]
pub const USBFS: usbfs::Usbfs = unsafe { usbfs::Usbfs::from_ptr(0x4009_0000usize as _) };
#[doc = "8-bit D/A converter"]
pub const DAC8: dac8::Dac8 = unsafe { dac8::Dac8::from_ptr(0x4009_e000usize as _) };
#[doc = "Temperature Sensor"]
pub const TSN: tsn::Tsn = unsafe { tsn::Tsn::from_ptr(0x407e_c000usize as _) };
#[doc = "Pointer to the actual registers"]
pub const FMIFRT_BASE: fmifrt_base::FmifrtBase =
    unsafe { fmifrt_base::FmifrtBase::from_ptr(0x407f_b19cusize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub mod acmplp;
pub mod adc14;
pub mod agt;
pub mod bus;
pub mod cac;
pub mod can0;
pub mod common;
pub mod crc;
pub mod ctsu;
pub mod dac12;
pub mod dac8;
pub mod dbg;
pub mod dma;
pub mod dmac;
pub mod doc;
pub mod dtc;
pub mod elc;
pub mod fcache;
pub mod fmifrt;
pub mod fmifrt_base;
pub mod gpt16;
pub mod gpt32;
pub mod gpt_ops;
pub mod icu;
pub mod iic0;
pub mod iic1;
pub mod iwdt;
pub mod kint;
pub mod mmpu;
pub mod mstp;
pub mod opamp;
pub mod pfs;
pub mod pmisc;
pub mod poeg;
pub mod port0;
pub mod port1;
pub mod rtc;
pub mod sci0;
pub mod sci2;
pub mod slcdc;
pub mod smpu;
pub mod spi0;
pub mod spi1;
pub mod spmon;
pub mod sram;
pub mod ssie0;
pub mod system;
pub mod tsn;
pub mod usbfs;
pub mod wdt;
