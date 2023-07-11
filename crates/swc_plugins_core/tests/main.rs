use std::{env::current_dir, fs, path::PathBuf, process::Command, sync::Arc};

use swc_core::base::Compiler;
use swc_plugins_collection::{
  pass::{internal_transform_after_pass, internal_transform_before_pass},
  types::Extensions,
};
use swc_plugins_core::{minify, minify_css, CssMinifyOptions};

#[test]
fn test() {
  let config = serde_json::from_str(
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
    &fs::read_to_string(
      current_dir()
        .unwrap()
        .join("benches/fixtures/minify/large_file.js"),
    )
    .unwrap(),
  )
  .unwrap();

  // should minify success
  minify_css(
    &CssMinifyOptions {
      source_map: false,
      inline_source_content: false,
    },
    "test",
    "body { color: red; }",
  )
  .unwrap();
}
