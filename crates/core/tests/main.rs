use core::transform::transform;
use shared::swc::config::Options;

#[test]
fn test() {
  let code = "const a = {};
  const b = { ...a }";

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
    extensions: Default::default(),
  };

  let res = transform(None, code, &config).unwrap();
  insta::assert_snapshot!("plugin-import", res.code);
}
