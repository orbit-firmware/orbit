use std::fs;

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

pub fn file(path: &str) -> String {
  let content = match fs::read_to_string(&path) {
    Ok(content) => content,
    Err(e) => {
      println!("{}Keycodes file does not exist!: {}{}", RED, path, RESET);
      eprintln!("Warning: Failed to read keycodes file at '{}': {}", path, e);
      std::process::exit(1);
    }
  };
  content
}
