use serde::Deserialize;
use swc_core::{
  ecma::{ast::Program, visit::VisitMutWith},
  plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use swc_plugin_import::{PluginImportConfig, StyleConfig};

#[plugin_transform]
fn transform(mut program: Program, meta: TransformPluginProgramMetadata) -> Program {
  let config = meta.get_transform_plugin_config();

  let config: Vec<PluginImportConfig> = config
    .map(|raw| {
      let config = serde_json::from_str::<Vec<WasmPluginImportConfig>>(&raw)
        .expect("Failed to parse config of plugin-import");

      config
        .into_iter()
        .map(|config| PluginImportConfig {
          library_name: config.library_name,
          library_directory: config.library_directory,
          custom_name: None,
          custom_style_name: None,
          style: config.style.map(StyleConfig::Bool),
          camel_to_dash_component_name: config.camel_to_dash_component_name,
          transform_to_default_import: config.transform_to_default_import,
          ignore_es_component: config.ignore_es_component,
          ignore_style_component: config.ignore_style_component,
        })
        .collect()
    })
    .unwrap_or_default();

  let mut v = swc_plugin_import::plugin_import(&config);

  program.visit_mut_with(&mut v);

  program
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WasmPluginImportConfig {
  pub library_name: String,
  pub library_directory: Option<String>, // default to `lib`
  pub style: Option<bool>,

  pub camel_to_dash_component_name: Option<bool>, // default to true
  pub transform_to_default_import: Option<bool>,

  pub ignore_es_component: Option<Vec<String>>,
  pub ignore_style_component: Option<Vec<String>>,
}
