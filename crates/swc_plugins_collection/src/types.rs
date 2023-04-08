use std::collections::HashMap;

use modularize_imports::PackageConfig;
use plugin_lock_corejs_version::LockCoreJsVersion;
use serde::Deserialize;
use swc_core::base::config::Options;
use swc_plugin_import::PluginImportConfig;
use swc_plugin_lodash::PluginLodashConfig;
use swc_plugin_react_utils::ReactUtilsConfig;

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
  pub modularize_imports: Option<HashMap<String, PackageConfig>>,
  pub plugin_import: Option<Vec<PluginImportConfig>>,
  pub react_utils: Option<ReactUtilsConfig>,

  pub lock_corejs_version: Option<LockCoreJsVersion>,

  pub emotion: Option<swc_emotion::EmotionOptions>,
  pub styled_components: Option<styled_components::Config>,
  pub styled_jsx: Option<bool>,

  pub lodash: Option<PluginLodashConfig>,
  pub modernjs_ssr_loader_id: Option<bool>,
  pub loadable_components: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
pub struct TransformConfig {
  pub swc: Options,

  /// Internal rust-swc Plugins
  pub extensions: Extensions,
}
