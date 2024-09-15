use crate::orbit::key::Key;

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

#[allow(dead_code)]
#[allow(unused)]
pub fn process(key: &Key) {
  let a = 0;
  // Order of processing is important!

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
