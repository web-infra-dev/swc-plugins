use modern_swc_plugins_collection::swc_plugin_lock_corejs_version::LockCoreJsVersion;
use napi_derive::napi;

use super::IntoRawConfig;

#[napi(object)]
pub struct LockCoreJsVersionNapi {
  pub corejs: String,
  pub swc_helpers: String,
}

impl IntoRawConfig<LockCoreJsVersion> for LockCoreJsVersionNapi {
  fn into_raw_config(self, _env: napi::Env) -> napi::Result<LockCoreJsVersion> {
    Ok(LockCoreJsVersion {
      corejs: self.corejs,
      swc_helpers: self.swc_helpers,
    })
  }
}
