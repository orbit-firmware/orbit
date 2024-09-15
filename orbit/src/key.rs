use crate::orbit::config::DEBOUNCE_TIME;
use crate::orbit::config::TAPPING_TERM;
use crate::orbit::time;

pub struct Key {
  index: usize,        // Index of the physical key
  state: bool,         // Pressed state of the key
  just_pressed: bool,  // Pressed state of the key, this tick
  just_released: bool, // Released state of the key, this tick
  taps: u16,           // How many times the key was tapped
  tapping_term: u64,   // Time in which a repeated press counts as a tap
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
      taps: 0,
      tapping_term: TAPPING_TERM,
      timestamp: time::now(),
      debounce_time: 0,
      debouncing: false,
    }
  }

  pub fn index(&self) -> usize {
    self.index
  }

  pub fn is_pressed(&self) -> bool {
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

  pub fn taps(&self) -> u16 {
    self.taps
  }

  pub fn tapping_term(&self) -> u64 {
    // TODO: tapping term per key
    self.tapping_term
  }

  #[allow(dead_code)]
  pub fn get_time(&self) -> u64 {
    time::elapsed(self.timestamp)
  }

  fn process(&mut self, state: bool, now: u64) {
    self.state = state;
    let time = self.get_time();
    if state {
      if time <= self.tapping_term() {
        self.taps += 1;
      } else {
        self.taps = 0;
      }
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
      self.process(state, now);
      self.debounce_time = now;
      self.debouncing = true;
    } else if self.debouncing {
      let debounce_time = time::elapsed(self.debounce_time);
      if debounce_time >= DEBOUNCE_TIME {
        self.debouncing = false;
        if self.state != state {
          self.process(state, now);
        }
      }
    }
  }
}
