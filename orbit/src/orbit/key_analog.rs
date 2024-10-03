use crate::config as KeyboardConfig;
use crate::prelude::*;

// 666 should be a sufficient range for the initial key press
// most ranges will be around ~800 when fully pressed
const INITIAL_RANGE: u16 = 666;
const RESOLUTION: u16 = 10000;
const CLEARANCE: u16 = 10; // 0.1%
const PRESS_THRESHOLD: u16 = 1_00; // 1%
const RELEASE_THRESHOLD: u16 = 5_00; // 5%

#[derive(Debug)]
pub struct AnalogKey {
  state: bool,
  debouncing: bool,
  debounce: u32,
  upper: u16,
  lower: u16,
  flip: u16,
}

impl AnalogKey {
  pub fn new() -> Self {
    Self {
      state: false,
      debouncing: false,
      debounce: 0,
      upper: u16::MIN,
      lower: u16::MAX,
      flip: RESOLUTION,
    }
  }

  #[inline(always)]
  fn diff(a: u16, b: u16) -> u16 {
    if a > b {
      a - b
    } else {
      b - a
    }
  }

  #[inline(always)]
  fn bounds(&mut self, value: u16) {
    if value > self.upper {
      self.upper = value;
    }

    if self.lower == u16::MAX && self.upper >= INITIAL_RANGE {
      self.lower = self.upper - INITIAL_RANGE;
    }

    if value < self.lower {
      self.lower = value;
    }
  }

  fn percent(&mut self, value: u16) -> u16 {
    self.bounds(value);
    let padding: u16 = 10;
    let lower = self.lower.saturating_add(padding);
    let upper = self.upper.saturating_sub(padding);
    let limited = value.clamp(lower, upper);

    let delta = limited.saturating_sub(lower);
    let range = upper.saturating_sub(lower);

    (delta as u32 * RESOLUTION as u32 / range as u32) as u16
  }

  fn debounce(&mut self, wanted: bool) -> bool {
    if self.debouncing {
      if elapsed(self.debounce) >= KeyboardConfig::DEBOUNCE_TIME {
        self.debouncing = false;
      } else {
        return self.state;
      }
    }

    if wanted != self.state {
      self.debouncing = true;
      self.debounce = now();
      return wanted;
    }

    self.state
  }

  pub fn process(&mut self, value: u16) -> bool {
    let percent = self.percent(value);

    let diff = Self::diff(percent, self.flip);
    if diff < CLEARANCE {
      return self.state;
    }

    let moving_down = percent < self.flip;

    let pressed = moving_down && (diff > PRESS_THRESHOLD || percent == 0);
    let released = !moving_down && (diff > RELEASE_THRESHOLD || percent == RESOLUTION);

    if pressed {
      self.state = self.debounce(true);
    } else if released {
      self.state = self.debounce(false);
    }

    self.flip = percent;
    self.state
  }
}
