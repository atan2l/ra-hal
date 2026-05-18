#![no_std]
#![no_main]

use defmt::unwrap;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::{Builder, Config, UsbDevice};
use panic_probe as _;
use ra_hal::usb::Driver;
use ra_hal::{
    adc::{Adc, AdcConfig, AdcPin},
    bind_interrupts,
    clock::ClockConfig,
    debug, error,
    gpio::{DriveCapacity, Level, Output},
    info,
    peripherals::USBFS,
    trace,
    usb::UsbInterruptHandler,
    warn,
};
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    IEL2 => UsbInterruptHandler<USBFS>;
});

static STATE: StaticCell<State> = StaticCell::new();
static CFG_BUF: StaticCell<[u8; 256]> = StaticCell::new();
static BOS_BUF: StaticCell<[u8; 256]> = StaticCell::new();
static CTR_BUF: StaticCell<[u8; 64]> = StaticCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra_hal::init(ClockConfig::default());
    info!("Hello, world!");

    let usb_driver = Driver::new(p.USBFS, p.P914, p.P915, p.P407, Irqs);
    let usb_config = Config::new(0xDEAD, 0xC0DE);

    let cfg_buf = CFG_BUF.init([0; 256]);
    let bos_buf = BOS_BUF.init([0; 256]);
    let ctr_buf = CTR_BUF.init([0; 64]);

    let mut usb_builder = Builder::new(usb_driver, usb_config, cfg_buf, bos_buf, &mut [], ctr_buf);
    let cdc_acm = CdcAcmClass::new(&mut usb_builder, STATE.init(State::new()), 64);
    let usb = usb_builder.build();

    _spawner.spawn(unwrap!(usb_task(usb)));
    _spawner.spawn(unwrap!(echo_task(cdc_acm)));
}

#[embassy_executor::task]
async fn usb_task(mut usb: UsbDevice<'static, Driver<'static, USBFS>>) {
    usb.run().await;
}

#[embassy_executor::task]
async fn echo_task(mut class: CdcAcmClass<'static, Driver<'static, USBFS>>) {
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
