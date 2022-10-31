use swc_plugins_core::plugin_lock_corejs_version::LockCoreJsVersion;
use napi_derive::napi;

use super::IntoRawConfig;

#[napi(object)]
pub struct LockCoreJsVersionNapi {
  pub corejs_path: String,
}

impl IntoRawConfig<LockCoreJsVersion> for LockCoreJsVersionNapi {
  fn into_raw_config(self, _env: napi::Env) -> napi::Result<LockCoreJsVersion> {
      Ok(LockCoreJsVersion {
        corejs_path: self.corejs_path
      })
  }
}
