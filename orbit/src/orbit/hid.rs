use defmt::*;
use embassy_futures::join::join;
use embassy_time::Timer;
use embassy_usb::class::hid::Config as HidConfig;
use embassy_usb::class::hid::{HidReader, HidReaderWriter, HidWriter, State};
use embassy_usb::driver::Driver;
use embassy_usb::Builder;
use embassy_usb::Config;
use embassy_usb::UsbDevice;
use static_cell::StaticCell;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

use crate::orbit::config as OrbitConfig;
use crate::orbit::handlers::{usb_ready, KeyboardDeviceHandler, KeyboardRequestHandler};

pub const MAX_POWER: u16 = 500; // mA // could get this from config
pub const READ_N: usize = 1;
pub const WRITE_N: usize = 8;
const USB_RETRY_TIME: u64 = 100; // ms

pub struct Hid {}

impl Hid {
  pub async fn ready() {
    while !usb_ready() {
      Timer::after_millis(USB_RETRY_TIME).await;
    }
  }

  fn create_builder<D: Driver<'static>>(driver: D) -> Builder<'static, D> {
    static CONFIG_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static MSOS_DESC: StaticCell<[u8; 128]> = StaticCell::new();
    static CONTROL_BUF: StaticCell<[u8; 128]> = StaticCell::new();

    // Create embassy-usb Config
    let mut config = Config::new(
      0x16c0, // VID 5824 (0x16c0) | For USB Keyboards
      0x27db, // PID 10203 (0x27db) | For USB Keyboards
    );
    config.manufacturer = Some(OrbitConfig::MANUFACTURER);
    config.product = Some(OrbitConfig::NAME);
    config.serial_number = Some(OrbitConfig::SERIAL_NUMBER);
    config.max_power = MAX_POWER;
    config.max_packet_size_0 = 64;

    // Required for windows compatibility.
    // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    Builder::new(
      driver,
      config,
      // Create embassy-usb DeviceBuilder using the driver and config.
      // It needs some buffers for building the descriptors.
      &mut CONFIG_DESC.init([0; 256])[..],
      &mut BOS_DESC.init([0; 256])[..],
      // You can also add a Microsoft OS descriptor.
      &mut MSOS_DESC.init([0; 128])[..],
      &mut CONTROL_BUF.init([0; 128])[..],
    )
  }

  pub async fn init<D: Driver<'static>>(
    driver: D,
  ) -> (
    UsbDevice<'static, D>,
    HidReader<'static, D, READ_N>,
    HidWriter<'static, D, WRITE_N>,
  ) {
    static STATE: StaticCell<State> = StaticCell::new();
    static REQUEST_HANDLER: StaticCell<KeyboardRequestHandler> = StaticCell::new();
    static DEVICE_HANDLER: StaticCell<KeyboardDeviceHandler> = StaticCell::new();

    let state = STATE.init(State::new());
    let request_handler = REQUEST_HANDLER.init(KeyboardRequestHandler {});
    let device_handler = DEVICE_HANDLER.init(KeyboardDeviceHandler::new());

    let mut builder = Hid::create_builder::<D>(driver);
    builder.handler(device_handler);

    let config = HidConfig {
      report_descriptor: KeyboardReport::desc(),
      request_handler: None,
      poll_ms: 60,
      max_packet_size: 8,
    };

    let hid = HidReaderWriter::<D, READ_N, WRITE_N>::new(&mut builder, state, config);

    let mut usb = builder.build();
    let (reader, writer) = hid.split();

    (usb, reader, writer)
  }
}
