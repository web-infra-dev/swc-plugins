use napi::Status;
use swc_plugins_collection::plugin_emotion::EmotionOptions;

use super::IntoRawConfig;

pub type EmotionOptionsNapi = String;

impl IntoRawConfig<EmotionOptions> for EmotionOptionsNapi {
  fn into_raw_config(self, _env: napi::Env) -> napi::Result<EmotionOptions> {
    serde_json::from_str(&self)
      .map_err(|_| napi::Error::new::<String>(Status::InvalidArg, "invalid emotion options".into()))
  }
}
