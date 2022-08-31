use shared::{
  serde::Deserialize,
  swc::config::{Options},
};

#[derive(Debug, Deserialize)]
pub struct Extensions {
    pub plugin_import: Option<Vec<PluginImportItem>>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformConfig {
  /// Raw swc options
  pub swc: Options,

  /// Internal rust-swc Plugins
  pub extensions: Extensions
}

#[derive(Debug, Deserialize)]
pub struct PluginImportItem {
  pub source: String,

  pub transform_es: String,
  pub transform_style: String,

  pub ignore_components: Option<Vec<String>>,
  pub lower: bool,
}
