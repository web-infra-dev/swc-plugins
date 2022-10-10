use std::{path::PathBuf, sync::Arc};

use shared::{
  anyhow::Result,
  swc_core:: {
    base::{config::JsMinifyOptions, try_with_handler, Compiler, HandlerOpts, TransformOutput},
    common::{errors::ColorConfig, sync::Lazy, FileName},
  }
};

static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| Arc::new(Compiler::new(Default::default())));

pub fn minify(config: &JsMinifyOptions, filename: String, src: &str) -> Result<TransformOutput> {
  let cm = COMPILER.cm.clone();
  let fm = cm.new_source_file(FileName::Real(PathBuf::from(filename)), src.to_string());

  try_with_handler(
    cm,
    HandlerOpts {
      color: ColorConfig::Never,
      skip_filename: false,
    },
    |handler| COMPILER.minify(fm, handler, config),
  )
}
