use crate::keycodes::types::KeyCode;
use proc_macro2::Span;
use std::fs;
use syn::Ident;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn valid_line(line: &str) -> bool {
  if line.is_empty() {
    return false;
  }

  if line.starts_with("#") {
    return false;
  }

  true
}

pub fn file(path: &str, log: bool) -> Vec<KeyCode> {
  let filepath = format!("rmk/keycodes/{}.k", path);

  let content = match fs::read_to_string(&filepath) {
    Ok(content) => content,
    Err(e) => {
      println!(
        "{}Keycodes file does not exist!: {}{}",
        RED, filepath, RESET
      );
      eprintln!(
        "Warning: Failed to read keycodes file at '{}': {}",
        filepath, e
      );
      std::process::exit(1);
    }
  };

  if log {
    println!("{}Using Keycodes: {}{}", GREEN, filepath, RESET);
  }

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

      let mut alias_list: Vec<String> = vec![];
      let main_alias = parts[0].to_lowercase();
      alias_list.push(main_alias);

      if parts.len() == 3 {
        let mut alias_string = parts[2].to_string();

        if alias_string.contains("\\") {
          alias_string = alias_string.replace("\\\\", "%%BS%%");
          alias_string = alias_string.replace("\\,", "%%COMMA%%");
        }

        let alias_parts = alias_string.split(',');
        for alias in alias_parts {
          let mut a = alias.trim().to_string();
          a = a.replace("%%BS%%", "\\");
          a = a.replace("%%COMMA%%", ",");
          alias_list.push(a);
        }
      }
      keycode.alias_list = alias_list.iter().map(|s| s.to_string()).collect();

      keycodes.push(keycode);
    }
  }

  return keycodes;
}
