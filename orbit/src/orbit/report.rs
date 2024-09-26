use core::array::from_fn as populate;
use core::option::Option;
use embassy_usb::class::hid::HidWriter;
use embassy_usb::driver::Driver;
use usbd_hid::descriptor::KeyboardReport;

use crate::orbit::config;
use crate::orbit::dbg::{info, warn};
use crate::orbit::hid::WRITE_N;
use crate::orbit::keycodes::KeyCode;
use crate::orbit::modifiers::*;

use super::keycodes;

const KEY_CODE_REPORT_SIZE: usize = 6;
const BUFFER_SIZE: usize = 16; // propably enough

pub struct Report {
  keycodes: [u16; config::KEY_COUNT],
  buffer: [Option<KeyboardReport>; BUFFER_SIZE],
}

impl Report {
  pub fn new() -> Report {
    Report {
      keycodes: populate(|_| KeyCode::None as u16),
      buffer: populate(|_| None),
    }
  }

  pub fn register_keycode(&mut self, keycode: u16) {
    for code in self.keycodes.iter_mut() {
      if *code == KeyCode::None as u16 {
        *code = keycode;
        break;
      }
    }
  }

  pub fn unregister_keycode(&mut self, keycode: u16) {
    for code in self.keycodes.iter_mut() {
      if *code == keycode {
        *code = KeyCode::None as u16;
        break;
      }
    }
  }

  fn get_free_buffer(&self, modifiers: u8) -> Option<(usize, usize)> {
    for (i, report_opt) in self.buffer.iter().enumerate() {
      if report_opt.is_none() {
        return Some((i, 0));
      } else {
        let report = report_opt.unwrap();
        if report.modifier == modifiers {
          if let Some(free_idx) = report.keycodes.iter().position(|&x| x == 0) {
            return Some((i, free_idx));
          } else {
            continue;
          }
        }
      }
    }
    None
  }

  fn fill_buffers(&mut self) -> bool {
    let mut any_buffer_written = false;
    let keycodes = &self.keycodes;

    for code in keycodes {
      if *code == KeyCode::None as u16 {
        break;
      }

      let mut buffer_idx: Option<usize> = None;
      let mut keycode_idx: Option<usize> = None;

      let modifier = get_modifier_u8(*code);
      if let Some((b, k)) = self.get_free_buffer(modifier) {
        buffer_idx = Some(b);
        keycode_idx = Some(k);
      }

      // create buffer if it doesn't exist
      if let Some(buffer_idx) = buffer_idx {
        if self.buffer[buffer_idx].is_none() {
          self.buffer[buffer_idx] = Some(KeyboardReport {
            keycodes: [0; KEY_CODE_REPORT_SIZE],
            leds: 0,
            modifier,
            reserved: 0,
          });
        }
        if let Some(ref mut report) = self.buffer[buffer_idx] {
          if let Some(keycode_idx) = keycode_idx {
            report.keycodes[keycode_idx] = *code as u8;
            any_buffer_written = true;
            info!("Keycode {:?} added to buffer {:?}", code, buffer_idx);
          }
        }
      }
    }

    any_buffer_written
  }

  pub async fn process<D: Driver<'static>>(&mut self, writer: &mut HidWriter<'static, D, WRITE_N>) {
    if !self.fill_buffers() {
      self.buffer[0] = Some(KeyboardReport {
        keycodes: [0; KEY_CODE_REPORT_SIZE],
        leds: 0,
        modifier: 0,
        reserved: 0,
      });
    }

    for report_opt in self.buffer.iter_mut() {
      if let Some(report) = report_opt {
        match writer.write_serialize(report).await {
          Ok(()) => {}
          Err(e) => warn!("Failed to send report: {:?}", e),
        }
      }
    }

    self.buffer = populate(|_| None);
  }
}
