use napi_derive::napi;
use swc_plugins_collection::plugin_ssr_loader_id::SSRLoaderIdConfig;

use super::IntoRawConfig;

#[derive(Default)]
#[napi(object)]
pub struct SSRLoaderIdConfigNapi {
  pub runtime_package_name: String,
  pub function_use_loader_name: Option<String>,
  pub function_use_static_loader_name: Option<String>,
  pub function_create_container_name: Option<String>,
}

impl IntoRawConfig<SSRLoaderIdConfig> for SSRLoaderIdConfigNapi {
  fn into_raw_config(self, _: napi::Env) -> napi::Result<SSRLoaderIdConfig> {
    let Self {
      runtime_package_name,
      function_create_container_name,
      function_use_loader_name,
      function_use_static_loader_name,
    } = self;
    Ok(SSRLoaderIdConfig {
      runtime_package_name,
      function_create_container_name,
      function_use_loader_name,
      function_use_static_loader_name,
    })
  }
}
