use std::{env::current_dir, path::PathBuf};

use napi_derive::napi;
use swc_plugins_collection::swc_plugin_lodash::PluginLodashConfig;

use super::IntoRawConfig;

#[napi(object)]
pub struct PluginLodashConfigNapi {
  pub cwd: Option<String>,
  pub ids: Option<Vec<String>>,
}

impl IntoRawConfig<PluginLodashConfig> for PluginLodashConfigNapi {
  fn into_raw_config(self, _env: napi::Env) -> napi::Result<PluginLodashConfig> {
    Ok(PluginLodashConfig {
      cwd: self
        .cwd
        .map(PathBuf::from)
        .unwrap_or_else(|| current_dir().unwrap()),
      ids: self.ids.unwrap_or_default(),
    })
  }
}
