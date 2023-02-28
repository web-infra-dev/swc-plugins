use std::env::current_dir;

use test_plugins::{
  fixture::{BaseFixtureHook, FixtureTester},
  swc_plugins_collection::types::TransformConfig,
};

#[test]
fn main() {
  let config: TransformConfig = serde_json::from_str(
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
  let mut tester = FixtureTester::new(config, BaseFixtureHook, vec![], None);

  tester.fixtures(&current_dir().unwrap().join("tests/fixtures"));
}
