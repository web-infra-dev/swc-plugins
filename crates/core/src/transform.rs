use std::path::PathBuf;

use crate::get_compiler;
use shared::{
  anyhow::Result,
  napi::Env,
  swc::{self, try_with_handler, HandlerOpts, TransformOutput},
  swc_common::FileName,
  swc_ecma_transforms_base::pass::noop,
};

use pass::{internal_transform_pass, types::TransformConfig};

pub fn transform(env: Option<Env>, mut config: TransformConfig, code: &str) -> Result<TransformOutput> {
  let compiler = get_compiler();
  let cm = compiler.cm.clone();

  try_with_handler(cm.clone(), HandlerOpts::default(), |handler| {
    compiler.run_transform(handler, true, || {
      let fm = cm.new_source_file(
        FileName::Real(PathBuf::from(&config.swc.filename)),
        code.to_string(),
      );
      compiler.process_js_with_custom_pass(
        fm,
        None,
        handler,
        &swc::config::Options { ..config.swc.clone() },
        |_, _| internal_transform_pass(env, &mut config),
        |_, _| noop(),
      )
    })
  })
}
