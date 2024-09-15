//# prettyplease = "0.2.2"
//# syn = "2.0"
//# proc-macro2 = "1.0"

use prettyplease::unparse;
use proc_macro2::TokenStream;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::{exit, Command, Stdio};
use std::thread;
use syn::{parse_file, File};

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

pub fn capitalize_first(s: &str) -> String {
  if s.is_empty() {
    return s.to_string();
  }
  let mut chars = s.chars();
  let first = chars.next().unwrap().to_uppercase().to_string();
  let rest: String = chars.collect();
  first + &rest
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

pub fn remove_extension(path: &str) -> String {
  let mut parts = path.split('.').collect::<Vec<&str>>();
  parts.pop();
  parts.join(".")
}

pub fn filename_no_ext(path: &str) -> String {
  remove_extension(&filename(path))
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
  write(file_path, &new_content);
}

pub fn quote_to_string(ts: TokenStream) -> String {
  let parsed: File = parse_file(&ts.to_string()).unwrap();
  let generation_comment = "// This file is generated by orbit\n\n";
  format!("{}{}", generation_comment, unparse(&parsed).as_str())
}

pub fn write(file_path: &str, content: &str) {
  std::fs::write(file_path, content).expect("Failed to write file");
}

pub fn list_files_recursive(path: &str) -> Vec<String> {
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
                files.extend(list_files_recursive(&entry_path.display().to_string()));
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
              if !entry_path.is_dir() {
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
  let mut command = Command::new(cmd)
    .args(args)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("Failed to start command");

  let stdout = command.stdout.take().expect("Failed to capture stdout");
  let stdout_thread = thread::spawn(move || {
    let reader = io::BufReader::new(stdout);
    for line in reader.lines() {
      match line {
        Ok(line) => println!("{}", line),
        Err(e) => eprintln!("Error reading stdout line: {}", e),
      }
    }
  });

  let stderr = command.stderr.take().expect("Failed to capture stderr");
  let stderr_thread = thread::spawn(move || {
    let reader = io::BufReader::new(stderr);
    for line in reader.lines() {
      match line {
        Ok(line) => eprintln!("{}", line),
        Err(e) => eprintln!("Error reading stderr line: {}", e),
      }
    }
  });

  let status = command.wait().expect("Command failed to run");

  stdout_thread.join().expect("Failed to join stdout thread");
  stderr_thread.join().expect("Failed to join stderr thread");

  status
}