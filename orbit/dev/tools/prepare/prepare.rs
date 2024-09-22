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

// fn run_emulator() {
//   info!("💾 Emulator chip detected, running it");
//   let args: Vec<&str> = vec!["run"];
//   util::run("cargo", &args);
// }

// fn compile_firmware() {
//   util::run("cargo", &["build", "--release"]);

//   let mut args: Vec<&str> = vec!["objcopy", "--release"];

//   args.push("--");
//   args.push("-O");

//   {
//     let mut bin_args = args.clone();
//     bin_args.push("binary");
//     bin_args.push("../firmware.bin");
//     let status = util::run("cargo", &bin_args);
//     if !status.success() {
//       error!("The command failed with status: {}", status);
//     } else {
//       if !status.success() {
//         error!("The command failed with status: {}", status);
//       } else {
//         if let Ok(metadata) = fs::metadata("../firmware.bin") {
//           let size = metadata.len() as f64 / 1000.0;
//           ok!(
//             "    🎉firmware.bin ({}) compiled successfully",
//             format!("{:.1}kb", size)
//           );
//         } else {
//           ok!("    🎉firmware.bin compiled successfully");
//         }
//       }
//     }
//   }

//   {
//     let mut hex_args = args.clone();
//     hex_args.push("ihex");
//     hex_args.push("../firmware.hex");
//     let status = util::run("cargo", &hex_args);
//     if !status.success() {
//       error!("The command failed with status: {}", status);
//     } else {
//       if let Ok(metadata) = fs::metadata("../firmware.hex") {
//         let size = metadata.len() as f64 / 1000.0;
//         ok!(
//           "    🎉firmware.hex ({}) compiled successfully",
//           format!("{:.1}kb", size)
//         );
//       } else {
//         ok!("    🎉firmware.hex compiled successfully");
//       }
//     }
//   }
// }

// pub fn compile(chip: &str) {
//   if chip == "_emulator" {
//     run_emulator();
//     return;
//   }

//   compile_firmware();
// }
