use std::collections::HashMap;

use plugin_modularize_imports::PackageConfig;
use plugin_import::PluginImportConfig;
use plugin_react_utils::ReactUtilsConfig;
use shared::swc::config::Options;

#[derive(Default)]
pub struct Extensions {
  pub modularize_imports: Option<HashMap<String, PackageConfig>>,
  pub plugin_import: Option<Vec<PluginImportConfig>>,
  pub react_utils: Option<ReactUtilsConfig>,
}

pub struct TransformConfig {
  pub swc: Options,

  /// Internal rust-swc Plugins
  pub extensions: Extensions,
}
