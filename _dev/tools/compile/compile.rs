use crate::error;
use crate::info;
use crate::ok;
use crate::toml;
use crate::util;

use std::fs;
use std::process::{exit, Command};

// merges together the chip and orbit directories
pub fn prepare(chip_dir: &str, keyboard: &str) {
  let orbit_src_dir = "orbit/src";

  let orbit_files = util::list_files_recursive(&orbit_src_dir);
  for file in orbit_files {
    let build_file = util::repath(&file, &orbit_src_dir, "build/src/orbit");
    util::mkdir(util::dirname(&build_file).as_str());
    util::copy(&file, &build_file);
  }

  let orbit_dir = "orbit";

  let orbit_files = util::list_files(&orbit_dir);
  for file in orbit_files {
    let build_file = util::repath(&file, &orbit_dir, "build");
    util::mkdir(util::dirname(&build_file).as_str());
    util::copy(&file, &build_file);
  }

  let chip_files = util::list_files_recursive(&chip_dir);

  for file in chip_files {
    let build_file = util::repath(&file, &chip_dir, "build");
    util::mkdir(util::dirname(&build_file).as_str());

    if util::file_exists(&build_file) {
      let name = util::filename(&build_file);

      if name == "Cargo.toml" {
        toml::merge(&file, &build_file);
        toml::set_package_name(&build_file, &keyboard);
        continue;
      }

      if name == "rust-toolchain.toml" {
        toml::merge(&file, &build_file);
        continue;
      }
    }

    util::copy(&file, &build_file);
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
    info!("Rust version {} is not installed. Installing...", version);

    let status = Command::new("rustup")
      .args(&["toolchain", "install", version])
      .status()
      .expect("Failed to install Rust version");

    if !status.success() {
      error!("Failed to install Rust version {}", version);
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
    error!("Failed to set Rust version {} as default", version);
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
      info!("Target '{}' is not installed. Installing...", target);
      let status = Command::new("rustup")
        .args(&["target", "add", target.as_str()])
        .status()
        .expect("Failed to install target");

      if !status.success() {
        error!("Failed to install target");
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
      info!("Component '{}' is not installed. Installing...", component);
      let status = Command::new("rustup")
        .args(&["component", "add", component.as_str()])
        .status()
        .expect("Failed to install component");

      if !status.success() {
        error!("Failed to install component");
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
      info!("cargo package '{}' is not installed. Installing...", package);

      let status = Command::new("cargo")
        .args(&["install", package])
        .status()
        .expect("Failed to install package");

      if !status.success() {
        error!("Failed to install package");
        exit(1);
      }
    }
  }
}

pub fn install() {
  if !util::file_exists("rust-toolchain.toml") {
    error!("Missing rust-toolchain.toml");
    exit(1);
  }

  let rust_toolchain = toml::read("rust-toolchain.toml", true);

  let version: String = toml::get(&rust_toolchain, "toolchain/channel", false);
  let targets: Vec<String> = toml::get(&rust_toolchain, "toolchain/targets", false);
  let components: Vec<String> = toml::get(&rust_toolchain, "toolchain/components", false);
  let cargo_packages: Vec<String> = toml::get(&rust_toolchain, "cargo/packages", false);

  install_rust_version(&version);
  install_targets(targets);
  install_components(components);
  install_cargo_packages(cargo_packages);
}

fn run_emulator() {
  info!("💾 Emulator chip detected, running it");
  let mut args: Vec<&str> = vec!["run"];
  args.push("--features");
  args.push("emulator_enabled");

  util::run("cargo", &args);
}

fn compile_firmware() {
  util::run("cargo", &["build", "--release"]);

  let mut args: Vec<&str> = vec!["objcopy", "--release"];

  args.push("--");
  args.push("-O");

  {
    let mut bin_args = args.clone();
    bin_args.push("binary");
    bin_args.push("../firmware.bin");
    let status = util::run("cargo", &bin_args);
    if !status.success() {
      error!("The command failed with status: {}", status);
    } else {
      if !status.success() {
        error!("The command failed with status: {}", status);
      } else {
        if let Ok(metadata) = fs::metadata("../firmware.bin") {
          let size = metadata.len() as f64 / 1000.0;
          ok!(
            "    🎉firmware.bin ({}) compiled successfully",
            format!("{:.1}kb", size)
          );
        } else {
          ok!("    🎉firmware.bin compiled successfully");
        }
      }
    }
  }

  {
    let mut hex_args = args.clone();
    hex_args.push("ihex");
    hex_args.push("../firmware.hex");
    let status = util::run("cargo", &hex_args);
    if !status.success() {
      error!("The command failed with status: {}", status);
    } else {
      if let Ok(metadata) = fs::metadata("../firmware.hex") {
        let size = metadata.len() as f64 / 1000.0;
        ok!(
          "    🎉firmware.hex ({}) compiled successfully",
          format!("{:.1}kb", size)
        );
      } else {
        ok!("    🎉firmware.hex compiled successfully");
      }
    }
  }
}

pub fn compile(chip: &str) {
  if chip == "_emulator" {
    run_emulator();
    return;
  }

  compile_firmware();
}
