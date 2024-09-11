use embassy_stm32::Peripherals;

macros::config!();

pub struct Pinout {
  // pub keys: [Input; KEY_COUNT],
  // pub leds: [Output; LED_COUNT],
  // pub usb: USB,
}

impl Pinout {
  pub fn new(p: Peripherals) -> Self {
    // let p = embassy_stm32::init(Default::default());
    // let usb = USB::new(p.USB, Irqs, p.PA12, p.PA11);
    // let keys = [Input::new(p.PA0)];
    // let leds = [Output::new(p.PA1)];
    Self {
      // keys,
      // leds,
      // usb,
    }
  }
}
