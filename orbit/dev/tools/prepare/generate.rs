use crate::util;

mod config;
mod features;
mod keycodes;
mod modifiers;
mod modules;
mod peripherals;

pub fn run(root: &str, chip: &str, remaps: &str) {
  let mut feature_list: Vec<String> = vec![];
  util::copy(
    "../orbit/dev/tools/prepare/generate/modifiers.rs", "src/orbit/modifiers.rs",
  );

  config::generate(&mut feature_list);
  keycodes::generate(&mut feature_list, root, remaps);
  peripherals::generate(&mut feature_list, &chip);
  modules::generate(&mut feature_list);
  features::generate(&mut feature_list);
}