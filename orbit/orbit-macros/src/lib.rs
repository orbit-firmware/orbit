use proc_macro::TokenStream;

mod generators;
mod modifiers;
mod toml;
mod util;

#[proc_macro]
pub fn generate_config(input: TokenStream) -> TokenStream {
  generators::config::generate(input)
}

#[proc_macro]
pub fn generate_keycodes(input: TokenStream) -> TokenStream {
  generators::keycodes::generate(input)
}

#[proc_macro]
pub fn generate_peripherals(input: TokenStream) -> TokenStream {
  generators::peripherals::generate(input)
}
