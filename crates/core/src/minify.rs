use crate::get_compiler;
use shared::{
  anyhow::Result,
  swc::{config::JsMinifyOptions, try_with_handler, HandlerOpts, TransformOutput},
  swc_common::FileName,
};

pub fn minify(
  config: &JsMinifyOptions,
  src: String,
  filename: String,
  // TODO sourcemap not support yet
  _map: Option<String>,
) -> Result<TransformOutput> {
  let c = get_compiler();
  let cm = c.cm.clone();
  let fm = cm.new_source_file(FileName::Custom(filename), src);
  try_with_handler(cm, HandlerOpts::default(), |handler| {
    c.minify(fm, handler, config)
  })
}
