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
  let cfg = Config::from_toml(merged);

  let name = cfg.keyboard.name;
  let manufacturer = cfg.keyboard.manufacturer;
  let chip = cfg.keyboard.chip;
  let key_count = cfg.keyboard.key_count;
  let debounce_ms = cfg.keyboard.debounce_ms;
  let use_matrix = cfg.use_matrix;
  let use_multiplexers = cfg.use_multiplexers;

  let mut layout = vec![];

  for key in cfg.layout {
    let row: usize = key[0] as usize;
    let col: usize = key[1] as usize;
    layout.push(quote! {
        [#row, #col]
    });
  }

  let mut matrix = quote! {};

  let mut multiplexers = quote! {};
  if use_multiplexers {
    let m = cfg.multiplexers.unwrap();
    let count = m.count;
    let channels = m.channels;
    let sel = &m.sel;
    let com = &m.com;
    let sel_count = channels / MULTIPLEXER_SEL_DEVIDER;

    multiplexers = quote! {
      pub const MULTIPLEXER_COUNT: usize = #count as usize;
      pub const MULTIPLEXER_CHANNELS: usize = #channels as usize;
      pub const MULTIPLEXER_SEL_PINS: [&str; #sel_count] = [#(#sel),*];
      pub const MULTIPLEXER_COM_PINS: [&str; #count] = [#(#com),*];
    };
  }

  let generated = quote! {
    pub const NAME: &str = #name;
    pub const MANUFACTURER: &str = #manufacturer;
    pub const CHIP: &str = #chip;
    pub const KEY_COUNT: usize = #key_count as usize;
    pub const LAYOUT: [[usize; 2]; #key_count] = [#(#layout),*];
    pub const DEBOUNCE_MS: u32 = #debounce_ms;
    pub const USE_MATRIX: bool = #use_matrix;
    #matrix
    pub const USE_MULTIPLEXERS: bool = #use_multiplexers;
    #multiplexers
  };

  TokenStream::from(generated)
}
