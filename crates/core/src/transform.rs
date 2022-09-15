use std::path::PathBuf;

use crate::get_compiler;
use shared::{
  anyhow::Result,
  napi::Env,
  swc::{self, config::SourceMapsConfig, try_with_handler, HandlerOpts, TransformOutput},
  swc_common::FileName,
  swc_ecma_transforms_base::pass::noop,
};

use pass::{internal_transform_pass, types::TransformConfig};

pub fn transform(
  env: Option<Env>,
  code: &str,
  filename: String,
  map: Option<String>,
  config: &TransformConfig,
) -> Result<TransformOutput> {
  let compiler = get_compiler();

  let cm = compiler.cm.clone();

  try_with_handler(cm.clone(), HandlerOpts::default(), |handler| {
    compiler.run_transform(handler, true, || {
      let fm = cm.new_source_file(FileName::Real(PathBuf::from(&filename)), code.to_string());

      let mut swc_config = swc::config::Options {
        ..config.swc.clone()
      };
      swc_config.source_maps = map.map(|m| SourceMapsConfig::Str(m));
      swc_config.filename = filename;

      compiler.process_js_with_custom_pass(
        fm,
        None,
        handler,
        &swc_config,
        |_, _| {
          internal_transform_pass(
            env,
            &config,
          )
        },
        |_, _| noop(),
      )
    })
  })
}
