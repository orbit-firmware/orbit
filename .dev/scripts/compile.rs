//! ```cargo
//! [dependencies]
//! toml = "0.8"
//! serde-toml-merge = "0.3.8"
//! ctrlc = "3.4.5"
//! ```
//!
extern crate serde_toml_merge as stm;
extern crate toml as ctoml;

use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::{exit, Command};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[0;31m";
const GREEN: &str = "\x1b[0;32m";
const BLUE: &str = "\x1b[0;34m";

mod toml {
  use ctoml::*;
  use std::fs;
  use std::process::{exit, Command};
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

  pub fn merge_cargo(source: &str, target: &str) {
    let mut source_toml = read(source, true);
    let mut target_toml = read(target, true);

    if let Some(source_features) = source_toml.get_mut("features") {
      if let Some(target_features) = target_toml.get_mut("features") {
        target_features
          .as_table_mut()
          .unwrap()
          .extend(source_features.as_table_mut().unwrap().clone());
      } else {
        target_toml["features"] = source_features.clone();
      }
    }

    if let Some(source_deps) = source_toml.get_mut("dependencies") {
      if let Some(target_deps) = target_toml.get_mut("dependencies") {
        target_deps
          .as_table_mut()
          .unwrap()
          .extend(source_deps.as_table_mut().unwrap().clone());
      } else {
        target_toml["dependencies"] = source_deps.clone();
      }
    }

    fs::write(
      target,
      toml::to_string(&target_toml).expect("Failed to write TOML"),
    )
    .expect("Failed to write target file");
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

fn ctrlc_handler(chip_dir: &String) {
  let chip_dir_arc = Arc::new(Mutex::new(chip_dir.clone()));

  ctrlc::set_handler({
    let chip_dir_arc = Arc::clone(&chip_dir_arc);
    move || {
      let chip_dir = chip_dir_arc.lock().unwrap();
      cleanup(&chip_dir);
      exit(0);
    }
  })
  .expect("Error setting Ctrl+C handler");
}

fn prepare(chip_dir: &str) {
  let files = list_files(Path::new(&chip_dir));

  let backup_folder = Path::new("bkp");
  if !directory_exists("bkp") {
    fs::create_dir_all(&backup_folder).expect("Failed to create backup directory");
  }

  for file in files {
    let mut relative = file.replace(&chip_dir, "");
    relative = relative.strip_prefix("/").unwrap().to_string();
    let backup_file = format!("bkp/{}", relative);
    let root_file = relative.clone();

    if file_exists(&root_file) {
      let dir = dirname(&backup_file);
      if !directory_exists(&dir) {
        fs::create_dir_all(&dir).expect("Failed to create output directory");
      }

      fs::copy(&root_file, &backup_file).expect("Failed to copy file");
    }

    let dir = dirname(&root_file);
    if !directory_exists(&dir) {
      fs::create_dir_all(&dir).expect("Failed to create output directory");
    }

    if relative == "Cargo.toml" {
      toml::merge_cargo(&file, &root_file);
    } else {
      fs::copy(&file, &root_file).expect("Failed to copy file");
    }
  }
}

fn cleanup(chip_dir: &str) {
  let files = list_files(Path::new(&chip_dir));

  for file in files.clone() {
    let mut relative = file.replace(&chip_dir, "");
    relative = relative.strip_prefix("/").unwrap().to_string();
    let root_file = relative.clone();
    let backup_file = format!("bkp/{}", relative);

    if file_exists(&root_file) {
      fs::remove_file(&root_file).expect("Failed to remove file");
    }
    let dir = dirname(&root_file);
    if directory_exists(&dir) {
      if fs::read_dir(&dir).unwrap().next().is_none() {
        fs::remove_dir(&dir).expect("Failed to remove directory");
      }
    }

    let backup_file = format!("bkp/{}", relative);
    if file_exists(&backup_file) {
      let dir = dirname(&root_file);
      if !directory_exists(&dir) {
        fs::create_dir_all(&dir).expect("Failed to create output directory");
      }
      fs::copy(&backup_file, &root_file).expect("Failed to copy file");
    }
  }

  fs::remove_dir_all("bkp").expect("Failed to remove backup directory");
  fs::remove_file("keyboard.toml").expect("Failed to remove keyboard file");
}

// TAKE CARE
// this script assumes its ran from the root of the project!
fn main() {
  run("cargo", &["script", ".dev/scripts/setup.rs"]);

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

  fs::copy(&keyboard_file, "keyboard.toml").expect("Failed to copy keyboard file");
  if file_exists("user/keyboard.toml") {
    toml::merge("user/keyboard.toml", "keyboard.toml");
  }

  ctrlc_handler(&chip_dir);
  prepare(&chip_dir);

  let status = run(
    "cargo",
    &[
      "objcopy", "--release", "--features", "keycodes_us", "--", "-O", "binary", "firmware.bin",
    ],
  );

  let status = run(
    "cargo",
    &[
      "objcopy", "--release", "--features", "keycodes_us", "--", "-O", "ihex", "firmware.hex",
    ],
  );

  if !status.success() {
    eprintln!("The command failed with status: {}", status);
  }

  cleanup(&chip_dir);
}
