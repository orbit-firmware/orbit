fn main() {
  println!("cargo:rerun-if-changed=keyboard_config.toml");
  println!("cargo:rerun-if-changed=../user/config.toml");
}
