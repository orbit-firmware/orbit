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
  keys: [Key; KEY_COUNT],
}

impl Keyboard {
  pub fn new() -> Self {
    assert!(KEY_COUNT > 0);
    let keys = populate(Key::new);

    Self {
      peripherals: Peripherals::new(),
      keys,
    }
  }

  pub fn instance() -> &'static mut Keyboard {
    unsafe {
      if !KEYBOARD_INIT.load(Ordering::SeqCst) {
        KEYBOARD_INIT.store(true, Ordering::SeqCst);
        (*KEYBOARD.get()) = Some(Keyboard::new());
      }
  
      (*KEYBOARD.get()).as_mut().expect("Singleton should be initialized")
    }
  }
  

  pub async fn process(&mut self) {
    let peripherals = &mut self.peripherals;
    peripherals.scan().await;
    let keys = &mut self.keys;
    for k in 0..KEY_COUNT {
      keys[k].process(peripherals.key(k));
    }
  }

  pub fn get_key(&mut self, index: usize) -> &mut Key {
    assert!(index < KEY_COUNT, "Index out of bounds");
    &mut self.keys[index]
  }
}

