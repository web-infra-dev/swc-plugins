use swc_plugins_core::plugin_emotion::EmotionOptions;
use napi::Status;
use shared::serde_json;

use super::IntoRawConfig;

pub type EmotionOptionsNapi = String;

impl IntoRawConfig<EmotionOptions> for EmotionOptionsNapi {
  fn into_raw_config(self, _env: napi::Env) -> napi::Result<EmotionOptions> {
    serde_json::from_str(&self)
      .map_err(|_| napi::Error::new(Status::InvalidArg, "invalid emotion options".into()))
  }
}
