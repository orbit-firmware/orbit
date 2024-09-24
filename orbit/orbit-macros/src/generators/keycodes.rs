use crate::modifiers;
use crate::toml;
use crate::util;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::fs;

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

fn read(name: &str, required: bool) -> Vec<KeyCode> {
  if !required && name.is_empty() {
    return vec![];
  }
  let root = util::get_root();
  let path = format!("{}/orbit/keycodes/{}.kcs", root, name);

  if !required && !util::file_exists(path.as_str()) {
    return vec![];
  }

  let content = match fs::read_to_string(&path) {
    Ok(content) => content,
    Err(e) => {
      println!("Keycodes file does not exist!: {}", path);
      println!("{}", e);
      std::process::exit(1);
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

fn parse_code(code_str: &str, list: &Vec<KeyCode>) -> u16 {
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

fn merge(target: Vec<KeyCode>, source: Vec<KeyCode>) -> Vec<KeyCode> {
  let mut _target: Vec<KeyCode> = vec![];
  let mut _source: Vec<KeyCode> = vec![];

  for item in target.clone() {
    let mut value = item.clone();
    value.code = parse_code(&value.code_str, &_target);
    _target.push(value);
  }

  for item in source.clone() {
    let mut value = item.clone();
    value.code = parse_code(&value.code_str, &_target);
    _source.push(value);
  }

  for source_item in _source.clone() {
    let mut found = false;
    let mut index = 0;
    for target_item in _target.clone() {
      if source_item.name == target_item.name {
        _target[index] = source_item.clone();
        found = true;
        break;
      }
      index += 1;
    }
    if !found {
      _target.push(source_item.clone());
    }
  }

  let mut used: Vec<KeyCode> = vec![];
  for i in _target.clone() {
    if let Some(pos) = used.iter().position(|u| u.code == i.code) {
      used.remove(pos);
    }
    used.push(i.clone());
  }

  used
}

pub fn get_config_keycode_remaps() -> String {
  let root = util::get_root();
  let path = format!("{}/build/keyboard.toml", root);
  let config = toml::read(&path, true);
  let remaps: String = toml::get(&config, "settings/keycodes", false);
  remaps
}

#[allow(unused_variables)]
pub fn generate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let mut keycodes = read("us", true);
  let remaps = get_config_keycode_remaps();
  let remapcodes = read(&remaps, false);
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

  quote! {
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
  }
  .into()
}
