use proc_macro::TokenStream;
use quote::quote;
use toml::value::Table;
use toml::Value;

mod config;
mod parser;
mod read;

use config::Config;

const MULTIPLEXER_SEL_DEVIDER: usize = 4;

pub fn merge(a: Table, b: Table) -> Table {
  let mut merged = a.clone();

  for (key, value) in b {
    if let Value::Table(b_table) = value {
      if let Some(Value::Table(a_table)) = a.get(&key) {
        merged.insert(key, Value::Table(merge(a_table.clone(), b_table.clone())));
      } else {
        merged.insert(key, Value::Table(b_table));
      }
    } else {
      merged.insert(key, value);
    }
  }

  merged
}

pub fn generate(_input: TokenStream) -> TokenStream {
  let keyboard_config = read::file("tmp/config.toml", false)
    .parse::<Table>()
    .unwrap();
  let user_config = read::file("user/config.toml", true)
    .parse::<Table>()
    .unwrap();

  let merged = merge(keyboard_config, user_config);
  let config = Config::from_toml(merged);

  let name = config.keyboard.name;
  let manufacturer = config.keyboard.manufacturer;
  let chip = config.keyboard.chip;
  let key_count = config.keyboard.key_count as usize;
  let debounce_ms = config.keyboard.debounce_ms;
  let use_matrix = config.use_matrix;
  let use_multiplexers = config.use_multiplexers;
  let behavior_count = config.behaviors.count as usize;

  let mut layout = vec![];
  for key in config.layout {
    let row: usize = key[0] as usize;
    let col: usize = key[1] as usize;
    layout.push(quote! {
        [#row, #col]
    });
  }

  let mut behaviors = vec![];
  for behavior in config.behaviors.list {
    behaviors.push(quote! {
      Behaviors::#behavior
    });
  }

  let mut matrix = quote! {
    pub const MATRIX_ROW_COUNT: usize = 0;
      pub const MATRIX_COL_COUNT: usize = 0;
      pub const MATRIX_ROW_PINS: [&str; 0] = [];
      pub const MATRIX_COL_PINS: [&str; 0] = [];
  };
  if use_matrix {
    let m = config.matrix.unwrap();
    let row_count = m.row_count as usize;
    let col_count = m.col_count as usize;
    let row_pins = &m.row_pins;
    let col_pins = &m.col_pins;

    matrix = quote! {
      pub const MATRIX_ROW_COUNT: usize = #row_count;
      pub const MATRIX_COL_COUNT: usize = #col_count;
      pub const MATRIX_ROW_PINS: [&str; #row_count] = [#(#row_pins),*];
      pub const MATRIX_COL_PINS: [&str; #col_count] = [#(#col_pins),*];
    };
  }

  let mut multiplexers = quote! {
    pub const MULTIPLEXER_COUNT: usize = 0;
      pub const MULTIPLEXER_CHANNELS: usize = 0;
      pub const MULTIPLEXER_SEL_COUNT: usize = 0;
      pub const MULTIPLEXER_SEL_PINS: [&str; 0] = [];
      pub const MULTIPLEXER_COM_COUNT: usize = 0;
      pub const MULTIPLEXER_COM_PINS: [&str; 0] = [];
  };
  if use_multiplexers {
    let m = config.multiplexers.unwrap();
    let count = m.count as usize;
    let channels = m.channels as usize;
    let sel = &m.sel;
    let com = &m.com;
    let sel_count = (channels / MULTIPLEXER_SEL_DEVIDER) as usize;

    multiplexers = quote! {
      pub const MULTIPLEXER_COUNT: usize = #count;
      pub const MULTIPLEXER_CHANNELS: usize = #channels;
      pub const MULTIPLEXER_SEL_COUNT: usize = #sel_count;
      pub const MULTIPLEXER_SEL_PINS: [&str; #sel_count] = [#(#sel),*];
      pub const MULTIPLEXER_COM_COUNT: usize = #count;
      pub const MULTIPLEXER_COM_PINS: [&str; #count] = [#(#com),*];
    };
  }

  let generated = quote! {
    use crate::behaviors::Behaviors;

    pub const NAME: &str = #name;
    pub const MANUFACTURER: &str = #manufacturer;
    pub const CHIP: &str = #chip;
    pub const KEY_COUNT: usize = #key_count as usize;
    pub const BEHAVIOR_COUNT: usize = #behavior_count as usize;
    pub const ENABLED_BEHAVIORS: [Behaviors; #behavior_count] = [#(#behaviors),*];
    pub const LAYOUT: [[usize; 2]; #key_count] = [#(#layout),*];
    pub const DEBOUNCE_MS: u32 = #debounce_ms;
    pub const USE_MATRIX: bool = #use_matrix;
    #matrix
    pub const USE_MULTIPLEXERS: bool = #use_multiplexers;
    #multiplexers
  };

  TokenStream::from(generated)
}
