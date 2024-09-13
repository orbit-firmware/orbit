pub mod behaviors;
pub mod config;
pub mod keyboard;
pub mod keycodes;
pub mod keymap;
pub mod log;
pub mod modifiers;
pub mod time;

use embassy_usb::driver::Driver;

#[allow(unused_variables)]
pub async fn run<D: Driver<'static>>(usb_driver: D) -> ! {
  let mut keyboard = keyboard::Keyboard::new();
  loop {
    keyboard.process().await;
  }
}
