use napi_derive::napi;
use swc_plugins_collection::plugin_react_const_elements::ReactConstElementsOptions;

use super::IntoRawConfig;

#[derive(Default)]
#[napi(object)]
pub struct ReactConstElementsOptionsNapi {
  pub immutable_globals: Vec<String>,
}

impl IntoRawConfig<ReactConstElementsOptions> for ReactConstElementsOptionsNapi {
  fn into_raw_config(self, _: napi::Env) -> napi::Result<ReactConstElementsOptions> {
    let Self { immutable_globals } = self;
    Ok(ReactConstElementsOptions { immutable_globals: rustc_hash::FxHashSet::from(immutable_globals.into_iter()) })
  }
}
