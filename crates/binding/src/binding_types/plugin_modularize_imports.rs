use swc_plugins_core::plugin_modularize_imports::PackageConfig;

use shared::serde::Deserialize;

use napi_derive::napi;

use super::IntoRawConfig;

#[napi(object)]
#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "shared::serde")]
pub struct PackageConfigNapi {
  pub transform: String,
  pub prevent_full_import: bool,
  pub skip_default_conversion: bool,
}

impl IntoRawConfig<PackageConfig> for PackageConfigNapi {
  fn into_raw_config(self, _: napi::Env) -> napi::Result<PackageConfig> {
    Ok(PackageConfig {
      transform: self.transform,
      prevent_full_import: self.prevent_full_import,
      skip_default_conversion: self.skip_default_conversion,
    })
  }
}
