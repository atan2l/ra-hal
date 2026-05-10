//! `usb_cdc` demonstrates a USB CDC ACM serial port using `usbd-serial`.
//!
//! The example uses the same proven USB bring-up sequence as the working
//! `ra4_r` test path, then echoes received bytes back to the host.

#![no_std]
#![no_main]
#![warn(missing_docs)]

use core::{
    cell::UnsafeCell,
    fmt::Write as _,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, AtomicU8, Ordering},
};

use cortex_m::asm;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_time::{Duration, block_for};
use heapless::String;
use panic_probe as _;
use ra_hal::{
    bind_interrupts,
    clock::ClockConfig,
    pac,
    peripherals::USBFS,
    usbfs::{self, UsbClockSource},
};
#[allow(unused)]
use ra_hal::{debug, error, info, trace, warn};
use usb_device::{
    bus::UsbBusAllocator,
    device::{StringDescriptors, UsbDeviceBuilder, UsbVidPid},
    prelude::UsbDeviceState,
};
use usbd_serial::SerialPort;

bind_interrupts!(struct Irqs {
    IEL5 => usbfs::InterruptHandler<USBFS>;
});

type HalBus = usbfs::Bus<'static, USBFS>;

static USB_LAST_STATE: AtomicU8 = AtomicU8::new(0xff);
static DTR_ACTIVE: AtomicBool = AtomicBool::new(false);

struct StaticCell<T>(UnsafeCell<MaybeUninit<T>>);

unsafe impl<T> Sync for StaticCell<T> {}

fn init_usb_bus_allocator(bus: HalBus) -> &'static UsbBusAllocator<HalBus> {
    static USB_BUS_ALLOC: StaticCell<UsbBusAllocator<HalBus>> =
        StaticCell(UnsafeCell::new(MaybeUninit::uninit()));

    unsafe {
        let ptr = (*USB_BUS_ALLOC.0.get()).as_mut_ptr();
        ptr.write(UsbBusAllocator::new(bus));
        &*ptr
    }
}

fn encode_state(state: UsbDeviceState) -> u8 {
    match state {
        UsbDeviceState::Default => 0,
        UsbDeviceState::Addressed => 1,
        UsbDeviceState::Configured => 2,
        UsbDeviceState::Suspend => 3,
    }
}

fn decode_state(code: u8) -> Option<UsbDeviceState> {
    match code {
        0 => Some(UsbDeviceState::Default),
        1 => Some(UsbDeviceState::Addressed),
        2 => Some(UsbDeviceState::Configured),
        3 => Some(UsbDeviceState::Suspend),
        _ => None,
    }
}

fn boot_pause() {
    // asm::delay(4_800_000);
    block_for(Duration::from_millis(100));
}

fn attach_settle_pause() {
    block_for(Duration::from_millis(10));
}

fn log_state(state: UsbDeviceState) {
    match state {
        UsbDeviceState::Default => info!("usb state=Default"),
        UsbDeviceState::Addressed => info!("usb state=Addressed"),
        UsbDeviceState::Configured => info!("usb state=Configured"),
        UsbDeviceState::Suspend => info!("usb state=Suspend"),
    }
}
use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());
    boot_pause();

    let usb_config = usbfs::Config {
        force_reset_on_init: false,
        clock_source: UsbClockSource::Pll,
    };

    let driver = usbfs::Driver::new(p.USBFS, Irqs, usb_config, p.P407, p.P815, p.P814).unwrap();
    let bus = usbfs::Bus::new(driver);
    let usb_bus = init_usb_bus_allocator(bus);

    boot_pause();
    let mut serial = SerialPort::new(usb_bus);

    let mcu_info = ra_hal::mcu_info::McuInfo::info();
    assert!(mcu_info.ok());

    let mut sn: String<35> = String::new();
    write!(
        sn,
        "{:08x}-{:08x}-{:08x}-{:08x}",
        (mcu_info.uid() >> 96) & 0xFFFFFFFF,
        (mcu_info.uid() >> 64) & 0xFFFFFFFF,
        (mcu_info.uid() >> 32) & 0xFFFFFFFF,
        mcu_info.uid() & 0xFFFFFFFF
    )
    .unwrap();

    let mut product: String<13> = String::new();
    write!(product, "{} Example", ra_hal::CONFIGURED_MCU).unwrap();

    let mut usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0x4d31))
        .composite_with_iads()
        .max_packet_size_0(64)
        .unwrap()
        .strings(&[StringDescriptors::new(usb_device::LangID::EN_US)
            .manufacturer("ra-hal")
            .product(&product)
            .serial_number(&sn)])
        .unwrap()
        .build();

    usb_dev.bus().driver().enable_interrupts();
    usb_dev.force_reset().expect("force reset must succeed");

    attach_settle_pause();

    let mut last_state = UsbDeviceState::Default;
    let mut last_dtr = false;
    let mut echo_buf = [0u8; 64];
    let mut echo_len = 0usize;
    let mut echo_off = 0usize;

    loop {
        usb_dev.poll(&mut [&mut serial]);
        USB_LAST_STATE.store(encode_state(usb_dev.state()), Ordering::Relaxed);
        DTR_ACTIVE.store(serial.dtr(), Ordering::Relaxed);

        if let Some(state) = decode_state(USB_LAST_STATE.load(Ordering::Relaxed)) {
            if state != last_state {
                last_state = state;
                log_state(last_state);
            }
        }
        let dtr_active = DTR_ACTIVE.load(Ordering::Relaxed);
        if dtr_active != last_dtr {
            last_dtr = dtr_active;
        }
        if echo_len == 0 {
            if dtr_active {
                match serial.read(&mut echo_buf) {
                    Ok(count) if count > 0 => {
                        echo_len = count;
                        echo_off = 0;
                    }
                    _ => {}
                }
            }
        } else {
            match serial.write(&echo_buf[echo_off..echo_len]) {
                Ok(written) if written > 0 => {
                    echo_off += written;
                    if echo_off == echo_len {
                        echo_len = 0;
                        echo_off = 0;
                    }
                }
                Ok(_) => {}
                Err(usb_device::UsbError::WouldBlock) => {}
                Err(_) => {
                    echo_len = 0;
                    echo_off = 0;
                }
            }
        }
        asm::nop();
    }
}
