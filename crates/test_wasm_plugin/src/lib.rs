use swc_core::{ecma::{ast::Program, visit::FoldWith}, plugin::{proxies::TransformPluginProgramMetadata, plugin_transform}};
use swc_plugin_import::{plugin_import, PluginImportConfig};

#[plugin_transform]
pub fn plugin_import_wasm(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
  let config = metadata.get_transform_plugin_config();
  if let Some(config) = config {
    let plugin_import_config = serde_json::from_str::<Vec<PluginImportConfig>>(&config).unwrap();

    let mut pass = plugin_import(&plugin_import_config);

    program.fold_with(&mut pass)
  } else {
    program
  }

}
