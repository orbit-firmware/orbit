use crate::toml;
use crate::util;

// merges together the chip and orbit directories
pub fn run(chip_dir: &str, keyboard: &str) {
  let orbit_src_dir = "orbit/orbit/src";

  let orbit_files = util::list_files_recursive(&orbit_src_dir);
  for file in orbit_files {
    let build_file = util::repath(&file, &orbit_src_dir, "build/src/orbit");
    util::mkdir(util::dirname(&build_file).as_str());
    util::copy(&file, &build_file);
  }

  let orbit_dir = "orbit/orbit";

  let orbit_files = util::list_files(&orbit_dir);
  for file in orbit_files {
    let build_file = util::repath(&file, &orbit_dir, "build");
    util::mkdir(util::dirname(&build_file).as_str());
    util::copy(&file, &build_file);
  }

  let chip_files = util::list_files_recursive(&chip_dir);

  for file in chip_files {
    let build_file = util::repath(&file, &chip_dir, "build");
    util::mkdir(util::dirname(&build_file).as_str());

    if util::file_exists(&build_file) {
      let name = util::filename(&build_file);

      if name == "Cargo.toml" {
        toml::merge(&file, &build_file);
        toml::set_package_name(&build_file, &keyboard);
        continue;
      }

      if name == "rust-toolchain.toml" {
        toml::merge(&file, &build_file);
        continue;
      }
    }

    util::copy(&file, &build_file);
  }
}
