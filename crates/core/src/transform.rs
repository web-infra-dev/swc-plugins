use crate::get_compiler;
use shared::{
  anyhow::Result,
  swc::{self, try_with_handler, HandlerOpts, TransformOutput},
  swc_common::FileName,
  swc_ecma_transforms_base::pass::noop,
};

use transform::{pass::internal_transform_pass, types::TransformConfig};

pub fn transform(config: TransformConfig, code: &str) -> Result<TransformOutput> {
  let compiler = get_compiler();
  let cm = compiler.cm.clone();

  try_with_handler(cm.clone(), HandlerOpts::default(), |handler| {
    compiler.run_transform(handler, true, || {
      let source_filename = "test";
      let fm = cm.new_source_file(
        FileName::Custom(source_filename.to_string()),
        code.to_string(),
      );
      compiler.process_js_with_custom_pass(
        fm,
        None,
        handler,
        &swc::config::Options {
          ..config.swc.clone()
        },
        |_, _| internal_transform_pass(&config),
        |_, _| noop(),
      )
    })
  })
}
