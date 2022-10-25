use plugin_react_utils::ReactUtilsConfig;
use test_plugins::{run_test, Extensions, TransformConfig};

#[test]
fn main() {
  run_test(
    "react",
    &TransformConfig {
      swc: Default::default(),
      extensions: Extensions {
        react_utils: Some(ReactUtilsConfig {
          auto_import_react: true,
          rm_effect: false,
        }),
        ..Default::default()
      },
    },
    "console.log(React)",
    Some(r#"
    import React from "react";
    console.log(React);"#),
    None,
  );
}
