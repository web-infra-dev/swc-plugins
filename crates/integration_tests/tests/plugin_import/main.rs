use swc_core::ecma::parser::Syntax;
use swc_plugins_collection::swc_plugin_import::{plugin_import, PluginImportConfig};

#[test]
fn import_test() {
  let config = vec![PluginImportConfig {
    library_name: "foo".into(),
    library_directory: None,
    custom_name: None,
    custom_style_name: None,
    style: None,
    camel_to_dash_component_name: None,
    transform_to_default_import: None,
    ignore_es_component: None,
    ignore_style_component: None,
  }];

  integration_tests::testing::test_transform(
    Syntax::Es(Default::default()),
    |_| plugin_import(&config),
    "import {Button} from 'foo';console.log(Button)",
    "import Button from \"foo/lib/button\";console.log(Button);",
    true,
  );
}
