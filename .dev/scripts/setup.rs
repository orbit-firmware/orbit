use std::process::{exit, Command};
use std::str;

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[0;31m";
const GREEN: &str = "\x1b[0;32m";
const BLUE: &str = "\x1b[0;34m";
const RUST_VERSION: &str = "1.80.1";

// use/install from toolchain????

fn main() {
  // Check and install the Rust version if needed
  let rust_version_installed = Command::new("rustup")
    .args(&["toolchain", "list"])
    .output()
    .expect("Failed to check installed Rust versions")
    .stdout;

  let rust_version_installed = str::from_utf8(&rust_version_installed).unwrap();
  if !rust_version_installed.contains(RUST_VERSION) {
    println!(
      "{}Rust version {}{} is not installed. Installing...",
      BLUE, RUST_VERSION, RESET
    );
    let status = Command::new("rustup")
      .args(&["toolchain", "install", RUST_VERSION])
      .status()
      .expect("Failed to install Rust version");

    if !status.success() {
      println!(
        "{}Failed to install Rust version {}!{}",
        RED, RUST_VERSION, RESET
      );
      exit(1);
    }
  }

  // Set the Rust version as default
  let status = Command::new("rustup")
    .args(&["default", RUST_VERSION])
    .status()
    .expect("Failed to set Rust version as default");

  if !status.success() {
    println!(
      "{}Failed to set Rust version {} as default.{}",
      RED, RUST_VERSION, RESET
    );
    exit(1);
  }

  // Install targets if needed
  let targets = [
    "thumbv6m-none-eabi", "thumbv7m-none-eabi", "thumbv7em-none-eabi", "thumbv7em-none-eabihf",
    "thumbv8m.base-none-eabi", "thumbv8m.main-none-eabi", "thumbv8m.main-none-eabihf",
    "riscv32i-unknown-none-elf", "riscv32imc-unknown-none-elf", "riscv32imac-unknown-none-elf",
    "riscv64gc-unknown-none-elf", "riscv64imac-unknown-none-elf",
  ];

  for target in &targets {
    let target_installed = Command::new("rustup")
      .args(&["target", "list", "--installed"])
      .output()
      .expect("Failed to check installed targets")
      .stdout;

    let target_installed = str::from_utf8(&target_installed).unwrap();
    if !target_installed.contains(*target) {
      println!("{}Installing target {}...{}", BLUE, target, RESET);
      let status = Command::new("rustup")
        .args(&["target", "add", target])
        .status()
        .expect("Failed to install target");

      if !status.success() {
        println!("{}Failed to install target {}{}", RED, target, RESET);
        exit(1);
      }
    }
  }

  // Install llvm-tools-preview if needed
  let llvm_tools_installed = Command::new("rustup")
    .args(&["component", "list", "--installed"])
    .output()
    .expect("Failed to check installed components")
    .stdout;

  let llvm_tools_installed = str::from_utf8(&llvm_tools_installed).unwrap();
  if !llvm_tools_installed.contains("llvm-tools") {
    println!("{}Installing llvm-tools-preview...{}", BLUE, RESET);
    let status = Command::new("rustup")
      .args(&["component", "add", "llvm-tools-preview"])
      .status()
      .expect("Failed to install llvm-tools-preview");

    if !status.success() {
      println!("{}Failed to install llvm-tools-preview{}", RED, RESET);
      exit(1);
    }
  }

  // cargo install cargo-binutils
  let cargo_binutils_installed = Command::new("cargo")
    .args(&["install", "cargo-binutils"])
    .status()
    .expect("Failed to install cargo-binutils");
  if !cargo_binutils_installed.success() {
    println!("{}Failed to install cargo-binutils{}", RED, RESET);
    exit(1);
  }
}
