use std::{env::current_dir, fs, path::Path};

use swc_plugins_collection::types::{Extensions, TransformConfig};
use test_plugins::fixture::{BaseFixtureHook, ExpectedInfo, FixtureTesterHook};

struct BabelPortedTest;

impl FixtureTesterHook for BabelPortedTest {
  fn on_resolve_actual_file(
    &mut self,
    fixture_path: &Path,
    _config: &mut swc_plugins_collection::types::TransformConfig,
  ) -> String {
    let mut path = fixture_path.join("input.js");
    if !path.exists() {
      path = fixture_path.join("input.mjs")
    }

    String::from_utf8(fs::read(path).unwrap()).unwrap()
  }

  fn on_resolve_expected_files(
    &mut self,
    fixture_path: &Path,
    _config: &mut swc_plugins_collection::types::TransformConfig,
  ) -> Vec<ExpectedInfo> {
    let expected_path = fixture_path.join("output.js");
    let expected = fs::read(&expected_path)
      .or_else(|_| fs::read(fixture_path.join("output.mjs")))
      .unwrap();

    let config = serde_json::from_str(
      r#"{
      "swc": {
        "jsc": {
          "target": "es2022"
        }
      },
      "extensions": { "reactConstElements": true }
    }"#,
    )
    .unwrap();

    vec![ExpectedInfo::new(
      expected_path.to_string_lossy().to_string(),
      String::from_utf8(expected).unwrap(),
      Some(config),
    )]
  }
}

#[test]
fn basic() {
  let mut tester =
    test_plugins::fixture::FixtureTester::new(Default::default(), BabelPortedTest, vec![], None);

  tester.fixtures(&current_dir().unwrap().join(Path::new("tests/fixtures")));

  // tester.run_test(
  //   "test_name",
  //   &serde_json::from_str(
  //     r#"{
  //   "swc": {
  //     "jsc": {
  //       "target": "es2022"
  //     }
  //   },
  //   "extensions": { "reactConstElements": true }
  // }"#,
  //   )
  //   .unwrap(),
  //   "class Component extends React.Component {
  //     subComponent = () => <span>Sub Component</span>
    
  //     render = () => <this.subComponent />
  //   }
    
  //   ",
  //   Some("()"),
  // )
}
