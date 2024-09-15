use crate::orbit::behaviors;
use crate::orbit::actions;
use crate::orbit::key::Key;
use crate::orbit::log::dump;
use crate::orbit::config::KEY_COUNT;
use crate::orbit::peripherals::Peripherals;
use core::array::from_fn as populate;

pub struct Keyboard {
  pub peripherals: Peripherals,
  pub keys: [Key; KEY_COUNT],
}

impl Keyboard {
  pub fn new() -> Self {
    assert!(KEY_COUNT > 0);
    let keys = populate(Key::new);

    Keyboard {
      peripherals: Peripherals::new(),
      keys,
    }
  }

  fn send(&self) {
    // Send the current state of the keyboard
  }

  pub async fn process(&mut self) {
    self.peripherals.scan();
    for key in self.keys.iter_mut() {
      key.update(&self.peripherals);
      if key.just_pressed() {
        dump!("Key {} is active", key.index());
      }
      behaviors::process(&key);
      actions::process(&key);
    }
    self.send();
  }
}
