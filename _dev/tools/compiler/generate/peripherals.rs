const TARGET_FILE: &str = "src/orbit/peripherals.rs";

use crate::toml;
use crate::util;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

mod emulator;
mod stm32;

#[allow(unused_variables)]
pub fn generate(feature_list: &mut Vec<String>, chip: &str) {
  let config = toml::read("keyboard.toml", true);
  let chip: String = toml::get(&config, "keyboard/chip", true);
  let use_matrix: bool = toml::contains(&config, "matrix");
  let use_multiplexers: bool = toml::contains(&config, "multiplexers");

  let mut inputs: Vec<String> = vec![];
  let mut outputs: Vec<String> = vec![];

  if use_matrix {
    let row_pins: Vec<String> = toml::get(&config, "matrix/row_pins", true);
    let col_pins: Vec<String> = toml::get(&config, "matrix/col_pins", true);
    inputs.extend(row_pins);
    inputs.extend(col_pins);
  }

  if use_multiplexers {
    let sel_pins: Vec<String> = toml::get(&config, "multiplexers/sel_pins", true);
    let com_pins: Vec<String> = toml::get(&config, "multiplexers/com_pins", true);
    outputs.extend(sel_pins);
    inputs.extend(com_pins);
  }

  #[rustfmt::skip]
  let generated = generate_chip(
    &chip,
    &inputs,
    &outputs,
  );

  // util::write(TARGET_FILE, util::quote_to_string(generated).as_str());
}

fn generate_chip(chip: &str, inputs: &Vec<String>, outputs: &Vec<String>) -> TokenStream {
  if chip == "_emulator" {
    return emulator::generate(inputs, outputs);
  }

  if chip.starts_with("stm32") {
    return stm32::generate(inputs, outputs);
  }

  return quote! {};
}
