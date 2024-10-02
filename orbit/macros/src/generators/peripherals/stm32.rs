use proc_macro2::{Ident,
  TokenStream};
use quote::quote;

// TODO: make this more generic
// to just return a list of pins
#[allow(unused_variables)]
pub fn generate_chip_peripherals(
  inputs: &Vec<Ident>,
  outputs: &Vec<Ident>,
) -> (
  TokenStream,
  TokenStream,
  TokenStream,
  TokenStream,
  TokenStream,
  TokenStream,
) {

  // https://dev.to/theembeddedrustacean/embedded-rust-embassy-analog-sensing-with-adcs-1e2n
  // somehow provide read function for ADC for input pins if the cfg is active
  let header = quote! {
    use embassy_stm32::gpio::{Input, Output, Level, Speed, Pull, AnyPin};
    use embassy_stm32::Peripherals as Stm32Peripherals;
  };

  let init = quote! {
    let p = unsafe { Stm32Peripherals::steal() };
  };

  let input_definition = quote! {
    Input<'static, AnyPin>
  };

  let input_declaration = quote! {
    [#(Input::new(p.#inputs, Pull::Down).degrade(),)*]
  };

  let output_definition = quote! {
    Output<'static, AnyPin>
  };

  let output_declaration = quote! {
    [#(Output::new(p.#output, Level::Low, Speed::VeryHigh).degrade(),)*]
  };

  #[rustfmt::skip]  
  (
    header,
    init,
    input_definition,
    input_declaration,
    output_definition,
    output_declaration
  )
}
