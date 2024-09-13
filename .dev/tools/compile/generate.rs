mod config;
mod keycodes;
mod modifiers;
mod pinout;

pub fn run(root: &str, remaps: &str) {
  config::generate();
  keycodes::generate(root, remaps);
  pinout::generate();
}
