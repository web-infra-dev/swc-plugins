use std::sync::Arc;

use shared::{swc::Compiler, swc_common::{SourceMap, sync::Lazy}};

pub mod pass;
pub mod transform;
pub mod minify;

pub static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| {
  let cm = Arc::new(SourceMap::default());

  Arc::new(Compiler::new(cm))
});

pub fn get_compiler() -> Arc<Compiler> {
  COMPILER.clone()
}
