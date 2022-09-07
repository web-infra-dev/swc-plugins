use std::collections::HashMap;

use modularize_imports::PackageConfig;
use plugin_import::PluginImportConfig;
use shared::{napi, napi_derive::napi, serde::Deserialize, swc::config::Options};

/**
 * Internal plugin
 */
#[derive(Default)]
#[napi(object)]
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
  pub extensions: Extensions,
}

pub struct TransformConfig {
  pub swc: Options,

  /// Internal rust-swc Plugins
  pub extensions: Extensions,
}
