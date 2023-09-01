use std::{env::current_dir, fs, path::Path};

use integration_tests::fixture::{ExpectedInfo, FixtureTesterHook};

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

    let plugin_path = current_dir()
      .unwrap()
      .join("../../packages/react-const-elements");

    let config = serde_json::from_str(&format!(
      r#"{{
        "extensions": {{}},
        "swc": {{
          "jsc": {{
            "target": "es2022",
            "experimental": {{
              "plugins": [
                ["{}", {{
                    "immutableGlobals": ["Component", "Counter"],
                    "allowMutablePropsOnTags": ["Counter", "FormattedMessage"]
                  }}
                ]
              ]
            }}
          }}
        }}
      }}"#,
      plugin_path.display()
    ))
    .unwrap();
    vec![ExpectedInfo::new(
      expected_path.to_string_lossy().to_string(),
      String::from_utf8(expected).unwrap(),
      Some(config),
    )]
  }
}

// the tests in `/deopt` are sceneries that this plugin doesn't optimize, but babel does. For example
// this plugin treats all global components as potential mutable, unlike babel, <Foo />
// is not permitted to hoist as it may change at some time.
#[test]
fn basic() {
  let mut tester = integration_tests::fixture::FixtureTester::new(
    Default::default(),
    BabelPortedTest,
    vec![],
    None,
  );

  tester.fixtures(
    &current_dir()
      .unwrap()
      .join(Path::new("tests/plugin_react_const_elements/fixtures")),
  );
}
