#![feature(let_chains)]
use std::sync::Arc;

pub mod remove_prop_types;
use shared::serde::Deserialize;
use swc_core::{
  common::{chain, pass::Either},
  ecma::{transforms::base::pass::noop, visit::Fold},
};
use shared::PluginContext;

mod import_react;
mod remove_effect;

pub use import_react::auto_import_react;
pub use remove_effect::remove_effect;

use crate::remove_prop_types::react_remove_prop_types;

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(crate = "shared::serde", rename_all = "camelCase")]
pub struct ReactUtilsConfig {
  #[serde(default)]
  pub auto_import_react: bool,
  #[serde(default)]
  pub remove_effect: bool,
  #[serde(default)]
  pub remove_prop_types: Option<remove_prop_types::ReactRemovePropTypeConfig>,
}

pub fn react_utils(
  config: &ReactUtilsConfig,
  plugin_context: Arc<PluginContext>,
) -> impl Fold + '_ {
  chain!(
    if config.auto_import_react {
      Either::Left(auto_import_react(plugin_context.top_level_mark))
    } else {
      Either::Right(noop())
    },
    if config.remove_effect {
      Either::Left(remove_effect())
    } else {
      Either::Right(noop())
    },
    if let Some(config) = &config.remove_prop_types {
      Either::Left(react_remove_prop_types(config, plugin_context))
    } else {
      Either::Right(noop())
    }
  )
}
