// INFO: files for cargo-play are executed in alphabetical order,
// that's why we need underscores in front of it.

mod compile;
mod generate;
mod toml;
mod util;

use std::process::exit;

// just to make sure we are in the right dir
fn validate_root_dir() {
  let e1 = util::directory_exists("chips");
  let e2 = util::directory_exists("keyboards");
  let e3 = util::directory_exists("orbit");
  let e4 = util::directory_exists(".dev");

  if !e1 || !e2 || !e3 || !e4 {
    error!("Invalid root directory");
    exit(1);
  }
}

fn main() {
  let root = util::get_root();
  util::cd(&root);
  validate_root_dir();

  let keyboard = util::get_arg(1);
  let keyboard_file = format!("keyboards/{}.toml", keyboard);
  if !util::file_exists(&keyboard_file) {
    error!("Keyboard does not exist: {}", keyboard_file);
    exit(1);
  }

  let keyboard_toml = toml::read(&keyboard_file, true);
  let keycodes: String = toml::get(&keyboard_toml, "keyboard/keycodes", false);
  let chip: String = toml::get(&keyboard_toml, "keyboard/chip", true);
  let chip_dir = format!("chips/{}", chip);

  if !util::directory_exists(&chip_dir) {
    error!("Chip does not exist: {}", chip);
    exit(1);
  }

  util::mkdir(".bin");
  util::write(".bin/.last_kb", &keyboard);

  // keyboard config
  util::copy(&keyboard_file, ".bin/keyboard.toml");
  if util::file_exists("user/keyboard.toml") {
    toml::merge("user/keyboard.toml", ".bin/keyboard.toml");
  }

  compile::prepare(&chip_dir, &chip, &keyboard.as_str());
  util::cd(".bin");

  let features: Vec<String> = generate::run(&root, &chip, &keycodes);
  compile::install();
  compile::compile(features, &chip);
}
