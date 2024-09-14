mod config;
mod features;
mod keycodes;
mod modifiers;
mod pinout;
mod rmk_module;

pub fn run(root: &str, remaps: &str) {
  config::generate();
  keycodes::generate(root, remaps);
  rmk_module::generate();
  features::generate();
  pinout::generate();
}
