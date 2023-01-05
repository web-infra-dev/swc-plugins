#![feature(test)]
#![allow(soft_unstable)]
use std::{env::current_dir, fs, path::Path, process::Termination, sync::Arc};
use swc_core::{
  base::{try_with_handler, Compiler},
  common::{FileName, SourceMap, GLOBALS},
};
use swc_plugins_core::minify;
extern crate test;

fn read_to_string(s: &Path) -> String {
  let file = fs::read(s).unwrap();

  String::from_utf8(file).unwrap()
}

#[bench]
fn minify_large_bundle_no_sourcemap(bencher: &mut test::Bencher) -> impl Termination {
  let config = shared::serde_json::from_str(
    r#"{
    "compress": {},
    "mangle": true,
    "sourceMap": false
  }"#,
  )
  .unwrap();
  bencher.iter(|| {
    test::black_box(
      minify(
        &config,
        "large_file.js",
        &read_to_string(
          &current_dir()
            .unwrap()
            .join("benches/fixtures/minify/large_file.js"),
        ),
      )
      .unwrap(),
    );
  })
}

#[bench]
fn minify_large_bundle_with_sourcemap(bencher: &mut test::Bencher) -> impl Termination {
  let config = shared::serde_json::from_str(
    r#"{
    "compress": {},
    "mangle": true,
    "sourceMap": true
  }"#,
  )
  .unwrap();
  bencher.iter(|| {
    test::black_box(
      minify(
        &config,
        "large_file.js",
        &read_to_string(
          &current_dir()
            .unwrap()
            .join("benches/fixtures/minify/large_file.js"),
        ),
      )
      .unwrap(),
    );
  })
}

#[bench]
fn swc_core_minify(bencher: &mut test::Bencher) -> impl Termination {
  let cm = Arc::new(SourceMap::new(Default::default()));
  let compiler = Compiler::new(cm.clone());
  let fm = cm.new_source_file(
    FileName::Anon,
    read_to_string(
      &current_dir()
        .unwrap()
        .join("benches/fixtures/minify/large_file.js"),
    ),
  );

  let config = shared::serde_json::from_str(
    r#"{
    "compress": {},
    "mangle": true,
    "sourceMap": true
  }"#,
  )
  .unwrap();

  bencher.iter(|| {
    GLOBALS.set(&Default::default(), || {
      try_with_handler(cm.clone(), Default::default(), |handler| {
        compiler.minify(fm.clone(), handler, &config)
      })
      .unwrap();
    })
  })
}
