use napi_derive::napi;
use swc_plugins_collection::plugin_react_const_elements::ReactConstElementsOptions;

use super::IntoRawConfig;

#[derive(Default)]
#[napi(object)]
pub struct ReactConstElementsOptionsNapi {
  pub immutable_globals: Option<Vec<String>>,
  pub allow_mutable_props_on_tags: Option<Vec<String>>,
}

impl IntoRawConfig<ReactConstElementsOptions> for ReactConstElementsOptionsNapi {
  fn into_raw_config(self, _: napi::Env) -> napi::Result<ReactConstElementsOptions> {
    let Self {
      immutable_globals,
      allow_mutable_props_on_tags,
    } = self;

    Ok(ReactConstElementsOptions {
      immutable_globals: rustc_hash::FxHashSet::from_iter(immutable_globals.unwrap_or_default()),
      allow_mutable_props_on_tags: rustc_hash::FxHashSet::from_iter(
        allow_mutable_props_on_tags.unwrap_or_default(),
      ),
    })
  }
}
