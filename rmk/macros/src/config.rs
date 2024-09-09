use proc_macro::TokenStream;
use quote::quote;
use toml::Table;

mod read;

pub fn generate(_input: TokenStream) -> TokenStream {
  let config = read::file("tmp/config.toml", false);

  // let custom_config = read::file("user/config.toml", true);

  let config_value = config.parse::<Table>().unwrap();
  let keyboard = config_value.get("keyboard").unwrap().as_table().unwrap();
  let name = keyboard.get("name").unwrap().as_str().unwrap();
  let manufacturer = keyboard.get("manufacturer").unwrap().as_str().unwrap();
  let chip = keyboard.get("chip").unwrap().as_str().unwrap();

  let generated = quote! {
    pub struct Config {}

    impl Config {

      pub fn new() -> Self {
        Self {}
      }
      
      pub fn get_name(&self) -> &str {
        #name
      }

      pub fn get_manufacturer(&self) -> &str {
        #manufacturer
      }

      pub fn get_chip(&self) -> &str {
        #chip
      }

    }
  };

  TokenStream::from(generated)
}
