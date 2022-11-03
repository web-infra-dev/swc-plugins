use std::env::current_dir;

use test_plugins::{BaseFixtureHook, FixtureTester, swc_plugins_core::types::TransformConfig};

#[test]
fn main() {
  let config: TransformConfig = shared::serde_json::from_str(
    r#"{
      "swc": { "jsc": { "externalHelpers": true }, "env": { "mode": "usage", "targets": "ie 11" } },
      "extensions": {
        "lockCorejsVersion": {
          "swcHelpers": "@@swc",
          "corejs": "@@corejs"
        }
      }
}"#,
  )
  .unwrap();
  let mut tester = FixtureTester::new(
    config,
    BaseFixtureHook,
    vec![],
  );

  tester.fixtures(&current_dir().unwrap().join("tests/fixtures"));
}
