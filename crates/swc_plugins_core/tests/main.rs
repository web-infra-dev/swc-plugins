use std::{env::current_dir, fs, path::Path, sync::Arc};
use swc_core::{
  base::{config::Options, Compiler},
  common::SourceMap,
};
use swc_plugins_core::{minify, transform};

#[test]
fn test() {
  let code = "const a = {};
  const b = { ...a }";

  let config = swc_plugins_core::types::TransformConfig {
    swc: Options {
      config: shared::serde_json::from_str(
        r#"{
        "jsc": {
          "externalHelpers": true
        }
      }"#,
      )
      .unwrap(),
      ..Default::default()
    },
    extensions: Default::default(),
  };

  let res = transform(
    Arc::new(Compiler::new(Arc::new(SourceMap::default()))),
    &config,
    "",
    code,
    None,
    Some("".into()),
  )
  .unwrap();
  insta::assert_snapshot!("plugin-import", res.code);

  let config = shared::serde_json::from_str(
    r#"{
    "compress": {},
    "mangle": true,
    "sourceMap": false
  }"#,
  )
  .unwrap();

  minify(
    &config,
    "large_file.js",
    &read_to_string(
      &current_dir()
        .unwrap()
        .join("benches/fixtures/minify/large_file.js"),
    ),
  )
  .unwrap();
}

fn read_to_string(s: &Path) -> String {
  let file = fs::read(s).unwrap();

  String::from_utf8(file).unwrap()
}
