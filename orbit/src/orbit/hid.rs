use embassy_futures::join::join;
use embassy_time::Timer;
use embassy_usb::class::hid::Config as HidConfig;
use embassy_usb::class::hid::{HidReader, HidReaderWriter, HidWriter, State};
use embassy_usb::driver::Driver;
use embassy_usb::Builder;
use embassy_usb::Config;
use embassy_usb::UsbDevice;
use static_cell::StaticCell;

use crate::orbit::config as Orbit;
use crate::orbit::dbg::info;
use crate::orbit::handlers::{usb_ready, KeyboardDeviceHandler, KeyboardRequestHandler};

pub const MAX_POWER: u16 = 500; // mA // could get this from config
pub const READ_N: usize = 1;
pub const WRITE_N: usize = 8;

const USB_RETRY_TIME: u64 = 100; // ms

const HID_REPORT_DESCRIPTOR: &[u8] = &[
  0x05, 0x01, // Usage Page (Generic Desktop),
  0x09, 0x06, // Usage (Keyboard),
  0xA1, 0x01, // Collection (Application),
  0x75, 0x01, // Report Size (1),
  0x95, 0x08, // Report Count (8),
  0x05, 0x07, // Usage Page (Key Codes),
  0x19, 0xE0, // Usage Minimum (224),
  0x29, 0xE7, // Usage Maximum (231),
  0x15, 0x00, // Logical Minimum (0),
  0x25, 0x01, // Logical Maximum (1),
  0x81, 0x02, // Input (Data, Variable, Absolute),Modifier byte
  0x95, 0x01, // Report Count (1),
  0x75, 0x08, // Report Size (8),
  0x81, 0x03, // Input (Constant), Reserved byte
  0x95, 0x05, // Report Count (5),
  0x75, 0x01, // Report Size (1),
  0x05, 0x08, // Usage Page (LEDs),
  0x19, 0x01, // Usage Minimum (1),
  0x29, 0x05, // Usage Maximum (5),
  0x91, 0x02, // Output (Data, Variable, Absolute), LED report
  0x95, 0x01, // Report Count (1),
  0x75, 0x03, // Report Size (3),
  0x91, 0x03, // Output (Constant), LED report
  0x95, 0x06, // Report Count (6),
  0x75, 0x08, // Report Size (8),
  0x15, 0x00, // Logical Minimum (0),
  0x25, 0x68, // Logical Maximum(104),
  0x05, 0x07, // Usage Page (Key Codes),
  0x19, 0x00, // Usage Minimum (0),
  0x29, 0x68, // Usage Maximum (104),
  0x81, 0x00, // Input (Data, Array),
  0xc0, // End Collection
];

pub struct Report {
  pub modifier: u8,
  pub reserved: u8,
  pub keycodes: [u8; 6],
}

impl Default for Report {
  fn default() -> Report {
    Report {
      modifier: 0,
      reserved: 0,
      keycodes: [0; 6],
    }
  }
}

impl Report {
  pub fn serialize(&self) -> [u8; 8] {
    let mut buf = [0; 8];
    buf[0] = self.modifier;
    buf[1] = self.reserved;
    buf[2..8].copy_from_slice(&self.keycodes);
    buf
  }
}

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
    config.manufacturer = Some(Orbit::MANUFACTURER);
    config.product = Some(Orbit::NAME);
    config.serial_number = Some(Orbit::SERIAL_NUMBER);
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
      report_descriptor: HID_REPORT_DESCRIPTOR,
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
