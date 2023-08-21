#![feature(let_chains)]
mod minify;
mod transform;

pub use minify::{minify, minify_css, CssMinifyOptions};
use swc_core::base::config::Options;
use swc_plugins_utils::PluginContext;
pub use transform::transform;

pub type TransformFn<'a, E, P> =
  fn(extensions_config: &'a E, swc_config: &Options, plugin_context: &PluginContext) -> P;
