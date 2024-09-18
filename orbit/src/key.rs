// use crate::orbit::actions;
// use crate::orbit::behaviors::{self, Behavior};
use crate::orbit::config;
use crate::orbit::keyboard::Keyboard;
use crate::orbit::behaviors::Behavior;
use crate::orbit::actions::Action;
use crate::orbit::log::dump;
use crate::orbit::time;

// use crate::orbit::time;

use heapless::String;

#[rustfmt::skip]
mod state {
  pub const PRESSED:      u8 = 0b00000001;
  pub const CHANGED:      u8 = 0b00000010;
  pub const DEBOUNCING:   u8 = 0b00000100;
  pub const SEND_ENABLED: u8 = 0b00001000;
  pub const SEND_RELEASE: u8 = 0b00010000;
  pub const SEND_ONESHOT: u8 = 0b00100000;
  pub const SEND_INSTANT: u8 = 0b01000000;
}

// should read in the keycodes/strings from the keymap here
// so we can process them in the corresponding action
// then in send, set the current behavior
// in action resolve the code
// we also need to tokenize the input code
// lg(ls(la(x))) should become [x ,la, ls, lg]
// so we can dequeue the code


#[derive(Debug)]
pub struct Key {
  // physical index of the key
  index: usize,

  // The key code to send
  code: String<32>,   

  // The encoded key state
  state: u8,          
  
  // How many times the key was tapped
  #[cfg(feature = "behavior_tap_enabled")]
  taps: u8,           
  
  // Time in which a repeated press counts as a tap
  #[cfg(feature = "behavior_tap_enabled")]
  tapping_term: u16,        
  
  // Delay for sends
  delay: u16,
  
  // Timestamp when the key state was last changed
  timestamp: u32,           
  
  // Timestamp when the key was last debounced
  debounce_timestamp: u32,  
}

impl Key {
  pub fn new(index: usize) -> Key {
    let mut code = String::new();
    code.push_str("xxx").unwrap();

    Key {
      index,
      code,
      state: 0,
      #[cfg(feature = "behavior_tap_enabled")]
      taps: 0,
      #[cfg(feature = "behavior_tap_enabled")]
      tapping_term: config::TAPPING_TERM,
      delay: 0,
      timestamp: 0,
      debounce_timestamp: 0,
    }
  }

  #[inline(always)]
  pub fn is_pressed(&self) -> bool {
    (self.state & state::PRESSED) != 0
  }

  #[inline(always)]
  pub fn just_changed(&self) -> bool {
    (self.state & state::CHANGED) != 0
  }

  #[inline(always)]
  pub fn just_pressed(&self) -> bool {
    (self.state & state::PRESSED) != 0 && self.just_changed()
  }

  #[inline(always)]
  pub fn is_released(&self) -> bool {
    (self.state & state::PRESSED) == 0
  }

  #[inline(always)]
  pub fn just_released(&self) -> bool {
    (self.state & state::PRESSED) == 0 && self.just_changed()
  }

  #[inline(always)]
  pub fn time(&self) -> u16 {
    time::elapsed(self.timestamp)
  }

  #[cfg(feature = "behavior_tap_enabled")]
  pub fn tapping_term(&self) -> u16 {
    // TODO: tapping term per key from keymap
    self.tapping_term
  }
  
  #[cfg(feature = "behavior_tap_enabled")]
  pub fn taps(&self) -> u8{
    self.taps
  }

  pub fn send(&mut self) {
    self.state |= state::SEND_INSTANT;
    self.state &= !state::SEND_RELEASE;
    self.state &= !state::SEND_ONESHOT;
  }

  pub fn send_oneshot(&mut self) {
    self.send();
    self.state |= state::SEND_ONESHOT;
  }

  pub fn send_delayed(&mut self, delay: u16) {
    self.delay = delay;
    self.send();
  }

  pub fn send_on_release(&mut self) {
    self.state &= !state::SEND_INSTANT;
    self.state |= state::SEND_RELEASE;
    self.state |= state::SEND_ONESHOT;
  }

  pub fn send_on_release_delayed(&mut self, delay: u16) {
    self.delay = delay;
    self.send_on_release();
  }

  pub fn has_behavior(&self, behavior: Behavior) -> bool {
    // TODO: get from keymap
    match behavior {
      Behavior::Hold => true,
      Behavior::Tap => true,
      _ => false,
    }
  }

  pub async fn process(&mut self, state: bool) {
    self.set_pressed(state).await;
    let keyboard = Keyboard::instance();
    Behavior::process(keyboard, self);
    if self.is_sendable() {
      if self.has_oneshot() {
        self.state |= state::PRESSED;
        Action::process(keyboard, self);
        self.state &= !state::PRESSED;
        Action::process(keyboard, self);
      } else {
        Action::process(keyboard, self);
      }
      self.state &= !state::SEND_ENABLED;
      self.state &= !state::SEND_ONESHOT;
      self.delay = 0;
    }
  }

  // 
  // PRIVATES
  // 
  
  async fn set_pressed(&mut self, state: bool) {
    self.state &= !state::CHANGED;
    let current = self.is_pressed();
    let pressed = self.debounce(state);
    if state == current {
      return;
    }

    if pressed {
      self.state |= state::PRESSED;
      #[cfg(feature = "behavior_tap_enabled")]
      self.process_taps();
    } else {
      self.state &= !state::PRESSED;
    }
    if current != pressed {
      self.state |= state::SEND_ENABLED;
      self.state |= state::CHANGED;
      self.timestamp = time::now();
    }
  }

  fn is_sendable(&mut self) -> bool {
    if self.delay > 0 {
      if time::elapsed(self.timestamp) < self.delay {
        return false;
      }
    }

    let sendable: bool = (self.state & state::SEND_ENABLED) != 0;
    if !sendable {
      return false;
    }

    let send_now: bool = (self.state & state::SEND_INSTANT) != 0;
    if send_now {
      self.state &= !state::SEND_INSTANT;
      return true;
    }

    let send_release: bool = (self.state & state::SEND_RELEASE) != 0;
    if send_release {
      if self.is_released() {
        self.state &= !state::SEND_RELEASE;
        return true;
      }
      return false;
    }
    
    sendable
  }

  fn has_oneshot(&self) -> bool {
    (self.state & state::SEND_ONESHOT) != 0
  }

  #[cfg(feature = "behavior_tap_enabled")]
  fn process_taps(&mut self) {
    let time = self.time();
    if time <= self.tapping_term() {
      self.taps += 1;
    } else {
      self.taps = 0;
    }
  }

  fn debounce(&mut self, wanted_state: bool) -> bool {
    if (self.state & state::DEBOUNCING) != 0 {
      if time::elapsed(self.debounce_timestamp) >= config::DEBOUNCE_TIME {
        self.state &= !state::DEBOUNCING;
      } else {
        return self.is_pressed();
      }
    }

    if wanted_state != self.is_pressed() {
      self.state |= state::DEBOUNCING;
      self.debounce_timestamp = time::now();
      return wanted_state;
    }

    self.is_pressed()
  }

}
