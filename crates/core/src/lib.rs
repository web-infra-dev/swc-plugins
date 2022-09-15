use std::sync::Arc;

use shared::{
  swc::Compiler,
  swc_common::{sync::Lazy, SourceMap},
};

pub mod minify;
pub mod transform;

/**
 * Provide compiler
 */
pub static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| {
  let cm = Arc::new(SourceMap::default());

  Arc::new(Compiler::new(cm))
});

pub fn get_compiler() -> Arc<Compiler> {
  COMPILER.clone()
}
