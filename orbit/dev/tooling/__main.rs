// INFO: files for cargo-play are executed in alphabetical order,
// that's why we need underscores in front of it.

mod dependencies;
mod generate;
mod prepare;
mod toml;
mod util;

// just to make sure we are in the right dir
fn validate_root_dir() {
  let e1 = util::directory_exists("keyboards");
  let e2 = util::directory_exists("orbit/chips");
  let e3 = util::directory_exists("orbit/dev");
  let e4 = util::directory_exists("orbit/orbit");

  if !e1 || !e2 || !e3 || !e4 {
    error!("Invalid root directory");
    std::process::exit(1);
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
    std::process::exit(1);
  }

  let keyboard_toml = toml::read(&keyboard_file, true);
  let keycodes: String = toml::get(&keyboard_toml, "settings/keycodes", false);
  let chip: String = toml::get(&keyboard_toml, "keyboard/chip", true);
  let chip_dir = format!("orbit/chips/{}", chip);

  if !util::directory_exists(&chip_dir) {
    error!("Chip does not exist: {}", chip);
    std::process::exit(1);
  }

  util::mkdir("build");
  util::write("build/.last_kb", &keyboard);

  // keyboard config
  util::copy(&keyboard_file, "build/keyboard.toml");
  if util::file_exists("user/keyboard.toml") {
    toml::merge("user/keyboard.toml", "build/keyboard.toml");
  }

  prepare::run(&chip_dir, &keyboard.as_str());
  util::cd("build");
  generate::run(&root, &chip, &keycodes);
  dependencies::install();
  // compile::compile(&chip);
}
