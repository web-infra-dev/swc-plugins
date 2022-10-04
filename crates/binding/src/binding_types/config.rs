use modern_swc_core::types::TransformConfig;

use napi_derive::napi;
use shared::serde_json;

use super::{extensions::ExtensionsNapi, IntoRawConfig};

#[napi(object)]
pub struct TransformConfigNapi {
  /// Raw swc options
  pub swc: String,

  /// Internal rust-swc Plugins
  pub extensions: ExtensionsNapi,
}

impl IntoRawConfig<TransformConfig> for TransformConfigNapi {
  fn into_raw_config(self, env: napi::Env) -> napi::Result<TransformConfig> {
    let Self { swc, extensions } = self;

    Ok(TransformConfig {
      swc: serde_json::from_str(&swc).unwrap(),
      extensions: extensions.into_raw_config(env)?,
    })
  }
}
