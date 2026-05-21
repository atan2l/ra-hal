#![no_std]
#![no_main]

use core::fmt::Write as _;

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_sync::lazy_lock::LazyLock;
use embassy_usb::{
    Builder, Config, UsbDevice,
    class::cdc_acm::{CdcAcmClass, State},
};
use heapless::String;
use panic_probe as _;
#[allow(unused)]
use ra_hal::{assert_eq, debug, error, info, trace, warn};
use ra_hal::{
    bind_interrupts,
    clock::ClockConfig,
    peripherals::USBFS,
    usb::{UsbInterruptHandler, Usbfs},
};
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    IEL2 => UsbInterruptHandler<USBFS>;
});

static STATE: StaticCell<State> = StaticCell::new();
static CFG_BUF: StaticCell<[u8; 256]> = StaticCell::new();
static BOS_BUF: StaticCell<[u8; 256]> = StaticCell::new();
static CTR_BUF: StaticCell<[u8; 128]> = StaticCell::new();

static USB_PRODUCT: LazyLock<String<13>> = LazyLock::new(|| {
    let mut product: String<13> = String::new();
    write!(product, "{} Example", ra_hal::CONFIGURED_MCU).unwrap();
    product
});

static USB_SN: LazyLock<String<35>> = LazyLock::new(|| {
    let mut sn: String<35> = String::new();

    let mcu_info = ra_hal::mcu_info::McuInfo::info();
    assert!(mcu_info.ok());

    write!(
        sn,
        "{:08x}-{:08x}-{:08x}-{:08x}",
        (mcu_info.uid() >> 96) & 0xFFFF_FFFF,
        (mcu_info.uid() >> 64) & 0xFFFF_FFFF,
        (mcu_info.uid() >> 32) & 0xFFFF_FFFF,
        mcu_info.uid() & 0xFFFF_FFFF
    )
    .expect("Buffer too small for serial number");
    sn
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());

    let usb_driver = Usbfs::new(p.USBFS, p.P914, p.P915, p.P407, Irqs);
    let mut usb_config = Config::new(0xDEAD, 0xC0DE);
    usb_config.product = Some(USB_PRODUCT.get().as_str());
    usb_config.serial_number = Some(USB_SN.get().as_str());

    let cfg_buf = CFG_BUF.init([0; 256]);
    let bos_buf = BOS_BUF.init([0; 256]);
    let ctr_buf = CTR_BUF.init([0; 128]);

    let mut usb_builder = Builder::new(usb_driver, usb_config, cfg_buf, bos_buf, &mut [], ctr_buf);
    let cdc_acm = CdcAcmClass::new(&mut usb_builder, STATE.init(State::new()), 64);
    let usb = usb_builder.build();

    spawner.spawn(usb_task(usb).unwrap());
    spawner.spawn(echo_task(cdc_acm).unwrap());
}

#[embassy_executor::task]
async fn usb_task(mut usb: UsbDevice<'static, Usbfs<'static, USBFS>>) {
    usb.run().await;
}

#[embassy_executor::task]
async fn echo_task(mut class: CdcAcmClass<'static, Usbfs<'static, USBFS>>) {
    let mut buf = [0; 64];
    loop {
        class.wait_connection().await;
        info!("CDC: connected");
        while let Ok(n) = class.read_packet(&mut buf).await {
            let data = &buf[..n];
            info!("CDC: rx {:x}", data);
            if class.write_packet(data).await.is_err() {
                break;
            }
        }
        info!("CDC: disconnected");
    }
}
