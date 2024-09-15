use crate::orbit::config::DEBOUNCE_MICRO_SECS;
use crate::orbit::log::dump;
use crate::orbit::peripherals::Peripherals;
use crate::orbit::time;

pub struct Key {
  index: usize,        // Index of the physical key
  state: bool,         // Pressed state of the key
  just_pressed: bool,  // Pressed state of the key, this tick
  just_released: bool, // Released state of the key, this tick
  timestamp: u64,      // Last timestamp when the key state changed
  debounce_time: u64,  // Timestamp when debouncing started
  debouncing: bool,    // Whether the key is currently debouncing
}

impl Key {
  pub fn new(index: usize) -> Key {
    Key {
      index,
      state: false,
      just_pressed: false,
      just_released: false,
      timestamp: time::now(),
      debounce_time: 0,
      debouncing: false,
    }
  }

  pub fn index(&self) -> usize {
    self.index
  }

  pub fn state(&self) -> bool {
    self.state
  }

  pub fn just_pressed(&self) -> bool {
    self.just_pressed
  }

  pub fn just_released(&self) -> bool {
    self.just_released
  }

  pub fn timestamp(&self) -> u64 {
    self.timestamp
  }

  #[allow(dead_code)]
  pub fn get_time(&self) -> u64 {
    time::elapsed(self.timestamp)
  }

  fn eval(&mut self, state: bool, now: u64) {
    self.state = state;
    let time = self.get_time();
    if state {
      dump!(
        "Key {} pressed after {} micros, {} ms, {} sec",
        self.index,
        time,
        time / 1000,
        time / 1000 / 1000
      );
      self.just_pressed = true;
    } else {
      self.just_released = true;
    }
    self.timestamp = now;
  }

  pub fn update(&mut self, state: bool) {
    let now = time::now();
    self.just_pressed = false;
    self.just_released = false;

    if self.state != state && !self.debouncing {
      self.eval(state, now);
      self.debounce_time = now;
      self.debouncing = true;
    } else if self.debouncing {
      let debounce_time = time::elapsed(self.debounce_time);
      if debounce_time >= DEBOUNCE_MICRO_SECS {
        self.debouncing = false;
        if self.state != state {
          self.eval(state, now);
        }
      }
    }
  }
}
