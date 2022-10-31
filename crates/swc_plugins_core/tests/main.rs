use swc_plugins_core::transform;
use shared::swc_core::{
  base::{config::Options, Compiler},
  common::SourceMap
};
use std::sync::Arc;

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
  )
  .unwrap();
  insta::assert_snapshot!("plugin-import", res.code);
}
