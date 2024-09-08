#![no_std]
#![no_main]
#![allow(unused_imports)]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};

use embassy_time::{Duration, Timer};
use fmt::info;

use rmk::keycode::KeyCode;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
  let _p = embassy_stm32::init(Default::default());
  loop {
    info!("{}", KeyCode::from_alias("a") as u16);
  }
}
