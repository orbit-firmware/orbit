use crate::orbit::key::Key;
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

#[cfg(feature = "behavior_tap_enabled")]
mod press;

#[cfg(feature = "behavior_combo_enabled")]
mod combo;

#[cfg(feature = "behavior_hold_enabled")]
mod hold;

#[cfg(feature = "behavior_modding_enabled")]
mod modding;

#[cfg(feature = "behavior_tap_enabled")]
mod tap;

pub fn process(key: &Key) -> bool {
  let mut finished = false;
  // Order of processing is important!

  // if key.pressed() {
  //   dump!("taps {}", key.taps());
  // }

  if (key.pressed() && key.time() > 300000) {
    dump!("held");
    finished = true;
  }

  finished

  // #[cfg(feature = "behavior_modding_enabled")]
  // if (!modding::process(&key)) {
  //   return;
  // }

  // #[cfg(feature = "behavior_hold_enabled")]
  // if (!hold::process(&key)) {
  //   return;
  // }

  // #[cfg(feature = "behavior_tap_enabled")]
  // if (!tap::process(&key)) {
  //   return;
  // }

  // #[cfg(feature = "behavior_combo_enabled")]
  // if (!combo::process(&key)) {
  //   return;
  // }

  // #[cfg(feature = "behavior_press_enabled")]
  // if (!press::process(&key)) {
  //   return;
  // }
}
