use core::array::from_fn as populate;

use crate::orbit::behaviors;
use crate::orbit::actions;
use crate::orbit::config::KEY_COUNT;
use crate::orbit::key::Key;
use crate::orbit::record::Record;
use crate::orbit::peripherals::Peripherals;

const BUFFER_SIZE :usize= 16;

pub struct Keyboard {
  peripherals: Peripherals,
  keys: [Key; KEY_COUNT],
  buffer: [Key; BUFFER_SIZE],
}

impl Keyboard {
  pub fn new() -> Self {
    assert!(KEY_COUNT > 0);
    let keys = populate(Key::new);
    let buffer = populate(Key::new);

    Keyboard {
      peripherals: Peripherals::new(),
      keys,
      buffer,
    }
  }

  fn send(&self) {
    // Send the current state of the keyboard
  }

  fn add_record(record: &Record) {
    // behaviors::process(&key);
      // actions::process(&key);
  }

  pub async fn process(&mut self) {
    self.peripherals.scan();
    for key in self.keys.iter_mut() {
      let state = self.peripherals.key(key.index());
      key.update(state);
      if key.just_pressed() || key.just_released() {
        // add_record(Record::from_key(key));
      }
    }
    self.send();
  }
}
