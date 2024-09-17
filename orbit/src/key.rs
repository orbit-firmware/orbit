use crate::orbit::actions;
use crate::orbit::behaviors::{self, Behavior};
use crate::orbit::config::DEBOUNCE_TIME;
use crate::orbit::config::TAPPING_TERM;
use crate::orbit::keyboard::Keyboard;
use crate::orbit::log::dump;
use crate::orbit::time;

use heapless::String;

#[rustfmt::skip]
mod state {
  pub const PRESSED:     u8 = 0b00000001;
  pub const CHANGED:     u8 = 0b00000010;
  pub const DEBOUNCING:  u8 = 0b00000100;
  pub const SHOULD_SEND: u8 = 0b00001000;
  pub const CUSTOM_SEND: u8 = 0b00010000;
  pub const SEND_NEXT:   u8 = 0b00100000;
  pub const SEND_NOW:    u8 = 0b01000000;
}

#[derive(Debug)]
pub struct Key {
  index: usize,       // Index of the physical key
  state: u8,          // The encoded key state
  taps: u16,          // How many times the key was tapped
  tapping_term: u64,  // Time in which a repeated press counts as a tap
  timestamp: u64,     // Last timestamp when the key state changed
  debounce_time: u64, // Timestamp when debouncing started
  code: String<32>,   // the key code
}

impl Key {
  pub fn new(index: usize) -> Key {
    let mut code = String::new();
    code.push_str("xxx").unwrap();

    Key {
      index,
      state: 0,
      taps: 0,
      tapping_term: TAPPING_TERM,
      timestamp: time::now(),
      debounce_time: 0,
      code,
    }
  }

  #[inline(always)]
  pub fn index(&self) -> usize {
    self.index
  }

  #[inline(always)]
  pub fn pressed(&self) -> bool {
    (self.state & state::PRESSED) != 0
  }

  #[inline(always)]
  pub fn just_pressed(&self) -> bool {
    (self.state & state::PRESSED) != 0 && self.changed()
  }

  #[inline(always)]
  pub fn released(&self) -> bool {
    (self.state & state::PRESSED) == 0
  }

  #[inline(always)]
  pub fn just_released(&self) -> bool {
    (self.state & state::PRESSED) == 0 && self.changed()
  }

  #[inline(always)]
  pub fn changed(&self) -> bool {
    (self.state & state::CHANGED) != 0
  }

  pub fn send_next(&mut self) {
    self.state |= state::CUSTOM_SEND;
    self.state &= !state::SEND_NOW;
    self.state |= state::SEND_NEXT;
  }

  pub fn send_now(&mut self) {
    self.state |= state::CUSTOM_SEND;
    self.state |= state::SEND_NOW;
    self.state &= !state::SEND_NEXT;
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

  pub fn has_behavior(&self, behavior: Behavior) -> bool {
    match behavior {
      Behavior::Hold => true,
      Behavior::Tap => true,
      _ => false,
    }
  }

  pub fn process(&mut self, state: bool) {
    let keyboard = Keyboard::instance();
    self.update(state);
    behaviors::process(keyboard, self);
    if self.sendable() {
      dump!("send");
      self.state &= !state::SHOULD_SEND;
      self.state &= !state::CUSTOM_SEND;
      actions::process(keyboard, self);
    }
  }

  fn sendable(&mut self) -> bool {
    let should_send: bool = (self.state & state::SHOULD_SEND) != 0;
    let send_next: bool = (self.state & state::SEND_NEXT) != 0;
    let send_now: bool = (self.state & state::SEND_NOW) != 0;
    let custom_send: bool = (self.state & state::SEND_NOW) != 0;

    if send_now && should_send {
      self.state &= !state::SEND_NOW;
      return true;
    }

    if self.changed() && send_next {
      self.state &= !state::SEND_NEXT;
      return false;
    }

    if self.changed() && should_send && !custom_send {
      return true;
    }

    false
  }

  fn get_code(&self, behavior: Behavior) -> &str {
    if !self.has_behavior(behavior) {
      return "";
    }

    return "a";
  }

  #[inline(always)]
  fn set_state(&mut self, state: bool) {
    if self.pressed() == state {
      return;
    }
    self.state |= state::CHANGED;
    if state {
      self.state |= state::PRESSED;
    } else {
      self.state &= !state::PRESSED;
    }
  }

  #[inline(always)]
  fn debouncing(&self) -> bool {
    (self.state & state::DEBOUNCING) != 0
  }

  #[inline(always)]
  fn set_debouncing(&mut self, state: bool) {
    if state {
      self.state |= state::DEBOUNCING;
    } else {
      self.state &= !state::DEBOUNCING;
    }
  }

  fn update(&mut self, state: bool) {
    let now = time::now();
    self.state &= !state::CHANGED;
    if self.pressed() != state && !self.debouncing() {
      self.evaluate(state, now);
      self.debounce_time = now;
      self.set_debouncing(true);
    } else if self.debouncing() {
      let debounce_time = time::elapsed(self.debounce_time);
      if debounce_time >= DEBOUNCE_TIME {
        self.set_debouncing(false);
        if self.pressed() != state {
          self.evaluate(state, now);
        }
      }
    }
  }

  fn evaluate(&mut self, state: bool, now: u64) {
    let time = self.time();
    self.set_state(state);
    self.timestamp = now;
    if state {
      self.state |= state::SHOULD_SEND;
      if time <= self.tapping_term() {
        self.taps += 1;
      } else {
        self.taps = 0;
      }
    }
  }
}
