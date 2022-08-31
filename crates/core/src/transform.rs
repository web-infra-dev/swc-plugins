use crate::{get_compiler, pass::internal_transform_pass};
use shared::{
  anyhow::Result,
  swc::{self, try_with_handler, HandlerOpts, TransformOutput},
  swc_common::{FileName, Mark},
  swc_ecma_transforms_base::pass::noop,
  swc_ecma_visit::as_folder,
};

use transform::types::TransformConfig;

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

      // main transform pass
      let top_level_mark = Mark::new();

      compiler.process_js_with_custom_pass(
        fm,
        None,
        handler,
        &swc::config::Options {
          top_level_mark: Some(top_level_mark),
          ..config.swc.clone()
        },
        |_, _| as_folder(internal_transform_pass(cm, &config, top_level_mark)),
        |_, _| noop(),
      )
    })
  })
}
