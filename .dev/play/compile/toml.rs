//# toml = "0.8"
//# serde-toml-merge = "0.3.8"

use crate::error;
use serde_toml_merge::merge as toml_merge;
use std::fs;
use std::process::exit;

pub fn read(path: &str, required: bool) -> toml::Table {
  let content = match fs::read_to_string(&path) {
    Ok(content) => content,
    Err(e) => {
      if !required {
        return toml::Table::new();
      }
      error!("File does not exist!: {}", path);
      error!("{}", e);
      exit(1);
    }
  };
  content.parse::<toml::Table>().unwrap()
}

pub fn read_as_value(path: &str) -> toml::Value {
  let content = match fs::read_to_string(&path) {
    Ok(content) => content,
    Err(e) => {
      error!("File does not exist!: {}", path);
      error!("{}", e);
      exit(1);
    }
  };
  content.parse::<toml::Value>().unwrap()
}

pub fn string(table: &toml::Table, key: &str, required: bool) -> String {
  let mut keys = key.split('/');
  let mut current = table;

  while let Some(part) = keys.next() {
    current = match current.get(part) {
      Some(toml::Value::Table(t)) => t,
      Some(toml::Value::String(s)) => {
        return s.to_string();
      }
      _ => {
        error!("Missing '{}'", key);
        exit(1);
      }
    };
  }
  if !required {
    return "".to_string();
  }
  error!("Missing '{}'", key);
  exit(1);
}

pub fn required_string_list(table: &toml::Table, key: &str) -> Vec<String> {
  let mut keys = key.split('/');
  let mut current = table;

  while let Some(part) = keys.next() {
    current = match current.get(part) {
      Some(toml::Value::Table(t)) => t,
      Some(toml::Value::Array(a)) => {
        return a
          .iter()
          .map(|v| match v {
            toml::Value::String(s) => s.to_string(),
            _ => {
              error!("Expected a string in '{}'", key);
              exit(1);
            }
          })
          .collect();
      }
      _ => {
        error!("Missing '{}'", key);
        exit(1);
      }
    };
  }

  error!("Missing '{}'", key);
  exit(1);
}

pub fn merge(source: &str, target: &str) {
  let source_toml = read_as_value(source);
  let target_toml = read_as_value(target);
  let merged_toml = toml_merge(target_toml, source_toml).unwrap();

  fs::write(target, toml::to_string(&merged_toml).expect("Failed to serialize TOML"))
    .expect("Failed to write target file");
}
