use std::path::PathBuf;

use modern_swc_core::plugin_lodash::PluginLodashConfig;
use napi_derive::napi;

use super::IntoRawConfig;

#[napi(object)]
pub struct PluginLodashConfigNapi {
  pub cwd: String,
  pub ids: Vec<String>,
}

impl IntoRawConfig<PluginLodashConfig> for PluginLodashConfigNapi {
  fn into_raw_config(self, _env: napi::Env) -> napi::Result<PluginLodashConfig> {
    Ok(PluginLodashConfig {
      cwd: PathBuf::from(&self.cwd),
      ids: self.ids,
    })
  }
}
