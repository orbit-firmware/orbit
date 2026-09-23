use core::array::from_fn as populate;
#[cfg(not(feature = "chip_type_emulator"))]
use embassy_futures::join::join;
#[cfg(not(feature = "chip_type_emulator"))]
use embassy_usb::driver::Driver;

use crate::orbit::config as Orbit;
#[cfg(feature = "multiplexers_scan")]
use crate::orbit::dbg::info;
#[cfg(not(feature = "chip_type_emulator"))]
use crate::orbit::dbg::warn;
use crate::orbit::engine::{Engine, Report};
#[cfg(not(feature = "chip_type_emulator"))]
use crate::orbit::hid;
use crate::orbit::key::Key;
use crate::orbit::keymap::Entry;
use crate::orbit::peripherals::*;
use crate::orbit::time;

pub struct Keyboard {
  peripherals: Peripherals,
  keys: [Key; Orbit::KEY_COUNT],
  pressed: [bool; Orbit::KEY_COUNT],
  engine: Engine,
}

impl Keyboard {
  pub fn new() -> Self {
    assert!(Orbit::KEY_COUNT > 0, "No keys defined");
    Self {
      peripherals: Peripherals::new(),
      keys: [Key::new(); Orbit::KEY_COUNT],
      pressed: [false; Orbit::KEY_COUNT],
      engine: Engine::new(),
    }
  }

  #[cfg(not(feature = "chip_type_emulator"))]
  pub async fn process<D: Driver<'static>>(&mut self, driver: D) {
    let (mut usb, _reader, mut writer) = hid::keyboard::init(driver).await;

    let process = async {
      loop {
        if hid::keyboard::ready().await {
          let report = self.tick();
          if let Err(e) = writer.write(&report.serialize()).await {
            warn!("Failed to send report: {:?}", e);
          }
        }
      }
    };

    join(usb.run(), process).await;
  }

  // one scan cycle: read the keys, run them through the keymap
  pub fn tick(&mut self) -> Report {
    self.scan();
    self.engine.update(&self.pressed, time::now())
  }

  pub fn is_pressed(&self, k: usize) -> bool {
    self.pressed[k]
  }

  // what key k sends if pressed now (current layers and shift)
  pub fn entry(&self, k: usize) -> Entry {
    self.engine.entry(k)
  }

  fn scan(&mut self) {
    #[cfg(feature = "matrix_scan")]
    self.scan_matrix();

    #[cfg(feature = "multiplexers_scan")]
    self.scan_multiplexers();
  }

  fn set_key(&mut self, k: usize, raw: bool) {
    self.pressed[k] = self.keys[k].update(raw, time::now());
  }

  #[cfg(feature = "matrix_scan")]
  fn scan_matrix(&mut self) {
    for k in 0..Orbit::LAYOUT.len() {
      let mut state = false;
      let pair = &Orbit::LAYOUT[k];
      if pair.len() != 2 {
        continue;
      }

      let row = pair[0];
      let col = pair[1];
      if row == Peripheral::None && col == Peripheral::None {
        continue;
      } else if row != Peripheral::None && col != Peripheral::None {
        let s1 = self.peripherals.input(row).is_high();
        let s2 = self.peripherals.input(col).is_high();
        state = s1 && s2;
      } else if row != Peripheral::None {
        state = self.peripherals.input(row).is_high();
      } else if col != Peripheral::None {
        state = self.peripherals.input(col).is_high();
      }

      self.set_key(k, state);
    }
  }

  #[cfg(feature = "multiplexers_scan")]
  fn scan_multiplexers(&mut self) {
    let mut state = false;
    let keys = &mut self.keys;
    let peri = &mut self.peripherals;
    let num_bits = Orbit::MULTIPLEXER_CHANNELS
      .next_power_of_two()
      .trailing_zeros() as usize;

    for k in 0..Orbit::LAYOUT.len() {
      let mut state = false;
      let pair = &Orbit::LAYOUT[k];
      let com = pair.0;
      let sel = pair.1 as usize & ((1 << num_bits) - 1) as usize;

      for (i, pin) in Orbit::MULTIPLEXER_SEL_PINS.iter().enumerate() {
        if (sel & (1 << i)) != 0 {
          peri.output(*pin).set_high();
        } else {
          peri.output(*pin).set_low();
        }
      }

      let mut state = peri.input(com).read();
      info!("{}", state);
      // self.set_key(k, state);
    }
  }
}
