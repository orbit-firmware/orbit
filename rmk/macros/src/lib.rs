extern crate proc_macro;
use proc_macro::TokenStream;

mod config;
mod keycodes;

#[proc_macro]
pub fn keycodes(input: TokenStream) -> TokenStream {
  keycodes::generate(input)
}

#[proc_macro]
pub fn config(input: TokenStream) -> TokenStream {
  config::generate(input)
}
