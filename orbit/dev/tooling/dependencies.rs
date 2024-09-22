use crate::error;
use crate::info;
use crate::toml;
use crate::util;

use std::process::{exit, Command};

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
