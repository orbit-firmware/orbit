use crate::orbit::key::Key;

#[allow(dead_code)]
#[repr(u8)]
pub enum Actions {
  Mouse,
}

#[cfg(feature = "action_mouse_enabled")]
mod mouse;

#[allow(dead_code)]
#[allow(unused)]
pub fn process(key: &Key) {
  // check if the keymapping has an action
  // if it does, call the action
  // otherwise send the keycode
}
