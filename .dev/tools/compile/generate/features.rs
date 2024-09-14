use crate::toml;
use crate::util;

pub fn generate() {
  let mut behaviors = vec![];
  for file in util::list_files("src/rmk/behaviors") {
    let behavior = util::filename_no_ext(&file);
    behaviors.push(format!("behavior_{}_enabled = []", behavior));
  }

  let mut actions = vec![];
  for file in util::list_files("src/rmk/actions") {
    let action = util::filename_no_ext(&file);
    actions.push(format!("action_{}_enabled = []", action));
  }

  let mut cargo = toml::read_as_value("Cargo.toml");

  if let toml::Value::Table(ref mut features_table) = cargo["features"] {
    for feature in behaviors {
      let parts: Vec<&str> = feature.split(" = ").collect();
      features_table.insert(parts[0].to_string(), toml::Value::Array(vec![]));
    }

    for feature in actions {
      let parts: Vec<&str> = feature.split(" = ").collect();
      features_table.insert(parts[0].to_string(), toml::Value::Array(vec![]));
    }
  }

  toml::write("Cargo.toml", &cargo);
}
