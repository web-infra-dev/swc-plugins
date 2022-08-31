use std::collections::HashMap;

use modularize_imports::PackageConfig;
use shared::{
  serde::Deserialize,
  swc::config::{Options},
};
use plugin_import::PluginImportItem;

/**
 * Internal any plugin
 */
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all="camelCase")]
pub struct Extensions {
    pub modularize_imports: Option<HashMap<String, PackageConfig>>,
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
