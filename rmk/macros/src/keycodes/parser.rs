// borrow modifiers from original source
// this is not the prefered way to use modules
// but it ensuures we always use the same masks
#[path = "../../../core/src/modifiers.rs"]
mod modifiers;

use crate::keycodes::types::KeyCode;

pub fn code(code_str: &str, list: &Vec<KeyCode>) -> u16 {
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

fn validate_codes(list: Vec<KeyCode>) {
  let mut used: Vec<(String, u16)> = vec![];
  for i in &list {
    let name = i.name.to_string();
    for u in &used {
      if u.1 == i.code {
        panic!("Duplicate code: {} and {}", i.name, u.0);
      }
    }
    used.push((name, i.code));
  }
}

pub fn merge(a: Vec<KeyCode>, b: Vec<KeyCode>) -> Vec<KeyCode> {
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
