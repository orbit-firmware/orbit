use crate::orbit::key::Key;
use crate::orbit::keyboard::Keyboard;

// get from config later
const HOLD_TIME: u64 = 1000;

#[allow(unused_variables)]
pub fn process(keyboard: &Keyboard, key: &mut Key) {
  if key.just_pressed() {
    key.send_next();
  }

  if key.pressed() && key.time() > HOLD_TIME {
    key.send_now();
  }
}
