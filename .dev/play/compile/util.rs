use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;

#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => {
      println!("\x1b[31m{}\x1b[0m", format_args!($($arg)*));
  };
}

#[macro_export]
macro_rules! ok {
  ($($arg:tt)*) => {
      println!("\x1b[32m{}\x1b[0m", format_args!($($arg)*));
  };
}

#[macro_export]
macro_rules! warn {
  ($($arg:tt)*) => {
      println!("\x1b[33m{}\x1b[0m", format_args!($($arg)*));
  };
}

#[macro_export]
macro_rules! info {
  ($($arg:tt)*) => {
      println!("\x1b[34m{}\x1b[0m", format_args!($($arg)*));
  };
}

pub fn get_arg(n: usize) -> String {
  let arg: String = std::env::args().nth(n).unwrap();
  arg.trim_matches('"').trim_matches('\'').to_string()
}

pub fn get_root() -> String {
  let mut root = std::env::current_dir().unwrap().display().to_string();
  if root.contains(".dev") {
    root = root.split(".dev").collect::<Vec<&str>>()[0].to_string();
  }
  root.strip_suffix('/').unwrap().to_string()
}

pub fn file_exists(path: &str) -> bool {
  let metadata = std::fs::metadata(path);
  metadata.is_ok() && metadata.unwrap().is_file()
}

pub fn directory_exists(path: &str) -> bool {
  let metadata = std::fs::metadata(path);
  metadata.is_ok() && metadata.unwrap().is_dir()
}

pub fn filename(path: &str) -> String {
  Path::new(&path).file_name().unwrap().to_str().unwrap().to_string()
}

pub fn dirname(path: &str) -> String {
  Path::new(&path).parent().unwrap().display().to_string()
}

pub fn cd(path: &str) {
  std::env::set_current_dir(path).expect("Failed to change directory");
}

pub fn mkdir(path: &str) {
  if !directory_exists(path) {
    std::fs::create_dir_all(path).expect("Failed to create directory");
  }
}

pub fn repath(file: &str, from: &str, to: &str) -> String {
  let prefix = format!("{}/", from);
  let rel = file.replacen(&prefix, "", 1);
  format!("{}/{}", to, rel)
}

pub fn copy(from: &str, to: &str) {
  let metadata = std::fs::metadata(from);
  if metadata.is_err() {
    error!("File does not exist: {}", from);
    exit(1);
  }
  if metadata.unwrap().is_dir() {
    error!("Cannot copy directory: {}", from);
    exit(1);
  }

  mkdir(&dirname(to));
  std::fs::copy(from, to).expect("Failed to copy file");
}

pub fn replace_in_file(file_path: &str, target: &str, replacement: &str) {
  let content = std::fs::read_to_string(file_path).expect("Failed to read file");
  let new_content = content.replace(target, replacement);
  std::fs::write(file_path, new_content).expect("Failed to write file");
}

pub fn list_files(path: &str) -> Vec<String> {
  let p = Path::new(path);
  let mut files = Vec::new();
  if p.is_dir() {
    match std::fs::read_dir(p) {
      Ok(entries) => {
        for entry in entries {
          match entry {
            Ok(entry) => {
              let entry_path = entry.path();
              if entry_path.is_dir() {
                if entry_path.file_name().unwrap() == ".git" {
                  continue;
                }
                files.extend(list_files(&entry_path.display().to_string()));
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

pub fn run(cmd: &str, args: &[&str]) -> std::process::ExitStatus {
  let mut command = std::process::Command::new(cmd)
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