#![feature(let_chains)]
mod minify;
mod pass;
mod transform;

pub mod types;
pub use minify::minify;
pub use transform::transform;

// plugins
pub use modularize_imports;
pub use swc_plugin_import;
pub use plugin_lock_corejs_version;
pub use swc_plugin_lodash;
pub use swc_plugin_react_utils;
pub use styled_components as plugin_styled_components;
pub use styled_jsx as plugin_styled_jsx;
pub use swc_emotion as plugin_emotion;
