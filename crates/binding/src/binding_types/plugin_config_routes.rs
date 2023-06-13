use napi_derive::napi;
use swc_plugins_collection::plugin_config_routes::ConfigRoutesConfig;

use super::IntoRawConfig;

#[derive(Default)]
#[napi(object)]
pub struct ConfigRoutesConfigNapi {
    pub lazy: Option<bool>,
}

impl IntoRawConfig<ConfigRoutesConfig> for ConfigRoutesConfigNapi {
    fn into_raw_config(self, _: napi::Env) -> napi::Result<ConfigRoutesConfig> {
        let Self {
            lazy,
        } = self;

        Ok(ConfigRoutesConfig { lazy })
    }
}