use std::collections::HashMap;

use plugin_import::PluginImportConfig;
use plugin_lock_corejs_version::LockCoreJsVersion;
use plugin_modularize_imports::PackageConfig;
use plugin_react_utils::ReactUtilsConfig;
use shared::swc::config::Options;

#[derive(Default, Debug)]
pub struct Extensions {
  pub modularize_imports: Option<HashMap<String, PackageConfig>>,
  pub plugin_import: Option<Vec<PluginImportConfig>>,
  pub react_utils: Option<ReactUtilsConfig>,
  pub lock_corejs_version: Option<LockCoreJsVersion>
}

#[derive(Debug, Default)]
pub struct TransformConfig {
  pub swc: Options,

  /// Internal rust-swc Plugins
  pub extensions: Extensions,
}
