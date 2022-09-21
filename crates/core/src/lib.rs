mod minify;
mod transform;
mod pass;

pub mod types;
pub use minify::minify;
pub use transform::transform;
pub use plugin_import;
pub use plugin_modularize_imports;
pub use plugin_react_utils;