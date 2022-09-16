use std::{path::PathBuf, sync::Arc};

use shared::{
  anyhow::Result,
  swc::{config::JsMinifyOptions, try_with_handler, HandlerOpts, TransformOutput, Compiler},
  swc_common::FileName,
};

pub fn minify(
  compiler: Arc<Compiler>,
  config: &JsMinifyOptions,
  filename: String,
  src: String,
) -> Result<TransformOutput> {
  let cm = compiler.cm.clone();
  let fm = cm.new_source_file(FileName::Real(PathBuf::from(filename)), src);

  try_with_handler(cm, HandlerOpts::default(), |handler| {
    compiler.minify(fm, handler, config)
  })
}
