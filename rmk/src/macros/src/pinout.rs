use proc_macro::TokenStream;
use quote::quote;

pub fn generate(_input: TokenStream) -> TokenStream {
  // Define the class prefix
  let cls = "embassy_stm32::gpio::";

  // Generate the tokens with the class prefix included
  let generated = quote! {
      let PIN = #cls Output::new(p.PB3, #cls Level::High, #cls Speed::Low);
  };

  generated.into()
}
