use crate::util;

mod config;
mod features;
mod keycodes;
mod modifiers;
mod module;
mod peripherals;

pub fn run(root: &str, chip: &str, remaps: &str) {
  let mut feature_list: Vec<String> = vec![];
  util::copy("../.dev/tools/compile/generate/modifiers.rs", "src/orbit/modifiers.rs");

  config::generate(&mut feature_list);
  keycodes::generate(&mut feature_list, root, remaps);
  peripherals::generate(&mut feature_list, &chip);
  module::generate(&mut feature_list);
  features::generate(&mut feature_list);
}
