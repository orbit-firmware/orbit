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
      println!("{}{}{}", RED, e, RESET);
      // std::process::exit(1);
      "".to_string()
    }
  };
  content
}
