//# syn = "2.0"
//# quote = "1.0"
const TARGET_FILE: &str = "src/orbit/config.rs";

use crate::error;
use crate::toml;
use crate::util;
use quote::quote;
use std::process::exit;
use syn::Ident as SynIdent;

const MULTIPLEXER_SEL_DEVIDER: usize = 4;

#[allow(unused_variables)]
pub fn generate(feature_list: &mut Vec<String>) {
  let config = toml::read("keyboard.toml", true);

  let product_id: u16 = toml::get(&config, "keyboard/product_id", true);
  let vendor_id: u16 = toml::get(&config, "keyboard/vendor_id", true);
  let name: String = toml::get(&config, "keyboard/name", true);
  let manufacturer: String = toml::get(&config, "keyboard/manufacturer", true);
  let chip: String = toml::get(&config, "keyboard/chip", true);

  let debounce_time: u16 = toml::get(&config, "settings/debounce_time", true);
  let tapping_term: u16 = toml::get(&config, "settings/tapping_term", true);

  let use_matrix: bool = toml::contains(&config, "matrix");
  let use_multiplexers: bool = toml::contains(&config, "multiplexers");
  let behaviors_list: Vec<(String, bool)> = toml::get(&config, "behaviors", false);
  let actions_list: Vec<(String, bool)> = toml::get(&config, "actions", false);

  {
    if !use_matrix && !use_multiplexers {
      error!("Missing matrix or multiplexers configuration!");
      exit(1);
    }

    if use_matrix && use_multiplexers {
      error!("Choose either multiplexers or matrix!");
      exit(1);
    }
  }

  let mut layout = vec![];
  let mut layout_list: Vec<(usize, usize)> = vec![];

  let mut behaviors = vec![];
  for b in behaviors_list {
    if b.0 == "press" || b.0 == "Press" {
      continue;
    }
    if b.1 {
      feature_list.push(format!("behavior_{}_enabled", b.0));

      let behavior = util::capitalize_first(&b.0);
      if b.1 {
        let ident = SynIdent::new(&behavior, proc_macro2::Span::call_site());
        behaviors.push(quote! {
            Behavior::#ident
        });
      }
    }
  }
  let behavior_count: usize = behaviors.len();

  let mut actions = vec![];
  for a in actions_list {
    if a.0 == "press" || a.0 == "Press" {
      continue;
    }
    if a.1 {
      feature_list.push(format!("action_{}_enabled", a.0));

      let action = util::capitalize_first(&a.0);
      if a.1 {
        let ident = SynIdent::new(&action, proc_macro2::Span::call_site());
        actions.push(quote! {
            Action::#ident
        });
      }
    }
  }
  let action_count: usize = actions.len();

  let mut matrix = quote! {
    pub const MATRIX_ROW_COUNT: usize = 0;
    pub const MATRIX_COL_COUNT: usize = 0;
    pub const MATRIX_ROW_PINS: [&str; 0] = [];
    pub const MATRIX_COL_PINS: [&str; 0] = [];
  };
  if use_matrix {
    layout_list = toml::get(&config, "matrix/layout", true);
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
    layout_list = toml::get(&config, "multiplexers/layout", true);
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
  }

  let key_count = layout_list.len();
  for key in layout_list {
    let row: usize = key.0;
    let col: usize = key.1;
    layout.push(quote! {
        [#row, #col]
    });
  }

  let generated = quote! {
    #![allow(dead_code)]

    use crate::orbit::behaviors::Behavior;
    use crate::orbit::actions::Action;

    pub const PRODUCT_ID: u16 = #product_id;
    pub const VENDOR_ID: u16 = #vendor_id;
    pub const NAME: &str = #name;
    pub const MANUFACTURER: &str = #manufacturer;
    pub const CHIP: &str = #chip;
    pub const KEY_COUNT: usize = #key_count;

    // settings
    pub const DEBOUNCE_TIME: u16 = #debounce_time;
    pub const TAPPING_TERM: u16 = #tapping_term;
    pub const BEHAVIORS: [Behavior; #behavior_count] = [#(#behaviors),*];
    pub const ACTIONS: [Action; #action_count] = [#(#actions),*];

    // layout
    pub const USE_MATRIX: bool = #use_matrix;
    pub const USE_MULTIPLEXERS: bool = #use_multiplexers;

    pub const LAYOUT: [[usize; 2]; #key_count] = [#(#layout),*];

    #matrix
    #multiplexers
  };

  util::write(TARGET_FILE, util::quote_to_string(generated).as_str());
}
