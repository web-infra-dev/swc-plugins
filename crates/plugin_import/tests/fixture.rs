use std::path::PathBuf;

use plugin_import::{plugin_import, PluginImportItem};
use shared::swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_transforms_testing::test_fixture;
use testing_macros::fixture;

#[fixture("tests/fixtures/*")]
fn test(base: PathBuf) {
  println!("{}", base.display());
  let input = base.join("input.jsx");

  let output = base.join("output.js");

  let config = vec![
    PluginImportItem {
      source: "basic".into(),
      transform_es: "foo/lib/es/{{member}}".into(),
      transform_style: None,
      ignore_components: None,
      snake_case: false,
    },
    PluginImportItem {
      source: "snake_case".into(),
      transform_es: "foo/lib/es/{{member}}".into(),
      transform_style: Some("foo/style/{{member}}".into()),
      ignore_components: None,
      snake_case: true,
    },
    PluginImportItem {
      source: "dash".into(),
      transform_es: "foo/lib/es/{{member}}".into(),
      transform_style: Some("foo/style/{{member}}".into()),
      ignore_components: None,
      snake_case: true,
    },
  ];

  test_fixture(
    Syntax::Es(EsConfig { jsx: true, ..Default::default() }),
    &|_| plugin_import(&config),
    &input,
    &output,
  );
}
