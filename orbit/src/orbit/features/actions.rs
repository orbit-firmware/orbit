use crate::orbit::dbg::dump;
use crate::orbit::key::Key;
use crate::orbit::keyboard::Keyboard;
use crate::orbit::peripherals::*;

#[allow(dead_code)]
#[repr(u8)]
pub enum Actions {
  Layers,
  Mouse,
}

#[cfg(feature = "action_mouse_enabled")]
mod mouse;

#[cfg(feature = "action_layers_enabled")]
mod layers;

impl Actions {
  #[allow(dead_code)]
  #[allow(unused)]
  pub fn process(keyboard: &mut Keyboard, key: &mut Key) {
    dump!("Actions::process");
    // if key.is_pressed() {
    //   keyboard.peripherals().output(Peripheral::PB9).set_high();
    // } else {
    //   keyboard.peripherals().output(Peripheral::PB9).set_low();
    // }
  }
}
