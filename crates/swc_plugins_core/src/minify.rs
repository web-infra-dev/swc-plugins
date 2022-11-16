use std::{path::PathBuf, sync::Arc};

use shared::{
  anyhow::Result,
  swc_core::{
    base::{config::JsMinifyOptions, try_with_handler, Compiler, HandlerOpts, TransformOutput},
    common::{errors::ColorConfig, sync::Lazy, FileName, Globals, GLOBALS},
  },
};

static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| Arc::new(Compiler::new(Default::default())));

pub fn minify(config: &JsMinifyOptions, filename: String, src: &str) -> Result<TransformOutput> {
  GLOBALS.set(&Globals::default(), || {
    let cm = COMPILER.cm.clone();

    let filename = if filename.is_empty() {
      FileName::Anon
    } else {
      FileName::Real(PathBuf::from(filename))
    };
    let fm = cm.new_source_file(filename, src.to_string());

    try_with_handler(
      cm,
      HandlerOpts {
        color: ColorConfig::Never,
        skip_filename: false,
      },
      |handler| COMPILER.minify(fm, handler, config),
    )
  })
}
