use toml::Table;

use crate::config::parser;

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

#[allow(unused)]
#[derive(Debug)]
pub struct Keyboard {
  pub product_id: u16,
  pub vendor_id: u16,
  pub name: String,
  pub manufacturer: String,
  pub chip: String,
  pub key_count: usize,
  pub debounce_ms: u32,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Matrix {
  pub row_count: usize,
  pub col_count: usize,
  pub row_pins: Vec<String>,
  pub col_pins: Vec<String>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Multiplexers {
  pub count: usize,
  pub channels: usize,
  pub sel: Vec<String>,
  pub com: Vec<String>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Config {
  pub keyboard: Keyboard,
  pub use_matrix: bool,
  pub matrix: Option<Matrix>,
  pub use_multiplexers: bool,
  pub multiplexers: Option<Multiplexers>,
  pub layout: Vec<[usize; 2]>,
}

impl Config {
  pub fn new() -> Self {
    Config {
      keyboard: Keyboard {
        product_id: 0,
        vendor_id: 0,
        name: "".to_string(),
        manufacturer: "".to_string(),
        chip: "".to_string(),
        key_count: 0,
        debounce_ms: 10,
      },
      use_matrix: false,
      matrix: None,
      use_multiplexers: false,
      multiplexers: None,
      layout: Vec::new(),
    }
  }

  pub fn from_toml(toml: Table) -> Self {
    let mut cfg = Config::new();
    if let Some(keyboard) = toml.get("keyboard").and_then(|v| v.as_table()) {
      let kb = Keyboard {
        product_id: parser::required_u16(keyboard, "product_id"),
        vendor_id: parser::required_u16(keyboard, "vendor_id"),
        name: parser::required_string(keyboard, "name"),
        manufacturer: parser::required_string(keyboard, "manufacturer"),
        chip: parser::required_string(keyboard, "chip"),
        key_count: parser::required_usize(keyboard, "key_count"),
        debounce_ms: parser::optional_u32(keyboard, "debounce_ms", 10),
      };

      cfg.keyboard = kb;
    } else {
      let msg = "Missing keyboard configuration!";
      println!("{}{}{}", RED, msg, RESET);
      std::process::exit(1);
    }

    if let Some(multiplexers) = toml.get("multiplexers").and_then(|v| v.as_table()) {
      cfg.use_multiplexers = true;

      let mp = Multiplexers {
        count: parser::required_usize(multiplexers, "count"),
        channels: parser::required_usize(multiplexers, "channels"),
        sel: multiplexers
          .get("sel")
          .and_then(|v| v.as_array())
          .map(|a| a.iter().map(|v| v.as_str().unwrap().to_string()).collect())
          .unwrap_or_else(|| {
            let msg = "Missing 'sel'!";
            println!("{}{}{}", RED, msg, RESET);
            std::process::exit(1);
          }),
        com: multiplexers
          .get("com")
          .and_then(|v| v.as_array())
          .map(|a| a.iter().map(|v| v.as_str().unwrap().to_string()).collect())
          .unwrap_or_else(|| {
            let msg = "Missing 'com'!";
            println!("{}{}{}", RED, msg, RESET);
            std::process::exit(1);
          }),
      };

      cfg.multiplexers = Some(mp);
    }

    if let Some(matrix) = toml.get("matrix").and_then(|v| v.as_table()) {
      cfg.use_matrix = true;

      let mat = Matrix {
        row_count: parser::required_usize(matrix, "row_count"),
        col_count: parser::required_usize(matrix, "col_count"),
        row_pins: matrix
          .get("row_pins")
          .and_then(|v| v.as_array())
          .map(|a| a.iter().map(|v| v.as_str().unwrap().to_string()).collect())
          .unwrap_or_else(|| {
            let msg = "Missing 'row_pins'!";
            println!("{}{}{}", RED, msg, RESET);
            std::process::exit(1);
          }),
        col_pins: matrix
          .get("col_pins")
          .and_then(|v| v.as_array())
          .map(|a| a.iter().map(|v| v.as_str().unwrap().to_string()).collect())
          .unwrap_or_else(|| {
            let msg = "Missing 'col_pins'!";
            println!("{}{}{}", RED, msg, RESET);
            std::process::exit(1);
          }),
      };
      cfg.matrix = Some(mat);
    }

    if let Some(layout) = toml.get("layout").and_then(|v| v.as_table()) {
      if let Some(keys) = layout.get("keys").and_then(|v| v.as_array()) {
        for key in keys {
          let row = key[0].as_integer().unwrap() as usize;
          let col = key[1].as_integer().unwrap() as usize;
          cfg.layout.push([row, col]);
        }
      }
    }

    if cfg.layout.len() != cfg.keyboard.key_count {
      let msg = "Layout does not match key count!";
      println!("{}{}{}", RED, msg, RESET);
      std::process::exit(1);
    }

    if !cfg.use_matrix && !cfg.use_multiplexers {
      let msg = "Missing matrix or multiplexers configuration!";
      println!("{}{}{}", RED, msg, RESET);
      std::process::exit(1);
    }

    if cfg.use_matrix && cfg.use_multiplexers {
      let msg = "Choose either multiplexers or matrix!";
      println!("{}{}{}", RED, msg, RESET);
      std::process::exit(1);
    }

    cfg
  }
}
