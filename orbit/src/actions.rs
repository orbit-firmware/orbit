use crate::orbit::key::Key;
use crate::orbit::keyboard::Keyboard;
use crate::orbit::log::dump;

// #[allow(dead_code)]
// #[repr(u8)]
// pub enum Actions {
//   Mouse,
// }

// #[cfg(feature = "action_mouse_enabled")]
// mod mouse;

#[allow(dead_code)]
#[allow(unused)]
pub fn process(keyboard: &mut Keyboard, key: &mut Key) {
  // check if the keymapping has an action
  // if it does, call the action
  // otherwise send the keycode
}
