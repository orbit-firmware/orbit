use crate::orbit::config as Orbit;

// debounces one physical key: a change is taken at once, then further changes are
// ignored for DEBOUNCE_TIME ms
#[derive(Clone, Copy)]
pub struct Key {
  pressed: bool,
  changed: u32,
}

impl Key {
  pub fn new() -> Key {
    Key { pressed: false, changed: 0 }
  }

  pub fn update(&mut self, raw: bool, now: u32) -> bool {
    if raw != self.pressed && now.wrapping_sub(self.changed) >= Orbit::DEBOUNCE_TIME as u32 {
      self.pressed = raw;
      self.changed = now;
    }
    self.pressed
  }
}
