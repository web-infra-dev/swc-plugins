#![feature(let_chains)]
mod minify;
mod transform;

use std::sync::Arc;
pub use minify::{minify, CssMinifyOptions, minify_css};
use swc_plugins_utils::PluginContext;
use swc_core::base::config::Options;
pub use transform::transform;

pub type TransformFn<'a, E, P> =
  fn(extensions_config: &'a E, swc_config: &Options, plugin_context: Arc<PluginContext>) -> P;
