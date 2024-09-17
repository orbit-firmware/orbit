use core::array::from_fn as populate;
use core::option::Option;
use core::mem;

use crate::orbit::config::KEY_COUNT;
use crate::orbit::key::Key;
use crate::orbit::event::Event;
use crate::orbit::peripherals::Peripherals;
use crate::orbit::actions;
use crate::orbit::behaviors;
use crate::orbit::log::dump;

const MIN_BUFFER_SIZE: usize = 16;
const BUFFER_SIZE: usize = if KEY_COUNT > MIN_BUFFER_SIZE { KEY_COUNT } else { MIN_BUFFER_SIZE };
const MAX_KEY_PROCESS_TIME: u64 = 5000; // 5 sec


pub struct Keyboard {
  peripherals: Peripherals,
  keys: [Key; KEY_COUNT],
  buffer: [Option<Event>; BUFFER_SIZE],
  buffer_count: usize,
}

impl Keyboard {

  pub fn new() -> Self {
    assert!(KEY_COUNT > 0);
    let keys = populate(Key::new);
    let buffer = populate(|_| None);

    Keyboard {
      peripherals: Peripherals::new(),
      keys,
      buffer,
      buffer_count: 0,
    }
  }

  pub async fn process(&mut self) {
    self.peripherals.scan();

    for key in self.keys.iter_mut() {
      let state = self.peripherals.key(key.index());
      key.update(state);
      if key.changed() {
        for i in (1..BUFFER_SIZE).rev() {
          self.buffer[i] = mem::replace(&mut self.buffer[i - 1], None);
        }

        self.buffer[0] = Some(Event::from_key(*key));

        if self.buffer_count < BUFFER_SIZE {
          self.buffer_count += 1;
        }
      }
    }

    for i in 0..self.buffer_count {
      if let Some(mut event) = self.buffer[i].take() {
        if !event.processed() {
          behaviors::process(self, &mut event);
          if event.processed() {
            actions::process(self, &mut event);
          }
        }
        self.buffer[i] = Some(event);
      }
    }

    if let Some(ref mut key_event) = self.get_latest_key_event(0) {
      if key_event.state() {
        key_event.send();
        dump!("{}", "send");
      }
      dump!("{}", key_event.state());
    }
  }

  pub fn get_latest_key_event(&mut self, key_index: usize) -> Option<&mut Event> {
    if key_index >= KEY_COUNT {
        return None;
    }

    self.buffer.iter_mut()
        .find(|event| event.as_ref().map_or(false, |e| e.index() == key_index))
        .and_then(Option::as_mut)
  }
    
}