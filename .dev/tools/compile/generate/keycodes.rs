//# quote = "1.0"
//# proc-macro2 = "1.0"
//# syn = "2.0"
const TARGET_FILE: &str = "src/orbit/keycodes.rs";

use crate::error;
use crate::generate::modifiers;
use crate::util;
use proc_macro2::Span;
use quote::quote;
use std::fs;
use std::process::exit;
use syn::Ident as SynIdent;

#[derive(Debug, Clone)]
pub struct KeyCode {
  pub name: SynIdent,
  pub code_str: String,
  pub code: u16,
  pub alias_list: Vec<String>,
}

impl KeyCode {
  pub fn new() -> KeyCode {
    KeyCode {
      name: SynIdent::new("Unknown", Span::call_site()),
      code_str: String::from("0"),
      code: 0,
      alias_list: vec![],
    }
  }
}

fn read(path: &str, root: &str, optional: bool) -> Vec<KeyCode> {
  let path = format!("{}/.dev/keycodes/{}.k", root, path);

  if optional && !util::file_exists(path.as_str()) {
    return vec![];
  }

  let content = match fs::read_to_string(&path) {
    Ok(content) => content,
    Err(e) => {
      error!("Keycodes file does not exist!: {}", path);
      error!("{}", e);
      exit(1);
    }
  };

  fn valid_line(line: &str) -> bool {
    if line.is_empty() {
      return false;
    }

    if line.starts_with("#") {
      return false;
    }

    true
  }

  let mut keycodes: Vec<KeyCode> = vec![];

  for line in content.lines() {
    if !valid_line(line) {
      continue;
    }
    let mut keycode = KeyCode::new();

    let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
    if parts.len() >= 2 {
      keycode.name = SynIdent::new(parts[0], Span::call_site());
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

fn code(code_str: &str, list: &Vec<KeyCode>) -> u16 {
  let msg = format!("Failed to parse code {:?}", code_str);

  let token_collector = code_str.trim_end_matches(")").split("(");
  let mut tokens = token_collector.collect::<Vec<&str>>();

  let code = tokens.pop().expect(msg.as_str());
  let mut parsed_code = 0;

  if !code.starts_with("0x") {
    for i in list {
      if i.name.to_string() == code {
        parsed_code = i.code;
        break;
      }
    }
  } else {
    let trimmed_code = code.trim_start_matches("0x");
    parsed_code = u16::from_str_radix(trimmed_code, 16).expect(msg.as_str());
  }

  while !tokens.is_empty() {
    let token = tokens.pop().expect(msg.as_str());
    match token {
      "lc" => parsed_code = modifiers::lc(parsed_code),
      "rc" => parsed_code = modifiers::rc(parsed_code),
      "r" => parsed_code = modifiers::r(parsed_code),
      "ls" => parsed_code = modifiers::ls(parsed_code),
      "rs" => parsed_code = modifiers::rs(parsed_code),
      "s" => parsed_code = modifiers::s(parsed_code),
      "la" => parsed_code = modifiers::la(parsed_code),
      "ra" => parsed_code = modifiers::ra(parsed_code),
      "a" => parsed_code = modifiers::a(parsed_code),
      "lg" => parsed_code = modifiers::lg(parsed_code),
      "rg" => parsed_code = modifiers::rg(parsed_code),
      "g" => parsed_code = modifiers::g(parsed_code),
      _ => println!("Unknown token: {}", token),
    }
  }

  parsed_code
}

fn merge(a: Vec<KeyCode>, b: Vec<KeyCode>) -> Vec<KeyCode> {
  fn validate_codes(list: Vec<KeyCode>) {
    let mut used: Vec<(String, u16)> = vec![];
    for i in &list {
      let name = i.name.to_string();
      for u in &used {
        if u.1 == i.code {
          error!("Duplicate code: {} and {}", i.name, u.0);
          exit(1);
        }
      }
      used.push((name, i.code));
    }
  }

  let mut merged: Vec<KeyCode> = vec![];

  for j in a.clone() {
    let mut v = j.clone();
    v.code = code(&v.code_str, &merged);
    merged.push(v);
  }

  for i in b {
    let mut found = false;
    let mut index = 0;
    for j in a.clone() {
      if i.name == j.name {
        let mut v = j.clone();
        v.code_str = i.code_str.clone();
        v.code = code(&v.code_str, &merged);
        merged[index] = v;
        found = true;
        break;
      }
      index += 1;
    }
    if !found {
      let mut v = i.clone();
      v.code = code(&v.code_str, &merged);
      merged.push(v);
    }
  }

  validate_codes(merged.clone());

  merged
}

#[allow(unused_variables)]
pub fn generate(feature_list: &mut Vec<String>, root: &str, remaps: &str) {
  let mut keycodes = read("us", root, false);
  let remapcodes = read(remaps, root, true);
  keycodes = merge(keycodes, remapcodes);

  let mut entries = vec![];
  let mut match_arms = vec![];
  let mut alias_entries = vec![];

  for keycode in keycodes {
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
      #![allow(dead_code)]

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

  util::write(TARGET_FILE, util::quote_to_string(generated).as_str());
}
