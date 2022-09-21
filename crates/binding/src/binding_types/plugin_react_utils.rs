use core::plugin_react_utils::ReactUtilsConfig;

use shared::serde::Deserialize;

use napi::Env;
use napi_derive::napi;

use super::FromNapi;

#[napi(object)]
#[derive(Deserialize, Debug)]
#[serde(crate = "shared::serde")]
pub struct ReactUtilsConfigNapi {
  pub auto_import_react: Option<bool>,
  pub rm_effect: Option<bool>,
}

impl FromNapi<ReactUtilsConfig> for ReactUtilsConfigNapi {
  fn from_napi(self, _: Env) -> napi::Result<ReactUtilsConfig> {
    Ok(ReactUtilsConfig {
      auto_import_react: self.auto_import_react,
      rm_effect: self.rm_effect,
    })
  }
}
