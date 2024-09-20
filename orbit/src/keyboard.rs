use core::array::from_fn as populate;
use core::option::Option;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::orbit::config;
use crate::orbit::key::Key;
use crate::orbit::peripherals::*;

static KEYBOARD_INIT: AtomicBool = AtomicBool::new(false);
static mut KEYBOARD: UnsafeCell<Option<Keyboard>> = UnsafeCell::new(None);

use crate::orbit::log::dump;

pub struct Keyboard {
  peripherals: Peripherals,
  layer: u32,
  keys: [Key; config::KEY_COUNT],
}

impl Keyboard {
  // IMPORTANT: always use this to get the keyboard
  pub fn instance() -> &'static mut Keyboard {
    unsafe {
      if !KEYBOARD_INIT.load(Ordering::SeqCst) {
        KEYBOARD_INIT.store(true, Ordering::SeqCst);
        (*KEYBOARD.get()) = Some(Keyboard::new());
      }
      (*KEYBOARD.get()).as_mut().expect("Singleton should be initialized")
    }
  }

  fn new() -> Self {
    assert!(config::KEY_COUNT > 0);
    Self {
      peripherals: Peripherals::new(),
      keys: populate(Key::new),
      layer: 0,
    }
  }
  
  pub fn set_layer(&mut self, layer: u32) {
    self.layer = layer;
  }

  pub fn get_layer(&self) -> u32{
    self.layer
  }

  pub async fn process(&mut self) {
    self.scan();
  }

  pub fn get_key(&mut self, index: usize) -> &mut Key {
    assert!(index < config::KEY_COUNT, "Index out of bounds");
    &mut self.keys[index]
  }

  fn scan(&mut self) {
    if config::USE_MATRIX {
      self.scan_matrix();
    }

    if config::USE_MULTIPLEXERS {
      // self.scan_multiplexers();
    }
  }

  fn scan_matrix(&mut self) {
    let keys = &mut self.keys;
    let peripherals = &mut self.peripherals;

    for k in 0..config::LAYOUT.len() {
      let pair = &config::LAYOUT[k];
      if pair.len() != 2 {
        continue;
      }

      let mut state = true;
      if pair[0] != Peripheral::None {
        state = state && peripherals.input(pair[0].clone()).is_high();
      }

      if pair[1] != Peripheral::None {
        state = state && peripherals.input(pair[1].clone()).is_high();
        
      }
      keys[k].process(state);
    }
  }


}

