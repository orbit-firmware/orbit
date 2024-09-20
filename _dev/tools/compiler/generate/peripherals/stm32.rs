use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

#[allow(unused_variables)]
pub fn generate(inputs: &Vec<String>, outputs: &Vec<String>) -> TokenStream {
  // let mut input_mappings: Vec<Ident> = vec![];
  // let mut output_mappings: Vec<Ident> = vec![];

  // for input in inputs {
  //   let ident = Ident::new(&input, Span::call_site());
  //   input_mappings.push(ident);
  // }

  // for output in outputs {
  //   let ident = Ident::new(&output, Span::call_site());
  //   output_mappings.push(ident);
  // }

  // let generated = quote! {
  //   // use embassy_stm32::gpio::{Input, Output, Level, Speed, Pull};
  //   // use embassy_stm32::Peripherals as Stm32Peripherals;
  //   // use embedded_hal::digital::{InputPin, OutputPin};

  //   // pub enum Peripheral {
  //   //   None,
  //   //   #(#input_mappings),*
  //   //   #(#output_mappings),*
  //   // }

  //   // pub struct Peripherals {
  //   //   p: Stm32Peripherals,
  //   // }

  //   // impl Peripherals {
  //   //   pub fn new(p: Stm32Peripherals) -> Self {
  //   //     Self {
  //   //       p
  //   //     }
  //   //   }

  //   //   pub fn input(&self, pin: Peripheral) -> impl InputPin {
  //   //     match pin {
  //   //       #(
  //   //         Peripheral::#input_mappings => Input::new(self.p.#input_mappings, Pull::Up),
  //   //       )*
  //   //       _ => panic!("Invalid input peripheral"),
  //   //     }
  //   //   }

  //   //   // pub fn output(&self, pin: Peripheral) -> impl OutputPin {
  //   //   //   match pin {
  //   //   //     #(
  //   //   //       Peripheral::#output_mappings => Output::new(embassy_stm32::peripherals::#output_mappings, Level::Low, Speed::VeryHigh),
  //   //   //     )*
  //   //   //     _ => panic!("Invalid output peripheral"),
  //   //   //   }
  //   //   // }
  //   }
  // };

  let generated = quote! {};
  generated
}
