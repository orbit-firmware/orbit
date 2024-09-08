use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn prepare(keyboard: &str) {
  let config_path = format!("../keyboards/{}.toml", keyboard);
  let path = Path::new(&config_path);
  if !path.exists() {
    println!("\x1b[31mConfig: {} does not exist!\x1b[0m", config_path);
    println!("cargo:warning=The configuration file for the keyboard does not exist.");
    process::exit(1);
  }

  println!("\x1b[32mUsing Config: {}\x1b[0m", config_path);
  let cargo_toml = fs::read_to_string(path).expect("Failed to read Cargo.toml");
  let cargo_toml: toml::Value = cargo_toml.parse().expect("Failed to parse Cargo.toml");

  let chip = cargo_toml
    .get("chip")
    .expect("Chip not found in Cargo.toml");
  let feature_chip = chip.as_str().expect("Chip is not a string");

  if !feature_chip.is_empty() {
    env::set_var("CARGO_FEATURE_", feature_chip);
  }

  // Print out the features being used (for debugging)
  println!("cargo:rerun-if-changed={}", config_path);
  println!("cargo:info=Features set: {}", feature_chip);
}

fn main() {
  let keyboard = env::var("RMK_KEYBOARD");
  match keyboard {
    Ok(keyboard) => {
      prepare(keyboard.as_str());
    }
    Err(_) => {
      println!("cargo:warning=RMK_KEYBOARD environment variable is not set.");
    }
  }

  // // Read the `Cargo.toml` file
  //
  // let cargo_toml = fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");

  // // Parse the `Cargo.toml` file
  // let cargo_toml: toml::Value = cargo_toml.parse().expect("Failed to parse Cargo.toml");

  // // Get the features from the parsed TOML
  // if let Some(features) = cargo_toml.get("features") {
  //   if let Some(features_table) = features.as_table() {
  //     let feature_list: Vec<String> = features_table.keys().cloned().collect();

  //     // Print the features being used
  //     println!("cargo:warning=Features: {:?}", feature_list);

  //     // Optionally, set environment variables or pass info to rustc
  //     for feature in feature_list {
  //       // Example of passing each feature as a compiler cfg flag
  //       println!("cargo:rustc-cfg=feature=\"{}\"", feature);
  //     }
  //   }
  // }
}
