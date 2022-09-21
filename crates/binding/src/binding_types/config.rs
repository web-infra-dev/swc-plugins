use core::types::TransformConfig;

use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use shared::serde_json;

use super::{extensions::ExtensionsNapi, FromNapi};

#[napi(object)]
pub struct TransformConfigNapi {
  /// Raw swc options
  pub swc: Buffer,

  /// Internal rust-swc Plugins
  pub extensions: ExtensionsNapi,
}

impl FromNapi<TransformConfig> for TransformConfigNapi {
  fn from_napi(self, env: napi::Env) -> napi::Result<TransformConfig> {
    let Self { swc, extensions } = self;

    Ok(TransformConfig {
      swc: serde_json::from_slice(&swc.to_vec().as_slice()).unwrap(),
      extensions: extensions.from_napi(env)?,
    })
  }
}
