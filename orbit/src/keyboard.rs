use core::array::from_fn as populate;
use core::option::Option;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::orbit::config;
use crate::orbit::key::Key;
// use crate::orbit::peripherals::*;

static KEYBOARD_INIT: AtomicBool = AtomicBool::new(false);
static mut KEYBOARD: UnsafeCell<Option<Keyboard>> = UnsafeCell::new(None);

#[cfg(feature = "family_EMULATOR")]
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::orbit::log::dump;

pub struct Keyboard {
  #[cfg(feature = "family_EMULATOR")]
  device_state: DeviceState,
  // peripherals: Peripherals,
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
      #[cfg(feature = "family_EMULATOR")]
      device_state: DeviceState::new(),
      // peripherals: Peripherals::new(),
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
    
    #[cfg(not(feature = "family_EMULATOR"))]
    self.scan();

    #[cfg(feature = "family_EMULATOR")]
    self.scan_emulator();

    
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
    dump!("Scanning matrix");
    // let peripherals = &mut self.peripherals;

    // for k in 0..config::LAYOUT.len() {
    //   let pair = config::LAYOUT[k];
    //   if pair.len() != 2 {
    //     continue;
    //   }

    //   let row = false;
    //   // if pair[0] != Peripherals::None {
    //   //   row = get_input_peripheral(pair[0]).is_high();
    //   // }

    //   let col = false;
    //   // if pair[1] != Peripherals::None {
    //     // col = get_input_peripheral(pair[1]).is_high();
    //   // }

    //   let state = row && col;
    //   keys[k].process(state);
    // }
  }


  fn scan_emulator(&mut self) {
    let device_keys: Vec<Keycode> = self.device_state.get_keys();
    let keys = &mut self.keys;
    for k in 0..config::LAYOUT.len() {
      let pair = config::LAYOUT[k];
      if pair.len() != 2 {
        continue;
      }

      let mut row = false;
      if config::MATRIX_ROW_PINS.len() > 0 {
        let key_code = config::MATRIX_ROW_PINS[pair[0]];
      }

      let mut col = false;
      if config::MATRIX_COL_PINS.len() > 0 {
        let key_code = config::MATRIX_COL_PINS[pair[1]];
      }
      
      // keys[k].process(true);

    //   let row = false;
    //   // if pair[0] != Peripherals::None {
    //   //   row = get_input_peripheral(pair[0]).is_high();
    //   // }

    //   let col = false;
    //   // if pair[1] != Peripherals::None {
    //     // col = get_input_peripheral(pair[1]).is_high();
    //   // }

      // key.process(state);
    }
  }

}

