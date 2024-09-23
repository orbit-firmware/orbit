//# fs_extra = "1.3.0"
//# serde-toml-merge = "0.3.8"
//# toml = "0.8"
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

fn write_toml(target: &str, merged_toml: &toml::Value) {
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

pub fn main() {
  let args = std::env::args().collect::<Vec<String>>();
  if args.len() < 2 {
    error!("Please provide a keyboard name!");
    std::process::exit(1);
  }
  let keyboard_name = args.get(1).unwrap();
  let (keyboard, keyboard_toml) = get_keyboard(&keyboard_name);
  let (chip_name, chip_dir, chip_toml) = get_chip(&keyboard);

  needs_clean(&keyboard_name, &chip_name);
  copy_folder("orbit", "build", vec!["target", "Cargo.lock", "build.rs"]);
  copy_folder(&chip_dir, "build", vec!["target", "Cargo.lock"]);
  merge_toml("orbit/Cargo.toml", &chip_toml, "build/Cargo.toml", true);
  merge_toml(&keyboard_toml, "user/keyboard.toml", "build/keyboard.toml", false);
  prepare_orbit_module();
  save_last_build_cfg(&keyboard_name, &chip_name);
  ok!("Pre-Compile completed!");
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
      if exclude_patterns
        .iter()
        .any(|pattern| path.file_name().map(|name| name == *pattern).unwrap_or(false))
      {
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
