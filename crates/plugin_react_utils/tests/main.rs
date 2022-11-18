use std::{fs, path::Path};

use plugin_react_utils::{
  remove_prop_types::{Mode, ReactRemovePropTypeConfig},
  ReactUtilsConfig,
};
use shared::swc_core::base::config::Options;
use test_plugins::{
  swc_plugins_core::types::{Extensions, TransformConfig},
  ExpectedInfo, FixtureTester, FixtureTesterHook,
};

#[test]
fn main() {
  let fixture_dir = std::env::current_dir().unwrap().join("tests/fixtures");
  // let fixture_dir = std::env::current_dir()
  //   .unwrap()
  //   .join("crates/plugin_react_utils/tests/fixtures/remove_prop_types");
  struct Hook;
  impl FixtureTesterHook for Hook {
    fn on_resolve_expected_files(
      &mut self,
      fixture_path: &Path,
      _config: &mut TransformConfig,
    ) -> Vec<ExpectedInfo> {
      let mut tests = vec![];

      let options_path = fixture_path.join("options.json");
      let remove_prop_types: ReactRemovePropTypeConfig = if options_path.exists() {
        shared::serde_json::from_str(&read(&options_path)).unwrap()
      } else {
        Default::default()
      };

      let swc: Options = shared::serde_json::from_str(r#"{
        "jsc": { "target": "es2020", "externalHelpers": true, "parser": { "syntax": "ecmascript", "jsx": true } }
      }"#).unwrap();
      let react_utils = ReactUtilsConfig {
        remove_prop_types: Some(remove_prop_types.clone()),
        ..Default::default()
      };

      let expected_path = &fixture_path.join("expected.js");
      if expected_path.exists() {
        let expected_code = read(expected_path);
        tests.push(ExpectedInfo::new(
          expected_path.to_string_lossy().to_string(),
          expected_code,
          Some(TransformConfig {
            swc: swc.clone(),
            extensions: Extensions {
              react_utils: Some(react_utils),
              ..Default::default()
            },
          }),
        ));
      }

      let expected_remove_path = &fixture_path.join("expected-remove.js");
      if expected_remove_path.exists() {
        let mut remove_prop_types = remove_prop_types.clone();
        remove_prop_types.mode = Mode::Removal;
        remove_prop_types.remove_import = true;

        let expected_code = read(expected_remove_path);
        tests.push(ExpectedInfo::new(
          expected_path.to_string_lossy().to_string(),
          expected_code,
          Some(TransformConfig {
            swc: swc.clone(),
            extensions: Extensions {
              react_utils: Some(ReactUtilsConfig {
                remove_prop_types: Some(remove_prop_types),
                ..Default::default()
              }),
              ..Default::default()
            },
          }),
        ));
      }

      let expected_wrap_path = &fixture_path.join("expected-wrap.js");
      if expected_wrap_path.exists() {
        let mut remove_prop_types = remove_prop_types.clone();
        remove_prop_types.mode = Mode::Wrap;
        remove_prop_types.remove_import = false;

        let expected_code = read(expected_wrap_path);
        tests.push(ExpectedInfo::new(
          expected_wrap_path.to_string_lossy().to_string(),
          expected_code,
          Some(TransformConfig {
            swc: swc.clone(),
            extensions: Extensions {
              react_utils: Some(ReactUtilsConfig {
                remove_prop_types: Some(remove_prop_types),
                ..Default::default()
              }),
              ..Default::default()
            },
          }),
        ));
      }

      let expected_unsafe_wrap_path = &fixture_path.join("expected-unsafe-wrap.js");
      if expected_unsafe_wrap_path.exists() {
        let mut remove_prop_types = remove_prop_types;
        remove_prop_types.mode = Mode::UnsafeWrap;
        remove_prop_types.remove_import = false;

        let expected_code = read(expected_unsafe_wrap_path);
        tests.push(ExpectedInfo::new(
          expected_unsafe_wrap_path.to_string_lossy().to_string(),
          expected_code,
          Some(TransformConfig {
            swc,
            extensions: Extensions {
              react_utils: Some(ReactUtilsConfig {
                remove_prop_types: Some(remove_prop_types),
                ..Default::default()
              }),
              ..Default::default()
            },
          }),
        ));
      }

      tests
    }
  }

  let mut tester = FixtureTester::new(
    shared::serde_json::from_str(r#"{
      "swc": {
        "jsc": { "target": "es2020", "externalHelpers": true, "parser": { "syntax": "ecmascript", "jsx": true } }
      },
      "extensions": {
        "reactUtils": {
          "rmPropTypes": {}
        }
      }
    }"#).unwrap(),
    Hook{},
    vec![],
    None
  );

  tester.fixtures(&fixture_dir);
}

fn read(dir: &Path) -> String {
  String::from_utf8(fs::read(dir).unwrap()).unwrap()
}
