use crate::orbit::config;
use crate::orbit::key::Key;
use crate::orbit::keyboard::Keyboard;

const REGISTERED_TAPS: u32 = 4;

#[allow(unused_variables)]
pub fn process(keyboard: &Keyboard, key: &mut Key) {
  if key.just_pressed() {
    key.send_on_release_delayed(key.tapping_term());
  }

  for t in 0..REGISTERED_TAPS {
    let in_time = key.time() >= key.tapping_term();
    let is_tap = key.taps() == (t + 1) as u8;

    if key.is_released() && is_tap && in_time {
      key.send_oneshot();
    }
  }
}
