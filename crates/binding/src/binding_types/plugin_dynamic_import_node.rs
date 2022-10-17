use super::IntoRawConfig;
use modern_swc_core::plugin_dynamic_import_node::DynImportNodeConfig;
use napi_derive::napi;

#[derive(Default)]
#[napi(object)]
pub struct DynImportNodeConfigNapi {
  pub interop: Option<bool>
}

impl IntoRawConfig<DynImportNodeConfig> for DynImportNodeConfigNapi {
  fn into_raw_config(self, _env: napi::Env) -> napi::Result<DynImportNodeConfig> {
    Ok(DynImportNodeConfig { interop: self.interop.unwrap_or(false) })
  }
}
