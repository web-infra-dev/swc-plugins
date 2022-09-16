use std::collections::HashMap;

use modularize_imports::PackageConfig;
use plugin_import::{PluginImportConfigNapi, PluginImportConfig};
use shared::{napi, napi_derive::napi, swc::config::Options};

/**
 * Internal plugin
 */
#[derive(Default)]
#[napi(object)]
pub struct ExtensionsNapi {
  pub modularize_imports: Option<HashMap<String, PackageConfig>>,
  pub plugin_import: Option<Vec<PluginImportConfigNapi>>,
}

#[derive(Default)]
pub struct Extensions {
  pub modularize_imports: Option<HashMap<String, PackageConfig>>,
  pub plugin_import: Option<Vec<PluginImportConfig>>,
}

#[napi(object)]
pub struct TransformConfigNapi {
  /// Raw swc options
  #[napi(ts_type = "import('./types').SwcOptions")]
  pub swc: String,

  /// Internal rust-swc Plugins
  pub extensions: ExtensionsNapi,
}

pub struct TransformConfig {
  pub swc: Options,

  /// Internal rust-swc Plugins
  pub extensions: Extensions,
}
