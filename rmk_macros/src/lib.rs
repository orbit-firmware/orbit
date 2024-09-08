extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;

mod modifiers;
mod parser;
mod read;
mod types;

#[proc_macro]
pub fn modifiers(_input: TokenStream) -> TokenStream {
  let modifiers = read::file("_modifiers");

  let mut entries = vec![];

  for modifier in modifiers {
    let name = modifier.name;
    let code = parser::code(&modifier.code_str, &vec![]);
    entries.push(quote! {
      #name = #code,
    });
  }

  let generated = quote! {
      #[derive(Debug, Copy, Clone, PartialEq, Eq)]
      #[repr(u16)]
      pub enum Modifier {
          #(#entries)*
      }
  };

  TokenStream::from(generated)
}

#[proc_macro]
pub fn keycodes(input: TokenStream) -> TokenStream {
  let input_str = input.to_string();
  let remap = input_str.trim_matches('"');

  let keycodes = read::file("_common");
  let remaps = read::file(remap);
  let merged = parser::merge(keycodes, remaps);

  let mut entries = vec![];

  for keycode in merged {
    let name = keycode.name;
    let code = keycode.code;
    entries.push(quote! {
      #name = #code,
    });
  }

  let generated = quote! {
      #[derive(Debug, Copy, Clone, PartialEq, Eq)]
      #[repr(u16)]
      pub enum KeyCode {
          #(#entries)*
      }

      // impl KeyCode {
      //     pub fn from_u16(code: u16) -> KeyCode {
      //         match code {
      //             #(
      //                 #enum_variants[0] => Some(KeyCodes::#enum_variants[1]),
      //             )*
      //             _ => KeyCode::None,
      //         }
      //     }

      //   get_by_alias(alias: &str) -> KeyCode {
      // }
      // }


      //  generate alias mapping

  };

  TokenStream::from(generated)
}
