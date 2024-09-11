#![no_std]
#![no_main]
#![allow(unused_imports)]

mod fmt;

use defmt::println;
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::{
  bind_interrupts,
  flash::Flash,
  gpio::{Input, Output},
  peripherals::USB,
  usb::{Driver, InterruptHandler},
  // Config,
};
use embassy_time::{Duration, Timer};
use fmt::info;
use static_cell::StaticCell;

use rmk::pinout::Pinout;

bind_interrupts!(struct Irqs {
  USB_LP_CAN_RX0 => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
  let p = embassy_stm32::init(Default::default());
  let usb_driver = Driver::new(p.USB, Irqs, p.PA12, p.PA11);

  let pinout = Pinout::new();

  rmk::run(usb_driver, pinout).await
}
