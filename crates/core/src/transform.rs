use std::{path::PathBuf, sync::Arc};

use shared::{
  anyhow::Result,
  swc_core::{
    base::{config, try_with_handler, Compiler, HandlerOpts, TransformOutput},
    common::{errors::ColorConfig, FileName, GLOBALS},
    ecma::{transforms::base::pass::noop},
  },
};

use crate::pass::internal_transform_pass;
use crate::types::TransformConfig;

pub fn transform(
  compiler: Arc<Compiler>,
  config: &TransformConfig,
  filename: String,
  code: &str,
  input_source_map: Option<String>,
) -> Result<TransformOutput> {
  GLOBALS.set(&Default::default(), || {
    let cm = compiler.cm.clone();

    try_with_handler(
      cm.clone(),
      HandlerOpts {
        color: ColorConfig::Never,
        skip_filename: false,
      },
      |handler| {
        compiler.run_transform(handler, true, || {
          let fm = cm.new_source_file(FileName::Real(PathBuf::from(&filename)), code.to_string());

          let mut swc_config = config::Options {
            ..config.swc.clone()
          };
          swc_config.config.input_source_map = input_source_map.map(config::InputSourceMap::Str);
          swc_config.filename = filename;

          compiler.process_js_with_custom_pass(
            fm,
            None,
            handler,
            &swc_config,
            // TODO pass comments to internal pass
            |_, _comments| internal_transform_pass(config, cm),
            |_, _| noop(),
          )
        })
      },
    )
  })
}
