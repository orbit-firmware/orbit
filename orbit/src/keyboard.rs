use core::array::from_fn as populate;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::orbit::config::KEY_COUNT;
use crate::orbit::key::Key;
use crate::orbit::peripherals::Peripherals;

static KEYBOARD_INIT: AtomicBool = AtomicBool::new(false);
static mut KEYBOARD: UnsafeCell<Option<Keyboard>> = UnsafeCell::new(None);

pub struct Keyboard {
  peripherals: Peripherals,
  layer: u32,
  keys: [Key; KEY_COUNT],
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
    assert!(KEY_COUNT > 0);
    let keys = populate(Key::new);

    Self {
      peripherals: Peripherals::new(),
      keys,
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
    let peripherals = &mut self.peripherals;
    peripherals.scan().await;
    let keys = &mut self.keys;
    for k in 0..KEY_COUNT {
      let state = peripherals.key(k);
      keys[k].process(state).await;
    }
  }

  pub fn get_key(&mut self, index: usize) -> &mut Key {
    assert!(index < KEY_COUNT, "Index out of bounds");
    &mut self.keys[index]
  }
  
}

