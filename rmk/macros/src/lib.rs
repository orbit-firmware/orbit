extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

mod parser;
mod read;
mod types;

#[proc_macro]
pub fn keycodes(input: TokenStream) -> TokenStream {
  let input_str = input.to_string();
  let remap = input_str.trim_matches('"');

  let keycodes = read::file("_common");
  let remaps = read::file(remap);
  let merged = parser::merge(keycodes, remaps);

  let mut entries = vec![];
  let mut match_arms = vec![];
  let mut alias_entries = vec![];

  for keycode in merged {
    let name = keycode.name;
    let code = keycode.code;
    let alias_list = keycode.alias_list;
    entries.push(quote! {
      #name = #code,
    });
    match_arms.push(quote! {
        #code => KeyCode::#name,
    });
    for alias in alias_list {
      alias_entries.push(quote! {
          #alias => KeyCode::#name,
      });
    }
  }

  let generated = quote! {
      #[derive(Debug, Copy, Clone, PartialEq, Eq)]
      #[repr(u16)]
      pub enum KeyCode {
          #(#entries)*
      }

      impl KeyCode {
        pub fn from_u16(code: u16) -> KeyCode {
            match code {
                #(#match_arms)*
                _ => KeyCode::None,
            }
        }

        pub fn from_alias(alias: &str) -> KeyCode {
          match alias {
              #(#alias_entries)*
              _ => KeyCode::None,
          }
        }
      }

  };

  TokenStream::from(generated)
}
