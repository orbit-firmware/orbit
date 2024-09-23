//# fs_extra = "1.3.0"
//# serde-toml-merge = "0.3.8"
//# toml = "0.8"

// IMPORTANT: this is not a normal build.rs file.
// IMPORTANT: it gets executed via cargo-play

use fs_extra::{copy_items, dir::CopyOptions};
use serde_toml_merge::merge as toml_merge;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;

macro_rules! error {
  ($($arg:tt)*) => {
      println!("\x1b[31m{}\x1b[0m", format_args!($($arg)*));
  };
}

macro_rules! ok {
  ($($arg:tt)*) => {
      println!("\x1b[32m{}\x1b[0m", format_args!($($arg)*));
  };
}

macro_rules! info {
  ($($arg:tt)*) => {
      println!("\x1b[34m{}\x1b[0m", format_args!($($arg)*));
  };
}

pub fn main() {
  let args = std::env::args().collect::<Vec<String>>();
  if args.len() < 2 {
    error!("Please provide a keyboard name!");
    std::process::exit(1);
  }
  let keyboard_name = args.get(1).unwrap();
  let mut input_features: Vec<String> = vec![];
  if args.len() == 3 {
    let features = args.get(2).unwrap();
    input_features = features.split(",").map(|s| s.to_string()).collect();
  }
  let (keyboard, keyboard_toml) = get_keyboard(&keyboard_name);
  let (chip_name, chip_dir, chip_toml) = get_chip(&keyboard);

  needs_clean(&keyboard_name, &chip_name);
  copy_folder("orbit", "build", vec!["target", "Cargo.lock", "build.rs"]);
  copy_folder(&chip_dir, "build", vec!["target", "Cargo.lock"]);
  merge_toml("orbit/Cargo.toml", &chip_toml, "build/Cargo.toml", true);
  merge_toml(
    &keyboard_toml, "user/keyboard.toml", "build/keyboard.toml", false,
  );
  configure(&keyboard, &chip_name, &keyboard_name, &input_features);
  prepare_orbit_module();
  save_last_build_cfg(&keyboard_name, &chip_name);
  ok!("Pre-Compile completed!");
}

fn configure(
  keyboard: &Value,
  chip_name: &str,
  keyboard_name: &str,
  input_features: &Vec<String>,
) {
  let chip_type: &str = get_chip_type(chip_name);

  let mut content = read_toml("build/Cargo.toml");

  content["package"]["name"] = Value::String(keyboard_name.to_string());
  content["dependencies"]["orbit-macros"]["features"] =
    Value::Array(vec![Value::String(chip_type.to_string())]);

  let enabled_features = get_features(keyboard, &mut content);
  let mut features = content["features"]["default"].as_array().unwrap().clone();
  for feature in enabled_features {
    features.push(Value::String(feature));
  }
  for feature in input_features {
    features.push(Value::String(feature.to_string()));
  }
  content["features"]["default"] = Value::Array(features.clone());
  write_toml("build/Cargo.toml", &content);
}

fn get_file_names(dir: &str) -> Vec<String> {
  let mut file_names: Vec<String> = vec![];
  fs::read_dir(dir).unwrap().for_each(|entry| {
    let entry = entry.unwrap();
    let filename = entry.file_name().into_string().unwrap();
    let path = Path::new(&filename);

    // Get the file name without the extension
    if let Some(stem) = path.file_stem() {
      file_names.push(stem.to_str().unwrap().to_string());
    }
  });

  file_names
}

fn get_features(keyboard: &Value, content: &mut Value) -> Vec<String> {
  let mut features: Vec<String> = vec![];

  let actions = get_file_names("orbit/src/orbit/features/actions");
  for action in keyboard["actions"].as_table().unwrap() {
    let name = action.0;
    let enabled = action.1.as_bool().unwrap();
    if !actions.contains(&name) {
      error!("Action not found: {}", name);
      std::process::exit(1);
    }
    features.push(format!("action_{}_enabled", name));
  }

  let behaviors = get_file_names("orbit/src/orbit/features/behaviors");
  for behavior in keyboard["behaviors"].as_table().unwrap() {
    let name = behavior.0;
    let enabled = behavior.1.as_bool().unwrap();
    if !behaviors.contains(&name) {
      error!("behavior not found: {}", name);
      std::process::exit(1);
    }
    features.push(format!("behavior_{}_enabled", name));
  }

  let flavors = get_file_names("orbit/src/orbit/features/flavors");
  for flavor in keyboard["flavors"].as_table().unwrap() {
    let name = flavor.0;
    let enabled = flavor.1.as_bool().unwrap();
    if !flavors.contains(&name) {
      error!("flavor not found: {}", name);
      std::process::exit(1);
    }
    features.push(format!("flavor_{}_enabled", name));
  }

  features
}

