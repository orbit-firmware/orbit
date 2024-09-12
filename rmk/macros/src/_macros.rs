// borrow from original source
// this is not the prefered way to use modules
// but it ensuures we always use the source code
#[path = "../../core/src/modifiers.rs"]
mod modifiers;

extern crate proc_macro;
use prettyplease::unparse;
use proc_macro::TokenStream;
use syn::{parse_file, File};
mod config;
mod keycodes;
mod pinout;

const DUMP_CONFIG: bool = true;
const DUMP_KEYCODES: bool = true;
const DUMP_PINOUT: bool = true;

fn dump(ts: &TokenStream, name: &str) {
  let parsed: File = parse_file(&ts.to_string()).unwrap();
  let code = unparse(&parsed).to_string();
  std::fs::write(format!("../dumps/{}.rs", name), code.as_bytes()).unwrap();
}

#[proc_macro]
pub fn keycodes(input: TokenStream) -> TokenStream {
  let ts = keycodes::generate(input);
  if DUMP_KEYCODES {
    dump(&ts, "keycodes");
  }
  ts
}

#[proc_macro]
pub fn config(input: TokenStream) -> TokenStream {
  let ts = config::generate(input);
  if DUMP_CONFIG {
    dump(&ts, "config");
  }
  ts
}

#[proc_macro]
pub fn pinout(input: TokenStream) -> TokenStream {
  let ts = pinout::generate(input);
  if DUMP_PINOUT {
    dump(&ts, "pinout");
  }
  ts
}
