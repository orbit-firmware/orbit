use crate::orbit::hid::MAX_POWER;
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_usb::class::hid::{ReportId, RequestHandler};
use embassy_usb::control::OutResponse;
use embassy_usb::Handler;

static USB_READY: AtomicBool = AtomicBool::new(false);

use crate::orbit::dbg::info;

pub fn usb_ready() -> bool {
  USB_READY.load(Ordering::SeqCst)
}

pub struct KeyboardRequestHandler {}

impl RequestHandler for KeyboardRequestHandler {
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

pub struct KeyboardDeviceHandler {
  configured: AtomicBool,
}

impl KeyboardDeviceHandler {
  pub fn new() -> Self {
    KeyboardDeviceHandler {
      configured: AtomicBool::new(false),
    }
  }
}

impl Handler for KeyboardDeviceHandler {
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
    info!("Bus reset, the Vbus current limit is {}mA", MAX_POWER);
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
      info!(
        "Device configured, it may now draw up to {}mA current limit from Vbus.",
        MAX_POWER
      )
    } else {
      if USB_READY.load(Ordering::SeqCst) {
        USB_READY.store(false, Ordering::SeqCst);
      }
      info!(
        "Device is no longer configured, the Vbus current limit is {}mA.",
        MAX_POWER
      );
    }
  }
}
