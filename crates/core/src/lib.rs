mod minify;
mod pass;
mod transform;

pub mod types;
pub use minify::minify;
pub use plugin_import;
pub use plugin_modularize_imports;
pub use plugin_react_utils;
pub use transform::transform;
