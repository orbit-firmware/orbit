use crate::orbit::config;
use crate::orbit::dbg::*;
use crate::orbit::features::*;
use crate::orbit::keyboard::Keyboard;
use crate::orbit::keymap::KeyMap;
use crate::orbit::time;

pub const PRESSED: u8 = 0b00000001;
pub const CHANGED: u8 = 0b00000010;
pub const DEBOUNCING: u8 = 0b00000100;
pub const SEND_ENABLED: u8 = 0b00001000;
pub const SEND_RELEASE: u8 = 0b00010000;
pub const SEND_ONESHOT: u8 = 0b00100000;
pub const SEND_INSTANT: u8 = 0b01000000;
pub const INTERRUPT: u8 = 0b10000000;

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
    Key {
      index,
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
  #[allow(unused)]
  pub fn index(&self) -> usize {
    self.index
  }

  #[inline(always)]
  #[allow(unused)]
  pub fn is_pressed(&self) -> bool {
    self.has_state(PRESSED)
  }

  #[inline(always)]
  #[allow(unused)]
  pub fn just_changed(&self) -> bool {
    self.has_state(CHANGED)
  }

  #[inline(always)]
  #[allow(unused)]
  pub fn just_pressed(&self) -> bool {
    self.is_pressed() && self.just_changed()
  }

  #[inline(always)]
  #[allow(unused)]
  pub fn is_released(&self) -> bool {
    !self.is_pressed()
  }

  #[inline(always)]
  #[allow(unused)]
  pub fn just_released(&self) -> bool {
    self.is_released() && self.just_changed()
  }

  #[inline(always)]
  pub fn timestamp(&self) -> u32 {
    self.timestamp
  }

  #[inline(always)]
  pub fn time(&self) -> u16 {
    time::elapsed(self.timestamp())
  }

  #[cfg(feature = "behavior_tap_enabled")]
  pub fn tapping_term(&self) -> u16 {
    // TODO: tapping term per key from keymap
    self.tapping_term
  }

  #[cfg(feature = "behavior_tap_enabled")]
  pub fn taps(&self) -> u8 {
    self.taps
  }

  #[allow(unused)]
  pub fn send(&mut self) {
    self.add_state(SEND_INSTANT);
    self.del_state(SEND_RELEASE);
    self.del_state(SEND_ONESHOT);
  }

  #[allow(unused)]
  pub fn send_oneshot(&mut self) {
    self.send();
    self.add_state(SEND_ONESHOT);
  }

  #[allow(unused)]
  pub fn send_delayed(&mut self, delay: u16) {
    self.delay = delay;
    self.send();
  }

  #[allow(unused)]
  pub fn send_delayed_oneshot(&mut self, delay: u16) {
    self.send_delayed(delay);
    self.add_state(SEND_ONESHOT);
  }

  #[allow(unused)]
  pub fn send_on_release(&mut self) {
    self.del_state(SEND_INSTANT);
    self.add_state(SEND_RELEASE);
    self.add_state(SEND_ONESHOT);
  }

  #[allow(unused)]
  pub fn send_on_release_delayed(&mut self, delay: u16) {
    self.delay = delay;
    self.send_on_release();
  }

  pub fn has_behavior(&self, behavior: Behaviors) -> bool {
    // TODO: get from keymap
    match behavior {
      Behaviors::Hold => true,
      Behaviors::Tap => true,
      _ => false,
    }
  }

  pub fn process(&mut self, state: bool) {
    let keyboard = Keyboard::instance();
    self.set_pressed(state);
    Behaviors::process(keyboard, self);

    if self.is_sendable() {
      if self.has_state(SEND_ONESHOT) {
        info!("send_oneshot");
        self.send_key(keyboard, true);
        self.send_key(keyboard, false);
      } else {
        if self.is_pressed() {
          info!("send_press");
        } else {
          info!("send_release");
        }
        self.send_key(keyboard, self.is_pressed());
      }
      self.state &= !SEND_ENABLED;
      self.state &= !SEND_ONESHOT;
      self.state &= !INTERRUPT;
      self.delay = 0;
    }
    self.detect_interrupt(keyboard);
  }

  //
  // PRIVATES
  //

  fn send_key(&mut self, keyboard: &mut Keyboard, real_state: bool) {
    let previous_state = self.is_pressed();
    self.force_pressed(real_state);
    Actions::process(keyboard, self);
    self.force_pressed(previous_state);
  }

  fn force_pressed(&mut self, state: bool) {
    if state {
      self.add_state(PRESSED);
    } else {
      self.del_state(PRESSED);
    }
  }

  fn set_pressed(&mut self, state: bool) {
    self.del_state(CHANGED);
    let current = self.is_pressed();
    let pressed = self.debounce(state);
    if state == current {
      return;
    }

    if pressed {
      self.add_state(PRESSED);
      #[cfg(feature = "behavior_tap_enabled")]
      self.process_taps();
    } else {
      self.del_state(PRESSED);
    }
    if current != pressed {
      self.add_state(SEND_ENABLED);
      self.add_state(CHANGED);
      self.del_state(INTERRUPT);
      self.timestamp = time::now();
    }
  }

  fn detect_interrupt(&mut self, keyboard: &mut Keyboard) {
    for k in 0..config::KEY_COUNT {
      let key: &Key = keyboard.key(k);
      if key.index != self.index && key.just_pressed() && !self.has_state(INTERRUPT) {
        self.state |= INTERRUPT;
      }
    }
  }

  fn is_sendable(&mut self) -> bool {
    if !self.has_state(SEND_ENABLED) {
      return false;
    }

    if self.has_state(INTERRUPT) {
      self.state &= !SEND_INSTANT;
      self.state &= !SEND_RELEASE;
      self.state &= !INTERRUPT;
      self.delay = 0;
      return true;
    }

    if self.delay > 0 {
      if time::elapsed(self.timestamp) < self.delay {
        return false;
      }
    }

    if self.has_state(SEND_INSTANT) {
      self.del_state(SEND_INSTANT);
      return true;
    }

    if self.has_state(SEND_RELEASE) {
      if self.is_released() {
        self.del_state(SEND_RELEASE);
        return true;
      }
      return false;
    }

    self.has_state(SEND_ENABLED)
  }

  #[cfg(feature = "behavior_tap_enabled")]
  fn process_taps(&mut self) {
    let time = self.time();
    if time <= self.tapping_term() {
      if self.taps < u8::MAX {
        self.taps += 1;
      }
    } else {
      self.taps = 0;
    }
  }

  fn debounce(&mut self, wanted_state: bool) -> bool {
    if self.has_state(DEBOUNCING) {
      if time::elapsed(self.debounce_timestamp) >= config::DEBOUNCE_TIME {
        self.del_state(DEBOUNCING);
      } else {
        return self.is_pressed();
      }
    }

    if wanted_state != self.is_pressed() {
      self.add_state(DEBOUNCING);
      self.debounce_timestamp = time::now();
      return wanted_state;
    }

    self.is_pressed()
  }

  fn has_state(&self, state: u8) -> bool {
    (self.state & state) != 0
  }

  fn add_state(&mut self, state: u8) {
    self.state |= state;
  }

  fn del_state(&mut self, state: u8) {
    self.state &= !state;
  }
}
