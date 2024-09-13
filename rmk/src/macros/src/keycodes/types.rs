use proc_macro2::Span;
use syn::Ident;

#[derive(Debug, Clone)]
pub struct KeyCode {
  pub name: Ident,
  pub code_str: String,
  pub code: u16,
  pub alias_list: Vec<String>,
}

impl KeyCode {
  pub fn new() -> KeyCode {
    KeyCode {
      name: Ident::new("Unknown", Span::call_site()),
      code_str: String::from("0"),
      code: 0,
      alias_list: vec![],
    }
  }
}
