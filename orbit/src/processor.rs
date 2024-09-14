#![allow(dead_code)]
#![allow(unused_variables)]

use crate::orbit::keyboard::Keyboard;
use embassy_usb::driver::Driver;

pub async fn run<D: Driver<'static>>(usb_driver: D) -> ! {
  let mut keyboard = Keyboard::new();
  loop {
    keyboard.process().await;
  }
}

pub async fn test() -> ! {
  let mut keyboard = Keyboard::new();
  loop {
    keyboard.process().await;
  }
}
