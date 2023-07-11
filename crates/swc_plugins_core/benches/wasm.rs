#![feature(test)]
#![allow(soft_unstable)]
use std::{
  env::current_dir,
  path::PathBuf,
  process::{Command, Termination},
  sync::Arc,
};

use swc_core::base::Compiler;
use swc_plugins_collection::{
  pass::{internal_transform_after_pass, internal_transform_before_pass},
  swc_plugin_import::PluginImportConfig,
  types::Extensions,
};
extern crate test;

#[cfg(feature = "plugin")]
#[bench]
fn wasm(bencher: &mut test::Bencher) -> impl Termination {
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

  bencher.iter(|| {
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
  })
}

#[bench]
fn native(bencher: &mut test::Bencher) -> impl Termination {
  let mut cmd = Command::new("cargo");
  dbg!(current_dir().unwrap());
  cmd
    .current_dir(current_dir().unwrap().join("tests/plugin"))
    .arg("build")
    .arg("--target")
    .arg("wasm32-wasi")
    .arg("--quiet")
    .arg("--release");

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

  bencher.iter(|| {
    let cm = Arc::new(Default::default());
    let compiler = Arc::new(Compiler::new(cm));
    let output = swc_plugins_core::transform(
      compiler,
      &serde_json::from_str(
        r#"{
        "jsc": {}
      }"#,
      )
      .unwrap(),
      &Extensions {
        plugin_import: Some(vec![PluginImportConfig {
          library_name: "foo".into(),
          library_directory: Some("lib".into()),
          ..Default::default()
        }]),
        ..Default::default()
      },
      "",
      "import { CamelCase } from 'foo';\nconsole.log(CamelCase)",
      None,
      None,
      internal_transform_before_pass,
      internal_transform_after_pass,
    )
    .unwrap();
    assert!(output.code.contains("foo/lib"));
  })
}
