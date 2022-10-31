use std::{
  fs,
  path::{Path, PathBuf},
};

use plugin_lodash::PluginLodashConfig;
use shared::swc_core::cached::regex::CachedRegex;
use test_plugins::{
  run_test,
  swc_plugins_core::types::{Extensions, TransformConfig},
};

#[test]
fn test_fixtures() {
  let cwd = std::env::current_dir().unwrap();
  let fixtures = fs::read_dir(cwd.join("tests/fixtures").to_str().unwrap()).unwrap();

  for fixture in fixtures {
    let fixture = fixture.unwrap();
    run_fixture(&fixture.path(), cwd.clone());
  }
}

fn run_fixture(path: &Path, cwd: PathBuf) {
  let tests = fs::read_dir(path).unwrap();
  for test_dir in tests {
    let test_dir = test_dir.unwrap().path();

    let actual_file = test_dir.join("actual.js");
    let actual_content = String::from_utf8(fs::read(actual_file).unwrap()).unwrap();

    let expected_file = test_dir.join("expected.js");
    println!("run fixture test: {}", expected_file.display());
    run_test(
      test_dir.display().to_string().as_str(),
      &TransformConfig {
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
      &actual_content,
      expected_file
        .exists()
        .then(|| String::from_utf8(fs::read(&expected_file).unwrap()).unwrap()),
      Some(CachedRegex::new("error-fixtures").unwrap()),
    );
  }
}
