#![no_std]
#![no_main]

#[cfg(not(feature = "debug"))]
use panic_halt as _;
#[cfg(feature = "debug")]
use {defmt_rtt as _, panic_probe as _};

use crate::orbit::dbg::*;
use embassy_executor::Spawner;
use embassy_stm32::{
  bind_interrupts,
  peripherals::USB_OTG_FS,
  time::Hertz,
  usb_otg::{Config as DriverConfig, Driver, InterruptHandler},
  Config,
};
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    OTG_FS => InterruptHandler<USB_OTG_FS>;
});

fn setup_clocks(config: &mut embassy_stm32::Config) {
  use embassy_stm32::rcc::*;

  // Set the HSI as the system clock source
  config.rcc.hse = None; // No HSE used in this configuration
  config.rcc.sys = Sysclk::HSI; // Use HSI as the system clock

  // Configure the PLL settings
  config.rcc.pll_src = PllSource::HSI; // Set PLL source to HSI

  // Adjust the PLL settings to achieve the desired clock frequencies
  config.rcc.pll = Some(Pll {
    prediv: PllPreDiv::DIV16,  // No pre-division
    mul: PllMul::MUL192,       // Multiply by 12 (16 MHz * 12 = 192 MHz)
    divp: Some(PllPDiv::DIV4), // AHB at 48 MHz (192 MHz / 4)
    divq: Some(PllQDiv::DIV4), // USB clock at 48 MHz (192 MHz / 4)
    divr: None,                // Optional; unused in this case
  });

  // Set AHB and APB prescalers
  config.rcc.ahb_pre = AHBPrescaler::DIV1; // No division, keep it at 48 MHz
  config.rcc.apb1_pre = APBPrescaler::DIV2; // APB1 runs at half the AHB frequency (24 MHz)
  config.rcc.apb2_pre = APBPrescaler::DIV1; // APB2 runs at 48 MHz

  // Use the PLL1_P as the system clock
  config.rcc.sys = Sysclk::PLL1_P; // Set the system clock to PLL1_P
}

mod orbit;
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
  // panic!
  let mut config = Config::default();
  setup_clocks(&mut config);
  let p = embassy_stm32::init(config);

  let mut config = DriverConfig::default();
  config.vbus_detection = false;

  static EP_OUT_BUFFER: StaticCell<[u8; 256]> = StaticCell::new();
  let driver = Driver::new_fs(
    p.USB_OTG_FS,
    Irqs,
    p.PA12,
    p.PA11,
    &mut EP_OUT_BUFFER.init([0; 256])[..],
    config,
  );

  orbit::process::run(driver).await
}
