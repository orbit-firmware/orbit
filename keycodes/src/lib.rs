extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::fs;
use syn::Ident;

struct KeyCode {
  name: Ident,
  code: u16,
  alias: Vec<Ident>,
}

fn read_kcs_file(path: &str) -> Vec<KeyCode> {
  let kcs_path = format!("keycodes/definitions/{}.k", path);
  let msg = format!("Failed to read keycodes file at {:?}", kcs_path);
  let content = fs::read_to_string(&kcs_path).expect(msg.as_str());

  let mut keycodes: Vec<KeyCode> = vec![];

  for line in content.lines() {
    let mut keycode = KeyCode {
      name: Ident::new("Unknown", Span::call_site()),
      code: 0,
      alias: vec![],
    };

    let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
    if parts.len() == 2 {
      let msg = "Failed to parse keycode value";
      let trimmed_code = parts[1].trim_start_matches("0x");
      let code = u16::from_str_radix(trimmed_code, 16).expect(msg);

      keycode.name = Ident::new(parts[0], Span::call_site());
      keycode.code = code;
      keycodes.push(keycode);
    }

    if parts.len() == 3 {
      println!("Found alias: {:?}", parts[2]);
    }
  }

  return keycodes;
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
  let input_str = input.to_string();
  let lang = input_str.trim_matches('"');

  let mut keycodes = read_kcs_file("core");
  let overrides = read_kcs_file(lang);

  keycodes.extend(overrides);

  let mut enum_variants = vec![];

  for keycode in keycodes {
    let name = keycode.name;
    let code = keycode.code;

    enum_variants.push(quote! {
      #name = #code,
    });
  }

  let generated_enum = quote! {
      #[derive(Debug, Copy, Clone, PartialEq, Eq)]
      #[repr(u16)]
      pub enum KeyCodes {
          #(#enum_variants)*
      }
  };

  TokenStream::from(generated_enum)
}
