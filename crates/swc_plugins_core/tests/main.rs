use shared::swc_core::{
  base::{try_with_handler, Compiler, config::Options},
  common::{sync::Lazy, FileName, GLOBALS, SourceMap},
};
use swc_plugins_core::transform;
use std::{env::current_dir, fs, sync::Arc, time};

static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| Arc::new(Compiler::new(Arc::default())));

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
    "".into(),
    code,
    None,
    Some("".into()),
  )
  .unwrap();
  insta::assert_snapshot!("plugin-import", res.code);

  GLOBALS.set(&Default::default(), || {
    try_with_handler(
      COMPILER.cm.clone(),
      Default::default(),
      |handler| -> shared::anyhow::Result<()> {
        let start = time::Instant::now();

        let source = String::from_utf8(
          fs::read(current_dir().unwrap().join("tests/source.js").as_path()).unwrap(),
        )
        .unwrap();

        let fm = COMPILER.cm.new_source_file(FileName::Anon, source);
        COMPILER
          .minify(
            fm,
            handler,
            &shared::serde_json::from_str(
              r#"{
        "mangle": true,
        "compress": {}
      }"#,
            )
            .unwrap(),
          )
          .unwrap();

        let end = time::Instant::now();
        panic!("{}", (end - start).as_millis())
      },
    ).unwrap()
  });
}
