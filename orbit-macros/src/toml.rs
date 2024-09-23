use serde_toml_merge::merge as toml_merge;
use std::fs;
use std::process::exit;
pub use toml::{Table, Value};

pub fn read(path: &str, required: bool) -> Table {
  let content = match fs::read_to_string(&path) {
    Ok(content) => content,
    Err(e) => {
      if !required {
        return Table::new();
      }
      println!("File does not exist!: {}", path);
      println!("{}", e);
      exit(1);
    }
  };
  content.parse::<Table>().unwrap()
}

pub fn read_as_value(path: &str) -> Value {
  let content = match fs::read_to_string(&path) {
    Ok(content) => content,
    Err(e) => {
      println!("File does not exist!: {}", path);
      println!("{}", e);
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

impl FromTomlValue for u64 {
  fn from_value(value: &Value, required: bool) -> Option<Self> {
    if let Value::Integer(i) = value {
      Some(*i as u64)
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
                println!("Expected integer in the first position");
                exit(1);
              }) as usize;

              let second = a[1].as_integer().unwrap_or_else(|| {
                println!("Expected integer in the second position");
                exit(1);
              }) as usize;

              (first, second)
            }
            _ => {
              println!("Expected array of length 2");
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
              if !required {}
              println!("Expected Vec<(String, bool)> value for key: {}", key);
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
              println!("Expected a string");
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
          println!("Expected value at '{}'", key);
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

  println!("Missing '{}'", key);
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

pub fn set_package_name(filepath: &str, name: &str) {
  let mut content = read_as_value(filepath);

  if let toml::Value::Table(ref mut root_table) = content {
    let package_table = root_table
      .entry("package")
      .or_insert_with(|| toml::Value::Table(toml::map::Map::new()));

    if let toml::Value::Table(ref mut package_table) = package_table {
      package_table.insert("name".to_string(), toml::Value::String(name.to_string()));
    }
  }

  write(filepath, &content);
}