fn get_chip_type(chip_name: &str) -> &str {
  if chip_name.starts_with("stm") {
    return "chip_type_stm32";
  }
  if chip_name.starts_with("nrf") {
    return "chip_type_nrf";
  }
  if chip_name.starts_with("rp") {
    return "chip_type_rp";
  }
  if chip_name.starts_with("esp") {
    return "chip_type_esp";
  }
  if chip_name.starts_with("ch") {
    return "chip_type_chw";
  }
  if chip_name.starts_with("_emulator") {
    return "chip_type_emulator";
  }

  return "none";
}

fn read_toml(path: &str) -> Value {
  let content = match fs::read_to_string(&path) {
    Ok(content) => content,
    Err(e) => {
      error!("File does not exist!: {}", path);
      error!("{}", e);
      std::process::exit(1);
    }
  };
  content.parse::<Value>().unwrap()
}

fn write_toml(target: &str, merged_toml: &Value) {
  let serialized = match toml::to_string(merged_toml) {
    Ok(s) => s,
    Err(e) => {
      error!("Failed to serialize TOML: {}", e);
      std::process::exit(1);
    }
  };

  if let Err(e) = fs::write(target, serialized) {
    error!("Failed to write target file: {}", e);
    std::process::exit(1);
  } else {
    info!("Successfully wrote to {}", target);
  }
}

pub fn merge_toml(src1: &str, src2: &str, target: &str, src2_required: bool) {
  let src1_toml = read_toml(src1);

  if !src2_required && fs::metadata(src2).is_err() {
    write_toml(&target, &src1_toml);
    return;
  }

  let src2_toml = read_toml(src2);
  let merged_toml = toml_merge(src1_toml, src2_toml).unwrap();

  write_toml(&target, &merged_toml);
}

fn get_keyboard(kb: &str) -> (Value, String) {
  let keyboard_toml = format!("keyboards/{}.toml", kb);
  let path = Path::new(&keyboard_toml);
  if path.exists() {
    info!("Using Keyboard: {}", kb);
    let keyboard = read_toml(&keyboard_toml);
    (keyboard, keyboard_toml)
  } else {
    error!("Keyboard not found: {}", kb);
    std::process::exit(1);
  }
}

fn get_chip(keyboard: &Value) -> (String, String, String) {
  if !keyboard["keyboard"].is_table() {
    error!("Invalid keyboard configuration!");
    std::process::exit(1);
  }
  if !keyboard["keyboard"]["chip"].is_str() {
    error!("Chip not found in keyboard configuration!");
    std::process::exit(1);
  }

  let chip = keyboard["keyboard"]["chip"].as_str().unwrap();

  if chip.is_empty() {
    error!("Chip not found in keyboard configuration!");
    std::process::exit(1);
  }

  let chip_dir = format!("chips/{}", chip);
  let chip_toml = format!("{}/Cargo.toml", chip_dir);
  let chip_toml_path = Path::new(&chip_toml);
  if chip_toml_path.exists() {
    info!("Using Chip: {}", chip);
  } else {
    error!("Chip not found: {}", chip);
    std::process::exit(1);
  }

  (chip.to_string(), chip_dir, chip_toml)
}

fn needs_clean(kb: &str, chip: &str) {
  let last_build = Path::new("build/.last_build");
  if last_build.exists() {
    let last_build = fs::read_to_string(last_build).unwrap();
    if last_build != format!("{}-{}", kb, chip) {
      error!("Configuration changed. Cleaning build folder...");
      fs::remove_dir_all("build").unwrap_or(());
    }
  }
}

fn copy_folder(source: &str, target: &str, exclude_patterns: Vec<&str>) {
  let source_path = Path::new(source);
  let target_path = Path::new(target);

  fs::create_dir_all(target_path).unwrap();

  let contents: Vec<PathBuf> = fs::read_dir(source_path)
    .unwrap()
    .filter_map(|entry| {
      let path = entry.unwrap().path();
      if exclude_patterns.iter().any(|pattern| {
        path
          .file_name()
          .map(|name| name == *pattern)
          .unwrap_or(false)
      }) {
        None
      } else {
        Some(path)
      }
    })
    .collect();

  let mut options = CopyOptions::new();
  options.overwrite = true;
  options.copy_inside = true;

  copy_items(&contents, "build", &options).unwrap();
}

fn prepare_orbit_module() {
  fs::rename("build/src/lib.rs", "build/src/orbit.rs").unwrap();
  let content = fs::read_to_string("build/src/orbit.rs").unwrap();
  let content = content.replace("mod orbit {", "").replace("}", "");
  let content = content
    .lines()
    .filter(|line| !line.trim().is_empty())
    .collect::<Vec<&str>>()
    .join("\n");
  let content = content
    .lines()
    .map(|line| line.trim())
    .collect::<Vec<&str>>()
    .join("\n");

  fs::write("build/src/orbit.rs", content).unwrap();
}

fn save_last_build_cfg(kb: &str, chip: &str) {
  let cfg = format!("{}-{}", kb, chip);
  fs::write("build/.last_build", cfg).unwrap();
}
