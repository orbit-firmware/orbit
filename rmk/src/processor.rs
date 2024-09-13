use embassy_usb::driver::Driver;
use crate::rmk::keyboard::Keyboard;

#[allow(unused_variables)]
pub async fn run<D: Driver<'static>>(usb_driver: D) -> ! {
  let mut keyboard = Keyboard::new();
  loop {
    keyboard.process().await;
  }
}
