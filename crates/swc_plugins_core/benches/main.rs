#![feature(test)]
#![allow(soft_unstable)]
use std::{env::current_dir, fs, path::Path, process::Termination};
use swc_plugins_core::minify;
extern crate test;

fn read_to_string(s: &Path) -> String {
  let file = fs::read(s).unwrap();

  String::from_utf8(file).unwrap()
}

#[bench]
fn minify_large_bundle(bencher: &mut test::Bencher) -> impl Termination {
  let config = shared::serde_json::from_str(
    r#"{
    "compress": {},
    "mangle": true,
    "sourceMap": false
  }"#,
  )
  .unwrap();
  bencher.iter(|| {
    test::black_box(minify(
      &config,
      "large_file.js".into(),
      &read_to_string(
        &current_dir()
          .unwrap()
          .join("benches/fixtures/minify/large_file.js"),
      ),
    ).unwrap());
  })
}
