use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};
use defmt::info;
use embassy_futures::join::join;
use embassy_time::Timer;
use embassy_usb::class::hid::Config as HidConfig;
use embassy_usb::class::hid::{HidReader, HidReaderWriter, HidWriter, State};
use embassy_usb::class::hid::{ReportId, RequestHandler};
use embassy_usb::control::OutResponse;
use embassy_usb::driver::Driver;
use embassy_usb::Builder;
use embassy_usb::Config;
use embassy_usb::Handler;
use embassy_usb::UsbDevice;
use static_cell::StaticCell;

use crate::orbit::hid::builder as HID;
use crate::orbit::hid::builder::{HIDDeviceHandler, HIDRequestHandler};

static INITIIALIZED: AtomicBool = AtomicBool::new(false);
static mut DEVICE: UnsafeCell<Option<HIDDeviceHandler>> = UnsafeCell::new(None);
static mut REQUEST: UnsafeCell<Option<HIDRequestHandler>> = UnsafeCell::new(None);

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

pub async fn ready() -> bool {
  unsafe {
    while !(*DEVICE.get()).as_ref().unwrap().is_ready() {
      Timer::after_millis(USB_RETRY_TIME).await;
    }

    true
  }
}

pub async fn init<D: Driver<'static>>(
  driver: D,
) -> (
  UsbDevice<'static, D>,
  HidReader<'static, D, READ_N>,
  HidWriter<'static, D, WRITE_N>,
) {
  unsafe {
    if !INITIIALIZED.load(Ordering::SeqCst) {
      INITIIALIZED.store(true, Ordering::SeqCst);
      (*DEVICE.get()) = Some(HIDDeviceHandler::new());
      (*REQUEST.get()) = Some(HIDRequestHandler {});
    }

    static STATE: StaticCell<State> = StaticCell::new();
    let state = STATE.init(State::new());

    // 10203 (0x27db) | 5824 (0x16c0) | For USB Keyboards
    // https://github.com/obdev/v-usb/blob/master/usbdrv/USB-IDs-for-free.txt
    let mut builder = HID::build::<D>(driver, 0x27db, 0x16c0);
    builder.handler((*DEVICE.get()).as_mut().unwrap());

    let config = HidConfig {
      report_descriptor: HID_REPORT_DESCRIPTOR,
      request_handler: Some((*REQUEST.get()).as_mut().unwrap()),
      poll_ms: 60,
      max_packet_size: 8,
    };

    let hid = HidReaderWriter::<D, READ_N, WRITE_N>::new(&mut builder, state, config);

    let mut usb = builder.build();
    let (reader, writer) = hid.split();

    return (usb, reader, writer);
  }
}
