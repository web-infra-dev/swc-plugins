use std::{fs, path::Path};

use plugin_react_utils::{
  remove_prop_types::{Mode, ReactRemovePropTypeConfig},
  ReactUtilsConfig,
};
use shared::swc_core::base::config::Options;
use test_plugins::{
  run_test,
  swc_plugins_core::types::{Extensions, TransformConfig},
};

#[test]
fn main() {
  let fixture_dir = std::env::current_dir()
    .unwrap()
    .join("tests/fixtures/remove_prop_types");
  // let fixture_dir = std::env::current_dir()
  //   .unwrap()
  //   .join("crates/plugin_react_utils/tests/fixtures/remove_prop_types");

  let dirs = fs::read_dir(fixture_dir).unwrap();
  let swc: Options = shared::serde_json::from_str(
    r#"{
      "jsc": { "target": "es2020", "externalHelpers": true, "parser": { "syntax": "ecmascript", "jsx": true } }
    }"#,
  )
  .unwrap();

  for dir in dirs {
    let fixture_path = dir.unwrap().path();
    println!("{}", fixture_path.display());
    let actual_file_path = fixture_path.join("actual.js");
    if !actual_file_path.exists() {
      continue;
    }

    let actual_code = read(&actual_file_path);
    let options_path = fixture_path.join("options.json");
    let options: ReactRemovePropTypeConfig = if options_path.exists() {
      shared::serde_json::from_str(&read(&options_path)).unwrap()
    } else {
      Default::default()
    };

    let expected_remove_path = &fixture_path.join("expected-remove.js");
    if expected_remove_path.exists() {
      let mut options = options.clone();
      options.mode = Mode::Removal;
      let expected_code = read(expected_remove_path);
      run_test(
        expected_remove_path.to_str().unwrap(),
        &TransformConfig {
          swc: swc.clone(),
          extensions: Extensions {
            react_utils: Some(ReactUtilsConfig {
              rm_prop_types: Some(options),
              ..Default::default()
            }),
            ..Default::default()
          },
        },
        &actual_code,
        Some(expected_code),
        None,
      )
    }
    let expected_wrap_path = &fixture_path.join("expected-wrap.js");
    if expected_wrap_path.exists() {
      let mut options = options.clone();
      options.mode = Mode::Wrap;
      options.remove_import = false;

      let expected_code = read(expected_wrap_path);
      run_test(
        expected_wrap_path.to_str().unwrap(),
        &TransformConfig {
          swc: swc.clone(),
          extensions: Extensions {
            react_utils: Some(ReactUtilsConfig {
              rm_prop_types: Some(options),
              ..Default::default()
            }),
            ..Default::default()
          },
        },
        &actual_code,
        Some(expected_code),
        None,
      )
    }

    let expected_unsafe_wrap_path = &fixture_path.join("expected-unsafe-wrap.js");
    if expected_unsafe_wrap_path.exists() {
      let mut options = options.clone();
      options.mode = Mode::UnsafeWrap;
      options.remove_import = false;
      let expected_code = read(expected_unsafe_wrap_path);
      run_test(
        expected_unsafe_wrap_path.to_str().unwrap(),
        &TransformConfig {
          swc: swc.clone(),
          extensions: Extensions {
            react_utils: Some(ReactUtilsConfig {
              rm_prop_types: Some(options),
              ..Default::default()
            }),
            ..Default::default()
          },
        },
        &actual_code,
        Some(expected_code),
        None,
      )
    }

    let expected_path = &fixture_path.join("expected.js");
    if expected_path.exists() {
      let expected_code = read(expected_path);
      run_test(
        expected_path.to_str().unwrap(),
        &TransformConfig {
          swc: swc.clone(),
          extensions: Extensions {
            react_utils: Some(ReactUtilsConfig {
              rm_prop_types: Some(options),
              ..Default::default()
            }),
            ..Default::default()
          },
        },
        &actual_code,
        Some(expected_code),
        None,
      )
    }
  }
  // test_plugins::run_test("", config, code, expected, ignore)
}

fn read(dir: &Path) -> String {
  String::from_utf8(fs::read(dir).unwrap()).unwrap()
}
