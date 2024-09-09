use proc_macro::TokenStream;
use quote::quote;

mod read;

pub fn generate(_input: TokenStream) -> TokenStream {
  let config = read::file("tmp/config.toml");

  println!("Config: {}", config);

  let generated = quote! {
    use crate::chip::Chip;
    pub struct Config {
      chip: Chip,
    }
  };

  TokenStream::from(generated)
}
