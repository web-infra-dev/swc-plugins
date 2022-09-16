use std::{path::PathBuf, sync::Arc};

use shared::{
  anyhow::Result,
  napi::Env,
  swc::{self, try_with_handler, Compiler, HandlerOpts, TransformOutput},
  swc_common::FileName,
  swc_ecma_transforms_base::pass::noop,
};

use pass::{internal_transform_pass, types::TransformConfig};

pub fn transform(
  env: Option<Env>,
  compiler: Arc<Compiler>,
  config: &TransformConfig,
  filename: String,
  code: &str,
  input_source_map: Option<String>,
) -> Result<TransformOutput> {
  let cm = compiler.cm.clone();
  try_with_handler(cm.clone(), HandlerOpts::default(), |handler| {
    compiler.run_transform(handler, true, || {
      let fm = cm.new_source_file(FileName::Real(PathBuf::from(&filename)), code.to_string());

      let mut swc_config = swc::config::Options {
        ..config.swc.clone()
      };
      swc_config.config.input_source_map =
        input_source_map.map(|m| swc::config::InputSourceMap::Str(m));
      swc_config.filename = filename;

      compiler.process_js_with_custom_pass(
        fm,
        None,
        handler,
        &swc_config,
        |_, _| internal_transform_pass(env, &config),
        |_, _| noop(),
      )
    })
  })
}
