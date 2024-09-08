use crate::types::KeyCode;
use proc_macro2::Span;
use std::fs;
use syn::Ident;

fn valid_line(line: &str) -> bool {
  if line.is_empty() {
    return false;
  }

  if line.starts_with("#") {
    return false;
  }

  true
}

pub fn file(path: &str) -> Vec<KeyCode> {
  let kcs_path = format!("rmk_macros/definitions/{}.k", path);
  let msg = format!("Failed to read keycodes file at {:?}", kcs_path);
  let content = fs::read_to_string(&kcs_path).expect(msg.as_str());

  let mut keycodes: Vec<KeyCode> = vec![];

  for line in content.lines() {
    if !valid_line(line) {
      continue;
    }
    let mut keycode = KeyCode::new();

    let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
    if parts.len() >= 2 {
      keycode.name = Ident::new(parts[0], Span::call_site());
      keycode.code_str = parts[1].trim().to_string();
      keycodes.push(keycode);
    }

    // if parts.len() == 3 {
    //   println!("Found alias: {:?}", parts[2]);
    // }
  }

  return keycodes;
}
