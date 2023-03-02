use super::IntoRawConfig;
use modern_swc_plugins_collection::modularize_imports::PackageConfig;
use napi_derive::napi;
use serde::Deserialize;

#[napi(object)]
#[derive(Clone, Debug, Deserialize)]
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
