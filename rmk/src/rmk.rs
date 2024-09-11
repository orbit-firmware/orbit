#![no_main]
#![no_std]

pub mod config;
pub mod keyboard;
pub mod keycodes;
pub mod modifiers;
pub mod time;

use embassy_usb::driver::Driver;

pub use keycodes::KeyCode;
pub use modifiers::Modifier;

#[allow(unused_variables)]
pub async fn run<D: Driver<'static>>(usb_driver: D, pinout: config::Pinout) -> ! {
  let mut keyboard = keyboard::Keyboard::new();
  loop {
    keyboard.scan().await;
    // Your processing logic here
    keyboard.send().await;
  }
}
