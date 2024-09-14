use crate::toml;
use crate::util;

#[allow(unused_variables)]
pub fn generate(feature_list: &mut Vec<String>) {
  let mut behaviors = vec![];
  for file in util::list_files("src/orbit/behaviors") {
    let behavior = util::filename_no_ext(&file);
    behaviors.push(format!("behavior_{}_enabled = []", behavior));
  }

  let mut actions = vec![];
  for file in util::list_files("src/orbit/actions") {
    let action = util::filename_no_ext(&file);
    actions.push(format!("action_{}_enabled = []", action));
  }

  let mut cargo = toml::read_as_value("Cargo.toml");

  if let toml::Value::Table(ref mut root_table) = cargo {
    let features_table = root_table
      .entry("features")
      .or_insert_with(|| toml::Value::Table(toml::Map::new()));

    if let toml::Value::Table(ref mut features_table) = features_table {
      for feature in behaviors {
        let parts: Vec<&str> = feature.split(" = ").collect();
        features_table.insert(parts[0].to_string(), toml::Value::Array(vec![]));
      }

      for feature in actions {
        let parts: Vec<&str> = feature.split(" = ").collect();
        features_table.insert(parts[0].to_string(), toml::Value::Array(vec![]));
      }
    }
  }

  toml::write("Cargo.toml", &cargo);
}
