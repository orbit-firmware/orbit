#[allow(dead_code)]
#[repr(u8)]
pub enum Behaviors {
  Press, // press is always enabled
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

pub fn process() {}
