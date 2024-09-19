#![no_std]
#![no_main]

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use static_cell::StaticCell;
use embassy_stm32::{
  bind_interrupts,
  flash::Flash,
  gpio::{Input, Output},
  peripherals::USB_OTG_FS,
  usb_otg::{Driver, InterruptHandler},
  Config,
};

bind_interrupts!(struct Irqs {
  OTG_FS => InterruptHandler<USB_OTG_FS>;
});

mod orbit;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
  let p = embassy_stm32::init(Default::default());

  static EP_OUT_BUFFER: StaticCell<[u8; 1024]> = StaticCell::new();
  let mut usb_config = embassy_stm32::usb_otg::Config::default();
  usb_config.vbus_detection = false;
  let usb_driver = Driver::new_fs(
      p.USB_OTG_FS,
      Irqs,
      p.PA12,
      p.PA11,
      &mut EP_OUT_BUFFER.init([0; 1024])[..],
      usb_config,
  );
  
  loop {
    orbit::process::run(usb_driver).await
  }
  
}
