#![no_std]
#![no_main]
#![allow(unused_imports)]

mod fmt;

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
  Config,
};
use embassy_time::{Duration, Timer};
use fmt::info;
use static_cell::StaticCell;

use rmk::config::{self, Config};

bind_interrupts!(struct Irqs {
  USB_LP_CAN_RX0 => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
  let p = embassy_stm32::init(Default::default());
  let usb_driver = Driver::new(p.USB, Irqs, p.PA12, p.PA11);

  let mut config = Config::new();
  // config.set_usb_driver(usb_driver);

  loop {
    // info!("{}", KeyCode::from_alias("a") as u16);
  }
}
