use crate::orbit::dbg::info;
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_usb::class::hid::{ReportId, RequestHandler};
use embassy_usb::control::OutResponse;
use embassy_usb::driver::Driver;
use embassy_usb::{Builder, Config, Handler};
use static_cell::StaticCell;

pub const MAX_POWER: u16 = 500; // mA // could get this from config

pub fn build<D: Driver<'static>>(driver: D, pid: u16, vid: u16) -> Builder<'static, D> {
  static CONFIG_DESC: StaticCell<[u8; 256]> = StaticCell::new();
  static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
  static MSOS_DESC: StaticCell<[u8; 128]> = StaticCell::new();
  static CONTROL_BUF: StaticCell<[u8; 128]> = StaticCell::new();

  // Create embassy-usb Config
  let mut config = Config::new(vid, pid);
  config.manufacturer = Some("feb");
  config.product = Some("feb-keyboard");
  config.serial_number = Some("00000001");
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

pub struct HIDRequestHandler {}

impl RequestHandler for HIDRequestHandler {
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

pub struct HIDDeviceHandler {
  pub ready: bool,
}

impl HIDDeviceHandler {
  pub fn new() -> Self {
    HIDDeviceHandler { ready: false }
  }

  pub fn is_ready(&self) -> bool {
    self.ready
  }
}

impl Handler for HIDDeviceHandler {
  fn enabled(&mut self, enabled: bool) {
    self.ready = false;
    if enabled {
      info!("Device enabled");
    } else {
      info!("Device disabled");
    }
  }

  fn reset(&mut self) {
    self.ready = false;
    info!("Bus reset");
  }

  fn addressed(&mut self, addr: u8) {
    self.ready = false;
    info!("USB address set to: {}", addr);
  }

  fn configured(&mut self, configured: bool) {
    if configured {
      self.ready = true;
      info!("USB Device configured.")
    } else {
      self.ready = false;
      info!("USB Device is no longer configured.",);
    }
  }
}
