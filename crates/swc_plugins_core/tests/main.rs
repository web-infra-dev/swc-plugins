use std::{env::current_dir, fs, path::Path};

use swc_plugins_core::minify;

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
