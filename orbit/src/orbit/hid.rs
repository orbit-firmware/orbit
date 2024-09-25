use core::cell::UnsafeCell;
use core::option::Option;
use core::sync::atomic::{AtomicBool, Ordering};
use static_cell::StaticCell;

use embassy_usb::{
  class::hid::{HidReader, HidReaderWriter, HidWriter, ReportId, RequestHandler, State},
  control::{InResponse, OutResponse},
  driver::Driver,
  Builder, Config, Handler,
};

use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

static USB_READY: AtomicBool = AtomicBool::new(false);

use crate::orbit::dbg::*;

pub struct Hid<D: Driver<'static>> {
  reader: Option<HidReader<'static, D, 1>>,
  writer: Option<HidWriter<'static, D, 8>>,
}

impl<D: Driver<'static>> Hid<D> {
  pub async fn new(driver: D) -> Self {
    let mut hid = Hid {
      reader: None,
      writer: None,
    };

    hid.configure(driver).await;

    hid
  }

  pub fn usb_ready(&self) -> bool {
    USB_READY.load(Ordering::SeqCst)
  }

  async fn configure(&mut self, driver: D) {
    let mut config = Config::new(
      0x16c0, // VID 5824 (0x16c0) | For USB Keyboards
      0x27db, // PID 10203 (0x27db) | For USB Keyboards
    );
    // TODO:
    config.manufacturer = Some("Embassy");
    config.product = Some("HID keyboard example");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Required for windows compatibility.
    // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    // need a few statics to ensure the buffers are available for the lifetime of the USB device.
    static STATE: StaticCell<State> = StaticCell::new();
    static REQUEST_HANDLER: StaticCell<KeyboardRequest> = StaticCell::new();
    static DEVICE_HANDLER: StaticCell<KeyboardDevice> = StaticCell::new();
    static CONFIG_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static MSOS_DESC: StaticCell<[u8; 128]> = StaticCell::new();
    static CONTROL_BUF: StaticCell<[u8; 128]> = StaticCell::new();

    #[rustfmt::skip]
    let mut builder = Builder::new(
      driver,
      config,
      &mut CONFIG_DESC.init([0; 256])[..],
      &mut BOS_DESC.init([0; 256])[..],
      &mut MSOS_DESC.init([0; 128])[..],
      &mut CONTROL_BUF.init([0; 128])[..],
    );

    builder.handler(DEVICE_HANDLER.init(KeyboardDevice::new()));

    let config = embassy_usb::class::hid::Config {
      report_descriptor: KeyboardReport::desc(),
      request_handler: Some(REQUEST_HANDLER.init(KeyboardRequest {})),
      poll_ms: 60,
      max_packet_size: 8,
    };

    #[rustfmt::skip]
    let hid = HidReaderWriter::<_, 1, 8>::new(
      &mut builder,
      STATE.init(State::new()),
      config
    );

    let mut usb = builder.build();

    let (reader, writer) = hid.split();
    self.reader = Some(reader);
    self.writer = Some(writer);

    usb.run().await;
  }
}

struct KeyboardRequest {}

impl RequestHandler for KeyboardRequest {
  fn get_report(&mut self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
    info!("Get report for {:?}", id);
    None
  }

  fn set_report(&mut self, id: ReportId, data: &[u8]) -> OutResponse {
    info!("Set report for {:?}: {=[u8]}", id, data);
    OutResponse::Accepted
  }

  fn set_idle_ms(&mut self, id: Option<ReportId>, dur: u32) {
    info!("Set idle rate for {:?} to {:?}", id, dur);
  }

  fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
    info!("Get idle rate for {:?}", id);
    None
  }
}

struct KeyboardDevice {
  configured: AtomicBool,
}

impl KeyboardDevice {
  fn new() -> Self {
    KeyboardDevice {
      configured: AtomicBool::new(false),
    }
  }
}

impl Handler for KeyboardDevice {
  fn enabled(&mut self, enabled: bool) {
    self.configured.store(false, Ordering::Relaxed);
    if enabled {
      info!("Device enabled");
    } else {
      info!("Device disabled");
    }
  }

  fn reset(&mut self) {
    self.configured.store(false, Ordering::Relaxed);
    info!("Bus reset, the Vbus current limit is 100mA");
  }

  fn addressed(&mut self, addr: u8) {
    self.configured.store(false, Ordering::Relaxed);
    info!("USB address set to: {}", addr);
  }

  fn configured(&mut self, configured: bool) {
    self.configured.store(configured, Ordering::Relaxed);
    if configured {
      if !USB_READY.load(Ordering::SeqCst) {
        USB_READY.store(true, Ordering::SeqCst);
      }
      info!("Device configured, it may now draw up to the configured current limit from Vbus.")
    } else {
      if USB_READY.load(Ordering::SeqCst) {
        USB_READY.store(false, Ordering::SeqCst);
      }
      info!("Device is no longer configured, the Vbus current limit is 100mA.");
    }
  }
}
