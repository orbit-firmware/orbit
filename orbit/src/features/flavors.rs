use crate::orbit::key::Key;
use crate::orbit::keyboard::Keyboard;
use crate::orbit::log::dump;

#[allow(dead_code)]
#[repr(u8)]
pub enum Flavors {
  SpaceCadet,
}

#[cfg(feature = "flavor_space_cadet_enabled")]
mod space_cadet;

impl Flavors {
  #[allow(dead_code)]
  #[allow(unused)]
  pub fn process(keyboard: &mut Keyboard, key: &mut Key) {}
}
