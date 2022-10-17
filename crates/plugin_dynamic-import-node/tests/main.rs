use std::{
  fs,
  path::{Path, PathBuf},
};

use plugin_dynamic_import_node::DynImportNodeConfig;
use test_plugins::{
  modern_swc_core::types::{Extensions, TransformConfig},
  run_test,
};

#[test]
fn basic_import() {
  test("basic-import");
}

#[test]
fn chained_import() {
  test("chained-import");
}

fn test(p: &str) {
  let mut fixture_dir = PathBuf::from("./tests/fixtures");
  fixture_dir.push(p);

  let input = read(&fixture_dir.as_path().join(Path::new("actual.js")));

  let (expected, config) = (
    read(&fixture_dir.as_path().join(Path::new("expected.js"))),
    TransformConfig {
      swc: serde_json::from_str(
        r#"{
          "jsc": {
            "target": "es2020",
            "externalHelpers": true
          },
          "module": {
            "type": "commonjs"
          }
        }"#,
      )
      .unwrap(),
      extensions: Extensions {
        dyn_import_node: Some(DynImportNodeConfig { interop: true }),
        ..Default::default()
      },
    },
  );
  run_test(&format!("{}-expected", p), &config, &input, &expected);

  let (expected, config) = (
    read(&fixture_dir.as_path().join(Path::new("expected.es2015.js"))),
    TransformConfig {
      swc: serde_json::from_str(
        r#"{
          "jsc": {
            "target": "es5",
            "externalHelpers": true
          },
          "module": {
            "type": "commonjs"
          }
        }"#,
      )
      .unwrap(),
      extensions: Extensions {
        dyn_import_node: Some(DynImportNodeConfig { interop: true }),
        ..Default::default()
      },
    },
  );
  run_test(&format!("{}-expected.es2015", p), &config, &input, &expected);

  let (expected, config) = (
    read(&fixture_dir.as_path().join(Path::new("expected.nointerop.js"))),
    TransformConfig {
      swc: serde_json::from_str(
        r#"{
          "jsc": {
            "target": "es2020",
            "externalHelpers": true
          },
          "module": {
            "type": "commonjs"
          }
        }"#,
      )
      .unwrap(),
      extensions: Extensions {
        dyn_import_node: Some(DynImportNodeConfig { interop: false }),
        ..Default::default()
      },
    },
  );
  dbg!(&config);
  run_test(&format!("{}-expected.nointerop", p), &config, &input, &expected);
}

fn read(p: &Path) -> String {
  String::from_utf8(fs::read(p).unwrap()).unwrap()
}
