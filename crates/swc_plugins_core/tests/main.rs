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

#[test]
fn wasm() {
  let mut cmd = Command::new("cargo");

  cmd
    .current_dir("../test_wasm_plugin")
    .arg("build")
    .arg("--release")
    .arg("--target")
    .arg("wasm32-wasi");

  let mut handle = cmd.spawn().unwrap();

  let status = handle.wait().unwrap();
  if !status.success() {
    panic!("wasm build failed")
  }

  let target_dir: PathBuf = cargo_metadata::MetadataCommand::new()
    .no_deps()
    .exec()
    .unwrap()
    .target_directory
    .into();

  let wasm_path = target_dir.join("wasm32-wasi/release/plugin.wasm");

  assert!(wasm_path.exists());

  let cm = Arc::new(Default::default());
  let compiler = Arc::new(Compiler::new(cm));
  let output = swc_plugins_core::transform(
    compiler,
    &serde_json::from_str(&format!(
      "{{
          \"jsc\": {{
            \"experimental\": {{
              \"plugins\": [
                [\"{}\", [
                  {{
                    \"libraryName\": \"foo\",
                    \"libraryDirectory\": \"lib\"
                  }}
                ]]
              ]
            }}
          }}
        }}",
      wasm_path.display()
    ))
    .unwrap(),
    &Extensions::default(),
    "",
    "import { CamelCase } from 'foo';\nconsole.log(CamelCase)",
    None,
    None,
    internal_transform_before_pass,
    internal_transform_after_pass,
  )
  .unwrap();
  assert!(output.code.contains("foo/lib"));
}
