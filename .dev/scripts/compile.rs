//! ```cargo
//! [dependencies]
//! toml = "0.8"
//! serde-toml-merge = "0.3.8"
//! ```
//!
extern crate serde_toml_merge as stm;
extern crate toml as ctoml;

use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::{exit, Command};

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[0;31m";
const BLUE: &str = "\x1b[0;34m";
const GREEN: &str = "\x1b[0;32m";

mod toml {
  use ctoml::*;
  use std::fs;
  use std::process::exit;
  use stm::merge as tmerge;
  use RED;
  use RESET;

  pub fn read(path: &str, required: bool) -> Table {
    let content = match fs::read_to_string(&path) {
      Ok(content) => content,
      Err(e) => {
        if !required {
          return Table::new();
        }
        println!("{}File does not exist!: {}{}", RED, path, RESET);
        println!("{}{}{}", RED, e, RESET);
        exit(1);
      }
    };
    content.parse::<Table>().unwrap()
  }

  pub fn read_as_value(path: &str) -> Value {
    let content = match fs::read_to_string(&path) {
      Ok(content) => content,
      Err(e) => {
        println!("{}File does not exist!: {}{}", RED, path, RESET);
        println!("{}{}{}", RED, e, RESET);
        exit(1);
      }
    };
    content.parse::<Value>().unwrap()
  }

  pub fn required_string(table: &Table, key: &str) -> String {
    let mut keys = key.split('/');
    let mut current = table;

    while let Some(part) = keys.next() {
      current = match current.get(part) {
        Some(Value::Table(t)) => t,
        Some(Value::String(s)) => {
          return s.to_string();
        }
        _ => {
          let msg = format!("Missing '{}'", key);
          println!("{}{}{}", RED, msg, RESET);
          exit(1);
        }
      };
    }

    let msg = format!("Missing '{}'", key);
    println!("{}{}{}", RED, msg, RESET);
    exit(1);
  }

  pub fn required_string_list(table: &Table, key: &str) -> Vec<String> {
    let mut keys = key.split('/');
    let mut current = table;

    while let Some(part) = keys.next() {
      current = match current.get(part) {
        Some(Value::Table(t)) => t,
        Some(Value::Array(a)) => {
          return a
            .iter()
            .map(|v| match v {
              Value::String(s) => s.to_string(),
              _ => {
                let msg = format!("Expected a string in '{}'", key);
                println!("{}{}{}", RED, msg, RESET);
                exit(1);
              }
            })
            .collect();
        }
        _ => {
          let msg = format!("Missing '{}'", key);
          println!("{}{}{}", RED, msg, RESET);
          exit(1);
        }
      };
    }

    let msg = format!("Missing '{}'", key);
    println!("{}{}{}", RED, msg, RESET);
    exit(1);
  }

  pub fn merge(source: &str, target: &str) {
    let source_toml = read_as_value(source);
    let target_toml = read_as_value(target);
    let merged_toml = tmerge(target_toml, source_toml).unwrap();

    fs::write(
      target,
      to_string(&merged_toml).expect("Failed to serialize TOML"),
    )
    .expect("Failed to write target file");
  }
}

fn run(cmd: &str, args: &[&str]) -> std::process::ExitStatus {
  let mut command = Command::new(cmd)
    .args(args)
    .stdin(std::process::Stdio::null())
    .stdout(std::process::Stdio::piped())
    .spawn()
    .expect("Failed to start command");

  if let Some(stdout) = command.stdout.as_mut() {
    let reader = io::BufReader::new(stdout);
    for line in reader.lines() {
      match line {
        Ok(line) => println!("{}", line),
        Err(e) => eprintln!("Error reading line: {}", e),
      }
    }
  }

  let status = command.wait().expect("Command failed to run");
  status
}

fn get_arg(n: usize) -> String {
  let mut arg: String = std::env::args().nth(n).unwrap();
  arg = arg.trim_matches('"').trim_matches('\'').to_string();
  arg
}

fn file_exists(path: &str) -> bool {
  let metadata = fs::metadata(path);
  metadata.is_ok() && metadata.unwrap().is_file()
}

fn directory_exists(path: &str) -> bool {
  let metadata = fs::metadata(path);
  metadata.is_ok() && metadata.unwrap().is_dir()
}

fn dirname(path: &str) -> String {
  Path::new(&path).parent().unwrap().display().to_string()
}

fn list_files(path: &Path) -> Vec<String> {
  let mut files = Vec::new();
  if path.is_dir() {
    match fs::read_dir(path) {
      Ok(entries) => {
        for entry in entries {
          match entry {
            Ok(entry) => {
              let entry_path = entry.path();
              if entry_path.is_dir() {
                if entry_path.file_name().unwrap() == ".git" {
                  continue;
                }
                files.extend(list_files(&entry_path));
              } else {
                if entry_path.file_name().unwrap() == ".DS_Store" {
                  continue;
                }
                files.push(entry_path.display().to_string());
              }
            }
            Err(e) => eprintln!("Error reading entry: {:?}", e),
          }
        }
      }
      Err(e) => eprintln!("Error reading directory: {:?}", e),
    }
  }
  files
}

pub fn replace_in_file(file_path: &str, target: &str, replacement: &str) {
  let content = fs::read_to_string(file_path).expect("Failed to read file");
  let new_content = content.replace(target, replacement);
  fs::write(file_path, new_content).expect("Failed to write file");
}

fn prepare(chip_dir: &str, chip: &str, keyboard: &str) {
  let rmk_dir = "rmk";
  let files = list_files(Path::new(&chip_dir));
  let rmk_files = list_files(Path::new(&rmk_dir));

  for file in rmk_files {
    let prefix = format!("{}/", rmk_dir);
    let relative = file.replacen(&prefix, "", 1);
    let build_file = format!(".build/{}", relative);
    let dir = dirname(&build_file);
    if !directory_exists(&dir) {
      fs::create_dir_all(&dir).expect("Failed to create output directory");
    }
    fs::copy(&file, &build_file).expect("Failed to copy file");
  }

  for file in files {
    let prefix = format!("{}/", chip_dir);
    let relative = file.replacen(&prefix, "", 1);
    let build_file = format!(".build/{}", relative);

    let dir = dirname(&build_file);
    if !directory_exists(&dir) {
      fs::create_dir_all(&dir).expect("Failed to create output directory");
    }

    if file_exists(&build_file) {
      if relative == "Cargo.toml" {
        toml::merge(&file, &build_file);
        replace_in_file(&build_file, &chip, &keyboard);
        continue;
      }

      if relative == "rust-toolchain.toml" {
        toml::merge(&file, &build_file);
        continue;
      }
    }

    fs::copy(&file, &build_file).expect("Failed to copy file");
  }
}

fn install_rust_version(version: &str) {
  let rust_version_installed = Command::new("rustup")
    .args(&["toolchain", "list"])
    .output()
    .expect("Failed to check installed Rust versions")
    .stdout;

  let rust_version_installed = std::str::from_utf8(&rust_version_installed).unwrap();
  if !rust_version_installed.contains(version) {
    println!(
      "{}Rust version {}{} is not installed. Installing...",
      BLUE, version, RESET
    );

    let status = Command::new("rustup")
      .args(&["toolchain", "install", version])
      .stdout(std::process::Stdio::null())
      .stderr(std::process::Stdio::null())
      .status()
      .expect("Failed to install Rust version");

    if !status.success() {
      println!(
        "{}Failed to install Rust version {}!{}",
        RED, version, RESET
      );
      exit(1);
    }
  }

  let status = Command::new("rustup")
    .args(&["default", version])
    .stdout(std::process::Stdio::null())
    .stderr(std::process::Stdio::null())
    .status()
    .expect("Failed to set Rust version as default");

  if !status.success() {
    println!(
      "{}Failed to set Rust version {} as default.{}",
      RED, version, RESET
    );
    exit(1);
  }
}

fn install_targets(targets: Vec<String>) {
  let target_installed = Command::new("rustup")
    .args(&["target", "list", "--installed"])
    .output()
    .expect("Failed to check installed targets")
    .stdout;

  for target in &targets {
    let target_installed = std::str::from_utf8(&target_installed).unwrap();
    if !target_installed.contains(target.as_str()) {
      println!("{}Installing target {}...{}", BLUE, target, RESET);
      let status = Command::new("rustup")
        .args(&["target", "add", target.as_str()])
        .status()
        .expect("Failed to install target");

      if !status.success() {
        println!("{}Failed to install target {}{}", RED, target, RESET);
        exit(1);
      }
    }
  }
}

fn install_components(components: Vec<String>) {
  let component_installed = Command::new("rustup")
    .args(&["component", "list", "--installed"])
    .output()
    .expect("Failed to check installed components")
    .stdout;

  for component in &components {
    let check_name = component.clone().replace("-preview", "");
    let component_installed = std::str::from_utf8(&component_installed).unwrap();
    if !component_installed.contains(check_name.as_str()) {
      println!("{}Installing component {}...{}", BLUE, component, RESET);
      let status = Command::new("rustup")
        .args(&["component", "add", component.as_str()])
        .status()
        .expect("Failed to install component");

      if !status.success() {
        println!("{}Failed to install component {}{}", RED, component, RESET);
        exit(1);
      }
    }
  }
}

fn install_cargo_packages(packages: Vec<String>) {
  let check_binutils = Command::new("cargo")
    .args(&["install", "--list"])
    .output()
    .expect("Failed to list installed cargo tools");

  for package in &packages {
    let output = std::str::from_utf8(&check_binutils.stdout).unwrap_or("");

    if !output.contains(package) {
      println!(
        "{}cargo package '{}' is not installed. Installing...{}",
        BLUE, package, RESET
      );

      let status = Command::new("cargo")
        .args(&["install", package])
        .status()
        .expect("Failed to install package");

      if !status.success() {
        println!("{}Failed to install package{}", RED, RESET);
        exit(1);
      }
    }
  }
}

fn install_toolchain() {
  if !file_exists("rust-toolchain.toml") {
    println!("{}Missing rust-toolchain.toml{}", RED, RESET);
    exit(1);
  }

  let rust_toolchain = toml::read("rust-toolchain.toml", true);

  let version = toml::required_string(&rust_toolchain, "toolchain/channel");
  let targets = toml::required_string_list(&rust_toolchain, "toolchain/targets");
  let components = toml::required_string_list(&rust_toolchain, "toolchain/components");
  let cargo_packages = toml::required_string_list(&rust_toolchain, "cargo/packages");

  install_rust_version(&version);
  install_targets(targets);
  install_components(components);
  install_cargo_packages(cargo_packages);
}

fn compile() {
  let status = run(
    "cargo",
    &[
      "objcopy", "--release", "--", "-O", "binary", "../firmware.bin",
    ],
  );
  if !status.success() {
    eprintln!("The command failed with status: {}", status);
  } else {
    println!("    🎉{}firmware.bin compiled successfully{}", GREEN, RESET);
  }

  let status = run(
    "cargo",
    &[
      "objcopy", "--release", "--features", "keycodes_us", "--", "-O", "ihex", "../firmware.hex",
    ],
  );

  if !status.success() {
    eprintln!("The command failed with status: {}", status);
  } else {
    println!("    🎉{}firmware.hex compiled successfully{}", GREEN, RESET);
  }
}

// TAKE CARE
// this script assumes its ran from the root of the project!
fn main() {
  let keyboard: String = get_arg(1);
  let keyboard_file = format!("keyboards/{}.toml", keyboard);
  if !file_exists(&keyboard_file) {
    println!("{}Keyboard does not exist: {}{}", RED, keyboard_file, RESET);
    exit(1);
  }

  let keyboard_toml = toml::read(&keyboard_file, true);
  let chip = toml::required_string(&keyboard_toml, "keyboard/chip");

  let chip_dir = format!("chips/{}", chip);
  if !directory_exists(&chip_dir) {
    println!("{}Chip does not exist: {}{}", RED, chip_dir, RESET);
    exit(1);
  }

  if !directory_exists(".build") {
    fs::create_dir(".build").expect("Failed to create build directory");
  }

  fs::copy(&keyboard_file, ".build/keyboard.toml").expect("Failed to copy keyboard file");
  if file_exists("user/keyboard.toml") {
    toml::merge("user/keyboard.toml", ".build/keyboard.toml");
  }
  prepare(&chip_dir, &chip, &keyboard.as_str());
  std::env::set_current_dir(".build").expect("Failed to change to  build directory");

  install_toolchain();
  compile();
}
