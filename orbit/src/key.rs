use crate::orbit::actions;
use crate::orbit::behaviors;
use crate::orbit::config::DEBOUNCE_TIME;
use crate::orbit::config::TAPPING_TERM;
use crate::orbit::keyboard::Keyboard;
use crate::orbit::log::dump;
use crate::orbit::time;

use heapless::String;

const PRESSED: u8 = 0b00000001;
const CHANGED: u8 = 0b00000010;
const DEBOUNCING: u8 = 0b00000100;
const SEND: u8 = 0b00001000;

#[derive(Debug)]
pub struct Key {
  index: usize,       // Index of the physical key
  state: u8,          // The encoded key state
  taps: u16,          // How many times the key was tapped
  tapping_term: u64,  // Time in which a repeated press counts as a tap
  timestamp: u64,     // Last timestamp when the key state changed
  debounce_time: u64, // Timestamp when debouncing started
  input: String<32>,
}

impl Key {
  pub fn new(index: usize) -> Key {
    let mut input = String::new();
    input.push_str("xxx").unwrap();

    Key {
      index,
      state: 0,
      taps: 0,
      tapping_term: TAPPING_TERM,
      timestamp: time::now(),
      debounce_time: 0,
      input,
    }
  }

  #[inline(always)]
  pub fn index(&self) -> usize {
    self.index
  }

  #[inline(always)]
  pub fn pressed(&self) -> bool {
    (self.state & PRESSED) != 0
  }

  #[inline(always)]
  pub fn released(&self) -> bool {
    (self.state & PRESSED) != 0
  }

  #[inline(always)]
  fn press(&mut self) {
    self.state |= PRESSED;
  }

  #[inline(always)]
  fn release(&mut self) {
    self.state &= !PRESSED;
  }

  #[inline(always)]
  fn changed(&self) -> bool {
    (self.state & CHANGED) != 0
  }

  #[inline(always)]
  fn changed_on(&mut self) {
    self.state |= CHANGED;
  }

  #[inline(always)]
  fn changed_off(&mut self) {
    self.state &= !CHANGED;
  }

  #[inline(always)]
  fn debouncing(&self) -> bool {
    (self.state & DEBOUNCING) != 0
  }

  #[inline(always)]
  fn debouncing_on(&mut self) {
    self.state |= DEBOUNCING;
  }

  #[inline(always)]
  fn debouncing_off(&mut self) {
    self.state &= !DEBOUNCING;
  }

  #[inline(always)]
  fn send_on(&mut self) {
    self.state |= SEND;
  }

  #[inline(always)]
  fn send_off(&mut self) {
    self.state &= !SEND;
  }

  pub fn time(&self) -> u64 {
    time::elapsed(self.timestamp)
  }
  #[inline(always)]
  pub fn timestamp(&self) -> u64 {
    self.timestamp
  }
  #[inline(always)]
  pub fn taps(&self) -> u16 {
    self.taps
  }

  pub fn tapping_term(&self) -> u64 {
    // TODO: tapping term per key
    self.tapping_term
  }

  pub fn process(&mut self, state: bool) {
    let mut keyboard = Keyboard::instance();
    self.update(state);
    behaviors::process(keyboard, self);
  }

  pub fn send(&mut self) {
    if (self.state & SEND) == 0 {
      dump!("{:#?}", self);
      let mut keyboard = Keyboard::instance();
      actions::process(keyboard, self);
      self.send_on()
    }
  }

  fn evaluate(&mut self, state: bool, now: u64) {
    let time = self.time();
    if state {
      self.press();
      if time <= self.tapping_term() {
        self.taps += 1;
      } else {
        self.taps = 0;
      }
    } else {
      self.release();
    }
    self.changed_on();
    self.send_off();
    self.timestamp = now;
  }

  fn update(&mut self, state: bool) {
    let now = time::now();
    self.changed_off();

    if self.pressed() != state && !self.debouncing() {
      self.evaluate(state, now);
      self.debounce_time = now;
      self.debouncing_on();
    } else if self.debouncing() {
      let debounce_time = time::elapsed(self.debounce_time);
      if debounce_time >= DEBOUNCE_TIME {
        self.debouncing_off();
        if self.pressed() != state {
          self.evaluate(state, now);
        }
      }
    }
  }
}
