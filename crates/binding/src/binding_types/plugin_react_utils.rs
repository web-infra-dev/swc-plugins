use swc_plugins_core::plugin_react_utils::{remove_prop_types::ReactRemovePropTypeConfig, ReactUtilsConfig};

use shared::serde::Deserialize;

use napi::Env;
use napi_derive::napi;

use super::IntoRawConfig;

#[napi(object)]
#[derive(Deserialize, Debug)]
#[serde(crate = "shared::serde")]
pub struct ReactUtilsConfigNapi {
  pub auto_import_react: Option<bool>,
  pub rm_effect: Option<bool>,
  pub rm_prop_types: Option<ReactUtilsRmPropTypesConfig>,
}

#[napi(object)]
#[derive(Deserialize, Debug)]
#[serde(crate = "shared::serde")]
pub struct ReactUtilsRmPropTypesConfig {
  pub mode: String,
  pub remove_import: bool,
  pub ignore_filenames: Vec<String>,
  pub additional_libraries: Vec<String>,
  pub class_name_matchers: Vec<String>,
}

impl IntoRawConfig<ReactUtilsConfig> for ReactUtilsConfigNapi {
  fn into_raw_config(self, env: Env) -> napi::Result<ReactUtilsConfig> {
    Ok(ReactUtilsConfig {
      auto_import_react: self.auto_import_react.unwrap_or(false),
      rm_effect: self.rm_effect.unwrap_or(false),
      rm_prop_types: self.rm_prop_types.map(|config| ReactRemovePropTypeConfig {
        mode: config.mode.into(),
        remove_import: config.remove_import,
        ignore_filenames: config.ignore_filenames.into_raw_config(env).unwrap(),
        additional_libraries: config.additional_libraries.into_raw_config(env).unwrap(),
        class_name_matchers: config.class_name_matchers.into_raw_config(env).unwrap(),
      }),
    })
  }
}
