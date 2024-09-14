use crate::util;

mod config;
mod features;
mod keycodes;
mod modifiers;
mod module;
mod pinout;

pub fn run(root: &str, remaps: &str) -> Vec<String> {
  let mut feature_list: Vec<String> = vec![];
  util::copy("../.dev/tools/compile/generate/modifiers.rs", "src/orbit/modifiers.rs");

  config::generate(&mut feature_list);
  keycodes::generate(&mut feature_list, root, remaps);
  module::generate(&mut feature_list);
  features::generate(&mut feature_list);
  pinout::generate(&mut feature_list);

  feature_list
}
