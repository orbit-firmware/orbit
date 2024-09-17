use crate::orbit::config::DEBOUNCE_TIME;
use crate::orbit::config::TAPPING_TERM;
use crate::orbit::time;

#[derive(Clone, Copy, Debug)]
pub struct Key {
  index: usize,       // Index of the physical key
  state: bool,        // Pressed state of the key
  changed: bool,      // If the key has changed this frame
  taps: u16,          // How many times the key was tapped
  tapping_term: u64,  // Time in which a repeated press counts as a tap
  timestamp: u64,     // Last timestamp when the key state changed
  debounce_time: u64, // Timestamp when debouncing started
  debouncing: bool,   // If the key is currently debouncing
}

impl Key {
  pub fn new(index: usize) -> Key {
    Key {
      index,
      state: false,
      changed: false,
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

  pub fn state(&self) -> bool {
    self.state
  }

  pub fn changed(&self) -> bool {
    self.changed
  }

  pub fn time(&self) -> u64 {
    time::elapsed(self.timestamp)
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

  fn process(&mut self, state: bool, now: u64) {
    self.state = state;
    let time = self.time();
    if state {
      if time <= self.tapping_term() {
        self.taps += 1;
      } else {
        self.taps = 0;
      }
    }
    self.changed = true;
    self.timestamp = now;
  }

  pub fn update(&mut self, state: bool) {
    let now = time::now();
    self.changed = false;

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
