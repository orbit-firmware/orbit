use core::array::from_fn as populate;
use core::option::Option;
use embassy_usb::class::hid::HidWriter;
use embassy_usb::driver::Driver;

use crate::orbit::config as Orbit;
use crate::orbit::dbg::warn;
use crate::orbit::hid::keyboard::{Report, WRITE_N};
use crate::orbit::keycodes::KeyCode;
use crate::orbit::modifiers::*;

use super::dbg::info;

const BUFFER_SIZE: usize = 16; // propably enough

struct Code {
  code: u16,
  keycode: u8,
  modifier: u8,
  sent_once: bool,
  delete: bool,
}

impl Code {
  fn new(code: u16) -> Code {
    let modifier = get_modifier_u8(code);
    let keycode = code as u8;
    Code {
      code,
      keycode,
      modifier,
      sent_once: false,
      delete: false,
    }
  }
}

pub struct Reports {
  codes: [Option<Code>; Orbit::KEY_COUNT],
  reports: [Option<Report>; BUFFER_SIZE],
}

impl Reports {
  pub fn new() -> Reports {
    Reports {
      codes: populate(|_| None),
      reports: populate(|_| None),
    }
  }

  pub fn add(&mut self, keycode: u16) {
    for (i, report) in self.codes.iter().enumerate() {
      if report.is_none() {
        self.codes[i] = Some(Code::new(keycode));
        break;
      }
    }
  }

  pub fn remove(&mut self, keycode: u16) {
    for (i, code_opt) in self.codes.iter_mut().enumerate() {
      if let Some(ref mut code) = code_opt {
        if code.code == keycode {
          code.delete = true;
          break;
        }
      }
    }
  }

  fn get_buf(&self, code: &Code) -> Option<(usize, usize)> {
    for (report_index, report) in self.reports.iter().enumerate() {
      if report.is_none() {
        return Some((report_index, 0));
      }

      if let Some(ref report) = report {
        if report.modifier == code.modifier {
          if let Some(keycode_index) = report.keycodes.iter().position(|&x| x == 0) {
            return Some((report_index, keycode_index));
          }
        }
      }
    }

    None
  }

  fn fill(&mut self) -> bool {
    let mut any = false;
    let codes = &self.codes;

    for (i, code) in codes.iter().enumerate() {
      if let Some(code) = code {
        if let Some((report_index, keycode_index)) = self.get_buf(code) {
          // create buffer if it doesn't exist
          if self.reports[report_index].is_none() {
            let mut report = Report::default();
            report.modifier = code.modifier;
            self.reports[report_index] = Some(report);
          }

          if let Some(ref mut report) = self.reports[report_index] {
            report.keycodes[keycode_index] = code.keycode;
            any = true;
          }
        }
      }
    }

    any
  }

  pub async fn process<D: Driver<'static>>(&mut self, writer: &mut HidWriter<'static, D, WRITE_N>) {
    for (i, code_opt) in self.codes.iter_mut().enumerate() {
      if let Some(ref mut code) = code_opt {
        if code.delete && code.sent_once {
          self.codes[i] = None;
          break;
        }
        code.sent_once = true;
      }
    }

    let any: bool = self.fill();

    // send empty report if no keycodes are pressed
    if !any {
      self.reports[0] = Some(Report::default());
    }

    for report_opt in self.reports.iter_mut() {
      if let Some(ref mut report) = report_opt {
        match writer.write(&report.serialize()).await {
          Ok(()) => {}
          Err(e) => warn!("Failed to send report: {:?}", e),
        }
      }
    }

    self.reports = populate(|_| None);
  }
}
