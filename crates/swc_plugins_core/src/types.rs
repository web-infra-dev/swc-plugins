use std::collections::HashMap;

use plugin_import::PluginImportConfig;
use plugin_lock_corejs_version::LockCoreJsVersion;
use plugin_lodash::PluginLodashConfig;
use plugin_modularize_imports::PackageConfig;
use plugin_react_utils::ReactUtilsConfig;
use shared::{serde::Deserialize, swc_core::base::config::Options};

#[derive(Default, Debug, Deserialize)]
#[serde(crate = "shared::serde", rename_all="camelCase")]
pub struct Extensions {
  pub modularize_imports: Option<HashMap<String, PackageConfig>>,
  pub plugin_import: Option<Vec<PluginImportConfig>>,
  pub react_utils: Option<ReactUtilsConfig>,

  pub lock_corejs_version: Option<LockCoreJsVersion>,

  pub emotion: Option<swc_emotion::EmotionOptions>,
  pub styled_components: Option<styled_components::Config>,
  pub styled_jsx: Option<bool>,

  pub lodash: Option<PluginLodashConfig>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(crate = "shared::serde")]
pub struct TransformConfig {
  pub swc: Options,

  /// Internal rust-swc Plugins
  pub extensions: Extensions,
}
