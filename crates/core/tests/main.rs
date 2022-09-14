use core::transform::transform;

use pass::types::Extensions;
use shared::swc::config::Options;

#[test]
fn test() {
  let code = "const a = {};
  const b = { ...a }";
  let mut extension: Extensions = Default::default();

  let config = pass::types::TransformConfig {
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
    extensions: &mut extension,
  };

  let res = transform(None, config, code).unwrap();
  insta::assert_snapshot!("plugin-import", res.code);
}
