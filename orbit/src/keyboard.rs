use core::array::from_fn as populate;
use core::option::Option;

use crate::orbit::{
  config::KEY_COUNT,
  key::Key,
  peripherals::Peripherals,
  actions,
  behaviors
};

const BUFFER_SIZE :usize= 16;

pub struct Keyboard {
  peripherals: Peripherals,
  keys: [Key; KEY_COUNT],
  buffer: [(bool, Option<Key>); BUFFER_SIZE],
  buffer_count: usize,
}

impl Keyboard {

  pub fn new() -> Self {
    assert!(KEY_COUNT > 0);
    let keys = populate(Key::new);
    let buffer = populate(|_| (false, None));

    Keyboard {
      peripherals: Peripherals::new(),
      keys,
      buffer,
      buffer_count: 0,
    }
  }

  pub async fn process(&mut self) {
    self.peripherals.scan();
    let mut frozen = Vec::new();
    for key in self.keys.iter_mut() {
      let state = self.peripherals.key(key.index());
      key.update(state);
      if key.just_pressed() || key.just_released() {
        frozen.push(key.freeze());
      }
    }

    // buffer
    for key in frozen {
      for i in (1..BUFFER_SIZE).rev() {
        self.buffer[i] = self.buffer[i - 1];
      }

      self.buffer[0] = (true, Some(key));

      if self.buffer_count < BUFFER_SIZE {
        self.buffer_count += 1;
      }
    }

    self.process_buffer();
  }

  fn process_buffer(&mut self) {
    for i in 0..self.buffer_count {
      if let (needs_process, Some(key)) = self.buffer[i] {
        if needs_process && Keyboard::process_key(key) {
          self.buffer[i].0 = false;
        }
      }
    }
  }

  fn process_key(key: Key) -> bool {
    let mut finished = false;
    finished = behaviors::process(&key);
    if finished {
      actions::process(&key);
    }
    finished
  }
}
