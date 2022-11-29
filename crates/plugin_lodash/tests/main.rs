use plugin_lodash::PluginLodashConfig;
use shared::swc_core::cached::regex::CachedRegex;
use test_plugins::{
  swc_plugins_core::types::{Extensions, TransformConfig},
  BaseFixtureHook, FixtureTester,
};

#[test]
fn test_fixtures() {
  let cwd = std::env::current_dir().unwrap();
  // let fixtures = fs::read_dir(cwd.join("tests/fixtures").to_str().unwrap()).unwrap();

  let mut tester = FixtureTester::new(
    TransformConfig {
      swc: shared::serde_json::from_str(
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

  tester.fixtures(&cwd.join("tests/fixtures"));
}
