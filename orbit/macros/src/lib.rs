use proc_macro::TokenStream;

mod generators;
mod modifiers;
mod toml;
mod util;

#[proc_macro]
pub fn generate_config(input: TokenStream) -> TokenStream {
  let mut out = generators::config::generate(input);
  out.extend(TokenStream::from(util::track_inputs()));
  out
}

#[proc_macro]
pub fn generate_keycodes(input: TokenStream) -> TokenStream {
  generators::keycodes::generate(input)
}

#[proc_macro]
pub fn generate_keymap(input: TokenStream) -> TokenStream {
  generators::keymap::generate(input)
}

#[proc_macro]
pub fn generate_peripherals(input: TokenStream) -> TokenStream {
  generators::peripherals::generate(input)
}
