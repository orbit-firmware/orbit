#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(not(feature = "emulator_enabled"))]
use crate::orbit::keyboard;
#[cfg(not(feature = "emulator_enabled"))]
use embassy_usb::driver::Driver;
#[cfg(not(feature = "emulator_enabled"))]
pub async fn run<D: Driver<'static>>(usb_driver: D) -> ! {
  let mut keyboard = Keyboard::instance();
  loop {
    keyboard.process().await;
  }
}


#[cfg(feature = "emulator_enabled")]
mod emulator;

#[cfg(feature = "emulator_enabled")]
pub async fn run() -> ! {
  emulator::emulate().await
}
