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

pub fn file_exists(path: &str) -> bool {
  let p = os_path(&path);
  let metadata = std::fs::metadata(p);
  metadata.is_ok() && metadata.unwrap().is_file()
}

// include_bytes! of every file the macros read, so cargo rebuilds the firmware when the
// keyboard toml, the keymap or a keycode table changes (rustc tracks included files)
pub fn track_inputs() -> proc_macro2::TokenStream {
  let root = get_root();
  let mut paths = vec![format!("{}/build/keyboard.toml", root), format!("{}/build/keymap.orbit", root)];
  let kcs = std::fs::read_dir(format!("{}/orbit/keycodes", root)).into_iter().flatten().flatten();
  paths.extend(kcs.map(|e| e.path().display().to_string()));
  let paths = paths.into_iter().filter(|p| file_exists(p));
  quote::quote! { #(const _: &[u8] = include_bytes!(#paths);)* }
}
