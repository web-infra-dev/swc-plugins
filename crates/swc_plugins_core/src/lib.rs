#![feature(let_chains)]
mod minify;
mod transform;

use std::sync::Arc;

pub use minify::minify;
use swc_core::base::config::Options;
use swc_plugins_utils::PluginContext;
pub use transform::transform;

pub type TransformFn<'a, E, P> =
  fn(configs: &'a E, swc_config: &Options, plugin_context: Arc<PluginContext>) -> P;

pub use modularize_imports;
pub use plugin_lock_corejs_version;
pub use styled_components as plugin_styled_components;
pub use styled_jsx as plugin_styled_jsx;
pub use swc_emotion as plugin_emotion;
pub use swc_plugin_import;
pub use swc_plugin_lodash;
pub use swc_plugin_react_utils;
