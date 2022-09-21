use std::collections::HashMap;

use modularize_imports::PackageConfig;
use napi::bindgen_prelude::Buffer;
use plugin_import::{PluginImportConfigNapi, PluginImportConfig};
use react_utils::ReactUtilsConfig;
use shared::{napi, napi_derive::napi, swc::config::Options};

/**
 * Internal plugin
 */
#[derive(Default)]
#[napi(object)]
pub struct ExtensionsNapi {
  pub modularize_imports: Option<HashMap<String, PackageConfig>>,
  pub plugin_import: Option<Vec<PluginImportConfigNapi>>,
  pub react_utils: Option<ReactUtilsConfig>
}

#[derive(Default)]
pub struct Extensions {
  pub modularize_imports: Option<HashMap<String, PackageConfig>>,
  pub plugin_import: Option<Vec<PluginImportConfig>>,
  pub react_utils: Option<ReactUtilsConfig>
}

#[napi(object)]
pub struct TransformConfigNapi {
  /// Raw swc options
  pub swc: Buffer,

  /// Internal rust-swc Plugins
  pub extensions: ExtensionsNapi,
}

pub struct TransformConfig {
  pub swc: Options,

  /// Internal rust-swc Plugins
  pub extensions: Extensions,
}
