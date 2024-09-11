use std::fs;

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

pub fn file(path: &str, optional: bool) -> String {
  let content = match fs::read_to_string(&path) {
    Ok(content) => content,
    Err(e) => {
      if optional {
        return String::new();
      }
      println!("{}Config file does not exist!: {}{}", RED, path, RESET);
      eprintln!("Warning: Failed to read Config file at '{}': {}", path, e);
      std::process::exit(1);
    }
  };
  content
}
