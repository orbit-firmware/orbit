//# toml = "0.8"
//# serde-toml-merge = "0.3.8"

use crate::error;
use serde_toml_merge::merge as toml_merge;
use std::fs;
use std::process::exit;
pub use toml::{map::Map, Table, Value};

pub fn read(path: &str, required: bool) -> Table {
  let content = match fs::read_to_string(&path) {
    Ok(content) => content,
    Err(e) => {
      if !required {
        return Table::new();
      }
      error!("File does not exist!: {}", path);
      error!("{}", e);
      exit(1);
    }
  };
  content.parse::<Table>().unwrap()
}

pub fn read_as_value(path: &str) -> Value {
  let content = match fs::read_to_string(&path) {
    Ok(content) => content,
    Err(e) => {
      error!("File does not exist!: {}", path);
      error!("{}", e);
      exit(1);
    }
  };
  content.parse::<Value>().unwrap()
}

pub fn merge(source: &str, target: &str) {
  let source_toml = read_as_value(source);
  let target_toml = read_as_value(target);
  let merged_toml = toml_merge(target_toml, source_toml).unwrap();

  fs::write(target, toml::to_string(&merged_toml).expect("Failed to serialize TOML"))
    .expect("Failed to write target file");
}

pub fn write(path: &str, table: &Value) {
  fs::write(path, toml::to_string(&table).expect("Failed to serialize TOML")).expect("Failed to write target file");
}

pub trait FromTomlValue: Sized {
  fn from_value(value: &Value, required: bool) -> Option<Self>;
}

impl FromTomlValue for String {
  fn from_value(value: &Value, required: bool) -> Option<Self> {
    if let Value::String(s) = value {
      Some(s.clone())
    } else if !required {
      Some("".to_string())
    } else {
      None
    }
  }
}

impl FromTomlValue for bool {
  fn from_value(value: &Value, required: bool) -> Option<Self> {
    if let Value::Boolean(i) = value {
      Some(*i as bool)
    } else if !required {
      Some(false)
    } else {
      None
    }
  }
}

impl FromTomlValue for u16 {
  fn from_value(value: &Value, required: bool) -> Option<Self> {
    if let Value::Integer(i) = value {
      Some(*i as u16)
    } else if !required {
      Some(0)
    } else {
      None
    }
  }
}

impl FromTomlValue for u32 {
  fn from_value(value: &Value, required: bool) -> Option<Self> {
    if let Value::Integer(i) = value {
      Some(*i as u32)
    } else if !required {
      Some(0)
    } else {
      None
    }
  }
}

impl FromTomlValue for usize {
  fn from_value(value: &Value, required: bool) -> Option<Self> {
    if let Value::Integer(i) = value {
      Some(*i as usize)
    } else if !required {
      Some(0)
    } else {
      None
    }
  }
}

impl FromTomlValue for Vec<(usize, usize)> {
  fn from_value(value: &Value, required: bool) -> Option<Self> {
    if let Value::Array(i) = value {
      Some(
        i.iter()
          .map(|v| match v {
            Value::Array(a) if a.len() == 2 => {
              let first = a[0].as_integer().unwrap_or_else(|| {
                error!("Expected integer in the first position");
                exit(1);
              }) as usize;

              let second = a[1].as_integer().unwrap_or_else(|| {
                error!("Expected integer in the second position");
                exit(1);
              }) as usize;

              (first, second)
            }
            _ => {
              error!("Expected array of length 2");
              exit(1);
            }
          })
          .collect(),
      )
    } else if !required {
      Some(vec![])
    } else {
      None
    }
  }
}

impl FromTomlValue for Vec<(String, bool)> {
  fn from_value(value: &Value, required: bool) -> Option<Self> {
    if let Value::Table(table) = value {
      Some(
        table
          .iter()
          .map(|(key, value)| match value {
            Value::Boolean(b) => (key.clone(), *b),
            _ => {
              error!("Expected boolean value for key: {}", key);
              exit(1);
            }
          })
          .collect(),
      )
    } else if !required {
      Some(vec![])
    } else {
      None
    }
  }
}

impl FromTomlValue for Vec<String> {
  fn from_value(value: &Value, required: bool) -> Option<Self> {
    if let Value::Array(i) = value {
      Some(
        i.iter()
          .map(|v| match v {
            Value::String(s) => s.clone(),
            _ => {
              error!("Expected a string");
              exit(1);
            }
          })
          .collect(),
      )
    } else if !required {
      Some(vec![])
    } else {
      None
    }
  }
}

pub fn get<T: FromTomlValue>(table: &Table, key: &str, required: bool) -> T {
  let mut keys = key.split('/');
  let mut current = table;

  while let Some(part) = keys.next() {
    current = match current.get(part) {
      Some(Value::Table(t)) => t,
      Some(value) => {
        if let Some(v) = T::from_value(value, required) {
          return v;
        } else {
          error!("Expected value at '{}'", key);
          exit(1);
        }
      }
      _ => {
        break;
      }
    };
  }

  if let Some(value) = T::from_value(&Value::Table(current.clone()), required) {
    return value;
  }

  error!("Missing '{}'", key);
  exit(1);
}

pub fn contains(table: &Table, key: &str) -> bool {
  let mut keys = key.split('/');
  let mut current = table;

  while let Some(part) = keys.next() {
    current = match current.get(part) {
      Some(Value::Table(t)) => t,
      Some(_) => return keys.next().is_none(),
      _ => return false,
    };
  }

  true
}
