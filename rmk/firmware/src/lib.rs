#![no_main]
#![no_std]

pub mod features;
pub mod flow;
pub mod key;
pub mod keyboard;
pub mod keycodes;
pub mod keymap;
pub mod time;

pub use crate::keyboard::Keyboard;

pub fn setup<const KEY_COUNT: usize>() {}

#[allow(unused_variables)]
pub fn process_keyboard<const KEY_COUNT: usize>(keyboard: &Keyboard<KEY_COUNT>) {
  // Your processing logic here
}

// #[allow(unused_variables)]
// pub fn process_encoders<const ENCODER_COUNT: usize>(encoders: &Encoders<LED_COUNT>) {
//   // Your processing logic here
// }

// #[allow(unused_variables)]
// pub fn process_key_leds<const LED_COUNT: usize>(key_leds: &KeyLEDs<LED_COUNT>) {
//   // Your processing logic here
// }

// #[allow(unused_variables)]
// pub fn process_underglow<const LED_COUNT: usize>(underglow: &Underglow<LED_COUNT>) {
//   // Your processing logic here
// }

// pub(crate) mod _generated {
//   #![allow(dead_code)]
//   #![allow(unused_imports)]
//   #![allow(non_snake_case)]
//   #![allow(missing_docs)]

//   include!(concat!(env!("OUT_DIR"), "/_generated.rs"));
// }
