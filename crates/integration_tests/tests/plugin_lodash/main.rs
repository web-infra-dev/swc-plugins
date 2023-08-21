use integration_tests::{
  fixture::{BaseFixtureHook, FixtureTester},
  swc_plugins_collection::types::{Extensions, TransformConfig},
};
use swc_core::cached::regex::CachedRegex;
use swc_plugins_collection::swc_plugin_lodash::PluginLodashConfig;

#[test]
fn lodash() {
  let cwd = std::env::current_dir().unwrap();

  let mut tester = FixtureTester::new(
    TransformConfig {
      swc: serde_json::from_str(
        r#"
      {
        "jsc": {
          "parser": {
            "syntax": "typescript",
            "tsx": true
          },
          "transform": {
            "react": {
              "runtime": "automatic"
            }
          },
          "externalHelpers": true
        },
        "module": {
          "type": "commonjs"
        }
      }
    "#,
      )
      .unwrap(),
      extensions: Extensions {
        lodash: Some(PluginLodashConfig {
          cwd: cwd.clone(),
          ids: vec!["react-bootstrap".into(), "@storybook/addon-links".into()],
        }),
        ..Default::default()
      },
    },
    BaseFixtureHook,
    vec![CachedRegex::new("error-fixtures").unwrap()],
    Some("".into()),
  );

  tester.fixtures(&cwd.join("tests/plugin_lodash/fixtures"));
}
