use crate::orbit::key::Key;
use crate::orbit::keyboard::Keyboard;
use crate::orbit::log::dump;

#[allow(dead_code)]
#[repr(u8)]
pub enum Behaviors {
  // Press is always enabled
  Press,
  Hold,
  Tap,
  Modding,
}

// #[cfg(feature = "behavior_tap_enabled")]
// mod press;

// #[cfg(feature = "behavior_combo_enabled")]
// mod combo;

// #[cfg(feature = "behavior_hold_enabled")]
// mod hold;

// #[cfg(feature = "behavior_modding_enabled")]
// mod modding;

// #[cfg(feature = "behavior_tap_enabled")]
// mod tap;

pub fn process(keyboard: &mut Keyboard, key: &mut Key) {
  if key.pressed() && key.time() > 1000 {
    key.send();
  }

  // #[cfg(feature = "behavior_modding_enabled")]
  // modding::process(&keyboard, &key);
  // if key.processed() {
  //   return;
  // }

  // #[cfg(feature = "behavior_hold_enabled")]
  // hold::process(&keyboard, &key);
  // if key.processed() {
  //   return;
  // }

  // #[cfg(feature = "behavior_tap_enabled")]
  // tap::process(&keyboard, &key);
  // if key.processed() {
  //   return;
  // }

  // #[cfg(feature = "behavior_combo_enabled")]
  // combo::process(&keyboard, &key);
  // if key.processed() {
  //   return;
  // }

  // #[cfg(feature = "behavior_press_enabled")]
  // press::process(&keyboard, &key);
  // if key.processed() {
  //   return;
  // }
}
