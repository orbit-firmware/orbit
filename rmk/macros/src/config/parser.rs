#![allow(dead_code)]
#![allow(unused)]

use toml::Table;
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

pub fn required_string(table: &Table, key: &str) -> String {
  table
    .get(key)
    .and_then(|v| v.as_str())
    .map(|s| s.to_string())
    .unwrap_or_else(|| {
      let msg = format!("Missing '{}'", key);
      println!("{}{}{}", RED, msg, RESET);
      std::process::exit(1);
    })
}

pub fn optional_string(table: &Table, key: &str, default: &str) -> String {
  table
    .get(key)
    .and_then(|v| v.as_str())
    .map(|s| s.to_string())
    .unwrap_or_else(|| default.to_string())
}

pub fn required_u16(table: &Table, key: &str) -> u16 {
  table
    .get(key)
    .and_then(|v| v.as_integer())
    .map(|i| i as u16)
    .unwrap_or_else(|| {
      let msg = format!("Missing '{}'", key);
      println!("{}{}{}", RED, msg, RESET);
      std::process::exit(1);
    })
}

pub fn optional_u16(table: &Table, key: &str, default: u16) -> u16 {
  table
    .get(key)
    .and_then(|v| v.as_integer())
    .map(|i| i as u16)
    .unwrap_or_else(|| default)
}

pub fn required_u32(table: &Table, key: &str) -> u32 {
  table
    .get(key)
    .and_then(|v| v.as_integer())
    .map(|i| i as u32)
    .unwrap_or_else(|| {
      let msg = format!("Missing '{}'", key);
      println!("{}{}{}", RED, msg, RESET);
      std::process::exit(1);
    })
}

pub fn optional_u32(table: &Table, key: &str, default: u32) -> u32 {
  table
    .get(key)
    .and_then(|v| v.as_integer())
    .map(|i| i as u32)
    .unwrap_or_else(|| default)
}

pub fn required_usize(table: &Table, key: &str) -> usize {
  table
    .get(key)
    .and_then(|v| v.as_integer())
    .map(|i| i as usize)
    .unwrap_or_else(|| {
      let msg = format!("Missing '{}'!", key);
      println!("{}{}{}", RED, msg, RESET);
      std::process::exit(1);
    })
}

pub fn optional_usize(table: &Table, key: &str, default: usize) -> usize {
  table
    .get(key)
    .and_then(|v| v.as_integer())
    .map(|i| i as usize)
    .unwrap_or_else(|| default)
}

pub fn required_bool(table: &Table, key: &str) -> bool {
  table
    .get(key)
    .and_then(|v| v.as_bool())
    .map(|i| i as bool)
    .unwrap_or_else(|| {
      let msg = format!("Missing '{}'!", key);
      println!("{}{}{}", RED, msg, RESET);
      std::process::exit(1);
    })
}

pub fn optional_bool(table: &Table, key: &str, default: bool) -> bool {
  table
    .get(key)
    .and_then(|v| v.as_bool())
    .map(|i| i as bool)
    .unwrap_or_else(|| default)
}
