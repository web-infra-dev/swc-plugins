use std::sync::Arc;

use swc_core::{
  base::{config::Options, Compiler},
  common::SourceMap,
};
use swc_plugins_collection::{pass, types};
use swc_plugins_core::transform;

#[test]
fn test() {
  let code = "const a = {};
    const b = { ...a }";
  let swc_config = Options {
    config: serde_json::from_str(
      r#"{
          "jsc": {
            "externalHelpers": true
          }
        }"#,
    )
    .unwrap(),
    ..Default::default()
  };

  let extensions_config = types::Extensions::default();
  let res = transform(
    Arc::new(Compiler::new(Arc::new(SourceMap::default()))),
    &swc_config,
    &extensions_config,
    "",
    code,
    None,
    Some("".into()),
    pass::internal_transform_before_pass,
    pass::internal_transform_after_pass,
  )
  .unwrap();
  insta::assert_snapshot!("plugin-import", res.code);
}
