fn main() {
  println!("cargo:rustc-link-arg-bins=--nmagic");
  println!("cargo:rustc-link-arg-bins=-Tlink.x");
  #[cfg(feature = "debug")]
  println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
