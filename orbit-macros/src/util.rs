pub fn os_path(path: &str) -> String {
  path.replace("/", &std::path::MAIN_SEPARATOR.to_string())
}

pub fn get_root() -> String {
  let mut root = std::env::current_dir().unwrap().display().to_string();
  if root.contains(os_path("orbit").as_str()) {
    root = root.split(os_path("orbit").as_str()).collect::<Vec<&str>>()[0].to_string();
    root = format!("{}orbit", root);
  }

  os_path(&root)
}

pub fn to_pascal_case(s: &str) -> String {
  s.split(|c: char| c == '_' || c == ' ' || c.is_ascii_uppercase() && !c.is_ascii_alphabetic())
    .flat_map(|word| {
      if word.is_empty() {
        None
      } else {
        let mut chars = word.chars();
        Some(chars.next().unwrap().to_ascii_uppercase().to_string() + chars.as_str())
      }
    })
    .collect()
}

pub fn file_exists(path: &str) -> bool {
  let p = os_path(&path);
  let metadata = std::fs::metadata(p);
  metadata.is_ok() && metadata.unwrap().is_file()
}
