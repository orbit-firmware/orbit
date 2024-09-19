use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub fn generate(key_count: usize, gpio_pins: Vec<String>) -> (TokenStream, TokenStream) {
  let mut device_keys = vec![];

  let mut count = -1;

  for gpio in gpio_pins {
    count += 1;
    let ident = Ident::new(&gpio, Span::call_site());
    device_keys.push(quote! {
      self.gpio[#count as usize] = keys.contains(&Keycode::#ident);
    });
  }

  let gpio_scan = quote! {
    let keys: Vec<Keycode> = self.device_state.get_keys();
    #(#device_keys)*
  };

  let key_scan = quote! {
    for i in 0..#key_count {
      self.keys[i] = self.gpio[i];
    }
  };

  (gpio_scan, key_scan)
}
