use proc_macro::TokenStream;
use quote::quote;

mod read;

pub fn generate(_input: TokenStream) -> TokenStream {
  let config = read::file("tmp/config.toml");

  let custom_config = read::file("user/config.toml");

  println!("Config: {}", config);
  println!("Config: {}", custom_config);

  let generated = quote! {
    use crate::chip::Chip;
    pub struct Config {
      chip: Chip,
    }
  };

  TokenStream::from(generated)
}
