use core::array::from_fn as populate;
use core::option::Option;
#[cfg(not(feature = "chip_type_emulator"))]
use embassy_usb::{class::hid::HidWriter, driver::Driver};

use crate::orbit::config as Orbit;
use crate::orbit::dbg::warn;
#[cfg(not(feature = "chip_type_emulator"))]
use crate::orbit::hid::keyboard::WRITE_N;
use crate::orbit::modifiers::*;


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

struct Code {
  code: u16,
  sent_once: bool,
  delete: bool,
}

pub struct Reports {
  codes: [Option<Code>; Orbit::KEY_COUNT],
}

impl Reports {
  pub fn new() -> Reports {
    Reports {
      codes: populate(|_| None),
    }
  }

  pub fn add(&mut self, keycode: u16) {
    if let Some(slot) = self.codes.iter_mut().find(|c| c.is_none()) {
      *slot = Some(Code {
        code: keycode,
        sent_once: false,
        delete: false,
      });
    }
  }

  pub fn remove(&mut self, keycode: u16) {
    let found = self.codes.iter_mut().flatten().find(|c| c.code == keycode && !c.delete);
    if let Some(code) = found {
      code.delete = true;
    }
  }

  // one boot report holds the whole keyboard state: modifiers are or'ed together.
  // ponytail: codes past the 6 keycode slots are dropped, add nkro when that matters
  pub fn build(&mut self) -> Report {
    let mut report = Report::default();
    let mut n = 0;

    for code_opt in self.codes.iter_mut() {
      if let Some(ref code) = code_opt {
        if code.delete && code.sent_once {
          *code_opt = None;
        }
      }

      if let Some(ref mut code) = code_opt {
        code.sent_once = true;
        report.modifier |= get_modifier_u8(code.code);
        let keycode = code.code as u8;
        if (0xE0..=0xE7).contains(&keycode) {
          // modifier keys are bits of the modifier byte, not array entries
          report.modifier |= 1 << (keycode - 0xE0);
        } else if keycode != 0
          && n < report.keycodes.len()
          && !report.keycodes[..n].contains(&keycode)
        {
          report.keycodes[n] = keycode;
          n += 1;
        }
      }
    }

    report
  }

  #[cfg(not(feature = "chip_type_emulator"))]
  pub async fn process<D: Driver<'static>>(&mut self, writer: &mut HidWriter<'static, D, WRITE_N>) {
    let report = self.build();
    if let Err(e) = writer.write(&report.serialize()).await {
      warn!("Failed to send report: {:?}", e);
    }
  }
}
