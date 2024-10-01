use crate::toml;
use crate::util;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

#[cfg(feature = "chip_type_emulator")]
mod emulator;
#[cfg(feature = "chip_type_emulator")]
use emulator::generate_chip_peripherals;

#[cfg(feature = "chip_type_stm32")]
mod stm32;
#[cfg(feature = "chip_type_stm32")]
use stm32::generate_chip_peripherals;

#[allow(unused_variables)]
pub fn generate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let root = util::get_root();
  let path = format!("{}/build/keyboard.toml", root);
  let config = toml::read(&path, true);

  #[cfg(feature = "chip_type_stm32")]
  println!("Generating peripherals for STM32");

  #[cfg(feature = "chip_type_emulator")]
  println!("Generating peripherals for emulator");

  let use_matrix: bool = toml::contains(&config, "matrix");
  let use_multiplexers: bool = toml::contains(&config, "multiplexers");

  let mut inputs: Vec<Ident> = vec![];
  let mut outputs: Vec<Ident> = vec![];

  if use_matrix {
    let row_pins: Vec<String> = toml::get(&config, "matrix/row_pins", true);
    let col_pins: Vec<String> = toml::get(&config, "matrix/col_pins", true);
    for row in row_pins.clone() {
      let ident = Ident::new(&row, Span::call_site());
      inputs.push(ident);
    }
    for col in col_pins.clone() {
      let ident = Ident::new(&col, Span::call_site());
      inputs.push(ident);
    }
  }

  if use_multiplexers {
    let sel_pins: Vec<String> = toml::get(&config, "multiplexers/sel_pins", true);
    let com_pins: Vec<String> = toml::get(&config, "multiplexers/com_pins", true);
    for sel in sel_pins.clone() {
      let ident = Ident::new(&sel, Span::call_site());
      outputs.push(ident);
    }
    for com in com_pins.clone() {
      let ident = Ident::new(&com, Span::call_site());
      inputs.push(ident);
    }
  }

  let peripheral_enum = generate_enum(&inputs, &outputs);
  let orbit_io = generate_orbit_io();
  let peripherals = generate_peripherals(&inputs, &outputs);

  quote! {
    use embedded_hal::digital::{InputPin, OutputPin};
    use core::convert::Infallible;

    #peripheral_enum
    #orbit_io
    #peripherals
  }
  .into()
}

fn generate_enum(inputs: &Vec<Ident>, outputs: &Vec<Ident>) -> TokenStream {
  let mut mappings: Vec<Ident> = vec![];
  mappings.extend(inputs.clone());
  mappings.extend(outputs.clone());

  let index_cases = mappings.iter().enumerate().map(|(i, ident)| {
    quote! {
        Peripheral::#ident => #i,
    }
  });

  quote! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Peripheral {
      None,
      #(#mappings),*
    }

    impl Peripheral {
      pub fn index(&self) -> usize {
        match self {
          #(#index_cases)*
          Peripheral::None => panic!("Invalid peripheral"),
        }
      }
    }
  }
}

fn generate_orbit_io() -> TokenStream {
  quote! {
    pub struct OrbitInputPin<'a>(&'a mut dyn InputPin<Error = Infallible>);

    impl<'a> OrbitInputPin<'a> {
      #[allow(dead_code)]
      pub fn is_high(&mut self) -> bool {
        self.0.is_high().unwrap_or(false)
      }

      #[allow(dead_code)]
      pub fn is_low(&mut self) -> bool {
        self.0.is_low().unwrap_or(false)
      }

      #[allow(dead_code)]
      pub fn read(&mut self) -> u16 {
        0
      }
    }

    pub struct OrbitOutputPin<'a>(&'a mut dyn OutputPin<Error = Infallible>);

    impl<'a> OrbitOutputPin<'a> {

      #[allow(dead_code)]
      pub fn set_high(&mut self) {
          self.0.set_high().ok();
      }

      #[allow(dead_code)]
      pub fn set_low(&mut self) {
          self.0.set_low().ok();
      }
    }
  }
}

#[cfg(feature = "chip_type_none")]
fn generate_peripherals(inputs: &Vec<Ident>, outputs: &Vec<Ident>) -> TokenStream {
  panic!("No chip type defined in keyboard.toml");
}

#[cfg(not(feature = "chip_type_none"))]
fn generate_peripherals(inputs: &Vec<Ident>, outputs: &Vec<Ident>) -> TokenStream {
  let input_count = inputs.len();
  let output_count = outputs.len();

  let mut header = quote! {};
  let mut init = quote! {};
  let mut input_definition = quote! {};
  let mut input_declaration = quote! {};
  let mut output_definition = quote! {};
  let mut output_declaration = quote! {};

  (
    header, init, input_definition, input_declaration, output_definition, output_declaration,
  ) = generate_chip_peripherals(inputs, outputs);

  quote! {
    #header

    pub struct Peripherals {
      #[allow(dead_code)]
      inputs: [#input_definition; #input_count],
      #[allow(dead_code)]
      outputs: [#output_definition; #output_count],
    }

    impl Peripherals {
      pub fn new() -> Self {
        #init
        Self {
          inputs: #input_declaration,
          outputs: #output_declaration,
        }
      }

      #[allow(dead_code)]
      pub fn input(&mut self, pin: Peripheral) -> OrbitInputPin {
        let index = pin.index();
        OrbitInputPin(&mut self.inputs[index])
      }

      #[allow(dead_code)]
      pub fn output(&mut self, pin: Peripheral) -> OrbitOutputPin {
        let index = pin.index();
        OrbitOutputPin(&mut self.outputs[index - #input_count])
      }
    }
  }
}
