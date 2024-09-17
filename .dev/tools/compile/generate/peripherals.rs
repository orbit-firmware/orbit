const TARGET_FILE: &str = "src/orbit/peripherals.rs";

use crate::toml;
use crate::util;
use quote::quote;

mod emulator;

#[allow(unused_variables)]
pub fn generate(feature_list: &mut Vec<String>, chip: &str) {
  let config = toml::read("keyboard.toml", true);
  let chip: String = toml::get(&config, "keyboard/chip", true);
  let use_matrix: bool = toml::contains(&config, "matrix");
  let use_multiplexers: bool = toml::contains(&config, "multiplexers");
  let mut layout_list: Vec<(usize, usize)> = vec![];

  let mut gpio_pins: Vec<String> = vec![];
  if use_matrix {
    layout_list = toml::get(&config, "matrix/layout", true);
    let row_pins: Vec<String> = toml::get(&config, "matrix/row_pins", true);
    let col_pins: Vec<String> = toml::get(&config, "matrix/col_pins", true);
    gpio_pins = row_pins.clone();
    gpio_pins.extend(col_pins.clone());
  }

  if use_multiplexers {
    layout_list = toml::get(&config, "multiplexers/layout", true);
    let sel_pins: Vec<String> = toml::get(&config, "multiplexers/sel_pins", true);
    let com_pins: Vec<String> = toml::get(&config, "multiplexers/com_pins", true);
    gpio_pins = sel_pins.clone();
    gpio_pins.extend(com_pins.clone());
  }

  let gpio_count = gpio_pins.len();
  let key_count = layout_list.len();
  let mut gpio_scan = quote! {};
  let mut key_scan = quote! {};

  // emulator
  if chip == "_emulator" {
    (gpio_scan, key_scan) = emulator::generate(key_count, gpio_pins);
  }

  // STM32
  // NRF

  let generated = quote! {
    #[cfg(feature = "emulator_enabled")]
    use device_query::{DeviceQuery, DeviceState, Keycode};

    #[allow(dead_code)]
    pub struct Peripherals {
      #[cfg(feature = "emulator_enabled")]
      device_state: DeviceState,
      gpio: [bool; #gpio_count],
      pub keys: [bool; #key_count],
    }

    impl Peripherals {
      pub fn new() -> Peripherals {
        Self {
          #[cfg(feature = "emulator_enabled")]
          device_state: DeviceState::new(),
          gpio: [false; #gpio_count],
          keys: [false; #key_count],
        }
      }

      pub fn scan(&mut self) {
        #gpio_scan
        #key_scan
      }

      pub fn key(&self, index: usize) -> bool {
        self.keys[index]
      }
    }
  };

  util::write(TARGET_FILE, util::quote_to_string(generated).as_str());
}
