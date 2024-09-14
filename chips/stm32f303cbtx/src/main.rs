#![no_std]
#![no_main]

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;

use embassy_stm32::{
  bind_interrupts,
  peripherals::USB,
  usb::{Driver, InterruptHandler},
};

bind_interrupts!(struct Irqs {
  USB_LP_CAN_RX0 => InterruptHandler<USB>;
});

mod orbit;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
  let p = embassy_stm32::init(Default::default());
  let usb_driver = Driver::new(p.USB, Irqs, p.PA12, p.PA11);

  // macros::pinout! {};
  orbit::processor::run(usb_driver).await
}
