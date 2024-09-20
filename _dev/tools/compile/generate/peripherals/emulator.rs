use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

#[allow(unused_variables)]
pub fn generate(inputs: &Vec<String>, outputs: &Vec<String>) -> TokenStream {
  // *header = quote! {
  //   use device_query::{DeviceQuery, DeviceState, Keycode};

  //   struct Input {
  //     key: Keycode,
  //     device_state: DeviceState,
  //   }

  //   impl Input {
  //     pub fn new(key: Keycode) -> Self {
  //       Self {
  //         device_state: DeviceState::new(),
  //         key,
  //       }
  //     }

  //     pub fn is_high(&self) -> bool {
  //       let keys: Vec<Keycode> = self.device_state.get_keys();
  //       keys.contains(&self.key)
  //     }

  //     pub fn is_low(&self) -> bool {
  //       let keys: Vec<Keycode> = self.device_state.get_keys();
  //       !keys.contains(&self.key)
  //     }
  //   }

  //   struct Output {
  //   }

  //   impl Output {
  //     pub fn new() -> Self {
  //       Self {
  //       }
  //     }

  //     pub fn read(&self) -> u16 {
  //       0
  //     }
  //   }
  // };

  // for input in inputs {
  //   let key = Ident::new(&input, Span::call_site());
  //   let definition = quote! {
  //     Input::new(p.#key)
  //   };
  //   input_definitions.push(definition);
  // }

  // for output in outputs {
  //   // let pin = Ident::new(&output, Span::call_site());
  //   let definition = quote! {
  //     Output::new()
  //   };
  //   output_definitions.push(definition);
  // }

  let generated = quote! {};

  generated
}
