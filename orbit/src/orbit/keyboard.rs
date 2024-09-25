use core::array::from_fn as populate;
use core::cell::UnsafeCell;
use core::option::Option;
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_usb::driver::Driver;

use crate::orbit::config;
use crate::orbit::hid::Hid;
use crate::orbit::key::Key;
use crate::orbit::peripherals::*;

static KEYBOARD_INITIIALIZED: AtomicBool = AtomicBool::new(false);
static mut KEYBOARD_INSTANCE: UnsafeCell<Option<Keyboard>> = UnsafeCell::new(None);

use crate::orbit::dbg::{dump, info};

pub struct Keyboard {
  peripherals: Peripherals,
  layer: u32,
  keys: [Key; config::KEY_COUNT],
  hid: Option<Hid<Driver<'static>>>,
}

impl Keyboard {
  // IMPORTANT: always use this to get the keyboard
  pub fn instance() -> &'static mut Keyboard {
    unsafe {
      if !KEYBOARD_INITIIALIZED.load(Ordering::SeqCst) {
        KEYBOARD_INITIIALIZED.store(true, Ordering::SeqCst);
        (*KEYBOARD_INSTANCE.get()) = Some(Keyboard::new());
      }
      (*KEYBOARD_INSTANCE.get())
        .as_mut()
        .expect("Singleton should be initialized")
    }
  }

  fn new() -> Self {
    assert!(config::KEY_COUNT > 0);
    Self {
      peripherals: Peripherals::new(),
      keys: populate(Key::new),
      layer: 0,
      hid: None,
    }
  }

  pub async fn create_hid<D: Driver<'static>>(&mut self, driver: D) {
    let hid = Hid::new(driver).await;
    self.hid = Some(hid);
  }

  pub fn set_layer(&mut self, layer: u32) {
    self.layer = layer;
  }

  pub fn get_layer(&self) -> u32 {
    self.layer
  }

  pub async fn process(&mut self) {
    self.scan().await;
  }

  pub fn key(&mut self, index: usize) -> &mut Key {
    assert!(index < config::KEY_COUNT, "Index out of bounds");
    &mut self.keys[index]
  }

  pub fn peripherals(&mut self) -> &mut Peripherals {
    &mut self.peripherals
  }

  async fn scan(&mut self) {
    if config::USE_MATRIX {
      self.scan_matrix().await;
    }

    if config::USE_MULTIPLEXERS {
      // self.scan_multiplexers();
    }
  }

  async fn scan_matrix(&mut self) {
    let keys = &mut self.keys;
    let peri = &mut self.peripherals;

    for k in 0..config::LAYOUT.len() {
      let mut state = false;

      let pair = &config::LAYOUT[k];
      if pair.len() == 2 {
        let row = pair[0];
        let col = pair[1];
        if row == Peripheral::None && col == Peripheral::None {
          continue;
        } else if row != Peripheral::None && col != Peripheral::None {
          let s1 = peri.input(row).is_high();
          let s2 = peri.input(col).is_high();
          state = s1 && s2;
        } else if row != Peripheral::None {
          state = peri.input(row).is_high();
        } else if col != Peripheral::None {
          state = peri.input(col).is_high();
        }
      }

      keys[k].process(state);
    }
  }
}
