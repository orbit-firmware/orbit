use crate::toml;
use crate::util;
use proc_macro2::Ident;
use quote::quote;

const MULTIPLEXER_SEL_DEVIDER: usize = 4;

#[allow(unused_variables)]
pub fn generate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let root = util::get_root();
  let path = format!("{}/build/keyboard.toml", root);
  let config = toml::read(&path, true);

  let product_id: u16 = toml::get(&config, "keyboard/product_id", true);
  let vendor_id: u16 = toml::get(&config, "keyboard/vendor_id", true);
  let name: String = toml::get(&config, "keyboard/name", true);
  let manufacturer: String = toml::get(&config, "keyboard/manufacturer", true);
  let chip: String = toml::get(&config, "keyboard/chip", true);

  let debounce_time: u16 = toml::get(&config, "settings/debounce_time", true);
  let tapping_term: u16 = toml::get(&config, "settings/tapping_term", true);

  let use_matrix: bool = toml::contains(&config, "matrix");
  let use_multiplexers: bool = toml::contains(&config, "multiplexers");

  {
    if !use_matrix && !use_multiplexers {
      println!("Missing matrix or multiplexers configuration!");
      std::process::exit(1);
    }

    if use_matrix && use_multiplexers {
      println!("Choose either multiplexers or matrix!");
      std::process::exit(1);
    }
  }

  let mut layout = vec![];

  // get the behavior count from the current configuration
  let mut behavior_count: usize = 0;
  let cargo_toml = toml::read("Cargo.toml", true);
  for feature in cargo_toml["features"]["default"].as_array().unwrap() {
    if feature.as_str().unwrap().starts_with("behavior_") {
      behavior_count += 1;
    }
  }

  let mut key_count: usize = 0;
  let mut matrix = quote! {
    pub const MATRIX_ROW_COUNT: usize = 0;
    pub const MATRIX_COL_COUNT: usize = 0;
    pub const MATRIX_ROW_PINS: [&str; 0] = [];
    pub const MATRIX_COL_PINS: [&str; 0] = [];
  };
  if use_matrix {
    let layout_list: Vec<(usize, usize)> = toml::get(&config, "matrix/layout", true);
    let row_pins: Vec<String> = toml::get(&config, "matrix/row_pins", true);
    let col_pins: Vec<String> = toml::get(&config, "matrix/col_pins", true);
    let row_count = row_pins.len() as usize;
    let col_count = col_pins.len() as usize;

    matrix = quote! {
      pub const MATRIX_ROW_COUNT: usize = #row_count;
      pub const MATRIX_COL_COUNT: usize = #col_count;
      pub const MATRIX_ROW_PINS: [&str; #row_count] = [#(#row_pins),*];
      pub const MATRIX_COL_PINS: [&str; #col_count] = [#(#col_pins),*];
    };

    key_count = layout_list.len();
    for key in layout_list {
      let mut row_ident = Ident::new("None", proc_macro2::Span::call_site());
      if row_pins.len() > 0 {
        let row: String = row_pins[key.0].clone();
        row_ident = Ident::new(&row, proc_macro2::Span::call_site());
      }

      let mut col_ident = Ident::new("None", proc_macro2::Span::call_site());
      if col_pins.len() > 0 {
        let col: String = col_pins[key.0].clone();
        col_ident = Ident::new(&col, proc_macro2::Span::call_site());
      }

      layout.push(quote! {
          [
            Peripheral::#row_ident,
            Peripheral::#col_ident
          ]
      });
    }
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
    let layout_list: Vec<(usize, usize)> = toml::get(&config, "multiplexers/layout", true);
    let count: usize = toml::get(&config, "multiplexers/count", true);
    let channels: usize = toml::get(&config, "multiplexers/channels", true);
    let sel_pins: Vec<String> = toml::get(&config, "multiplexers/sel_pins", true);
    let com_pins: Vec<String> = toml::get(&config, "multiplexers/com_pins", true);
    let sel_count = (channels / MULTIPLEXER_SEL_DEVIDER) as usize;

    multiplexers = quote! {
      pub const MULTIPLEXER_COUNT: usize = #count;
      pub const MULTIPLEXER_CHANNELS: usize = #channels;
      pub const MULTIPLEXER_SEL_COUNT: usize = #sel_count;
      pub const MULTIPLEXER_SEL_PINS: [&str; #sel_count] = [#(#sel_pins),*];
      pub const MULTIPLEXER_COM_COUNT: usize = #count;
      pub const MULTIPLEXER_COM_PINS: [&str; #count] = [#(#com_pins),*];
    };

    key_count = layout_list.len();
    for key in layout_list {
      let mut sel_ident = Ident::new("None", proc_macro2::Span::call_site());
      if sel_pins.len() > 0 {
        let sel: String = sel_pins[key.0].clone();
        sel_ident = Ident::new(&sel, proc_macro2::Span::call_site());
      }

      let mut com_ident = Ident::new("None", proc_macro2::Span::call_site());
      if com_pins.len() > 0 {
        let com: String = com_pins[key.0].clone();
        com_ident = Ident::new(&com, proc_macro2::Span::call_site());
      }

      layout.push(quote! {
          [
            Peripheral::#sel_ident,
            Peripheral::#com_ident
          ]
      });
    }
  }

  quote! {
    use crate::orbit::features::*;
    use crate::orbit::peripherals::*;

    pub const PRODUCT_ID: u16 = #product_id;
    pub const VENDOR_ID: u16 = #vendor_id;
    pub const NAME: &str = #name;
    pub const MANUFACTURER: &str = #manufacturer;
    pub const CHIP: &str = #chip;
    pub const KEY_COUNT: usize = #key_count;

    // settings
    pub const DEBOUNCE_TIME: u16 = #debounce_time;
    pub const TAPPING_TERM: u16 = #tapping_term;
    pub const BEHAVIOR_COUNT: usize = #behavior_count;

    // layout
    pub const USE_MATRIX: bool = #use_matrix;
    pub const USE_MULTIPLEXERS: bool = #use_multiplexers;

    pub const LAYOUT: [[Peripheral; 2]; #key_count] = [#(#layout),*];

    #matrix
    #multiplexers
  }
  .into()
}
