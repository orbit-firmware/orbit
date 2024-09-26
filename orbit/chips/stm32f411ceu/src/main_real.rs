#![no_std]
#![no_main]

#[cfg(not(feature = "debug"))]
use panic_halt as _;
#[cfg(feature = "debug")]
use {defmt_rtt as _, panic_probe as _};

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
  config.rcc.hse = Some(Hse {
    freq: Hertz(25_000_000),
    mode: HseMode::Oscillator,
  });
  config.rcc.pll_src = PllSource::HSE;
  config.rcc.pll = Some(Pll {
    prediv: PllPreDiv::DIV25,
    mul: PllMul::MUL336,
    divp: Some(PllPDiv::DIV4), // 25mhz / 25 * 336 / 4 = 84Mhz.
    divq: Some(PllQDiv::DIV7), // 25mhz / 25 * 336 / 7 = 48Mhz.
    divr: None,
  });
  config.rcc.ahb_pre = AHBPrescaler::DIV1;
  config.rcc.apb1_pre = APBPrescaler::DIV2;
  config.rcc.apb2_pre = APBPrescaler::DIV1;
  config.rcc.sys = Sysclk::PLL1_P;
}

mod orbit;
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
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
